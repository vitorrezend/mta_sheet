use axum::response::IntoResponse;
use sqlx::Row;

#[derive(serde::Deserialize)]
pub struct FormAuthPayload {
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub confirm_password: Option<String>,
}

pub async fn form_login_handler(
    axum::extract::Form(payload): axum::extract::Form<FormAuthPayload>,
) -> impl IntoResponse {
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
        let session_token = uuid::Uuid::new_v4().to_string();
        let _ = sqlx::query("INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, datetime('now', '+30 days'))")
            .bind(&session_token)
            .bind(&user_id)
            .execute(&pool)
            .await;

        let cookie_str = format!("session_token={}; Path=/; SameSite=Lax; HttpOnly; Max-Age=2592000", session_token);
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
    axum::extract::Form(payload): axum::extract::Form<FormAuthPayload>,
) -> impl IntoResponse {
    let clean_user = payload.username.trim().to_string();
    if clean_user.len() < 3 || payload.password.len() < 4 {
        return (
            [(http::header::LOCATION, "/login?tab=register&error=Usu%C3%A1rio+(m%C3%ADnimo+3+caracteres)+ou+senha+(m%C3%ADnimo+4+caracteres)+inv%C3%A1lidos".to_string())],
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

    let session_token = uuid::Uuid::new_v4().to_string();
    let _ = sqlx::query("INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, datetime('now', '+30 days'))")
        .bind(&session_token)
        .bind(&user_id)
        .execute(&pool)
        .await;

    let cookie_str = format!("session_token={}; Path=/; SameSite=Lax; HttpOnly; Max-Age=2592000", session_token);
    (
        [
            (http::header::SET_COOKIE, cookie_str),
            (http::header::LOCATION, "/".to_string()),
        ],
        http::StatusCode::SEE_OTHER,
    )
        .into_response()
}
