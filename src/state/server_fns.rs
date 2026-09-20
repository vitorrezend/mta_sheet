use leptos::*;
use super::models::{CharacterData, CharacterSummary, QuizQuestionEntry};

// ==========================================
// Server Functions with Robust Error Handling
// ==========================================

#[cfg(feature = "ssr")]
use crate::repositories::SheetRepository;

#[server(endpoint = "get_sheets")]
pub async fn get_sheets() -> Result<Vec<CharacterSummary>, ServerFnError> {
    use sqlx::SqlitePool;
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
    let summaries = SheetRepository::list_by_user(&pool, &user_id).await.map_err(|e| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to fetch sheets from DB", Some(&e.to_string()));
        ServerFnError::new("Falha ao consultar fichas. Tente novamente mais tarde.")
    })?;

    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("SELECT character_sheets: retornou {} fichas do usuário '{}' em {}ms (otimizado via summary_json)", summaries.len(), user_id, start.elapsed().as_millis()),
        None,
    );

    Ok(summaries)
}

#[server(endpoint = "get_public_sheets")]
pub async fn get_public_sheets() -> Result<Vec<CharacterSummary>, ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in get_public_sheets", None);
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None);
    let start = std::time::Instant::now();

    let summaries = SheetRepository::list_public(&pool, auth_user_id.as_deref()).await.map_err(|e| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to fetch public sheets from DB", Some(&e.to_string()));
        ServerFnError::new("Falha ao consultar fichas públicas. Tente novamente mais tarde.")
    })?;

    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("SELECT public character_sheets: retornou {} fichas em {}ms (otimizado via summary_json)", summaries.len(), start.elapsed().as_millis()),
        None,
    );

    Ok(summaries)
}

