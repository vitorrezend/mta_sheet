use serde::{Deserialize, Serialize};
use leptos::*;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub is_admin: bool,
}

/// Valida a complexidade e tamanho de uma senha de acordo com as diretrizes de segurança canônicas:
/// - Mínimo de 8 caracteres
/// - Máximo de 128 caracteres
/// - Pelo menos uma letra e pelo menos um número
pub fn validate_password_strength(pwd: &str) -> Result<(), &'static str> {
    if pwd.len() < 8 {
        return Err("A senha deve ter no mínimo 8 caracteres.");
    }
    if pwd.len() > 128 {
        return Err("A senha não pode exceder 128 caracteres.");
    }
    let has_letter = pwd.chars().any(|c| c.is_alphabetic());
    let has_number = pwd.chars().any(|c| c.is_numeric());
    if !has_letter || !has_number {
        return Err("A senha deve conter pelo menos uma letra e um número.");
    }
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn extract_client_ip_from_context() -> String {
    let headers: Option<http::HeaderMap> = if let Ok(h) = leptos_axum::extract::<http::HeaderMap>().await {
        Some(h)
    } else {
        use_context::<http::HeaderMap>()
    };

    match headers {
        Some(h) => crate::server::extract_client_ip(&h),
        None => "127.0.0.1".to_string(),
    }
}

#[cfg(feature = "ssr")]
fn mask_token(token: &str) -> String {
    if token.len() > 8 {
        format!("{}...{}", &token[..4], &token[token.len() - 4..])
    } else {
        "***".to_string()
    }
}

#[cfg(feature = "ssr")]
pub async fn extract_session_token() -> Option<String> {
    use http::HeaderMap;
    let headers: HeaderMap = if let Ok(h) = leptos_axum::extract().await {
        h
    } else if let Some(h) = use_context::<HeaderMap>() {
        h
    } else {
        log::debug!("Could not extract HeaderMap from request");
        return None;
    };

    let cookie_header = headers.get(http::header::COOKIE)?.to_str().ok()?;
    log::trace!("Cookie header present in request");
    for pair in cookie_header.split(';') {
        let mut parts = pair.trim().splitn(2, '=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            if key.trim() == "session_token" {
                let token = value.trim().to_string();
                log::debug!("Extracted session_token: {}", mask_token(&token));
                return Some(token);
            }
        }
    }
    None
}

#[cfg(feature = "ssr")]
pub fn set_session_cookie(token: &str, max_age_secs: i64) {
    if let Some(res_options) = use_context::<leptos_axum::ResponseOptions>() {
        let is_prod = std::env::var("LEPTOS_ENV").map(|e| e.to_lowercase() == "production" || e.to_lowercase() == "prod").unwrap_or(false)
            || std::env::var("ENABLE_SECURE_COOKIE").map(|e| e == "1" || e.to_lowercase() == "true").unwrap_or(false);

        let secure_flag = if is_prod { "; Secure" } else { "" };
        let cookie_str = format!(
            "session_token={}; Path=/; SameSite=Lax; HttpOnly{}; Max-Age={}",
            token, secure_flag, max_age_secs
        );
        if let Ok(header_val) = http::HeaderValue::from_str(&cookie_str) {
            res_options.insert_header(http::header::SET_COOKIE, header_val);
            log::debug!("Set-Cookie registered: session_token={}; Max-Age={}", mask_token(token), max_age_secs);
        }
    } else {
        log::warn!("ResponseOptions not found in context when setting cookie");
    }
}

#[cfg(feature = "ssr")]
pub async fn get_auth_user_id() -> Result<Option<String>, ServerFnError> {
    use sqlx::{SqlitePool, Row};
    let token = match extract_session_token().await {
        Some(t) if !t.is_empty() => t,
        _ => return Ok(None),
    };

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let row = sqlx::query(
        "SELECT user_id FROM sessions WHERE id = ? AND expires_at > CURRENT_TIMESTAMP"
    )
    .bind(token)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(row.map(|r| r.get("user_id")))
}

#[cfg(feature = "ssr")]
pub fn is_username_in_admin_env(username: &str) -> bool {
    let env_val = std::env::var("ADMIN_USERNAMES")
        .or_else(|_| std::env::var("ADMIN_USERNAME"))
        .unwrap_or_default();

    if env_val.trim().is_empty() {
        return false;
    }

    env_val
        .split(',')
        .map(|s| s.trim())
        .any(|admin| !admin.is_empty() && admin.eq_ignore_ascii_case(username))
}

pub const MAX_SESSIONS_PER_USER: usize = 5;

