use mta_sheet::state::{CharacterData, default_quiz_questions};

#[test]
fn test_default_quiz_questions_length_and_structure() {
    let questions = default_quiz_questions();
    assert_eq!(questions.len(), 14);
    
    let char_questions: Vec<_> = questions.iter().filter(|q| q.category == "character").collect();
    let player_questions: Vec<_> = questions.iter().filter(|q| q.category == "player").collect();
    
    assert_eq!(char_questions.len(), 8, "Devem existir 8 perguntas para o Personagem");
    assert_eq!(player_questions.len(), 6, "Devem existir 6 perguntas para o Jogador");
    
    assert!(char_questions[0].title.contains("Qual É A Sua Idade?"));
    assert!(char_questions[1].title.contains("Quando Você Percebeu Que Era... Diferente?"));
    assert!(char_questions[7].title.contains("Você Mantém Uma Vida Comum?"));

    assert!(player_questions[0].title.contains("O Que Você, o Jogador, Quer Fazer?"));
    assert!(player_questions[5].title.contains("Que Conflitos Podem Surgir ao Longo do Caminho?"));
}

#[test]
fn test_character_data_new_includes_quiz_data() {
    let char_data = CharacterData::new("test-1".to_string(), "Hermes".to_string());
    assert_eq!(char_data.quiz_data.entries.len(), 14);
    assert_eq!(char_data.quiz_data.entries[0].answer, "");
}

#[test]
fn test_quiz_data_serialization_and_answer_preservation() {
    let mut char_data = CharacterData::new("test-2".to_string(), "Voormas".to_string());
    char_data.quiz_data.entries[0].answer = "Tenho 28 anos, estudo a Ordem de Hermes desde os 19.".to_string();
    char_data.quiz_data.entries[8].answer = "Quero explorar conspirações de alta magia e intriga política.".to_string();

    let json = serde_json::to_string(&char_data).expect("serialize");
    assert!(json.contains("Tenho 28 anos"));
    assert!(json.contains("conspirações de alta magia"));

    let recovered: CharacterData = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(recovered.quiz_data.entries[0].answer, "Tenho 28 anos, estudo a Ordem de Hermes desde os 19.");
    assert_eq!(recovered.quiz_data.entries[8].answer, "Quero explorar conspirações de alta magia e intriga política.");
    assert_eq!(recovered.quiz_data.entries.len(), 14);
}

#[test]
fn test_legacy_json_without_quiz_data_populates_defaults() {
    let legacy_json = r#"{
        "id": "legacy-1",
        "name": "Mago Antigo",
        "sheet_type": "mage",
        "attributes": {}
    }"#;

    let parsed = CharacterData::from_raw_json_resilient("legacy-1", legacy_json).expect("parse legacy");
    assert_eq!(parsed.quiz_data.entries.len(), 14);
    assert_eq!(parsed.name, "Mago Antigo");
}

#[test]
fn test_quiz_modal_first_render_immediate_entry_resolution() {
    // Simula a lógica síncrona de inicialização e abertura do QuizModal
    let mut char_data = CharacterData::new("test-first-open".to_string(), "Novo Personagem".to_string());
    
    // 1. Caso com dados normais
    let initial_entries = if char_data.quiz_data.entries.is_empty() {
        default_quiz_questions()
    } else {
        char_data.quiz_data.entries.clone()
    };
    assert_eq!(initial_entries.len(), 14, "Primeiro render deve conter 14 perguntas imediatamente");

    // 2. Caso extremo onde quiz_data.entries foi esvaziado manualmente
    char_data.quiz_data.entries.clear();
    assert!(char_data.quiz_data.entries.is_empty());

    let resolved_on_first_open = if char_data.quiz_data.entries.is_empty() {
        default_quiz_questions()
    } else {
        char_data.quiz_data.entries.clone()
    };
    assert_eq!(resolved_on_first_open.len(), 14, "Mesmo com lista vazia, a resolução síncrona no primeiro clique deve carregar 14 perguntas");
}
