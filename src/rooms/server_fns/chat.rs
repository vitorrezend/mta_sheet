use leptos::*;
use super::super::models::RoomChatMessage;

/// Quantidade de mensagens carregadas ao abrir a sala (o banco guarda até 200 por sala).
pub const CHAT_HISTORY_LIMIT: i64 = 100;
/// Tamanho máximo de uma mensagem de texto, em caracteres.
pub const CHAT_MAX_CHARS: usize = 1000;

#[cfg(feature = "ssr")]
mod helpers {
    use leptos::*;
    use sqlx::{Row, SqlitePool};
    use crate::errors::{AppError, AppResult};
    use crate::repositories::RoomRepository;
    use crate::rooms::models::{ChatMessageKind, ChatRollData, RoomBroadcastEvent, RoomChatMessage};
    use crate::rules::RollRequest;

    /// Avatares maiores que isso (ex.: imagens base64 legadas) não são copiados para cada mensagem.
    const MAX_AVATAR_URL_LEN: usize = 2048;

    pub struct ChatContext {
        pub pool: SqlitePool,
        pub user_id: String,
    }

    /// Exige usuário logado que seja mestre ou membro da sala.
    pub async fn require_room_member(room_id: &str) -> AppResult<ChatContext> {
        let user_id = crate::auth::get_auth_user_id()
            .await
            .map_err(|e| AppError::internal("Falha ao ler a sessão", e))?
            .ok_or(AppError::NotLoggedIn)?;
        let pool = use_context::<SqlitePool>()
            .ok_or_else(|| AppError::internal("Pool do banco ausente no contexto", "chat"))?;
        if !RoomRepository::check_is_gm_or_member(&pool, room_id, &user_id).await? {
            return Err(AppError::forbidden("Você não é membro desta sala"));
        }
        Ok(ChatContext { pool, user_id })
    }

    /// Mensagens e rolagens por usuário: até 15 a cada 10 segundos (contra spam e scripts).
    pub fn check_chat_rate_limit(ctx: &ChatContext) -> AppResult<()> {
        let allowed = crate::server::middleware::security::check_action_rate_limit(
            &format!("chat:{}", ctx.user_id),
            15,
            std::time::Duration::from_secs(10),
        );
        if allowed {
            Ok(())
        } else {
            Err(AppError::RateLimited("Muitas mensagens em sequência. Aguarde alguns segundos.".into()))
        }
    }

    /// Quem está falando: o jogador (fora de cena) ou um personagem da sala.
    pub struct Speaker {
        pub sender_name: String,
        pub character: Option<String>,
        pub avatar: Option<String>,
    }

    /// Resolve nome e avatar no servidor. Só é possível falar como um personagem
    /// vinculado à sala que pertença ao usuário (o mestre pode usar qualquer um).
    pub async fn resolve_speaker(ctx: &ChatContext, room_id: &str, sheet_id: Option<String>) -> AppResult<Speaker> {
        let sender_name: String = sqlx::query_scalar("SELECT username FROM users WHERE id = ?")
            .bind(&ctx.user_id)
            .fetch_optional(&ctx.pool)
            .await?
            .unwrap_or_else(|| "Jogador".to_string());

        let Some(sheet_id) = sheet_id.filter(|s| !s.trim().is_empty()) else {
            return Ok(Speaker { sender_name, character: None, avatar: None });
        };

        let row = sqlx::query(
            "SELECT cs.name, cs.data, cs.user_id, cs.updated_at, r.gm_id
             FROM character_sheets cs JOIN rooms r ON r.id = cs.room_id
             WHERE cs.id = ? AND cs.room_id = ?",
        )
        .bind(&sheet_id)
        .bind(room_id)
        .fetch_optional(&ctx.pool)
        .await?
        .ok_or_else(|| AppError::not_found("Personagem não encontrado nesta sala"))?;

        let owner: Option<String> = row.get("user_id");
        let gm_id: String = row.get("gm_id");
        if owner.as_deref() != Some(ctx.user_id.as_str()) && gm_id != ctx.user_id {
            return Err(AppError::forbidden("Você só pode falar como os seus próprios personagens"));
        }

        let name: String = row.get("name");
        let data: String = row.get("data");
        let updated_at: String = row.try_get("updated_at").unwrap_or_default();
        let summary = crate::rooms::models::build_room_sheet_summary(sheet_id, name, &data, updated_at, false, true);
        let avatar = Some(summary.photo_url).filter(|url| !url.is_empty() && url.len() <= MAX_AVATAR_URL_LEN);
        Ok(Speaker { sender_name, character: Some(summary.name), avatar })
    }

    fn now_utc() -> String {
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn build_message(room_id: &str, ctx: &ChatContext, speaker: Speaker, kind: ChatMessageKind, content: String, roll: Option<ChatRollData>) -> RoomChatMessage {
        RoomChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            room_id: room_id.to_string(),
            sender_id: ctx.user_id.clone(),
            sender_name: speaker.sender_name,
            sender_character: speaker.character,
            sender_avatar: speaker.avatar,
            content,
            message_type: kind,
            roll_data: roll,
            created_at: now_utc(),
        }
    }