#[cfg(feature = "ssr")]
pub async fn create_session_with_fifo(pool: &sqlx::SqlitePool, user_id: &str) -> Result<String, sqlx::Error> {
    use uuid::Uuid;

    // 1. Limpa quaisquer sessões expiradas deste usuário
    let _ = sqlx::query("DELETE FROM sessions WHERE user_id = ? AND expires_at < CURRENT_TIMESTAMP")
        .bind(user_id)
        .execute(pool)
        .await;

    // 2. Política FIFO: Mantém no máximo (MAX_SESSIONS_PER_USER - 1) sessões antes de inserir a nova
    let keep_count = (MAX_SESSIONS_PER_USER.saturating_sub(1)) as i64;
    let _ = sqlx::query(
        "DELETE FROM sessions WHERE user_id = ? AND id NOT IN (
            SELECT id FROM sessions 
            WHERE user_id = ? 
            ORDER BY created_at DESC, rowid DESC 
            LIMIT ?
        )"
    )
    .bind(user_id)
    .bind(user_id)
    .bind(keep_count)
    .execute(pool)
    .await;

    // 3. Cria a nova sessão válida por 30 dias
    let session_token = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO sessions (id, user_id, expires_at) 
         VALUES (?, ?, datetime('now', '+30 days'))"
    )
    .bind(&session_token)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(session_token)
}

#[cfg(feature = "ssr")]
pub async fn is_current_user_admin() -> Result<bool, ServerFnError> {
    use sqlx::{SqlitePool, Row};
    let token = match extract_session_token().await {
        Some(t) if !t.is_empty() => t,
        _ => return Ok(false),
    };

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let row = sqlx::query(
        "SELECT u.username, u.is_admin FROM sessions s 
         JOIN users u ON s.user_id = u.id 
         WHERE s.id = ? AND s.expires_at > CURRENT_TIMESTAMP"
    )
    .bind(token)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    if let Some(r) = row {
        let is_admin_col: i64 = r.try_get("is_admin").unwrap_or(0);
        let username: String = r.get("username");
        let is_admin = is_admin_col == 1 || is_username_in_admin_env(&username);
        Ok(is_admin)
    } else {
        Ok(false)
    }
}

#[server(endpoint = "get_current_user")]
pub async fn get_current_user() -> Result<Option<UserInfo>, ServerFnError> {
    use sqlx::{SqlitePool, Row};
    let token = match extract_session_token().await {
        Some(t) if !t.is_empty() => t,
        _ => return Ok(None),
    };

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let row = sqlx::query(
        "SELECT u.id, u.username, u.is_admin FROM sessions s 
         JOIN users u ON s.user_id = u.id 
         WHERE s.id = ? AND s.expires_at > CURRENT_TIMESTAMP"
    )
    .bind(token)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    if let Some(r) = row {
        let id: String = r.get("id");
        let username: String = r.get("username");
        let is_admin_col: i64 = r.try_get("is_admin").unwrap_or(0);
        let is_admin = is_admin_col == 1 || is_username_in_admin_env(&username);

        Ok(Some(UserInfo {
            id,
            username,
            is_admin,
        }))
    } else {
        Ok(None)
    }
}

#[server(endpoint = "register")]
pub async fn register(username: String, password: String) -> Result<UserInfo, ServerFnError> {
    let clean_user = username.trim().to_string();
    if clean_user.len() < 3 {
        return Err(ServerFnError::new("Nome de usuário deve ter no mínimo 3 caracteres"));
    }
    if let Err(err_msg) = validate_password_strength(&password) {
        return Err(ServerFnError::new(err_msg));
    }

    #[cfg(feature = "ssr")]
    {
        use std::time::Duration;
        let client_ip = extract_client_ip_from_context().await;
        if !crate::server::handlers::auth::check_auth_rate_limit(&client_ip, 5, Duration::from_secs(60)) {
            return Err(ServerFnError::new("Muitas tentativas de cadastro. Aguarde 1 minuto antes de tentar novamente."));
        }
    }

    use sqlx::{SqlitePool, Row};
    use uuid::Uuid;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    // Verifica se é o primeiro usuário do sistema ou se está na lista ADMIN_USERNAMES
    let user_count_row = sqlx::query("SELECT COUNT(*) as count FROM users")
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let user_count: i64 = user_count_row.get("count");
    let is_first_user = user_count == 0;
    let is_admin = is_first_user || is_username_in_admin_env(&clean_user);

    let password_hash = bcrypt::hash(&password, bcrypt::DEFAULT_COST)
        .map_err(|e| ServerFnError::new(format!("Erro ao processar senha: {}", e)))?;

    let user_id = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO users (id, username, password_hash, is_admin) VALUES (?, ?, ?, ?)")
        .bind(&user_id)
        .bind(&clean_user)
        .bind(&password_hash)
        .bind(if is_admin { 1i64 } else { 0i64 })
        .execute(&pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE") || e.to_string().contains("users.username") {
                ServerFnError::new("Este nome de usuário já está em uso")
            } else {
                ServerFnError::new(format!("Erro ao criar usuário: {}", e))
            }
        })?;

    // Create session for 30 days (FIFO policy: max 5 active sessions)
    let session_token = create_session_with_fifo(&pool, &user_id)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao criar sessão: {}", e)))?;

    set_session_cookie(&session_token, 30 * 24 * 60 * 60);

    log::info!("Registered new user: {} (admin: {})", clean_user, is_admin);
    Ok(UserInfo {
        id: user_id,
        username: clean_user,
        is_admin,
    })
}

