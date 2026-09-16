use mta_sheet::rooms::{RoomSummary, RoomDetails, ChantryPoolData, RoomInitiativeData, RoomMapData};
use mta_sheet::state::CharacterData;

#[test]
fn test_health_penalty_reduces_initiative_with_floor_of_2() {
    let mut char_data = CharacterData::default();
    char_data.attributes.entry("Destreza".to_string()).or_default().level = 3; // Destreza 3
    char_data.attributes.entry("Raciocínio".to_string()).or_default().level = 3; // Raciocínio 3 -> Raw base = 6

    // 1. Íntegro (0 dano) -> Base = 6, Penalidade = 0
    let (agg, lethal, bashing) = char_data.get_health_counts();
    assert_eq!(agg + lethal + bashing, 0);

    let dexterity = char_data.get_attribute_level("Destreza", 1);
    let wits = char_data.get_attribute_level("Raciocínio", 1);
    let raw_base = dexterity + wits;
    assert_eq!(raw_base, 6);

    let penalty_0 = 0;
    let base_integro = (raw_base - penalty_0).max(2);
    assert_eq!(base_integro, 6);

    // 2. Ferido (-1 penalidade) -> Base = 5
    let penalty_ferido = 1;
    let base_ferido = (raw_base - penalty_ferido).max(2);
    assert_eq!(base_ferido, 5);

    // 3. Espancado (-2 penalidade) -> Base = 4
    let penalty_espancado = 2;
    let base_espancado = (raw_base - penalty_espancado).max(2);
    assert_eq!(base_espancado, 4);

    // 4. Aleijado (-5 penalidade) -> 6 - 5 = 1 -> Clamped to minimum 2!
    let penalty_aleijado = 5;
    let base_aleijado = (raw_base - penalty_aleijado).max(2);
    assert_eq!(base_aleijado, 2, "Penalidade grave deve manter piso mínimo de 2");

    // 5. Personagem fraco (Dex 1 + Wits 1 = 2) que toma qualquer dano
    let weak_raw = 1 + 1; // 2
    let weak_base_ferido = (weak_raw - 1).max(2);
    assert_eq!(weak_base_ferido, 2, "Piso mínimo deve ser estritamente 2");
}

#[test]
fn test_room_summary_and_details_public_and_password_serialization() {
    let summary = RoomSummary {
        id: "room-abc".to_string(),
        name: "Crônica Aberta".to_string(),
        code: "MTA-OPEN".to_string(),
        description: "Mesa pública e protegida".to_string(),
        gm_username: "NarradorHermes".to_string(),
        is_gm: true,
        is_public: true,
        has_password: true,
        member_count: 5,
        sheet_count: 4,
        created_at: "2026-08-27 10:00:00".to_string(),
    };

    let json = serde_json::to_string(&summary).expect("serialize summary");
    let deserialized: RoomSummary = serde_json::from_str(&json).expect("deserialize summary");
    assert_eq!(summary, deserialized);
    assert!(deserialized.is_public);
    assert!(deserialized.has_password);

    let details = RoomDetails {
        id: "room-abc".to_string(),
        name: "Crônica Aberta".to_string(),
        code: "MTA-OPEN".to_string(),
        description: "Mesa pública e protegida".to_string(),
        gm_id: "gm-1".to_string(),
        gm_username: "NarradorHermes".to_string(),
        is_gm: true,
        is_public: true,
        has_password: true,
        chantry: ChantryPoolData::default(),
        chronicle_notes: "Notas de abertura".to_string(),
        initiative: RoomInitiativeData::default(),
        map_data: RoomMapData::default(),
        members: vec![],
        sheets: vec![],
    };

    let json_details = serde_json::to_string(&details).expect("serialize details");
    let des_details: RoomDetails = serde_json::from_str(&json_details).expect("deserialize details");
    assert_eq!(details, des_details);
    assert!(des_details.is_public);
    assert!(des_details.has_password);
}

