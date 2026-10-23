//! Auxiliares do mapa usados pela interface (régua e posicionamento de tokens).

use crate::rooms::RoomMapData;
use crate::rules::map_grid::{self, Cell};

/// Medição da régua: (células, metros, faixa de distância, referência em M20).
pub fn ruler_info(map: &RoomMapData, a: Cell, b: Cell) -> (f32, f32, &'static str, &'static str) {
    let cells = map_grid::cell_distance(map.grid_shape, a, b);
    let meters = cells * map.meters_per_cell;
    let (range, m20) = if meters <= 1.5 {
        ("Toque", "Toque corporal / corpo a corpo")
    } else if meters <= 5.0 {
        ("Curto (1-3 passos)", "Proximidade imediata")
    } else if meters <= 20.0 {
        ("Médio (arremesso)", "Linha direta de visão")
    } else if meters <= 50.0 {
        ("Longo (armas de fogo)", "Alcance balístico")
    } else {
        ("Extremo (além de 50 m)", "Requer Correspondência 2+")
    };
    (cells, meters, range, m20)
}

/// Primeira célula livre (sem token e sem terreno bloqueante), varrendo a partir de `start`.
pub fn free_cell_near(map: &RoomMapData, start: Cell) -> Cell {
    let occupied = |c: Cell| {
        map.tokens.iter().any(|t| t.cell() == c) || map.terrain_at(c).is_some_and(|k| k.blocks_movement())
    };
    let total = (map.cols * map.rows) as i32;
    let cols = map.cols as i32;
    let start_index = start.row * cols + start.col;
    (0..total)
        .map(|i| (start_index + i).rem_euclid(total))
        .map(|i| Cell::new(i % cols, i / cols))
        .find(|c| !occupied(*c))
        .unwrap_or(start)
}

/// Células de um pincel de `size` (1 = só a célula; 2 = com as vizinhas; ...).
pub fn brush_cells(map: &RoomMapData, center: Cell, size: u8) -> Vec<Cell> {
    let radius = size.saturating_sub(1) as f32;
    let reach = size as i32;
    let mut cells = Vec::new();
    for row in center.row - reach..=center.row + reach {
        for col in center.col - reach..=center.col + reach {
            let cell = Cell::new(col, row);
            if map_grid::in_bounds(cell, map.cols, map.rows) && map_grid::cell_distance(map.grid_shape, center, cell) <= radius + 0.01 {
                cells.push(cell);
            }
        }
    }
    cells
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rooms::{MapToken, TerrainCell, TerrainKind};

    #[test]
    fn free_cell_skips_tokens_and_rock() {
        let mut map = RoomMapData { cols: 5, rows: 5, ..Default::default() };
        map.terrain.push(TerrainCell { cell: Cell::new(0, 0), kind: TerrainKind::Rock });
        map.tokens.push(MapToken {
            id: "t".into(), sheet_id: None, name: "A".into(), avatar_url: String::new(), grid_col: 1.0, grid_row: 0.0,
            size_cells: 1.0, is_npc: false, is_hidden: false, color: "#fff".into(), health_status: None,
        });
        assert_eq!(free_cell_near(&map, Cell::new(0, 0)), Cell::new(2, 0));
    }

    #[test]
    fn brush_sizes() {
        let map = RoomMapData { cols: 10, rows: 10, ..Default::default() };
        assert_eq!(brush_cells(&map, Cell::new(5, 5), 1).len(), 1);
        assert_eq!(brush_cells(&map, Cell::new(5, 5), 2).len(), 5, "cruz no quadrado (distância reta ≤ 1)");
        assert_eq!(brush_cells(&map, Cell::new(0, 0), 2).len(), 3, "corta na borda");
    }
}
