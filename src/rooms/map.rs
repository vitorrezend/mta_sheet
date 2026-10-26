//! Mapa tático montável da sala: grade (quadrada ou hexagonal), terreno pintado,
//! paredes e portas nos lados das células, objetos, imagem de fundo e tokens.
//!
//! Toda edição é uma [`MapOp`] pequena ("pinte estas células", "mova este token"). O
//! servidor valida, aplica com controle de versão e repassa a mesma operação para quem
//! está na sala, que aplica localmente com o mesmo código ([`RoomMapData::apply`]).

use serde::{Deserialize, Serialize};
use crate::rules::map_grid::{self, Cell, Edge};

pub const MIN_GRID_CELLS: u32 = 5;
pub const MAX_GRID_CELLS: u32 = 80;
pub const MIN_CELL_SIZE: u32 = 24;
pub const MAX_CELL_SIZE: u32 = 120;
/// Limite de operações por envio (uma pincelada grande ainda é uma única operação)
pub const MAX_OPS_PER_BATCH: usize = 64;
/// Limite de células/lados por operação de pintura
pub const MAX_CELLS_PER_OP: usize = 4096;

pub use mta_core::rules::map_grid::GridShape;

// ---------------------------------------------------------------------------
// Terreno
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum TerrainKind {
    Floor,
    Stone,
    Wood,
    Grass,
    Sand,
    Snow,
    Water,
    DeepWater,
    Mud,
    Rubble,
    Lava,
    Fire,
    Pit,
    Darkness,
    Rock,
    Node,
}

impl TerrainKind {
    pub const ALL: [TerrainKind; 16] = [
        Self::Floor, Self::Stone, Self::Wood, Self::Grass, Self::Sand, Self::Snow, Self::Water, Self::DeepWater,
        Self::Mud, Self::Rubble, Self::Lava, Self::Fire, Self::Pit, Self::Darkness, Self::Rock, Self::Node,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Floor => "Piso",
            Self::Stone => "Pedra",
            Self::Wood => "Madeira",
            Self::Grass => "Grama",
            Self::Sand => "Areia",
            Self::Snow => "Neve",
            Self::Water => "Água rasa",
            Self::DeepWater => "Água funda",
            Self::Mud => "Lama",
            Self::Rubble => "Escombros",
            Self::Lava => "Lava",
            Self::Fire => "Fogo",
            Self::Pit => "Abismo",
            Self::Darkness => "Escuridão",
            Self::Rock => "Rocha maciça",
            Self::Node => "Nodo",
        }
    }

    pub fn color(self) -> &'static str {
        match self {
            Self::Floor => "#d6c7a8",
            Self::Stone => "#8b8f98",
            Self::Wood => "#9a6b3f",
            Self::Grass => "#5f9c4a",
            Self::Sand => "#e2c98f",
            Self::Snow => "#eef3f8",
            Self::Water => "#5fa8d3",
            Self::DeepWater => "#245d8f",
            Self::Mud => "#6b5236",
            Self::Rubble => "#7a6f63",
            Self::Lava => "#d9480f",
            Self::Fire => "#f59f00",
            Self::Pit => "#111111",
            Self::Darkness => "#2b2440",
            Self::Rock => "#3d3d45",
            Self::Node => "#7c3aed",
        }
    }

    /// Impede a passagem (não dá para parar um token aqui sem ser mestre)
    pub fn blocks_movement(self) -> bool {
        matches!(self, Self::Rock | Self::Pit)
    }

    /// Terreno difícil: conta em dobro para movimento (indicado no desenho)
    pub fn is_difficult(self) -> bool {
        matches!(self, Self::Mud | Self::Rubble | Self::Snow | Self::Water | Self::Sand)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerrainCell {
    pub cell: Cell,
    pub kind: TerrainKind,
}

// ---------------------------------------------------------------------------
// Paredes e portas (nos lados das células)
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum WallKind {
    Wall,
    Door,
    DoorOpen,
    Window,
}

impl WallKind {
    pub const ALL: [WallKind; 4] = [Self::Wall, Self::Door, Self::DoorOpen, Self::Window];

    pub fn label(self) -> &'static str {
        match self {
            Self::Wall => "Parede",
            Self::Door => "Porta fechada",
            Self::DoorOpen => "Porta aberta",
            Self::Window => "Janela",
        }
    }

    pub fn blocks_movement(self) -> bool {
        matches!(self, Self::Wall | Self::Door | Self::Window)
    }
}

