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


#[test]
fn test_combat_maneuvers_count_and_categories() {
    // Total de 44 manobras canônicas de M20 (pp. 423-426, 448-450, 580-581)
    assert_eq!(ALL_COMBAT_MANEUVERS.len(), 44, "O compêndio deve conter 44 manobras canônicas (10 gerais + 8 luta suja + 16 artes marciais + 9 dô + 1 regra especial)");

    let general = get_maneuvers_by_category(Some(ManeuverCategory::General));
    let dirty = get_maneuvers_by_category(Some(ManeuverCategory::DirtyFighting));
    let martial = get_maneuvers_by_category(Some(ManeuverCategory::MartialArts));
    let do_maneuvers = get_maneuvers_by_category(Some(ManeuverCategory::Do));
    let special = get_maneuvers_by_category(Some(ManeuverCategory::SpecialRules));

    assert_eq!(general.len(), 10, "Devem existir 10 manobras gerais de corpo a corpo");
    assert_eq!(dirty.len(), 8, "Devem existir 8 manobras de luta suja (Briga 3+)");
    assert_eq!(martial.len(), 16, "Devem existir 16 manobras de artes marciais canônicas de M20 (pp. 423-426)");
    assert_eq!(do_maneuvers.len(), 9, "Devem existir 9 técnicas especiais de Dô akashiano");
    assert_eq!(special.len(), 1, "Deve existir 1 regra especial (Two Weapons)");

    assert_eq!(ALL_MANEUVER_CATEGORIES.len(), 5, "Devem existir 5 categorias de manobras de combate");
}

#[test]
fn test_specific_combat_maneuvers_stats() {
    // 1. Mordida / Bite (Geral)
    let bite = find_maneuver("Bite").expect("Bite deve ser encontrada");
    assert_eq!(bite.id, "bite");
    assert_eq!(bite.category, ManeuverCategory::General);
    assert_eq!(bite.difficulty, "5");
    assert_eq!(bite.actions, 1);
    assert_eq!(bite.damage, "Strength + 1 / B or L");

    // 2. Golpe Baixo / Low Blow (Luta Suja)
    let low_blow = find_maneuver("Low Blow").expect("Low Blow deve ser encontrada");
    assert_eq!(low_blow.id, "low_blow");
    assert_eq!(low_blow.category, ManeuverCategory::DirtyFighting);
    assert_eq!(low_blow.difficulty, "7");
    assert_eq!(low_blow.damage, "Strength + Stun / B or L");

    // 3. Chute Giratório / Spinning Kick (Artes Marciais)
    let spinning = find_maneuver("Spinning Kick").expect("Spinning Kick deve ser encontrada");
    assert_eq!(spinning.id, "spinning_kick");
    assert_eq!(spinning.category, ManeuverCategory::MartialArts);
    assert_eq!(spinning.difficulty, "6");
    assert_eq!(spinning.damage, "Strength + 3 / B");

    // 4. Golpe Mortal / Death Strike (Artes Marciais)
    let death = find_maneuver("Death Strike").expect("Death Strike deve ser encontrada");
    assert_eq!(death.id, "death_strike");
    assert_eq!(death.category, ManeuverCategory::MartialArts);
    assert_eq!(death.difficulty, "5");
    assert_eq!(death.damage, "Strength + 2 / L");

    // 5. Rasteira Cauda de Dragão / Dragon Tail Sweep (Artes Marciais)
    let dragon = find_maneuver("Dragon Tail Sweep").expect("Dragon Tail Sweep deve ser encontrada");
    assert_eq!(dragon.id, "dragon_tail_sweep");
    assert_eq!(dragon.category, ManeuverCategory::MartialArts);
    assert_eq!(dragon.difficulty, "8");
    assert_eq!(dragon.damage, "Opponent’s Strength / B");
}

