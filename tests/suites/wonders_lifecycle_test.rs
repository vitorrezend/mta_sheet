use mta_sheet::state::{CharacterData, WonderItem};

#[test]
fn test_new_character_starts_with_empty_wonders() {
    let sheet = CharacterData::new("sheet_w1".to_string(), "Mago Alquimista".to_string());
    assert_eq!(sheet.wonders.len(), 0, "Novos personagens devem iniciar com 0 maravilhas");
}

#[test]
fn test_add_and_remove_first_wonder_lifecycle() {
    let mut sheet = CharacterData::new("sheet_w2".to_string(), "Hermético".to_string());
    assert_eq!(sheet.wonders.len(), 0);

    // 1. Adiciona a primeira maravilha
    sheet.wonders.push(WonderItem {
        id: "wonder_1".to_string(),
        name: "Anel de Prata".to_string(),
        points: mta_sheet::state::AttributeValue::new(3, String::new()),
        arete: mta_sheet::state::AttributeValue::new(2, String::new()),
        quintessence_max: 5,
        quintessence_current: 3,
        description: "Um anel que canaliza forças.".to_string(),
        image_url: String::new(),
    });
    assert_eq!(sheet.wonders.len(), 1);

    // 2. Remove a primeira maravilha diretamente quando len == 1
    let idx_to_remove = 0;
    if idx_to_remove < sheet.wonders.len() {
        sheet.wonders.remove(idx_to_remove);
    }
    assert_eq!(sheet.wonders.len(), 0, "A primeira maravilha deve poder ser removida imediatamente, deixando o vetor vazio");
}

#[test]
fn test_multiple_wonders_removal_any_order() {
    let mut sheet = CharacterData::new("sheet_w3".to_string(), "Eutanatos".to_string());

    // Adiciona 3 maravilhas
    sheet.wonders.push(WonderItem {
        id: "w_1".to_string(),
        name: "Amuleto".to_string(),
        ..Default::default()
    });
    sheet.wonders.push(WonderItem {
        id: "w_2".to_string(),
        name: "Talismã".to_string(),
        ..Default::default()
    });
    sheet.wonders.push(WonderItem {
        id: "w_3".to_string(),
        name: "Grimório".to_string(),
        ..Default::default()
    });
    assert_eq!(sheet.wonders.len(), 3);

    // Remove a do meio (idx 1)
    sheet.wonders.remove(1);
    assert_eq!(sheet.wonders.len(), 2);
    assert_eq!(sheet.wonders[0].name, "Amuleto");
    assert_eq!(sheet.wonders[1].name, "Grimório");

    // Remove a primeira (idx 0)
    sheet.wonders.remove(0);
    assert_eq!(sheet.wonders.len(), 1);
    assert_eq!(sheet.wonders[0].name, "Grimório");

    // Remove a última restante
    sheet.wonders.remove(0);
    assert_eq!(sheet.wonders.len(), 0, "Todas as maravilhas foram removidas até 0");
}

#[test]
fn test_sanitization_preserves_exact_wonders_count() {
    // Caso 1: 0 maravilhas
    let mut sheet_empty = CharacterData::new("s_empty".to_string(), "Mago Vazio".to_string());
    sheet_empty.sanitize();
    assert_eq!(sheet_empty.wonders.len(), 0, "Sanitize não deve injetar maravilhas dummy vazias");

    // Caso 2: 1 maravilha válida
    let mut sheet_one = CharacterData::new("s_one".to_string(), "Mago Singular".to_string());
    sheet_one.wonders.push(WonderItem {
        id: "".to_string(), // id vazio para ser preenchido pelo sanitize
        name: "Adaga Cerimonial".to_string(),
        points: mta_sheet::state::AttributeValue::new(4, String::new()),
        arete: mta_sheet::state::AttributeValue::new(3, String::new()),
        quintessence_max: 7, // não é múltiplo de 5, sanitize deve arredondar para 10
        quintessence_current: 6,
        description: "Adaga sagrada".to_string(),
        image_url: String::new(),
    });
    sheet_one.sanitize();

    assert_eq!(sheet_one.wonders.len(), 1, "Sanitize deve manter exatamente 1 maravilha");
    assert!(!sheet_one.wonders[0].id.is_empty(), "Sanitize deve gerar UUID para id vazio");
    assert_eq!(sheet_one.wonders[0].quintessence_max, 10, "Sanitize deve arredondar quintessence_max");
    assert_eq!(sheet_one.wonders[0].points.dot_origins.len(), 4, "Sanitize deve inicializar dot_origins para o nível de pontos");
}

#[test]
fn test_roundtrip_serialization_without_phantom_wonders() {
    let mut sheet = CharacterData::new("s_json".to_string(), "Mago Serializado".to_string());
    sheet.wonders.push(WonderItem {
        id: "w_saved".to_string(),
        name: "Cajado do Éter".to_string(),
        points: mta_sheet::state::AttributeValue::new(2, String::new()),
        arete: mta_sheet::state::AttributeValue::new(1, String::new()),
        quintessence_max: 5,
        quintessence_current: 5,
        description: "Canalizador éterico".to_string(),
        image_url: String::new(),
    });

    let json = serde_json::to_string(&sheet).expect("Falha ao serializar");
    let mut restored: CharacterData = serde_json::from_str(&json).expect("Falha ao desserializar");
    restored.sanitize();

    assert_eq!(restored.wonders.len(), 1, "Não deve haver maravilhas fantasmas após serialize/deserialize/sanitize");
    assert_eq!(restored.wonders[0].name, "Cajado do Éter");
}
