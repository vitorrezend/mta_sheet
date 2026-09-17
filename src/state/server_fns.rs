use leptos::*;
use super::models::{CharacterData, CharacterSummary, QuizQuestionEntry, SheetFolder, FolderAclEntry};

// ==========================================
// Server Functions with Robust Error Handling
// ==========================================

#[cfg(feature = "ssr")]
fn parse_summary_or_fallback(
    id: &str,
    name: &str,
    summary_opt: Option<&str>,
    fallback_data: &str,
    sheet_type: &str,
    updated_at: &str,
    is_public: bool,
    is_owner: bool,
    to_backfill: &mut Vec<(String, String)>,
) -> CharacterSummary {
    if let Some(summary_str) = summary_opt.filter(|s| !s.trim().is_empty()) {
        if let Ok(mut s) = serde_json::from_str::<CharacterSummary>(summary_str) {
            s.id = id.to_string();
            s.updated_at = updated_at.to_string();
            s.is_public = is_public;
            s.is_owner = is_owner;
            if !sheet_type.is_empty() {
                s.sheet_type = sheet_type.to_string();
            }
            return s;
        }
    }

    // Fallback para linhas legadas que ainda não possuem summary_json
    if let Some(mut data) = CharacterData::parse_from_db(id, fallback_data) {
        if data.id.is_empty() {
            data.id = id.to_string();
        }
        if data.name.is_empty() || (data.name == "Novo Mago" && !name.is_empty() && name != "Novo Mago") {
            data.set_display_name(name);
        }
        data.sanitize();
        if (data.sheet_type.is_empty() || data.sheet_type == "mage") && !sheet_type.is_empty() && sheet_type != "mage" {
            data.sheet_type = sheet_type.to_string();
        }
        let summary = data.to_summary(updated_at.to_string(), is_public, is_owner);
        if let Ok(s_json) = serde_json::to_string(&summary) {
            to_backfill.push((id.to_string(), s_json));
        }
        summary
    } else {
        let clean_name = if name.trim().is_empty() { "Novo Mago".to_string() } else { name.to_string() };
        CharacterSummary::fallback(id.to_string(), clean_name, updated_at.to_string(), is_public, is_owner)
    }
}

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
    let rows = sqlx::query(
        "SELECT id, name, summary_json, is_public, updated_at, folder_id, sheet_type, \
         CASE WHEN summary_json IS NULL OR summary_json = '' THEN data ELSE '' END as fallback_data \
         FROM character_sheets WHERE user_id = ? ORDER BY updated_at DESC"
    )
        .bind(&user_id)
        .fetch_all(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to fetch sheets from DB", Some(&e.to_string()));
            ServerFnError::new("Falha ao consultar fichas. Tente novamente mais tarde.")
        })?;

    let count = rows.len();
    let mut to_backfill: Vec<(String, String)> = Vec::new();

    let summaries: Vec<CharacterSummary> = rows.into_iter().map(|row| {
        let id: String = row.get("id");
        let name: String = row.get("name");
        let summary_opt: Option<String> = row.try_get("summary_json").unwrap_or(None);
        let fallback_data: String = row.try_get("fallback_data").unwrap_or_default();
        let is_public: bool = row.get::<i32, _>("is_public") == 1;
        let updated_at: String = row.get("updated_at");
        let sheet_type = row.try_get::<String, _>("sheet_type").unwrap_or_else(|_| "mage".to_string());
        let folder_id = row.try_get::<Option<String>, _>("folder_id").unwrap_or(None);

        let mut summary = parse_summary_or_fallback(
            &id,
            &name,
            summary_opt.as_deref(),
            &fallback_data,
            &sheet_type,
            &updated_at,
            is_public,
            true,
            &mut to_backfill,
        );
        summary.folder_id = folder_id;
        summary
    }).collect();

    // Auto-migra em segundo plano as fichas legadas que ainda não tinham summary_json
    for (sheet_id, s_json) in to_backfill {
        let _ = sqlx::query("UPDATE character_sheets SET summary_json = ? WHERE id = ?")
            .bind(s_json)
            .bind(sheet_id)
            .execute(&pool)
            .await;
    }

    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("SELECT character_sheets: retornou {} fichas do usuário '{}' em {}ms (otimizado via summary_json)", count, user_id, start.elapsed().as_millis()),
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

    let rows = sqlx::query(
        "SELECT id, user_id, name, summary_json, sheet_type, is_public, updated_at, \
         CASE WHEN summary_json IS NULL OR summary_json = '' THEN data ELSE '' END as fallback_data \
         FROM character_sheets WHERE is_public = 1 ORDER BY updated_at DESC LIMIT 100"
    )
        .fetch_all(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to fetch public sheets from DB", Some(&e.to_string()));
            ServerFnError::new("Falha ao consultar fichas públicas. Tente novamente mais tarde.")
        })?;

    let count = rows.len();
    let mut to_backfill: Vec<(String, String)> = Vec::new();

    let summaries: Vec<CharacterSummary> = rows.into_iter().map(|row| {
        let id: String = row.get("id");
        let owner_id: Option<String> = row.get("user_id");
        let name: String = row.get("name");
        let summary_opt: Option<String> = row.try_get("summary_json").unwrap_or(None);
        let fallback_data: String = row.try_get("fallback_data").unwrap_or_default();
        let updated_at: String = row.get("updated_at");
        let sheet_type = row.try_get::<String, _>("sheet_type").unwrap_or_else(|_| "mage".to_string());
        let is_owner = auth_user_id.is_some() && auth_user_id == owner_id;

        parse_summary_or_fallback(
            &id,
            &name,
            summary_opt.as_deref(),
            &fallback_data,
            &sheet_type,
            &updated_at,
            true,
            is_owner,
            &mut to_backfill,
        )
    }).collect();

    for (sheet_id, s_json) in to_backfill {
        let _ = sqlx::query("UPDATE character_sheets SET summary_json = ? WHERE id = ?")
            .bind(s_json)
            .bind(sheet_id)
            .execute(&pool)
            .await;
    }

    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("SELECT public character_sheets: retornou {} fichas em {}ms (otimizado via summary_json)", count, start.elapsed().as_millis()),
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
            ServerFnError::new("Erro ao buscar ficha no banco de dados.")
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
    let mut data: CharacterData = match CharacterData::parse_from_db(&id, &data_json) {
        Some(d) => d,
        None => {
            crate::logging::server::write_log(
                crate::logging::LogCategory::Errors,
                "ERROR",
                &format!("Corrupted JSON for sheet {}", id),
                None,
            );
            return Err(ServerFnError::new("Dados da ficha corrompidos no banco de dados."));
        }
    };

    if (data.sheet_type.is_empty() || data.sheet_type == "mage") && !sheet_type_db.is_empty() && sheet_type_db != "mage" {
        data.sheet_type = sheet_type_db;
    }
    data.is_public = is_public;
    data.sanitize();

    // Carrega respostas relacionais salvas na tabela character_quiz_answers
    if let Ok(quiz_rows) = sqlx::query("SELECT question_id, answer FROM character_quiz_answers WHERE character_id = ?")
        .bind(&id)
        .fetch_all(&pool)
        .await
    {
        for r in quiz_rows {
            let q_id: String = r.get("question_id");
            let ans: String = r.get("answer");
            if let Some(entry) = data.quiz_data.entries.iter_mut().find(|e| e.id == q_id) {
                entry.answer = ans;
            }
        }
    }

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
        imported_data.sanitize();

        let display_name = imported_data.get_display_name();
        let final_name = if display_name.is_empty() || display_name == "Novo Mago" || display_name == "Sem Nome" {
            let raw_name = imported_data.name.trim();
            if !raw_name.is_empty() && raw_name != "Novo Mago" && raw_name != "Sem Nome" {
                raw_name.to_string()
            } else {
                "Ficha Importada".to_string()
            }
        } else {
            display_name
        };
        imported_data.name = final_name.clone();
        imported_data.labels.insert(crate::state::keys::HEADER_NOME.to_string(), final_name.clone());

        let s_type = imported_data.sheet_type.clone();

        let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None);

        // Limite de cota de 50 fichas por conta
        if let Some(ref uid) = auth_user_id {
            let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM character_sheets WHERE user_id = ?")
                .bind(uid)
                .fetch_one(&pool)
                .await
                .unwrap_or(0);
            if count >= 50 {
                return Err(ServerFnError::new("Limite de 50 fichas por conta atingido. Exclua fichas antigas para importar novas."));
            }
        }

        let data_json = serde_json::to_string(&imported_data).map_err(|e: serde_json::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Serialization error importing sheet", Some(&e.to_string()));
            ServerFnError::new("Falha ao serializar dados importados.")
        })?;

        // Validação de limite de 5MB por JSON
        if data_json.len() > 5 * 1024 * 1024 {
            return Err(ServerFnError::new("O arquivo JSON excede o limite máximo permitido de 5 MB"));
        }

        let summary = imported_data.to_summary(String::new(), false, true);
        let summary_json = serde_json::to_string(&summary).unwrap_or_default();

        sqlx::query("INSERT INTO character_sheets (id, user_id, name, data, sheet_type, summary_json) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&new_id)
            .bind(auth_user_id)
            .bind(&final_name)
            .bind(data_json)
            .bind(&s_type)
            .bind(&summary_json)
            .execute(&pool)
            .await
            .map_err(|e: sqlx::Error| {
                crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Failed to insert imported sheet {}", new_id), Some(&e.to_string()));
                ServerFnError::new("Falha ao salvar ficha importada no banco de dados.")
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
pub async fn create_sheet(
    name: String,
    sheet_type: Option<String>,
    folder_id: Option<String>,
) -> Result<String, ServerFnError> {
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

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None);

    // Limite de cota de 50 fichas por conta
    if let Some(ref uid) = auth_user_id {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM character_sheets WHERE user_id = ?")
            .bind(uid)
            .fetch_one(&pool)
            .await
            .unwrap_or(0);
        if count >= 50 {
            return Err(ServerFnError::new("Limite de 50 fichas por conta atingido. Exclua fichas antigas para criar novas."));
        }
    }

    let valid_folder_id = if let (Some(uid), Some(fid)) = (&auth_user_id, &folder_id) {
        let exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM sheet_folders WHERE id = ? AND user_id = ?")
            .bind(fid)
            .bind(uid)
            .fetch_one(&pool)
            .await
            .unwrap_or(0);
        if exists > 0 { Some(fid.clone()) } else { None }
    } else {
        None
    };

    let start = std::time::Instant::now();
    let id = Uuid::new_v4().to_string();
    let initial_data = if s_type == "gods_and_monsters" {
        CharacterData::new_gods_and_monsters(id.clone(), final_name.clone())
    } else {
        CharacterData::new(id.clone(), final_name.clone())
    };
    let resolved_name = initial_data.get_display_name();

    let data_json = serde_json::to_string(&initial_data).map_err(|e: serde_json::Error| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Serialization error creating sheet", Some(&e.to_string()));
        ServerFnError::new("Falha ao serializar dados iniciais da ficha.")
    })?;

    let summary = initial_data.to_summary(String::new(), false, true);
    let summary_json = serde_json::to_string(&summary).unwrap_or_default();

    sqlx::query("INSERT INTO character_sheets (id, user_id, name, data, sheet_type, folder_id, summary_json) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(&id)
        .bind(auth_user_id)
        .bind(&resolved_name)
        .bind(data_json)
        .bind(&s_type)
        .bind(valid_folder_id)
        .bind(&summary_json)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Failed to insert new sheet {}", id), Some(&e.to_string()));
            ServerFnError::new("Falha ao salvar nova ficha no banco de dados.")
        })?;

    crate::logging::server::write_log(
        crate::logging::LogCategory::UserActions,
        "INFO",
        &format!("CREATE SHEET: Nova ficha criada id='{}', tipo='{}', nome='{}' em {}ms", id, s_type, resolved_name, start.elapsed().as_millis()),
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
    let display_name = data.get_display_name();
    data.name = display_name.clone();
    data.labels.insert(crate::state::keys::HEADER_NOME.to_string(), display_name.clone());

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
        ServerFnError::new("Falha ao serializar dados da ficha.")
    })?;

    // Limite máximo de 5MB por ficha no banco
    if data_json.len() > 5 * 1024 * 1024 {
        return Err(ServerFnError::new("O tamanho dos dados da ficha excede o limite máximo permitido de 5 MB"));
    }

    let payload_kb = (data_json.len() as f64) / 1024.0;
    let is_public_int = if data.is_public { 1 } else { 0 };
    let summary = data.to_summary(String::new(), data.is_public, true);
    let summary_json = serde_json::to_string(&summary).unwrap_or_default();

    let result = sqlx::query("UPDATE character_sheets SET name = ?, data = ?, sheet_type = ?, is_public = ?, summary_json = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(&display_name)
        .bind(data_json)
        .bind(&data.sheet_type)
        .bind(is_public_int)
        .bind(&summary_json)
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Failed to update sheet {}", id), Some(&e.to_string()));
            ServerFnError::new("Falha ao atualizar dados da ficha no banco de dados.")
        })?;

    if result.rows_affected() == 0 {
        crate::logging::server::write_log(crate::logging::LogCategory::Requests, "WARN", &format!("Ficha com ID '{}' não encontrada para atualização", id), None);
        return Err(ServerFnError::new(format!("Ficha com ID '{}' não encontrada para atualização", id)));
    }

    // Sincroniza respostas relacionais na tabela character_quiz_answers
    for entry in &data.quiz_data.entries {
        let clean_ans = entry.answer.trim();
        if clean_ans.is_empty() {
            let _ = sqlx::query("DELETE FROM character_quiz_answers WHERE character_id = ? AND question_id = ?")
                .bind(&id)
                .bind(&entry.id)
                .execute(&pool)
                .await;
        } else {
            let _ = sqlx::query(
                "INSERT INTO character_quiz_answers (character_id, question_id, answer, updated_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP)
                 ON CONFLICT(character_id, question_id) DO UPDATE SET answer = excluded.answer, updated_at = CURRENT_TIMESTAMP"
            )
            .bind(&id)
            .bind(&entry.id)
            .bind(clean_ans)
            .execute(&pool)
            .await;
        }
    }

    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("UPDATE character_sheets id='{}' (nome='{}', public={}) salva com sucesso em {}ms ({:.1} KB)", id, data.name, data.is_public, start.elapsed().as_millis(), payload_kb),
        None,
    );

    Ok(())
}