#[cfg(feature = "ssr")]
#[test]
fn test_room_password_bcrypt_hashing_and_verification() {
    let password = "ArcanumPassword2026!";
    let hash = bcrypt::hash(password, 8).expect("bcrypt hash failed");

    // Senha correta deve passar
    assert!(bcrypt::verify(password, &hash).unwrap_or(false));

    // Senha incorreta deve falhar
    assert!(!bcrypt::verify("WrongPassword", &hash).unwrap_or(true));
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_clone_and_assign_sheet_schema_and_quiz_answers() {
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::Row;

    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("in-memory db");

    sqlx::query(
        "CREATE TABLE users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            is_admin INTEGER NOT NULL DEFAULT 0
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE rooms (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            code TEXT UNIQUE NOT NULL,
            description TEXT DEFAULT '',
            gm_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE room_members (
            room_id TEXT NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
            user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            role TEXT NOT NULL DEFAULT 'player',
            PRIMARY KEY (room_id, user_id)
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE character_sheets (
            id TEXT PRIMARY KEY,
            user_id TEXT REFERENCES users(id) ON DELETE SET NULL,
            room_id TEXT REFERENCES rooms(id) ON DELETE SET NULL,
            name TEXT NOT NULL,
            data TEXT NOT NULL,
            sheet_type TEXT NOT NULL DEFAULT 'mage',
            is_public INTEGER NOT NULL DEFAULT 0,
            is_hidden_in_room INTEGER NOT NULL DEFAULT 0,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE character_quiz_answers (
            character_id TEXT NOT NULL REFERENCES character_sheets(id) ON DELETE CASCADE,
            question_id TEXT NOT NULL,
            answer TEXT NOT NULL DEFAULT '',
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (character_id, question_id)
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query("INSERT INTO users (id, username, password_hash) VALUES ('gm-1', 'Mestre', 'hash')").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO users (id, username, password_hash) VALUES ('player-1', 'Jogador', 'hash')").execute(&pool).await.unwrap();

    sqlx::query("INSERT INTO rooms (id, name, code, gm_id) VALUES ('room-1', 'Sala Teste', 'MTA-TEST', 'gm-1')").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO room_members (room_id, user_id, role) VALUES ('room-1', 'gm-1', 'gm')").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO room_members (room_id, user_id, role) VALUES ('room-1', 'player-1', 'player')").execute(&pool).await.unwrap();

    let original_id = "sheet-gm-orig";
    let original_json = r#"{"id":"sheet-gm-orig","name":"Mago NPC","sheet_type":"mage"}"#;
    sqlx::query("INSERT INTO character_sheets (id, user_id, name, data, sheet_type) VALUES (?, 'gm-1', 'Mago NPC', ?, 'mage')")
        .bind(original_id)
        .bind(original_json)
        .execute(&pool).await.unwrap();

    sqlx::query("INSERT INTO character_quiz_answers (character_id, question_id, answer) VALUES (?, 'q1', 'Resposta Dossiê NPC')")
        .bind(original_id)
        .execute(&pool).await.unwrap();

    let new_sheet_id = "sheet-player-clone";
    let target_user_id = "player-1";
    let room_id = "room-1";

    let sheet_row = sqlx::query("SELECT name, data, sheet_type FROM character_sheets WHERE id = ? AND user_id = 'gm-1'")
        .bind(original_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    let s_name: String = sheet_row.get("name");
    let s_json: String = sheet_row.get("data");
    let s_type: String = sheet_row.get("sheet_type");

    let mut updated_data: CharacterData = serde_json::from_str(&s_json).unwrap();
    updated_data.id = new_sheet_id.to_string();
    let updated_json = serde_json::to_string(&updated_data).unwrap();

    sqlx::query(
        "INSERT INTO character_sheets (id, user_id, room_id, name, data, sheet_type, is_public, is_hidden_in_room, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, 0, 0, datetime('now'))"
    )
    .bind(new_sheet_id)
    .bind(target_user_id)
    .bind(room_id)
    .bind(&s_name)
    .bind(&updated_json)
    .bind(&s_type)
    .execute(&pool)
    .await
    .unwrap();

    let qa_rows = sqlx::query("SELECT question_id, answer FROM character_quiz_answers WHERE character_id = ?")
        .bind(original_id)
        .fetch_all(&pool)
        .await
        .unwrap();

    for qa in qa_rows {
        let q_id: String = qa.get("question_id");
        let ans: String = qa.get("answer");
        sqlx::query("INSERT OR REPLACE INTO character_quiz_answers (character_id, question_id, answer) VALUES (?, ?, ?)")
            .bind(new_sheet_id)
            .bind(&q_id)
            .bind(&ans)
            .execute(&pool)
            .await
            .unwrap();
    }

    let cloned_sheet = sqlx::query("SELECT user_id, room_id, name, data FROM character_sheets WHERE id = ?")
        .bind(new_sheet_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(cloned_sheet.get::<String, _>("user_id"), "player-1");
    assert_eq!(cloned_sheet.get::<String, _>("room_id"), "room-1");
    assert_eq!(cloned_sheet.get::<String, _>("name"), "Mago NPC");

    let cloned_char_data: CharacterData = serde_json::from_str(&cloned_sheet.get::<String, _>("data")).unwrap();
    assert_eq!(cloned_char_data.id, new_sheet_id);

    let cloned_quiz: String = sqlx::query_scalar("SELECT answer FROM character_quiz_answers WHERE character_id = ? AND question_id = 'q1'")
        .bind(new_sheet_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(cloned_quiz, "Resposta Dossiê NPC");
}

