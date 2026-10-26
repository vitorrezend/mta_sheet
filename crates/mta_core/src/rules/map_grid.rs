//! Geometria da grade do mapa tático: quadrada e hexagonal.
//!
//! Convenções (iguais às do grid antigo, para não deslocar mapas salvos):
//! - quadrado: célula (col, row) começa em (col·tam, row·tam);
//! - hexágono "pontudo" (`HexPointy`): linhas ímpares deslocadas meia célula para a direita;
//! - hexágono "chato" (`HexFlat`): colunas ímpares deslocadas meia célula para baixo;
//! - o "raio" do hexágono é metade do tamanho de célula configurado.
//!
//! Lados (usados pelas paredes):
//! - quadrado: 0 = norte, 1 = leste, 2 = sul, 3 = oeste;
//! - hexágono: 6 lados, numerados no sentido horário a partir do leste (pontudo)
//!   ou do sudeste (chato). Um lado é compartilhado por duas células; [`canonical_edge`]
//!   escolhe uma única representação para ele.

use serde::{Deserialize, Serialize};

/// Formato da grade do mapa tático.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GridShape {
    #[default]
    #[serde(rename = "square")]
    Square,
    #[serde(rename = "hex_pointy")]
    HexPointy,
    #[serde(rename = "hex_flat")]
    HexFlat,
}

impl GridShape {
    pub fn label(self) -> &'static str {
        match self {
            Self::Square => "Quadrada",
            Self::HexPointy => "Hexagonal (pontas em cima)",
            Self::HexFlat => "Hexagonal (lados em cima)",
        }
    }
}


const SQRT3: f32 = 1.732_050_8;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cell {
    pub col: i32,
    pub row: i32,
}

impl Cell {
    pub const fn new(col: i32, row: i32) -> Self {
        Self { col, row }
    }
}

/// Um lado de célula (onde ficam paredes e portas).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Edge {
    pub cell: Cell,
    pub side: u8,
}

pub fn side_count(shape: GridShape) -> u8 {
    match shape {
        GridShape::Square => 4,
        GridShape::HexPointy | GridShape::HexFlat => 6,
    }
}

pub fn opposite(shape: GridShape, side: u8) -> u8 {
    let n = side_count(shape);
    (side + n / 2) % n
}

pub fn in_bounds(cell: Cell, cols: u32, rows: u32) -> bool {
    cell.col >= 0 && cell.row >= 0 && (cell.col as u32) < cols && (cell.row as u32) < rows
}

/// Largura e altura totais do tabuleiro em pixels.
pub fn board_size(shape: GridShape, cols: u32, rows: u32, cell_size: f32) -> (f32, f32) {
    let (cols, rows) = (cols as f32, rows as f32);
    let r = cell_size / 2.0;
    match shape {
        GridShape::Square => (cols * cell_size, rows * cell_size),
        GridShape::HexPointy => {
            let w = SQRT3 * r;
            (cols * w + w / 2.0, rows * 1.5 * r + r / 2.0)
        }
        GridShape::HexFlat => {
            let h = SQRT3 * r;
            (cols * 1.5 * r + r / 2.0, rows * h + h / 2.0)
        }
    }
}

/// Centro da célula em pixels.
pub fn cell_center(shape: GridShape, cell_size: f32, cell: Cell) -> (f32, f32) {
    let (c, r_) = (cell.col as f32, cell.row as f32);
    let r = cell_size / 2.0;
    match shape {
        GridShape::Square => (c * cell_size + r, r_ * cell_size + r),
        GridShape::HexPointy => {
            let w = SQRT3 * r;
            let shift = if cell.row.rem_euclid(2) == 1 { w / 2.0 } else { 0.0 };
            (c * w + shift + w / 2.0, r_ * 1.5 * r + r)
        }
        GridShape::HexFlat => {
            let h = SQRT3 * r;
            let shift = if cell.col.rem_euclid(2) == 1 { h / 2.0 } else { 0.0 };
            (c * 1.5 * r + r, r_ * h + shift + h / 2.0)
        }
    }
}