#[server(endpoint = "get_quiz_questions")]
pub async fn get_quiz_questions(sheet_type: Option<String>) -> Result<Vec<QuizQuestionEntry>, ServerFnError> {
    use sqlx::{SqlitePool, Row};
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco indisponível")
    })?;

    let splat = sheet_type.unwrap_or_else(|| "mage".to_string());
    let rows = sqlx::query("SELECT id, title, prompt, category FROM quiz_questions WHERE splat = ? ORDER BY sort_order ASC")
        .bind(&splat)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to fetch quiz questions", Some(&e.to_string()));
            ServerFnError::new("Erro ao consultar perguntas do questionário.")
        })?;

    if rows.is_empty() {
        return Ok(crate::state::models::default_quiz_questions());
    }

    let mut list = Vec::with_capacity(rows.len());
    for r in rows {
        list.push(QuizQuestionEntry {
            id: r.get("id"),
            title: r.get("title"),
            prompt: r.get("prompt"),
            answer: String::new(),
            category: r.get("category"),
        });
    }

    Ok(list)
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
            ServerFnError::new("Falha ao atualizar visibilidade no banco de dados.")
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
            ServerFnError::new("Falha ao excluir ficha do banco de dados.")
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

    // Limite estrito de 5MB por imagem
    if bytes.len() > 5 * 1024 * 1024 {
        return Err(ServerFnError::new("A imagem excede o limite máximo de 5MB"));
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

// ==========================================
// Folder Management Server Functions
// ==========================================

#[server(endpoint = "get_folders")]
pub async fn get_folders() -> Result<Vec<SheetFolder>, ServerFnError> {
    use sqlx::{SqlitePool, Row};
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in get_folders", None);
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None);
    if auth_user_id.is_none() {
        return Ok(Vec::new());
    }
    let user_id = auth_user_id.unwrap_or_default();

    let rows = sqlx::query(
        "SELECT f.id, f.user_id, f.parent_id, f.name, f.icon, f.color, f.sort_order, f.created_at, 
                COUNT(s.id) as sheet_count
         FROM sheet_folders f
         LEFT JOIN character_sheets s ON s.folder_id = f.id
         WHERE f.user_id = ?
         GROUP BY f.id
         ORDER BY f.sort_order ASC, f.created_at ASC"
    )
    .bind(&user_id)
    .fetch_all(&pool)
    .await
    .map_err(|e: sqlx::Error| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to fetch folders", Some(&e.to_string()));
        ServerFnError::new("Falha ao consultar pastas.")
    })?;

    let folders = rows.into_iter().map(|row| {
        SheetFolder {
            id: row.get("id"),
            user_id: row.get("user_id"),
            parent_id: row.get("parent_id"),
            name: row.get("name"),
            icon: row.get("icon"),
            color: row.get("color"),
            sort_order: row.get("sort_order"),
            sheet_count: row.get::<i64, _>("sheet_count"),
            created_at: row.get("created_at"),
        }
    }).collect();

    Ok(folders)
}

