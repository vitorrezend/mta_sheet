use axum::response::IntoResponse;
use axum::response::sse::{Event, KeepAlive, Sse};
use futures_util::stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;
use crate::rooms::models::RoomBroadcastEvent;

pub async fn room_events_sse_handler(
    crate::auth::AuthUser(user_id): crate::auth::AuthUser,
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

    let is_authorized = crate::repositories::RoomRepository::check_is_gm_or_member(&pool, &room_id, &user_id)
        .await
        .unwrap_or(false);

    if !is_authorized {
        return (http::StatusCode::FORBIDDEN, "Permissão negada: Você não é membro nem mestre desta sala").into_response();
    }

    let sender = crate::rooms::get_or_create_room_channel(&room_id);
    let rx = sender.subscribe();

    // Inscreve antes de marcar presença: assim este cliente também recebe a lista de online.
    // O guard vive dentro do stream; quando a conexão cai, ele sai da presença da sala.
    let presence = crate::rooms::events::join_presence(&room_id, &user_id);
    let stream = BroadcastStream::new(rx).map(move |msg| {
        let _keep_online = &presence;
        // Cliente lento perdeu eventos da fila: pede para ele recarregar o estado
        let event = msg.unwrap_or(RoomBroadcastEvent::Resync);
        let json = serde_json::to_string(&event).unwrap_or_default();
        Ok::<_, std::convert::Infallible>(Event::default().data(json))
    });

    // `retry` encurta a espera de reconexão automática do EventSource (Safari usa ~3s por padrão)
    let hello = futures_util::stream::once(async {
        Ok::<_, std::convert::Infallible>(Event::default().retry(std::time::Duration::from_secs(2)).comment("connected"))
    });

    Sse::new(hello.chain(stream)).keep_alive(KeepAlive::default()).into_response()
}
