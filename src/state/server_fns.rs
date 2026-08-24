use leptos::*;
use super::models::{CharacterData, CharacterSummary};

// ==========================================
// Server Functions with Robust Error Handling
// ==========================================

#[server(endpoint = "get_sheets")]
pub async fn get_sheets() -> Result<Vec<CharacterSummary>, ServerFnError> {
    use sqlx::{SqlitePool, Row};
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in get_sheets", None);
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None);
    if auth_user_id.is_none() {
        // Deslogado não possui fichas privadas
        return Ok(Vec::new());
    }
    let user_id = auth_user_id.unwrap_or_default();

    let start = std::time::Instant::now();
    let rows = sqlx::query("SELECT id, name, data, is_public, updated_at FROM character_sheets WHERE user_id = ? ORDER BY updated_at DESC")
        .bind(&user_id)
        .fetch_all(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to fetch sheets from DB", Some(&e.to_string()));
            ServerFnError::new(format!("Falha ao consultar fichas: {}", e))
        })?;

    let count = rows.len();
    let summaries = rows.into_iter().map(|row| {
        let id: String = row.get("id");
        let name: String = row.get("name");
        let data_json: String = row.get("data");
        let is_public: bool = row.get::<i32, _>("is_public") == 1;
        let updated_at: String = row.get("updated_at");

        let mut tradition = String::new();
        let mut essence = String::new();
        let mut arete = 1;
        let mut willpower = 5;
        let mut photo_url = String::new();
        let mut spheres = Vec::new();
        let mut sheet_type = row.try_get::<String, _>("sheet_type").unwrap_or_else(|_| "mage".to_string());

        if let Ok(data) = serde_json::from_str::<CharacterData>(&data_json) {
            if sheet_type.is_empty() || sheet_type == "mage" {
                sheet_type = data.sheet_type.clone();
            }
            if data.is_gods_and_monsters() {
                tradition = data.labels.get("Type").cloned().unwrap_or_else(|| "Familiar / Bygone".to_string());
                essence = data.labels.get("Concept").cloned().unwrap_or_default();
                arete = data.get_attribute_level("Gnosis", 0);
            } else {
                tradition = data.labels.get("Tradição").cloned().unwrap_or_default();
                essence = data.labels.get("Essência").cloned().unwrap_or_default();
                arete = data.get_attribute_level(crate::state::models::keys::KEY_ARETE, 1);
            }
            willpower = data.get_attribute_level(crate::state::models::keys::KEY_WILLPOWER_TOTAL, 5);
            photo_url = if !data.visuals.character_sketch_url.is_empty() {
                data.visuals.character_sketch_url.clone()
            } else {
                data.get_profile_photo()
            };
            for sphere in crate::state::models::STANDARD_SPHERES {
                let lvl = data.get_attribute_level(sphere, 0);
                spheres.push((sphere.to_string(), lvl));
            }
        } else if let Some(data) = CharacterData::from_raw_json_resilient(&id, &data_json) {
            if sheet_type.is_empty() || sheet_type == "mage" {
                sheet_type = data.sheet_type.clone();
            }
            if data.is_gods_and_monsters() {
                tradition = data.labels.get("Type").cloned().unwrap_or_else(|| "Familiar / Bygone".to_string());
                essence = data.labels.get("Concept").cloned().unwrap_or_default();
                arete = data.get_attribute_level("Gnosis", 0);
            } else {
                tradition = data.labels.get("Tradição").cloned().unwrap_or_default();
                essence = data.labels.get("Essência").cloned().unwrap_or_default();
                arete = data.get_attribute_level(crate::state::models::keys::KEY_ARETE, 1);
            }
            willpower = data.get_attribute_level(crate::state::models::keys::KEY_WILLPOWER_TOTAL, 5);
            photo_url = if !data.visuals.character_sketch_url.is_empty() {
                data.visuals.character_sketch_url.clone()
            } else {
                data.get_profile_photo()
            };
            for sphere in crate::state::models::STANDARD_SPHERES {
                let lvl = data.get_attribute_level(sphere, 0);
                spheres.push((sphere.to_string(), lvl));
            }
        } else {
            for sphere in crate::state::models::STANDARD_SPHERES {
                spheres.push((sphere.to_string(), 0));
            }
        }

        CharacterSummary {
            id,
            name,
            tradition,
            essence,
            arete,
            willpower,
            photo_url,
            spheres,
            sheet_type,
            is_public,
            is_owner: true,
            updated_at,
        }
    }).collect();

    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("SELECT character_sheets: retornou {} fichas do usuário '{}' em {}ms", count, user_id, start.elapsed().as_millis()),
        None,
    );

    Ok(summaries)
}