#[server(endpoint = "create_folder")]
pub async fn create_folder(
    name: String,
    icon: Option<String>,
    color: Option<String>,
    parent_id: Option<String>,
) -> Result<SheetFolder, ServerFnError> {
    use sqlx::SqlitePool;
    use uuid::Uuid;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Você precisa estar autenticado para criar pastas."))?;

    let clean_name = name.trim().to_string();
    if clean_name.is_empty() {
        return Err(ServerFnError::new("O nome da pasta não pode ser vazio."));
    }
    if clean_name.len() > 60 {
        return Err(ServerFnError::new("O nome da pasta não pode ter mais de 60 caracteres."));
    }

    if let Some(ref pid) = parent_id {
        let parent_owner = sqlx::query_scalar::<_, String>("SELECT user_id FROM sheet_folders WHERE id = ?")
            .bind(pid)
            .fetch_optional(&pool)
            .await
            .map_err(|_| ServerFnError::new("Erro ao validar pasta pai."))?;
        match parent_owner {
            Some(owner_id) if owner_id == auth_user_id => {},
            _ => return Err(ServerFnError::new("Pasta pai não encontrada ou acesso negado.")),
        }
    }

    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM sheet_folders WHERE user_id = ?")
        .bind(&auth_user_id)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);
    if count >= 60 {
        return Err(ServerFnError::new("Limite de 60 pastas por conta atingido."));
    }

    let id = Uuid::new_v4().to_string();
    let folder_icon = icon.unwrap_or_else(|| "📁".to_string());
    let folder_color = color.unwrap_or_else(|| "#b89347".to_string());

    sqlx::query(
        "INSERT INTO sheet_folders (id, user_id, parent_id, name, icon, color, sort_order) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&auth_user_id)
    .bind(&parent_id)
    .bind(&clean_name)
    .bind(&folder_icon)
    .bind(&folder_color)
    .bind(count as i32)
    .execute(&pool)
    .await
    .map_err(|e: sqlx::Error| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to create folder", Some(&e.to_string()));
        ServerFnError::new("Falha ao criar pasta no banco de dados.")
    })?;

    Ok(SheetFolder {
        id,
        user_id: auth_user_id,
        parent_id,
        name: clean_name,
        icon: folder_icon,
        color: folder_color,
        sort_order: count as i32,
        sheet_count: 0,
        created_at: String::new(),
    })
}