/// Parede do formato anterior (um lado de célula). Só é lida para conversão; ver
/// [`RoomMapData::normalize`].
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct WallEdge {
    pub edge: Edge,
    pub kind: WallKind,
}

/// Ponto de uma parede em **unidades de célula** (pixels ÷ tamanho da célula). Assim a
/// parede continua no lugar quando o tamanho da célula muda.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct WallPoint {
    pub x: f32,
    pub y: f32,
}

impl WallPoint {
    pub fn from_px(x: f32, y: f32, cell_size: f32) -> Self {
        Self { x: x / cell_size, y: y / cell_size }
    }
    pub fn to_px(self, cell_size: f32) -> (f32, f32) {
        (self.x * cell_size, self.y * cell_size)
    }
}

/// Parede, porta ou janela: um segmento reto entre dois pontos.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WallSegment {
    pub id: String,
    pub a: WallPoint,
    pub b: WallPoint,
    pub kind: WallKind,
}

/// Limite de segmentos de parede por mapa
pub const MAX_WALL_SEGMENTS: usize = 4000;

// ---------------------------------------------------------------------------
// Objetos
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PropKind {
    Table,
    Chair,
    Bed,
    Shelf,
    Chest,
    Barrel,
    Crate,
    Door,
    Barricade,
    Column,
    Statue,
    Altar,
    RitualCircle,
    Ward,
    Portal,
    Tree,
    Bush,
    Boulder,
    Campfire,
    Trap,
    Computer,
    Car,
}