#[server(endpoint = "get_public_sheets")]
pub async fn get_public_sheets() -> Result<Vec<CharacterSummary>, ServerFnError> {
    use sqlx::{SqlitePool, Row};
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in get_public_sheets", None);
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None);
    let start = std::time::Instant::now();

    let rows = sqlx::query("SELECT id, user_id, name, data, sheet_type, is_public, updated_at FROM character_sheets WHERE is_public = 1 ORDER BY updated_at DESC LIMIT 100")
        .fetch_all(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to fetch public sheets from DB", Some(&e.to_string()));
            ServerFnError::new(format!("Falha ao consultar fichas públicas: {}", e))
        })?;

    let count = rows.len();
    let summaries = rows.into_iter().map(|row| {
        let id: String = row.get("id");
        let owner_id: Option<String> = row.get("user_id");
        let name: String = row.get("name");
        let data_json: String = row.get("data");
        let updated_at: String = row.get("updated_at");
        let is_owner = auth_user_id.is_some() && auth_user_id == owner_id;

        let mut tradition = String::new();
        let mut essence = String::new();
        let mut arete = 1;
        let mut willpower = 5;
        let mut photo_url = String::new();
        let mut spheres = Vec::new();
        let mut sheet_type = row.try_get::<String, _>("sheet_type").unwrap_or_else(|_| "mage".to_string());

        if let Ok(data) = serde_json::from_str::<CharacterData>(&data_json) {
            if sheet_type.is_empty() || sheet_type == "mage" {
                sheet_type = data.sheet_type.clone();
            }
            if data.is_gods_and_monsters() {
                tradition = data.labels.get("Type").cloned().unwrap_or_else(|| "Familiar / Bygone".to_string());
                essence = data.labels.get("Concept").cloned().unwrap_or_default();
                arete = data.get_attribute_level("Gnosis", 0);
            } else {
                tradition = data.labels.get("Tradição").cloned().unwrap_or_default();
                essence = data.labels.get("Essência").cloned().unwrap_or_default();
                arete = data.get_attribute_level(crate::state::models::keys::KEY_ARETE, 1);
            }
            willpower = data.get_attribute_level(crate::state::models::keys::KEY_WILLPOWER_TOTAL, 5);
            photo_url = if !data.visuals.character_sketch_url.is_empty() {
                data.visuals.character_sketch_url.clone()
            } else {
                data.get_profile_photo()
            };
            for sphere in crate::state::models::STANDARD_SPHERES {
                let lvl = data.get_attribute_level(sphere, 0);
                spheres.push((sphere.to_string(), lvl));
            }
        } else if let Some(data) = CharacterData::from_raw_json_resilient(&id, &data_json) {
            if sheet_type.is_empty() || sheet_type == "mage" {
                sheet_type = data.sheet_type.clone();
            }
            if data.is_gods_and_monsters() {
                tradition = data.labels.get("Type").cloned().unwrap_or_else(|| "Familiar / Bygone".to_string());
                essence = data.labels.get("Concept").cloned().unwrap_or_default();
                arete = data.get_attribute_level("Gnosis", 0);
            } else {
                tradition = data.labels.get("Tradição").cloned().unwrap_or_default();
                essence = data.labels.get("Essência").cloned().unwrap_or_default();
                arete = data.get_attribute_level(crate::state::models::keys::KEY_ARETE, 1);
            }
            willpower = data.get_attribute_level(crate::state::models::keys::KEY_WILLPOWER_TOTAL, 5);
            photo_url = if !data.visuals.character_sketch_url.is_empty() {
                data.visuals.character_sketch_url.clone()
            } else {
                data.get_profile_photo()
            };
            for sphere in crate::state::models::STANDARD_SPHERES {
                let lvl = data.get_attribute_level(sphere, 0);
                spheres.push((sphere.to_string(), lvl));
            }
        } else {
            for sphere in crate::state::models::STANDARD_SPHERES {
                spheres.push((sphere.to_string(), 0));
            }
        }

        CharacterSummary {
            id,
            name,
            tradition,
            essence,
            arete,
            willpower,
            photo_url,
            spheres,
            sheet_type,
            is_public: true,
            is_owner,
            updated_at,
        }
    }).collect();

    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("SELECT public character_sheets: retornou {} fichas em {}ms", count, start.elapsed().as_millis()),
        None,
    );

    Ok(summaries)
}