#[server(endpoint = "move_folder")]
pub async fn move_folder(folder_id: String, target_parent_id: Option<String>) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado."))?;

    // 1. Verify folder exists and belongs to user
    let folder_owner = sqlx::query_scalar::<_, String>("SELECT user_id FROM sheet_folders WHERE id = ?")
        .bind(&folder_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| ServerFnError::new("Erro ao buscar pasta."))?;

    match folder_owner {
        Some(owner_id) if owner_id == auth_user_id => {},
        _ => return Err(ServerFnError::new("Pasta não encontrada ou acesso não autorizado.")),
    }

    // 2. If moving to another folder, check self and cycle prevention
    if let Some(ref target_id) = target_parent_id {
        if target_id == &folder_id {
            return Err(ServerFnError::new("Não é possível mover uma pasta para dentro de si mesma."));
        }

        let target_owner = sqlx::query_scalar::<_, String>("SELECT user_id FROM sheet_folders WHERE id = ?")
            .bind(target_id)
            .fetch_optional(&pool)
            .await
            .map_err(|_| ServerFnError::new("Erro ao buscar pasta destino."))?;

        match target_owner {
            Some(owner_id) if owner_id == auth_user_id => {},
            _ => return Err(ServerFnError::new("Pasta destino não encontrada ou acesso negado.")),
        }

        // Cycle check: verify if folder_id is an ancestor of target_id
        let is_descendant = sqlx::query(
            "WITH RECURSIVE ancestry AS (
                SELECT id, parent_id FROM sheet_folders WHERE id = ?
                UNION ALL
                SELECT f.id, f.parent_id FROM sheet_folders f JOIN ancestry a ON f.id = a.parent_id
            )
            SELECT 1 FROM ancestry WHERE id = ? LIMIT 1"
        )
        .bind(target_id)
        .bind(&folder_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| ServerFnError::new("Erro ao validar hierarquia de pastas."))?;

        if is_descendant.is_some() {
            return Err(ServerFnError::new("Operação inválida: não é possível mover uma pasta para dentro de uma de suas subpastas."));
        }
    }

    // 3. Update parent_id
    sqlx::query("UPDATE sheet_folders SET parent_id = ? WHERE id = ? AND user_id = ?")
        .bind(&target_parent_id)
        .bind(&folder_id)
        .bind(&auth_user_id)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to move folder", Some(&e.to_string()));
            ServerFnError::new("Falha ao mover pasta.")
        })?;

    Ok(())
}