#[server(endpoint = "get_sheet")]
pub async fn get_sheet(id: String, token: Option<String>) -> Result<CharacterData, ServerFnError> {
    if id.trim().is_empty() {
        return Err(ServerFnError::new("ID da ficha não fornecido"));
    }

    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in get_sheet", Some(&format!("id={}", id)));
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None);
    let start = std::time::Instant::now();

    let (sheet_user_id, room_id, data_json, sheet_type_db, is_public) = SheetRepository::find_raw_by_id(&pool, &id)
        .await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Error querying sheet {}", id), Some(&e.to_string()));
            ServerFnError::new("Erro ao buscar ficha no banco de dados.")
        })?
        .ok_or_else(|| {
            crate::logging::server::write_log(crate::logging::LogCategory::Requests, "WARN", &format!("Sheet with id {} not found", id), None);
            ServerFnError::new(format!("Ficha com ID '{}' não encontrada", id))
        })?;

    let is_owner = auth_user_id.is_some() && auth_user_id == sheet_user_id;
    let mut is_gm = false;
    if let (Some(u_id), Some(r_id)) = (&auth_user_id, &room_id) {
        is_gm = SheetRepository::check_room_gm(&pool, r_id, u_id).await.unwrap_or(false);
    }

    let mut has_acl_read = false;
    let mut has_acl_write = false;
    if let Some(ref u_id) = auth_user_id {
        has_acl_write = crate::repositories::SheetAclRepository::check_sheet_permission(&pool, &id, u_id, "write").await.unwrap_or(false);
        if has_acl_write {
            has_acl_read = true;
        } else {
            has_acl_read = crate::repositories::SheetAclRepository::check_sheet_permission(&pool, &id, u_id, "read").await.unwrap_or(false);
        }
    }

    let has_token_access = crate::repositories::SheetAclRepository::verify_token_or_public(&pool, &id, token.as_deref()).await.unwrap_or(false);

    // Validação de Permissão de Leitura (Proprietário, GM, Pública, ACL nominal ou Token de Compartilhamento válido)
    let can_read = is_owner || is_gm || is_public || has_acl_read || has_token_access || sheet_user_id.is_none();
    if !can_read {
        return Err(ServerFnError::new("Permissão negada: Esta ficha é privada. Verifique se possui permissão ou se o link de compartilhamento expirou."));
    }

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
    data.can_edit = is_owner || is_gm || has_acl_write || sheet_user_id.is_none();
    data.author_username = if let Some(ref uid) = sheet_user_id {
        SheetRepository::find_username_by_id(&pool, uid).await.ok().flatten()
    } else {
        None
    };
    data.sanitize();

    // Carrega respostas relacionais salvas na tabela character_quiz_answers
    if let Ok(quiz_rows) = SheetRepository::find_quiz_answers(&pool, &id).await {
        for (q_id, ans) in quiz_rows {
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

    // SVG / XML text (allow safe <svg or <?xml ... <svg)
    if bytes.len() >= 4 {
        let text = String::from_utf8_lossy(bytes).to_lowercase();
        if text.contains("<svg") {
            // Rejeitar scripts ou eventos embutidos que possibilitem Stored XSS
            if text.contains("<script")
                || text.contains("javascript:")
                || text.contains("onload")
                || text.contains("onerror")
                || text.contains("onclick")
                || text.contains("onmouseover")
                || text.contains("<foreignobject")
            {
                return Err(ServerFnError::new("SVG inseguro rejeitado: scripts e manipuladores de eventos embutidos não são permitidos."));
            }
            return Ok(("image/svg+xml", "svg"));
        }
    }

    Err(ServerFnError::new("Formato de arquivo não suportado. Apenas imagens autênticas PNG, JPEG, WebP, GIF e SVG são permitidas."))
}

#[cfg(feature = "ssr")]
async fn verify_sheet_write_permission(pool: &sqlx::SqlitePool, sheet_id: &str) -> Result<(), ServerFnError> {
    let auth_user_id = crate::auth::get_auth_user_id().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    match SheetRepository::verify_write_permission(pool, sheet_id, auth_user_id.as_deref()).await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(msg)) => Err(ServerFnError::new(msg)),
        Err(e) => Err(ServerFnError::new(e.to_string())),
    }
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
            let count = SheetRepository::count_by_user(&pool, uid).await.unwrap_or(0);
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

        SheetRepository::create(
            &pool,
            &new_id,
            auth_user_id.as_deref(),
            &final_name,
            &data_json,
            &s_type,
            None,
            &summary_json,
        )
        .await
        .map_err(|e| {
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
        let count = SheetRepository::count_by_user(&pool, uid).await.unwrap_or(0);
        if count >= 50 {
            return Err(ServerFnError::new("Limite de 50 fichas por conta atingido. Exclua fichas antigas para criar novas."));
        }
    }

    let valid_folder_id = if let (Some(uid), Some(fid)) = (&auth_user_id, &folder_id) {
        if SheetRepository::check_folder_exists_for_user(&pool, fid, uid).await.unwrap_or(false) {
            Some(fid.clone())
        } else {
            None
        }
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

    SheetRepository::create(
        &pool,
        &id,
        auth_user_id.as_deref(),
        &resolved_name,
        &data_json,
        &s_type,
        valid_folder_id.as_deref(),
        &summary_json,
    )
    .await
    .map_err(|e| {
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
    let summary = data.to_summary(String::new(), data.is_public, true);
    let summary_json = serde_json::to_string(&summary).unwrap_or_default();

    let rows_affected = SheetRepository::update(
        &pool,
        &id,
        &display_name,
        &data_json,
        &data.sheet_type,
        data.is_public,
        &summary_json,
    )
    .await
    .map_err(|e| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Failed to update sheet {}", id), Some(&e.to_string()));
        ServerFnError::new("Falha ao atualizar dados da ficha no banco de dados.")
    })?;

    if rows_affected == 0 {
        crate::logging::server::write_log(crate::logging::LogCategory::Requests, "WARN", &format!("Ficha com ID '{}' não encontrada para atualização", id), None);
        return Err(ServerFnError::new(format!("Ficha com ID '{}' não encontrada para atualização", id)));
    }

    // Sincroniza respostas relacionais na tabela character_quiz_answers
    let _ = SheetRepository::sync_quiz_answers(&pool, &id, &data.quiz_data.entries).await;

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

    let rows_affected = SheetRepository::set_visibility(&pool, &id, is_public)
        .await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Failed to update sheet visibility {}", id), Some(&e.to_string()));
            ServerFnError::new("Falha ao atualizar visibilidade no banco de dados.")
        })?;

    if rows_affected == 0 {
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
    let rows_affected = SheetRepository::delete(&pool, &id)
        .await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Failed to delete sheet {}", id), Some(&e.to_string()));
            ServerFnError::new("Falha ao excluir ficha do banco de dados.")
        })?;

    if rows_affected == 0 {
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

pub use super::server_fns_folders::*;
pub use super::server_fns_share::*;

#[server(endpoint = "clone_sheet")]
pub async fn clone_sheet(id: String, token: Option<String>) -> Result<String, ServerFnError> {
    if id.trim().is_empty() {
        return Err(ServerFnError::new("ID da ficha não fornecido"));
    }

    use sqlx::SqlitePool;
    use uuid::Uuid;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in clone_sheet", Some(&format!("id={}", id)));
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Você precisa estar logado para clonar uma ficha."))?;

    // Limite de cota de 50 fichas por conta
    let count = SheetRepository::count_by_user(&pool, &auth_user_id).await.unwrap_or(0);
    if count >= 50 {
        return Err(ServerFnError::new("Limite de 50 fichas por conta atingido. Exclua fichas antigas para clonar novas."));
    }

    let (sheet_user_id, room_id, data_json, sheet_type_db, is_public) = SheetRepository::find_raw_by_id(&pool, &id)
        .await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Error querying sheet for cloning {}", id), Some(&e.to_string()));
            ServerFnError::new("Erro ao buscar ficha no banco de dados.")
        })?
        .ok_or_else(|| {
            ServerFnError::new(format!("Ficha com ID '{}' não encontrada", id))
        })?;

    let is_owner = auth_user_id == sheet_user_id.clone().unwrap_or_default();
    let mut is_gm = false;
    if let Some(r_id) = room_id.as_deref() {
        is_gm = SheetRepository::check_room_gm(&pool, r_id, &auth_user_id).await.unwrap_or(false);
    }

    let has_acl_read = crate::repositories::SheetAclRepository::check_sheet_permission(&pool, &id, &auth_user_id, "read").await.unwrap_or(false);
    let has_token_access = crate::repositories::SheetAclRepository::verify_token_or_public(&pool, &id, token.as_deref()).await.unwrap_or(false);

    let can_clone = is_owner || is_gm || is_public || has_acl_read || has_token_access || sheet_user_id.is_none();
    if !can_clone {
        return Err(ServerFnError::new("Permissão negada: Esta ficha é privada e não pode ser clonada."));
    }

    let mut data: CharacterData = CharacterData::parse_from_db(&id, &data_json)
        .ok_or_else(|| ServerFnError::new("Dados da ficha corrompidos para clonagem."))?;

    let new_id = Uuid::new_v4().to_string();
    data.id = new_id.clone();
    let base_name = data.get_display_name();
    let cloned_name = format!("{} (Cópia)", if base_name.is_empty() { "Novo Mago" } else { &base_name });
    data.name = cloned_name.clone();
    data.labels.insert(crate::state::keys::HEADER_NOME.to_string(), cloned_name.clone());
    data.is_public = false;
    data.can_edit = true;
    data.author_username = None;
    data.sanitize();

    let new_data_json = serde_json::to_string(&data).map_err(|e| ServerFnError::new(e.to_string()))?;
    let summary = data.to_summary(String::new(), false, true);
    let summary_json = serde_json::to_string(&summary).unwrap_or_default();

    let final_sheet_type = if !data.sheet_type.is_empty() { data.sheet_type.clone() } else { sheet_type_db };

    SheetRepository::create(
        &pool,
        &new_id,
        Some(&auth_user_id),
        &cloned_name,
        &new_data_json,
        &final_sheet_type,
        None,
        &summary_json,
    )
    .await
    .map_err(|e| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Failed to insert cloned sheet {}", new_id), Some(&e.to_string()));
        ServerFnError::new("Falha ao salvar ficha clonada no banco de dados.")
    })?;

    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("Ficha clonada com sucesso: origem='{}', novo_id='{}', usuario='{}'", id, new_id, auth_user_id),
        None,
    );

    Ok(new_id)
}