#[server(endpoint = "get_sheet")]
pub async fn get_sheet(id: String) -> Result<CharacterData, ServerFnError> {
    if id.trim().is_empty() {
        return Err(ServerFnError::new("ID da ficha não fornecido"));
    }

    use sqlx::{SqlitePool, Row};
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in get_sheet", Some(&format!("id={}", id)));
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None);

    let start = std::time::Instant::now();
    let row = sqlx::query("SELECT user_id, room_id, data, sheet_type, is_public FROM character_sheets WHERE id = ?")
        .bind(&id)
        .fetch_optional(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Error querying sheet {}", id), Some(&e.to_string()));
            ServerFnError::new(format!("Erro ao buscar ficha no banco: {}", e))
        })?
        .ok_or_else(|| {
            crate::logging::server::write_log(crate::logging::LogCategory::Requests, "WARN", &format!("Sheet with id {} not found", id), None);
            ServerFnError::new(format!("Ficha com ID '{}' não encontrada", id))
        })?;

    let sheet_user_id: Option<String> = row.get("user_id");
    let room_id: Option<String> = row.get("room_id");
    let sheet_type_db = row.try_get::<String, _>("sheet_type").unwrap_or_else(|_| "mage".to_string());
    let is_public: bool = row.get::<i32, _>("is_public") == 1;

    let is_owner = auth_user_id.is_some() && auth_user_id == sheet_user_id;
    let mut is_gm = false;
    if let (Some(u_id), Some(r_id)) = (&auth_user_id, &room_id) {
        if let Ok(Some(room)) = sqlx::query("SELECT gm_id FROM rooms WHERE id = ?").bind(r_id).fetch_optional(&pool).await {
            is_gm = room.get::<String, _>("gm_id") == *u_id;
        }
    }

    // Validação de Permissão de Leitura
    if !is_owner && !is_gm && !is_public && sheet_user_id.is_some() {
        return Err(ServerFnError::new("Permissão negada: Esta ficha é privada e pertence a outro usuário."));
    }

    let data_json: String = row.get("data");
    let mut data: CharacterData = match serde_json::from_str(&data_json) {
        Ok(d) => d,
        Err(e) => {
            crate::logging::server::write_log(
                crate::logging::LogCategory::Errors,
                "WARN",
                &format!("JSON parsing falhou para ficha {}. Tentando recuperação resiliente...", id),
                Some(&e.to_string()),
            );
            CharacterData::from_raw_json_resilient(&id, &data_json).ok_or_else(|| {
                crate::logging::server::write_log(
                    crate::logging::LogCategory::Errors,
                    "ERROR",
                    &format!("Corrupted JSON for sheet {}", id),
                    Some(&e.to_string()),
                );
                ServerFnError::new(format!("Dados da ficha corrompidos: {}", e))
            })?
        }
    };

    if (data.sheet_type.is_empty() || data.sheet_type == "mage") && !sheet_type_db.is_empty() && sheet_type_db != "mage" {
        data.sheet_type = sheet_type_db;
    }
    data.is_public = is_public;
    data.sanitize();
    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("SELECT character_sheets id='{}' (nome='{}', public={}) carregada com sucesso em {}ms", id, data.name, is_public, start.elapsed().as_millis()),
        None,
    );

    Ok(data)
}

pub fn validate_image_magic_bytes(bytes: &[u8]) -> Result<(&'static str, &'static str), ServerFnError> {
    if bytes.len() < 4 {
        return Err(ServerFnError::new("Arquivo muito pequeno para ser uma imagem válida"));
    }

    // PNG: 89 50 4E 47 0D 0A 1A 0A
    if bytes.len() >= 8 && &bytes[0..8] == b"\x89PNG\r\n\x1a\n" {
        return Ok(("image/png", "png"));
    }

    // JPEG / JPG: FF D8 FF
    if bytes.len() >= 3 && &bytes[0..3] == b"\xFF\xD8\xFF" {
        return Ok(("image/jpeg", "jpg"));
    }

    // GIF: GIF87a or GIF89a
    if bytes.len() >= 6 && (&bytes[0..6] == b"GIF87a" || &bytes[0..6] == b"GIF89a") {
        return Ok(("image/gif", "gif"));
    }

    // WEBP: RIFF....WEBP
    if bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        return Ok(("image/webp", "webp"));
    }

    // SVG / XML text (allow <svg or <?xml ... <svg)
    if bytes.len() >= 4 {
        let snippet = String::from_utf8_lossy(&bytes[0..std::cmp::min(bytes.len(), 512)]).to_lowercase();
        if snippet.contains("<svg") {
            return Ok(("image/svg+xml", "svg"));
        }
    }

    Err(ServerFnError::new("Formato de arquivo não suportado. Apenas imagens autênticas PNG, JPEG, WebP, GIF e SVG são permitidas."))
}