#[server(endpoint = "update_folder")]
pub async fn update_folder(folder_id: String, name: String, icon: Option<String>, color: Option<String>) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado."))?;

    let clean_name = name.trim().to_string();
    if clean_name.is_empty() {
        return Err(ServerFnError::new("O nome da pasta não pode ser vazio."));
    }
    if clean_name.len() > 60 {
        return Err(ServerFnError::new("O nome da pasta não pode ter mais de 60 caracteres."));
    }

    let folder_icon = icon.unwrap_or_else(|| "📁".to_string());
    let folder_color = color.unwrap_or_else(|| "#b89347".to_string());

    let res = sqlx::query(
        "UPDATE sheet_folders SET name = ?, icon = ?, color = ? WHERE id = ? AND user_id = ?"
    )
    .bind(&clean_name)
    .bind(&folder_icon)
    .bind(&folder_color)
    .bind(&folder_id)
    .bind(&auth_user_id)
    .execute(&pool)
    .await
    .map_err(|e: sqlx::Error| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to update folder", Some(&e.to_string()));
        ServerFnError::new("Falha ao atualizar pasta.")
    })?;

    if res.rows_affected() == 0 {
        return Err(ServerFnError::new("Pasta não encontrada ou acesso não autorizado."));
    }

    Ok(())
}

