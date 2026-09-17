use mta_sheet::compendium::weapons::*;
use mta_sheet::i18n::Language;
use mta_sheet::state::WeaponItem;

#[test]
fn test_all_weapons_count_and_classes() {
    // Total de 82 armas canônicas de M20 (pp. 450-453)
    assert_eq!(ALL_WEAPONS.len(), 82, "O compêndio deve conter exatamente 82 armas (42 corpo a corpo + 34 à distância + 6 arremesso)");
    assert_eq!(ALL_MELEE_WEAPONS.len(), 42, "Devem existir 42 armas brancas");
    assert_eq!(ALL_RANGED_WEAPONS.len(), 34, "Devem existir 34 armas de fogo e à distância");
    assert_eq!(ALL_THROWN_WEAPONS.len(), 6, "Devem existir 6 armas de arremesso");

    // Grupos Principais
    assert_eq!(get_weapons_by_main_group(WeaponMainGroup::Melee).len(), 42, "Devem existir 42 armas no grupo Corpo a Corpo");
    assert_eq!(get_weapons_by_main_group(WeaponMainGroup::Ranged).len(), 40, "Devem existir 40 armas no grupo À Distância (incluindo arremesso)");

    // Classes
    assert_eq!(get_weapons_by_class(WeaponClass::Melee).len(), 42);
    assert_eq!(get_weapons_by_class(WeaponClass::Ranged).len(), 34);
    assert_eq!(get_weapons_by_class(WeaponClass::Thrown).len(), 6);

    // Categorias Corpo a Corpo (42)
    let axes = get_weapons_by_category(WeaponCategory::Axes);
    let knives = get_weapons_by_category(WeaponCategory::Knives);
    let swords = get_weapons_by_category(WeaponCategory::Swords);
    let clubbing = get_weapons_by_category(WeaponCategory::Clubbing);
    let fist = get_weapons_by_category(WeaponCategory::FistExtension);
    let improvised = get_weapons_by_category(WeaponCategory::Improvised);
    let whips = get_weapons_by_category(WeaponCategory::WhipsAndChains);

    assert_eq!(axes.len(), 5, "Machados deve ter 5 armas");
    assert_eq!(knives.len(), 3, "Facas & Adagas deve ter 3 armas");
    assert_eq!(swords.len(), 5, "Espadas deve ter 5 armas");
    assert_eq!(clubbing.len(), 9, "Armas Contundentes deve ter 9 armas");
    assert_eq!(fist.len(), 8, "Extensões de Punho deve ter 8 armas");
    assert_eq!(improvised.len(), 4, "Armas Improvisadas deve ter 4 armas");
    assert_eq!(whips.len(), 8, "Chicotes e Correntes deve ter 8 armas");

    // Categorias À Distância & Arremesso (40)
    let pistols = get_weapons_by_category(WeaponCategory::Pistols);
    let rifles_smgs = get_weapons_by_category(WeaponCategory::RiflesAndSmgs);
    let shotguns = get_weapons_by_category(WeaponCategory::Shotguns);
    let bows = get_weapons_by_category(WeaponCategory::Bows);
    let military = get_weapons_by_category(WeaponCategory::HeavyAndMilitary);
    let techno = get_weapons_by_category(WeaponCategory::TechnocracySidearms);
    let non_lethal = get_weapons_by_category(WeaponCategory::NonLethal);
    let thrown = get_weapons_by_category(WeaponCategory::ThrownWeapons);

    assert_eq!(pistols.len(), 4, "Pistolas & Revólveres deve ter 4 armas");
    assert_eq!(rifles_smgs.len(), 4, "Fuzis & Submetralhadoras deve ter 4 armas");
    assert_eq!(shotguns.len(), 4, "Espingardas deve ter 4 armas");
    assert_eq!(bows.len(), 6, "Arcos e Bestas deve ter 6 armas");
    assert_eq!(military.len(), 7, "Lança-Foguetes & Armas Pesadas deve ter 7 armas");
    assert_eq!(techno.len(), 6, "Armas da Tecnocracia deve ter 6 armas");
    assert_eq!(non_lethal.len(), 3, "Armas Não-Letais deve ter 3 armas");
    assert_eq!(thrown.len(), 6, "Armas de Arremesso deve ter 6 armas");
}

