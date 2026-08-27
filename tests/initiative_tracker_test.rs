#[derive(Clone, Debug, PartialEq)]
pub struct InitiativeEntryTest {
    pub id: String,
    pub name: String,
    pub is_npc: bool,
    pub is_active: bool,
    pub base_dex: i32,
    pub base_wits: i32,
    pub base_total: i32,
    pub rolled_die: Option<i32>,
    pub final_total: Option<i32>,
}

impl InitiativeEntryTest {
    pub fn new_character(id: &str, name: &str, dex: i32, wits: i32) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            is_npc: false,
            is_active: true,
            base_dex: dex,
            base_wits: wits,
            base_total: dex + wits,
            rolled_die: None,
            final_total: None,
        }
    }

    pub fn new_npc(id: &str, name: &str, base: i32) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            is_npc: true,
            is_active: true,
            base_dex: 0,
            base_wits: 0,
            base_total: base,
            rolled_die: None,
            final_total: None,
        }
    }
}

pub fn sort_initiative_entries(list: &mut [InitiativeEntryTest]) {
    list.sort_by(|a, b| {
        match (a.final_total, b.final_total) {
            (Some(fa), Some(fb)) => {
                fb.cmp(&fa)
                    .then_with(|| b.base_total.cmp(&a.base_total))
                    .then_with(|| b.base_dex.cmp(&a.base_dex))
            }
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => b.base_total.cmp(&a.base_total),
        }
    });
}

#[test]
fn test_wod_initiative_formula_and_sorting() {
    let mut list = vec![
        InitiativeEntryTest::new_character("c1", "Dante", 3, 4), // Base 7
        InitiativeEntryTest::new_character("c2", "Ignatius", 2, 3), // Base 5
        InitiativeEntryTest::new_npc("n1", "Tecnocrata MIB", 6), // Base 6
        InitiativeEntryTest::new_npc("n2", "Guarda Costas", 5), // Base 5
    ];

    // Simula rolagem com dados específicos
    list[0].rolled_die = Some(6); // Dante: 7 + 6 = 13
    list[0].final_total = Some(13);

    list[1].rolled_die = Some(10); // Ignatius: 5 + 10 = 15
    list[1].final_total = Some(15);

    list[2].rolled_die = Some(7); // Tecnocrata MIB: 6 + 7 = 13
    list[2].final_total = Some(13);

    list[3].rolled_die = Some(2); // Guarda Costas: 5 + 2 = 7
    list[3].final_total = Some(7);

    sort_initiative_entries(&mut list);

    // Ordem esperada:
    // 1º: Ignatius (15)
    // 2º: Dante (13 - Desempate por Base: Dante tem Base 7 vs Tecnocrata Base 6)
    // 3º: Tecnocrata MIB (13 - Base 6)
    // 4º: Guarda Costas (7)
    assert_eq!(list[0].name, "Ignatius");
    assert_eq!(list[0].final_total, Some(15));

    assert_eq!(list[1].name, "Dante");
    assert_eq!(list[1].final_total, Some(13));

    assert_eq!(list[2].name, "Tecnocrata MIB");
    assert_eq!(list[2].final_total, Some(13));

    assert_eq!(list[3].name, "Guarda Costas");
    assert_eq!(list[3].final_total, Some(7));
}

#[test]
fn test_inactive_participants_placed_at_end() {
    let mut list = vec![
        InitiativeEntryTest::new_character("c1", "Ativo 1", 3, 3), // Base 6
        InitiativeEntryTest::new_character("c2", "Inativo", 4, 4), // Base 8 (desmarcado)
        InitiativeEntryTest::new_character("c3", "Ativo 2", 2, 2), // Base 4
    ];

    list[0].rolled_die = Some(4);
    list[0].final_total = Some(10);

    list[1].is_active = false;
    list[1].rolled_die = None;
    list[1].final_total = None;

    list[2].rolled_die = Some(5);
    list[2].final_total = Some(9);

    sort_initiative_entries(&mut list);

    assert_eq!(list[0].name, "Ativo 1");
    assert_eq!(list[1].name, "Ativo 2");
    assert_eq!(list[2].name, "Inativo");
    assert_eq!(list[2].final_total, None);
}

#[test]
fn test_room_broadcast_event_serialization() {
    use mta_sheet::rooms::{RoomBroadcastEvent, RoomInitiativeData, InitiativeEntry};

    let event = RoomBroadcastEvent {
        event_type: "DICE_ROLLED".to_string(),
        initiative: RoomInitiativeData {
            round: 2,
            is_open: true,
            entries: vec![
                InitiativeEntry {
                    id: "c1".to_string(),
                    name: "Hermes".to_string(),
                    is_npc: false,
                    is_active: true,
                    base_dex: 3,
                    base_wits: 3,
                    base_total: 6,
                    health_penalty: 0,
                    rolled_die: Some(8),
                    final_total: Some(14),
                },
            ],
        },
        play_sound: true,
    };

    let json = serde_json::to_string(&event).expect("serialize RoomBroadcastEvent");
    let deserialized: RoomBroadcastEvent = serde_json::from_str(&json).expect("deserialize RoomBroadcastEvent");
    assert_eq!(event, deserialized);
    assert!(deserialized.play_sound);
    assert_eq!(deserialized.initiative.round, 2);
}

