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

    let start = std::time::Instant::now();
    let rows = sqlx::query("SELECT id, name, updated_at FROM character_sheets ORDER BY updated_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to fetch sheets from DB", Some(&e.to_string()));
            ServerFnError::new(format!("Falha ao consultar fichas: {}", e))
        })?;

    let count = rows.len();
    let summaries = rows.into_iter().map(|row| CharacterSummary {
        id: row.get("id"),
        name: row.get("name"),
        updated_at: row.get("updated_at"),
    }).collect();

    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("SELECT character_sheets: retornou {} fichas em {}ms", count, start.elapsed().as_millis()),
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

    let start = std::time::Instant::now();
    let row = sqlx::query("SELECT data FROM character_sheets WHERE id = ?")
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

    data.sanitize();
    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("SELECT character_sheets id='{}' (nome='{}') carregada com sucesso em {}ms", id, data.name, start.elapsed().as_millis()),
        None,
    );

    Ok(data)
}

#[server(endpoint = "create_sheet")]
pub async fn create_sheet(name: String) -> Result<String, ServerFnError> {
    let clean_name = name.trim().to_string();
    let final_name = if clean_name.is_empty() { "Novo Mago".to_string() } else { clean_name };

    use sqlx::SqlitePool;
    use uuid::Uuid;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in create_sheet", None);
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let start = std::time::Instant::now();
    let id = Uuid::new_v4().to_string();
    let initial_data = CharacterData::new(id.clone(), final_name.clone());

    let data_json = serde_json::to_string(&initial_data).map_err(|e: serde_json::Error| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Serialization error creating sheet", Some(&e.to_string()));
        ServerFnError::new(format!("Falha ao serializar dados iniciais: {}", e))
    })?;

    sqlx::query("INSERT INTO character_sheets (id, name, data) VALUES (?, ?, ?)")
        .bind(&id)
        .bind(&final_name)
        .bind(data_json)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Failed to insert new sheet {}", id), Some(&e.to_string()));
            ServerFnError::new(format!("Falha ao salvar nova ficha no banco: {}", e))
        })?;

    crate::logging::server::write_log(
        crate::logging::LogCategory::UserActions,
        "INFO",
        &format!("CREATE SHEET: Nova ficha criada id='{}', nome='{}' em {}ms", id, final_name, start.elapsed().as_millis()),
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

    let start = std::time::Instant::now();
    let data_json = serde_json::to_string(&data).map_err(|e: serde_json::Error| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", &format!("Serialization error updating sheet {}", id), Some(&e.to_string()));
        ServerFnError::new(format!("Falha ao serializar dados da ficha: {}", e))
    })?;

    let payload_kb = (data_json.len() as f64) / 1024.0;
    let result = sqlx::query("UPDATE character_sheets SET name = ?, data = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(&data.name)
        .bind(data_json)
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
        &format!("UPDATE character_sheets id='{}' (nome='{}') salva com sucesso em {}ms ({:.1} KB)", id, data.name, start.elapsed().as_millis(), payload_kb),
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

    let (mime_type, base64_payload) = if let Some(idx) = data_base64.find(";base64,") {
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

    let ext = match mime_type.as_str() {
        "image/png" => "png",
        "image/jpeg" | "image/jpg" => "jpg",
        "image/gif" => "gif",
        "image/svg+xml" => "svg",
        _ => "webp",
    };

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
    .bind(&mime_type)
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
