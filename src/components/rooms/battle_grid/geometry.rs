use crate::rooms::GridShape;

/// Calcula as dimensões totais (largura, altura) em pixels do tabuleiro baseado no shape e tamanho das células.
pub fn calculate_board_dimensions(cols: u32, rows: u32, cs: f32, shape: GridShape) -> (f32, f32) {
    let cols = cols as f32;
    let rows = rows as f32;

    match shape {
        GridShape::Square => {
            let w = cols * cs;
            let h = rows * cs;
            (w, h)
        }
        GridShape::HexPointy => {
            let r = cs / 2.0;
            let w_hex = 1.7320508 * r; // sqrt(3) * R
            let total_w = cols * w_hex + (w_hex / 2.0);
            let total_h = rows * (1.5 * r) + (r * 0.5);
            (total_w, total_h)
        }
        GridShape::HexFlat => {
            let r = cs / 2.0;
            let h_hex = 1.7320508 * r;
            let total_w = cols * (1.5 * r) + (r * 0.5);
            let total_h = rows * h_hex + (h_hex / 2.0);
            (total_w, total_h)
        }
    }
}

/// Cálculo do centro de uma célula (col, row) em pixels -> (px_x, px_y)
pub fn calculate_cell_center(col: f32, row: f32, cs: f32, shape: GridShape) -> (f32, f32) {
    match shape {
        GridShape::Square => {
            let cx = col * cs + (cs / 2.0);
            let cy = row * cs + (cs / 2.0);
            (cx, cy)
        }
        GridShape::HexPointy => {
            let r = cs / 2.0;
            let w_hex = 1.7320508 * r;
            let offset_x = if (row as i32) % 2 != 0 { w_hex / 2.0 } else { 0.0 };
            let cx = col * w_hex + offset_x + (w_hex / 2.0);
            let cy = row * (1.5 * r) + r;
            (cx, cy)
        }
        GridShape::HexFlat => {
            let r = cs / 2.0;
            let h_hex = 1.7320508 * r;
            let offset_y = if (col as i32) % 2 != 0 { h_hex / 2.0 } else { 0.0 };
            let cx = col * (1.5 * r) + r;
            let cy = row * h_hex + offset_y + (h_hex / 2.0);
            (cx, cy)
        }
    }
}

/// Gera string de pontos para polígono SVG de hexágono
pub fn generate_hex_points(cx: f32, cy: f32, r: f32, shape: GridShape) -> String {
    let mut pts = String::new();
    for i in 0..6 {
        let angle_deg = match shape {
            GridShape::HexPointy => 60.0 * (i as f32) - 30.0,
            _ => 60.0 * (i as f32),
        };
        let angle_rad = angle_deg.to_radians();
        let px = cx + r * angle_rad.cos();
        let py = cy + r * angle_rad.sin();
        if i > 0 {
            pts.push(' ');
        }
        pts.push_str(&format!("{:.1},{:.1}", px, py));
    }
    pts
}

/// Medição da Régua Tática: calcula distância em células, metros e faixa canônica de M20
pub fn calculate_ruler_info(
    start: (f32, f32),
    end: (f32, f32),
    cs: f32,
    shape: GridShape,
    meters_per_cell: f32,
) -> (f32, f32, &'static str, &'static str) {
    let (x1, y1) = calculate_cell_center(start.0, start.1, cs, shape);
    let (x2, y2) = calculate_cell_center(end.0, end.1, cs, shape);
    let dx = x2 - x1;
    let dy = y2 - y1;
    let dist_px = (dx * dx + dy * dy).sqrt();
    let cells = dist_px / cs;
    let meters = cells * meters_per_cell;

    let (range_name, m20_sphere) = if meters <= 1.5 {
        ("Toque", "Toque Corporal / Corpo a Corpo")
    } else if meters <= 5.0 {
        ("Curto (1-3 Passos)", "Proximidade Imediata")
    } else if meters <= 20.0 {
        ("Médio (Arremesso)", "Linha Direta de Visão")
    } else if meters <= 50.0 {
        ("Longo (Armas de Fogo)", "Alcance Balístico")
    } else {
        ("Extremo (Além de 50m)", "Requer Correspondência 2+")
    };

    (cells, meters, range_name, m20_sphere)
}
