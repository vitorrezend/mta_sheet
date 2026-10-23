use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct RoomSummary {
    pub id: String,
    pub name: String,
    pub code: String,
    pub description: String,
    pub gm_username: String,
    pub is_gm: bool,
    pub is_public: bool,
    pub has_password: bool,
    pub member_count: i64,
    pub sheet_count: i64,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct RoomMemberInfo {
    pub user_id: String,
    pub username: String,
    pub role: String,
    pub joined_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct ChantryPoolData {
    pub quintessence_pool: i32,
    pub max_quintessence: i32,
    pub node_rating: i32,
    pub library_rating: i32,
    pub location_name: String,
    pub notes: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct RoomSheetSummary {
    pub id: String,
    pub name: String,
    pub player_name: String,
    pub tradition: String,
    #[serde(default)]
    pub essence: String,
    pub concept: String,
    #[serde(default)]
    pub demeanor: String,
    #[serde(default = "default_sheet_type_str")]
    pub sheet_type: String,
    pub arete: i32,
    pub willpower_total: i32,
    pub willpower_current: i32,
    pub quintessence: i32,
    pub paradox: i32,
    #[serde(default = "default_qp_track")]
    pub quintessence_paradox_track: String,
    pub photo_url: String,
    #[serde(default = "default_photo_focus_i32")]
    pub photo_focus_y: i32,
    #[serde(default = "default_photo_focus_i32")]
    pub photo_focus_x: i32,
    pub health_label: String,
    pub health_penalty: String,
    pub health_badge_class: String,
    pub health_damage_str: String,
    #[serde(default)]
    pub health_boxes: Vec<String>,
    #[serde(default)]
    pub spheres: Vec<(String, i32)>,
    pub is_hidden: bool,
    pub is_owner: bool,
    #[serde(default = "default_initiative_attr")]
    pub dexterity: i32,
    #[serde(default = "default_initiative_attr")]
    pub wits: i32,
    #[serde(default = "default_initiative_base")]
    pub initiative_base: i32,
    #[serde(default)]
    pub health_penalty_val: i32,
    pub updated_at: String,
}

pub fn default_sheet_type_str() -> String {
    "mage".to_string()
}

pub fn default_qp_track() -> String {
    "0".repeat(20)
}

pub fn default_initiative_attr() -> i32 { 1 }
pub fn default_initiative_base() -> i32 { 2 }
pub fn default_photo_focus_i32() -> i32 { 50 }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum QuickStatAction {
    CycleHealthBox { index: usize },
    HealHealthBox { index: usize },
    ClearHealth,
    AddQuintessence,
    RemoveQuintessence,
    AddParadox,
    RemoveParadox,
    CycleQuintessenceParadoxBox { index: usize },
    ClearQuintessenceParadoxBox { index: usize },
    SetWillpowerTotal { value: i32 },
    SetWillpowerCurrent { value: i32 },
    AdjustWillpowerCurrent { delta: i32 },
    AdjustWillpowerTotal { delta: i32 },
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct InitiativeEntry {
    pub id: String,
    pub name: String,
    pub is_npc: bool,
    pub is_active: bool,
    pub base_dex: i32,
    pub base_wits: i32,
    pub base_total: i32,
    #[serde(default)]
    pub health_penalty: i32,
    pub rolled_die: Option<i32>,
    pub final_total: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct RoomInitiativeData {
    pub round: u32,
    pub is_open: bool,
    pub entries: Vec<InitiativeEntry>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct ChatRollData {
    pub dice_count: i32,
    pub difficulty: i32,
    pub specialty: bool,
    #[serde(default)]
    pub specialty_text: Option<String>,
    pub dice_results: Vec<i32>,
    pub successes: i32,
    pub is_botch: bool,
    pub label: String,
}

/// Sala à qual uma ficha está vinculada (para rolar dados da ficha direto na mesa).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SheetRoomLink {
    pub room_id: String,
    pub room_name: String,
}

/// Tipo de mensagem do chat. Sempre definido pelo servidor (nunca confiado ao cliente).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ChatMessageKind {
    /// Fala do jogador, fora de cena
    #[default]
    Ooc,
    /// Fala em cena, como um personagem da sala
    Ic,
    /// Resultado de rolagem de dados
    Roll,
    /// Aviso gerado pelo sistema
    System,
}

impl ChatMessageKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ooc => "ooc",
            Self::Ic => "ic",
            Self::Roll => "roll",
            Self::System => "system",
        }
    }

    pub fn from_db(value: &str) -> Self {
        match value {
            "ic" => Self::Ic,
            "roll" => Self::Roll,
            "system" => Self::System,
            _ => Self::Ooc,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct RoomChatMessage {
    pub id: String,
    pub room_id: String,
    pub sender_id: String,
    pub sender_name: String,
    #[serde(default)]
    pub sender_character: Option<String>,
    #[serde(default)]
    pub sender_avatar: Option<String>,
    pub content: String,
    #[serde(default)]
    pub message_type: ChatMessageKind,
    #[serde(default)]
    pub roll_data: Option<ChatRollData>,
    /// UTC no formato do SQLite (`YYYY-MM-DD HH:MM:SS`)
    pub created_at: String,
}

/// Eventos em tempo real de uma sala, enviados via SSE (`/api/room_events/:id`).
/// O JSON é `{"type": "...", "data": ...}`, então cada consumidor trata só o que lhe interessa.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum RoomBroadcastEvent {
    /// Estado completo da iniciativa; `play_sound` quando o mestre rolou a iniciativa
    InitiativeUpdated { initiative: RoomInitiativeData, play_sound: bool },
    /// Nova mensagem ou rolagem no chat
    ChatMessage(RoomChatMessage),
    /// O mestre limpou o histórico do chat
    ChatCleared,
    /// O cliente perdeu eventos (fila cheia ou reconexão) e deve recarregar o estado
    Resync,
    /// Algo estrutural mudou (configurações, membros, fichas vinculadas ou ocultas):
    /// o cliente recarrega os detalhes da sala
    RoomUpdated,
    /// Resumo novo de uma ficha visível na sala (vida, força de vontade, nome...).
    /// `is_owner` vem sempre `false`: cada cliente mantém o seu valor local.
    SheetUpdated(RoomSheetSummary),
    /// Mapa tático alterado por alguém da sala
    MapUpdated(RoomMapData),
    /// Usuários com a sala aberta neste momento
    PresenceChanged { online_user_ids: Vec<String> },
    /// Capela salva por alguém da sala
    ChantryUpdated { chantry: ChantryPoolData, version: i64 },
    /// Diário da crônica salvo por alguém da sala
    ChronicleUpdated { notes: String, version: i64 },
}

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

pub fn default_token_size() -> f32 { 1.0 }
pub fn default_token_color() -> String { "#6366f1".to_string() }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MapStructure {
    pub id: String,
    #[serde(default = "default_structure_type")]
    pub structure_type: String, // "wall", "door", "cover", "water", "fire", "ward"
    #[serde(default = "default_structure_name")]
    pub name: String,
    #[serde(default = "default_structure_color")]
    pub color: String,
    #[serde(default = "default_structure_icon")]
    pub icon: String,
    #[serde(default)]
    pub grid_col: f32,
    #[serde(default)]
    pub grid_row: f32,
    #[serde(default = "default_structure_blocks_move")]
    pub blocks_movement: bool,
    #[serde(default)]
    pub blocks_sight: bool,
    #[serde(default = "default_structure_opacity")]
    pub opacity: f32,
}

pub fn default_structure_type() -> String { "wall".to_string() }
pub fn default_structure_name() -> String { "Muro".to_string() }
pub fn default_structure_color() -> String { "#334155".to_string() }
pub fn default_structure_icon() -> String { "🧱".to_string() }
pub fn default_structure_blocks_move() -> bool { true }
pub fn default_structure_opacity() -> f32 { 0.85 }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RoomMapData {
    #[serde(default = "default_map_enabled")]
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
    #[serde(default)]
    pub bg_image_url: String,
    #[serde(default = "default_map_bg_opacity")]
    pub bg_opacity: f32,
    #[serde(default = "default_map_meters_per_cell")]
    pub meters_per_cell: f32,
    #[serde(default)]
    pub tokens: Vec<MapToken>,
    #[serde(default)]
    pub structures: Vec<MapStructure>,
}

pub fn default_map_enabled() -> bool { true }
pub fn default_map_cols() -> u32 { 20 }
pub fn default_map_rows() -> u32 { 20 }
pub fn default_map_cell_size() -> u32 { 50 }
pub fn default_map_grid_color() -> String { "rgba(99, 102, 241, 0.4)".to_string() }
pub fn default_map_bg_opacity() -> f32 { 0.85 }
pub fn default_map_meters_per_cell() -> f32 { 1.5 }

impl Default for RoomMapData {
    fn default() -> Self {
        Self {
            enabled: true,
            grid_shape: GridShape::Square,
            cols: default_map_cols(),
            rows: default_map_rows(),
            cell_size: default_map_cell_size(),
            grid_color: default_map_grid_color(),
            bg_image_url: String::new(),
            bg_opacity: default_map_bg_opacity(),
            meters_per_cell: default_map_meters_per_cell(),
            tokens: Vec::new(),
            structures: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RoomDetails {
    pub id: String,
    pub name: String,
    pub code: String,
    pub description: String,
    pub gm_id: String,
    pub gm_username: String,
    pub is_gm: bool,
    pub is_public: bool,
    pub has_password: bool,
    pub chantry: ChantryPoolData,
    pub chronicle_notes: String,
    /// Versões da Capela e do Diário, para detectar edições simultâneas
    #[serde(default)]
    pub chantry_version: i64,
    #[serde(default)]
    pub chronicle_version: i64,
    pub initiative: RoomInitiativeData,
    pub map_data: RoomMapData,
    pub members: Vec<RoomMemberInfo>,
    pub sheets: Vec<RoomSheetSummary>,
    #[serde(default)]
    pub sheet_order: Vec<String>,
}

pub fn build_room_sheet_summary(
    id: String,
    name: String,
    data_json: &str,
    updated_at: String,
    is_hidden: bool,
    is_owner: bool,
) -> RoomSheetSummary {
    use crate::state::CharacterData;
    let mut char_data = CharacterData::parse_from_db(&id, data_json).unwrap_or_default();
    if char_data.id.is_empty() {
        char_data.id = id.clone();
    }
    if char_data.name.is_empty() || (char_data.name == "Novo Mago" && !name.is_empty() && name != "Novo Mago") {
        char_data.set_display_name(&name);
    }
    char_data.sanitize();
    let display_name = char_data.get_display_name();
    let (wp_total, wp_cur) = char_data.get_willpower();
    let (quint, paradox, raw_qp) = char_data.get_quintessence_paradox();

    // Calculate health status & penalty
    let (agg, lethal, bashing) = char_data.get_health_counts();
    let total_dmg = agg + lethal + bashing;
    let (health_label, health_penalty, health_badge_class, penalty_num) = match total_dmg {
        0 => ("Íntegro", "0", "health-full", 0),
        1 => ("Escoriado", "-0", "health-bruised", 0),
        2 => ("Ferido", "-1", "health-hurt", 1),
        3 => ("Gravemente Ferido", "-1", "health-injured", 1),
        4 => ("Espancado", "-2", "health-wounded", 2),
        5 => ("Estropiado", "-2", "health-mauled", 2),
        6 => ("Aleijado", "-5", "health-crippled", 5),
        _ => ("Incapacitado", "☠️", "health-incapacitated", 10),
    };

    let mut dmg_parts = Vec::new();
    if agg > 0 { dmg_parts.push(format!("{} Agravado", agg)); }
    if lethal > 0 { dmg_parts.push(format!("{} Letal", lethal)); }
    if bashing > 0 { dmg_parts.push(format!("{} Contundente", bashing)); }
    let health_damage_str = if dmg_parts.is_empty() {
        "Sem dano".to_string()
    } else {
        dmg_parts.join(", ")
    };

    let photo_url = if !char_data.visuals.character_sketch_url.is_empty() {
        char_data.visuals.character_sketch_url.clone()
    } else {
        char_data.get_profile_photo()
    };

    let essence = char_data.get_essence();
    let sheet_type = if char_data.sheet_type.is_empty() {
        "mage".to_string()
    } else {
        char_data.sheet_type.clone()
    };

    let total_boxes = char_data.get_total_health_boxes();
    let health_boxes: Vec<String> = (0..total_boxes)
        .map(|i| char_data.get_health(i).to_key().to_string())
        .collect();

    let spheres: Vec<(String, i32)> = crate::state::STANDARD_SPHERES
        .iter()
        .map(|&s| (s.to_string(), char_data.get_attribute_level(s, 0)))
        .collect();

    let dexterity = char_data.get_attribute_level("Destreza", 1);
    let wits = char_data.get_attribute_level("Raciocínio", 1);
    let raw_base = dexterity + wits;
    // Aplica a penalidade de dano com piso mínimo de 2
    let initiative_base = (raw_base - penalty_num).max(2);

    let (focus_x, focus_y) = char_data.get_photo_focus();

    RoomSheetSummary {
        id,
        name: display_name,
        player_name: char_data.get_label(crate::state::keys::HEADER_JOGADOR),
        tradition: char_data.get_tradition(),
        essence,
        concept: char_data.get_label(crate::state::keys::HEADER_CONCEITO),
        demeanor: char_data.get_label(crate::state::keys::HEADER_COMPORTAMENTO),
        sheet_type,
        arete: char_data.get_arete(),
        willpower_total: wp_total,
        willpower_current: wp_cur,
        quintessence: quint,
        paradox,
        quintessence_paradox_track: raw_qp,
        photo_url,
        photo_focus_y: focus_y,
        photo_focus_x: focus_x,
        health_label: health_label.to_string(),
        health_penalty: health_penalty.to_string(),
        health_badge_class: health_badge_class.to_string(),
        health_damage_str,
        health_boxes,
        spheres,
        is_hidden,
        is_owner,
        dexterity,
        wits,
        initiative_base,
        health_penalty_val: penalty_num,
        updated_at,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_summary_serialization() {
        let summary = RoomSummary {
            id: "room-1".to_string(),
            name: "Crônica dos Filhos do Éter".to_string(),
            code: "MTA-88AB".to_string(),
            description: "Uma crônica vitoriana".to_string(),
            gm_username: "MestreArkano".to_string(),
            is_gm: true,
            is_public: true,
            has_password: false,
            member_count: 4,
            sheet_count: 3,
            created_at: "2026-08-20 15:00:00".to_string(),
        };

        let json = serde_json::to_string(&summary).expect("serialize");
        let deserialized: RoomSummary = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(summary, deserialized);
    }

    #[test]
    fn test_chantry_pool_and_room_details_serialization() {
        let chantry = ChantryPoolData {
            quintessence_pool: 15,
            max_quintessence: 30,
            node_rating: 3,
            library_rating: 2,
            location_name: "Mansão Horizon".to_string(),
            notes: "Guardiões autômatos no jardim".to_string(),
        };

        let details = RoomDetails {
            id: "room-123".to_string(),
            name: "Cabala de São Paulo".to_string(),
            code: "MTA-SP01".to_string(),
            description: "Crônica urbana".to_string(),
            gm_id: "gm-1".to_string(),
            gm_username: "Mestre".to_string(),
            is_gm: true,
            is_public: false,
            has_password: true,
            chantry: chantry.clone(),
            chronicle_notes: "Sessão 1: Encontro na Avenida Paulista".to_string(),
            chantry_version: 0,
            chronicle_version: 0,
            initiative: RoomInitiativeData::default(),
            map_data: RoomMapData::default(),
            members: vec![],
            sheets: vec![],
            sheet_order: vec![],
        };

        let json = serde_json::to_string(&details).expect("serialize");
        let deserialized: RoomDetails = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(details, deserialized);
    }

    #[test]
    fn test_room_map_data_serialization() {
        let token = MapToken {
            id: "token-1".to_string(),
            sheet_id: Some("sheet-123".to_string()),
            name: "John Mage".to_string(),
            avatar_url: "/uploads/john.webp".to_string(),
            grid_col: 5.0,
            grid_row: 8.0,
            size_cells: 1.0,
            is_npc: false,
            is_hidden: false,
            color: "#6366f1".to_string(),
            health_status: Some("Saudável".to_string()),
        };

        let structure = MapStructure {
            id: "struct-1".to_string(),
            structure_type: "wall".to_string(),
            name: "Muro de Concreto".to_string(),
            color: "#334155".to_string(),
            icon: "🧱".to_string(),
            grid_col: 2.0,
            grid_row: 3.0,
            blocks_movement: true,
            blocks_sight: true,
            opacity: 0.85,
        };

        let map = RoomMapData {
            enabled: true,
            grid_shape: GridShape::HexPointy,
            cols: 25,
            rows: 25,
            cell_size: 48,
            grid_color: "rgba(99, 102, 241, 0.3)".to_string(),
            bg_image_url: "/uploads/map.png".to_string(),
            bg_opacity: 0.9,
            meters_per_cell: 1.5,
            tokens: vec![token],
            structures: vec![structure],
        };

        let json = serde_json::to_string(&map).expect("serialize map");
        let deserialized: RoomMapData = serde_json::from_str(&json).expect("deserialize map");
        assert_eq!(map, deserialized);
        assert_eq!(deserialized.grid_shape, GridShape::HexPointy);
        assert_eq!(deserialized.tokens.len(), 1);
        assert_eq!(deserialized.tokens[0].grid_col, 5.0);
        assert_eq!(deserialized.structures.len(), 1);
        assert_eq!(deserialized.structures[0].icon, "🧱");
    }

    #[test]
    fn test_room_member_info_serialization() {
        let member = RoomMemberInfo {
            user_id: "42".to_string(),
            username: "Hermes".to_string(),
            role: "player".to_string(),
            joined_at: "2026-08-27 00:00:00".to_string(),
        };

        let json = serde_json::to_string(&member).expect("serialize");
        let deserialized: RoomMemberInfo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(member, deserialized);
    }

    #[test]
    fn test_room_sheet_summary_serialization() {
        let summary = RoomSheetSummary {
            id: "sheet-999".to_string(),
            name: "John Doe".to_string(),
            player_name: "Jogador 1".to_string(),
            tradition: "Ordem de Hermes".to_string(),
            essence: "Dinâmica".to_string(),
            concept: "Erudito".to_string(),
            demeanor: "Visionário".to_string(),
            sheet_type: "mage".to_string(),
            arete: 3,
            willpower_total: 7,
            willpower_current: 5,
            quintessence: 4,
            paradox: 1,
            quintessence_paradox_track: "11110000000000000002".to_string(),
            photo_url: "https://example.com/photo.png".to_string(),
            photo_focus_y: 20,
            photo_focus_x: 50,
            health_label: "Ferido".to_string(),
            health_penalty: "-1".to_string(),
            health_badge_class: "health-hurt".to_string(),
            health_damage_str: "1 Letal, 1 Contundente".to_string(),
            health_boxes: vec!["lethal".to_string(), "bashing".to_string(), "none".to_string(), "none".to_string(), "none".to_string(), "none".to_string(), "none".to_string()],
            spheres: vec![("Forces".to_string(), 3), ("Prime".to_string(), 2)],
            is_hidden: false,
            is_owner: true,
            dexterity: 3,
            wits: 4,
            initiative_base: 6,
            health_penalty_val: 1,
            updated_at: "2026-08-28 20:00:00".to_string(),
        };

        let json = serde_json::to_string(&summary).expect("serialize");
        let deserialized: RoomSheetSummary = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(summary, deserialized);
    }
}
