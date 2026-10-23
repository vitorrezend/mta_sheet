use mta_sheet::state::{CharacterData, DotOrigin};

#[test]
fn test_xp_ledger_entry_creation_and_award_calculation() {
    let mut sheet = CharacterData::new("test_sheet".to_string(), "Hermetic Mage".to_string());
    assert_eq!(sheet.total_xp_awarded(), 0);
    assert!(sheet.xp_ledger.is_empty());

    // 1. Adiciona entradas no histórico
    sheet.add_xp_entry("Sessão 1: Prelúdio & Despertar".to_string(), 4, Some("2026-10-01".to_string()));
    sheet.add_xp_entry("Sessão 2: Resgate da Biblioteca".to_string(), 3, Some("2026-10-08".to_string()));
    sheet.add_xp_entry("Interlúdio: Treinamento Místico".to_string(), 2, None);

    assert_eq!(sheet.xp_ledger.len(), 3);
    assert_eq!(sheet.total_xp_awarded(), 9);

    // 2. Remove uma entrada e valida recálculo
    let first_id = sheet.xp_ledger[0].id.clone();
    sheet.remove_xp_entry(&first_id);

    assert_eq!(sheet.xp_ledger.len(), 2);
    assert_eq!(sheet.total_xp_awarded(), 5);
}

#[test]
fn test_xp_ledger_balance_with_sheet_expenditures() {
    let mut sheet = CharacterData::new("balance_test".to_string(), "Verbena Witch".to_string());
    
    // Concede 20 XP
    sheet.add_xp_entry("Recompensa de Crônica".to_string(), 20, Some("2026-10-15".to_string()));
    assert_eq!(sheet.total_xp_awarded(), 20);

    // Compra 1 ponto de Forças como XP (custo 7 ou 8)
    sheet.set_attribute_with_origin("Forças", Some(1), None, DotOrigin::Experience);
    let costs = sheet.calculate_costs();
    assert!(costs.total_xp_spent > 0);

    let balance = sheet.total_xp_awarded() - costs.total_xp_spent;
    assert!(balance > 0 && balance < 20);
}

#[test]
fn test_xp_ledger_json_serialization_and_backward_compatibility() {
    // 1. Serialização com entradas
    let mut sheet = CharacterData::new("ser_test".to_string(), "Akashic Brother".to_string());
    sheet.add_xp_entry("Sessão 1".to_string(), 5, Some("2026-10-20".to_string()));

    let json = serde_json::to_string(&sheet).expect("Falha ao serializar");
    assert!(json.contains("xp_ledger"));
    assert!(json.contains("Sessão 1"));

    let deserialized: CharacterData = serde_json::from_str(&json).expect("Falha ao deserializar");
    assert_eq!(deserialized.xp_ledger.len(), 1);
    assert_eq!(deserialized.total_xp_awarded(), 5);

    // 2. Compatibilidade com JSON antigo sem campo xp_ledger
    let legacy_json = r#"{"id":"legacy","name":"Old Mage","sheet_type":"mage"}"#;
    let legacy_sheet: CharacterData = serde_json::from_str(legacy_json).expect("Falha ao deserializar json legado");
    assert!(legacy_sheet.xp_ledger.is_empty());
    assert_eq!(legacy_sheet.total_xp_awarded(), 0);

    // 3. Quando vazio, não deve poluir o JSON
    let empty_sheet = CharacterData::new("empty_test".to_string(), "Clean Mage".to_string());
    let empty_json = serde_json::to_string(&empty_sheet).expect("Falha ao serializar");
    assert!(!empty_json.contains("xp_ledger"));
}
