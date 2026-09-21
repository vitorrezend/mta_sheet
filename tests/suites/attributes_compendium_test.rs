use mta_sheet::compendium::attributes::*;
use mta_sheet::i18n::Language;

#[test]
fn test_all_attributes_count_and_categories() {
    // Exatamente 9 atributos canônicos de M20 (pp. 273-275)
    assert_eq!(ALL_ATTRIBUTES.len(), 9, "O compêndio deve conter exatamente 9 atributos canônicos");

    let physical: Vec<_> = ALL_ATTRIBUTES.iter().filter(|a| a.category == "physical").collect();
    let social: Vec<_> = ALL_ATTRIBUTES.iter().filter(|a| a.category == "social").collect();
    let mental: Vec<_> = ALL_ATTRIBUTES.iter().filter(|a| a.category == "mental").collect();

    assert_eq!(physical.len(), 3, "Devem existir 3 atributos físicos (Força, Destreza, Vigor)");
    assert_eq!(social.len(), 3, "Devem existir 3 atributos sociais (Carisma, Manipulação, Aparência)");
    assert_eq!(mental.len(), 3, "Devem existir 3 atributos mentais (Percepção, Inteligência, Raciocínio)");
}

#[test]
fn test_ratings_scale_structure() {
    for attr in ALL_ATTRIBUTES.iter() {
        assert_eq!(attr.ratings.len(), 5, "Cada atributo deve ter exatamente 5 níveis de escala (1 a 5 bolinhas): {}", attr.id);
        for (idx, rating) in attr.ratings.iter().enumerate() {
            assert_eq!(rating.dots, (idx + 1) as i32, "O nível de bolinhas deve ser sequencial de 1 a 5 para {}", attr.id);
            assert!(!rating.title.is_empty(), "O título em inglês não pode ser vazio para {} nível {}", attr.id, rating.dots);
            assert!(!rating.title_pt.is_empty(), "O título em português não pode ser vazio para {} nível {}", attr.id, rating.dots);
            assert!(!rating.description.is_empty(), "A descrição em inglês não pode ser vazia para {} nível {}", attr.id, rating.dots);
            assert!(!rating.description_pt.is_empty(), "A descrição em português não pode ser vazia para {} nível {}", attr.id, rating.dots);
            assert_eq!(rating.title(Language::EnUs), rating.title);
            assert_eq!(rating.title(Language::PtBr), rating.title_pt);
        }
    }
}

#[test]
fn test_suggested_specialties_present() {
    for attr in ALL_ATTRIBUTES.iter() {
        assert!(!attr.suggested_specialties_pt.is_empty(), "Especialidades sugeridas em PT não podem estar vazias para {}", attr.id);
        assert!(!attr.suggested_specialties.is_empty(), "Especialidades sugeridas em EN não podem estar vazias para {}", attr.id);
        assert_eq!(
            attr.suggested_specialties_pt.len(),
            attr.suggested_specialties.len(),
            "A quantidade de especialidades sugeridas em PT e EN deve coincidir para {}",
            attr.id
        );
        assert_eq!(attr.specialties(Language::PtBr), attr.suggested_specialties_pt);
        assert_eq!(attr.specialties(Language::EnUs), attr.suggested_specialties);
    }
}

#[test]
fn test_find_attribute_lookups() {
    // English ID
    assert!(find_attribute("strength").is_some());
    assert!(find_attribute("dexterity").is_some());
    assert!(find_attribute("stamina").is_some());
    assert!(find_attribute("charisma").is_some());
    assert!(find_attribute("manipulation").is_some());
    assert!(find_attribute("appearance").is_some());
    assert!(find_attribute("perception").is_some());
    assert!(find_attribute("intelligence").is_some());
    assert!(find_attribute("wits").is_some());

    // Portuguese name with accent
    let forca = find_attribute("Força").expect("Deve encontrar Força em PT");
    assert_eq!(forca.id, "strength");

    let destreza = find_attribute("Destreza").expect("Deve encontrar Destreza em PT");
    assert_eq!(destreza.id, "dexterity");

    let vigor = find_attribute("Vigor").expect("Deve encontrar Vigor em PT");
    assert_eq!(vigor.id, "stamina");

    let raciocinio = find_attribute("Raciocínio").expect("Deve encontrar Raciocínio em PT");
    assert_eq!(raciocinio.id, "wits");

    // Case and accent insensitivity (without accents)
    assert!(find_attribute("forca").is_some());
    assert!(find_attribute("raciocinio").is_some());
    assert!(find_attribute("STRENGTH").is_some());
    assert!(find_attribute("DesTreZa").is_some());
}

#[test]
fn test_get_suggested_specialties() {
    let specs_pt = get_suggested_specialties("strength", Language::PtBr);
    assert!(specs_pt.contains(&"Poder Bruto"));
    assert!(specs_pt.contains(&"Pegada de Ferro"));

    let specs_en = get_suggested_specialties("strength", Language::EnUs);
    assert!(specs_en.contains(&"Raw Power"));
    assert!(specs_en.contains(&"Iron Grip"));

    // By Portuguese name
    let specs_pt_lookup = get_suggested_specialties("Força", Language::PtBr);
    assert_eq!(specs_pt, specs_pt_lookup);
}

#[test]
fn test_specialties_rule_m20() {
    assert_eq!(SPECIALTIES_RULE.page_ref, "M20, p. 273");
    assert!(SPECIALTIES_RULE.title_pt.contains("Especialidades"));
    assert!(SPECIALTIES_RULE.title.contains("Specialties"));
    assert!(SPECIALTIES_RULE.content_pt.contains("10"));
    assert!(SPECIALTIES_RULE.content_pt.contains("DOIS sucessos"));
    assert!(SPECIALTIES_RULE.content.contains("two successes"));
    assert!(SPECIALTIES_RULE.content.contains("Spider Chase"));
}
