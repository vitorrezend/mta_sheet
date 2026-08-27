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
    pub concept: String,
    pub arete: i32,
    pub willpower_total: i32,
    pub willpower_current: i32,
    pub quintessence: i32,
    pub paradox: i32,
    pub photo_url: String,
    pub health_label: String,
    pub health_penalty: String,
    pub health_badge_class: String,
    pub health_damage_str: String,
    pub is_hidden: bool,
    pub is_owner: bool,
    #[serde(default = "default_initiative_attr")]
    pub dexterity: i32,
    #[serde(default = "default_initiative_attr")]
    pub wits: i32,
    #[serde(default = "default_initiative_base")]
    pub initiative_base: i32,
    pub updated_at: String,
}

fn default_initiative_attr() -> i32 { 1 }
fn default_initiative_base() -> i32 { 2 }

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct InitiativeEntry {
    pub id: String,
    pub name: String,
    pub is_npc: bool,
    pub is_active: bool,
    pub base_dex: i32,
    pub base_wits: i32,
    pub base_total: i32,
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
pub struct RoomDetails {
    pub id: String,
    pub name: String,
    pub code: String,
    pub description: String,
    pub gm_id: String,
    pub gm_username: String,
    pub is_gm: bool,
    pub chantry: ChantryPoolData,
    pub chronicle_notes: String,
    pub initiative: RoomInitiativeData,
    pub members: Vec<RoomMemberInfo>,
    pub sheets: Vec<RoomSheetSummary>,
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
            r.id, r.name, r.code, r.description, r.created_at,
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
        member_count: row.get("member_count"),
        sheet_count: row.get("sheet_count"),
        created_at: row.get("created_at"),
    }).collect();

    Ok(summaries)
}

