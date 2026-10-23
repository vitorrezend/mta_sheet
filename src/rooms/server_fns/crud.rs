use leptos::*;
use super::super::models::{RoomSummary, RoomDetails, ChantryPoolData};
#[allow(unused_imports)]
use super::super::map::RoomMapData;
#[cfg(feature = "ssr")]
use super::super::models::{RoomInitiativeData, RoomMemberInfo, RoomSheetSummary, build_room_sheet_summary};

#[server(endpoint = "get_user_rooms")]
pub async fn get_user_rooms() -> Result<Vec<RoomSummary>, ServerFnError> {
    use sqlx::SqlitePool;
    use crate::auth::get_auth_user_id;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Faça login para ver suas salas")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    crate::repositories::RoomRepository::list_user_rooms(&pool, &user_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(endpoint = "get_public_rooms")]
pub async fn get_public_rooms() -> Result<Vec<RoomSummary>, ServerFnError> {
    use sqlx::SqlitePool;
    use crate::auth::get_auth_user_id;

    let current_user_id = get_auth_user_id().await?.unwrap_or_default();

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    crate::repositories::RoomRepository::list_public_rooms(&pool, &current_user_id, 50)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
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
    let code = crate::rooms::events::generate_room_code();

    let password_hash = if let Some(ref p) = password {
        let p_trimmed = p.trim();
        if !p_trimmed.is_empty() {
            crate::auth::hash_password(p_trimmed, 8).await.map_err(|e| ServerFnError::new(format!("Erro ao processar senha: {}", e)))?
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    crate::repositories::RoomRepository::create_room(
        &pool,
        &room_id,
        &clean_name,
        &code,
        &description,
        &user_id,
        is_public,
        &password_hash,
    )
    .await
    .map_err(|e| ServerFnError::new(format!("Erro ao criar sala: {}", e)))?;

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

    // Limita tentativas por usuário: impede varrer códigos ou senhas de sala por tentativa e erro
    if !crate::server::middleware::security::check_action_rate_limit(&format!("join:{}", user_id), 10, std::time::Duration::from_secs(60)) {
        return Err(ServerFnError::new("Muitas tentativas de entrar em salas. Aguarde 1 minuto."));
    }

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
        let valid = crate::auth::verify_password(pwd_input, &password_hash).await.unwrap_or(false);
        if !valid {
            return Err(ServerFnError::new("Senha incorreta para esta sala."));
        }
    }

    // Add member if not exists
    crate::repositories::RoomRepository::add_member_if_not_exists(&pool, &room_id, &user_id, "player")
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao entrar na sala: {}", e)))?;

    log::info!("User joined room '{}' ({})", room_name, room_id);
    crate::rooms::events::notify_room_changed(&room_id);
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
        let valid = crate::auth::verify_password(pwd_input, &password_hash).await.unwrap_or(false);
        if !valid {
            return Err(ServerFnError::new("Senha incorreta para esta sala."));
        }
    }

    crate::repositories::RoomRepository::add_member_if_not_exists(&pool, &room_id, &user_id, "player")
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao entrar na sala: {}", e)))?;

    log::info!("User joined public room '{}' ({})", room_name, room_id);
    crate::rooms::events::notify_room_changed(&room_id);
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

    let is_gm = crate::repositories::RoomRepository::check_is_gm(&pool, &room_id, &user_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

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
            let hash = crate::auth::hash_password(p_trimmed, 8).await.map_err(|e| ServerFnError::new(e.to_string()))?;
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

    crate::rooms::events::notify_room_changed(&room_id);
    Ok(())
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
        "SELECT r.id, r.name, r.code, r.description, r.gm_id, r.chantry_data, r.chronicle_notes, r.chantry_version, r.chronicle_version, r.initiative_data, r.map_data, r.sheet_order, r.is_public,
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
    let chantry_version: i64 = room_row.try_get("chantry_version").unwrap_or(0);
    let chronicle_version: i64 = room_row.try_get("chronicle_version").unwrap_or(0);
    let initiative_raw: String = room_row.try_get("initiative_data").unwrap_or_default();
    let mut initiative: RoomInitiativeData = if !initiative_raw.is_empty() {
        serde_json::from_str(&initiative_raw).unwrap_or_default()
    } else {
        RoomInitiativeData::default()
    };
    let map_raw: String = room_row.try_get("map_data").unwrap_or_default();
    let mut map_data: RoomMapData = if !map_raw.is_empty() {
        serde_json::from_str(&map_raw).unwrap_or_default()
    } else {
        RoomMapData::default()
    };
    map_data.normalize();
    // Tokens ocultos pelo mestre não saem do servidor para os jogadores
    let map_data = if is_gm { map_data } else { map_data.visible_to_players() };
    let sheet_order_raw: String = room_row.try_get("sheet_order").unwrap_or_default();
    let sheet_order: Vec<String> = if !sheet_order_raw.is_empty() {
        serde_json::from_str(&sheet_order_raw).unwrap_or_default()
    } else {
        Vec::new()
    };

    // 2. Get members
    let members: Vec<RoomMemberInfo> = crate::repositories::RoomRepository::get_room_members(&pool, &room_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // 3. Get sheets (GM sees all, players only see non-hidden sheets or their own)
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
        chantry_version,
        chronicle_version,
        initiative,
        map_data,
        members,
        sheets,
        sheet_order,
    })
}

#[server(endpoint = "update_room_chantry")]
/// Salva a Capela se ninguém a alterou desde `expected_version`; retorna a nova versão.
pub async fn update_room_chantry(room_id: String, chantry: ChantryPoolData, expected_version: i64) -> Result<i64, ServerFnError> {
    use sqlx::SqlitePool;
    use crate::auth::get_auth_user_id;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para atualizar os recursos da capela")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let allowed = crate::repositories::RoomRepository::check_is_gm_or_member(&pool, &room_id, &user_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    if !allowed {
        return Err(ServerFnError::new("Permissão negada: apenas o mestre e os membros da sala podem editar a capela"));
    }

    let mut chantry = chantry;
    chantry.notes = crate::state::rich_text::sanitize_if_html(&chantry.notes).unwrap_or(chantry.notes);
    let json_data = serde_json::to_string(&chantry).unwrap_or_default();

    let new_version: Option<i64> = sqlx::query_scalar(
        "UPDATE rooms SET chantry_data = ?, chantry_version = chantry_version + 1 WHERE id = ? AND chantry_version = ? RETURNING chantry_version",
    )
    .bind(&json_data)
    .bind(&room_id)
    .bind(expected_version)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Erro ao atualizar capela: {}", e)))?;
    let version = new_version.ok_or_else(|| ServerFnError::new(format!(
        "{}: outra pessoa salvou a capela antes de você", crate::errors::VERSION_CONFLICT_PREFIX
    )))?;

    crate::rooms::events::broadcast_to_room(&room_id, crate::rooms::RoomBroadcastEvent::ChantryUpdated { chantry, version });
    Ok(version)
}