#[test]
fn test_specific_melee_weapons_stats() {
    // Katana
    let katana = find_weapon("Katana").expect("Katana deve ser encontrada");
    assert_eq!(katana.difficulty, 6);
    assert_eq!(katana.damage, "Strength +3/L");
    assert_eq!(katana.damage_pt, "Força +3/L");
    assert_eq!(katana.conceal, "T");
    assert_eq!(katana.notes, &["#4"]);
    assert_eq!(katana.range, "—");
    assert_eq!(katana.rate, "—");
    assert_eq!(katana.clip, "—");

    // Great Axe (#1)
    let great_axe = find_weapon("Great Axe").expect("Great Axe deve ser encontrada");
    assert_eq!(great_axe.difficulty, 7);
    assert_eq!(great_axe.damage, "Strength +6/L");
    assert_eq!(great_axe.conceal, "N");
    assert_eq!(great_axe.notes, &["#1"]);

    // Polearm (#2)
    let polearm = find_weapon("Arma de Haste").expect("Arma de Haste deve ser encontrada");
    assert_eq!(polearm.difficulty, 7);
    assert_eq!(polearm.notes, &["#2"]);

    // Stiletto (#3)
    let stiletto = find_weapon("Estilete").expect("Estilete deve ser encontrado");
    assert_eq!(stiletto.difficulty, 4);
    assert_eq!(stiletto.conceal, "P");
    assert_eq!(stiletto.notes, &["#3"]);

    // Sai (#5, #6)
    let sai = find_weapon("Sai").expect("Sai deve ser encontrado");
    assert_eq!(sai.difficulty, 5);
    assert_eq!(sai.notes, &["#5", "#6"]);

    // Hook Sword(s) (Used as pair, #2, #5, #6)
    let hook_swords = find_weapon("Espadas Gancho").expect("Espadas Gancho deve ser encontrada");
    assert_eq!(hook_swords.difficulty, 7);
    assert_eq!(hook_swords.notes, &["Used as pair", "#2", "#5", "#6"]);

    // Chainsaw (#9)
    let chainsaw = find_weapon("Motosserra").expect("Motosserra deve ser encontrada");
    assert_eq!(chainsaw.difficulty, 8);
    assert_eq!(chainsaw.damage, "Strength +7/L");
    assert_eq!(chainsaw.notes, &["#9"]);

    // Kusarigama (#2, #6, #10)
    let kusari = find_weapon("Kusarigama").expect("Kusarigama deve ser encontrada");
    assert_eq!(kusari.difficulty, 7);
    assert_eq!(kusari.notes, &["#2", "#6", "#10"]);
}

