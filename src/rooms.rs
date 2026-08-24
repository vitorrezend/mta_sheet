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
    pub updated_at: String,
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

    // 1. Get room info
    let room_row = sqlx::query(
        "SELECT r.id, r.name, r.code, r.description, r.gm_id, u.username as gm_username 
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

    // 3. Get sheets
    let sheet_rows = sqlx::query(
        "SELECT id, name, data, updated_at FROM character_sheets WHERE room_id = ? ORDER BY updated_at DESC"
    )
    .bind(&room_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let sheets = sheet_rows.into_iter().map(|r| {
        let id: String = r.get("id");
        let name: String = r.get("name");
        let data_json: String = r.get("data");
        let updated_at: String = r.get("updated_at");

        let char_data: CharacterData = serde_json::from_str(&data_json).unwrap_or_default();
        let (wp_total, wp_cur) = char_data.get_willpower();
        let (quint, paradox, _) = char_data.get_quintessence_paradox();

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
        members,
        sheets,
    })
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
}
