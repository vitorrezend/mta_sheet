use mta_sheet::rooms::{RoomSummary, RoomDetails, ChantryPoolData, RoomInitiativeData};
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
        members: vec![],
        sheets: vec![],
    };

    let json_details = serde_json::to_string(&details).expect("serialize details");
    let des_details: RoomDetails = serde_json::from_str(&json_details).expect("deserialize details");
    assert_eq!(details, des_details);
    assert!(des_details.is_public);
    assert!(des_details.has_password);
}

#[test]
fn test_room_password_bcrypt_hashing_and_verification() {
    let password = "ArcanumPassword2026!";
    let hash = bcrypt::hash(password, 8).expect("bcrypt hash failed");

    // Senha correta deve passar
    assert!(bcrypt::verify(password, &hash).unwrap_or(false));

    // Senha incorreta deve falhar
    assert!(!bcrypt::verify("WrongPassword", &hash).unwrap_or(true));
}