#[server(endpoint = "get_user_profile")]
pub async fn get_user_profile(username: Option<String>) -> Result<crate::state::models::UserProfileData, ServerFnError> {
    use sqlx::SqlitePool;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in get_user_profile", None);
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None);

    let (target_user_id, target_username, target_created_at, is_self) = match username {
        Some(ref name) if !name.trim().is_empty() => {
            let user_row = SheetRepository::find_user_by_username(&pool, name.trim())
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?
                .ok_or_else(|| ServerFnError::new(format!("Usuário '{}' não encontrado", name)))?;

            let is_self = auth_user_id.as_deref() == Some(&user_row.0);
            (user_row.0, user_row.1, user_row.2, is_self)
        }
        _ => {
            let uid = auth_user_id.ok_or_else(|| ServerFnError::new("Você precisa estar logado para acessar seu perfil."))?;
            let user_row = SheetRepository::find_user_by_id(&pool, &uid)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?
                .ok_or_else(|| ServerFnError::new("Usuário não encontrado."))?;
            (user_row.0, user_row.1, user_row.2, true)
        }
    };

    let total_sheets = SheetRepository::count_by_user(&pool, &target_user_id).await.unwrap_or(0);
    let public_sheets_count = SheetRepository::count_public_by_user(&pool, &target_user_id).await.unwrap_or(0);
    let private_sheets_count = total_sheets.saturating_sub(public_sheets_count);
    let (mage_sheets_count, gods_monsters_sheets_count) = SheetRepository::count_sheets_by_splat(&pool, &target_user_id).await.unwrap_or((0, 0));
    let folders_count = SheetRepository::count_folders_by_user(&pool, &target_user_id).await.unwrap_or(0);

    let gm_rooms_count = SheetRepository::count_rooms_by_user(&pool, &target_user_id).await.unwrap_or(0);
    let player_rooms_count = SheetRepository::count_player_rooms_by_user(&pool, &target_user_id).await.unwrap_or(0);
    let rooms_count = gm_rooms_count + player_rooms_count;

    let active_sessions_count = if is_self {
        SheetRepository::count_active_sessions_by_user(&pool, &target_user_id).await.unwrap_or(1)
    } else {
        0
    };

    let is_admin_db: i64 = sqlx::query_scalar("SELECT is_admin FROM users WHERE id = ?")
        .bind(&target_user_id)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None)
        .unwrap_or(0);
    let is_admin = is_admin_db == 1 || crate::auth::is_username_in_admin_env(&target_username);

    let sheets = if is_self {
        SheetRepository::list_by_user(&pool, &target_user_id).await.unwrap_or_default()
    } else {
        SheetRepository::list_by_user_public_only(&pool, &target_user_id, Some(&target_username)).await.unwrap_or_default()
    };

    Ok(crate::state::models::UserProfileData {
        id: target_user_id,
        username: target_username,
        created_at: target_created_at,
        is_self,
        is_admin,
        total_sheets,
        public_sheets_count,
        private_sheets_count,
        mage_sheets_count,
        gods_monsters_sheets_count,
        folders_count,
        rooms_count,
        gm_rooms_count,
        player_rooms_count,
        active_sessions_count,
        sheets,
    })
}

