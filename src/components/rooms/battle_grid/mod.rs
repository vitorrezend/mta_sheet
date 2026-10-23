//! Mapa tático montável da sala.
//!
//! - [`store`]: estado do mapa, edições otimistas e sincronização em tempo real;
//! - [`render`]: desenho em canvas (camada do mapa + camada de destaques);
//! - [`toolbar`] e [`modals`]: ferramentas, paletas e janelas;
//! - [`geometry`]: régua, pincel e posicionamento.
//!
//! Toda a interação passa por um único elemento com Pointer Events, que funciona igual
//! com mouse, caneta e toque (iPhone incluído). A geometria das grades fica em
//! `rules::map_grid`.

pub mod geometry;
pub mod modals;
pub mod render;
pub mod store;
pub mod toolbar;

use leptos::*;
use leptos::html::{Canvas, Div};
use crate::components::common::Callback;
use crate::components::rooms::room_events::{on_room_event, RoomStreamEvent};
use crate::rooms::{MapLayer, MapOp, MapProp, MapToken, RoomBroadcastEvent, RoomSheetSummary, WallKind, WallPoint, WallSegment};
use crate::rules::map_grid::{self, Cell};
use modals::{BackgroundModal, GridConfigModal, NpcModal};
use render::Overlay;
use store::MapStore;
use toolbar::{GridToolbar, Tool, ToolState};

/// Movimento mínimo (px de tela) para um toque virar arraste
const DRAG_THRESHOLD: f32 = 6.0;

#[derive(Clone, Copy, PartialEq, Eq)]
enum GridModal {
    Config,
    Npc,
    Background,
}

#[derive(Clone)]
enum Drag {
    None,
    Paint(Vec<Cell>),
    /// Parede sendo desenhada: início e fim já encaixados (pixels)
    WallLine { start: (f32, f32), end: (f32, f32) },
    /// Borracha de paredes: ids marcados até agora
    EraseWalls(Vec<String>),
    Token { id: String, start: Cell, movable: bool, moved: bool },
    Prop { id: String, start: Cell, moved: bool },
    Door(String),
    PlaceProp(Cell),
    Pan { x0: f32, y0: f32, left: i32, top: i32, moved: bool },
    Ruler(Cell),
}

fn token_radius(token: &MapToken, cs: f32) -> f32 {
    cs * token.size_cells * 0.85 / 2.0
}