#[test]
fn test_combat_maneuvers_bilingual_lookup() {
    // Busca em PT
    assert_eq!(find_maneuver("Mordida").unwrap().id, "bite");
    assert_eq!(find_maneuver("Chute Giratório").unwrap().id, "spinning_kick");
    assert_eq!(find_maneuver("Golpe Mortal").unwrap().id, "death_strike");
    assert_eq!(find_maneuver("Golpe Vital").unwrap().id, "vital_strike");
    assert_eq!(find_maneuver("Rasteira Cauda de Dragão").unwrap().id, "dragon_tail_sweep");

    // Busca em EN
    assert_eq!(find_maneuver("Bite").unwrap().id, "bite");
    assert_eq!(find_maneuver("Spinning Kick").unwrap().id, "spinning_kick");
    assert_eq!(find_maneuver("Death Strike").unwrap().id, "death_strike");
    assert_eq!(find_maneuver("Vital Strike").unwrap().id, "vital_strike");
    assert_eq!(find_maneuver("Dragon Tail Sweep").unwrap().id, "dragon_tail_sweep");

    // Aliases e apelidos de manobras
    assert_eq!(find_maneuver("Chute Trovão").unwrap().id, "thunder_kick");
    assert_eq!(find_maneuver("Soco Trovão").unwrap().id, "punch");
    assert_eq!(find_maneuver("Thunder Punch").unwrap().id, "punch");
    assert_eq!(find_maneuver("Roundhouse").unwrap().id, "spinning_kick");
    assert_eq!(find_maneuver("Chute Circular").unwrap().id, "spinning_kick");
    assert_eq!(find_maneuver("Strike Vital Point").unwrap().id, "vital_strike");
    assert_eq!(find_maneuver("Nerve Strike").unwrap().id, "nerve_strike");
    assert_eq!(find_maneuver("Ponto de Pressão").unwrap().id, "nerve_strike");
}

#[test]
fn test_thunder_punch_mage_trick_canonical_box() {
    let trick = &THUNDER_PUNCH_TRICK;
    assert_eq!(trick.id, "thunder_punch");
    assert_eq!(trick.page_ref, "M20, p. 449");
    assert_eq!(trick.title(Language::PtBr), "Truque de Mago: O Golpe Trovão");
    assert_eq!(trick.title(Language::EnUs), "Mage Trick: The Thunder Punch");

    assert_eq!(trick.spheres_summary(Language::EnUs), "Correspondence 1, Entropy 1-2, Forces 2, Life 3, Mind 2, Matter 2, Prime 2-3, or Time 2");
    assert_eq!(trick.spheres_summary(Language::PtBr), "Correspondência 1, Entropia 1-2, Forças 2, Vida 3, Mente 2, Matéria 2, Primórdio 2-3 ou Tempo 2");

    assert_eq!(trick.difficulty_rule(Language::EnUs), "Each success reduces attack difficulty by -1 (maximum adjustment: -3)");
    assert_eq!(trick.difficulty_rule(Language::PtBr), "Cada sucesso reduz a dificuldade do ataque em -1 (ajuste máximo: -3)");

    assert_eq!(trick.damage_rule(Language::EnUs), "Coincidental: Bashing damage. Life 3 or Prime 3: Aggravated damage (Pattern assault). Vulgar if visibly disproportionate to physique.");
    assert_eq!(trick.damage_rule(Language::PtBr), "Coincidente: Dano Contundente. Vida 3 ou Primórdio 3: Dano Agravado (ataque ao Padrão). Vulgar se visivelmente desproporcional à compleição.");

    assert_eq!(trick.backlash_rule(Language::EnUs), "If target soaks ALL damage, attacker takes the full intended damage (Bashing; Lethal if striking walls/steel armor).");
    assert_eq!(trick.backlash_rule(Language::PtBr), "Se o alvo absorver TODO o dano, o próprio mago sofre o dano pretendido (Contundente; Letal contra paredes/armadura de aço).");

    assert_eq!(trick.paragraphs(Language::EnUs).len(), 3);
    assert_eq!(trick.paragraphs(Language::PtBr).len(), 3);
    assert_eq!(trick.sphere_tags(Language::EnUs).len(), 8);
    assert_eq!(trick.sphere_tags(Language::PtBr).len(), 8);

    // Verificação de termos chave nos parágrafos canônicos
    assert!(trick.paragraphs(Language::EnUs)[0].contains("Correspondence 1"));
    assert!(trick.paragraphs(Language::EnUs)[0].contains("maximum adjustment of -3"));
    assert!(trick.paragraphs(Language::EnUs)[1].contains("Life 3 or Prime 3 Pattern assault"));
    assert!(trick.paragraphs(Language::EnUs)[2].contains("If the target manages to soak every level of damage"));

    assert!(trick.paragraphs(Language::PtBr)[0].contains("Correspondência 1"));
    assert!(trick.paragraphs(Language::PtBr)[0].contains("ajuste máximo de -3"));
    assert!(trick.paragraphs(Language::PtBr)[1].contains("Vida 3 ou Primórdio 3"));
    assert!(trick.paragraphs(Language::PtBr)[2].contains("Se o alvo conseguir absorver cada nível de dano"));
}

