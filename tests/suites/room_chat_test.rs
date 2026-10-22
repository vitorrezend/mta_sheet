use mta_sheet::rooms::models::{RoomChatMessage, ChatRollData, RoomBroadcastEvent};
use mta_sheet::rules::specialties::evaluate_roll;

#[test]
fn test_room_chat_message_and_roll_data_serialization() {
    let roll = ChatRollData {
        dice_count: 5,
        difficulty: 6,
        specialty: true,
        specialty_text: Some("Tiro Rápido".to_string()),
        dice_results: vec![10, 8, 4, 1, 6],
        successes: 3, // 10 (2) + 8 (1) + 6 (1) - 1 (1) = 3
        is_botch: false,
        label: "Destreza + Armas de Fogo".to_string(),
    };

    let msg = RoomChatMessage {
        id: "msg-123".to_string(),
        room_id: "room-abc".to_string(),
        sender_id: "user-456".to_string(),
        sender_name: "Saint".to_string(),
        sender_character: Some("Victor Crowley".to_string()),
        sender_avatar: Some("/uploads/avatar.png".to_string()),
        content: "3 sucesso(s)".to_string(),
        message_type: "roll".to_string(),
        roll_data: Some(roll),
        created_at: "2026-10-22 17:00:00".to_string(),
    };

    let json = serde_json::to_string(&msg).expect("Falha ao serializar mensagem");
    assert!(json.contains("Destreza + Armas de Fogo"));
    assert!(json.contains("Victor Crowley"));

    let deserialized: RoomChatMessage = serde_json::from_str(&json).expect("Falha ao deserializar");
    assert_eq!(deserialized.id, "msg-123");
    assert_eq!(deserialized.sender_character.as_deref(), Some("Victor Crowley"));
    assert_eq!(deserialized.roll_data.as_ref().unwrap().successes, 3);
    assert_eq!(deserialized.roll_data.as_ref().unwrap().dice_results, vec![10, 8, 4, 1, 6]);
}

#[test]
fn test_room_broadcast_event_chat_integration() {
    let msg = RoomChatMessage {
        id: "m1".to_string(),
        room_id: "r1".to_string(),
        sender_id: "u1".to_string(),
        sender_name: "Mestre".to_string(),
        sender_character: None,
        sender_avatar: None,
        content: "A sessão foi iniciada!".to_string(),
        message_type: "ooc".to_string(),
        roll_data: None,
        created_at: "2026-10-22 17:05:00".to_string(),
    };

    let event = RoomBroadcastEvent {
        event_type: "CHAT_MESSAGE".to_string(),
        initiative: Default::default(),
        play_sound: false,
        chat_message: Some(msg),
    };

    let json = serde_json::to_string(&event).expect("Falha ao serializar evento");
    assert!(json.contains("CHAT_MESSAGE"));
    assert!(json.contains("A sessão foi iniciada!"));

    let deserialized: RoomBroadcastEvent = serde_json::from_str(&json).expect("Falha ao deserializar evento");
    assert_eq!(deserialized.event_type, "CHAT_MESSAGE");
    assert!(!deserialized.play_sound);
    assert_eq!(deserialized.chat_message.unwrap().content, "A sessão foi iniciada!");
}

#[test]
fn test_m20_dice_roll_evaluation_and_botch_logic() {
    // 1. Rolagem com especialidade ativa (10 conta dobro, 1 cancela)
    let dice = [10, 7, 5, 2, 1];
    let (successes, is_botch) = evaluate_roll(&dice, 6, true);
    // 10 = +2, 7 = +1, 1 = -1 -> Total 2
    assert_eq!(successes, 2);
    assert!(!is_botch);

    // 2. Rolagem comum sem especialidade (10 conta como 1)
    let dice2 = [10, 7, 5, 2, 1];
    let (successes2, is_botch2) = evaluate_roll(&dice2, 6, false);
    // 10 = +1, 7 = +1, 1 = -1 -> Total 1
    assert_eq!(successes2, 1);
    assert!(!is_botch2);

    // 3. Falha Crítica / Botch: 0 sucessos e pelo menos um 1
    let dice_botch = [4, 3, 2, 1];
    let (successes_b, is_botch_b) = evaluate_roll(&dice_botch, 6, false);
    assert_eq!(successes_b, 0);
    assert!(is_botch_b);

    // 4. Falha Simples: 0 sucessos mas sem nenhum 1
    let dice_simple_fail = [5, 4, 3, 2];
    let (successes_sf, is_botch_sf) = evaluate_roll(&dice_simple_fail, 6, false);
    assert_eq!(successes_sf, 0);
    assert!(!is_botch_sf);
}

#[tokio::test]
async fn test_room_chat_database_persistence_and_order() {
    use sqlx::sqlite::SqlitePoolOptions;
    use mta_sheet::repositories::room_repository::RoomRepository;

    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Falha ao conectar no SQLite em memória");

    // Inicializa schema essencial
    sqlx::query("
        CREATE TABLE users (id TEXT PRIMARY KEY, username TEXT NOT NULL);
        CREATE TABLE rooms (id TEXT PRIMARY KEY, name TEXT NOT NULL, code TEXT UNIQUE NOT NULL, gm_id TEXT NOT NULL);
        CREATE TABLE room_messages (
            id TEXT PRIMARY KEY,
            room_id TEXT NOT NULL,
            sender_id TEXT NOT NULL,
            sender_name TEXT NOT NULL,
            sender_character TEXT,
            sender_avatar TEXT,
            content TEXT NOT NULL,
            message_type TEXT NOT NULL DEFAULT 'ooc',
            roll_data TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    ").execute(&pool).await.expect("Falha ao criar tabelas");

    let msg1 = RoomChatMessage {
        id: "msg1".to_string(),
        room_id: "room_test".to_string(),
        sender_id: "u1".to_string(),
        sender_name: "Jogador 1".to_string(),
        sender_character: None,
        sender_avatar: None,
        content: "Primeira mensagem".to_string(),
        message_type: "ooc".to_string(),
        roll_data: None,
        created_at: String::new(),
    };

    let msg2 = RoomChatMessage {
        id: "msg2".to_string(),
        room_id: "room_test".to_string(),
        sender_id: "u2".to_string(),
        sender_name: "Jogador 2".to_string(),
        sender_character: Some("Hermes".to_string()),
        sender_avatar: None,
        content: "Em cena: Eu entro na capela".to_string(),
        message_type: "ic".to_string(),
        roll_data: None,
        created_at: String::new(),
    };

    RoomRepository::save_room_message(&pool, &msg1).await.expect("Falha ao salvar msg1");
    RoomRepository::save_room_message(&pool, &msg2).await.expect("Falha ao salvar msg2");

    let history = RoomRepository::get_room_messages(&pool, "room_test", 50)
        .await
        .expect("Falha ao buscar mensagens");

    assert_eq!(history.len(), 2);
    assert_eq!(history[0].id, "msg1");
    assert_eq!(history[1].id, "msg2");
    assert_eq!(history[1].sender_character.as_deref(), Some("Hermes"));

    // Limpeza de histórico
    RoomRepository::clear_room_messages(&pool, "room_test")
        .await
        .expect("Falha ao limpar mensagens");

    let history_empty = RoomRepository::get_room_messages(&pool, "room_test", 50)
        .await
        .expect("Falha ao buscar mensagens após limpar");
    assert!(history_empty.is_empty());
}
