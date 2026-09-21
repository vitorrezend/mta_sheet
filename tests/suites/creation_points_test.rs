use mta_sheet::state::{keys, AttributeValue, CharacterData};

fn set_trait(data: &mut CharacterData, key: &str, level: i32) {
    data.attributes.insert(key.to_string(), AttributeValue::new(level, key.to_string()));
}

fn add_custom_trait(data: &mut CharacterData, category: &str, id: &str, label: &str, level: i32) {
    data.custom_lists.entry(category.to_string()).or_default().push(id.to_string());
    data.labels.insert(id.to_string(), label.to_string());
    data.attributes.insert(id.to_string(), AttributeValue::new(level, label.to_string()));
}

#[test]
fn test_default_character_creation_points() {
    let char_data = CharacterData::new("Mago Teste".to_string(), "Tradicionalista".to_string());
    let cp = char_data.calculate_creation_points();

    // Default character: 1 free in all 9 attributes (0 creation points spent)
    assert_eq!(cp.attr_total_spent, 0);
    assert_eq!(cp.attr_physical, 0);
    assert_eq!(cp.attr_social, 0);
    assert_eq!(cp.attr_mental, 0);
    assert!(cp.attr_spread_valid);
    assert!(!cp.attr_exceeded);

    // Default abilities: 0 spent
    assert_eq!(cp.ab_total_spent, 0);
    assert!(cp.ab_spread_valid);
    assert!(!cp.ab_exceeded);
    assert!(cp.ab_cap_violations.is_empty());

    // Default spheres, backgrounds, resonance
    assert_eq!(cp.spheres_spent, 0);
    assert!(!cp.spheres_exceeded);
    assert_eq!(cp.backgrounds_spent, 0);
    assert!(!cp.backgrounds_exceeded);
    assert_eq!(cp.resonance_spent, 0);
    assert!(!cp.resonance_exceeded);

    // Default Arete (1 free) and Willpower (5 free)
    assert_eq!(cp.arete_base, 1);
    assert!(!cp.arete_exceeded);
    assert_eq!(cp.willpower_base, 5);
    assert!(!cp.willpower_exceeded);

    // Overall status
    assert!(!cp.has_any_overflow);
    assert!(cp.warnings.is_empty());
}

#[test]
fn test_valid_creation_points_allocation() {
    let mut char_data = CharacterData::new("Hermético".to_string(), "Ordem de Hermes".to_string());
    char_data.set_affinity_sphere(Some("Forças".to_string()));

    // 1. Atributos: 7 em Físicos (Força 4 = 3 pts, Destreza 3 = 2 pts, Vigor 3 = 2 pts -> 7),
    //               5 em Sociais (Carisma 3 = 2 pts, Manipulação 3 = 2 pts, Aparência 2 = 1 pt -> 5),
    //               3 em Mentais (Percepção 2 = 1 pt, Inteligência 2 = 1 pt, Raciocínio 2 = 1 pt -> 3)
    set_trait(&mut char_data, "Força", 4);
    set_trait(&mut char_data, "Destreza", 3);
    set_trait(&mut char_data, "Vigor", 3);

    set_trait(&mut char_data, "Carisma", 3);
    set_trait(&mut char_data, "Manipulação", 3);
    set_trait(&mut char_data, "Aparência", 2);

    set_trait(&mut char_data, "Percepção", 2);
    set_trait(&mut char_data, "Inteligência", 2);
    set_trait(&mut char_data, "Raciocínio", 2);

    // 2. Habilidades: 13 em Talentos, 9 em Perícias, 5 em Conhecimentos (máximo 3 por habilidade)
    set_trait(&mut char_data, "Prontidão", 3);
    set_trait(&mut char_data, "Esportes", 3);
    set_trait(&mut char_data, "Briga", 3);
    set_trait(&mut char_data, "Esquiva", 3);
    set_trait(&mut char_data, "Consciência", 1); // 3+3+3+3+1 = 13

    set_trait(&mut char_data, "Ofícios", 3);
    set_trait(&mut char_data, "Condução", 3);
    set_trait(&mut char_data, "Furtividade", 3); // 3+3+3 = 9

    set_trait(&mut char_data, "Ocultismo", 3);
    set_trait(&mut char_data, "Esotérica", 2); // 3+2 = 5

    // 3. Esferas: 6 pontos (+ 1 grátis em Forças)
    set_trait(&mut char_data, "Forças", 3); // 1 grátis + 2 gastos = 2 pts
    set_trait(&mut char_data, "Primórdio", 2); // 2 pts
    set_trait(&mut char_data, "Mente", 2); // 2 pts -> Total 2+2+2 = 6

    // 4. Antecedentes: 7 pontos
    add_custom_trait(&mut char_data, keys::CAT_ANTECEDENTES, "bg_avatar", "Avatar", 3);
    add_custom_trait(&mut char_data, keys::CAT_ANTECEDENTES, "bg_recursos", "Recursos", 2);
    add_custom_trait(&mut char_data, keys::CAT_ANTECEDENTES, "bg_mentor", "Mentor", 2); // 3+2+2 = 7

    // 5. Ressonância: 1 ponto
    add_custom_trait(&mut char_data, keys::CAT_RESONANCE, "res_dinamica", "Dinâmica", 1);

    let cp = char_data.calculate_creation_points();

    assert_eq!(cp.attr_physical, 7);
    assert_eq!(cp.attr_social, 5);
    assert_eq!(cp.attr_mental, 3);
    assert_eq!(cp.attr_total_spent, 15);
    assert!(cp.attr_spread_valid);
    assert!(!cp.attr_exceeded);

    assert_eq!(cp.ab_talents, 13);
    assert_eq!(cp.ab_skills, 9);
    assert_eq!(cp.ab_knowledges, 5);
    assert_eq!(cp.ab_total_spent, 27);
    assert!(cp.ab_spread_valid);
    assert!(!cp.ab_exceeded);
    assert!(cp.ab_cap_violations.is_empty());

    assert_eq!(cp.spheres_spent, 6);
    assert!(!cp.spheres_exceeded);

    assert_eq!(cp.backgrounds_spent, 7);
    assert!(!cp.backgrounds_exceeded);

    assert_eq!(cp.resonance_spent, 1);
    assert!(!cp.resonance_exceeded);

    assert!(!cp.has_any_overflow);
    assert!(cp.warnings.is_empty());
}