    pub fn roll_dice(request: &RollRequest) -> ChatRollData {
        let dice_results = crate::rules::roll_d10s(request.dice_count);
        let (successes, is_botch) = crate::rules::specialties::evaluate_roll(&dice_results, request.difficulty, request.specialty);
        ChatRollData {
            dice_count: request.dice_count,
            difficulty: request.difficulty,
            specialty: request.specialty,
            specialty_text: None,
            dice_results,
            successes,
            is_botch,
            label: request.label.clone(),
        }
    }

    pub fn roll_summary(roll: &ChatRollData) -> String {
        if roll.is_botch {
            "FALHA CRÍTICA (BOTCH)!".to_string()
        } else if roll.successes == 0 {
            "Nenhum sucesso (Falha Simples)".to_string()
        } else {
            format!("{} sucesso(s)", roll.successes)
        }
    }

    /// Persiste e transmite a mensagem para todos conectados à sala.
    pub async fn publish(ctx: &ChatContext, msg: RoomChatMessage) -> AppResult<RoomChatMessage> {
        RoomRepository::save_room_message(&ctx.pool, &msg)
            .await
            .map_err(|e| AppError::internal("Erro ao salvar mensagem do chat", e))?;
        crate::rooms::events::broadcast_to_room(&msg.room_id, RoomBroadcastEvent::ChatMessage(msg.clone()));
        Ok(msg)
    }

    pub async fn publish_roll(room_id: &str, ctx: &ChatContext, speaker: Speaker, request: &RollRequest) -> AppResult<RoomChatMessage> {
        let roll = roll_dice(request);
        let msg = build_message(room_id, ctx, speaker, ChatMessageKind::Roll, roll_summary(&roll), Some(roll));
        publish(ctx, msg).await
    }
}

#[server(endpoint = "get_room_chat_messages")]
pub async fn get_room_chat_messages(room_id: String) -> Result<Vec<RoomChatMessage>, ServerFnError> {
    let ctx = helpers::require_room_member(&room_id).await?;
    let history = crate::repositories::RoomRepository::get_room_messages(&ctx.pool, &room_id, CHAT_HISTORY_LIMIT)
        .await
        .map_err(|e| crate::errors::AppError::internal("Erro ao buscar histórico do chat", e))?;
    Ok(history)
}

/// Envia uma mensagem. `sheet_id` escolhe falar como um personagem (em cena);
/// sem ele, a mensagem é do jogador (fora de cena). Textos `/r ...` viram rolagens.
#[server(endpoint = "send_room_chat_message")]
pub async fn send_room_chat_message(
    room_id: String,
    content: String,
    sheet_id: Option<String>,
) -> Result<RoomChatMessage, ServerFnError> {
    use crate::rooms::models::ChatMessageKind;

    let content = content.trim().to_string();
    if content.is_empty() {
        return Err(ServerFnError::new("A mensagem não pode estar vazia"));
    }
    if content.chars().count() > CHAT_MAX_CHARS {
        return Err(ServerFnError::new(format!("A mensagem excede o limite de {} caracteres", CHAT_MAX_CHARS)));
    }

    let ctx = helpers::require_room_member(&room_id).await?;
    helpers::check_chat_rate_limit(&ctx)?;
    let speaker = helpers::resolve_speaker(&ctx, &room_id, sheet_id).await?;

    if let Some(request) = crate::rules::parse_roll_command(&content) {
        return Ok(helpers::publish_roll(&room_id, &ctx, speaker, &request).await?);
    }

    let kind = if speaker.character.is_some() { ChatMessageKind::Ic } else { ChatMessageKind::Ooc };
    let msg = helpers::build_message(&room_id, &ctx, speaker, kind, content, None);
    Ok(helpers::publish(&ctx, msg).await?)
}

#[server(endpoint = "roll_room_dice")]
pub async fn roll_room_dice(
    room_id: String,
    dice_count: i32,
    difficulty: i32,
    specialty: bool,
    label: String,
    sheet_id: Option<String>,
) -> Result<RoomChatMessage, ServerFnError> {
    let ctx = helpers::require_room_member(&room_id).await?;
    helpers::check_chat_rate_limit(&ctx)?;
    let speaker = helpers::resolve_speaker(&ctx, &room_id, sheet_id).await?;
    let request = crate::rules::RollRequest::new(dice_count, difficulty, specialty, &label);
    Ok(helpers::publish_roll(&room_id, &ctx, speaker, &request).await?)
}

#[server(endpoint = "clear_room_chat")]
pub async fn clear_room_chat(room_id: String) -> Result<(), ServerFnError> {
    use crate::repositories::RoomRepository;
    use crate::rooms::models::RoomBroadcastEvent;

    let ctx = helpers::require_room_member(&room_id).await?;
    let is_gm = RoomRepository::check_is_gm(&ctx.pool, &room_id, &ctx.user_id)
        .await
        .map_err(crate::errors::AppError::from)?;
    if !is_gm {
        return Err(crate::errors::AppError::forbidden("Apenas o Narrador pode limpar o histórico do chat.").into());
    }

    RoomRepository::clear_room_messages(&ctx.pool, &room_id)
        .await
        .map_err(|e| crate::errors::AppError::internal("Erro ao limpar chat", e))?;

    crate::rooms::events::broadcast_to_room(&room_id, RoomBroadcastEvent::ChatCleared);
    Ok(())
}