#[component]
pub fn BattleGrid(room_id: Signal<String>, room_sheets: Signal<Vec<RoomSheetSummary>>) -> impl IntoView {
    let store = MapStore::new(room_id);
    create_effect(move |_| {
        if !room_id.get().is_empty() {
            store.reload();
        }
    });
    on_room_event(move |event| match event {
        RoomStreamEvent::Event(RoomBroadcastEvent::MapChanged { version, ops }) => store.on_remote_change(*version, ops.as_ref()),
        RoomStreamEvent::Event(RoomBroadcastEvent::Resync) | RoomStreamEvent::Reconnected => store.reload(),
        _ => {}
    });

    let tools = ToolState::new();
    let overlay = create_rw_signal(Overlay::default());
    let selected_token = create_rw_signal(Option::<String>::None);
    let token_drag = create_rw_signal(Option::<(String, f32, f32)>::None);
    let modal = create_rw_signal(Option::<GridModal>::None);
    let drag = store_value(Drag::None);
    let down_at = store_value((0.0f32, 0.0f32));

    let scroll_ref = create_node_ref::<Div>();
    let board_ref = create_node_ref::<Div>();
    let base_canvas = create_node_ref::<Canvas>();
    let overlay_canvas = create_node_ref::<Canvas>();
    // Os canvas só são desenhados no navegador
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (base_canvas, overlay_canvas);

    let is_gm = Signal::derive(move || store.is_gm.get());
    let board_size = Signal::derive(move || store.view.with(|m| map_grid::board_size(m.grid_shape, m.cols, m.rows, m.cell_size as f32)));
    let status = Signal::derive(move || match (store.error.get(), store.saving.get()) {
        (Some(e), _) => Some((true, e)),
        (None, true) => Some((false, "Salvando…".to_string())),
        _ => None,
    });

    // ---------------------------------------------------------------------
    // Desenho
    // ---------------------------------------------------------------------
    #[cfg(target_arch = "wasm32")]
    let bg_tick = create_rw_signal(0u32);
    #[cfg(target_arch = "wasm32")]
    let bg_image = store_value(Option::<(String, web_sys::HtmlImageElement)>::None);
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::{closure::Closure, JsCast};

        let context = |canvas: &web_sys::HtmlCanvasElement| -> Option<web_sys::CanvasRenderingContext2d> {
            canvas.get_context("2d").ok().flatten().and_then(|c| c.dyn_into().ok())
        };
        let pixel_ratio = move || {
            let (w, h) = board_size.get_untracked();
            let device = web_sys::window().map(|w| w.device_pixel_ratio()).unwrap_or(1.0);
            render::canvas_pixel_ratio(w, h, device)
        };

        // Carrega a imagem de fundo quando o endereço muda
        create_effect(move |_| {
            let url = store.view.with(|m| m.bg_image_url.clone());
            let cached = bg_image.with_value(|c| c.as_ref().map(|(u, _)| u.clone()));
            if url.is_empty() || cached.as_deref() == Some(url.as_str()) {
                return;
            }
            if let Ok(img) = web_sys::HtmlImageElement::new() {
                let onload = Closure::<dyn FnMut()>::new(move || bg_tick.update(|t| *t += 1));
                img.set_onload(Some(onload.as_ref().unchecked_ref()));
                onload.forget();
                img.set_src(&url);
                bg_image.set_value(Some((url, img)));
            }
        });

        // Tamanho físico dos canvas acompanha o tabuleiro
        create_effect(move |_| {
            let (w, h) = board_size.get();
            let ratio = pixel_ratio();
            for canvas in [base_canvas.get(), overlay_canvas.get()].into_iter().flatten() {
                canvas.set_width((w as f64 * ratio).round() as u32);
                canvas.set_height((h as f64 * ratio).round() as u32);
            }
        });

        // Camada do mapa
        create_effect(move |_| {
            bg_tick.track();
            board_size.track();
            let Some(canvas) = base_canvas.get() else { return };
            let Some(ctx) = context(&canvas) else { return };
            let ratio = pixel_ratio();
            store.view.with(|map| {
                bg_image.with_value(|bg| {
                    let img = bg.as_ref().filter(|(u, _)| *u == map.bg_image_url).map(|(_, i)| i);
                    render::draw_base(&ctx, map, img, ratio);
                });
            });
        });

        // Camada de destaques
        create_effect(move |_| {
            board_size.track();
            let Some(canvas) = overlay_canvas.get() else { return };
            let Some(ctx) = context(&canvas) else { return };
            let ratio = pixel_ratio();
            let ov = overlay.get();
            store.view.with(|map| render::draw_overlay(&ctx, map, &ov, ratio));
        });
    }

    // ---------------------------------------------------------------------
    // Zoom
    // ---------------------------------------------------------------------
    let fit = move || {
        let (w, h) = board_size.get_untracked();
        if let Some(scroll) = scroll_ref.get_untracked() {
            let (cw, ch) = (scroll.client_width() as f32, scroll.client_height() as f32);
            if cw > 0.0 && w > 0.0 {
                tools.zoom.set((cw / w).min(ch.max(240.0) / h).clamp(0.2, 2.0));
            }
        }
    };
    create_effect(move |was_loaded: Option<bool>| {
        let loaded = store.loaded.get();
        if loaded && was_loaded != Some(true) {
            request_animation_frame(fit);
        }
        loaded
    });

    // ---------------------------------------------------------------------
    // Entrada (mouse, caneta e toque)
    // ---------------------------------------------------------------------
    let board_point = move |ev: &ev::PointerEvent| -> (f32, f32) {
        let zoom = tools.zoom.get_untracked();
        board_ref.get_untracked().map_or((0.0, 0.0), |b| {
            let rect = b.get_bounding_client_rect();
            ((ev.client_x() as f32 - rect.left() as f32) / zoom, (ev.client_y() as f32 - rect.top() as f32) / zoom)
        })
    };
    // Ponta da parede: cantos da grade (e centros no hexágono)
    let snap = move |x: f32, y: f32| store.view.with_untracked(|m| map_grid::snap_point(m.grid_shape, m.cell_size as f32, x, y));
    // Na grade quadrada, um arraste quase horizontal/vertical trava no eixo: a linha fica reta
    let straighten = move |start: (f32, f32), end: (f32, f32)| -> (f32, f32) {
        if !store.view.with_untracked(|m| m.grid_shape == crate::rooms::GridShape::Square) {
            return end;
        }
        let (dx, dy) = ((end.0 - start.0).abs(), (end.1 - start.1).abs());
        if dy <= dx * 0.3 {
            (end.0, start.1)
        } else if dx <= dy * 0.3 {
            (start.0, end.1)
        } else {
            end
        }
    };
    let wall_at = move |x: f32, y: f32| store.view.with_untracked(|m| m.wall_near(x, y, m.cell_size as f32 * 0.3).cloned());
    let preview_drag = move |d: &Drag| match d {
        Drag::Paint(cells) => store.preview(&[MapOp::PaintTerrain { cells: cells.clone(), kind: tools.terrain.get_untracked() }]),
        Drag::WallLine { start, end } => {
            let (start, end) = (*start, *end);
            let kind = tools.wall.get_untracked().unwrap_or(WallKind::Wall);
            overlay.update(|o| {
                o.wall_preview = Some((start, end, kind));
                o.snap_point = None;
            });
        }
        Drag::EraseWalls(ids) => {
            let ids = ids.clone();
            overlay.update(|o| o.erasing_walls = ids);
        }
        _ => {}
    };

    let on_down = move |ev: ev::PointerEvent| {
        if ev.button() > 0 {
            return;
        }
        ev.prevent_default();
        #[cfg(target_arch = "wasm32")]
        if let Some(target) = ev.target().and_then(|t| wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(t).ok()) {
            let _ = target.set_pointer_capture(ev.pointer_id());
        }
        down_at.set_value((ev.client_x() as f32, ev.client_y() as f32));
        let (x, y) = board_point(&ev);
        let gm = store.is_gm.get_untracked();
        let (cell, cs, shape) = store.view.with_untracked(|m| (map_grid::pixel_to_cell(m.grid_shape, m.cell_size as f32, x, y), m.cell_size as f32, m.grid_shape));

        let next = match tools.tool.get_untracked() {
            Tool::Move => {
                let token = store.view.with_untracked(|m| {
                    m.tokens.iter().rev().find(|t| {
                        let (tx, ty) = map_grid::cell_center(shape, cs, t.cell());
                        (tx - x).hypot(ty - y) <= token_radius(t, cs)
                    }).cloned()
                });
                let prop = || store.view.with_untracked(|m| {
                    m.props.iter().rev().find(|p| {
                        let (px, py) = map_grid::cell_center(shape, cs, p.cell);
                        (px - x).hypot(py - y) <= cs * 0.45 * p.scale
                    }).map(|p| (p.id.clone(), p.cell))
                });
                let door = || wall_at(x, y).filter(|w| matches!(w.kind, WallKind::Door | WallKind::DoorOpen)).map(|w| w.id);
                if let Some(t) = token {
                    let (movable, start) = (store.can_move_token(&t), t.cell());
                    Drag::Token { movable, id: t.id, start, moved: false }
                } else if let Some((id, start)) = prop().filter(|_| gm) {
                    Drag::Prop { id, start, moved: false }
                } else if let Some(id) = door() {
                    Drag::Door(id)
                } else {
                    let (left, top) = scroll_ref.get_untracked().map_or((0, 0), |s| (s.scroll_left(), s.scroll_top()));
                    Drag::Pan { x0: ev.client_x() as f32, y0: ev.client_y() as f32, left, top, moved: false }
                }
            }
            Tool::Terrain if gm => Drag::Paint(store.view.with_untracked(|m| geometry::brush_cells(m, cell, tools.brush_size.get_untracked()))),
            Tool::Walls if gm => match tools.wall.get_untracked() {
                Some(_) => {
                    let start = snap(x, y);
                    Drag::WallLine { start, end: start }
                }
                None => Drag::EraseWalls(wall_at(x, y).map(|w| vec![w.id]).unwrap_or_default()),
            },
            Tool::Props if gm => Drag::PlaceProp(cell),
            Tool::Ruler => {
                overlay.update(|o| o.ruler = Some((cell, cell)));
                Drag::Ruler(cell)
            }
            _ => Drag::None,
        };
        preview_drag(&next);
        drag.set_value(next);
    };

    let on_move = move |ev: ev::PointerEvent| {
        let (x, y) = board_point(&ev);
        let (cell, in_bounds) = store.view.with_untracked(|m| {
            let c = map_grid::pixel_to_cell(m.grid_shape, m.cell_size as f32, x, y);
            (c, map_grid::in_bounds(c, m.cols, m.rows))
        });
        let (dx0, dy0) = down_at.get_value();
        let moved_far = (ev.client_x() as f32 - dx0).hypot(ev.client_y() as f32 - dy0) > DRAG_THRESHOLD;

        // Destaque sob o ponteiro, conforme a ferramenta
        let tool = tools.tool.get_untracked();
        let gm = store.is_gm.get_untracked();
        let hover_cell = (in_bounds && gm && matches!(tool, Tool::Terrain | Tool::Props)).then_some(cell);
        let idle = matches!(drag.get_value(), Drag::None);
        let snap_point = (gm && idle && tool == Tool::Walls && tools.wall.get_untracked().is_some()).then(|| snap(x, y));
        if overlay.with_untracked(|o| o.hover_cell != hover_cell || o.snap_point != snap_point) {
            overlay.update(|o| {
                o.hover_cell = hover_cell;
                o.snap_point = snap_point;
            });
        }

        let mut current = drag.get_value();
        match &mut current {
            Drag::Paint(cells) => {
                let fresh: Vec<Cell> = store.view.with_untracked(|m| geometry::brush_cells(m, cell, tools.brush_size.get_untracked()))
                    .into_iter()
                    .filter(|c| !cells.contains(c))
                    .collect();
                if fresh.is_empty() {
                    return;
                }
                cells.extend(fresh);
            }
            Drag::WallLine { start, end } => {
                let next = straighten(*start, snap(x, y));
                if next == *end {
                    return;
                }
                *end = next;
            }
            Drag::EraseWalls(ids) => match wall_at(x, y) {
                Some(w) if !ids.contains(&w.id) => ids.push(w.id),
                _ => return,
            },
            Drag::Token { id, moved, movable, .. } => {
                if !*movable || !(*moved || moved_far) {
                    return;
                }
                *moved = true;
                token_drag.set(Some((id.clone(), x, y)));
            }
            Drag::Prop { id, moved, .. } => {
                if !(*moved || moved_far) {
                    return;
                }
                *moved = true;
                let id = id.clone();
                overlay.update(|o| o.dragging_prop = Some((id, x, y)));
            }
            Drag::Pan { x0, y0, left, top, moved } => {
                if !(*moved || moved_far) {
                    return;
                }
                *moved = true;
                if let Some(scroll) = scroll_ref.get_untracked() {
                    scroll.set_scroll_left(*left - (ev.client_x() as f32 - *x0) as i32);
                    scroll.set_scroll_top(*top - (ev.client_y() as f32 - *y0) as i32);
                }
            }
            Drag::Ruler(start) => {
                let start = *start;
                overlay.update(|o| o.ruler = Some((start, cell)));
                return;
            }
            Drag::Door(_) | Drag::PlaceProp(_) | Drag::None => return,
        }
        preview_drag(&current);
        drag.set_value(current);
    };

    let on_up = move |ev: ev::PointerEvent| {
        let (x, y) = board_point(&ev);
        let (cell, in_bounds) = store.view.with_untracked(|m| {
            let c = map_grid::pixel_to_cell(m.grid_shape, m.cell_size as f32, x, y);
            (c, map_grid::in_bounds(c, m.cols, m.rows))
        });
        match drag.get_value() {
            Drag::Paint(cells) => store.send(vec![MapOp::PaintTerrain { cells, kind: tools.terrain.get_untracked() }]),
            Drag::WallLine { start, end } => {
                overlay.update(|o| o.wall_preview = None);
                let cs = store.view.with_untracked(|m| m.cell_size as f32);
                if (start.0 - end.0).hypot(start.1 - end.1) > cs * 0.2 {
                    store.send(vec![MapOp::AddWall(WallSegment {
                        id: format!("wall-{}", uuid::Uuid::new_v4()),
                        a: WallPoint::from_px(start.0, start.1, cs),
                        b: WallPoint::from_px(end.0, end.1, cs),
                        kind: tools.wall.get_untracked().unwrap_or(WallKind::Wall),
                    })]);
                }
            }
            Drag::EraseWalls(ids) => {
                overlay.update(|o| o.erasing_walls.clear());
                if !ids.is_empty() {
                    store.send(vec![MapOp::RemoveWalls { ids }]);
                }
            }
            Drag::Token { id, start, moved, .. } => {
                token_drag.set(None);
                if moved {
                    if in_bounds && cell != start {
                        store.send(vec![MapOp::MoveToken { id, cell }]);
                    }
                } else {
                    selected_token.update(|s| *s = if s.as_deref() == Some(id.as_str()) { None } else { Some(id) });
                    overlay.update(|o| o.selected_prop = None);
                }
            }
            Drag::Prop { id, start, moved } => {
                overlay.update(|o| o.dragging_prop = None);
                if moved {
                    if in_bounds && cell != start {
                        store.send(vec![MapOp::MoveProp { id, cell }]);
                    }
                } else {
                    selected_token.set(None);
                    overlay.update(|o| o.selected_prop = if o.selected_prop.as_deref() == Some(id.as_str()) { None } else { Some(id) });
                }
            }
            Drag::Door(id) => store.send(vec![MapOp::ToggleDoor { id }]),
            Drag::PlaceProp(at) if at == cell && in_bounds => {
                let id = format!("prop-{}", uuid::Uuid::new_v4());
                let rotation = 0;
                store.send(vec![MapOp::PlaceProp(MapProp { id, kind: tools.prop.get_untracked(), cell, rotation, scale: 1.0 })]);
            }
            Drag::Pan { moved: false, .. } => {
                selected_token.set(None);
                overlay.update(|o| o.selected_prop = None);
            }
            _ => {}
        }
        drag.set_value(Drag::None);
    };

    let on_cancel = move |_: ev::PointerEvent| {
        drag.set_value(Drag::None);
        token_drag.set(None);
        overlay.update(|o| {
            o.dragging_prop = None;
            o.wall_preview = None;
            o.erasing_walls.clear();
        });
        store.preview(&[]);
    };

    // ---------------------------------------------------------------------
    // Ações da barra e do painel de seleção
    // ---------------------------------------------------------------------
    let spawn_party = Callback::new(move |_: ()| {
        let mut map = store.view.get_untracked();
        let present: Vec<String> = map.tokens.iter().filter_map(|t| t.sheet_id.clone()).collect();
        let mut ops = Vec::new();
        for sheet in room_sheets.get_untracked().into_iter().filter(|s| !present.contains(&s.id)) {
            let cell = geometry::free_cell_near(&map, Cell::new(0, 0));
            let color = if sheet.health_badge_class.contains("incapacitated") || sheet.health_badge_class.contains("crippled") {
                "#ef4444"
            } else if sheet.health_badge_class.contains("full") {
                "#22c55e"
            } else {
                "#eab308"
            };
            let token = MapToken {
                id: format!("token-{}", uuid::Uuid::new_v4()),
                sheet_id: Some(sheet.id.clone()),
                name: sheet.name.clone(),
                avatar_url: sheet.photo_url.clone(),
                grid_col: cell.col as f32,
                grid_row: cell.row as f32,
                size_cells: 1.0,
                is_npc: false,
                is_hidden: sheet.is_hidden,
                color: color.to_string(),
                health_status: Some(sheet.health_label.clone()),
            };
            let op = MapOp::AddToken(token);
            let _ = map.apply(&op);
            ops.push(op);
        }
        for chunk in ops.chunks(crate::rooms::MAX_OPS_PER_BATCH) {
            store.send(chunk.to_vec());
        }
    });
    let create_npc = Callback::new(move |(name, color, size): (String, String, f32)| {
        let (map, center) = store.view.with_untracked(|m| (m.clone(), Cell::new(m.cols as i32 / 2, m.rows as i32 / 2)));
        let count = map.tokens.iter().filter(|t| t.is_npc).count() + 1;
        let cell = geometry::free_cell_near(&map, center);
        let name = if name.trim().is_empty() { format!("NPC {}", count) } else { name.trim().to_string() };
        store.send(vec![MapOp::AddToken(MapToken {
            id: format!("npc-{}", uuid::Uuid::new_v4()),
            sheet_id: None,
            name,
            avatar_url: String::new(),
            grid_col: cell.col as f32,
            grid_row: cell.row as f32,
            size_cells: size,
            is_npc: true,
            is_hidden: false,
            color,
            health_status: Some("NPC".into()),
        })]);
    });
    let close_modal = Callback::new(move |_: ()| modal.set(None));
    let (close_config, close_npc, close_background) = (close_modal.clone(), close_modal.clone(), close_modal);

    let selected_token_data = Signal::derive(move || {
        selected_token.get().and_then(|id| store.view.with(|m| m.tokens.iter().find(|t| t.id == id).cloned()))
    });
    let selected_prop_data = Signal::derive(move || {
        overlay.with(|o| o.selected_prop.clone()).and_then(|id| store.view.with(|m| m.props.iter().find(|p| p.id == id).cloned()))
    });
    let rotation_step = move || if store.view.with(|m| m.grid_shape == crate::rooms::GridShape::Square) { 90 } else { 60 };

    let ruler_text = move || {
        overlay.with(|o| o.ruler).filter(|(a, b)| a != b).map(|(a, b)| {
            let (cells, meters, range, m20) = store.view.with(|m| geometry::ruler_info(m, a, b));
            format!("📏 {:.1} m ({:.1} células) · {} · {}", meters, cells, range, m20)
        })
    };

    view! {
        <div class="battle-grid-wrapper">
            <GridToolbar
                tools=tools
                is_gm=is_gm
                status=status
                on_fit=Callback::new(move |_: ()| fit())
                on_spawn_party=spawn_party
                on_open_npc=Callback::new(move |_: ()| modal.set(Some(GridModal::Npc)))
                on_open_background=Callback::new(move |_: ()| modal.set(Some(GridModal::Background)))
                on_open_config=Callback::new(move |_: ()| modal.set(Some(GridModal::Config)))
                on_clear=Callback::new(move |layer: MapLayer| store.send(vec![MapOp::ClearLayer { layer }]))
            />

            <Show when=move || ruler_text().is_some()>
                <div class="grid-ruler-banner">{move || ruler_text().unwrap_or_default()}</div>
            </Show>

            <div class="grid-scroll" node_ref=scroll_ref>
                <div class="grid-spacer" style=move || {
                    let (w, h) = board_size.get();
                    let z = tools.zoom.get();
                    format!("width:{}px;height:{}px;", w * z, h * z)
                }>
                    <div class="grid-board" node_ref=board_ref style=move || {
                        let (w, h) = board_size.get();
                        format!("width:{}px;height:{}px;transform:scale({});", w, h, tools.zoom.get())
                    }>
                        <canvas class="grid-canvas" node_ref=base_canvas></canvas>
                        <canvas class="grid-canvas" node_ref=overlay_canvas></canvas>
                        <div class="grid-tokens">
                            <For
                                each=move || store.view.with(|m| m.tokens.iter().map(|t| t.id.clone()).collect::<Vec<_>>())
                                key=|id| id.clone()
                                children=move |id: String| view! { <TokenView id=id store=store selected=selected_token dragging=token_drag /> }
                            />
                        </div>
                        <div
                            class="grid-input"
                            class:tool-move=move || tools.tool.get() == Tool::Move
                            on:pointerdown=on_down
                            on:pointermove=on_move
                            on:pointerup=on_up
                            on:pointercancel=on_cancel
                            on:pointerleave=move |_| overlay.update(|o| { o.hover_cell = None; o.snap_point = None; })
                            on:contextmenu=move |ev: ev::MouseEvent| ev.prevent_default()
                        ></div>
                    </div>
                </div>
                <Show when=move || !store.loaded.get()>
                    <div class="grid-loading">"Carregando mapa…"</div>
                </Show>
            </div>

            <Show when=move || selected_token_data.with(Option::is_some)>
                <div class="grid-selection-panel">
                    <strong>{move || selected_token_data.with(|t| t.as_ref().map(|t| t.name.clone()).unwrap_or_default())}</strong>
                    <span class="grid-selection-hint">{move || selected_token_data.with(|t| t.as_ref().and_then(|t| t.health_status.clone()).unwrap_or_default())}</span>
                    <Show when=move || is_gm.get()>
                        <div class="grid-selection-actions">
                            <button type="button" on:click=move |_| {
                                if let Some(t) = selected_token_data.get_untracked() {
                                    store.send(vec![MapOp::SetTokenHidden { id: t.id, hidden: !t.is_hidden }]);
                                }
                            }>{move || if selected_token_data.with(|t| t.as_ref().is_some_and(|t| t.is_hidden)) { "👁 Mostrar aos jogadores" } else { "🙈 Ocultar dos jogadores" }}</button>
                            <button type="button" class="danger" on:click=move |_| {
                                if let Some(t) = selected_token_data.get_untracked() {
                                    store.send(vec![MapOp::RemoveToken { id: t.id }]);
                                    selected_token.set(None);
                                }
                            }>"✕ Remover do mapa"</button>
                        </div>
                    </Show>
                </div>
            </Show>

            <Show when=move || is_gm.get() && selected_prop_data.with(Option::is_some)>
                <div class="grid-selection-panel">
                    <strong>{move || selected_prop_data.with(|p| p.as_ref().map(|p| format!("{} {}", p.kind.icon(), p.kind.label())).unwrap_or_default())}</strong>
                    <div class="grid-selection-actions">
                        <button type="button" on:click=move |_| if let Some(p) = selected_prop_data.get_untracked() {
                            store.send(vec![MapOp::RotateProp { id: p.id, rotation: (p.rotation + rotation_step()) % 360 }]);
                        }>"⟳ Girar"</button>
                        <button type="button" on:click=move |_| if let Some(p) = selected_prop_data.get_untracked() {
                            store.send(vec![MapOp::ResizeProp { id: p.id, scale: p.scale - 0.5 }]);
                        }>"− Menor"</button>
                        <button type="button" on:click=move |_| if let Some(p) = selected_prop_data.get_untracked() {
                            store.send(vec![MapOp::ResizeProp { id: p.id, scale: p.scale + 0.5 }]);
                        }>"+ Maior"</button>
                        <button type="button" class="danger" on:click=move |_| if let Some(p) = selected_prop_data.get_untracked() {
                            store.send(vec![MapOp::RemoveProp { id: p.id }]);
                            overlay.update(|o| o.selected_prop = None);
                        }>"✕ Remover"</button>
                    </div>
                </div>
            </Show>

            <Show when=move || modal.get() == Some(GridModal::Config)>
                <GridConfigModal store=store on_close=close_config.clone() />
            </Show>
            <Show when=move || modal.get() == Some(GridModal::Npc)>
                <NpcModal on_create=create_npc.clone() on_close=close_npc.clone() />
            </Show>
            <Show when=move || modal.get() == Some(GridModal::Background)>
                <BackgroundModal store=store on_close=close_background.clone() />
            </Show>
        </div>
    }
}

