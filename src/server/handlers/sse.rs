use axum::response::IntoResponse;
use axum::response::sse::{Event, KeepAlive, Sse};
use futures_util::stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

pub async fn room_events_sse_handler(
    axum::extract::Path(room_id): axum::extract::Path<String>,
) -> impl IntoResponse {
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

    Sse::new(stream).keep_alive(KeepAlive::default())
}