#[cfg(feature = "ssr")]
async fn verify_sheet_write_permission(pool: &sqlx::SqlitePool, sheet_id: &str) -> Result<(), ServerFnError> {
    use sqlx::Row;
    let auth_user_id = crate::auth::get_auth_user_id().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let row = sqlx::query("SELECT user_id, room_id FROM character_sheets WHERE id = ?")
        .bind(sheet_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if let Some(r) = row {
        let sheet_owner: Option<String> = r.get("user_id");
        let room_id: Option<String> = r.get("room_id");

        // If sheet has an owner, verify if caller is owner or room GM
        if let Some(owner_id) = sheet_owner {
            if let Some(user_id) = auth_user_id {
                if user_id == owner_id {
                    return Ok(());
                }

                // Check if user is GM of the room
                if let Some(r_id) = room_id {
                    let is_gm = sqlx::query("SELECT 1 FROM rooms WHERE id = ? AND gm_id = ?")
                        .bind(r_id)
                        .bind(&user_id)
                        .fetch_optional(pool)
                        .await
                        .map_err(|e| ServerFnError::new(e.to_string()))?;
                    if is_gm.is_some() {
                        return Ok(());
                    }
                }

                return Err(ServerFnError::new("Permissão negada: Você não é o proprietário desta ficha"));
            } else {
                return Err(ServerFnError::new("Autenticação necessária para alterar esta ficha"));
            }
        }
    }

    Ok(())
}

#[server(endpoint = "import_sheet")]
pub async fn import_sheet(data: CharacterData) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::SqlitePool;
        use uuid::Uuid;

        let pool = use_context::<SqlitePool>().ok_or_else(|| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in import_sheet", None);
            ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
        })?;

        let start = std::time::Instant::now();
        let new_id = Uuid::new_v4().to_string();
        let mut imported_data = data;
        imported_data.id = new_id.clone();

        let raw_name = imported_data.name.trim().to_string();
        let final_name = if raw_name.is_empty() {
            "Ficha Importada".to_string()
        } else {
            raw_name
        };

        let s_type = imported_data.sheet_type.clone();

        let data_json = serde_json::to_string(&imported_data).map_err(|e: serde_json::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Serialization error importing sheet", Some(&e.to_string()));
            ServerFnError::new(format!("Falha ao serializar dados importados: {}", e))
        })?;

        let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None);

        sqlx::query("INSERT INTO character_sheets (id, user_id, name, data, sheet_type) VALUES (?, ?, ?, ?, ?)")
            .bind(&new_id)
            .bind(auth_user_id)
            .bind(&final_name)
            .bind(data_json)
            .bind(&s_type)
            .execute(&pool)
            .await
            .map_err(|e: sqlx::Error| {
                crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Failed to insert imported sheet {}", new_id), Some(&e.to_string()));
                ServerFnError::new(format!("Falha ao salvar ficha importada no banco: {}", e))
            })?;

        crate::logging::server::write_log(
            crate::logging::LogCategory::UserActions,
            "INFO",
            &format!("IMPORT SHEET: Ficha importada com sucesso id='{}', tipo='{}', nome='{}' em {}ms", new_id, s_type, final_name, start.elapsed().as_millis()),
            None,
        );

        Ok(new_id)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = data;
        Err(ServerFnError::new("Disponível apenas no servidor"))
    }
}