/// Vértices da célula, no sentido horário. O lado `i` liga o vértice `i` ao `i + 1`.
pub fn cell_corners(shape: GridShape, cell_size: f32, cell: Cell) -> Vec<(f32, f32)> {
    let (cx, cy) = cell_center(shape, cell_size, cell);
    let r = cell_size / 2.0;
    match shape {
        GridShape::Square => vec![(cx - r, cy - r), (cx + r, cy - r), (cx + r, cy + r), (cx - r, cy + r)],
        GridShape::HexPointy | GridShape::HexFlat => {
            let start = if shape == GridShape::HexPointy { -30.0f32 } else { 0.0 };
            (0..6)
                .map(|i| {
                    let a = (start + 60.0 * i as f32).to_radians();
                    (cx + r * a.cos(), cy + r * a.sin())
                })
                .collect()
        }
    }
}

/// Extremidades de um lado da célula.
pub fn edge_endpoints(shape: GridShape, cell_size: f32, edge: Edge) -> ((f32, f32), (f32, f32)) {
    let corners = cell_corners(shape, cell_size, edge.cell);
    let n = corners.len();
    let i = edge.side as usize % n;
    (corners[i], corners[(i + 1) % n])
}

/// Célula vizinha através de um lado.
pub fn neighbor(shape: GridShape, cell: Cell, side: u8) -> Cell {
    let Cell { col: c, row: r } = cell;
    match shape {
        GridShape::Square => match side % 4 {
            0 => Cell::new(c, r - 1),
            1 => Cell::new(c + 1, r),
            2 => Cell::new(c, r + 1),
            _ => Cell::new(c - 1, r),
        },
        // Lados: 0 L, 1 SE, 2 SO, 3 O, 4 NO, 5 NE
        GridShape::HexPointy => {
            let odd = r.rem_euclid(2) == 1;
            match side % 6 {
                0 => Cell::new(c + 1, r),
                1 => Cell::new(if odd { c + 1 } else { c }, r + 1),
                2 => Cell::new(if odd { c } else { c - 1 }, r + 1),
                3 => Cell::new(c - 1, r),
                4 => Cell::new(if odd { c } else { c - 1 }, r - 1),
                _ => Cell::new(if odd { c + 1 } else { c }, r - 1),
            }
        }
        // Lados: 0 SE, 1 S, 2 SO, 3 NO, 4 N, 5 NE
        GridShape::HexFlat => {
            let odd = c.rem_euclid(2) == 1;
            match side % 6 {
                0 => Cell::new(c + 1, if odd { r + 1 } else { r }),
                1 => Cell::new(c, r + 1),
                2 => Cell::new(c - 1, if odd { r + 1 } else { r }),
                3 => Cell::new(c - 1, if odd { r } else { r - 1 }),
                4 => Cell::new(c, r - 1),
                _ => Cell::new(c + 1, if odd { r } else { r - 1 }),
            }
        }
    }
}

/// Representação única de um lado compartilhado por duas células: fica com a célula
/// "menor" (linha, depois coluna). Assim a mesma parede nunca é gravada duas vezes.
pub fn canonical_edge(shape: GridShape, cell: Cell, side: u8) -> Edge {
    let side = side % side_count(shape);
    let other = neighbor(shape, cell, side);
    let mine = Edge { cell, side };
    let theirs = Edge { cell: other, side: opposite(shape, side) };
    if (other.row, other.col) < (cell.row, cell.col) { theirs } else { mine }
}

/// Lado da célula mais próximo do ponto (para desenhar paredes tocando perto da borda).
pub fn nearest_side(shape: GridShape, cell_size: f32, cell: Cell, x: f32, y: f32) -> u8 {
    let n = side_count(shape) as usize;
    (0..n)
        .min_by(|&a, &b| {
            let mid = |i: usize| {
                let ((x1, y1), (x2, y2)) = edge_endpoints(shape, cell_size, Edge { cell, side: i as u8 });
                let (mx, my) = ((x1 + x2) / 2.0, (y1 + y2) / 2.0);
                (mx - x).powi(2) + (my - y).powi(2)
            };
            mid(a).total_cmp(&mid(b))
        })
        .unwrap_or(0) as u8
}

/// Distância do ponto ao lado mais próximo, em pixels (perto o bastante = mirando a borda).
pub fn distance_to_side(shape: GridShape, cell_size: f32, edge: Edge, x: f32, y: f32) -> f32 {
    let ((x1, y1), (x2, y2)) = edge_endpoints(shape, cell_size, edge);
    let (dx, dy) = (x2 - x1, y2 - y1);
    let len2 = dx * dx + dy * dy;
    let t = if len2 > 0.0 { (((x - x1) * dx + (y - y1) * dy) / len2).clamp(0.0, 1.0) } else { 0.0 };
    let (px, py) = (x1 + t * dx, y1 + t * dy);
    ((x - px).powi(2) + (y - py).powi(2)).sqrt()
}

