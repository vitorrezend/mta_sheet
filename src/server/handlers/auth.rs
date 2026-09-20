use axum::response::IntoResponse;
use sqlx::Row;
use std::sync::{Arc, Mutex, LazyLock};
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct AuthRateLimiter {
    attempts: HashMap<String, Vec<Instant>>,
}

static AUTH_RATE_LIMITER: LazyLock<Arc<Mutex<AuthRateLimiter>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(AuthRateLimiter {
        attempts: HashMap::new(),
    }))
});

pub fn check_auth_rate_limit(client_ip: &str, max_attempts: usize, window: Duration) -> bool {
    let mut limiter = match AUTH_RATE_LIMITER.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    };
    let now = Instant::now();
    let entry = limiter.attempts.entry(client_ip.to_string()).or_default();
    entry.retain(|&time| now.duration_since(time) < window);
    if entry.len() >= max_attempts {
        false
    } else {
        entry.push(now);
        true
    }
}

pub fn reset_auth_rate_limit_for_test() {
    let mut limiter = match AUTH_RATE_LIMITER.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    };
    limiter.attempts.clear();
}

use crate::server::middleware::security::extract_client_ip;

fn is_secure_connection(headers: &http::HeaderMap) -> bool {
    let is_prod = std::env::var("LEPTOS_ENV").map(|e| e.to_lowercase() == "production" || e.to_lowercase() == "prod").unwrap_or(false)
        || std::env::var("ENABLE_SECURE_COOKIE").map(|e| e == "1" || e.to_lowercase() == "true").unwrap_or(false);

    if is_prod {
        return true;
    }

    if let Some(proto) = headers.get("x-forwarded-proto").and_then(|h| h.to_str().ok()) {
        return proto.eq_ignore_ascii_case("https");
    }

    false
}

fn build_session_cookie(session_token: &str, is_secure: bool) -> String {
    let secure_flag = if is_secure { "; Secure" } else { "" };
    format!("session_token={}; Path=/; SameSite=Lax; HttpOnly{}; Max-Age=2592000", session_token, secure_flag)
}

#[derive(serde::Deserialize)]
pub struct FormAuthPayload {
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub confirm_password: Option<String>,
}

pub async fn form_login_handler(
    headers: http::HeaderMap,
    axum::extract::Form(payload): axum::extract::Form<FormAuthPayload>,
) -> impl IntoResponse {
    let client_ip = extract_client_ip(&headers);
    if !check_auth_rate_limit(&client_ip, 10, Duration::from_secs(60)) {
        return (
            [(http::header::LOCATION, "/login?error=Muitas+tentativas+de+login.+Aguarde+1+minuto+antes+de+tentar+novamente.".to_string())],
            http::StatusCode::SEE_OTHER,
        )
            .into_response();
    }

    let clean_user = payload.username.trim().to_string();
    if clean_user.is_empty() || payload.password.is_empty() {
        return (
            [(http::header::LOCATION, "/login?error=Usu%C3%A1rio+e+senha+s%C3%A3o+obrigat%C3%B3rios".to_string())],
            http::StatusCode::SEE_OTHER,
        )
            .into_response();
    }

    let pool = crate::database::get_db().await;

    let row = match sqlx::query("SELECT id, username, password_hash FROM users WHERE username = ? COLLATE NOCASE")
        .bind(&clean_user)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(r)) => r,
        _ => {
            return (
                [(http::header::LOCATION, "/login?error=Usu%C3%A1rio+ou+senha+incorretos".to_string())],
                http::StatusCode::SEE_OTHER,
            )
                .into_response();
        }
    };

    let user_id: String = row.get("id");
    let password_hash: String = row.get("password_hash");

    if bcrypt::verify(&payload.password, &password_hash).unwrap_or(false) {
        let session_token = match crate::auth::create_session_with_fifo(&pool, &user_id).await {
            Ok(t) => t,
            Err(e) => {
                log::error!("Erro ao criar sessão FIFO no login: {}", e);
                return (
                    [(http::header::LOCATION, "/login?error=Erro+interno+ao+criar+sess%C3%A3o".to_string())],
                    http::StatusCode::SEE_OTHER,
                ).into_response();
            }
        };

        let cookie_str = build_session_cookie(&session_token, is_secure_connection(&headers));
        (
            [
                (http::header::SET_COOKIE, cookie_str),
                (http::header::LOCATION, "/".to_string()),
            ],
            http::StatusCode::SEE_OTHER,
        )
            .into_response()
    } else {
        (
            [(http::header::LOCATION, "/login?error=Usu%C3%A1rio+ou+senha+incorretos".to_string())],
            http::StatusCode::SEE_OTHER,
        )
            .into_response()
    }
}

