pub mod geometry;
pub mod modals;
pub mod toolbar;

pub use geometry::*;
pub use modals::*;
pub use toolbar::*;

use leptos::*;
use crate::rooms::{update_room_map, GridShape, MapStructure, MapToken, RoomMapData, RoomSheetSummary};

/// Helper para log de alta visibilidade no console do navegador (DevTools F12)
fn grid_log(msg: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(msg));
}

#[component]
pub fn BattleGrid(
    room_id: Signal<String>,
    is_gm: Signal<bool>,
    map_data: Signal<RoomMapData>,
    set_map_data: WriteSignal<RoomMapData>,
    room_sheets: Signal<Vec<RoomSheetSummary>>,
) -> impl IntoView {
    let (active_tool, set_active_tool) = create_signal("select"); // "select" | "ruler" | "build"
    let (selected_build_type, set_selected_build_type) = create_signal("wall".to_string());
    let (selected_token_id, set_selected_token_id) = create_signal(Option::<String>::None);
    let (hovered_cell, set_hovered_cell) = create_signal(Option::<(f32, f32)>::None);
    let (dragged_token_id, set_dragged_token_id) = create_signal(Option::<String>::None);

    // Sinais para modo Pincel / Arraste contínuo
    let (is_pointer_down, set_is_pointer_down) = create_signal(false);
    let (pointer_action, set_pointer_action) = create_signal("none"); // "none" | "build" | "erase"

    // Zoom Signals
    let (zoom_scale, set_zoom_scale) = create_signal(1.0f32);

    // Ruler Signals
    let (ruler_start, set_ruler_start) = create_signal(Option::<(f32, f32)>::None);
    let (ruler_end, set_ruler_end) = create_signal(Option::<(f32, f32)>::None);

    // Modals
    let (show_gm_modal, set_show_gm_modal) = create_signal(false);
    let (show_npc_modal, set_show_npc_modal) = create_signal(false);

    // GM Config Form Signals
    let (cfg_shape, set_cfg_shape) = create_signal(GridShape::Square);
    let (cfg_cols, set_cfg_cols) = create_signal(20u32);
    let (cfg_rows, set_cfg_rows) = create_signal(20u32);
    let (cfg_cell_size, set_cfg_cell_size) = create_signal(50u32);
    let (cfg_grid_color, set_cfg_grid_color) = create_signal("rgba(99, 102, 241, 0.35)".to_string());
    let (cfg_bg_url, set_cfg_bg_url) = create_signal(String::new());
    let (cfg_bg_opacity, set_cfg_bg_opacity) = create_signal(0.85f32);
    let (cfg_meters_per_cell, set_cfg_meters_per_cell) = create_signal(1.5f32);

    // NPC Form Signals
    let (npc_name, set_npc_name) = create_signal(String::new());
    let (npc_color, set_npc_color) = create_signal("#ef4444".to_string());
    let (npc_size, set_npc_size) = create_signal(1.0f32);

    // Feedback & Auto-save
    let (save_msg, set_save_msg) = create_signal(Option::<String>::None);
    let (_is_saving, set_is_saving) = create_signal(false);

    // Sincroniza configurações locais quando o mapa remoto muda
    create_effect(move |_| {
        let current = map_data.get();
        if !show_gm_modal.get() {
            set_cfg_shape.set(current.grid_shape);
            set_cfg_cols.set(current.cols);
            set_cfg_rows.set(current.rows);
            set_cfg_cell_size.set(current.cell_size);
            set_cfg_grid_color.set(current.grid_color);
            set_cfg_bg_url.set(current.bg_image_url);
            set_cfg_bg_opacity.set(current.bg_opacity);
            set_cfg_meters_per_cell.set(current.meters_per_cell);
        }
    });

    // Salvar Mapa no Servidor com Logs
    let save_map_to_server = move |data_to_save: RoomMapData| {
        let r_id = room_id.get_untracked();
        if r_id.is_empty() {
            grid_log("[BattleGrid] ⚠️ Aviso: room_id vazio, salvamento ignorado.");
            return;
        }

        set_is_saving.set(true);
        set_save_msg.set(Some("Salvando...".to_string()));
        grid_log(&format!(
            "[BattleGrid] 💾 Salvando mapa na sala '{}' ({} estruturas, {} tokens)...",
            r_id,
            data_to_save.structures.len(),
            data_to_save.tokens.len()
        ));

        spawn_local(async move {
            match update_room_map(r_id, data_to_save).await {
                Ok(_) => {
                    let _ = set_is_saving.try_set(false);
                    let _ = set_save_msg.try_set(Some("✓ Salvo".to_string()));
                    grid_log("[BattleGrid] ✓ Mapa salvo com sucesso no servidor!");
                }
                Err(e) => {
                    let _ = set_is_saving.try_set(false);
                    let _ = set_save_msg.try_set(Some(format!("Erro: {}", e)));
                    grid_log(&format!("[BattleGrid] ❌ Erro ao salvar mapa: {}", e));
                }
            }
        });
    };

    // Mover token por ID diretamente para célula
    let move_token_to_cell = move |token_id: &str, col: f32, row: f32| {
        let mut current = map_data.get_untracked();
        if let Some(token) = current.tokens.iter_mut().find(|t| t.id == token_id) {
            token.grid_col = col;
            token.grid_row = row;
            grid_log(&format!("[BattleGrid] 🎯 Token '{}' movido para ({:.0}, {:.0})", token.name, col, row));
            set_map_data.set(current.clone());
            save_map_to_server(current);
        }
    };

    // Remover Token
    let remove_token = move |token_id: String| {
        let mut current = map_data.get_untracked();
        let token_name = current.tokens.iter().find(|t| t.id == token_id).map(|t| t.name.clone()).unwrap_or_else(|| "Token".to_string());
        current.tokens.retain(|t| t.id != token_id);
        if selected_token_id.get_untracked().as_deref() == Some(&token_id) {
            set_selected_token_id.set(None);
        }
        grid_log(&format!("[BattleGrid] 🗑️ Token '{}' ({}) removido", token_name, token_id));
        set_map_data.set(current.clone());
        save_map_to_server(current);
    };

    // Alternar visibilidade de Token (GM)
    let toggle_token_hidden = move |token_id: String| {
        let mut current = map_data.get_untracked();
        if let Some(token) = current.tokens.iter_mut().find(|t| t.id == token_id) {
            token.is_hidden = !token.is_hidden;
            grid_log(&format!("[BattleGrid] 👁️ Token '{}' visibilidade alterada para {}", token.name, if token.is_hidden { "Oculto" } else { "Visível" }));
            set_map_data.set(current.clone());
            save_map_to_server(current);
        }
    };

    // Construir ou Demolir Estrutura na célula (col, row)
    let place_or_erase_structure = move |col: f32, row: f32, is_right_click: bool| {
        let mut current = map_data.get_untracked();
        let b_type = selected_build_type.get_untracked();

        if is_right_click || b_type == "eraser" {
            let before_count = current.structures.len();
            current.structures.retain(|s| !((s.grid_col - col).abs() < 0.1 && (s.grid_row - row).abs() < 0.1));
            if current.structures.len() != before_count {
                grid_log(&format!("[BattleGrid] 🧹 Estrutura demolida em ({:.0}, {:.0})", col, row));
                set_map_data.set(current.clone());
                save_map_to_server(current);
            }
            return;
        }

        if let Some(pos) = current.structures.iter().position(|s| (s.grid_col - col).abs() < 0.1 && (s.grid_row - row).abs() < 0.1) {
            if current.structures[pos].structure_type == b_type {
                current.structures.remove(pos);
                grid_log(&format!("[BattleGrid] 🧹 Estrutura '{}' removida (toggle) em ({:.0}, {:.0})", b_type, col, row));
            } else {
                let (name, color, icon, blocks_m, blocks_s, op) = match b_type.as_str() {
                    "door" => ("Porta", "#b45309", "🚪", false, true, 0.9),
                    "cover" => ("Barricada", "#475569", "🛡️", false, false, 0.75),
                    "water" => ("Água", "#0284c7", "🌊", false, false, 0.65),
                    "fire" => ("Fogo", "#ea580c", "🔥", false, false, 0.8),
                    "ward" => ("Selo Arcano", "#7c3aed", "🔯", false, false, 0.75),
                    _ => ("Muro", "#1e293b", "🧱", true, true, 0.95),
                };
                current.structures[pos].structure_type = b_type.clone();
                current.structures[pos].name = name.to_string();
                current.structures[pos].color = color.to_string();
                current.structures[pos].icon = icon.to_string();
                current.structures[pos].blocks_movement = blocks_m;
                current.structures[pos].blocks_sight = blocks_s;
                current.structures[pos].opacity = op;
                grid_log(&format!("[BattleGrid] 🔄 Estrutura substituída por '{}' em ({:.0}, {:.0})", b_type, col, row));
            }
        } else {
            let (name, color, icon, blocks_m, blocks_s, op) = match b_type.as_str() {
                "door" => ("Porta", "#b45309", "🚪", false, true, 0.9),
                "cover" => ("Barricada", "#475569", "🛡️", false, false, 0.75),
                "water" => ("Água", "#0284c7", "🌊", false, false, 0.65),
                "fire" => ("Fogo", "#ea580c", "🔥", false, false, 0.8),
                "ward" => ("Selo Arcano", "#7c3aed", "🔯", false, false, 0.75),
                _ => ("Muro", "#1e293b", "🧱", true, true, 0.95),
            };

            let new_struct = MapStructure {
                id: format!("struct-{}", uuid::Uuid::new_v4()),
                structure_type: b_type.clone(),
                name: name.to_string(),
                color: color.to_string(),
                icon: icon.to_string(),
                grid_col: col,
                grid_row: row,
                blocks_movement: blocks_m,
                blocks_sight: blocks_s,
                opacity: op,
            };
            current.structures.push(new_struct);
            grid_log(&format!("[BattleGrid] 🧱 Nova estrutura '{}' construída em ({:.0}, {:.0})", b_type, col, row));
        }

        set_map_data.set(current.clone());
        save_map_to_server(current);
    };

    // Ação de Arrastar / Pincelar sobre células
    let on_cell_drag_action = move |col: f32, row: f32| {
        let action = pointer_action.get_untracked();
        if action == "build" {
            place_or_erase_structure(col, row, false);
        } else if action == "erase" {
            place_or_erase_structure(col, row, true);
        }
    };

    // Puxar todos os personagens da sala para o grid (se ainda não existirem)
    let on_spawn_party = move |_| {
        let sheets = room_sheets.get_untracked();
        let mut current_map = map_data.get_untracked();
        let existing_sheet_ids: Vec<String> = current_map.tokens.iter().filter_map(|t| t.sheet_id.clone()).collect();

        let mut next_col = 0.0f32;
        let mut next_row = 0.0f32;

        for sheet in sheets {
            if !existing_sheet_ids.contains(&sheet.id) {
                let badge_color = if sheet.health_badge_class.contains("bruised") || sheet.health_badge_class.contains("hurt") {
                    "#eab308".to_string()
                } else if sheet.health_badge_class.contains("incapacitated") || sheet.health_badge_class.contains("crippled") {
                    "#ef4444".to_string()
                } else {
                    "#22c55e".to_string()
                };

                let new_token = MapToken {
                    id: format!("token-{}", uuid::Uuid::new_v4()),
                    sheet_id: Some(sheet.id.clone()),
                    name: sheet.name.clone(),
                    avatar_url: sheet.photo_url.clone(),
                    grid_col: next_col,
                    grid_row: next_row,
                    size_cells: 1.0,
                    is_npc: false,
                    is_hidden: sheet.is_hidden,
                    color: badge_color,
                    health_status: Some(sheet.health_label.clone()),
                };

                current_map.tokens.push(new_token);

                next_col += 1.0;
                if next_col >= current_map.cols as f32 {
                    next_col = 0.0;
                    next_row += 1.0;
                }
            }
        }

        set_map_data.set(current_map.clone());
        save_map_to_server(current_map);
    };

    // Adicionar NPC Token
    let on_add_npc = move |()| {
        let mut name = npc_name.get().trim().to_string();
        if name.is_empty() {
            let current_count = map_data.with(|m| m.tokens.iter().filter(|t| t.is_npc).count() + 1);
            name = format!("Inimigo #{}", current_count);
        }

        let mut current_map = map_data.get();
        let mut spawn_col = 0.0f32;
        let mut spawn_row = 0.0f32;
        for token in &current_map.tokens {
            if (token.grid_col - spawn_col).abs() < 0.1 && (token.grid_row - spawn_row).abs() < 0.1 {
                spawn_col += 1.0;
                if spawn_col >= current_map.cols as f32 {
                    spawn_col = 0.0;
                    spawn_row += 1.0;
                }
            }
        }

        let new_token = MapToken {
            id: format!("npc-{}", uuid::Uuid::new_v4()),
            sheet_id: None,
            name,
            avatar_url: String::new(),
            grid_col: spawn_col,
            grid_row: spawn_row,
            size_cells: npc_size.get(),
            is_npc: true,
            is_hidden: false,
            color: npc_color.get(),
            health_status: Some("NPC".to_string()),
        };

        current_map.tokens.push(new_token);
        set_map_data.set(current_map.clone());
        save_map_to_server(current_map);

        set_npc_name.set(String::new());
        set_show_npc_modal.set(false);
    };

    // Salvar configurações do GM
    let on_save_gm_config = move |()| {
        let mut current = map_data.get();
        current.grid_shape = cfg_shape.get();
        current.cols = cfg_cols.get().clamp(5, 60);
        current.rows = cfg_rows.get().clamp(5, 60);
        current.cell_size = cfg_cell_size.get().clamp(25, 120);
        current.grid_color = cfg_grid_color.get();
        current.bg_image_url = cfg_bg_url.get().trim().to_string();
        current.bg_opacity = cfg_bg_opacity.get().clamp(0.1, 1.0);
        current.meters_per_cell = cfg_meters_per_cell.get().clamp(0.5, 10.0);

        set_map_data.set(current.clone());
        save_map_to_server(current);
        set_show_gm_modal.set(false);
    };

    // Abrir Modal de Configurações
    let on_open_gm_modal = move |()| {
        let current = map_data.get_untracked();
        set_cfg_shape.set(current.grid_shape);
        set_cfg_cols.set(current.cols);
        set_cfg_rows.set(current.rows);
        set_cfg_cell_size.set(current.cell_size);
        set_cfg_grid_color.set(current.grid_color);
        set_cfg_bg_url.set(current.bg_image_url);
        set_cfg_bg_opacity.set(current.bg_opacity);
        set_cfg_meters_per_cell.set(current.meters_per_cell);
        set_show_gm_modal.set(true);
    };

    // Funções de Geometria e Projeção
    let cell_size_val = move || map_data.with(|m| m.cell_size as f32);
    let shape_val = move || map_data.with(|m| m.grid_shape);
    let cols_val = move || map_data.with(|m| m.cols);
    let rows_val = move || map_data.with(|m| m.rows);

    let board_dimensions = move || {
        calculate_board_dimensions(cols_val(), rows_val(), cell_size_val(), shape_val())
    };

    // Auto-Fit: dimensiona o tabuleiro para caber na tela sem rolagem
    let on_fit_to_screen = move |()| {
        let (board_w, board_h) = board_dimensions();
        if board_w > 0.0 && board_h > 0.0 {
            let fit_w = 960.0 / board_w;
            let fit_h = 600.0 / board_h;
            let best_scale = fit_w.min(fit_h).clamp(0.4, 1.25);
            set_zoom_scale.set(best_scale);
        }
    };

    let selected_token_name = Signal::derive(move || {
        let sel_id = selected_token_id.get();
        if let Some(id) = sel_id {
            map_data.with(|m| m.tokens.iter().find(|t| t.id == id).map(|t| t.name.clone()))
        } else {
            None
        }
    });

    let ruler_info = move || {
        if let (Some(start), Some(end)) = (ruler_start.get(), ruler_end.get()) {
            let cs = cell_size_val();
            let shape = shape_val();
            let meters = map_data.with(|m| m.meters_per_cell);
            Some(calculate_ruler_info(start, end, cs, shape, meters))
        } else {
            None
        }
    };

    view! {
        <div class="battle-grid-wrapper">
            <BattleGridToolbar
                active_tool=active_tool
                set_active_tool=set_active_tool
                selected_build_type=selected_build_type
                set_selected_build_type=set_selected_build_type
                selected_token_id=selected_token_id
                selected_token_name=selected_token_name
                zoom_scale=zoom_scale
                set_zoom_scale=set_zoom_scale
                save_msg=save_msg
                is_gm=is_gm
                grid_shape=Signal::derive(shape_val)
                cols=Signal::derive(cols_val)
                rows=Signal::derive(rows_val)
                on_spawn_party=Callback::new(move |()| on_spawn_party(()))
                on_open_npc_modal=Callback::new(move |()| set_show_npc_modal.set(true))
                on_open_gm_modal=Callback::new(on_open_gm_modal)
                on_fit_to_screen=Callback::new(on_fit_to_screen)
                on_remove_token=Callback::new(move |id| remove_token(id))
                on_reset_ruler=Callback::new(move |()| {
                    set_ruler_start.set(None);
                    set_ruler_end.set(None);
                })
            />

            // Display de Medição da Régua Tática
            {move || ruler_info().map(|(cells, meters, range_name, m20_sphere)| {
                view! {
                    <div class="ruler-banner">
                        <div class="ruler-info-col">
                            <span class="ruler-val">{format!("{:.1} m", meters)}</span>
                            <span class="ruler-sub">{format!("({:.1} células / {:.0} passos)", cells, meters / 0.8)}</span>
                        </div>
                        <div class="ruler-divider"></div>
                        <div class="ruler-range-col">
                            <span class="ruler-range-tag">{range_name}</span>
                            <span class="ruler-m20-tag">"🔮 " {m20_sphere}</span>
                        </div>
                    </div>
                }
            })}

            // Viewport do Mapa Interativo
            <div class="battle-grid-viewport">
                {move || {
                    let (board_w, board_h) = board_dimensions();
                    let cols = cols_val();
                    let rows = rows_val();
                    let cs = cell_size_val();
                    let shape = shape_val();
                    let grid_color = map_data.with(|m| m.grid_color.clone());
                    let bg_url = map_data.with(|m| m.bg_image_url.clone());
                    let bg_opacity = map_data.with(|m| m.bg_opacity);
                    let has_bg = !bg_url.is_empty();
                    let current_zoom = zoom_scale.get();

                    let container_style = format!(
                        "width: {}px; height: {}px; transform: scale({}); transform-origin: center center;",
                        board_w, board_h, current_zoom
                    );
                    let svg_style = format!("width: {}px; height: {}px;", board_w, board_h);

                    view! {
                        <div 
                            class="battle-board-container" 
                            style=container_style
                            on:pointerup=move |_| {
                                set_is_pointer_down.set(false);
                                set_pointer_action.set("none");
                            }
                            on:pointerleave=move |_| {
                                set_is_pointer_down.set(false);
                                set_pointer_action.set("none");
                                set_hovered_cell.set(None);
                            }
                        >
                            // Imagem de Fundo do Mapa
                            {if has_bg {
                                let bg_style = format!(
                                    "background-image: url('{}'); opacity: {}; width: {}px; height: {}px;",
                                    bg_url, bg_opacity, board_w, board_h
                                );
                                view! { <div class="battle-board-bg" style=bg_style></div> }.into_view()
                            } else {
                                view! { <div class="battle-board-bg-empty"></div> }.into_view()
                            }}

                            // Camada DOM de Estruturas Construídas
                            <div class="battle-structures-layer">
                                {move || {
                                    let structs = map_data.with(|m| m.structures.clone());
                                    let current_shape = shape_val();
                                    let cur_cs = cell_size_val();

                                    structs.into_iter().map(|s| {
                                        let (cx, cy) = calculate_cell_center(s.grid_col, s.grid_row, cur_cs, current_shape);
                                        let s_col = s.grid_col;
                                        let s_row = s.grid_row;
                                        let s_color = s.color.clone();
                                        let s_icon = s.icon.clone();
                                        let s_opacity = s.opacity;

                                        if current_shape == GridShape::Square {
                                            let left = s.grid_col * cur_cs + 1.0;
                                            let top = s.grid_row * cur_cs + 1.0;
                                            let size = cur_cs - 2.0;
                                            let font_sz = cur_cs * 0.48;

                                            let struct_style = format!(
                                                "left: {}px; top: {}px; width: {}px; height: {}px; background-color: {}; opacity: {}; font-size: {}px;",
                                                left, top, size, size, s_color, s_opacity, font_sz
                                            );

                                            view! {
                                                <div
                                                    class="map-structure-block"
                                                    style=struct_style
                                                    title=s.name.clone()
                                                    on:contextmenu=move |ev: ev::MouseEvent| {
                                                        ev.prevent_default();
                                                        ev.stop_propagation();
                                                        place_or_erase_structure(s_col, s_row, true);
                                                    }
                                                    on:click=move |ev: ev::MouseEvent| {
                                                        ev.stop_propagation();
                                                        let cur_tool = active_tool.get_untracked();
                                                        if cur_tool == "build" {
                                                            place_or_erase_structure(s_col, s_row, false);
                                                        }
                                                    }
                                                >
                                                    <span class="structure-icon">{s_icon}</span>
                                                </div>
                                            }.into_view()
                                        } else {
                                            let size = cur_cs * 0.88;
                                            let half = size / 2.0;
                                            let font_sz = cur_cs * 0.44;
                                            let struct_style = format!(
                                                "left: {}px; top: {}px; width: {}px; height: {}px; background-color: {}; opacity: {}; font-size: {}px;",
                                                cx - half, cy - half, size, size, s_color, s_opacity, font_sz
                                            );

                                            view! {
                                                <div
                                                    class="map-structure-block hex-structure"
                                                    style=struct_style
                                                    title=s.name.clone()
                                                    on:contextmenu=move |ev: ev::MouseEvent| {
                                                        ev.prevent_default();
                                                        ev.stop_propagation();
                                                        place_or_erase_structure(s_col, s_row, true);
                                                    }
                                                    on:click=move |ev: ev::MouseEvent| {
                                                        ev.stop_propagation();
                                                        let cur_tool = active_tool.get_untracked();
                                                        if cur_tool == "build" {
                                                            place_or_erase_structure(s_col, s_row, false);
                                                        }
                                                    }
                                                >
                                                    <span class="structure-icon">{s_icon}</span>
                                                </div>
                                            }.into_view()
                                        }
                                    }).collect_view()
                                }}
                            </div>

                            // Renderizador SVG da Grade
                            <svg class="battle-grid-svg" style=svg_style.clone()>
                                <defs>
                                    <pattern id="ruler-pattern" width="10" height="10" patternUnits="userSpaceOnUse">
                                        <line x1="0" y1="0" x2="10" y2="10" stroke="#f59e0b" stroke-width="1.5" />
                                    </pattern>
                                </defs>

                                // Células Interativas (Cliques, Hover e Linhas)
                                {match shape {
                                    GridShape::Square => {
                                        (0..rows).flat_map(|r| {
                                            let cur_tool = active_tool.clone();
                                            let cur_grid_color = grid_color.clone();

                                            (0..cols).map(move |c| {
                                                let cell_col = c as f32;
                                                let cell_row = r as f32;
                                                let x = cell_col * cs;
                                                let y = cell_row * cs;
                                                let stroke_color = cur_grid_color.clone();
                                                let tool_mode = cur_tool.clone();

                                                view! {
                                                    <rect
                                                        x=x
                                                        y=y
                                                        width=cs
                                                        height=cs
                                                        fill="transparent"
                                                        stroke=stroke_color
                                                        stroke-width="1"
                                                        class="grid-cell-interactive"
                                                        class:hovered=move || {
                                                            hovered_cell.get().map(|(hc, hr)| (hc - cell_col).abs() < 0.1 && (hr - cell_row).abs() < 0.1).unwrap_or(false)
                                                        }
                                                        on:pointerenter=move |_| {
                                                            set_hovered_cell.set(Some((cell_col, cell_row)));
                                                            if is_pointer_down.get_untracked() {
                                                                on_cell_drag_action(cell_col, cell_row);
                                                            }
                                                        }
                                                        on:pointerdown=move |ev: ev::PointerEvent| {
                                                            if ev.button() == 2 {
                                                                set_is_pointer_down.set(true);
                                                                set_pointer_action.set("erase");
                                                                place_or_erase_structure(cell_col, cell_row, true);
                                                            } else if ev.button() == 0 {
                                                                let current_tool = tool_mode.get_untracked();
                                                                if current_tool == "build" {
                                                                    set_is_pointer_down.set(true);
                                                                    let b_type = selected_build_type.get_untracked();
                                                                    set_pointer_action.set(if b_type == "eraser" { "erase" } else { "build" });
                                                                    place_or_erase_structure(cell_col, cell_row, false);
                                                                }
                                                            }
                                                        }
                                                        on:contextmenu=move |ev: ev::MouseEvent| {
                                                            ev.prevent_default();
                                                            place_or_erase_structure(cell_col, cell_row, true);
                                                        }
                                                        on:click=move |_| {
                                                            let current_tool = tool_mode.get_untracked();
                                                            if current_tool == "select" {
                                                                if let Some(t_id) = selected_token_id.get_untracked() {
                                                                    move_token_to_cell(&t_id, cell_col, cell_row);
                                                                    set_selected_token_id.set(None);
                                                                }
                                                            } else if current_tool == "ruler" {
                                                                if ruler_start.get_untracked().is_none() {
                                                                    set_ruler_start.set(Some((cell_col, cell_row)));
                                                                    set_ruler_end.set(None);
                                                                } else if ruler_end.get_untracked().is_none() {
                                                                    set_ruler_end.set(Some((cell_col, cell_row)));
                                                                } else {
                                                                    set_ruler_start.set(Some((cell_col, cell_row)));
                                                                    set_ruler_end.set(None);
                                                                }
                                                            }
                                                        }
                                                    />
                                                }
                                            })
                                        }).collect_view()
                                    }
                                    GridShape::HexPointy | GridShape::HexFlat => {
                                        (0..rows).flat_map(|r| {
                                            let cur_tool = active_tool.clone();
                                            let cur_grid_color = grid_color.clone();

                                            (0..cols).map(move |c| {
                                                let cell_col = c as f32;
                                                let cell_row = r as f32;
                                                let (cx, cy) = calculate_cell_center(cell_col, cell_row, cs, shape);
                                                let radius = cs / 2.0;
                                                let points_str = generate_hex_points(cx, cy, radius, shape);
                                                let stroke_color = cur_grid_color.clone();
                                                let tool_mode = cur_tool.clone();

                                                view! {
                                                    <polygon
                                                        points=points_str
                                                        fill="transparent"
                                                        stroke=stroke_color
                                                        stroke-width="1"
                                                        class="grid-cell-interactive hex-cell"
                                                        class:hovered=move || {
                                                            hovered_cell.get().map(|(hc, hr)| (hc - cell_col).abs() < 0.1 && (hr - cell_row).abs() < 0.1).unwrap_or(false)
                                                        }
                                                        on:pointerenter=move |_| {
                                                            set_hovered_cell.set(Some((cell_col, cell_row)));
                                                            if is_pointer_down.get_untracked() {
                                                                on_cell_drag_action(cell_col, cell_row);
                                                            }
                                                        }
                                                        on:pointerdown=move |ev: ev::PointerEvent| {
                                                            if ev.button() == 2 {
                                                                set_is_pointer_down.set(true);
                                                                set_pointer_action.set("erase");
                                                                place_or_erase_structure(cell_col, cell_row, true);
                                                            } else if ev.button() == 0 {
                                                                let current_tool = tool_mode.get_untracked();
                                                                if current_tool == "build" {
                                                                    set_is_pointer_down.set(true);
                                                                    let b_type = selected_build_type.get_untracked();
                                                                    set_pointer_action.set(if b_type == "eraser" { "erase" } else { "build" });
                                                                    place_or_erase_structure(cell_col, cell_row, false);
                                                                }
                                                            }
                                                        }
                                                        on:contextmenu=move |ev: ev::MouseEvent| {
                                                            ev.prevent_default();
                                                            place_or_erase_structure(cell_col, cell_row, true);
                                                        }
                                                        on:click=move |_| {
                                                            let current_tool = tool_mode.get_untracked();
                                                            if current_tool == "select" {
                                                                if let Some(t_id) = selected_token_id.get_untracked() {
                                                                    move_token_to_cell(&t_id, cell_col, cell_row);
                                                                    set_selected_token_id.set(None);
                                                                }
                                                            } else if current_tool == "ruler" {
                                                                if ruler_start.get_untracked().is_none() {
                                                                    set_ruler_start.set(Some((cell_col, cell_row)));
                                                                    set_ruler_end.set(None);
                                                                } else if ruler_end.get_untracked().is_none() {
                                                                    set_ruler_end.set(Some((cell_col, cell_row)));
                                                                } else {
                                                                    set_ruler_start.set(Some((cell_col, cell_row)));
                                                                    set_ruler_end.set(None);
                                                                }
                                                            }
                                                        }
                                                    />
                                                }
                                            })
                                        }).collect_view()
                                    }
                                }}

                                // Linha Visual da Régua
                                {move || {
                                    if let (Some(start), Some(end)) = (ruler_start.get(), ruler_end.get()) {
                                        let cs = cell_size_val();
                                        let shape = shape_val();
                                        let (x1, y1) = calculate_cell_center(start.0, start.1, cs, shape);
                                        let (x2, y2) = calculate_cell_center(end.0, end.1, cs, shape);

                                        view! {
                                            <g class="ruler-svg-group">
                                                <line 
                                                    x1=x1 y1=y1 x2=x2 y2=y2 
                                                    stroke="#f59e0b" 
                                                    stroke-width="3" 
                                                    stroke-dasharray="6,4"
                                                    class="ruler-line"
                                                />
                                                <circle cx=x1 cy=y1 r="6" fill="#f59e0b" class="ruler-endpoint" />
                                                <circle cx=x2 cy=y2 r="6" fill="#ef4444" class="ruler-endpoint" />
                                            </g>
                                        }.into_view()
                                    } else {
                                        view! {}.into_view()
                                    }
                                }}
                            </svg>

                            // Camada de Tokens (HTML sobre o SVG)
                            <div class="battle-tokens-layer">
                                {move || {
                                    let tokens = map_data.with(|m| m.tokens.clone());
                                    let user_is_gm = is_gm.get();
                                    let cur_shape = shape_val();
                                    let cur_cs = cell_size_val();

                                    tokens.into_iter().filter(|t| !t.is_hidden || user_is_gm).map(|token| {
                                        let (cx, cy) = calculate_cell_center(token.grid_col, token.grid_row, cur_cs, cur_shape);
                                        let t_size_cells = token.size_cells;
                                        let diameter = cur_cs * t_size_cells * 0.85;
                                        let radius = diameter / 2.0;

                                        let t_id = token.id.clone();
                                        let t_id_drag = token.id.clone();
                                        let t_id_toggle = token.id.clone();
                                        let t_id_del = token.id.clone();
                                        let is_hidden = token.is_hidden;

                                        let token_id_for_select = token.id.clone();
                                        let token_id_for_bubble = token.id.clone();

                                        let token_style = format!(
                                            "left: {}px; top: {}px; width: {}px; height: {}px; border-color: {};",
                                            cx - radius,
                                            cy - radius,
                                            diameter,
                                            diameter,
                                            token.color
                                        );

                                        view! {
                                            <div
                                                class="battle-token"
                                                class:selected=move || selected_token_id.get().as_deref() == Some(&token_id_for_select)
                                                class:hidden-gm=is_hidden
                                                class:is-npc=token.is_npc
                                                style=token_style
                                                title=format!("{} ({})", token.name, token.health_status.as_deref().unwrap_or("Ativo"))
                                                on:click=move |ev: ev::MouseEvent| {
                                                    ev.stop_propagation();
                                                    if active_tool.get_untracked() == "select" {
                                                        if selected_token_id.get_untracked().as_deref() == Some(&t_id) {
                                                            set_selected_token_id.set(None);
                                                        } else {
                                                            set_selected_token_id.set(Some(t_id.clone()));
                                                        }
                                                    }
                                                }
                                                draggable="true"
                                                on:dragstart=move |_| {
                                                    set_dragged_token_id.set(Some(t_id_drag.clone()));
                                                }
                                                on:dragend=move |_| {
                                                    if let (Some(t_id), Some((hc, hr))) = (dragged_token_id.get_untracked(), hovered_cell.get_untracked()) {
                                                        move_token_to_cell(&t_id, hc, hr);
                                                    }
                                                    set_dragged_token_id.set(None);
                                                }
                                            >
                                                {if !token.avatar_url.is_empty() {
                                                    view! {
                                                        <img 
                                                            src=token.avatar_url 
                                                            alt=token.name.clone() 
                                                            class="token-avatar-img" 
                                                        />
                                                    }.into_view()
                                                } else {
                                                    let initial = token.name.chars().next().unwrap_or('?').to_string();
                                                    view! {
                                                        <span class="token-initial">{initial}</span>
                                                    }.into_view()
                                                }}

                                                <span class="token-name-label">{token.name.clone()}</span>

                                                // Mini menu de ações do Token (Visível quando selecionado)
                                                {move || if selected_token_id.get().as_deref() == Some(&token_id_for_bubble) {
                                                    let t_id_tog = t_id_toggle.clone();
                                                    let t_id_del = t_id_del.clone();
                                                    view! {
                                                        <div class="token-action-bubble">
                                                            {if user_is_gm {
                                                                view! {
                                                                    <button
                                                                        type="button"
                                                                        class="token-act-btn hide-btn"
                                                                        on:click=move |ev: ev::MouseEvent| {
                                                                            ev.stop_propagation();
                                                                            toggle_token_hidden(t_id_tog.clone());
                                                                        }
                                                                        title=if is_hidden { "Tornar Visível aos Jogadores" } else { "Ocultar dos Jogadores" }
                                                                    >
                                                                        {if is_hidden { "👁️" } else { "🔒" }}
                                                                    </button>
                                                                }.into_view()
                                                            } else {
                                                                view! {}.into_view()
                                                            }}

                                                            <button
                                                                type="button"
                                                                class="token-act-btn del-btn"
                                                                on:click=move |ev: ev::MouseEvent| {
                                                                    ev.stop_propagation();
                                                                    remove_token(t_id_del.clone());
                                                                }
                                                                title="Remover Token do Mapa"
                                                            >
                                                                "✕"
                                                            </button>
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    view! {}.into_view()
                                                }}
                                            </div>
                                        }.into_view()
                                    }).collect_view()
                                }}
                            </div>
                        </div>
                    }
                }}
            </div>

            <GmConfigModal
                show=Signal::derive(move || show_gm_modal.get())
                cfg_shape=cfg_shape
                set_cfg_shape=set_cfg_shape
                cfg_cols=cfg_cols
                set_cfg_cols=set_cfg_cols
                cfg_rows=cfg_rows
                set_cfg_rows=set_cfg_rows
                cfg_cell_size=cfg_cell_size
                set_cfg_cell_size=set_cfg_cell_size
                cfg_meters_per_cell=cfg_meters_per_cell
                set_cfg_meters_per_cell=set_cfg_meters_per_cell
                cfg_bg_url=cfg_bg_url
                set_cfg_bg_url=set_cfg_bg_url
                cfg_bg_opacity=cfg_bg_opacity
                set_cfg_bg_opacity=set_cfg_bg_opacity
                on_close=Callback::new(move |()| set_show_gm_modal.set(false))
                on_save=Callback::new(on_save_gm_config)
            />

            <NpcCreateModal
                show=Signal::derive(move || show_npc_modal.get())
                npc_name=npc_name
                set_npc_name=set_npc_name
                npc_color=npc_color
                set_npc_color=set_npc_color
                npc_size=npc_size
                set_npc_size=set_npc_size
                on_close=Callback::new(move |()| set_show_npc_modal.set(false))
                on_create=Callback::new(on_add_npc)
            />
        </div>
    }
}