#[server(endpoint = "login")]
pub async fn login(username: String, password: String) -> Result<UserInfo, ServerFnError> {
    let clean_user = username.trim().to_string();
    if clean_user.is_empty() || password.is_empty() {
        return Err(ServerFnError::new("Usuário e senha são obrigatórios"));
    }

    #[cfg(feature = "ssr")]
    {
        use std::time::Duration;
        let client_ip = extract_client_ip_from_context().await;
        if !crate::server::handlers::auth::check_auth_rate_limit(&client_ip, 10, Duration::from_secs(60)) {
            return Err(ServerFnError::new("Muitas tentativas de login. Aguarde 1 minuto antes de tentar novamente."));
        }
    }

    use sqlx::{SqlitePool, Row};
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let row = sqlx::query("SELECT id, username, password_hash, is_admin FROM users WHERE username = ? COLLATE NOCASE")
        .bind(&clean_user)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Usuário ou senha incorretos"))?;

    let user_id: String = row.get("id");
    let actual_username: String = row.get("username");
    let password_hash: String = row.get("password_hash");
    let is_admin_col: i64 = row.try_get("is_admin").unwrap_or(0);
    let is_admin = is_admin_col == 1 || is_username_in_admin_env(&actual_username);

    let is_valid = bcrypt::verify(&password, &password_hash)
        .map_err(|_| ServerFnError::new("Erro ao verificar credenciais"))?;

    if !is_valid {
        return Err(ServerFnError::new("Usuário ou senha incorretos"));
    }

    // Create session for 30 days (FIFO policy: max 5 active sessions)
    let session_token = create_session_with_fifo(&pool, &user_id)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao criar sessão: {}", e)))?;

    set_session_cookie(&session_token, 30 * 24 * 60 * 60);

    log::info!("Logged in user: {} (admin: {})", actual_username, is_admin);
    Ok(UserInfo {
        id: user_id,
        username: actual_username,
        is_admin,
    })
}

#[server(endpoint = "logout")]
pub async fn logout() -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    if let Some(token) = extract_session_token().await {
        if let Some(pool) = use_context::<SqlitePool>() {
            let _ = sqlx::query("DELETE FROM sessions WHERE id = ?")
                .bind(token)
                .execute(&pool)
                .await;
        }
    }

    set_session_cookie("", 0);
    log::info!("User logged out");
    Ok(())
}

#[cfg(feature = "ssr")]
static PASSWORD_RATE_LIMITER: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, Vec<std::time::Instant>>>>> =
    std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())));

#[cfg(feature = "ssr")]
fn check_password_rate_limit(key: &str) -> bool {
    let mut limiter = match PASSWORD_RATE_LIMITER.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    };
    let now = std::time::Instant::now();
    let entry = limiter.entry(key.to_string()).or_default();
    entry.retain(|&t| now.duration_since(t) < std::time::Duration::from_secs(15 * 60));
    if entry.len() >= 5 {
        false
    } else {
        entry.push(now);
        true
    }
}

