//! Desenho do mapa em `<canvas>`: fundo, terreno, grade, paredes, objetos e destaques.
//!
//! Um único canvas redesenhado por inteiro a cada mudança é muito mais leve que milhares
//! de elementos SVG/HTML (uma grade 60×60 tem 3.600 células). Tokens ficam em HTML por
//! cima, porque têm fotos e nomes.

use crate::rooms::{GridShape, MapProp, RoomMapData, TerrainKind, WallKind};
use crate::rules::map_grid::{self, Cell};

/// Maior área de canvas que o Safari do iPhone desenha (acima disso fica em branco).
const MAX_CANVAS_AREA: f64 = 16_000_000.0;
const MAX_CANVAS_SIDE: f64 = 16_000.0;

/// Destaques que dependem da interação (não fazem parte do mapa salvo).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Overlay {
    pub hover_cell: Option<Cell>,
    /// Ponto onde a parede vai encaixar (ferramenta Paredes)
    pub snap_point: Option<(f32, f32)>,
    /// Parede sendo desenhada: início, fim (pixels) e tipo
    pub wall_preview: Option<((f32, f32), (f32, f32), WallKind)>,
    /// Paredes marcadas pela borracha
    pub erasing_walls: Vec<String>,
    pub selected_prop: Option<String>,
    pub ruler: Option<(Cell, Cell)>,
    /// Objeto sendo arrastado: posição atual em pixels do tabuleiro
    pub dragging_prop: Option<(String, f32, f32)>,
}

/// Resolução do canvas: nítida em telas de alta densidade, sem passar do limite do iOS.
pub fn canvas_pixel_ratio(board_w: f32, board_h: f32, device_ratio: f64) -> f64 {
    let (w, h) = (board_w.max(1.0) as f64, board_h.max(1.0) as f64);
    let by_area = (MAX_CANVAS_AREA / (w * h)).sqrt();
    let by_side = MAX_CANVAS_SIDE / w.max(h);
    device_ratio.min(by_area).min(by_side).max(0.1)
}

/// Camada do mapa: só muda quando o mapa (ou a imagem de fundo) muda.
#[cfg(target_arch = "wasm32")]
pub fn draw_base(
    ctx: &web_sys::CanvasRenderingContext2d,
    map: &RoomMapData,
    background: Option<&web_sys::HtmlImageElement>,
    pixel_ratio: f64,
) {
    use wasm_bindgen::JsValue;

    let shape = map.grid_shape;
    let cs = map.cell_size as f32;
    let (bw, bh) = map_grid::board_size(shape, map.cols, map.rows, cs);
    let fill = |c: &str| ctx.set_fill_style(&JsValue::from_str(c));
    let stroke = |c: &str| ctx.set_stroke_style(&JsValue::from_str(c));
    let path_cell = |cell: Cell| {
        let corners = map_grid::cell_corners(shape, cs, cell);
        ctx.begin_path();
        for (i, (x, y)) in corners.iter().enumerate() {
            if i == 0 {
                ctx.move_to(*x as f64, *y as f64);
            } else {
                ctx.line_to(*x as f64, *y as f64);
            }
        }
        ctx.close_path();
    };

    let _ = ctx.set_transform(pixel_ratio, 0.0, 0.0, pixel_ratio, 0.0, 0.0);
    ctx.clear_rect(0.0, 0.0, bw as f64, bh as f64);

    // 1) Fundo
    fill("#ece6d6");
    ctx.fill_rect(0.0, 0.0, bw as f64, bh as f64);
    if let Some(img) = background.filter(|i| i.complete() && i.natural_width() > 0) {
        ctx.set_global_alpha(map.bg_opacity as f64);
        let scale = map.bg_scale as f64;
        let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(
            img,
            map.bg_offset_x as f64,
            map.bg_offset_y as f64,
            img.natural_width() as f64 * scale,
            img.natural_height() as f64 * scale,
        );
        ctx.set_global_alpha(1.0);
    }

    // 2) Terreno
    for t in &map.terrain {
        path_cell(t.cell);
        fill(t.kind.color());
        ctx.fill();
        let (cx, cy) = map_grid::cell_center(shape, cs, t.cell);
        if t.kind.is_difficult() {
            // Hachura discreta indica terreno difícil
            stroke("rgba(0,0,0,0.18)");
            ctx.set_line_width(1.0);
            ctx.save();
            path_cell(t.cell);
            ctx.clip();
            let r = cs as f64;
            let mut k = -r;
            while k < r {
                ctx.begin_path();
                ctx.move_to(cx as f64 + k - r / 2.0, cy as f64 - r / 2.0);
                ctx.line_to(cx as f64 + k + r / 2.0, cy as f64 + r / 2.0);
                ctx.stroke();
                k += r / 4.0;
            }
            ctx.restore();
        }
        if t.kind == TerrainKind::Node {
            fill("rgba(255,255,255,0.55)");
            ctx.begin_path();
            let _ = ctx.arc(cx as f64, cy as f64, (cs * 0.12) as f64, 0.0, std::f64::consts::TAU);
            ctx.fill();
        }
    }

    // 3) Linhas da grade
    if map.show_grid {
        stroke(&map.grid_color);
        ctx.set_line_width(1.0);
        if shape == GridShape::Square {
            ctx.begin_path();
            for c in 0..=map.cols {
                let x = (c as f32 * cs) as f64;
                ctx.move_to(x, 0.0);
                ctx.line_to(x, bh as f64);
            }
            for r in 0..=map.rows {
                let y = (r as f32 * cs) as f64;
                ctx.move_to(0.0, y);
                ctx.line_to(bw as f64, y);
            }
            ctx.stroke();
        } else {
            for row in 0..map.rows as i32 {
                for col in 0..map.cols as i32 {
                    path_cell(Cell::new(col, row));
                    ctx.stroke();
                }
            }
        }
    }

    // 4) Paredes, portas e janelas
    ctx.set_line_cap("round");
    for w in &map.segments {
        draw_wall(ctx, w.a.to_px(cs), w.b.to_px(cs), cs, w.kind, 1.0);
    }

    // 5) Objetos
    for prop in &map.props {
        draw_prop(ctx, prop, map_grid::cell_center(shape, cs, prop.cell), cs, false);
    }
}