#[server(endpoint = "delete_folder")]
pub async fn delete_folder(folder_id: String) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado."))?;

    // Safely unlink all character sheets in this folder AND all its recursive subfolders to root
    let _ = sqlx::query(
        "WITH RECURSIVE subfolders AS (
            SELECT id FROM sheet_folders WHERE id = ? AND user_id = ?
            UNION ALL
            SELECT f.id FROM sheet_folders f JOIN subfolders s ON f.parent_id = s.id
        )
        UPDATE character_sheets SET folder_id = NULL WHERE folder_id IN (SELECT id FROM subfolders)"
    )
    .bind(&folder_id)
    .bind(&auth_user_id)
    .execute(&pool)
    .await;

    let res = sqlx::query("DELETE FROM sheet_folders WHERE id = ? AND user_id = ?")
        .bind(&folder_id)
        .bind(&auth_user_id)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to delete folder", Some(&e.to_string()));
            ServerFnError::new("Falha ao excluir pasta.")
        })?;

    if res.rows_affected() == 0 {
        return Err(ServerFnError::new("Pasta não encontrada ou acesso não autorizado."));
    }

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn check_folder_permission_internal(
    pool: &sqlx::SqlitePool,
    folder_id: &str,
    user_id: &str,
    required_permission: &str,
) -> Result<bool, sqlx::Error> {
    // 1. Check direct owner
    let owner = sqlx::query_scalar::<_, String>("SELECT user_id FROM sheet_folders WHERE id = ?")
        .bind(folder_id)
        .fetch_optional(pool)
        .await?;

    if let Some(ref oid) = owner {
        if oid == user_id {
            return Ok(true);
        }
    } else {
        return Ok(false);
    }

    // 2. Recursive walk-up resolution through folder ancestry
    let row = sqlx::query(
        "WITH RECURSIVE folder_ancestry AS (
            SELECT id, parent_id, 0 AS depth
            FROM sheet_folders WHERE id = ?
            UNION ALL
            SELECT f.id, f.parent_id, a.depth + 1
            FROM sheet_folders f
            JOIN folder_ancestry a ON f.id = a.parent_id
        )
        SELECT acl.permission
        FROM folder_acls acl
        JOIN folder_ancestry fa ON acl.folder_id = fa.id
        WHERE (acl.grantee_type = 'user' AND acl.grantee_id = ?)
           OR (acl.grantee_type = 'public')
           OR (acl.grantee_type = 'room' AND acl.grantee_id IN (SELECT room_id FROM room_members WHERE user_id = ?))
        ORDER BY fa.depth ASC,
                 CASE acl.permission WHEN 'admin' THEN 3 WHEN 'write' THEN 2 WHEN 'read' THEN 1 ELSE 0 END DESC
        LIMIT 1"
    )
    .bind(folder_id)
    .bind(user_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    if let Some(r) = row {
        use sqlx::Row;
        let perm: String = r.get("permission");
        let satisfies = match (required_permission, perm.as_str()) {
            ("read", "read" | "write" | "admin") => true,
            ("write", "write" | "admin") => true,
            ("admin", "admin") => true,
            _ => false,
        };
        return Ok(satisfies);
    }

    Ok(false)
}

