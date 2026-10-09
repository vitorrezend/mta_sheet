use leptos::*;
use super::super::models::{RoomInitiativeData, RoomBroadcastEvent};
use super::super::events::broadcast_to_room;

#[server(endpoint = "update_room_initiative")]
pub async fn update_room_initiative(room_id: String, initiative: RoomInitiativeData, play_sound: bool) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    use crate::auth::get_auth_user_id;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para alterar a iniciativa da sala")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let is_gm: bool = sqlx::query_scalar::<_, i64>("SELECT 1 FROM rooms WHERE id = ? AND gm_id = ?")
        .bind(&room_id)
        .bind(&user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .is_some();

    if !is_gm {
        return Err(ServerFnError::new("Apenas o Narrador pode alterar a iniciativa da sala."));
    }

    let data_json = serde_json::to_string(&initiative).unwrap_or_default();
    sqlx::query("UPDATE rooms SET initiative_data = ? WHERE id = ?")
        .bind(&data_json)
        .bind(&room_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao atualizar iniciativa: {}", e)))?;

    broadcast_to_room(&room_id, RoomBroadcastEvent {
        event_type: if play_sound { "DICE_ROLLED".to_string() } else { "INITIATIVE_UPDATE".to_string() },
        initiative,
        play_sound,
    });

    Ok(())
}