/// Camada de destaques: muda a cada movimento do ponteiro, por isso é separada e leve.
#[cfg(target_arch = "wasm32")]
pub fn draw_overlay(ctx: &web_sys::CanvasRenderingContext2d, map: &RoomMapData, overlay: &Overlay, pixel_ratio: f64) {
    use wasm_bindgen::JsValue;

    let shape = map.grid_shape;
    let cs = map.cell_size as f32;
    let (bw, bh) = map_grid::board_size(shape, map.cols, map.rows, cs);
    let fill = |c: &str| ctx.set_fill_style(&JsValue::from_str(c));
    let stroke = |c: &str| ctx.set_stroke_style(&JsValue::from_str(c));
    let path_cell = |cell: Cell| {
        let corners = map_grid::cell_corners(shape, cs, cell);
        ctx.begin_path();
        for (i, (x, y)) in corners.iter().enumerate() {
            if i == 0 {
                ctx.move_to(*x as f64, *y as f64);
            } else {
                ctx.line_to(*x as f64, *y as f64);
            }
        }
        ctx.close_path();
    };

    let _ = ctx.set_transform(pixel_ratio, 0.0, 0.0, pixel_ratio, 0.0, 0.0);
    ctx.clear_rect(0.0, 0.0, bw as f64, bh as f64);
    ctx.set_line_cap("round");

    if let Some(prop) = overlay.selected_prop.as_ref().and_then(|id| map.props.iter().find(|p| &p.id == id)) {
        let (x, y) = map_grid::cell_center(shape, cs, prop.cell);
        draw_prop(ctx, prop, (x, y), cs, true);
    }
    if let Some((id, x, y)) = &overlay.dragging_prop {
        if let Some(prop) = map.props.iter().find(|p| &p.id == id) {
            ctx.set_global_alpha(0.75);
            draw_prop(ctx, prop, (*x, *y), cs, true);
            ctx.set_global_alpha(1.0);
        }
    }
    if let Some(cell) = overlay.hover_cell.filter(|c| map_grid::in_bounds(*c, map.cols, map.rows)) {
        path_cell(cell);
        stroke("rgba(99,102,241,0.95)");
        ctx.set_line_width(2.0);
        ctx.stroke();
    }
    for w in map.segments.iter().filter(|w| overlay.erasing_walls.contains(&w.id)) {
        let ((x1, y1), (x2, y2)) = (w.a.to_px(cs), w.b.to_px(cs));
        stroke("rgba(239,68,68,0.85)");
        ctx.set_line_width((cs as f64 * 0.2).max(5.0));
        ctx.begin_path();
        ctx.move_to(x1 as f64, y1 as f64);
        ctx.line_to(x2 as f64, y2 as f64);
        ctx.stroke();
    }
    if let Some((a, b, kind)) = overlay.wall_preview {
        draw_wall(ctx, a, b, cs, kind, 0.75);
    }
    for (x, y) in overlay.wall_preview.map(|(a, b, _)| vec![a, b]).unwrap_or_default().into_iter().chain(overlay.snap_point) {
        fill("#6366f1");
        ctx.begin_path();
        let _ = ctx.arc(x as f64, y as f64, 5.0, 0.0, std::f64::consts::TAU);
        ctx.fill();
    }
    if let Some((a, b)) = overlay.ruler {
        let (x1, y1) = map_grid::cell_center(shape, cs, a);
        let (x2, y2) = map_grid::cell_center(shape, cs, b);
        stroke("#f59e0b");
        ctx.set_line_width(3.0);
        let _ = ctx.set_line_dash(&js_sys::Array::of2(&JsValue::from(8.0), &JsValue::from(6.0)));
        ctx.begin_path();
        ctx.move_to(x1 as f64, y1 as f64);
        ctx.line_to(x2 as f64, y2 as f64);
        ctx.stroke();
        let _ = ctx.set_line_dash(&js_sys::Array::new());
        for ((x, y), color) in [((x1, y1), "#f59e0b"), ((x2, y2), "#ef4444")] {
            fill(color);
            ctx.begin_path();
            let _ = ctx.arc(x as f64, y as f64, 6.0, 0.0, std::f64::consts::TAU);
            ctx.fill();
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn draw_wall(ctx: &web_sys::CanvasRenderingContext2d, (x1, y1): (f32, f32), (x2, y2): (f32, f32), cs: f32, kind: WallKind, alpha: f64) {
    use wasm_bindgen::JsValue;
    let (x1, y1, x2, y2) = (x1 as f64, y1 as f64, x2 as f64, y2 as f64);
    let thick = (cs as f64 * 0.12).max(3.0);
    ctx.set_global_alpha(alpha);
    let line = |color: &str, width: f64, from: f64, to: f64| {
        ctx.set_stroke_style(&JsValue::from_str(color));
        ctx.set_line_width(width);
        ctx.begin_path();
        ctx.move_to(x1 + (x2 - x1) * from, y1 + (y2 - y1) * from);
        ctx.line_to(x1 + (x2 - x1) * to, y1 + (y2 - y1) * to);
        ctx.stroke();
    };
    match kind {
        WallKind::Wall => line("#1f2937", thick, 0.0, 1.0),
        WallKind::Window => {
            line("#1f2937", thick, 0.0, 1.0);
            line("#7dd3fc", thick * 0.45, 0.2, 0.8);
        }
        WallKind::Door => {
            line("#1f2937", thick, 0.0, 0.18);
            line("#1f2937", thick, 0.82, 1.0);
            line("#92400e", thick * 0.85, 0.2, 0.8);
        }
        WallKind::DoorOpen => {
            line("#1f2937", thick, 0.0, 0.18);
            line("#1f2937", thick, 0.82, 1.0);
            // Folha da porta girada 90° a partir da dobradiça
            let (hx, hy) = (x1 + (x2 - x1) * 0.2, y1 + (y2 - y1) * 0.2);
            let (dx, dy) = ((x2 - x1) * 0.6, (y2 - y1) * 0.6);
            ctx.set_stroke_style(&JsValue::from_str("#b45309"));
            ctx.set_line_width(thick * 0.5);
            ctx.begin_path();
            ctx.move_to(hx, hy);
            ctx.line_to(hx - dy, hy + dx);
            ctx.stroke();
        }
    }
    ctx.set_global_alpha(1.0);
}

#[cfg(target_arch = "wasm32")]
fn draw_prop(ctx: &web_sys::CanvasRenderingContext2d, prop: &MapProp, (cx, cy): (f32, f32), cs: f32, selected: bool) {
    use wasm_bindgen::JsValue;
    let size = (cs * 0.72 * prop.scale) as f64;
    ctx.save();
    let _ = ctx.translate(cx as f64, cy as f64);
    if selected {
        ctx.set_stroke_style(&JsValue::from_str("#6366f1"));
        ctx.set_line_width(3.0);
        ctx.begin_path();
        let _ = ctx.arc(0.0, 0.0, size * 0.72, 0.0, std::f64::consts::TAU);
        ctx.stroke();
    }
    let _ = ctx.rotate((prop.rotation as f64).to_radians());
    ctx.set_font(&format!("{}px \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Noto Color Emoji\", sans-serif", size.round()));
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");
    let _ = ctx.fill_text(prop.kind.icon(), 0.0, size * 0.04);
    ctx.restore();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_resolution_respects_ios_limits() {
        // Mapa enorme (80 × 120 px = 9.600 px de lado) numa tela 3x: precisa reduzir
        let ratio = canvas_pixel_ratio(9600.0, 9600.0, 3.0);
        assert!(9600.0 * ratio * 9600.0 * ratio <= MAX_CANVAS_AREA + 1.0);
        // Mapa pequeno mantém a nitidez da tela
        assert_eq!(canvas_pixel_ratio(500.0, 500.0, 2.0), 2.0);
    }
}
