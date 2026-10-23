#[cfg(feature = "ssr")]
use super::models::RoomBroadcastEvent;

#[cfg(feature = "ssr")]
use std::sync::{Arc, Mutex, LazyLock};
#[cfg(feature = "ssr")]
use std::collections::HashMap;
#[cfg(feature = "ssr")]
use tokio::sync::broadcast;

#[cfg(feature = "ssr")]
static ROOM_CHANNELS: LazyLock<Arc<Mutex<HashMap<String, broadcast::Sender<RoomBroadcastEvent>>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

#[cfg(feature = "ssr")]
pub fn get_or_create_room_channel(room_id: &str) -> broadcast::Sender<RoomBroadcastEvent> {
    let mut channels = match ROOM_CHANNELS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    // Limpeza oportunística: previne vazamento de memória descartando canais inativos sem ouvintes
    if channels.len() > 30 {
        channels.retain(|_, sender| sender.receiver_count() > 0);
    }
    if let Some(sender) = channels.get(room_id) {
        sender.clone()
    } else {
        let (sender, _) = broadcast::channel(100);
        channels.insert(room_id.to_string(), sender.clone());
        sender
    }
}

#[cfg(feature = "ssr")]
pub fn remove_room_channel(room_id: &str) {
    let mut channels = match ROOM_CHANNELS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    channels.remove(room_id);
}

#[cfg(feature = "ssr")]
pub fn prune_inactive_room_channels() {
    let mut channels = match ROOM_CHANNELS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    channels.retain(|_, sender| sender.receiver_count() > 0);
}

#[cfg(feature = "ssr")]
pub fn broadcast_to_room(room_id: &str, event: RoomBroadcastEvent) {
    let sender = get_or_create_room_channel(room_id);
    let _ = sender.send(event);
}

#[cfg(feature = "ssr")]
pub fn generate_room_code() -> String {
    let chars = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let bytes = uuid::Uuid::new_v4().into_bytes();
    let c1 = (bytes[0] as usize) % chars.len();
    let c2 = (bytes[1] as usize) % chars.len();
    let c3 = (bytes[2] as usize) % chars.len();
    let c4 = (bytes[3] as usize) % chars.len();
    format!("MTA-{}{}{}{}", &chars[c1..c1+1], &chars[c2..c2+1], &chars[c3..c3+1], &chars[c4..c4+1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "ssr")]
    #[test]
    fn test_room_code_generation() {
        let code = generate_room_code();
        assert!(code.starts_with("MTA-"));
        assert_eq!(code.len(), 8);
        assert!(code.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'));
    }
}