pub async fn form_register_handler(
    headers: http::HeaderMap,
    axum::extract::Form(payload): axum::extract::Form<FormAuthPayload>,
) -> impl IntoResponse {
    let client_ip = extract_client_ip(&headers);
    if !check_auth_rate_limit(&client_ip, 5, Duration::from_secs(60)) {
        return (
            [(http::header::LOCATION, "/login?tab=register&error=Muitas+tentativas+de+cadastro.+Aguarde+1+minuto+antes+de+tentar+novamente.".to_string())],
            http::StatusCode::SEE_OTHER,
        )
            .into_response();
    }

    let clean_user = payload.username.trim().to_string();
    if clean_user.len() < 3 {
        return (
            [(http::header::LOCATION, "/login?tab=register&error=Nome+de+usu%C3%A1rio+deve+ter+no+m%C3%ADnimo+3+caracteres".to_string())],
            http::StatusCode::SEE_OTHER,
        )
            .into_response();
    }

    if let Err(err) = crate::auth::validate_password_strength(&payload.password) {
        let err_url = match err {
            "A senha deve ter no mínimo 8 caracteres." => "A+senha+deve+ter+no+m%C3%ADnimo+8+caracteres.",
            "A senha não pode exceder 128 caracteres." => "A+senha+n%C3%A3o+pode+exceder+128+caracteres.",
            "A senha deve conter pelo menos uma letra e um número." => "A+senha+deve+conter+pelo+menos+uma+letra+e+um+n%C3%BAmero.",
            _ => "Senha+inv%C3%A1lida",
        };
        return (
            [(http::header::LOCATION, format!("/login?tab=register&error={}", err_url))],
            http::StatusCode::SEE_OTHER,
        )
            .into_response();
    }

    if let Some(confirm) = payload.confirm_password {
        if !confirm.is_empty() && confirm != payload.password {
            return (
                [(http::header::LOCATION, "/login?tab=register&error=As+senhas+n%C3%A3o+conferem".to_string())],
                http::StatusCode::SEE_OTHER,
            )
                .into_response();
        }
    }

    let pool = crate::database::get_db().await;

    let user_count: i64 = sqlx::query("SELECT COUNT(*) as count FROM users")
        .fetch_one(&pool)
        .await
        .map(|r| r.get("count"))
        .unwrap_or(0);

    let is_admin = user_count == 0 || crate::auth::is_username_in_admin_env(&clean_user);

    let password_hash = match bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return (
                [(http::header::LOCATION, "/login?tab=register&error=Erro+ao+criptografar+senha".to_string())],
                http::StatusCode::SEE_OTHER,
            )
                .into_response();
        }
    };

    let user_id = uuid::Uuid::new_v4().to_string();
    let insert_res = sqlx::query("INSERT INTO users (id, username, password_hash, is_admin) VALUES (?, ?, ?, ?)")
        .bind(&user_id)
        .bind(&clean_user)
        .bind(&password_hash)
        .bind(if is_admin { 1i64 } else { 0i64 })
        .execute(&pool)
        .await;

    if let Err(_) = insert_res {
        return (
            [(http::header::LOCATION, "/login?tab=register&error=Este+nome+de+usu%C3%A1rio+j%C3%A1+est%C3%A1+em+uso".to_string())],
            http::StatusCode::SEE_OTHER,
        )
            .into_response();
    }

    let session_token = match crate::auth::create_session_with_fifo(&pool, &user_id).await {
        Ok(t) => t,
        Err(e) => {
            log::error!("Erro ao criar sessão FIFO no registro: {}", e);
            return (
                [(http::header::LOCATION, "/login?tab=register&error=Erro+interno+ao+criar+sess%C3%A3o".to_string())],
                http::StatusCode::SEE_OTHER,
            ).into_response();
        }
    };

    let cookie_str = build_session_cookie(&session_token, is_secure_connection(&headers));
    (
        [
            (http::header::SET_COOKIE, cookie_str),
            (http::header::LOCATION, "/".to_string()),
        ],
        http::StatusCode::SEE_OTHER,
    )
        .into_response()
}