/// Arredonda coordenadas cúbicas fracionárias para o hexágono mais próximo.
fn cube_round(q: f32, r: f32) -> (i32, i32) {
    let s = -q - r;
    let (mut rq, mut rr, rs) = (q.round(), r.round(), s.round());
    let (dq, dr, ds) = ((rq - q).abs(), (rr - r).abs(), (rs - s).abs());
    if dq > dr && dq > ds {
        rq = -rr - rs;
    } else if dr > ds {
        rr = -rq - rs;
    }
    (rq as i32, rr as i32)
}

/// Célula sob um ponto em pixels (pode estar fora do tabuleiro; confira com [`in_bounds`]).
pub fn pixel_to_cell(shape: GridShape, cell_size: f32, x: f32, y: f32) -> Cell {
    let r = cell_size / 2.0;
    match shape {
        GridShape::Square => Cell::new((x / cell_size).floor() as i32, (y / cell_size).floor() as i32),
        GridShape::HexPointy => {
            let w = SQRT3 * r;
            let (px, py) = (x - w / 2.0, y - r);
            let q = (SQRT3 / 3.0 * px - py / 3.0) / r;
            let ar = (2.0 / 3.0 * py) / r;
            let (q, ar) = cube_round(q, ar);
            Cell::new(q + (ar - (ar & 1)) / 2, ar)
        }
        GridShape::HexFlat => {
            let h = SQRT3 * r;
            let (px, py) = (x - r, y - h / 2.0);
            let q = (2.0 / 3.0 * px) / r;
            let ar = (-px / 3.0 + SQRT3 / 3.0 * py) / r;
            let (q, ar) = cube_round(q, ar);
            Cell::new(q, ar + (q - (q & 1)) / 2)
        }
    }
}

