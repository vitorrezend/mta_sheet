use serde::{Deserialize, Serialize};
use leptos::*;

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

fn default_sheet_type_str() -> String {
    "mage".to_string()
}

fn default_qp_track() -> String {
    "0".repeat(20)
}

fn default_initiative_attr() -> i32 { 1 }
fn default_initiative_base() -> i32 { 2 }
fn default_photo_focus_i32() -> i32 { 50 }

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
pub struct RoomBroadcastEvent {
    pub event_type: String, // "INITIATIVE_UPDATE" | "DICE_ROLLED" | "DRAWER_TOGGLED"
    pub initiative: RoomInitiativeData,
    pub play_sound: bool,
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

fn default_token_size() -> f32 { 1.0 }
fn default_token_color() -> String { "#6366f1".to_string() }

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

fn default_structure_type() -> String { "wall".to_string() }
fn default_structure_name() -> String { "Muro".to_string() }
fn default_structure_color() -> String { "#334155".to_string() }
fn default_structure_icon() -> String { "🧱".to_string() }
fn default_structure_blocks_move() -> bool { true }
fn default_structure_opacity() -> f32 { 0.85 }

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

fn default_map_enabled() -> bool { true }
fn default_map_cols() -> u32 { 20 }
fn default_map_rows() -> u32 { 20 }
fn default_map_cell_size() -> u32 { 50 }
fn default_map_grid_color() -> String { "rgba(99, 102, 241, 0.4)".to_string() }
fn default_map_bg_opacity() -> f32 { 0.85 }
fn default_map_meters_per_cell() -> f32 { 1.5 }

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
    pub initiative: RoomInitiativeData,
    pub map_data: RoomMapData,
    pub members: Vec<RoomMemberInfo>,
    pub sheets: Vec<RoomSheetSummary>,
    #[serde(default)]
    pub sheet_order: Vec<String>,
}

#[cfg(feature = "ssr")]
use std::sync::{Arc, Mutex, LazyLock};
#[cfg(feature = "ssr")]
use std::collections::HashMap;
#[cfg(feature = "ssr")]
use tokio::sync::broadcast;

#[cfg(feature = "ssr")]
static ROOM_CHANNELS: LazyLock<Arc<Mutex<HashMap<String, broadcast::Sender<RoomBroadcastEvent>>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

#[cfg(feature = "ssr")]
pub fn get_or_create_room_channel(room_id: &str) -> broadcast::Sender<RoomBroadcastEvent> {
    let mut channels = match ROOM_CHANNELS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    // Limpeza oportunística: previne vazamento de memória descartando canais inativos sem ouvintes
    if channels.len() > 30 {
        channels.retain(|_, sender| sender.receiver_count() > 0);
    }
    if let Some(sender) = channels.get(room_id) {
        sender.clone()
    } else {
        let (sender, _) = broadcast::channel(100);
        channels.insert(room_id.to_string(), sender.clone());
        sender
    }
}

#[cfg(feature = "ssr")]
pub fn remove_room_channel(room_id: &str) {
    let mut channels = match ROOM_CHANNELS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    channels.remove(room_id);
}

#[cfg(feature = "ssr")]
pub fn prune_inactive_room_channels() {
    let mut channels = match ROOM_CHANNELS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    channels.retain(|_, sender| sender.receiver_count() > 0);
}

#[cfg(feature = "ssr")]
pub fn broadcast_to_room(room_id: &str, event: RoomBroadcastEvent) {
    let sender = get_or_create_room_channel(room_id);
    let _ = sender.send(event);
}

#[cfg(feature = "ssr")]
fn generate_room_code() -> String {
    let chars = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let bytes = uuid::Uuid::new_v4().into_bytes();
    let c1 = (bytes[0] as usize) % chars.len();
    let c2 = (bytes[1] as usize) % chars.len();
    let c3 = (bytes[2] as usize) % chars.len();
    let c4 = (bytes[3] as usize) % chars.len();
    format!("MTA-{}{}{}{}", &chars[c1..c1+1], &chars[c2..c2+1], &chars[c3..c3+1], &chars[c4..c4+1])
}

