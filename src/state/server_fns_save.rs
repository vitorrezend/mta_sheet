//! Gravação da ficha: completa (`update_sheet`) ou só do que mudou (`patch_sheet`).
//!
//! As duas passam por [`persist_sheet`], que sanitiza, valida o tamanho, grava com
//! controle de versão e avisa a sala. O patch é aplicado sobre o JSON do banco, então
//! campos que o cliente não tocou mantêm o que estiver salvo (inclusive mudanças do mestre).

use leptos::*;
use super::models::CharacterData;
#[cfg(feature = "ssr")]
use crate::errors::AppError;
#[cfg(feature = "ssr")]
use crate::repositories::SheetRepository;

/// Mensagem de erro usada quando a ficha foi alterada por outra pessoa desde que foi aberta.
/// O cliente reconhece este prefixo para oferecer "recarregar" ou "manter minhas alterações".
pub const SHEET_CONFLICT_ERROR: &str = crate::errors::VERSION_CONFLICT_PREFIX;

/// Tamanho máximo de uma ficha no banco.
#[cfg(feature = "ssr")]
const MAX_SHEET_BYTES: usize = 5 * 1024 * 1024;

#[cfg(feature = "ssr")]
fn log(category: crate::logging::LogCategory, level: &str, msg: &str, detail: Option<&str>) {
    crate::logging::server::write_log(category, level, msg, detail);
}

#[cfg(feature = "ssr")]
fn db_pool(context: &str, id: &str) -> Result<sqlx::SqlitePool, ServerFnError> {
    use_context::<sqlx::SqlitePool>().ok_or_else(|| {
        log(crate::logging::LogCategory::Errors, "ERROR", &format!("Database pool not found in {}", context), Some(&format!("id={}", id)));
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })
}

/// Sanitiza e grava a ficha se ela ainda estiver em `expected_version`. Retorna a nova versão.
#[cfg(feature = "ssr")]
async fn persist_sheet(pool: &sqlx::SqlitePool, id: &str, mut data: CharacterData, expected_version: i64, payload_bytes: usize) -> Result<i64, ServerFnError> {
    use crate::logging::LogCategory;

    data.sanitize();
    let display_name = data.get_display_name();
    data.name = display_name.clone();
    data.labels.insert(crate::state::keys::HEADER_NOME.to_string(), display_name.clone());

    let start = std::time::Instant::now();
    let data_json = serde_json::to_string(&data).map_err(|e| {
        log(LogCategory::Errors, "ERROR", &format!("Serialization error updating sheet {}", id), Some(&e.to_string()));
        ServerFnError::new("Falha ao serializar dados da ficha.")
    })?;
    if data_json.len() > MAX_SHEET_BYTES {
        return Err(ServerFnError::new("O tamanho dos dados da ficha excede o limite máximo permitido de 5 MB"));
    }

    let summary = data.to_summary(String::new(), data.is_public, true);
    let summary_json = serde_json::to_string(&summary).unwrap_or_default();

    let new_version = SheetRepository::update(pool, id, expected_version, &display_name, &data_json, &data.sheet_type, data.is_public, &summary_json)
        .await
        .map_err(|e| {
            log(LogCategory::Errors, "ERROR", &format!("Failed to update sheet {}", id), Some(&e.to_string()));
            AppError::internal("Falha ao atualizar a ficha no banco", e)
        })?;

    let Some(new_version) = new_version else {
        let exists = SheetRepository::find_version(pool, id).await.ok().flatten().is_some();
        if exists {
            log(LogCategory::Requests, "WARN", &format!("Conflito de versão ao salvar ficha '{}' (cliente na versão {})", id, expected_version), None);
            return Err(AppError::VersionConflict("esta ficha foi alterada em outro lugar desde que você a abriu".into()).into());
        }
        log(LogCategory::Requests, "WARN", &format!("Ficha com ID '{}' não encontrada para atualização", id), None);
        return Err(AppError::not_found("Ficha não encontrada").into());
    };

    // Sincroniza respostas relacionais na tabela character_quiz_answers
    let _ = SheetRepository::sync_quiz_answers(pool, id, &data.quiz_data.entries).await;

    log(
        LogCategory::Database,
        "INFO",
        &format!(
            "UPDATE character_sheets id='{}' (nome='{}', public={}) salva em {}ms (recebido {:.1} KB, ficha {:.1} KB)",
            id, data.name, data.is_public, start.elapsed().as_millis(), payload_bytes as f64 / 1024.0, data_json.len() as f64 / 1024.0
        ),
        None,
    );

    crate::rooms::events::notify_sheet_changed(pool, id).await;
    Ok(new_version)
}

/// Salva a ficha inteira e retorna a nova versão. Falha com [`SHEET_CONFLICT_ERROR`] se a
/// ficha mudou desde a versão que o cliente carregou (`data.version`).
#[server(endpoint = "update_sheet")]
pub async fn update_sheet(id: String, data: CharacterData) -> Result<i64, ServerFnError> {
    if id.trim().is_empty() {
        return Err(ServerFnError::new("ID da ficha não pode ser vazio"));
    }
    let pool = db_pool("update_sheet", &id)?;
    super::server_fns::verify_sheet_write_permission(&pool, &id).await?;
    let expected = data.version;
    let payload = serde_json::to_string(&data).map(|j| j.len()).unwrap_or(0);
    persist_sheet(&pool, &id, data, expected, payload).await
}

/// Salva só o que mudou: `patch` é um JSON Merge Patch (RFC 7386) em relação à versão
/// `base_version` da ficha. Retorna a nova versão.
#[server(endpoint = "patch_sheet")]
pub async fn patch_sheet(id: String, base_version: i64, patch: String) -> Result<i64, ServerFnError> {
    if id.trim().is_empty() {
        return Err(ServerFnError::new("ID da ficha não pode ser vazio"));
    }
    if patch.len() > MAX_SHEET_BYTES {
        return Err(ServerFnError::new("Alteração grande demais"));
    }
    let pool = db_pool("patch_sheet", &id)?;
    super::server_fns::verify_sheet_write_permission(&pool, &id).await?;

    let patch_value: serde_json::Value = serde_json::from_str(&patch).map_err(|_| ServerFnError::new("Alteração inválida"))?;
    let (_, _, data_json, _, _) = SheetRepository::find_raw_by_id(&pool, &id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Ficha não encontrada"))?;

    // Mesma checagem que a gravação faria, mas antes de aplicar: o patch só faz sentido
    // sobre a versão em que o cliente se baseou
    let current_version = SheetRepository::find_version(&pool, &id).await.ok().flatten().unwrap_or(0);
    if current_version != base_version {
        return Err(AppError::VersionConflict("esta ficha foi alterada em outro lugar desde que você a abriu".into()).into());
    }

    let mut value: serde_json::Value = serde_json::from_str(&data_json).unwrap_or_else(|_| serde_json::json!({}));
    super::json_patch::merge_apply(&mut value, &patch_value);
    let data = CharacterData::parse_from_db(&id, &value.to_string())
        .ok_or_else(|| ServerFnError::new("A alteração deixaria a ficha inválida"))?;
    persist_sheet(&pool, &id, data, base_version, patch.len()).await
}
