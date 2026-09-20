use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeatureFlagStatus {
    pub key: String,
    pub value: String, // "disabled" | "admin_only" | "enabled"
    pub is_enabled_for_user: bool,
    pub description: String,
}

/// Retorna o status de todas as feature flags do sistema, indicando se o usuário atual tem acesso.
#[server(endpoint = "get_system_feature_flags")]
pub async fn get_system_feature_flags() -> Result<Vec<FeatureFlagStatus>, ServerFnError> {
    use sqlx::{SqlitePool, Row};

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let is_admin = crate::auth::is_current_user_admin().await.unwrap_or(false);

    let rows = sqlx::query("SELECT key, value FROM system_settings")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao carregar configurações: {}", e)))?;

    let mut flags = Vec::new();
    for row in rows {
        let key: String = row.try_get("key").unwrap_or_default();
        let value: String = row.try_get("value").unwrap_or_else(|_| "disabled".to_string());

        let is_enabled_for_user = match value.as_str() {
            "enabled" => true,
            "admin_only" => is_admin,
            _ => false,
        };

        let description = match key.as_str() {
            "feature_tactical_grid" => "🗺️ Mapa & Grid Tático nas Salas de Jogo".to_string(),
            _ => key.clone(),
        };

        flags.push(FeatureFlagStatus {
            key,
            value,
            is_enabled_for_user,
            description,
        });
    }

    Ok(flags)
}

/// Atualiza uma feature flag no sistema. Requer autenticação de Administrador.
#[server(endpoint = "update_system_feature_flag")]
pub async fn update_system_feature_flag(key: String, value: String) -> Result<(), ServerFnError> {
    let is_admin = crate::auth::is_current_user_admin().await?;
    if !is_admin {
        return Err(ServerFnError::new(
            "Acesso negado. Apenas administradores do sistema podem alterar configurações de recursos."
        ));
    }

    let clean_key = key.trim();
    let clean_val = value.trim();

    if clean_val != "disabled" && clean_val != "admin_only" && clean_val != "enabled" {
        return Err(ServerFnError::new(
            "Valor de configuração inválido. Use 'disabled', 'admin_only' ou 'enabled'."
        ));
    }

    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    sqlx::query(
        "INSERT INTO system_settings (key, value, updated_at) VALUES (?, ?, CURRENT_TIMESTAMP)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = CURRENT_TIMESTAMP"
    )
    .bind(clean_key)
    .bind(clean_val)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Erro ao atualizar configuração: {}", e)))?;

    log::info!("Admin alterou feature flag '{}' para '{}'", clean_key, clean_val);

    Ok(())
}

/// Helper server-side para verificar se uma feature está habilitada para o chamador da requisição atual.
#[cfg(feature = "ssr")]
pub async fn is_feature_enabled_for_caller(pool: &sqlx::SqlitePool, key: &str) -> Result<bool, ServerFnError> {
    let is_admin = crate::auth::is_current_user_admin().await.unwrap_or(false);

    let val: Option<String> = sqlx::query_scalar("SELECT value FROM system_settings WHERE key = ?")
        .bind(key)
        .fetch_optional(pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let flag_val = val.unwrap_or_else(|| "admin_only".to_string());
    let allowed = match flag_val.as_str() {
        "enabled" => true,
        "admin_only" => is_admin,
        _ => false,
    };

    Ok(allowed)
}