#[server(endpoint = "get_user_rooms")]
pub async fn get_user_rooms() -> Result<Vec<RoomSummary>, ServerFnError> {
    use sqlx::{SqlitePool, Row};
    use crate::auth::get_auth_user_id;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Faça login para ver suas salas")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let query = "
        SELECT 
            r.id, r.name, r.code, r.description, r.created_at, r.is_public,
            (r.password_hash IS NOT NULL AND r.password_hash != '') as has_password,
            u.username as gm_username,
            (r.gm_id = ?) as is_gm,
            (SELECT COUNT(*) FROM room_members rm WHERE rm.room_id = r.id) as member_count,
            (SELECT COUNT(*) FROM character_sheets cs WHERE cs.room_id = r.id) as sheet_count
        FROM rooms r
        JOIN users u ON r.gm_id = u.id
        LEFT JOIN room_members rm ON rm.room_id = r.id AND rm.user_id = ?
        WHERE r.gm_id = ? OR rm.user_id IS NOT NULL
        GROUP BY r.id
        ORDER BY r.created_at DESC
    ";

    let rows = sqlx::query(query)
        .bind(&user_id)
        .bind(&user_id)
        .bind(&user_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let summaries = rows.into_iter().map(|row| RoomSummary {
        id: row.get("id"),
        name: row.get("name"),
        code: row.get("code"),
        description: row.get("description"),
        gm_username: row.get("gm_username"),
        is_gm: row.get("is_gm"),
        is_public: row.try_get::<i64, _>("is_public").map(|v| v == 1).unwrap_or(false),
        has_password: row.get("has_password"),
        member_count: row.get("member_count"),
        sheet_count: row.get("sheet_count"),
        created_at: row.get("created_at"),
    }).collect();

    Ok(summaries)
}

#[server(endpoint = "get_public_rooms")]
pub async fn get_public_rooms() -> Result<Vec<RoomSummary>, ServerFnError> {
    use sqlx::{SqlitePool, Row};
    use crate::auth::get_auth_user_id;

    let current_user_id = get_auth_user_id().await?.unwrap_or_default();

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let query = "
        SELECT 
            r.id, r.name, r.code, r.description, r.created_at, r.is_public,
            (r.password_hash IS NOT NULL AND r.password_hash != '') as has_password,
            u.username as gm_username,
            (r.gm_id = ?) as is_gm,
            (SELECT COUNT(*) FROM room_members rm WHERE rm.room_id = r.id) as member_count,
            (SELECT COUNT(*) FROM character_sheets cs WHERE cs.room_id = r.id) as sheet_count
        FROM rooms r
        JOIN users u ON r.gm_id = u.id
        WHERE r.is_public = 1
        ORDER BY r.created_at DESC
        LIMIT 50
    ";

    let rows = sqlx::query(query)
        .bind(&current_user_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let summaries = rows.into_iter().map(|row| RoomSummary {
        id: row.get("id"),
        name: row.get("name"),
        code: row.get("code"),
        description: row.get("description"),
        gm_username: row.get("gm_username"),
        is_gm: row.get("is_gm"),
        is_public: true,
        has_password: row.get("has_password"),
        member_count: row.get("member_count"),
        sheet_count: row.get("sheet_count"),
        created_at: row.get("created_at"),
    }).collect();

    Ok(summaries)
}

#[server(endpoint = "create_room")]
pub async fn create_room(
    name: String,
    description: String,
    is_public: bool,
    password: Option<String>,
) -> Result<String, ServerFnError> {
    let clean_name = name.trim().to_string();
    if clean_name.is_empty() {
        return Err(ServerFnError::new("O nome da sala/crônica não pode ser vazio"));
    }

    use sqlx::SqlitePool;
    use uuid::Uuid;
    use crate::auth::get_auth_user_id;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para criar uma sala")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let room_id = Uuid::new_v4().to_string();
    let code = generate_room_code();

    let password_hash = if let Some(ref p) = password {
        let p_trimmed = p.trim();
        if !p_trimmed.is_empty() {
            bcrypt::hash(p_trimmed, 8).map_err(|e| ServerFnError::new(format!("Erro ao processar senha: {}", e)))?
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let is_pub_val: i64 = if is_public { 1 } else { 0 };

    sqlx::query(
        "INSERT INTO rooms (id, name, code, description, gm_id, is_public, password_hash) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&room_id)
    .bind(&clean_name)
    .bind(&code)
    .bind(&description)
    .bind(&user_id)
    .bind(is_pub_val)
    .bind(&password_hash)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Erro ao criar sala: {}", e)))?;

    // Add GM to room members
    sqlx::query(
        "INSERT INTO room_members (room_id, user_id, role) VALUES (?, ?, 'gm')"
    )
    .bind(&room_id)
    .bind(&user_id)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Erro ao adicionar membro: {}", e)))?;

    log::info!("Created room '{}' ({}) is_public={} has_pwd={}", clean_name, code, is_public, !password_hash.is_empty());
    Ok(room_id)
}

#[server(endpoint = "join_room_by_code")]
pub async fn join_room_by_code(code: String, password: Option<String>) -> Result<String, ServerFnError> {
    let clean_code = code.trim().to_uppercase();
    if clean_code.is_empty() {
        return Err(ServerFnError::new("Código da sala não fornecido"));
    }

    use sqlx::{SqlitePool, Row};
    use crate::auth::get_auth_user_id;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para entrar em uma sala")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let room_row = sqlx::query("SELECT id, name, gm_id, password_hash FROM rooms WHERE code = ?")
        .bind(&clean_code)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Código de sala inválido ou sala não encontrada"))?;

    let room_id: String = room_row.get("id");
    let room_name: String = room_row.get("name");
    let gm_id: String = room_row.get("gm_id");
    let password_hash: String = room_row.try_get("password_hash").unwrap_or_default();

    if user_id != gm_id && !password_hash.is_empty() {
        let pwd_input = password.as_deref().unwrap_or("").trim();
        if pwd_input.is_empty() {
            return Err(ServerFnError::new("Esta sala é protegida por senha. Digite a senha para entrar."));
        }
        let valid = bcrypt::verify(pwd_input, &password_hash).unwrap_or(false);
        if !valid {
            return Err(ServerFnError::new("Senha incorreta para esta sala."));
        }
    }

    // Add member if not exists
    sqlx::query(
        "INSERT OR IGNORE INTO room_members (room_id, user_id, role) VALUES (?, ?, 'player')"
    )
    .bind(&room_id)
    .bind(&user_id)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Erro ao entrar na sala: {}", e)))?;

    log::info!("User joined room '{}' ({})", room_name, room_id);
    Ok(room_id)
}

#[server(endpoint = "join_public_room")]
pub async fn join_public_room(room_id: String, password: Option<String>) -> Result<String, ServerFnError> {
    use sqlx::{SqlitePool, Row};
    use crate::auth::get_auth_user_id;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para entrar em uma sala")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let room_row = sqlx::query("SELECT id, name, gm_id, password_hash, is_public FROM rooms WHERE id = ?")
        .bind(&room_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Sala não encontrada"))?;

    let room_name: String = room_row.get("name");
    let gm_id: String = room_row.get("gm_id");
    let password_hash: String = room_row.try_get("password_hash").unwrap_or_default();

    if user_id != gm_id && !password_hash.is_empty() {
        let pwd_input = password.as_deref().unwrap_or("").trim();
        if pwd_input.is_empty() {
            return Err(ServerFnError::new("Esta sala é protegida por senha. Digite a senha para entrar."));
        }
        let valid = bcrypt::verify(pwd_input, &password_hash).unwrap_or(false);
        if !valid {
            return Err(ServerFnError::new("Senha incorreta para esta sala."));
        }
    }

    sqlx::query(
        "INSERT OR IGNORE INTO room_members (room_id, user_id, role) VALUES (?, ?, 'player')"
    )
    .bind(&room_id)
    .bind(&user_id)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Erro ao entrar na sala: {}", e)))?;

    log::info!("User joined public room '{}' ({})", room_name, room_id);
    Ok(room_id)
}

#[server(endpoint = "update_room_settings")]
pub async fn update_room_settings(
    room_id: String,
    is_public: bool,
    new_password: Option<String>,
    remove_password: bool,
) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    use crate::auth::get_auth_user_id;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para alterar as configurações da sala")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let is_gm: bool = sqlx::query_scalar::<_, i64>("SELECT 1 FROM rooms WHERE id = ? AND gm_id = ?")
        .bind(&room_id)
        .bind(&user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .is_some();

    if !is_gm {
        return Err(ServerFnError::new("Apenas o Narrador pode alterar as configurações da sala."));
    }

    let is_pub_val: i64 = if is_public { 1 } else { 0 };

    if remove_password {
        sqlx::query("UPDATE rooms SET is_public = ?, password_hash = '' WHERE id = ?")
            .bind(is_pub_val)
            .bind(&room_id)
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
    } else if let Some(ref p) = new_password {
        let p_trimmed = p.trim();
        if !p_trimmed.is_empty() {
            let hash = bcrypt::hash(p_trimmed, 8).map_err(|e| ServerFnError::new(e.to_string()))?;
            sqlx::query("UPDATE rooms SET is_public = ?, password_hash = ? WHERE id = ?")
                .bind(is_pub_val)
                .bind(&hash)
                .bind(&room_id)
                .execute(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        } else {
            sqlx::query("UPDATE rooms SET is_public = ? WHERE id = ?")
                .bind(is_pub_val)
                .bind(&room_id)
                .execute(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        }
    } else {
        sqlx::query("UPDATE rooms SET is_public = ? WHERE id = ?")
            .bind(is_pub_val)
            .bind(&room_id)
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
    }

    Ok(())
}

#[cfg(feature = "ssr")]
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
    let health_damage_str = if dmg_parts.is_empty() { "Sem dano".to_string() } else { dmg_parts.join(", ") };

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

#[server(endpoint = "get_room_details")]
pub async fn get_room_details(room_id: String) -> Result<RoomDetails, ServerFnError> {
    if room_id.trim().is_empty() {
        return Err(ServerFnError::new("ID da sala não fornecido"));
    }

    use sqlx::{SqlitePool, Row};
    use crate::auth::get_auth_user_id;

    let current_user_id = get_auth_user_id().await?.unwrap_or_default();

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    // 1. Get room info including chantry, chronicle notes, initiative, map, and sheet_order
    let room_row = sqlx::query(
        "SELECT r.id, r.name, r.code, r.description, r.gm_id, r.chantry_data, r.chronicle_notes, r.initiative_data, r.map_data, r.sheet_order, r.is_public,
                (r.password_hash IS NOT NULL AND r.password_hash != '') as has_password,
                u.username as gm_username 
         FROM rooms r 
         JOIN users u ON r.gm_id = u.id 
         WHERE r.id = ?"
    )
    .bind(&room_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?
    .ok_or_else(|| ServerFnError::new("Sala não encontrada"))?;

    let gm_id: String = room_row.get("gm_id");
    let is_gm = !current_user_id.is_empty() && current_user_id == gm_id;
    let is_public = room_row.try_get::<i64, _>("is_public").map(|v| v == 1).unwrap_or(false);
    let has_password: bool = room_row.get("has_password");

    let chantry_raw: String = room_row.try_get("chantry_data").unwrap_or_default();
    let chantry: ChantryPoolData = serde_json::from_str(&chantry_raw).unwrap_or_default();
    let chronicle_notes: String = room_row.try_get("chronicle_notes").unwrap_or_default();
    let initiative_raw: String = room_row.try_get("initiative_data").unwrap_or_default();
    let mut initiative: RoomInitiativeData = if !initiative_raw.is_empty() {
        serde_json::from_str(&initiative_raw).unwrap_or_default()
    } else {
        RoomInitiativeData::default()
    };
    let map_raw: String = room_row.try_get("map_data").unwrap_or_default();
    let map_data: RoomMapData = if !map_raw.is_empty() {
        serde_json::from_str(&map_raw).unwrap_or_default()
    } else {
        RoomMapData::default()
    };
    let sheet_order_raw: String = room_row.try_get("sheet_order").unwrap_or_default();
    let sheet_order: Vec<String> = if !sheet_order_raw.is_empty() {
        serde_json::from_str(&sheet_order_raw).unwrap_or_default()
    } else {
        Vec::new()
    };

    // 2. Get members
    let member_rows = sqlx::query(
        "SELECT rm.user_id, u.username, rm.role, rm.joined_at 
         FROM room_members rm 
         JOIN users u ON rm.user_id = u.id 
         WHERE rm.room_id = ? 
         ORDER BY rm.role DESC, rm.joined_at ASC"
    )
    .bind(&room_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let members = member_rows.into_iter().map(|r| RoomMemberInfo {
        user_id: r.get("user_id"),
        username: r.get("username"),
        role: r.get("role"),
        joined_at: r.get("joined_at"),
    }).collect();

    // 3. Get sheets (GM sees all, players only see non-hidden sheets or their own)
    // Ordenação padrão estável por ordem alfabética para evitar deslocamento indesejado ao editar fichas
    let sheet_query = if is_gm {
        "SELECT id, user_id, name, data, is_hidden_in_room, updated_at FROM character_sheets WHERE room_id = ? ORDER BY LOWER(name) ASC, id ASC"
    } else {
        "SELECT id, user_id, name, data, is_hidden_in_room, updated_at FROM character_sheets WHERE room_id = ? AND (is_hidden_in_room = 0 OR user_id = ?) ORDER BY LOWER(name) ASC, id ASC"
    };

    let mut q = sqlx::query(sheet_query).bind(&room_id);
    if !is_gm {
        q = q.bind(&current_user_id);
    }

    let sheet_rows = q.fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let mut sheets: Vec<RoomSheetSummary> = sheet_rows.into_iter().map(|r| {
        let id: String = r.get("id");
        let sheet_user_id: Option<String> = r.get("user_id");
        let is_owner = !current_user_id.is_empty() && sheet_user_id.as_deref() == Some(&current_user_id);
        let is_hidden: bool = r.try_get::<i64, _>("is_hidden_in_room").map(|v| v == 1).unwrap_or(false);
        let name: String = r.get("name");
        let data_json: String = r.get("data");
        let updated_at: String = r.get("updated_at");

        build_room_sheet_summary(id, name, &data_json, updated_at, is_hidden, is_owner)
    }).collect();

    // Aplica a ordem customizada persistida da sala (se houver)
    if !sheet_order.is_empty() {
        sheets.sort_by_key(|s| {
            sheet_order.iter().position(|id| id == &s.id).unwrap_or(usize::MAX)
        });
    }

    // Reconcilia e expurga fichas fantasmas da iniciativa da sala
    // Busca todos os IDs reais das fichas vinculadas a esta sala
    let all_room_sheet_ids: Vec<String> = sqlx::query_scalar("SELECT id FROM character_sheets WHERE room_id = ?")
        .bind(&room_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let initial_entries_count = initiative.entries.len();
    initiative.entries.retain(|e| e.is_npc || all_room_sheet_ids.iter().any(|id| id == &e.id));
    if initiative.entries.len() != initial_entries_count {
        if let Ok(cleaned_json) = serde_json::to_string(&initiative) {
            let _ = sqlx::query("UPDATE rooms SET initiative_data = ? WHERE id = ?")
                .bind(&cleaned_json)
                .bind(&room_id)
                .execute(&pool)
                .await;
        }
    }

    Ok(RoomDetails {
        id: room_row.get("id"),
        name: room_row.get("name"),
        code: room_row.get("code"),
        description: room_row.get("description"),
        gm_id,
        gm_username: room_row.get("gm_username"),
        is_gm,
        is_public,
        has_password,
        chantry,
        chronicle_notes,
        initiative,
        map_data,
        members,
        sheets,
        sheet_order,
    })
}

#[server(endpoint = "update_room_sheet_order")]
pub async fn update_room_sheet_order(room_id: String, ordered_sheet_ids: Vec<String>) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    use crate::auth::get_auth_user_id;

    if room_id.trim().is_empty() {
        return Err(ServerFnError::new("ID da sala não fornecido"));
    }

    let current_user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Autenticação necessária para reordenar fichas")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    // Verifica se é narrador ou membro da sala
    let is_authorized = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM rooms r 
         LEFT JOIN room_members rm ON rm.room_id = r.id AND rm.user_id = ? 
         WHERE r.id = ? AND (r.gm_id = ? OR rm.user_id IS NOT NULL)"
    )
    .bind(&current_user_id)
    .bind(&room_id)
    .bind(&current_user_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))? > 0;

    if !is_authorized {
        return Err(ServerFnError::new("Você não tem permissão para reordenar as fichas nesta sala"));
    }

    let order_json = serde_json::to_string(&ordered_sheet_ids)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    sqlx::query("UPDATE rooms SET sheet_order = ? WHERE id = ?")
        .bind(&order_json)
        .bind(&room_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[server(endpoint = "update_room_initiative")]
pub async fn update_room_initiative(room_id: String, initiative: RoomInitiativeData, play_sound: bool) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    use crate::auth::get_auth_user_id;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para alterar a iniciativa da sala")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let is_gm: bool = sqlx::query_scalar::<_, i64>("SELECT 1 FROM rooms WHERE id = ? AND gm_id = ?")
        .bind(&room_id)
        .bind(&user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .is_some();

    if !is_gm {
        return Err(ServerFnError::new("Apenas o Narrador pode alterar a iniciativa da sala."));
    }

    let data_json = serde_json::to_string(&initiative).unwrap_or_default();
    sqlx::query("UPDATE rooms SET initiative_data = ? WHERE id = ?")
        .bind(&data_json)
        .bind(&room_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao atualizar iniciativa: {}", e)))?;

    broadcast_to_room(&room_id, RoomBroadcastEvent {
        event_type: if play_sound { "DICE_ROLLED".to_string() } else { "INITIATIVE_UPDATE".to_string() },
        initiative,
        play_sound,
    });

    Ok(())
}

#[server(endpoint = "update_room_map")]
pub async fn update_room_map(room_id: String, map_data: RoomMapData) -> Result<(), ServerFnError> {
    if room_id.trim().is_empty() {
        return Err(ServerFnError::new("ID da sala não fornecido"));
    }

    use sqlx::{SqlitePool, Row};
    use crate::auth::get_auth_user_id;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para alterar o mapa da sala")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    // Feature Flag Guard: Verifica se o mapa tático está liberado para o chamador
    let is_feature_allowed = crate::settings::is_feature_enabled_for_caller(&pool, "feature_tactical_grid").await?;
    if !is_feature_allowed {
        return Err(ServerFnError::new(
            "O recurso de Mapa & Grid Tático está temporariamente em manutenção ou restrito a administradores."
        ));
    }

    let member_check = sqlx::query("SELECT rm.role, r.gm_id FROM rooms r LEFT JOIN room_members rm ON rm.room_id = r.id AND rm.user_id = ? WHERE r.id = ?")
        .bind(&user_id)
        .bind(&room_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Sala não encontrada"))?;

    let gm_id: String = member_check.try_get("gm_id").unwrap_or_default();
    let is_gm = gm_id == user_id;
    let has_role: bool = member_check.try_get::<Option<String>, _>("role").ok().flatten().is_some();

    if !is_gm && !has_role {
        return Err(ServerFnError::new("Você não tem permissão para alterar o mapa desta sala"));
    }

    let map_json = serde_json::to_string(&map_data)
        .map_err(|e| ServerFnError::new(format!("Erro ao serializar dados do mapa: {}", e)))?;

    sqlx::query("UPDATE rooms SET map_data = ? WHERE id = ?")
        .bind(&map_json)
        .bind(&room_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao salvar mapa: {}", e)))?;

    Ok(())
}

#[server(endpoint = "assign_sheet_to_room")]
pub async fn assign_sheet_to_room(sheet_id: String, room_id: String) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    sqlx::query("UPDATE character_sheets SET room_id = ? WHERE id = ?")
        .bind(&room_id)
        .bind(&sheet_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao associar ficha à sala: {}", e)))?;

    Ok(())
}

#[server(endpoint = "remove_sheet_from_room")]
pub async fn remove_sheet_from_room(sheet_id: String) -> Result<(), ServerFnError> {
    use sqlx::{SqlitePool, Row};
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    // 1. Descobre a sala a que a ficha pertencia antes de desvincular
    let room_id: Option<String> = sqlx::query_scalar("SELECT room_id FROM character_sheets WHERE id = ?")
        .bind(&sheet_id)
        .fetch_optional(&pool)
        .await
        .unwrap_or_default();

    // 2. Desvincula a ficha
    sqlx::query("UPDATE character_sheets SET room_id = NULL WHERE id = ?")
        .bind(&sheet_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao desvincular ficha: {}", e)))?;

    // 3. Se estava vinculada a uma sala, limpa a iniciativa e a ordem de fichas dessa sala
    if let Some(r_id) = room_id {
        if let Ok(Some(row)) = sqlx::query("SELECT initiative_data, sheet_order FROM rooms WHERE id = ?")
            .bind(&r_id)
            .fetch_optional(&pool)
            .await
        {
            let init_raw: String = row.try_get("initiative_data").unwrap_or_default();
            if !init_raw.is_empty() {
                if let Ok(mut init) = serde_json::from_str::<RoomInitiativeData>(&init_raw) {
                    let old_len = init.entries.len();
                    init.entries.retain(|e| e.id != sheet_id);
                    if init.entries.len() != old_len {
                        if let Ok(new_json) = serde_json::to_string(&init) {
                            let _ = sqlx::query("UPDATE rooms SET initiative_data = ? WHERE id = ?")
                                .bind(&new_json)
                                .bind(&r_id)
                                .execute(&pool)
                                .await;

                            broadcast_to_room(&r_id, RoomBroadcastEvent {
                                event_type: "INITIATIVE_UPDATE".to_string(),
                                initiative: init,
                                play_sound: false,
                            });
                        }
                    }
                }
            }

            let order_raw: String = row.try_get("sheet_order").unwrap_or_default();
            if !order_raw.is_empty() {
                if let Ok(mut order) = serde_json::from_str::<Vec<String>>(&order_raw) {
                    let old_len = order.len();
                    order.retain(|id| id != &sheet_id);
                    if order.len() != old_len {
                        if let Ok(new_order) = serde_json::to_string(&order) {
                            let _ = sqlx::query("UPDATE rooms SET sheet_order = ? WHERE id = ?")
                                .bind(&new_order)
                                .bind(&r_id)
                                .execute(&pool)
                                .await;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

#[server(endpoint = "toggle_sheet_room_visibility")]
pub async fn toggle_sheet_room_visibility(sheet_id: String, is_hidden: bool) -> Result<(), ServerFnError> {
    use sqlx::{SqlitePool, Row};
    use crate::auth::get_auth_user_id;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para alterar a visibilidade")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    // Check if user is GM of the room or owner of the sheet
    let row = sqlx::query(
        "SELECT cs.user_id, r.gm_id FROM character_sheets cs 
         LEFT JOIN rooms r ON cs.room_id = r.id 
         WHERE cs.id = ?"
    )
    .bind(&sheet_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?
    .ok_or_else(|| ServerFnError::new("Ficha não encontrada"))?;

    let sheet_owner: Option<String> = row.get("user_id");
    let room_gm: Option<String> = row.get("gm_id");

    if sheet_owner.as_deref() != Some(&user_id) && room_gm.as_deref() != Some(&user_id) {
        return Err(ServerFnError::new("Você não tem permissão para alterar a visibilidade desta ficha"));
    }

    let val: i64 = if is_hidden { 1 } else { 0 };
    sqlx::query("UPDATE character_sheets SET is_hidden_in_room = ? WHERE id = ?")
        .bind(val)
        .bind(&sheet_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao alterar visibilidade: {}", e)))?;

    Ok(())
}

#[server(endpoint = "update_room_chantry")]
pub async fn update_room_chantry(room_id: String, chantry: ChantryPoolData) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    use crate::auth::get_auth_user_id;

    let _user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para atualizar os recursos da capela")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let json_data = serde_json::to_string(&chantry).unwrap_or_default();

    sqlx::query("UPDATE rooms SET chantry_data = ? WHERE id = ?")
        .bind(&json_data)
        .bind(&room_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao atualizar capela: {}", e)))?;

    Ok(())
}

#[server(endpoint = "update_room_chronicle_notes")]
pub async fn update_room_chronicle_notes(room_id: String, notes: String) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    use crate::auth::get_auth_user_id;

    let _user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para editar o diário da crônica")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    sqlx::query("UPDATE rooms SET chronicle_notes = ? WHERE id = ?")
        .bind(&notes)
        .bind(&room_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao atualizar diário da crônica: {}", e)))?;

    Ok(())
}

#[server(endpoint = "clone_and_assign_sheet_to_member")]
pub async fn clone_and_assign_sheet_to_member(
    room_id: String,
    sheet_id: String,
    target_user_id: String,
) -> Result<String, ServerFnError> {
    use sqlx::{SqlitePool, Row};
    use crate::auth::get_auth_user_id;
    use crate::state::CharacterData;

    let caller_user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para realizar esta ação")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    // 1. Valida se o chamador é o Narrador da sala
    let is_gm_row = sqlx::query("SELECT gm_id FROM rooms WHERE id = ?")
        .bind(&room_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao verificar sala: {}", e)))?;

    let gm_row = is_gm_row.ok_or_else(|| ServerFnError::new("Sala não encontrada"))?;
    let room_gm_id: String = gm_row.get("gm_id");

    if room_gm_id != caller_user_id {
        return Err(ServerFnError::new("Apenas o Narrador da sala pode clonar e entregar fichas para membros"));
    }

    // 2. Valida se o target_user_id é membro desta sala
    let is_member = sqlx::query("SELECT 1 FROM room_members WHERE room_id = ? AND user_id = ?")
        .bind(&room_id)
        .bind(&target_user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao verificar membro: {}", e)))?;

    if is_member.is_none() && target_user_id != room_gm_id {
        return Err(ServerFnError::new("O jogador selecionado não é membro desta sala"));
    }

    // 3. Validação de cota de 50 fichas para o jogador de destino
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM character_sheets WHERE user_id = ?")
        .bind(&target_user_id)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    if count >= 50 {
        return Err(ServerFnError::new("O jogador selecionado atingiu o limite de 50 fichas. Ele precisa excluir fichas antigas para receber uma nova."));
    }

    // 4. Busca a ficha original do Narrador
    let sheet_row = sqlx::query("SELECT name, data, sheet_type FROM character_sheets WHERE id = ? AND user_id = ?")
        .bind(&sheet_id)
        .bind(&caller_user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao buscar ficha original: {}", e)))?;

    let sheet_data = sheet_row.ok_or_else(|| ServerFnError::new("Ficha original não encontrada no seu inventário"))?;
    let sheet_name: String = sheet_data.get("name");
    let sheet_json: String = sheet_data.get("data");
    let sheet_type: String = sheet_data.get("sheet_type");

    // 5. Cria nova ficha com novo UUID pertencente ao jogador e já associada à sala
    let new_sheet_id = uuid::Uuid::new_v4().to_string();

    let mut updated_data: CharacterData = serde_json::from_str(&sheet_json)
        .unwrap_or_else(|_| CharacterData::new(new_sheet_id.clone(), sheet_name.clone()));
    updated_data.id = new_sheet_id.clone();
    let updated_json = serde_json::to_string(&updated_data).unwrap_or(sheet_json);

    let summary = updated_data.to_summary(String::new(), false, true);
    let summary_json = serde_json::to_string(&summary).unwrap_or_default();

    sqlx::query(
        "INSERT INTO character_sheets (id, user_id, room_id, name, data, sheet_type, is_public, is_hidden_in_room, summary_json, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, 0, 0, ?, datetime('now'))"
    )
    .bind(&new_sheet_id)
    .bind(&target_user_id)
    .bind(&room_id)
    .bind(&sheet_name)
    .bind(&updated_json)
    .bind(&sheet_type)
    .bind(&summary_json)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Erro ao criar ficha clonada: {}", e)))?;

    // 6. Clona respostas do Dossiê/Questionário, se houverem
    let quiz_answers = sqlx::query("SELECT question_id, answer FROM character_quiz_answers WHERE character_id = ?")
        .bind(&sheet_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    for qa in quiz_answers {
        let q_id: String = qa.get("question_id");
        let ans: String = qa.get("answer");
        let _ = sqlx::query(
            "INSERT OR REPLACE INTO character_quiz_answers (character_id, question_id, answer, updated_at) \
             VALUES (?, ?, ?, datetime('now'))"
        )
        .bind(&new_sheet_id)
        .bind(&q_id)
        .bind(&ans)
        .execute(&pool)
        .await;
    }

    crate::logging::server::write_log(
        crate::logging::LogCategory::UserActions,
        "INFO",
        &format!("CLONE & ASSIGN SHEET: Ficha '{}' ({}) clonada com sucesso para o jogador '{}' na sala '{}'", sheet_name, new_sheet_id, target_user_id, room_id),
        None,
    );

    Ok(new_sheet_id)
}

#[server(endpoint = "update_room_sheet_stats")]
pub async fn update_room_sheet_stats(
    room_id: String,
    sheet_id: String,
    action: QuickStatAction,
) -> Result<RoomSheetSummary, ServerFnError> {
    use sqlx::{SqlitePool, Row};
    use crate::auth::get_auth_user_id;
    use crate::state::CharacterData;

    let caller_user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para realizar esta ação")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    // 1. Valida se o chamador é o Narrador da sala
    let is_gm_row = sqlx::query("SELECT gm_id FROM rooms WHERE id = ?")
        .bind(&room_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao verificar sala: {}", e)))?;

    let gm_row = is_gm_row.ok_or_else(|| ServerFnError::new("Sala não encontrada"))?;
    let room_gm_id: String = gm_row.get("gm_id");

    if room_gm_id != caller_user_id {
        return Err(ServerFnError::new("Apenas o Narrador da sala pode alterar os atributos táticos da ficha"));
    }

    // 2. Busca a ficha associada à sala
    let sheet_row = sqlx::query(
        "SELECT id, name, data, user_id, is_hidden_in_room, updated_at \
         FROM character_sheets WHERE id = ? AND room_id = ?"
    )
    .bind(&sheet_id)
    .bind(&room_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Erro ao buscar ficha na sala: {}", e)))?;

    let sheet_record = sheet_row.ok_or_else(|| ServerFnError::new("Ficha não encontrada nesta sala"))?;
    let sheet_name: String = sheet_record.get("name");
    let sheet_json: String = sheet_record.get("data");
    let sheet_user_id: Option<String> = sheet_record.get("user_id");
    let is_hidden_val: i64 = sheet_record.get("is_hidden_in_room");
    let is_hidden = is_hidden_val == 1;
    let is_owner = sheet_user_id.as_deref() == Some(&caller_user_id);

    let mut char_data = CharacterData::parse_from_db(&sheet_id, &sheet_json)
        .unwrap_or_else(|| CharacterData::new(sheet_id.clone(), sheet_name.clone()));

    // 3. Aplica a ação tática
    match action {
        QuickStatAction::CycleHealthBox { index } => {
            char_data.click_health_box(index);
        }
        QuickStatAction::HealHealthBox { index } => {
            char_data.heal_health_box(index);
        }
        QuickStatAction::ClearHealth => {
            char_data.clear_health();
        }
        QuickStatAction::AddQuintessence => {
            char_data.add_quintessence();
        }
        QuickStatAction::RemoveQuintessence => {
            char_data.remove_quintessence();
        }
        QuickStatAction::AddParadox => {
            char_data.add_paradox();
        }
        QuickStatAction::RemoveParadox => {
            char_data.remove_paradox();
        }
        QuickStatAction::CycleQuintessenceParadoxBox { index } => {
            char_data.cycle_quintessence_paradox_box(index);
        }
        QuickStatAction::ClearQuintessenceParadoxBox { index } => {
            char_data.set_quintessence_paradox_box(index, '0');
        }
        QuickStatAction::SetWillpowerTotal { value } => {
            char_data.set_willpower_total(value);
        }
        QuickStatAction::SetWillpowerCurrent { value } => {
            char_data.set_willpower_current(value);
        }
        QuickStatAction::AdjustWillpowerCurrent { delta } => {
            let (tot, cur) = char_data.get_willpower();
            let new_cur = (cur + delta).clamp(0, tot);
            char_data.set_willpower_current(new_cur);
        }
        QuickStatAction::AdjustWillpowerTotal { delta } => {
            let (tot, _) = char_data.get_willpower();
            let new_tot = (tot + delta).clamp(1, 10);
            char_data.set_willpower_total(new_tot);
        }
    }

    char_data.sanitize();
    let updated_json = serde_json::to_string(&char_data)
        .map_err(|e| ServerFnError::new(format!("Erro ao serializar ficha: {}", e)))?;

    let summary = char_data.to_summary(String::new(), char_data.is_public, true);
    let summary_json = serde_json::to_string(&summary).unwrap_or_default();

    // 4. Salva no banco de dados SQLite
    sqlx::query("UPDATE character_sheets SET data = ?, summary_json = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(&updated_json)
        .bind(&summary_json)
        .bind(&sheet_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao salvar alterações na ficha: {}", e)))?;

    // Busca o updated_at atualizado
    let updated_at: String = sqlx::query_scalar("SELECT updated_at FROM character_sheets WHERE id = ?")
        .bind(&sheet_id)
        .fetch_one(&pool)
        .await
        .unwrap_or_default();

    let summary = build_room_sheet_summary(
        sheet_id,
        sheet_name,
        &updated_json,
        updated_at,
        is_hidden,
        is_owner,
    );

    crate::logging::server::write_log(
        crate::logging::LogCategory::UserActions,
        "INFO",
        &format!("GM QUICK STAT UPDATE: Ficha '{}' ({}) atualizada pelo Narrador na sala '{}'", summary.name, summary.id, room_id),
        None,
    );

    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "ssr")]
    #[test]
    fn test_room_code_generation() {
        let code = generate_room_code();
        assert!(code.starts_with("MTA-"));
        assert_eq!(code.len(), 8);
        assert!(code.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'));
    }

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
