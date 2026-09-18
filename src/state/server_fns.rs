use leptos::*;
use super::models::{CharacterData, CharacterSummary, QuizQuestionEntry, SheetFolder, FolderAclEntry};

// ==========================================
// Server Functions with Robust Error Handling
// ==========================================

#[cfg(feature = "ssr")]
use crate::repositories::{SheetRepository, FolderRepository, AclRepository};

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
pub async fn get_sheet(id: String) -> Result<CharacterData, ServerFnError> {
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

    // Validação de Permissão de Leitura
    if !is_owner && !is_gm && !is_public && sheet_user_id.is_some() {
        return Err(ServerFnError::new("Permissão negada: Esta ficha é privada e pertence a outro usuário."));
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

// ==========================================
// Folder Management Server Functions
// ==========================================

#[server(endpoint = "get_folders")]
pub async fn get_folders() -> Result<Vec<SheetFolder>, ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in get_folders", None);
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None);
    if auth_user_id.is_none() {
        return Ok(Vec::new());
    }
    let user_id = auth_user_id.unwrap_or_default();

    let folders = FolderRepository::list_by_user(&pool, &user_id).await.map_err(|e| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to fetch folders", Some(&e.to_string()));
        ServerFnError::new("Falha ao consultar pastas.")
    })?;

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
        let parent_owner = FolderRepository::find_owner_id(&pool, pid).await
            .map_err(|_| ServerFnError::new("Erro ao validar pasta pai."))?;
        match parent_owner {
            Some(owner_id) if owner_id == auth_user_id => {},
            _ => return Err(ServerFnError::new("Pasta pai não encontrada ou acesso negado.")),
        }
    }

    let count = FolderRepository::count_by_user(&pool, &auth_user_id).await.unwrap_or(0);
    if count >= 60 {
        return Err(ServerFnError::new("Limite de 60 pastas por conta atingido."));
    }

    let id = Uuid::new_v4().to_string();
    let folder_icon = icon.unwrap_or_else(|| "📁".to_string());
    let folder_color = color.unwrap_or_else(|| "#b89347".to_string());

    FolderRepository::create(
        &pool,
        &id,
        &auth_user_id,
        parent_id.as_deref(),
        &clean_name,
        &folder_icon,
        &folder_color,
        count as i32,
    )
    .await
    .map_err(|e| {
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

    let folder_owner = FolderRepository::find_owner_id(&pool, &folder_id).await
        .map_err(|_| ServerFnError::new("Erro ao buscar pasta."))?;

    match folder_owner {
        Some(owner_id) if owner_id == auth_user_id => {},
        _ => return Err(ServerFnError::new("Pasta não encontrada ou acesso não autorizado.")),
    }

    if let Some(ref target_id) = target_parent_id {
        if target_id == &folder_id {
            return Err(ServerFnError::new("Não é possível mover uma pasta para dentro de si mesma."));
        }

        let target_owner = FolderRepository::find_owner_id(&pool, target_id).await
            .map_err(|_| ServerFnError::new("Erro ao buscar pasta destino."))?;

        match target_owner {
            Some(owner_id) if owner_id == auth_user_id => {},
            _ => return Err(ServerFnError::new("Pasta destino não encontrada ou acesso negado.")),
        }

        let is_descendant = FolderRepository::is_descendant(&pool, &folder_id, target_id).await
            .map_err(|_| ServerFnError::new("Erro ao validar hierarquia de pastas."))?;

        if is_descendant {
            return Err(ServerFnError::new("Operação inválida: não é possível mover uma pasta para dentro de uma de suas subpastas."));
        }
    }

    FolderRepository::move_folder(&pool, &folder_id, target_parent_id.as_deref(), &auth_user_id).await
        .map_err(|e| {
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

    let rows_affected = FolderRepository::update(
        &pool,
        &folder_id,
        &auth_user_id,
        &clean_name,
        &folder_icon,
        &folder_color,
    )
    .await
    .map_err(|e| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to update folder", Some(&e.to_string()));
        ServerFnError::new("Falha ao atualizar pasta.")
    })?;

    if rows_affected == 0 {
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

    let rows_affected = FolderRepository::delete(&pool, &folder_id, &auth_user_id).await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to delete folder", Some(&e.to_string()));
            ServerFnError::new("Falha ao excluir pasta.")
        })?;

    if rows_affected == 0 {
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
    AclRepository::check_folder_permission(pool, folder_id, user_id, required_permission).await
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

    let rows_affected = SheetRepository::move_to_folder(&pool, &sheet_id, folder_id.as_deref(), &auth_user_id)
        .await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to move sheet to folder", Some(&e.to_string()));
            ServerFnError::new("Falha ao mover ficha para a pasta.")
        })?;

    if rows_affected == 0 {
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

    let entries = AclRepository::list_for_folder(&pool, &folder_id).await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to fetch folder ACLs", Some(&e.to_string()));
            ServerFnError::new("Falha ao consultar permissões da pasta.")
        })?;

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
            let u_opt = AclRepository::find_user_by_name(&pool, clean_user).await
                .map_err(|_| ServerFnError::new("Erro ao buscar usuário."))?;

            match u_opt {
                Some((uid, uname)) => {
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
            let r_opt = AclRepository::find_room_by_id_or_code(&pool, clean_room).await
                .map_err(|_| ServerFnError::new("Erro ao buscar sala."))?;

            match r_opt {
                Some((rid, rname)) => (Some(rid), rname),
                None => return Err(ServerFnError::new("Sala de crônica não encontrada.")),
            }
        }
        "public" => (None, "Público".to_string()),
        _ => return Err(ServerFnError::new("Tipo de destinatário inválido.")),
    };

    let acl_id = Uuid::new_v4().to_string();
    AclRepository::grant(&pool, &acl_id, &folder_id, &grantee_type, grantee_id.as_deref(), &clean_perm)
        .await
        .map_err(|e| {
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

    let folder_id = AclRepository::find_folder_id_by_acl(&pool, &acl_id).await
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

    AclRepository::revoke(&pool, &acl_id).await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to revoke folder ACL", Some(&e.to_string()));
            ServerFnError::new("Falha ao revogar permissão.")
        })?;

    Ok(())
}

