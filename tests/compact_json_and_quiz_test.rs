use mta_sheet::state::{
    CharacterData, DotOrigin, MeritItem,
};

#[test]
fn test_compact_json_export_omits_empty_structures_and_unanswered_quiz() {
    let mut char_data = CharacterData::new("compact-test-1".to_string(), "Dante".to_string());
    char_data.labels.insert("Tradicao".to_string(), "Ordem de Hermes".to_string());
    if let Some(a) = char_data.attributes.get_mut("Força") {
        a.level = 3;
        a.dot_origins = vec![DotOrigin::Base, DotOrigin::Base, DotOrigin::Bonus];
    }
    
    // Responde apenas a 1 pergunta do quiz
    char_data.quiz_data.entries[0].answer = "Tenho 32 anos e despertei em Praga.".to_string();

    let json_str = serde_json::to_string_pretty(&char_data).expect("serialize compact");

    // 1. Deve conter os dados preenchidos
    assert!(json_str.contains("\"name\": \"Dante\""));
    assert!(json_str.contains("\"Tradicao\": \"Ordem de Hermes\""));
    assert!(json_str.contains("Tenho 32 anos e despertei em Praga."));

    // 2. NÃO deve conter campos vazios ou estruturas em branco
    assert!(!json_str.contains("\"gear_carried\""), "Possessions vazias devem ser omitidas");
    assert!(!json_str.contains("\"session_notes\""), "Notes vazias devem ser omitidas");
    assert!(!json_str.contains("\"history\""), "Histórico vazio deve ser omitido");
    assert!(!json_str.contains("\"apparent_age\""), "Descrição vazia deve ser omitida");
    assert!(!json_str.contains("\"merits\""), "Lista de méritos vazia deve ser omitida");
    assert!(!json_str.contains("\"flaws\""), "Lista de defeitos vazia deve ser omitida");
    assert!(!json_str.contains("\"weapons\""), "Lista de armas vazia deve ser omitida");

    // 3. NÃO deve conter enunciados das 13 perguntas não respondidas
    assert!(!json_str.contains("Como Era A Sua Família?"), "Perguntas sem resposta não devem poluir o JSON");
    assert!(!json_str.contains("O que o místiko vê no seu destino?"), "Prompts não respondidos devem ser omitidos");

    // 4. O tamanho do JSON deve ser ultra-compacto (< 1.8 KB em vez de 20 KB)
    assert!(json_str.len() < 1800, "JSON compacto deve ser menor que 1.8KB (atual: {} bytes)", json_str.len());
}

#[test]
fn test_compact_json_roundtrip_restores_all_defaults_and_answers() {
    let mut original = CharacterData::new("roundtrip-1".to_string(), "Ignatius".to_string());
    original.labels.insert("Essencia".to_string(), "Primordial".to_string());
    original.merits = vec![MeritItem {
        name: "Concentração".to_string(),
        merit_type: "Mental".to_string(),
        cost: 1,
    }];
    original.quiz_data.entries[5].answer = "Mestre Flambeau nos Alpes.".to_string(); // q_char_mentor

    let json_str = serde_json::to_string(&original).expect("serialize");
    let recovered: CharacterData = serde_json::from_str(&json_str).expect("deserialize");

    assert_eq!(recovered.name, "Ignatius");
    assert_eq!(recovered.labels.get("Essencia").unwrap(), "Primordial");
    assert_eq!(recovered.merits.len(), 1);
    assert_eq!(recovered.merits[0].name, "Concentração");
    
    // Garante que todas as 14 perguntas do quiz foram reconstruídas com prompts completos
    assert_eq!(recovered.quiz_data.entries.len(), 14);
    let mentor_q = recovered.quiz_data.entries.iter().find(|q| q.id == "q_char_mentor").expect("mentor question");
    assert_eq!(mentor_q.title, "Quem Era O Seu Mentor?");
    assert_eq!(mentor_q.answer, "Mestre Flambeau nos Alpes.");
    assert!(!mentor_q.prompt.is_empty(), "Prompt padrão deve ser restaurado");
}