#[server(endpoint = "create_sheet")]
pub async fn create_sheet(name: String, sheet_type: Option<String>) -> Result<String, ServerFnError> {
    let clean_name = name.trim().to_string();
    let s_type = sheet_type.unwrap_or_else(|| "mage".to_string());
    let default_name = if s_type == "gods_and_monsters" { "New Monster / Familiar" } else { "Novo Mago" };
    let final_name = if clean_name.is_empty() { default_name.to_string() } else { clean_name };

    use sqlx::SqlitePool;
    use uuid::Uuid;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in create_sheet", None);
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let start = std::time::Instant::now();
    let id = Uuid::new_v4().to_string();
    let initial_data = if s_type == "gods_and_monsters" {
        CharacterData::new_gods_and_monsters(id.clone(), final_name.clone())
    } else {
        CharacterData::new(id.clone(), final_name.clone())
    };

    let data_json = serde_json::to_string(&initial_data).map_err(|e: serde_json::Error| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Serialization error creating sheet", Some(&e.to_string()));
        ServerFnError::new(format!("Falha ao serializar dados iniciais: {}", e))
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None);

    sqlx::query("INSERT INTO character_sheets (id, user_id, name, data, sheet_type) VALUES (?, ?, ?, ?, ?)")
        .bind(&id)
        .bind(auth_user_id)
        .bind(&final_name)
        .bind(data_json)
        .bind(&s_type)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Failed to insert new sheet {}", id), Some(&e.to_string()));
            ServerFnError::new(format!("Falha ao salvar nova ficha no banco: {}", e))
        })?;

    crate::logging::server::write_log(
        crate::logging::LogCategory::UserActions,
        "INFO",
        &format!("CREATE SHEET: Nova ficha criada id='{}', tipo='{}', nome='{}' em {}ms", id, s_type, final_name, start.elapsed().as_millis()),
        None,
    );

    Ok(id)
}

#[server(endpoint = "update_sheet")]
pub async fn update_sheet(id: String, data: CharacterData) -> Result<(), ServerFnError> {
    if id.trim().is_empty() {
        return Err(ServerFnError::new("ID da ficha não pode ser vazio"));
    }

    let mut data = data;
    data.sanitize();

    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in update_sheet", Some(&format!("id={}", id)));
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    // Check ownership / GM permission
    verify_sheet_write_permission(&pool, &id).await?;

    let start = std::time::Instant::now();
    let data_json = serde_json::to_string(&data).map_err(|e: serde_json::Error| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Serialization error updating sheet {}", id), Some(&e.to_string()));
        ServerFnError::new(format!("Falha ao serializar dados da ficha: {}", e))
    })?;

    let payload_kb = (data_json.len() as f64) / 1024.0;
    let is_public_int = if data.is_public { 1 } else { 0 };
    let result = sqlx::query("UPDATE character_sheets SET name = ?, data = ?, sheet_type = ?, is_public = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(&data.name)
        .bind(data_json)
        .bind(&data.sheet_type)
        .bind(is_public_int)
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Failed to update sheet {}", id), Some(&e.to_string()));
            ServerFnError::new(format!("Falha ao atualizar dados da ficha no banco: {}", e))
        })?;

    if result.rows_affected() == 0 {
        crate::logging::server::write_log(crate::logging::LogCategory::Requests, "WARN", &format!("Ficha com ID '{}' não encontrada para atualização", id), None);
        return Err(ServerFnError::new(format!("Ficha com ID '{}' não encontrada para atualização", id)));
    }

    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("UPDATE character_sheets id='{}' (nome='{}', public={}) salva com sucesso em {}ms ({:.1} KB)", id, data.name, data.is_public, start.elapsed().as_millis(), payload_kb),
        None,
    );

    Ok(())
}

#[server(endpoint = "set_sheet_visibility")]
pub async fn set_sheet_visibility(id: String, is_public: bool) -> Result<(), ServerFnError> {
    if id.trim().is_empty() {
        return Err(ServerFnError::new("ID da ficha não fornecido"));
    }

    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in set_sheet_visibility", None);
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    verify_sheet_write_permission(&pool, &id).await?;

    let is_public_int = if is_public { 1 } else { 0 };
    let result = sqlx::query("UPDATE character_sheets SET is_public = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(is_public_int)
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Failed to update sheet visibility {}", id), Some(&e.to_string()));
            ServerFnError::new(format!("Falha ao atualizar visibilidade no banco: {}", e))
        })?;

    if result.rows_affected() == 0 {
        return Err(ServerFnError::new(format!("Ficha com ID '{}' não encontrada", id)));
    }

    crate::logging::server::write_log(
        crate::logging::LogCategory::UserActions,
        "INFO",
        &format!("SET VISIBILITY: Ficha id='{}' visibilidade alterada para is_public={}", id, is_public),
        None,
    );

    Ok(())
}