#[server(endpoint = "change_password")]
pub async fn change_password(
    current_password: String,
    new_password: String,
    confirm_password: String,
) -> Result<(), ServerFnError> {
    use sqlx::{SqlitePool, Row};

    let token = match extract_session_token().await {
        Some(t) if !t.is_empty() => t,
        _ => return Err(ServerFnError::new("Sessão não autenticada. Faça login para continuar.")),
    };

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível.")
    })?;

    // 1. Identificar usuário pela sessão ativa
    let session_row = sqlx::query(
        "SELECT u.id, u.username, u.password_hash FROM sessions s 
         JOIN users u ON s.user_id = u.id 
         WHERE s.id = ? AND s.expires_at > CURRENT_TIMESTAMP"
    )
    .bind(&token)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?
    .ok_or_else(|| ServerFnError::new("Sessão expirada ou inválida. Faça login novamente."))?;

    let user_id: String = session_row.get("id");
    let username: String = session_row.get("username");
    let current_password_hash: String = session_row.get("password_hash");

    // 2. Proteção contra força bruta (Rate Limiting por User ID)
    if !check_password_rate_limit(&user_id) {
        crate::logging::server::write_log(
            crate::logging::LogCategory::Access,
            "WARN",
            &format!("Bloqueio de força bruta: excesso de tentativas de troca de senha para @{}", username),
            None,
        );
        return Err(ServerFnError::new("Muitas tentativas de alteração de senha. Aguarde 15 minutos antes de tentar novamente."));
    }

    // 3. Validações de entrada
    if current_password.is_empty() || new_password.is_empty() || confirm_password.is_empty() {
        return Err(ServerFnError::new("Todos os campos de senha são obrigatórios."));
    }

    if new_password != confirm_password {
        return Err(ServerFnError::new("A nova senha e a confirmação de senha não coincidem."));
    }

    if let Err(err_msg) = validate_password_strength(&new_password) {
        return Err(ServerFnError::new(err_msg));
    }

    if current_password == new_password {
        return Err(ServerFnError::new("A nova senha deve ser diferente da senha atual."));
    }

    // 4. Verificação criptográfica da senha atual
    let is_valid = bcrypt::verify(&current_password, &current_password_hash)
        .map_err(|_| ServerFnError::new("Erro ao verificar senha atual."))?;

    if !is_valid {
        crate::logging::server::write_log(
            crate::logging::LogCategory::Access,
            "WARN",
            &format!("Tentativa de alteração de senha falhou: senha atual incorreta para @{}", username),
            None,
        );
        return Err(ServerFnError::new("Senha atual incorreta."));
    }

    // 5. Criptografar nova senha com Bcrypt
    let new_password_hash = bcrypt::hash(&new_password, bcrypt::DEFAULT_COST)
        .map_err(|e| ServerFnError::new(format!("Erro ao processar nova senha: {}", e)))?;

    // 6. Atualizar hash no banco de dados
    sqlx::query("UPDATE users SET password_hash = ? WHERE id = ?")
        .bind(&new_password_hash)
        .bind(&user_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao salvar nova senha: {}", e)))?;

    // 7. Revogar todas as outras sessões ativas do usuário por segurança
    let revoked_sessions = sqlx::query("DELETE FROM sessions WHERE user_id = ? AND id != ?")
        .bind(&user_id)
        .bind(&token)
        .execute(&pool)
        .await
        .map(|res| res.rows_affected())
        .unwrap_or(0);

    crate::logging::server::write_log(
        crate::logging::LogCategory::UserActions,
        "INFO",
        &format!(
            "Senha alterada com sucesso para @{} (id={}). {} outras sessões ativas foram desconectadas por segurança.",
            username, user_id, revoked_sessions
        ),
        None,
    );

    Ok(())
}

#[server(endpoint = "revoke_other_sessions")]
pub async fn revoke_other_sessions() -> Result<u64, ServerFnError> {
    use sqlx::SqlitePool;

    let token = match extract_session_token().await {
        Some(t) if !t.is_empty() => t,
        _ => return Err(ServerFnError::new("Sessão não autenticada.")),
    };

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível.")
    })?;

    let auth_user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Sessão inválida.")
    })?;

    let res = sqlx::query("DELETE FROM sessions WHERE user_id = ? AND id != ?")
        .bind(&auth_user_id)
        .bind(&token)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao revogar sessões: {}", e)))?;

    let count = res.rows_affected();
    crate::logging::server::write_log(
        crate::logging::LogCategory::UserActions,
        "INFO",
        &format!("Usuário id={} revogou {} outras sessões ativas manualmente.", auth_user_id, count),
        None,
    );

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_info_serialization() {
        let user = UserInfo {
            id: "u-123".to_string(),
            username: "HermesTrismegistus".to_string(),
            is_admin: false,
        };

        let json = serde_json::to_string(&user).expect("serialize");
        let deserialized: UserInfo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(user, deserialized);
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn test_password_hashing_and_verification() {
        let password = "SecretHermeticPassword123!";
        let hash = bcrypt::hash(password, 4).expect("hash");
        assert!(bcrypt::verify(password, &hash).expect("verify"));
        assert!(!bcrypt::verify("WrongPassword", &hash).expect("verify wrong"));
    }
}