/// Distância entre duas células, em células (quadrado: linha reta; hexágono: passos).
pub fn cell_distance(shape: GridShape, a: Cell, b: Cell) -> f32 {
    let to_cube = |c: Cell| -> (i32, i32, i32) {
        let (q, r) = match shape {
            GridShape::HexPointy => (c.col - (c.row - (c.row & 1)) / 2, c.row),
            _ => (c.col, c.row - (c.col - (c.col & 1)) / 2),
        };
        (q, r, -q - r)
    };
    match shape {
        GridShape::Square => (((a.col - b.col).pow(2) + (a.row - b.row).pow(2)) as f32).sqrt(),
        _ => {
            let (a, b) = (to_cube(a), to_cube(b));
            (((a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()) / 2) as f32
        }
    }
}

/// Ponto de encaixe mais próximo para as pontas de uma parede, em pixels.
///
/// Quadrado: cantos da grade (as paredes seguem as linhas). Hexágono: cantos **e** centros
/// dos hexágonos, o que permite linhas retas que atravessam o meio das células.
pub fn snap_point(shape: GridShape, cell_size: f32, x: f32, y: f32) -> (f32, f32) {
    match shape {
        GridShape::Square => ((x / cell_size).round() * cell_size, (y / cell_size).round() * cell_size),
        GridShape::HexPointy | GridShape::HexFlat => {
            let here = pixel_to_cell(shape, cell_size, x, y);
            let mut best = cell_center(shape, cell_size, here);
            let mut best_d = (best.0 - x).hypot(best.1 - y);
            let cells = std::iter::once(here).chain((0..6).map(|side| neighbor(shape, here, side)));
            for cell in cells {
                let candidates = std::iter::once(cell_center(shape, cell_size, cell)).chain(cell_corners(shape, cell_size, cell));
                for (px, py) in candidates {
                    let d = (px - x).hypot(py - y);
                    if d < best_d {
                        best = (px, py);
                        best_d = d;
                    }
                }
            }
            best
        }
    }
}

/// Distância de um ponto a um segmento de reta.
pub fn point_segment_distance(p: (f32, f32), a: (f32, f32), b: (f32, f32)) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len2 = dx * dx + dy * dy;
    let t = if len2 > 0.0 { (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / len2).clamp(0.0, 1.0) } else { 0.0 };
    (p.0 - (a.0 + t * dx)).hypot(p.1 - (a.1 + t * dy))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHAPES: [GridShape; 3] = [GridShape::Square, GridShape::HexPointy, GridShape::HexFlat];

    #[test]
    fn pixel_to_cell_inverts_cell_center() {
        for shape in SHAPES {
            for row in 0..12 {
                for col in 0..12 {
                    let cell = Cell::new(col, row);
                    let (x, y) = cell_center(shape, 50.0, cell);
                    assert_eq!(pixel_to_cell(shape, 50.0, x, y), cell, "{shape:?} {cell:?}");
                    // Um ponto um pouco fora do centro continua na mesma célula
                    assert_eq!(pixel_to_cell(shape, 50.0, x + 6.0, y - 5.0), cell, "{shape:?} {cell:?} deslocado");
                }
            }
        }
    }

    #[test]
    fn neighbors_share_the_edge_both_ways() {
        for shape in SHAPES {
            for row in 0..6 {
                for col in 0..6 {
                    let cell = Cell::new(col, row);
                    for side in 0..side_count(shape) {
                        let other = neighbor(shape, cell, side);
                        assert_eq!(neighbor(shape, other, opposite(shape, side)), cell, "{shape:?} {cell:?} lado {side}");
                        // O lado é o mesmo segmento visto das duas células
                        let (a1, a2) = edge_endpoints(shape, 50.0, Edge { cell, side });
                        let (b1, b2) = edge_endpoints(shape, 50.0, Edge { cell: other, side: opposite(shape, side) });
                        let close = |p: (f32, f32), q: (f32, f32)| (p.0 - q.0).abs() < 0.01 && (p.1 - q.1).abs() < 0.01;
                        assert!((close(a1, b2) && close(a2, b1)) || (close(a1, b1) && close(a2, b2)), "{shape:?} {cell:?} lado {side}");
                        assert_eq!(canonical_edge(shape, cell, side), canonical_edge(shape, other, opposite(shape, side)));
                    }
                }
            }
        }
    }

    #[test]
    fn nearest_side_follows_the_pointer() {
        let cell = Cell::new(3, 3);
        let (cx, cy) = cell_center(GridShape::Square, 50.0, cell);
        assert_eq!(nearest_side(GridShape::Square, 50.0, cell, cx, cy - 20.0), 0);
        assert_eq!(nearest_side(GridShape::Square, 50.0, cell, cx + 20.0, cy), 1);
        assert_eq!(nearest_side(GridShape::Square, 50.0, cell, cx - 20.0, cy + 2.0), 3);
        let (hx, hy) = cell_center(GridShape::HexPointy, 50.0, cell);
        assert_eq!(nearest_side(GridShape::HexPointy, 50.0, cell, hx + 20.0, hy), 0, "leste do hexágono pontudo");
    }

    #[test]
    fn walls_snap_to_corners_and_hex_centers() {
        // Quadrado: sempre no canto mais próximo da grade
        assert_eq!(snap_point(GridShape::Square, 50.0, 123.0, 76.0), (100.0, 100.0));
        assert_eq!(snap_point(GridShape::Square, 50.0, 137.0, 24.0), (150.0, 0.0));
        // Hexágono: perto do centro, encaixa no centro; perto de um canto, no canto
        for shape in [GridShape::HexPointy, GridShape::HexFlat] {
            let cell = Cell::new(4, 3);
            let (cx, cy) = cell_center(shape, 50.0, cell);
            assert_eq!(snap_point(shape, 50.0, cx + 3.0, cy - 2.0), (cx, cy), "{shape:?} centro");
            let corner = cell_corners(shape, 50.0, cell)[2];
            let snapped = snap_point(shape, 50.0, corner.0 + 1.5, corner.1 - 1.0);
            assert!((snapped.0 - corner.0).abs() < 0.01 && (snapped.1 - corner.1).abs() < 0.01, "{shape:?} canto");
        }
        assert_eq!(point_segment_distance((5.0, 3.0), (0.0, 0.0), (10.0, 0.0)), 3.0);
        assert_eq!(point_segment_distance((14.0, 0.0), (0.0, 0.0), (10.0, 0.0)), 4.0);
    }

    #[test]
    fn distances() {
        assert_eq!(cell_distance(GridShape::Square, Cell::new(0, 0), Cell::new(3, 4)), 5.0);
        for shape in [GridShape::HexPointy, GridShape::HexFlat] {
            let c = Cell::new(4, 4);
            for side in 0..6 {
                assert_eq!(cell_distance(shape, c, neighbor(shape, c, side)), 1.0, "{shape:?} lado {side}");
            }
        }
    }
}