#[server(endpoint = "move_sheet_to_folder")]
pub async fn move_sheet_to_folder(sheet_id: String, folder_id: Option<String>) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado."))?;

    if let Some(ref fid) = folder_id {
        let is_allowed = check_folder_permission_internal(&pool, fid, &auth_user_id, "write").await
            .map_err(|_| ServerFnError::new("Erro ao validar permissões na pasta de destino."))?;

        if !is_allowed {
            return Err(ServerFnError::new("Pasta de destino não encontrada ou acesso negado."));
        }
    }

    let res = sqlx::query("UPDATE character_sheets SET folder_id = ? WHERE id = ? AND user_id = ?")
        .bind(&folder_id)
        .bind(&sheet_id)
        .bind(&auth_user_id)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to move sheet to folder", Some(&e.to_string()));
            ServerFnError::new("Falha ao mover ficha para a pasta.")
        })?;

    if res.rows_affected() == 0 {
        return Err(ServerFnError::new("Ficha não encontrada ou acesso não autorizado."));
    }

    Ok(())
}

#[server(endpoint = "get_folder_acls")]
pub async fn get_folder_acls(folder_id: String) -> Result<Vec<FolderAclEntry>, ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado."))?;

    // Check ownership or admin
    let is_allowed = check_folder_permission_internal(&pool, &folder_id, &auth_user_id, "admin").await
        .map_err(|_| ServerFnError::new("Erro ao verificar permissão de administração da pasta."))?;

    if !is_allowed {
        return Err(ServerFnError::new("Apenas o proprietário ou administrador pode gerenciar permissões da pasta."));
    }

    let rows = sqlx::query(
        "SELECT a.id, a.folder_id, a.grantee_type, a.grantee_id, a.permission, a.created_at,
                COALESCE(u.username, r.name, 'Público') as grantee_name
         FROM folder_acls a
         LEFT JOIN users u ON a.grantee_type = 'user' AND a.grantee_id = u.id
         LEFT JOIN rooms r ON a.grantee_type = 'room' AND a.grantee_id = r.id
         WHERE a.folder_id = ?
         ORDER BY a.created_at ASC"
    )
    .bind(&folder_id)
    .fetch_all(&pool)
    .await
    .map_err(|e: sqlx::Error| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to fetch folder ACLs", Some(&e.to_string()));
        ServerFnError::new("Falha ao consultar permissões da pasta.")
    })?;

    let entries = rows.into_iter().map(|r| {
        use sqlx::Row;
        FolderAclEntry {
            id: r.get("id"),
            folder_id: r.get("folder_id"),
            grantee_type: r.get("grantee_type"),
            grantee_id: r.get("grantee_id"),
            grantee_name: r.get("grantee_name"),
            permission: r.get("permission"),
            created_at: r.get("created_at"),
        }
    }).collect();

    Ok(entries)
}