#[test]
fn test_do_special_techniques_stats_and_lookups() {
    // 1. Arrow Cutting / Aparar Flechas
    let arrow = find_maneuver("Arrow Cutting").expect("Arrow Cutting deve ser encontrada");
    assert_eq!(arrow.id, "arrow_cutting");
    assert_eq!(arrow.category, ManeuverCategory::Do);
    assert_eq!(arrow.difficulty, "7 (deflect) / 9 (catch & throw)");
    assert_eq!(arrow.damage, "As Weapon");

    // 2. Hurricane Throw / Arremesso Furacão
    let hurricane = find_maneuver("Hurricane Throw").expect("Hurricane Throw deve ser encontrada");
    assert_eq!(hurricane.id, "hurricane_throw");
    assert_eq!(hurricane.difficulty, "8");
    assert_eq!(hurricane.damage, "Strength + 3 + successes / B");

    // 3. Iron Shirt / Camisa de Ferro
    let iron_shirt = find_maneuver("Iron Shirt").expect("Iron Shirt deve ser encontrada");
    assert_eq!(iron_shirt.id, "iron_shirt");
    assert_eq!(iron_shirt.difficulty, "N/A");
    assert_eq!(iron_shirt.damage, "N/A (Soak Bonus)");
    assert_eq!(iron_shirt.actions, 0);
    assert!(iron_shirt.roll_pt.contains("absorção"));

    // 4. Kiaijutsu / Grito de Ferro
    let kiai = find_maneuver("Kiaijutsu").expect("Kiaijutsu deve ser encontrada");
    assert_eq!(kiai.id, "kiaijutsu");
    assert_eq!(kiai.difficulty, "7 / Willpower + 3 / 8");
    assert_eq!(kiai.difficulty(Language::EnUs), "7 / Willpower + 3 / 8");
    assert_eq!(kiai.difficulty(Language::PtBr), "7 / Vontade + 3 / 8");
    assert!(kiai.description(Language::PtBr).contains("Vontade do alvo + 3"));
    assert!(kiai.damage_pt.contains("Especial"));

    // 5. Plum Flower Blossom / Desabrochar da Flor de Ameixeira
    let plum = find_maneuver("Desabrochar da Flor de Ameixeira").expect("Desabrochar da Flor de Ameixeira deve ser encontrada");
    assert_eq!(plum.id, "plum_flower_blossom");
    assert_eq!(plum.difficulty, "6 (or 7 if attack)");
    assert!(plum.damage_pt.contains("dados de dano"));

    // 6. Soft Fist / Punho Suave (Jou Chuan)
    let soft_fist = find_maneuver("Soft Fist").expect("Soft Fist deve ser encontrada");
    assert_eq!(soft_fist.id, "soft_fist");
    assert_eq!(soft_fist.difficulty, "7");
    assert!(soft_fist.damage_pt.contains("Força do atacante"));
    assert!(soft_fist.damage.contains("Attacker's Strength"));

    // 7. Ten Thousand Weapons / Dez Mil Armas
    let weapons = find_maneuver("Dez Mil Armas").expect("Dez Mil Armas deve ser encontrada");
    assert_eq!(weapons.id, "ten_thousand_weapons");
    assert_eq!(weapons.difficulty, "As weapon / 6");
    assert_eq!(weapons.damage, "Do dice (B) / Lethal / +1 die (L)");

    // 8. Typhoon Kick / Chute Tufão
    let typhoon = find_maneuver("Typhoon Kick").expect("Typhoon Kick deve ser encontrada");
    assert_eq!(typhoon.id, "typhoon_kick");
    assert_eq!(typhoon.difficulty, "8");
    assert!(typhoon.damage_pt.contains("Força + 5 + sucessos / C ou L"));

    // 9. Weapon Art / Arte das Armas
    let weapon_art = find_maneuver("Arte das Armas").expect("Arte das Armas deve ser encontrada");
    assert_eq!(weapon_art.id, "weapon_art");
    assert_eq!(weapon_art.difficulty, "Normal - 1");
    assert!(weapon_art.damage_pt.contains("Conforme Arma"));

    // Teste de busca bilíngue direta
    assert_eq!(find_maneuver("Aparar Flechas").unwrap().id, "arrow_cutting");
    assert_eq!(find_maneuver("Camisa de Ferro").unwrap().id, "iron_shirt");
    assert_eq!(find_maneuver("Grito de Ferro").unwrap().id, "kiaijutsu");
    assert_eq!(find_maneuver("Flor de Ameixeira").unwrap().id, "plum_flower_blossom");
    assert_eq!(find_maneuver("Punho Suave").unwrap().id, "soft_fist");
    assert_eq!(find_maneuver("Jou Chuan").unwrap().id, "soft_fist");
    assert_eq!(find_maneuver("Chute Tufão").unwrap().id, "typhoon_kick");

    // Conversão de Dô CombatManeuver para WeaponItem
    let kiai_item = kiai.to_weapon_item(Language::PtBr);
    assert_eq!(kiai_item.name, "Kiaijutsu (Grito de Ferro)");
    assert_eq!(kiai_item.diff, "7 / Vontade + 3 / 8");
    assert_eq!(kiai_item.range, "C/C");

    let kiai_item_en = kiai.to_weapon_item(Language::EnUs);
    assert_eq!(kiai_item_en.name, "Kiaijutsu (Iron Shout)");
    assert_eq!(kiai_item_en.diff, "7 / Willpower + 3 / 8");
    assert_eq!(kiai_item_en.range, "Close");
}

