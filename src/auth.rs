use serde::{Deserialize, Serialize};
use leptos::*;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
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
    log::info!("Received Cookie header: {}", cookie_header);
    for pair in cookie_header.split(';') {
        let mut parts = pair.trim().splitn(2, '=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            if key.trim() == "session_token" {
                let token = value.trim().to_string();
                log::info!("Extracted session_token: {}", token);
                return Some(token);
            }
        }
    }
    None
}

#[cfg(feature = "ssr")]
pub fn set_session_cookie(token: &str, max_age_secs: i64) {
    if let Some(res_options) = use_context::<leptos_axum::ResponseOptions>() {
        let cookie_str = format!(
            "session_token={}; Path=/; SameSite=Lax; Max-Age={}",
            token, max_age_secs
        );
        if let Ok(header_val) = http::HeaderValue::from_str(&cookie_str) {
            res_options.insert_header(http::header::SET_COOKIE, header_val);
            log::info!("Set-Cookie registered: {}", cookie_str);
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
        "SELECT u.id, u.username FROM sessions s 
         JOIN users u ON s.user_id = u.id 
         WHERE s.id = ? AND s.expires_at > CURRENT_TIMESTAMP"
    )
    .bind(token)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(row.map(|r| UserInfo {
        id: r.get("id"),
        username: r.get("username"),
    }))
}

#[server(endpoint = "register")]
pub async fn register(username: String, password: String) -> Result<UserInfo, ServerFnError> {
    let clean_user = username.trim().to_string();
    if clean_user.len() < 3 {
        return Err(ServerFnError::new("Nome de usuário deve ter no mínimo 3 caracteres"));
    }
    if password.len() < 4 {
        return Err(ServerFnError::new("A senha deve ter no mínimo 4 caracteres"));
    }

    use sqlx::SqlitePool;
    use uuid::Uuid;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let password_hash = bcrypt::hash(&password, bcrypt::DEFAULT_COST)
        .map_err(|e| ServerFnError::new(format!("Erro ao processar senha: {}", e)))?;

    let user_id = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO users (id, username, password_hash) VALUES (?, ?, ?)")
        .bind(&user_id)
        .bind(&clean_user)
        .bind(&password_hash)
        .execute(&pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE") || e.to_string().contains("users.username") {
                ServerFnError::new("Este nome de usuário já está em uso")
            } else {
                ServerFnError::new(format!("Erro ao criar usuário: {}", e))
            }
        })?;

    // Create session for 30 days
    let session_token = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO sessions (id, user_id, expires_at) 
         VALUES (?, ?, datetime('now', '+30 days'))"
    )
    .bind(&session_token)
    .bind(&user_id)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Erro ao criar sessão: {}", e)))?;

    set_session_cookie(&session_token, 30 * 24 * 60 * 60);

    log::info!("Registered new user: {}", clean_user);
    Ok(UserInfo {
        id: user_id,
        username: clean_user,
    })
}

#[server(endpoint = "login")]
pub async fn login(username: String, password: String) -> Result<UserInfo, ServerFnError> {
    let clean_user = username.trim().to_string();
    if clean_user.is_empty() || password.is_empty() {
        return Err(ServerFnError::new("Usuário e senha são obrigatórios"));
    }

    use sqlx::{SqlitePool, Row};
    use uuid::Uuid;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let row = sqlx::query("SELECT id, username, password_hash FROM users WHERE username = ? COLLATE NOCASE")
        .bind(&clean_user)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Usuário ou senha incorretos"))?;

    let user_id: String = row.get("id");
    let actual_username: String = row.get("username");
    let password_hash: String = row.get("password_hash");

    let is_valid = bcrypt::verify(&password, &password_hash)
        .map_err(|_| ServerFnError::new("Erro ao verificar credenciais"))?;

    if !is_valid {
        return Err(ServerFnError::new("Usuário ou senha incorretos"));
    }

    // Create session for 30 days
    let session_token = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO sessions (id, user_id, expires_at) 
         VALUES (?, ?, datetime('now', '+30 days'))"
    )
    .bind(&session_token)
    .bind(&user_id)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Erro ao criar sessão: {}", e)))?;

    set_session_cookie(&session_token, 30 * 24 * 60 * 60);

    log::info!("Logged in user: {}", actual_username);
    Ok(UserInfo {
        id: user_id,
        username: actual_username,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_info_serialization() {
        let user = UserInfo {
            id: "u-123".to_string(),
            username: "HermesTrismegistus".to_string(),
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