#[server(endpoint = "grant_folder_acl")]
pub async fn grant_folder_acl(
    folder_id: String,
    grantee_type: String,
    grantee_identifier: String,
    permission: String,
) -> Result<FolderAclEntry, ServerFnError> {
    use sqlx::SqlitePool;
    use uuid::Uuid;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado."))?;

    let is_allowed = check_folder_permission_internal(&pool, &folder_id, &auth_user_id, "admin").await
        .map_err(|_| ServerFnError::new("Erro ao verificar permissão."))?;

    if !is_allowed {
        return Err(ServerFnError::new("Apenas o proprietário ou administrador pode compartilhar a pasta."));
    }

    let clean_perm = match permission.to_lowercase().as_str() {
        "admin" => "admin".to_string(),
        "write" | "editor" => "write".to_string(),
        _ => "read".to_string(),
    };

    let (grantee_id, grantee_name) = match grantee_type.to_lowercase().as_str() {
        "user" => {
            let clean_user = grantee_identifier.trim();
            if clean_user.is_empty() {
                return Err(ServerFnError::new("Digite o nome de usuário para compartilhar."));
            }
            let u_row = sqlx::query("SELECT id, username FROM users WHERE LOWER(username) = LOWER(?)")
                .bind(clean_user)
                .fetch_optional(&pool)
                .await
                .map_err(|_| ServerFnError::new("Erro ao buscar usuário."))?;

            match u_row {
                Some(r) => {
                    use sqlx::Row;
                    let uid: String = r.get("id");
                    let uname: String = r.get("username");
                    if uid == auth_user_id {
                        return Err(ServerFnError::new("Você já é o proprietário desta pasta."));
                    }
                    (Some(uid), uname)
                }
                None => return Err(ServerFnError::new(format!("Usuário '{}' não encontrado.", clean_user))),
            }
        }
        "room" => {
            let clean_room = grantee_identifier.trim();
            if clean_room.is_empty() {
                return Err(ServerFnError::new("Selecione uma sala para compartilhar."));
            }
            let r_row = sqlx::query("SELECT id, name FROM rooms WHERE id = ? OR code = ?")
                .bind(clean_room)
                .bind(clean_room)
                .fetch_optional(&pool)
                .await
                .map_err(|_| ServerFnError::new("Erro ao buscar sala."))?;

            match r_row {
                Some(r) => {
                    use sqlx::Row;
                    let rid: String = r.get("id");
                    let rname: String = r.get("name");
                    (Some(rid), rname)
                }
                None => return Err(ServerFnError::new("Sala de crônica não encontrada.")),
            }
        }
        "public" => (None, "Público".to_string()),
        _ => return Err(ServerFnError::new("Tipo de destinatário inválido.")),
    };

    let acl_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO folder_acls (id, folder_id, grantee_type, grantee_id, permission)
         VALUES (?, ?, ?, ?, ?)
         ON CONFLICT(folder_id, grantee_type, grantee_id) DO UPDATE SET permission = excluded.permission"
    )
    .bind(&acl_id)
    .bind(&folder_id)
    .bind(&grantee_type)
    .bind(&grantee_id)
    .bind(&clean_perm)
    .execute(&pool)
    .await
    .map_err(|e: sqlx::Error| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to grant folder ACL", Some(&e.to_string()));
        ServerFnError::new("Falha ao salvar permissão da pasta.")
    })?;

    Ok(FolderAclEntry {
        id: acl_id,
        folder_id,
        grantee_type,
        grantee_id,
        grantee_name,
        permission: clean_perm,
        created_at: String::new(),
    })
}

#[server(endpoint = "revoke_folder_acl")]
pub async fn revoke_folder_acl(acl_id: String) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado."))?;

    // Check ownership of the folder this ACL belongs to
    let folder_id = sqlx::query_scalar::<_, String>(
        "SELECT folder_id FROM folder_acls WHERE id = ?"
    )
    .bind(&acl_id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| ServerFnError::new("Erro ao buscar permissão."))?;

    let fid = match folder_id {
        Some(f) => f,
        None => return Err(ServerFnError::new("Regra de permissão não encontrada.")),
    };

    let is_allowed = check_folder_permission_internal(&pool, &fid, &auth_user_id, "admin").await
        .map_err(|_| ServerFnError::new("Erro ao verificar permissão."))?;

    if !is_allowed {
        return Err(ServerFnError::new("Acesso negado: apenas o proprietário ou administrador pode revogar permissões."));
    }

    sqlx::query("DELETE FROM folder_acls WHERE id = ?")
        .bind(&acl_id)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to revoke folder ACL", Some(&e.to_string()));
            ServerFnError::new("Falha ao revogar permissão.")
        })?;

    Ok(())
}

