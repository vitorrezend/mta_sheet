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

// ----------------------------------------------------------------------------
// Presença: quem está com a sala aberta agora (uma entrada por conexão SSE)
// ----------------------------------------------------------------------------

#[cfg(feature = "ssr")]
static PRESENCE: LazyLock<Mutex<HashMap<String, HashMap<String, usize>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Usuários conectados à sala, em ordem estável.
#[cfg(feature = "ssr")]
pub fn online_users(room_id: &str) -> Vec<String> {
    let presence = PRESENCE.lock().unwrap_or_else(|p| p.into_inner());
    let mut users: Vec<String> = presence.get(room_id).map(|m| m.keys().cloned().collect()).unwrap_or_default();
    users.sort();
    users
}

#[cfg(feature = "ssr")]
fn broadcast_presence(room_id: &str) {
    let online_user_ids = online_users(room_id);
    broadcast_to_room(room_id, RoomBroadcastEvent::PresenceChanged { online_user_ids });
}

/// Mantém o usuário "online" na sala enquanto existir; ao ser descartado (conexão SSE
/// encerrada), remove a presença e avisa os demais.
#[cfg(feature = "ssr")]
pub struct PresenceGuard {
    room_id: String,
    user_id: String,
}

/// Registra uma conexão do usuário na sala. Várias abas do mesmo usuário contam uma vez.
#[cfg(feature = "ssr")]
pub fn join_presence(room_id: &str, user_id: &str) -> PresenceGuard {
    {
        let mut presence = PRESENCE.lock().unwrap_or_else(|p| p.into_inner());
        *presence.entry(room_id.to_string()).or_default().entry(user_id.to_string()).or_default() += 1;
    }
    broadcast_presence(room_id);
    PresenceGuard { room_id: room_id.to_string(), user_id: user_id.to_string() }
}

#[cfg(feature = "ssr")]
impl Drop for PresenceGuard {
    fn drop(&mut self) {
        let changed = {
            let mut presence = PRESENCE.lock().unwrap_or_else(|p| p.into_inner());
            let Some(room) = presence.get_mut(&self.room_id) else { return };
            let gone = match room.get_mut(&self.user_id) {
                Some(count) if *count > 1 => {
                    *count -= 1;
                    false
                }
                Some(_) => {
                    room.remove(&self.user_id);
                    true
                }
                None => false,
            };
            if room.is_empty() {
                presence.remove(&self.room_id);
            }
            gone
        };
        if changed {
            broadcast_presence(&self.room_id);
        }
    }
}

// ----------------------------------------------------------------------------
// Notificações de mudança de ficha para quem está na sala
// ----------------------------------------------------------------------------

/// Avisa a sala da ficha (se houver) que ela mudou.
///
/// Ficha visível: envia o resumo novo pronto (os clientes atualizam sem recarregar nada).
/// Ficha oculta: envia só `RoomUpdated`, para não vazar os dados a quem não pode vê-la;
/// cada cliente recarrega os detalhes e o servidor filtra o que ele pode ver.
#[cfg(feature = "ssr")]
pub async fn notify_sheet_changed(pool: &sqlx::SqlitePool, sheet_id: &str) {
    use sqlx::Row;
    let Ok(Some(row)) = sqlx::query(
        "SELECT room_id, name, data, updated_at, is_hidden_in_room FROM character_sheets WHERE id = ?",
    )
    .bind(sheet_id)
    .fetch_optional(pool)
    .await
    else {
        return;
    };
    let Some(room_id) = row.try_get::<Option<String>, _>("room_id").ok().flatten() else { return };
    let is_hidden = row.try_get::<i64, _>("is_hidden_in_room").map(|v| v == 1).unwrap_or(false);
    if is_hidden {
        broadcast_to_room(&room_id, RoomBroadcastEvent::RoomUpdated);
        return;
    }
    let name: String = row.try_get("name").unwrap_or_default();
    let data: String = row.try_get("data").unwrap_or_default();
    let updated_at: String = row.try_get("updated_at").unwrap_or_default();
    let summary = super::models::build_room_sheet_summary(sheet_id.to_string(), name, &data, updated_at, false, false);
    broadcast_to_room(&room_id, RoomBroadcastEvent::SheetUpdated(summary));
}

/// Pede a todos na sala que recarreguem os detalhes (membros, fichas, configurações).
#[cfg(feature = "ssr")]
pub fn notify_room_changed(room_id: &str) {
    broadcast_to_room(room_id, RoomBroadcastEvent::RoomUpdated);
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