#[test]
fn test_eight_limbs_and_do_rules_canonical_articles() {
    // 1. Artigo dos Oito Membros da Maestria (Eight Limbs of Expertise)
    let limbs_art = &EIGHT_LIMBS_ARTICLE;
    assert_eq!(limbs_art.id, "eight_limbs");
    assert_eq!(limbs_art.page_ref, "M20 Ch. 6, p. 580");
    assert_eq!(limbs_art.title(Language::PtBr), "Os Oito Membros da Maestria");
    assert_eq!(limbs_art.title(Language::EnUs), "Eight Limbs of Expertise");

    assert_eq!(limbs_art.limbs.len(), 8, "Devem existir exatamente 8 Membros de Dô");

    let limb1 = &limbs_art.limbs[0];
    assert_eq!(limb1.name, "Dharmamukti");
    assert_eq!(limb1.title(Language::PtBr), "Dharmamukti (A Mão Unida do Dharma)");
    assert_eq!(limb1.title(Language::EnUs), "Dharmamukti (The Dharma Clasped Hand)");
    assert!(limb1.abilities(Language::PtBr).contains(&"Atletismo"));
    assert!(limb1.abilities(Language::EnUs).contains(&"Athletics"));

    let limb2 = &limbs_art.limbs[1];
    assert_eq!(limb2.name, "Dhyana");
    assert_eq!(limb2.title(Language::PtBr), "Dhyana (O Membro da Meditação)");
    assert!(limb2.abilities(Language::PtBr).contains(&"Consciência"));
    assert!(limb2.abilities(Language::PtBr).contains(&"Meditação"));
    assert!(limb2.abilities(Language::EnUs).contains(&"Awareness"));
    assert!(limb2.abilities(Language::EnUs).contains(&"Meditation"));

    let limb3 = &limbs_art.limbs[2];
    assert_eq!(limb3.name, "Jivahasta");
    assert_eq!(limb3.title(Language::PtBr), "Jivahasta (A Mão da Vida)");
    assert!(limb3.abilities(Language::PtBr).iter().any(|a| a.contains("Medicina")));
    assert!(limb3.abilities(Language::EnUs).iter().any(|a| a.contains("Medicine")));

    let limb4 = &limbs_art.limbs[3];
    assert_eq!(limb4.name, "Karma");
    assert_eq!(limb4.title(Language::PtBr), "Karma (Devoção ao Labor Humilde)");
    assert!(limb4.abilities(Language::PtBr).iter().any(|a| a.contains("Etiqueta")));
    assert!(limb4.abilities(Language::EnUs).iter().any(|a| a.contains("Etiquette")));

    let limb5 = &limbs_art.limbs[4];
    assert_eq!(limb5.name, "Prajna");
    assert_eq!(limb5.title(Language::PtBr), "Prajna (Estudo da Ética e Filosofia)");
    assert!(limb5.abilities(Language::PtBr).iter().any(|a| a.contains("Cosmologia")));
    assert!(limb5.abilities(Language::EnUs).iter().any(|a| a.contains("Cosmology")));

    let limb6 = &limbs_art.limbs[5];
    assert_eq!(limb6.name, "Shastamarga");
    assert_eq!(limb6.title(Language::PtBr), "Shastamarga (O Caminho das Armas)");
    assert!(limb6.abilities(Language::PtBr).contains(&"Armas Brancas"));
    assert!(limb6.abilities(Language::EnUs).contains(&"Melee"));

    let limb7 = &limbs_art.limbs[6];
    assert_eq!(limb7.name, "Sunyakaya");
    assert_eq!(limb7.title(Language::PtBr), "Sunyakaya (O Membro do Corpo Vazio)");
    assert!(limb7.abilities(Language::PtBr).contains(&"Furtividade"));
    assert!(limb7.abilities(Language::EnUs).contains(&"Stealth"));

    let limb8 = &limbs_art.limbs[7];
    assert_eq!(limb8.name, "Tricanmarga");
    assert_eq!(limb8.title(Language::PtBr), "Tricanmarga (O Caminho da Tripla Luta)");
    assert!(limb8.abilities(Language::PtBr).contains(&"Acrobacia"));
    assert!(limb8.abilities(Language::EnUs).contains(&"Acrobatics"));

    // Validação da regra opcional: O Caminho Pacífico (The Peaceful Way)
    assert!(limbs_art.peaceful_way_rule_pt.contains("Força de Vontade"));
    assert!(limbs_art.peaceful_way_rule_pt.contains("dificuldade 8"));
    assert!(limbs_art.peaceful_way_rule.contains("Willpower"));
    assert!(limbs_art.peaceful_way_rule.contains("difficulty 8"));

    // 2. Artigo de Regras & Treino Canônico de Dô (Do Rules & Training)
    let rules_art = &DO_RULES_ARTICLE;
    assert_eq!(rules_art.id, "do_rules");
    assert_eq!(rules_art.page_ref, "M20 Ch. 6, pp. 580-581");
    assert_eq!(rules_art.title(Language::PtBr), "Dô: O Caminho da Vida & Destreza Marcial");
    assert_eq!(rules_art.title(Language::EnUs), "Do: The Way of Life & Fighting Prowess");

    // Validação do compromisso diário (1 hora por dia)
    assert!(rules_art.commitment_pt.contains("uma hora"));
    assert!(rules_art.commitment.contains("one hour"));

    // 7 Vantagens e peculiaridades de Dô
    assert_eq!(rules_art.advantages.len(), 7);
    let adv_codes: Vec<&str> = rules_art.advantages.iter().map(|a| a.code).collect();
    assert!(adv_codes.contains(&"secret_teachings"));
    assert!(adv_codes.contains(&"flexibility"));
    assert!(adv_codes.contains(&"precision"));
    assert!(adv_codes.contains(&"martial_mastery"));
    assert!(adv_codes.contains(&"differences_in_mastery"));
    assert!(adv_codes.contains(&"lethal_damage"));
    assert!(adv_codes.contains(&"hardened_defense"));

    // Verificação de conteúdo em português
    let precision = rules_art.advantages.iter().find(|a| a.code == "precision").unwrap();
    assert_eq!(precision.title(Language::PtBr), "Precisão (Vantagem de Bem Treinado)");
    assert!(precision.rule(Language::PtBr).contains("-1"));
    assert!(precision.rule(Language::EnUs).contains("-1"));

    let hardened = rules_art.advantages.iter().find(|a| a.code == "hardened_defense").unwrap();
    assert_eq!(hardened.title(Language::PtBr), "Defesa Endurecida");
    assert!(hardened.rule(Language::PtBr).contains("mãos nuas"));
    assert!(hardened.rule(Language::EnUs).contains("bare palms"));
}