#[server(endpoint = "delete_sheet")]
pub async fn delete_sheet(id: String) -> Result<(), ServerFnError> {
    if id.trim().is_empty() {
        return Err(ServerFnError::new("ID da ficha não pode ser vazio"));
    }

    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in delete_sheet", Some(&format!("id={}", id)));
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    // Check ownership / GM permission
    verify_sheet_write_permission(&pool, &id).await?;

    let start = std::time::Instant::now();
    let result = sqlx::query("DELETE FROM character_sheets WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Failed to delete sheet {}", id), Some(&e.to_string()));
            ServerFnError::new(format!("Falha ao excluir ficha do banco: {}", e))
        })?;

    if result.rows_affected() == 0 {
        return Err(ServerFnError::new(format!("Ficha com ID '{}' não encontrada", id)));
    }

    crate::logging::server::write_log(
        crate::logging::LogCategory::UserActions,
        "INFO",
        &format!("DELETE SHEET: Ficha '{}' excluída com sucesso em {}ms", id, start.elapsed().as_millis()),
        None,
    );

    Ok(())
}

#[server(endpoint = "save_uploaded_media")]
pub async fn save_uploaded_media(
    sheet_id: String,
    block: String,
    file_name: String,
    data_base64: String,
) -> Result<String, ServerFnError> {
    use sqlx::SqlitePool;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Banco de dados indisponível")
    })?;

    let clean_sheet_id = if sheet_id.trim().is_empty() { "temp".to_string() } else { sheet_id.trim().to_string() };
    let clean_block = if block.trim().is_empty() { "wonders".to_string() } else { block.trim().to_string() };

    // Check ownership / GM permission if sheet exists
    if clean_sheet_id != "temp" {
        verify_sheet_write_permission(&pool, &clean_sheet_id).await?;
    }

    let (_mime_hint, base64_payload) = if let Some(idx) = data_base64.find(";base64,") {
        let mime = if data_base64.starts_with("data:") {
            &data_base64[5..idx]
        } else {
            "image/webp"
        };
        let payload = &data_base64[idx + 8..];
        (mime.to_string(), payload)
    } else {
        ("image/webp".to_string(), data_base64.as_str())
    };

    use base64::Engine;
    let bytes = base64::engine::general_purpose::STANDARD.decode(base64_payload.trim())
        .map_err(|e| ServerFnError::new(format!("Base64 inválido: {}", e)))?;

    if bytes.len() > 10 * 1024 * 1024 {
        return Err(ServerFnError::new("A imagem excede o limite máximo de 10MB"));
    }

    // Strict Magic Bytes Validation
    let (mime_type, ext) = validate_image_magic_bytes(&bytes)?;

    let asset_id = format!("img_{}", uuid::Uuid::new_v4());
    let safe_filename = if file_name.trim().is_empty() {
        format!("{}.{}", asset_id, ext)
    } else {
        let sanitized_name: String = file_name.chars().filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_').collect();
        if sanitized_name.is_empty() {
            format!("{}.{}", asset_id, ext)
        } else {
            format!("{}_{}.{}", sanitized_name, asset_id, ext)
        }
    };

    let dir_path = format!("uploads/sheets/{}/{}", clean_sheet_id, clean_block);
    let file_path = format!("{}/{}", dir_path, safe_filename);
    let relative_url = format!("/uploads/sheets/{}/{}/{}", clean_sheet_id, clean_block, safe_filename);

    tokio::fs::create_dir_all(&dir_path).await
        .map_err(|e| ServerFnError::new(format!("Falha ao criar diretório de upload: {}", e)))?;
    tokio::fs::write(&file_path, &bytes).await
        .map_err(|e| ServerFnError::new(format!("Falha ao gravar arquivo em disco: {}", e)))?;

    sqlx::query(
        "INSERT OR REPLACE INTO media_assets (id, sheet_id, block, file_path, mime_type, size_bytes, data_blob) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&asset_id)
    .bind(&clean_sheet_id)
    .bind(&clean_block)
    .bind(&file_path)
    .bind(mime_type)
    .bind(bytes.len() as i64)
    .bind(&bytes)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Falha ao registrar backup no banco: {}", e)))?;

    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("Upload de imagem salvo com sucesso: {} ({} bytes)", relative_url, bytes.len()),
        None,
    );

    Ok(relative_url)
}
