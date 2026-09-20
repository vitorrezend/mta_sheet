use axum::response::IntoResponse;
use axum::response::sse::{Event, KeepAlive, Sse};
use futures_util::stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;
use sqlx::Row;

pub async fn room_events_sse_handler(
    headers: http::HeaderMap,
    axum::extract::Path(room_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let pool = crate::database::get_db().await;

    let room_exists: bool = sqlx::query("SELECT 1 FROM rooms WHERE id = ?")
        .bind(&room_id)
        .fetch_optional(&pool)
        .await
        .ok()
        .flatten()
        .is_some();

    if !room_exists {
        return (http::StatusCode::NOT_FOUND, "Sala não encontrada").into_response();
    }

    let auth_user_id = {
        if let Some(cookie_hdr) = headers.get(http::header::COOKIE).and_then(|h| h.to_str().ok()) {
            let mut found_token = None;
            for pair in cookie_hdr.split(';') {
                let mut parts = pair.trim().splitn(2, '=');
                if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
                    if k == "session_token" {
                        found_token = Some(v.trim().to_string());
                        break;
                    }
                }
            }
            if let Some(token) = found_token {
                sqlx::query("SELECT user_id FROM sessions WHERE id = ? AND expires_at > CURRENT_TIMESTAMP")
                    .bind(token)
                    .fetch_optional(&pool)
                    .await
                    .ok()
                    .flatten()
                    .map(|r| r.get::<String, _>("user_id"))
            } else {
                None
            }
        } else {
            None
        }
    };

    let user_id = match auth_user_id {
        Some(u) => u,
        None => return (http::StatusCode::UNAUTHORIZED, "Autenticação necessária para ouvir eventos da sala").into_response(),
    };

    let is_authorized: bool = sqlx::query(
        "SELECT 1 FROM rooms WHERE id = ? AND gm_id = ? 
         UNION 
         SELECT 1 FROM room_members WHERE room_id = ? AND user_id = ?"
    )
    .bind(&room_id)
    .bind(&user_id)
    .bind(&room_id)
    .bind(&user_id)
    .fetch_optional(&pool)
    .await
    .ok()
    .flatten()
    .is_some();

    if !is_authorized {
        return (http::StatusCode::FORBIDDEN, "Permissão negada: Você não é membro nem mestre desta sala").into_response();
    }

    let sender = crate::rooms::get_or_create_room_channel(&room_id);
    let rx = sender.subscribe();

    let stream = BroadcastStream::new(rx).filter_map(|msg| async move {
        match msg {
            Ok(event) => {
                let json = serde_json::to_string(&event).unwrap_or_default();
                Some(Ok::<_, std::convert::Infallible>(Event::default().data(json)))
            }
            Err(_) => None,
        }
    });

    Sse::new(stream).keep_alive(KeepAlive::default()).into_response()
}