#[test]
fn test_find_combat_entity_quick_index() {
    use mta_sheet::compendium::weapons::{find_combat_entity, CombatEntity};

    // 1. Armas exatas e parciais
    assert!(matches!(find_combat_entity("Katana"), Some(CombatEntity::Weapon(w)) if w.id == "katana"));
    assert!(matches!(find_combat_entity("Espada Longa"), Some(CombatEntity::Weapon(w)) if w.id == "sword"));
    assert!(matches!(find_combat_entity("Espada Longa +1"), Some(CombatEntity::Weapon(w)) if w.id == "sword"));
    assert!(matches!(find_combat_entity("Pistola Pesada (Glock 9mm)"), Some(CombatEntity::Weapon(w)) if w.id == "pistol_hvy"));
    assert!(matches!(find_combat_entity("Revólver Pesado .44"), Some(CombatEntity::Weapon(w)) if w.id == "revolver_hvy"));

    // 2. Manobras de combate gerais e marciais
    assert!(matches!(find_combat_entity("Soco"), Some(CombatEntity::Maneuver(m)) if m.id == "punch"));
    assert!(matches!(find_combat_entity("Punch"), Some(CombatEntity::Maneuver(m)) if m.id == "punch"));
    assert!(matches!(find_combat_entity("Chute"), Some(CombatEntity::Maneuver(m)) if m.id == "kick"));
    assert!(matches!(find_combat_entity("Desarmar"), Some(CombatEntity::Maneuver(m)) if m.id == "disarm"));
    assert!(matches!(find_combat_entity("Agarrar"), Some(CombatEntity::Maneuver(m)) if m.id == "grapple"));
    assert!(matches!(find_combat_entity("Arremesso Furacão"), Some(CombatEntity::Maneuver(m)) if m.id == "hurricane_throw"));
    assert!(matches!(find_combat_entity("Aparar Flechas"), Some(CombatEntity::Maneuver(m)) if m.id == "arrow_cutting"));

    // 3. Técnicas especiais de Dô com parênteses geradas pelo sistema
    assert!(matches!(find_combat_entity("Kiaijutsu (Grito de Ferro)"), Some(CombatEntity::Maneuver(m)) if m.id == "kiaijutsu"));
    assert!(matches!(find_combat_entity("Kiaijutsu (Iron Shout)"), Some(CombatEntity::Maneuver(m)) if m.id == "kiaijutsu"));
    assert!(matches!(find_combat_entity("Desabrochar da Flor de Ameixeira"), Some(CombatEntity::Maneuver(m)) if m.id == "plum_flower_blossom"));

    // 4. Casos vazios ou inválidos
    assert!(find_combat_entity("").is_none());
    assert!(find_combat_entity("   ").is_none());
    assert!(find_combat_entity("item_totalmente_inexistente_12345").is_none());
}