#[server(endpoint = "get_system_stats")]
pub async fn get_system_stats() -> Result<crate::state::models::SystemStats, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::SqlitePool;
        let pool = use_context::<SqlitePool>().ok_or_else(|| {
            ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
        })?;

        SheetRepository::get_system_stats(&pool)
            .await
            .map_err(|e| {
                crate::logging::server::write_log(
                    crate::logging::LogCategory::Errors,
                    "ERROR",
                    "Falha ao consultar estatísticas do sistema",
                    Some(&e.to_string()),
                );
                ServerFnError::new("Falha ao consultar estatísticas do sistema.")
            })
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(endpoint = "toggle_sheet_like")]
pub async fn toggle_sheet_like(sheet_id: String) -> Result<crate::state::models::LikeToggleResult, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let clean_id = sheet_id.trim().to_string();
        if clean_id.is_empty() {
            return Err(ServerFnError::new("ID da ficha não fornecido"));
        }

        use sqlx::SqlitePool;
        let pool = use_context::<SqlitePool>().ok_or_else(|| {
            ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
        })?;

        let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
            .ok_or_else(|| ServerFnError::new("Você precisa estar logado para curtir fichas."))?;

        let (is_liked, likes_count) = SheetRepository::toggle_like(&pool, &clean_id, &auth_user_id)
            .await
            .map_err(|e| {
                crate::logging::server::write_log(
                    crate::logging::LogCategory::Errors,
                    "ERROR",
                    &format!("Erro ao alterar like da ficha {}", clean_id),
                    Some(&e.to_string()),
                );
                ServerFnError::new("Falha ao registrar curtida. Tente novamente.")
            })?;

        crate::logging::server::write_log(
            crate::logging::LogCategory::UserActions,
            "INFO",
            &format!("LIKE TOGGLE: sheet_id='{}', user_id='{}', is_liked={}, count={}", clean_id, auth_user_id, is_liked, likes_count),
            None,
        );

        Ok(crate::state::models::LikeToggleResult {
            sheet_id: clean_id,
            is_liked,
            likes_count,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = sheet_id;
        unreachable!()
    }
}