/// Token de personagem ou NPC (HTML por cima do canvas, por causa da foto e do nome).
#[component]
fn TokenView(id: String, store: MapStore, selected: RwSignal<Option<String>>, dragging: RwSignal<Option<(String, f32, f32)>>) -> impl IntoView {
    let id_for_token = id.clone();
    let token = Signal::derive(move || store.view.with(|m| m.tokens.iter().find(|t| t.id == id_for_token).cloned()));
    let id_for_style = id.clone();
    let style = move || {
        let Some(t) = token.get() else { return "display:none".to_string() };
        let (shape, cs) = store.view.with(|m| (m.grid_shape, m.cell_size as f32));
        let (x, y) = match dragging.get().filter(|(d, _, _)| *d == id_for_style) {
            Some((_, x, y)) => (x, y),
            None => map_grid::cell_center(shape, cs, t.cell()),
        };
        let r = token_radius(&t, cs);
        format!("left:{}px;top:{}px;width:{}px;height:{}px;border-color:{};font-size:{}px;", x - r, y - r, r * 2.0, r * 2.0, t.color, (r * 0.8).max(10.0))
    };
    let id_for_selected = id.clone();
    let id_for_drag = id.clone();
    view! {
        <div
            class="grid-token"
            class:selected=move || selected.get().as_deref() == Some(id_for_selected.as_str())
            class:dragging=move || dragging.with(|d| d.as_ref().is_some_and(|(d, _, _)| *d == id_for_drag))
            class:hidden-token=move || token.with(|t| t.as_ref().is_some_and(|t| t.is_hidden))
            class:npc=move || token.with(|t| t.as_ref().is_some_and(|t| t.is_npc))
            style=style
        >
            {move || token.get().map(|t| if t.avatar_url.is_empty() {
                view! { <span class="grid-token-initial">{t.name.chars().next().unwrap_or('?').to_string()}</span> }.into_view()
            } else {
                view! { <img class="grid-token-img" src=t.avatar_url.clone() alt="" draggable="false" /> }.into_view()
            })}
            <span class="grid-token-name">{move || token.with(|t| t.as_ref().map(|t| t.name.clone()).unwrap_or_default())}</span>
        </div>
    }
}
