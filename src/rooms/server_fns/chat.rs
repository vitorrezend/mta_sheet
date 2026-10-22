use leptos::*;
use super::super::models::RoomChatMessage;
#[cfg(feature = "ssr")]
use super::super::models::{ChatRollData, RoomBroadcastEvent};
#[cfg(feature = "ssr")]
use super::super::events::broadcast_to_room;

#[server(endpoint = "get_room_chat_messages")]
pub async fn get_room_chat_messages(room_id: String) -> Result<Vec<RoomChatMessage>, ServerFnError> {
    use sqlx::SqlitePool;
    use crate::auth::get_auth_user_id;
    use crate::repositories::room_repository::RoomRepository;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para visualizar o chat da sala")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let is_authorized: bool = sqlx::query_scalar::<_, i64>(
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
    .map_err(|e| ServerFnError::new(e.to_string()))?
    .is_some();

    if !is_authorized {
        return Err(ServerFnError::new("Você não é membro desta sala"));
    }

    RoomRepository::get_room_messages(&pool, &room_id, 100)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao buscar histórico do chat: {}", e)))
}

#[server(endpoint = "send_room_chat_message")]
pub async fn send_room_chat_message(
    room_id: String,
    content: String,
    sender_character: Option<String>,
    sender_avatar: Option<String>,
    message_type: String,
) -> Result<RoomChatMessage, ServerFnError> {
    use sqlx::SqlitePool;
    use crate::auth::get_auth_user_id;
    use crate::repositories::room_repository::RoomRepository;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para enviar mensagens na sala")
    })?;

    let clean_content = content.trim().to_string();
    if clean_content.is_empty() {
        return Err(ServerFnError::new("A mensagem não pode estar vazia"));
    }
    if clean_content.len() > 1000 {
        return Err(ServerFnError::new("A mensagem excede o limite de 1000 caracteres"));
    }

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let is_authorized: bool = sqlx::query_scalar::<_, i64>(
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
    .map_err(|e| ServerFnError::new(e.to_string()))?
    .is_some();

    if !is_authorized {
        return Err(ServerFnError::new("Você não é membro desta sala"));
    }

    let sender_name: String = sqlx::query_scalar("SELECT username FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .unwrap_or_else(|| "Jogador".to_string());

    let msg = RoomChatMessage {
        id: uuid::Uuid::new_v4().to_string(),
        room_id: room_id.clone(),
        sender_id: user_id,
        sender_name,
        sender_character,
        sender_avatar,
        content: clean_content,
        message_type,
        roll_data: None,
        created_at: String::new(),
    };

    RoomRepository::save_room_message(&pool, &msg)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao salvar mensagem: {}", e)))?;

    broadcast_to_room(&room_id, RoomBroadcastEvent {
        event_type: "CHAT_MESSAGE".to_string(),
        initiative: Default::default(),
        play_sound: false,
        chat_message: Some(msg.clone()),
    });

    Ok(msg)
}

#[server(endpoint = "roll_room_dice")]
pub async fn roll_room_dice(
    room_id: String,
    dice_count: i32,
    difficulty: i32,
    specialty: bool,
    label: String,
    sender_character: Option<String>,
    sender_avatar: Option<String>,
) -> Result<RoomChatMessage, ServerFnError> {
    use sqlx::SqlitePool;
    use crate::auth::get_auth_user_id;
    use crate::repositories::room_repository::RoomRepository;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para rolar dados na sala")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    let is_authorized: bool = sqlx::query_scalar::<_, i64>(
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
    .map_err(|e| ServerFnError::new(e.to_string()))?
    .is_some();

    if !is_authorized {
        return Err(ServerFnError::new("Você não é membro desta sala"));
    }

    let sender_name: String = sqlx::query_scalar("SELECT username FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .unwrap_or_else(|| "Jogador".to_string());

    let count = dice_count.clamp(1, 30);
    let diff = difficulty.clamp(3, 10);

    let mut dice_results = Vec::with_capacity(count as usize);
    let mut rng_buf = [0u8; 1];
    for _ in 0..count {
        let _ = getrandom::getrandom(&mut rng_buf);
        let val = (rng_buf[0] % 10 + 1) as i32;
        dice_results.push(val);
    }

    let (successes, is_botch) = crate::rules::specialties::evaluate_roll(&dice_results, diff, specialty);

    let roll_data = ChatRollData {
        dice_count: count,
        difficulty: diff,
        specialty,
        specialty_text: None,
        dice_results,
        successes,
        is_botch,
        label: if label.trim().is_empty() { "Rolagem de Dados".to_string() } else { label.trim().to_string() },
    };

    let outcome_text = if is_botch {
        "FALHA CRÍTICA (BOTCH)!".to_string()
    } else if successes == 0 {
        "Nenhum sucesso (Falha Simples)".to_string()
    } else {
        format!("{} sucesso(s)", successes)
    };

    let msg = RoomChatMessage {
        id: uuid::Uuid::new_v4().to_string(),
        room_id: room_id.clone(),
        sender_id: user_id,
        sender_name,
        sender_character,
        sender_avatar,
        content: outcome_text,
        message_type: "roll".to_string(),
        roll_data: Some(roll_data),
        created_at: String::new(),
    };

    RoomRepository::save_room_message(&pool, &msg)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao registrar rolagem: {}", e)))?;

    broadcast_to_room(&room_id, RoomBroadcastEvent {
        event_type: "DICE_ROLLED".to_string(),
        initiative: Default::default(),
        play_sound: true,
        chat_message: Some(msg.clone()),
    });

    Ok(msg)
}

#[server(endpoint = "clear_room_chat")]
pub async fn clear_room_chat(room_id: String) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    use crate::auth::get_auth_user_id;
    use crate::repositories::room_repository::RoomRepository;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado")
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
        return Err(ServerFnError::new("Apenas o Narrador pode limpar o histórico do chat."));
    }

    RoomRepository::clear_room_messages(&pool, &room_id)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao limpar chat: {}", e)))?;

    broadcast_to_room(&room_id, RoomBroadcastEvent {
        event_type: "CHAT_CLEARED".to_string(),
        initiative: Default::default(),
        play_sound: false,
        chat_message: None,
    });

    Ok(())
}