#[server(endpoint = "update_room_chronicle_notes")]
/// Salva o Diário se ninguém o alterou desde `expected_version`; retorna a nova versão.
pub async fn update_room_chronicle_notes(room_id: String, notes: String, expected_version: i64) -> Result<i64, ServerFnError> {
    use sqlx::SqlitePool;
    use crate::auth::get_auth_user_id;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para editar o diário da crônica")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let allowed = crate::repositories::RoomRepository::check_is_gm_or_member(&pool, &room_id, &user_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    if !allowed {
        return Err(ServerFnError::new("Permissão negada: apenas o mestre e os membros da sala podem editar o diário da crônica"));
    }

    let notes = crate::state::rich_text::sanitize_if_html(&notes).unwrap_or(notes);

    let new_version: Option<i64> = sqlx::query_scalar(
        "UPDATE rooms SET chronicle_notes = ?, chronicle_version = chronicle_version + 1 WHERE id = ? AND chronicle_version = ? RETURNING chronicle_version",
    )
    .bind(&notes)
    .bind(&room_id)
    .bind(expected_version)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Erro ao atualizar diário da crônica: {}", e)))?;
    let version = new_version.ok_or_else(|| ServerFnError::new(format!(
        "{}: outra pessoa salvou o diário antes de você", crate::errors::VERSION_CONFLICT_PREFIX
    )))?;

    crate::rooms::events::broadcast_to_room(&room_id, crate::rooms::RoomBroadcastEvent::ChronicleUpdated { notes, version });
    Ok(version)
}