#[test]
fn test_specific_ranged_weapons_stats() {
    // Light Pistol (Glock 17)
    let light_pistol = find_weapon("Glock 17").expect("Glock 17 deve ser encontrada");
    assert_eq!(light_pistol.difficulty, 6);
    assert_eq!(light_pistol.damage, "4/L");
    assert_eq!(light_pistol.range, "20");
    assert_eq!(light_pistol.rate, "4");
    assert_eq!(light_pistol.clip, "17+1");
    assert_eq!(light_pistol.conceal, "P");

    // Heavy Pistol (Desert Eagle)
    let deagle = find_weapon("Desert Eagle").expect("Desert Eagle deve ser encontrada");
    assert_eq!(deagle.difficulty, 6);
    assert_eq!(deagle.damage, "5/L");
    assert_eq!(deagle.range, "30");
    assert_eq!(deagle.rate, "3");
    assert_eq!(deagle.clip, "7+1");
    assert_eq!(deagle.conceal, "J");

    // Assault Rifle (M-16 / AK-47) - Note #1
    let assault_rifle = find_weapon("Assault Rifle").expect("Assault Rifle deve ser encontrada");
    assert_eq!(assault_rifle.difficulty, 6);
    assert_eq!(assault_rifle.damage, "7/L");
    assert_eq!(assault_rifle.range, "150");
    assert_eq!(assault_rifle.rate, "3");
    assert_eq!(assault_rifle.clip, "42+1");
    assert_eq!(assault_rifle.conceal, "N");
    assert_eq!(assault_rifle.notes, &["#1"]);

    // Shotgun (Pump)
    let shotgun = find_weapon("Pump Shotgun").expect("Pump Shotgun deve ser encontrada");
    assert_eq!(shotgun.difficulty, 6);
    assert_eq!(shotgun.damage, "8/L");
    assert_eq!(shotgun.range, "20");
    assert_eq!(shotgun.rate, "1");
    assert_eq!(shotgun.clip, "5+1");
    assert_eq!(shotgun.conceal, "T");

    // Technocracy Biggs X-5 Model R - Notes #2, #3
    let biggs_r = find_weapon("Biggs X-5 Model R").expect("Biggs X-5 Model R deve ser encontrada");
    assert_eq!(biggs_r.difficulty, 6);
    assert_eq!(biggs_r.damage, "5/L");
    assert_eq!(biggs_r.notes, &["#2", "#3"]);

    // Technocracy Mjollner Mk IV - Notes #2, #4
    let mjollner = find_weapon("Biggs Mjollner Mk. IV").expect("Mjollner deve ser encontrada");
    assert_eq!(mjollner.difficulty, 6);
    assert_eq!(mjollner.damage, "10/L");
    assert_eq!(mjollner.notes, &["#2", "#4"]);

    // Compound Bow
    let comp_bow = find_weapon("Arco Composto").expect("Arco Composto deve ser encontrado");
    assert_eq!(comp_bow.difficulty, 6);
    assert_eq!(comp_bow.damage, "5/L");
    assert_eq!(comp_bow.range, "100");
    assert_eq!(comp_bow.rate, "1");
    assert_eq!(comp_bow.clip, "1");
    assert_eq!(comp_bow.notes, &["#6"]);

    // Taser - Notes #8
    let taser = find_weapon("Taser").expect("Taser deve ser encontrado");
    assert_eq!(taser.difficulty, 6);
    assert_eq!(taser.damage, "5/B");
    assert_eq!(taser.notes, &["#8"]);

    // LAW Rocket - Notes #9, #12
    let law = find_weapon("Rocket Launcher").expect("LAW Rocket deve ser encontrado");
    assert_eq!(law.difficulty, 6);
    assert_eq!(law.damage, "12-16/L (#12)");
    assert_eq!(law.notes, &["#9", "#12"]);
}

#[test]
fn test_combat_notes_melee_and_ranged() {
    // 10 notas para armas brancas
    assert_eq!(ALL_RULE_NOTES.len(), 10, "Devem existir 10 notas de armas brancas (#1 a #10)");

    for i in 1..=10 {
        let code = format!("#{}", i);
        let expl_pt = explain_weapon_note(&code, Language::PtBr, WeaponClass::Melee);
        assert!(expl_pt.is_some(), "Nota Melee {} deve ter explicação em PT", code);
        let (title_pt, desc_pt) = expl_pt.unwrap();
        assert!(!title_pt.is_empty());
        assert!(!desc_pt.is_empty());
    }

    // 12 notas para armas à distância
    for i in 1..=12 {
        let code = format!("#{}", i);
        let expl_pt = explain_weapon_note(&code, Language::PtBr, WeaponClass::Ranged);
        assert!(expl_pt.is_some(), "Nota Ranged {} deve ter explicação em PT", code);
        let (title_pt, desc_pt) = expl_pt.unwrap();
        assert!(!title_pt.is_empty());
        assert!(!desc_pt.is_empty());

        let expl_en = explain_weapon_note(&code, Language::EnUs, WeaponClass::Ranged);
        assert!(expl_en.is_some(), "Nota Ranged {} deve ter explicação em EN", code);
    }
}