impl PropKind {
    pub const ALL: [PropKind; 22] = [
        Self::Table, Self::Chair, Self::Bed, Self::Shelf, Self::Chest, Self::Barrel, Self::Crate, Self::Door,
        Self::Barricade, Self::Column, Self::Statue, Self::Altar, Self::RitualCircle, Self::Ward, Self::Portal,
        Self::Tree, Self::Bush, Self::Boulder, Self::Campfire, Self::Trap, Self::Computer, Self::Car,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Table => "Mesa",
            Self::Chair => "Cadeira",
            Self::Bed => "Cama",
            Self::Shelf => "Estante",
            Self::Chest => "Baú",
            Self::Barrel => "Barril",
            Self::Crate => "Caixote",
            Self::Door => "Porta avulsa",
            Self::Barricade => "Barricada",
            Self::Column => "Coluna",
            Self::Statue => "Estátua",
            Self::Altar => "Altar",
            Self::RitualCircle => "Círculo ritual",
            Self::Ward => "Selo arcano",
            Self::Portal => "Portal",
            Self::Tree => "Árvore",
            Self::Bush => "Arbusto",
            Self::Boulder => "Pedregulho",
            Self::Campfire => "Fogueira",
            Self::Trap => "Armadilha",
            Self::Computer => "Terminal",
            Self::Car => "Carro",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Table => "🪑",
            Self::Chair => "💺",
            Self::Bed => "🛏️",
            Self::Shelf => "📚",
            Self::Chest => "🧰",
            Self::Barrel => "🛢️",
            Self::Crate => "📦",
            Self::Door => "🚪",
            Self::Barricade => "🛡️",
            Self::Column => "🏛️",
            Self::Statue => "🗿",
            Self::Altar => "⛩️",
            Self::RitualCircle => "🔮",
            Self::Ward => "🔯",
            Self::Portal => "🌀",
            Self::Tree => "🌳",
            Self::Bush => "🌿",
            Self::Boulder => "🪨",
            Self::Campfire => "🔥",
            Self::Trap => "⚠️",
            Self::Computer => "💻",
            Self::Car => "🚗",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MapProp {
    pub id: String,
    pub kind: PropKind,
    pub cell: Cell,
    /// Rotação em graus (múltiplos de 90 na grade quadrada, de 60 na hexagonal)
    #[serde(default)]
    pub rotation: u16,
    /// Tamanho em células (1 = uma célula)
    #[serde(default = "default_prop_scale")]
    pub scale: f32,
}

fn default_prop_scale() -> f32 {
    1.0
}

// ---------------------------------------------------------------------------
// Tokens (personagens e NPCs)
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MapToken {
    pub id: String,
    #[serde(default)]
    pub sheet_id: Option<String>,
    pub name: String,
    #[serde(default)]
    pub avatar_url: String,
    #[serde(default)]
    pub grid_col: f32,
    #[serde(default)]
    pub grid_row: f32,
    #[serde(default = "default_token_size")]
    pub size_cells: f32,
    #[serde(default)]
    pub is_npc: bool,
    #[serde(default)]
    pub is_hidden: bool,
    #[serde(default = "default_token_color")]
    pub color: String,
    #[serde(default)]
    pub health_status: Option<String>,
}

pub fn default_token_size() -> f32 {
    1.0
}
pub fn default_token_color() -> String {
    "#6366f1".to_string()
}

impl MapToken {
    pub fn cell(&self) -> Cell {
        Cell::new(self.grid_col.round() as i32, self.grid_row.round() as i32)
    }
}

/// Estrutura do grid antigo (bloco ocupando uma célula). Só é lida para converter
/// mapas salvos antes da versão montável; ver [`RoomMapData::normalize`].
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MapStructure {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub structure_type: String,
    #[serde(default)]
    pub grid_col: f32,
    #[serde(default)]
    pub grid_row: f32,
}

// ---------------------------------------------------------------------------
// Mapa
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GridConfig {
    pub grid_shape: GridShape,
    pub cols: u32,
    pub rows: u32,
    pub cell_size: u32,
    pub meters_per_cell: f32,
    pub grid_color: String,
    pub show_grid: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BackgroundConfig {
    pub url: String,
    pub opacity: f32,
    /// Deslocamento da imagem em pixels do tabuleiro (para alinhar a grade)
    pub offset_x: f32,
    pub offset_y: f32,
    /// Escala da imagem (1 = tamanho natural)
    pub scale: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RoomMapData {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub grid_shape: GridShape,
    #[serde(default = "default_map_cols")]
    pub cols: u32,
    #[serde(default = "default_map_rows")]
    pub rows: u32,
    #[serde(default = "default_map_cell_size")]
    pub cell_size: u32,
    #[serde(default = "default_map_grid_color")]
    pub grid_color: String,
    #[serde(default = "default_true")]
    pub show_grid: bool,
    #[serde(default = "default_map_meters_per_cell")]
    pub meters_per_cell: f32,
    #[serde(default)]
    pub bg_image_url: String,
    #[serde(default = "default_map_bg_opacity")]
    pub bg_opacity: f32,
    #[serde(default)]
    pub bg_offset_x: f32,
    #[serde(default)]
    pub bg_offset_y: f32,
    #[serde(default = "default_one")]
    pub bg_scale: f32,
    #[serde(default)]
    pub terrain: Vec<TerrainCell>,
    #[serde(default)]
    pub segments: Vec<WallSegment>,
    /// Somente leitura do formato anterior (lados de célula); convertido por `normalize`
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub walls: Vec<WallEdge>,
    #[serde(default)]
    pub props: Vec<MapProp>,
    #[serde(default)]
    pub tokens: Vec<MapToken>,
    /// Somente leitura de mapas antigos; convertido e esvaziado por `normalize`
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub structures: Vec<MapStructure>,
}

fn default_true() -> bool {
    true
}
fn default_one() -> f32 {
    1.0
}
pub fn default_map_cols() -> u32 {
    20
}
pub fn default_map_rows() -> u32 {
    20
}
pub fn default_map_cell_size() -> u32 {
    50
}
pub fn default_map_grid_color() -> String {
    "rgba(30, 41, 59, 0.45)".to_string()
}
pub fn default_map_bg_opacity() -> f32 {
    1.0
}
pub fn default_map_meters_per_cell() -> f32 {
    1.5
}

impl Default for RoomMapData {
    fn default() -> Self {
        Self {
            enabled: true,
            grid_shape: GridShape::Square,
            cols: default_map_cols(),
            rows: default_map_rows(),
            cell_size: default_map_cell_size(),
            grid_color: default_map_grid_color(),
            show_grid: true,
            meters_per_cell: default_map_meters_per_cell(),
            bg_image_url: String::new(),
            bg_opacity: default_map_bg_opacity(),
            bg_offset_x: 0.0,
            bg_offset_y: 0.0,
            bg_scale: 1.0,
            terrain: Vec::new(),
            segments: Vec::new(),
            walls: Vec::new(),
            props: Vec::new(),
            tokens: Vec::new(),
            structures: Vec::new(),
        }
    }
}

/// Camada que pode ser limpa de uma vez.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MapLayer {
    Terrain,
    Walls,
    Props,
    Npcs,
}

/// Uma edição do mapa. Só `MoveToken` pode ser feita por jogadores (no próprio token).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "op", content = "args", rename_all = "snake_case")]
pub enum MapOp {
    /// `kind: None` apaga o terreno das células
    PaintTerrain { cells: Vec<Cell>, kind: Option<TerrainKind> },
    /// Desenha uma parede/porta/janela reta
    AddWall(WallSegment),
    /// Apaga paredes pelo id
    RemoveWalls { ids: Vec<String> },
    /// Abre/fecha uma porta
    ToggleDoor { id: String },
    PlaceProp(MapProp),
    MoveProp { id: String, cell: Cell },
    RotateProp { id: String, rotation: u16 },
    ResizeProp { id: String, scale: f32 },
    RemoveProp { id: String },
    AddToken(MapToken),
    MoveToken { id: String, cell: Cell },
    SetTokenHidden { id: String, hidden: bool },
    RemoveToken { id: String },
    Configure(GridConfig),
    SetBackground(BackgroundConfig),
    ClearLayer { layer: MapLayer },
}

impl MapOp {
    /// Operações de montagem: só o mestre. Jogadores apenas movem o próprio token.
    pub fn requires_gm(&self) -> bool {
        !matches!(self, Self::MoveToken { .. } | Self::ToggleDoor { .. })
    }

    /// Envolve tokens ocultos (o servidor não repassa os detalhes a todos)
    pub fn touches_hidden_token(&self, map: &RoomMapData) -> bool {
        let hidden = |id: &str| map.tokens.iter().any(|t| t.id == id && t.is_hidden);
        match self {
            Self::AddToken(t) => t.is_hidden,
            Self::MoveToken { id, .. } | Self::RemoveToken { id } => hidden(id),
            Self::SetTokenHidden { .. } => true,
            _ => false,
        }
    }
}

impl RoomMapData {
    /// Confere se o usuário pode fazer a operação. Mestre: tudo. Jogador: só mover o
    /// token de uma ficha sua (nunca para rocha ou abismo) e abrir/fechar portas.
    pub fn authorize(&self, op: &MapOp, is_gm: bool, my_sheet_ids: &[String]) -> Result<(), String> {
        if is_gm {
            return Ok(());
        }
        if op.requires_gm() {
            return Err("Só o mestre pode montar o mapa".into());
        }
        if let MapOp::MoveToken { id, cell } = op {
            let token = self.tokens.iter().find(|t| &t.id == id).ok_or("Token não encontrado")?;
            if !token.sheet_id.as_ref().is_some_and(|s| my_sheet_ids.contains(s)) {
                return Err("Você só pode mover o token do seu personagem".into());
            }
            if self.terrain_at(*cell).is_some_and(|k| k.blocks_movement()) {
                return Err("Não dá para entrar nessa célula".into());
            }
        }
        Ok(())
    }

    fn cell_ok(&self, cell: Cell) -> bool {
        map_grid::in_bounds(cell, self.cols, self.rows)
    }

    /// Parede mais próxima de um ponto (em pixels), se estiver a menos de `max_px`.
    pub fn wall_near(&self, x: f32, y: f32, max_px: f32) -> Option<&WallSegment> {
        let cs = self.cell_size as f32;
        self.segments
            .iter()
            .map(|w| (w, map_grid::point_segment_distance((x, y), w.a.to_px(cs), w.b.to_px(cs))))
            .filter(|(_, d)| *d <= max_px)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(w, _)| w)
    }

    pub fn terrain_at(&self, cell: Cell) -> Option<TerrainKind> {
        self.terrain.iter().find(|t| t.cell == cell).map(|t| t.kind)
    }

    /// Aplica uma operação. Erros indicam operação inválida (fora do mapa, valor absurdo);
    /// ids inexistentes são ignorados para que operações repetidas sejam inofensivas.
    pub fn apply(&mut self, op: &MapOp) -> Result<(), String> {
        let shape = self.grid_shape;
        match op {
            MapOp::PaintTerrain { cells, kind } => {
                if cells.len() > MAX_CELLS_PER_OP {
                    return Err("Pincelada grande demais".into());
                }
                let valid: Vec<Cell> = cells.iter().copied().filter(|c| self.cell_ok(*c)).collect();
                for cell in valid {
                    self.terrain.retain(|t| t.cell != cell);
                    if let Some(kind) = kind {
                        self.terrain.push(TerrainCell { cell, kind: *kind });
                    }
                }
            }
            MapOp::AddWall(wall) => {
                if self.segments.len() >= MAX_WALL_SEGMENTS {
                    return Err("Limite de paredes do mapa atingido".into());
                }
                let (bw, bh) = map_grid::board_size(shape, self.cols, self.rows, 1.0);
                let inside = |p: WallPoint| p.x.is_finite() && p.y.is_finite() && (-0.01..=bw + 0.01).contains(&p.x) && (-0.01..=bh + 0.01).contains(&p.y);
                if !inside(wall.a) || !inside(wall.b) {
                    return Err("Parede fora do mapa".into());
                }
                if (wall.a.x - wall.b.x).hypot(wall.a.y - wall.b.y) < 0.05 {
                    return Err("Parede curta demais".into());
                }
                self.segments.retain(|w| w.id != wall.id);
                self.segments.push(wall.clone());
            }
            MapOp::RemoveWalls { ids } => self.segments.retain(|w| !ids.contains(&w.id)),
            MapOp::ToggleDoor { id } => {
                if let Some(w) = self.segments.iter_mut().find(|w| &w.id == id) {
                    w.kind = match w.kind {
                        WallKind::Door => WallKind::DoorOpen,
                        WallKind::DoorOpen => WallKind::Door,
                        other => other,
                    };
                }
            }
            MapOp::PlaceProp(prop) => {
                if !self.cell_ok(prop.cell) {
                    return Err("Objeto fora do mapa".into());
                }
                let mut prop = prop.clone();
                prop.scale = prop.scale.clamp(0.5, 4.0);
                prop.rotation %= 360;
                self.props.retain(|p| p.id != prop.id);
                self.props.push(prop);
            }
            MapOp::MoveProp { id, cell } => {
                if !self.cell_ok(*cell) {
                    return Err("Destino fora do mapa".into());
                }
                if let Some(p) = self.props.iter_mut().find(|p| &p.id == id) {
                    p.cell = *cell;
                }
            }
            MapOp::RotateProp { id, rotation } => {
                if let Some(p) = self.props.iter_mut().find(|p| &p.id == id) {
                    p.rotation = rotation % 360;
                }
            }
            MapOp::ResizeProp { id, scale } => {
                if let Some(p) = self.props.iter_mut().find(|p| &p.id == id) {
                    p.scale = scale.clamp(0.5, 4.0);
                }
            }
            MapOp::RemoveProp { id } => self.props.retain(|p| &p.id != id),
            MapOp::AddToken(token) => {
                let mut token = token.clone();
                let cell = token.cell();
                if !self.cell_ok(cell) {
                    return Err("Token fora do mapa".into());
                }
                token.size_cells = token.size_cells.clamp(0.5, 4.0);
                self.tokens.retain(|t| t.id != token.id);
                self.tokens.push(token);
            }
            MapOp::MoveToken { id, cell } => {
                if !self.cell_ok(*cell) {
                    return Err("Destino fora do mapa".into());
                }
                if let Some(t) = self.tokens.iter_mut().find(|t| &t.id == id) {
                    t.grid_col = cell.col as f32;
                    t.grid_row = cell.row as f32;
                }
            }
            MapOp::SetTokenHidden { id, hidden } => {
                if let Some(t) = self.tokens.iter_mut().find(|t| &t.id == id) {
                    t.is_hidden = *hidden;
                }
            }
            MapOp::RemoveToken { id } => self.tokens.retain(|t| &t.id != id),
            MapOp::Configure(cfg) => {
                let shape_changed = cfg.grid_shape != self.grid_shape;
                self.grid_shape = cfg.grid_shape;
                self.cols = cfg.cols.clamp(MIN_GRID_CELLS, MAX_GRID_CELLS);
                self.rows = cfg.rows.clamp(MIN_GRID_CELLS, MAX_GRID_CELLS);
                self.cell_size = cfg.cell_size.clamp(MIN_CELL_SIZE, MAX_CELL_SIZE);
                self.meters_per_cell = cfg.meters_per_cell.clamp(0.1, 100.0);
                self.grid_color = cfg.grid_color.chars().take(64).collect();
                self.show_grid = cfg.show_grid;
                // Paredes são segmentos livres: continuam no lugar mesmo trocando o formato
                let _ = shape_changed;
                self.normalize();
            }
            MapOp::SetBackground(bg) => {
                let url = bg.url.trim();
                let allowed = url.is_empty() || url.starts_with("/uploads/") || url.starts_with("https://");
                if !allowed || url.len() > 2048 {
                    return Err("Endereço de imagem não permitido".into());
                }
                self.bg_image_url = url.to_string();
                self.bg_opacity = bg.opacity.clamp(0.05, 1.0);
                self.bg_offset_x = bg.offset_x.clamp(-10_000.0, 10_000.0);
                self.bg_offset_y = bg.offset_y.clamp(-10_000.0, 10_000.0);
                self.bg_scale = bg.scale.clamp(0.05, 20.0);
            }
            MapOp::ClearLayer { layer } => match layer {
                MapLayer::Terrain => self.terrain.clear(),
                MapLayer::Walls => self.segments.clear(),
                MapLayer::Props => self.props.clear(),
                MapLayer::Npcs => self.tokens.retain(|t| !t.is_npc),
            },
        }
        Ok(())
    }

    /// Ajusta um mapa lido do banco: converte o formato antigo, aplica limites e remove
    /// o que ficou fora do tabuleiro (por exemplo, depois de diminuir a grade).
    pub fn normalize(&mut self) {
        for s in std::mem::take(&mut self.structures) {
            let cell = Cell::new(s.grid_col.round() as i32, s.grid_row.round() as i32);
            let id = if s.id.is_empty() { format!("legacy-{}-{}", cell.col, cell.row) } else { s.id.clone() };
            let prop = |kind| MapProp { id: id.clone(), kind, cell, rotation: 0, scale: 1.0 };
            match s.structure_type.as_str() {
                "water" => self.terrain.push(TerrainCell { cell, kind: TerrainKind::Water }),
                "fire" => self.terrain.push(TerrainCell { cell, kind: TerrainKind::Fire }),
                "door" => self.props.push(prop(PropKind::Door)),
                "cover" => self.props.push(prop(PropKind::Barricade)),
                "ward" => self.props.push(prop(PropKind::Ward)),
                _ => self.terrain.push(TerrainCell { cell, kind: TerrainKind::Rock }),
            }
        }

        self.cols = self.cols.clamp(MIN_GRID_CELLS, MAX_GRID_CELLS);
        self.rows = self.rows.clamp(MIN_GRID_CELLS, MAX_GRID_CELLS);
        self.cell_size = self.cell_size.clamp(MIN_CELL_SIZE, MAX_CELL_SIZE);
        let (cols, rows, shape) = (self.cols, self.rows, self.grid_shape);

        self.terrain.retain(|t| map_grid::in_bounds(t.cell, cols, rows));
        self.props.retain(|p| map_grid::in_bounds(p.cell, cols, rows));
        // Paredes do formato anterior (lados de célula) viram segmentos
        let mut seen = std::collections::HashSet::new();
        for w in std::mem::take(&mut self.walls) {
            let edge = map_grid::canonical_edge(shape, w.edge.cell, w.edge.side);
            if !seen.insert(edge) {
                continue;
            }
            let ((x1, y1), (x2, y2)) = map_grid::edge_endpoints(shape, 1.0, edge);
            self.segments.push(WallSegment {
                id: format!("edge-{}-{}-{}", edge.cell.col, edge.cell.row, edge.side),
                a: WallPoint { x: x1, y: y1 },
                b: WallPoint { x: x2, y: y2 },
                kind: w.kind,
            });
        }
        let (bw, bh) = map_grid::board_size(shape, cols, rows, 1.0);
        let on_board = |p: WallPoint| p.x >= -0.01 && p.y >= -0.01 && p.x <= bw + 0.01 && p.y <= bh + 0.01;
        self.segments.retain(|w| on_board(w.a) && on_board(w.b));
        for t in &mut self.tokens {
            t.grid_col = t.grid_col.round().clamp(0.0, (cols - 1) as f32);
            t.grid_row = t.grid_row.round().clamp(0.0, (rows - 1) as f32);
        }
    }

    /// Cópia para um jogador: sem tokens ocultos pelo mestre.
    pub fn visible_to_players(&self) -> Self {
        let mut copy = self.clone();
        copy.tokens.retain(|t| !t.is_hidden);
        copy
    }

    pub fn config(&self) -> GridConfig {
        GridConfig {
            grid_shape: self.grid_shape,
            cols: self.cols,
            rows: self.rows,
            cell_size: self.cell_size,
            meters_per_cell: self.meters_per_cell,
            grid_color: self.grid_color.clone(),
            show_grid: self.show_grid,
        }
    }

    pub fn background(&self) -> BackgroundConfig {
        BackgroundConfig {
            url: self.bg_image_url.clone(),
            opacity: self.bg_opacity,
            offset_x: self.bg_offset_x,
            offset_y: self.bg_offset_y,
            scale: self.bg_scale,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map() -> RoomMapData {
        RoomMapData { cols: 10, rows: 10, ..Default::default() }
    }

    #[test]
    fn paint_and_erase_terrain() {
        let mut m = map();
        m.apply(&MapOp::PaintTerrain { cells: vec![Cell::new(1, 1), Cell::new(2, 1), Cell::new(99, 99)], kind: Some(TerrainKind::Water) }).expect("pintar");
        assert_eq!(m.terrain.len(), 2, "célula fora do mapa é ignorada");
        m.apply(&MapOp::PaintTerrain { cells: vec![Cell::new(1, 1)], kind: Some(TerrainKind::Lava) }).expect("repintar");
        assert_eq!(m.terrain_at(Cell::new(1, 1)), Some(TerrainKind::Lava));
        assert_eq!(m.terrain.len(), 2, "repintar substitui, não duplica");
        m.apply(&MapOp::PaintTerrain { cells: vec![Cell::new(1, 1)], kind: None }).expect("apagar");
        assert_eq!(m.terrain_at(Cell::new(1, 1)), None);
    }

    fn wall(id: &str, a: (f32, f32), b: (f32, f32), kind: WallKind) -> WallSegment {
        WallSegment { id: id.into(), a: WallPoint { x: a.0, y: a.1 }, b: WallPoint { x: b.0, y: b.1 }, kind }
    }

    #[test]
    fn straight_walls_and_doors() {
        let mut m = map();
        // Uma linha reta de 8 células é UMA parede, não vários pedaços
        m.apply(&MapOp::AddWall(wall("w1", (1.0, 2.0), (9.0, 2.0), WallKind::Wall))).expect("parede");
        assert_eq!(m.segments.len(), 1);
        m.apply(&MapOp::AddWall(wall("d1", (3.0, 5.0), (4.0, 5.0), WallKind::Door))).expect("porta");
        m.apply(&MapOp::ToggleDoor { id: "d1".into() }).expect("abrir");
        assert_eq!(m.segments[1].kind, WallKind::DoorOpen);
        // Acha a parede perto de um ponto em pixels (célula de 50 px)
        assert_eq!(m.wall_near(250.0, 104.0, 10.0).map(|w| w.id.as_str()), Some("w1"));
        assert!(m.wall_near(250.0, 140.0, 10.0).is_none());
        m.apply(&MapOp::RemoveWalls { ids: vec!["w1".into()] }).expect("apagar");
        assert_eq!(m.segments.len(), 1);
        // Fora do tabuleiro e comprimento zero são recusados
        assert!(m.apply(&MapOp::AddWall(wall("x", (1.0, 1.0), (99.0, 1.0), WallKind::Wall))).is_err());
        assert!(m.apply(&MapOp::AddWall(wall("y", (1.0, 1.0), (1.0, 1.0), WallKind::Wall))).is_err());
    }

    #[test]
    fn walls_survive_grid_shape_change() {
        let mut m = map();
        m.apply(&MapOp::AddWall(wall("w", (1.0, 1.0), (4.0, 1.0), WallKind::Wall))).expect("parede");
        let mut cfg = m.config();
        cfg.grid_shape = GridShape::HexPointy;
        m.apply(&MapOp::Configure(cfg)).expect("hexagonal");
        assert_eq!(m.segments.len(), 1, "segmento livre não depende dos lados das células");
    }

    #[test]
    fn legacy_edge_walls_become_segments() {
        let json = r#"{"cols":10,"rows":10,"walls":[
            {"edge":{"cell":{"col":2,"row":2},"side":1},"kind":"door"},
            {"edge":{"cell":{"col":3,"row":2},"side":3},"kind":"door"}]}"#;
        let mut m: RoomMapData = serde_json::from_str(json).expect("formato anterior");
        m.normalize();
        assert!(m.walls.is_empty());
        assert_eq!(m.segments.len(), 1, "o mesmo lado visto das duas células vira uma só parede");
        let w = &m.segments[0];
        assert_eq!((w.a.x, w.b.x), (3.0, 3.0), "lado leste de (2,2) = linha x = 3");
        assert!(!serde_json::to_string(&m).expect("json").contains("\"walls\""));
    }

    #[test]
    fn player_authorization() {
        let mut m = map();
        let token = |id: &str, sheet: Option<&str>| MapToken {
            id: id.into(), sheet_id: sheet.map(Into::into), name: id.into(), avatar_url: String::new(), grid_col: 0.0, grid_row: 0.0,
            size_cells: 1.0, is_npc: sheet.is_none(), is_hidden: false, color: "#fff".into(), health_status: None,
        };
        m.tokens.push(token("meu", Some("ficha-1")));
        m.tokens.push(token("npc", None));
        m.terrain.push(TerrainCell { cell: Cell::new(3, 3), kind: TerrainKind::Rock });
        let mine = vec!["ficha-1".to_string()];
        let mv = |id: &str, c: Cell| MapOp::MoveToken { id: id.into(), cell: c };

        assert!(m.authorize(&mv("meu", Cell::new(1, 1)), false, &mine).is_ok());
        assert!(m.authorize(&mv("npc", Cell::new(1, 1)), false, &mine).is_err(), "jogador não move NPC");
        assert!(m.authorize(&mv("meu", Cell::new(3, 3)), false, &mine).is_err(), "não entra em rocha");
        assert!(m.authorize(&MapOp::PaintTerrain { cells: vec![], kind: None }, false, &mine).is_err(), "jogador não monta");
        assert!(m.authorize(&MapOp::PaintTerrain { cells: vec![], kind: None }, true, &[]).is_ok(), "mestre monta");
        assert!(m.authorize(&mv("npc", Cell::new(3, 3)), true, &[]).is_ok(), "mestre move qualquer um");
    }

    #[test]
    fn players_can_only_move_tokens() {
        assert!(!MapOp::MoveToken { id: "t".into(), cell: Cell::new(0, 0) }.requires_gm());
        assert!(MapOp::PaintTerrain { cells: vec![], kind: None }.requires_gm());
        assert!(MapOp::RemoveToken { id: "t".into() }.requires_gm());
    }

    #[test]
    fn shrinking_the_grid_drops_what_falls_outside() {
        let mut m = map();
        m.apply(&MapOp::PaintTerrain { cells: vec![Cell::new(8, 8), Cell::new(1, 1)], kind: Some(TerrainKind::Grass) }).expect("pintar");
        let mut cfg = m.config();
        cfg.cols = 5;
        cfg.rows = 5;
        m.apply(&MapOp::Configure(cfg)).expect("configurar");
        assert_eq!(m.terrain.len(), 1);
    }

    #[test]
    fn legacy_structures_are_converted() {
        let json = r#"{"cols":10,"rows":10,"structures":[
            {"id":"a","structure_type":"wall","grid_col":1,"grid_row":1},
            {"id":"b","structure_type":"water","grid_col":2,"grid_row":1},
            {"id":"c","structure_type":"ward","grid_col":3,"grid_row":1}]}"#;
        let mut m: RoomMapData = serde_json::from_str(json).expect("mapa antigo");
        m.normalize();
        assert!(m.structures.is_empty());
        assert_eq!(m.terrain_at(Cell::new(1, 1)), Some(TerrainKind::Rock));
        assert_eq!(m.terrain_at(Cell::new(2, 1)), Some(TerrainKind::Water));
        assert_eq!(m.props.len(), 1);
        assert!(!serde_json::to_string(&m).expect("json").contains("structures"), "formato antigo não é regravado");
    }

    #[test]
    fn background_only_accepts_safe_urls() {
        let mut m = map();
        let bg = |url: &str| MapOp::SetBackground(BackgroundConfig { url: url.into(), opacity: 1.0, offset_x: 0.0, offset_y: 0.0, scale: 1.0 });
        assert!(m.apply(&bg("javascript:alert(1)")).is_err());
        assert!(m.apply(&bg("/uploads/abc.webp")).is_ok());
    }

    #[test]
    fn ops_roundtrip_as_json() {
        let op = MapOp::AddWall(wall("j", (1.0, 2.0), (3.0, 2.0), WallKind::Window));
        let json = serde_json::to_string(&op).expect("json");
        assert_eq!(serde_json::from_str::<MapOp>(&json).expect("volta"), op);
    }
}
