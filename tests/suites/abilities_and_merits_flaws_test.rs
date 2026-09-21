use mta_sheet::compendium::abilities::{
    find_ability, find_ability_theory_rule, ALL_ABILITIES, ABILITY_THEORY_RULES, AbilityCategory, AbilityScope,
};
use mta_sheet::compendium::merits_flaws::{
    find_derangement, find_merit_flaw, ALL_DERANGEMENTS, ALL_MERITS_FLAWS, TraitCategory, TraitType,
};
use mta_sheet::i18n::Language;
use mta_sheet::state::{CharacterData, DotOrigin, keys};

#[test]
fn test_abilities_compendium_data_integrity() {
    assert!(!ALL_ABILITIES.is_empty(), "ALL_ABILITIES não pode ser vazio");
    assert!(ALL_ABILITIES.len() >= 40, "Deve haver pelo menos 40 habilidades catalogadas");

    // Verifica que temos habilidades em todas as 3 categorias
    let talents_count = ALL_ABILITIES.iter().filter(|a| a.category == AbilityCategory::Talents).count();
    let skills_count = ALL_ABILITIES.iter().filter(|a| a.category == AbilityCategory::Skills).count();
    let knowledges_count = ALL_ABILITIES.iter().filter(|a| a.category == AbilityCategory::Knowledges).count();

    assert!(talents_count >= 11, "Deve haver pelo menos 11 talentos (core + secundárias)");
    assert!(skills_count >= 11, "Deve haver pelo menos 11 perícias (core + secundárias)");
    assert!(knowledges_count >= 11, "Deve haver pelo menos 11 conhecimentos (core + secundárias)");

    // Verifica que temos habilidades Core e Secundárias
    let core_count = ALL_ABILITIES.iter().filter(|a| a.scope == AbilityScope::Core).count();
    let sec_count = ALL_ABILITIES.iter().filter(|a| a.scope == AbilityScope::Secondary).count();

    assert_eq!(core_count, 33, "Devem existir exatamente 33 habilidades core (11 de cada categoria)");
    assert!(sec_count >= 15, "Devem existir pelo menos 15 habilidades secundárias");

    // Valida regras teóricas opcionais
    assert!(!ABILITY_THEORY_RULES.is_empty());
    let body_ctrl = find_ability_theory_rule("rule_body_control");
    assert!(body_ctrl.is_some());
    assert!(!body_ctrl.unwrap().content(Language::PtBr).is_empty());
    assert!(!body_ctrl.unwrap().content(Language::EnUs).is_empty());
}

#[test]
fn test_abilities_bilingual_lookups() {
    // Busca em PT
    let prontidao = find_ability("Prontidão");
    assert!(prontidao.is_some());
    assert_eq!(prontidao.unwrap().id, "alertness");

    // Busca em EN
    let alertness = find_ability("Alertness");
    assert!(alertness.is_some());
    assert_eq!(alertness.unwrap().id, "alertness");

    // Busca por ID
    let by_id = find_ability("alertness");
    assert!(by_id.is_some());

    // Secundária Do / Dô
    let do_art = find_ability("Do");
    assert!(do_art.is_some());
    assert_eq!(do_art.unwrap().category, AbilityCategory::Talents);
    assert_eq!(do_art.unwrap().scope, AbilityScope::Secondary);
}

#[test]
fn test_merits_flaws_data_integrity() {
    assert!(!ALL_MERITS_FLAWS.is_empty(), "ALL_MERITS_FLAWS não pode ser vazio");
    assert!(ALL_MERITS_FLAWS.len() >= 15, "Deve haver pelo menos 15 qualidades e defeitos");

    let merits_count = ALL_MERITS_FLAWS.iter().filter(|mf| mf.trait_type == TraitType::Merit).count();
    let flaws_count = ALL_MERITS_FLAWS.iter().filter(|mf| mf.trait_type == TraitType::Flaw).count();

    assert!(merits_count >= 8, "Deve haver pelo menos 8 qualidades");
    assert!(flaws_count >= 6, "Deve haver pelo menos 6 defeitos");

    let senses = find_merit_flaw("acute_senses");
    assert!(senses.is_some());
    assert_eq!(senses.unwrap().category, TraitCategory::Physical);
    assert_eq!(senses.unwrap().trait_type, TraitType::Merit);
    assert_eq!(senses.unwrap().name(Language::PtBr), "Sentidos Aguçados");
    assert_eq!(senses.unwrap().name(Language::EnUs), "Acute Senses");

    let addiction = find_merit_flaw("addiction");
    assert!(addiction.is_some());
    assert_eq!(addiction.unwrap().category, TraitCategory::Physical);
    assert_eq!(addiction.unwrap().trait_type, TraitType::Flaw);
    assert_eq!(addiction.unwrap().name(Language::PtBr), "Vício");
    assert_eq!(addiction.unwrap().name(Language::EnUs), "Addiction");

    // Verifica custos válidos
    for mf in ALL_MERITS_FLAWS.iter() {
        assert!(!mf.available_costs.is_empty(), "Qualidade/Defeito {} deve ter custos disponíveis", mf.id);
        for &cost in mf.available_costs {
            assert!(cost >= 1 && cost <= 10, "Custo {} fora do intervalo em {}", cost, mf.id);
        }
    }
}

#[test]
fn test_derangements_data_integrity() {
    assert!(!ALL_DERANGEMENTS.is_empty(), "ALL_DERANGEMENTS não pode ser vazio");
    assert_eq!(ALL_DERANGEMENTS.len(), 10, "Devem existir exatamente 10 perturbações mentais canônicas de M20");

    let amnesia = find_derangement("amnesia");
    assert!(amnesia.is_some());
    assert_eq!(amnesia.unwrap().name(Language::PtBr), "Amnésia");
    assert_eq!(amnesia.unwrap().name(Language::EnUs), "Amnesia");

    let paranoia = find_derangement("paranoia");
    assert!(paranoia.is_some());
    assert_eq!(paranoia.unwrap().name(Language::PtBr), "Paranoia");
}

#[test]
fn test_character_sheet_merits_flaws_cost_balance() {
    let mut sheet = CharacterData::default();

    // Adiciona uma qualidade de 3 pontos no modo bônus
    let m_id = "merit_1".to_string();
    sheet.custom_lists.entry(keys::CAT_MERITS.to_string()).or_default().push(m_id.clone());
    sheet.labels.insert(m_id.clone(), "Sentidos Aguçados".to_string());
    sheet.set_attribute_with_origin(&m_id, Some(3), None, DotOrigin::Bonus);

    // Adiciona um defeito de 2 pontos no modo bônus
    let f_id = "flaw_1".to_string();
    sheet.custom_lists.entry(keys::CAT_FLAWS.to_string()).or_default().push(f_id.clone());
    sheet.labels.insert(f_id.clone(), "Vício".to_string());
    sheet.set_attribute_with_origin(&f_id, Some(2), None, DotOrigin::Bonus);

    let costs = sheet.calculate_costs();
    let merit_item = costs.items.iter().find(|i| i.category == "Qualidade").expect("Qualidade deve estar presente nos custos");
    assert_eq!(merit_item.bonus_cost, 3, "Qualidade de nível 3 deve custar 3 pontos de bônus");

    let flaw_item = costs.items.iter().find(|i| i.category == "Defeito").expect("Defeito deve estar presente nos custos");
    assert_eq!(flaw_item.bonus_cost, -2, "Defeito de nível 2 deve subtrair 2 pontos de bônus (-2)");

    assert_eq!(costs.total_bonus_spent, 1, "Saldo líquido de bônus (3 da qualidade - 2 do defeito) deve ser 1");
}