#[test]
fn test_legacy_full_json_compatibility() {
    let legacy_json = r#"{
        "id": "legacy-full",
        "name": "Mago Veterano",
        "sheet_type": "mage",
        "is_public": false,
        "attributes": {
            "Força": { "level": 2, "modifier": "", "dot_origins": ["Base", "Base"] }
        },
        "labels": {
            "Tradicao": "Filhos do Éter"
        },
        "custom_lists": {},
        "merits": [],
        "flaws": [],
        "wonders": [],
        "rotes": "",
        "weapons": [],
        "armor": { "class_name": "", "rating": "", "penalty": "", "description": "" },
        "expanded_backgrounds": {
            "allies": "", "contacts": "", "fame": "", "influence": "", "library": "",
            "node": "", "resources": "", "retainers": "", "sanctum": "", "other_title": "", "other_text": ""
        },
        "possessions": { "gear_carried": "", "equipment_owned": "", "foci": "", "familiar": "", "grimoire": "" },
        "chantry": [],
        "history_data": { "history": "", "goals_destiny": "", "seekings": "", "quiets": "" },
        "description_data": { "age": "", "apparent_age": "", "date_of_birth": "", "age_of_awakening": "", "hair": "", "eyes": "", "race": "", "nationality": "", "height": "", "weight": "", "sex": "", "physical_description": "", "avatar_nature": "" },
        "visuals": { "cabal_chart_url": "", "character_sketch_url": "" },
        "grimoire": { "paradigm": "", "practices": [], "instruments": [], "rotes": [], "general_notes": "" },
        "notes_data": { "session_notes": "", "campaign_journal": "", "attachment_image_url": "" },
        "quiz_data": {
            "entries": [
                {
                    "id": "q_char_age",
                    "title": "Qual É A Sua Idade?",
                    "prompt": "Quantos anos...",
                    "answer": "Tenho 45 anos.",
                    "category": "character"
                }
            ]
        }
    }"#;

    let parsed: CharacterData = serde_json::from_str(legacy_json).expect("parse legacy full json");
    assert_eq!(parsed.name, "Mago Veterano");
    assert_eq!(parsed.labels.get("Tradicao").unwrap(), "Filhos do Éter");
    assert_eq!(parsed.quiz_data.entries[0].answer, "Tenho 45 anos.");
    assert_eq!(parsed.quiz_data.entries.len(), 14, "Deve expandir para todas as 14 perguntas");
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_sqlite_relational_quiz_tables_and_cascade() {
    use sqlx::sqlite::SqlitePoolOptions;

    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("in-memory db");

    sqlx::query(
        "CREATE TABLE users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE character_sheets (
            id TEXT PRIMARY KEY,
            user_id TEXT REFERENCES users(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            data TEXT NOT NULL,
            sheet_type TEXT NOT NULL DEFAULT 'mage'
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE quiz_questions (
            id TEXT PRIMARY KEY,
            splat TEXT NOT NULL DEFAULT 'mage',
            category TEXT NOT NULL DEFAULT 'character',
            title TEXT NOT NULL,
            prompt TEXT NOT NULL,
            sort_order INTEGER NOT NULL DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE character_quiz_answers (
            character_id TEXT NOT NULL REFERENCES character_sheets(id) ON DELETE CASCADE,
            question_id TEXT NOT NULL REFERENCES quiz_questions(id) ON DELETE CASCADE,
            answer TEXT NOT NULL DEFAULT '',
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (character_id, question_id)
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "INSERT INTO quiz_questions (id, splat, category, title, prompt, sort_order) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind("q_test_1")
    .bind("mage")
    .bind("character")
    .bind("Título Teste")
    .bind("Prompt Teste")
    .bind(1)
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query("INSERT INTO character_sheets (id, name, data) VALUES (?, ?, ?)")
        .bind("sheet_001")
        .bind("Dante")
        .bind("{}")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query(
        "INSERT INTO character_quiz_answers (character_id, question_id, answer) VALUES (?, ?, ?)"
    )
    .bind("sheet_001")
    .bind("q_test_1")
    .bind("Resposta do Dante")
    .execute(&pool)
    .await
    .unwrap();

    let ans: String = sqlx::query_scalar(
        "SELECT answer FROM character_quiz_answers WHERE character_id = ? AND question_id = ?"
    )
    .bind("sheet_001")
    .bind("q_test_1")
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(ans, "Resposta do Dante");

    sqlx::query("DELETE FROM character_sheets WHERE id = ?")
        .bind("sheet_001")
        .execute(&pool)
        .await
        .unwrap();

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM character_quiz_answers WHERE character_id = ?")
        .bind("sheet_001")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(count, 0, "Ao deletar o personagem, respostas no quiz devem ser deletadas em cascata");
}