#[test]
fn test_bilingual_lookups_and_aliases() {
    // Busca em PT
    assert_eq!(find_weapon("Machadinha").unwrap().id, "hatchet");
    assert_eq!(find_weapon("Faca").unwrap().id, "knife");
    assert_eq!(find_weapon("Taco de Beisebol").unwrap().id, "baseball_bat");
    assert_eq!(find_weapon("Soco-Inglês").unwrap().id, "brass_knuckles");
    assert_eq!(find_weapon("Cadeira").unwrap().id, "chair");
    assert_eq!(find_weapon("Chicote de Couro").unwrap().id, "bullwhip");
    assert_eq!(find_weapon("Pistola Leve").unwrap().id, "pistol_lt");
    assert_eq!(find_weapon("Pistola Pesada").unwrap().id, "pistol_hvy");
    assert_eq!(find_weapon("Arco Longo").unwrap().id, "long_bow");

    // Busca em EN
    assert_eq!(find_weapon("Hatchet").unwrap().id, "hatchet");
    assert_eq!(find_weapon("Knife").unwrap().id, "knife");
    assert_eq!(find_weapon("Light Pistol").unwrap().id, "pistol_lt");
    assert_eq!(find_weapon("Heavy Pistol").unwrap().id, "pistol_hvy");
    assert_eq!(find_weapon("Assault Rifle").unwrap().id, "assault_rifle");

    // Aliases
    assert_eq!(find_weapon("Glock").unwrap().id, "pistol_lt");
    assert_eq!(find_weapon("Desert Eagle").unwrap().id, "pistol_hvy");
    assert_eq!(find_weapon("M-16").unwrap().id, "assault_rifle");
    assert_eq!(find_weapon("AK-47").unwrap().id, "assault_rifle");
    assert_eq!(find_weapon("Uzi").unwrap().id, "smg_small");
    assert_eq!(find_weapon("MP5").unwrap().id, "smg_large");
    assert_eq!(find_weapon("Remington").unwrap().id, "rifle_hunting");
    assert_eq!(find_weapon("Shuriken").unwrap().id, "shuriken");
}

#[test]
fn test_weapon_item_serde_with_full_combat_stats() {
    let item = WeaponItem {
        name: "Glock 17".to_string(),
        diff: "6".to_string(),
        damage: "4/L".to_string(),
        range: "20".to_string(),
        rate: "4".to_string(),
        clip: "17+1".to_string(),
        conceal: "P".to_string(),
        notes: "".to_string(),
    };

    let json = serde_json::to_string(&item).expect("Serialização deve suceder");
    assert!(json.contains("\"range\":\"20\""));
    assert!(json.contains("\"rate\":\"4\""));
    assert!(json.contains("\"clip\":\"17+1\""));

    let deserialized: WeaponItem = serde_json::from_str(&json).expect("Desserialização deve suceder");
    assert_eq!(deserialized.name, "Glock 17");
    assert_eq!(deserialized.range, "20");
    assert_eq!(deserialized.rate, "4");
    assert_eq!(deserialized.clip, "17+1");

    // Retrocompatibilidade: JSON legado sem 'range', 'rate', 'clip'
    let legacy_json = r#"{"name":"Faca","diff":"4","damage":"Força +1/L","conceal":"P"}"#;
    let legacy_item: WeaponItem = serde_json::from_str(legacy_json).expect("JSON legado deve desserializar");
    assert_eq!(legacy_item.name, "Faca");
    assert_eq!(legacy_item.range, "");
    assert_eq!(legacy_item.rate, "");
    assert_eq!(legacy_item.clip, "");
}