#[test]
fn test_attributes_overflow_and_invalid_spread() {
    let mut char_data = CharacterData::new("Mago".to_string(), "Tradição".to_string());

    // 8 em Físicos (excede 7), 5 em Sociais, 3 em Mentais = 16 pts
    set_trait(&mut char_data, "Força", 5); // 4 pts
    set_trait(&mut char_data, "Destreza", 3); // 2 pts
    set_trait(&mut char_data, "Vigor", 3); // 2 pts -> 8 pts físicos

    let cp = char_data.calculate_creation_points();
    assert_eq!(cp.attr_physical, 8);
    assert!(!cp.attr_spread_valid);
    assert!(cp.attr_exceeded);
    assert!(cp.has_any_overflow);
    assert!(!cp.warnings.is_empty());
}

#[test]
fn test_ability_cap_violation_and_spread_overflow() {
    let mut char_data = CharacterData::new("Mago".to_string(), "Tradição".to_string());

    // Esquiva com 4 bolinhas base (violação da regra de max 3 na criação base)
    set_trait(&mut char_data, "Esquiva", 4);

    let cp = char_data.calculate_creation_points();
    assert_eq!(cp.ab_cap_violations.len(), 1);
    assert!(cp.ab_exceeded);
    assert!(cp.has_any_overflow);
    assert!(cp.warnings.iter().any(|w| w.contains("Esquiva")));
}

#[test]
fn test_spheres_and_special_advantages_overflow() {
    let mut char_data = CharacterData::new("Mago".to_string(), "Tradição".to_string());

    // Esferas: 7 pontos gastos (orçamento é 6)
    set_trait(&mut char_data, "Forças", 4);
    set_trait(&mut char_data, "Matéria", 3); // 4 + 3 = 7

    // Arete base 2 (orçamento é 1 grátis)
    set_trait(&mut char_data, keys::KEY_ARETE, 2);

    // Força de Vontade base 6 (orçamento é 5 grátis)
    set_trait(&mut char_data, keys::KEY_WILLPOWER_TOTAL, 6);

    // Antecedentes: 8 pontos (orçamento é 7)
    add_custom_trait(&mut char_data, keys::CAT_ANTECEDENTES, "bg_avatar", "Avatar", 5);
    add_custom_trait(&mut char_data, keys::CAT_ANTECEDENTES, "bg_recursos", "Recursos", 3); // 8

    // Ressonância: 2 pontos (orçamento é 1)
    add_custom_trait(&mut char_data, keys::CAT_RESONANCE, "res_estatica", "Estática", 2);

    let cp = char_data.calculate_creation_points();
    assert_eq!(cp.spheres_spent, 7);
    assert!(cp.spheres_exceeded);

    assert_eq!(cp.arete_base, 2);
    assert!(cp.arete_exceeded);

    assert_eq!(cp.willpower_base, 6);
    assert!(cp.willpower_exceeded);

    assert_eq!(cp.backgrounds_spent, 8);
    assert!(cp.backgrounds_exceeded);

    assert_eq!(cp.resonance_spent, 2);
    assert!(cp.resonance_exceeded);

    assert!(cp.has_any_overflow);
    assert_eq!(cp.warnings.len(), 5);
}