#[server(endpoint = "create_room")]
pub async fn create_room(name: String, description: String) -> Result<String, ServerFnError> {
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

    sqlx::query(
        "INSERT INTO rooms (id, name, code, description, gm_id) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&room_id)
    .bind(&clean_name)
    .bind(&code)
    .bind(&description)
    .bind(&user_id)
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

    log::info!("Created room '{}' with code {}", clean_name, code);
    Ok(room_id)
}

#[server(endpoint = "join_room_by_code")]
pub async fn join_room_by_code(code: String) -> Result<String, ServerFnError> {
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

    let room_row = sqlx::query("SELECT id, name FROM rooms WHERE code = ?")
        .bind(&clean_code)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Código de sala inválido ou sala não encontrada"))?;

    let room_id: String = room_row.get("id");
    let room_name: String = room_row.get("name");

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

#[server(endpoint = "get_room_details")]
pub async fn get_room_details(room_id: String) -> Result<RoomDetails, ServerFnError> {
    if room_id.trim().is_empty() {
        return Err(ServerFnError::new("ID da sala não fornecido"));
    }

    use sqlx::{SqlitePool, Row};
    use crate::auth::get_auth_user_id;
    use crate::state::CharacterData;

    let current_user_id = get_auth_user_id().await?.unwrap_or_default();

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    // 1. Get room info including chantry, chronicle notes, and initiative
    let room_row = sqlx::query(
        "SELECT r.id, r.name, r.code, r.description, r.gm_id, r.chantry_data, r.chronicle_notes, r.initiative_data, u.username as gm_username 
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

    let chantry_raw: String = room_row.try_get("chantry_data").unwrap_or_default();
    let chantry: ChantryPoolData = serde_json::from_str(&chantry_raw).unwrap_or_default();
    let chronicle_notes: String = room_row.try_get("chronicle_notes").unwrap_or_default();
    let initiative_raw: String = room_row.try_get("initiative_data").unwrap_or_default();
    let initiative: RoomInitiativeData = if !initiative_raw.is_empty() {
        serde_json::from_str(&initiative_raw).unwrap_or_default()
    } else {
        RoomInitiativeData::default()
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
    let sheet_query = if is_gm {
        "SELECT id, user_id, name, data, is_hidden_in_room, updated_at FROM character_sheets WHERE room_id = ? ORDER BY updated_at DESC"
    } else {
        "SELECT id, user_id, name, data, is_hidden_in_room, updated_at FROM character_sheets WHERE room_id = ? AND (is_hidden_in_room = 0 OR user_id = ?) ORDER BY updated_at DESC"
    };

    let mut q = sqlx::query(sheet_query).bind(&room_id);
    if !is_gm {
        q = q.bind(&current_user_id);
    }

    let sheet_rows = q.fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let sheets = sheet_rows.into_iter().map(|r| {
        let id: String = r.get("id");
        let sheet_user_id: Option<String> = r.get("user_id");
        let is_owner = !current_user_id.is_empty() && sheet_user_id.as_deref() == Some(&current_user_id);
        let is_hidden: bool = r.try_get::<i64, _>("is_hidden_in_room").map(|v| v == 1).unwrap_or(false);
        let name: String = r.get("name");
        let data_json: String = r.get("data");
        let updated_at: String = r.get("updated_at");

        let char_data: CharacterData = serde_json::from_str(&data_json).unwrap_or_default();
        let (wp_total, wp_cur) = char_data.get_willpower();
        let (quint, paradox, _) = char_data.get_quintessence_paradox();

        // Calculate health status
        let (agg, lethal, bashing) = char_data.get_health_counts();
        let total_dmg = agg + lethal + bashing;
        let (health_label, health_penalty, health_badge_class) = match total_dmg {
            0 => ("Íntegro", "0", "health-full"),
            1 => ("Escoriado", "-0", "health-bruised"),
            2 => ("Ferido", "-1", "health-hurt"),
            3 => ("Gravemente Ferido", "-1", "health-injured"),
            4 => ("Espancado", "-2", "health-wounded"),
            5 => ("Estropiado", "-2", "health-mauled"),
            6 => ("Aleijado", "-5", "health-crippled"),
            _ => ("Incapacitado", "☠️", "health-incapacitated"),
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

        let dexterity = char_data.get_attribute_level("Destreza", 1);
        let wits = char_data.get_attribute_level("Raciocínio", 1);
        let initiative_base = dexterity + wits;

        RoomSheetSummary {
            id,
            name,
            player_name: char_data.get_label(crate::state::keys::HEADER_JOGADOR),
            tradition: char_data.get_label(crate::state::keys::HEADER_TRADICAO),
            concept: char_data.get_label(crate::state::keys::HEADER_CONCEITO),
            arete: char_data.get_arete(),
            willpower_total: wp_total,
            willpower_current: wp_cur,
            quintessence: quint,
            paradox,
            photo_url,
            health_label: health_label.to_string(),
            health_penalty: health_penalty.to_string(),
            health_badge_class: health_badge_class.to_string(),
            health_damage_str,
            is_hidden,
            is_owner,
            dexterity,
            wits,
            initiative_base,
            updated_at,
        }
    }).collect();

    Ok(RoomDetails {
        id: room_row.get("id"),
        name: room_row.get("name"),
        code: room_row.get("code"),
        description: room_row.get("description"),
        gm_id,
        gm_username: room_row.get("gm_username"),
        is_gm,
        chantry,
        chronicle_notes,
        initiative,
        members,
        sheets,
    })
}

#[server(endpoint = "update_room_initiative")]
pub async fn update_room_initiative(room_id: String, initiative: RoomInitiativeData) -> Result<(), ServerFnError> {
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
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    sqlx::query("UPDATE character_sheets SET room_id = NULL WHERE id = ?")
        .bind(&sheet_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao desvincular ficha: {}", e)))?;

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

    // 3. Busca a ficha original do Narrador
    let sheet_row = sqlx::query("SELECT name, data, sheet_type, photo_url FROM characters WHERE id = ? AND user_id = ?")
        .bind(&sheet_id)
        .bind(&caller_user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao buscar ficha original: {}", e)))?;

    let sheet_data = sheet_row.ok_or_else(|| ServerFnError::new("Ficha original não encontrada no seu inventário"))?;
    let sheet_name: String = sheet_data.get("name");
    let sheet_json: String = sheet_data.get("data");
    let sheet_type: String = sheet_data.get("sheet_type");
    let photo_url: Option<String> = sheet_data.get("photo_url");

    // 4. Cria nova ficha com novo UUID pertencente ao jogador
    let new_sheet_id = uuid::Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO characters (id, user_id, name, data, sheet_type, photo_url, is_public, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, 0, datetime('now'), datetime('now'))")
        .bind(&new_sheet_id)
        .bind(&target_user_id)
        .bind(&sheet_name)
        .bind(&sheet_json)
        .bind(&sheet_type)
        .bind(&photo_url)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao criar ficha clonada: {}", e)))?;

    // 5. Vincula a nova ficha à sala automaticamente
    sqlx::query("INSERT OR IGNORE INTO room_sheets (room_id, sheet_id, is_hidden, created_at) VALUES (?, ?, 0, datetime('now'))")
        .bind(&room_id)
        .bind(&new_sheet_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao vincular ficha à mesa: {}", e)))?;

    Ok(new_sheet_id)
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
            chantry: chantry.clone(),
            chronicle_notes: "Sessão 1: Encontro na Avenida Paulista".to_string(),
            initiative: RoomInitiativeData::default(),
            members: vec![],
            sheets: vec![],
        };

        let json = serde_json::to_string(&details).expect("serialize");
        let deserialized: RoomDetails = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(details, deserialized);
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
}
