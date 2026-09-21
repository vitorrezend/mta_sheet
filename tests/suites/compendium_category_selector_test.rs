use mta_sheet::components::compendium::{ALL_SECTIONS, get_section_meta, CompendiumSection};
use mta_sheet::i18n::Language;

#[test]
fn test_compendium_all_sections_count_and_metadata() {
    assert_eq!(ALL_SECTIONS.len(), 9, "O Compêndio M20 deve ter exatamente 9 seções canônicas");

    let expected_sections = [
        CompendiumSection::Practices,
        CompendiumSection::Instruments,
        CompendiumSection::Archetypes,
        CompendiumSection::Attributes,
        CompendiumSection::Backgrounds,
        CompendiumSection::Spheres,
        CompendiumSection::Weapons,
        CompendiumSection::Abilities,
        CompendiumSection::MeritsFlaws,
    ];

    for (idx, expected) in expected_sections.iter().enumerate() {
        assert_eq!(ALL_SECTIONS[idx].section, *expected);
        assert!(!ALL_SECTIONS[idx].icon.is_empty(), "Cada seção deve ter um ícone");
        assert!(!ALL_SECTIONS[idx].name_pt.is_empty(), "Cada seção deve ter um nome em PT");
        assert!(!ALL_SECTIONS[idx].name_en.is_empty(), "Cada seção deve ter um nome em EN");
        assert!(!ALL_SECTIONS[idx].count.is_empty(), "Cada seção deve ter a contagem");
    }
}

#[test]
fn test_get_section_meta_lookup() {
    let bg_meta = get_section_meta(CompendiumSection::Backgrounds);
    assert_eq!(bg_meta.icon, "👥");
    assert_eq!(bg_meta.name_pt, "Antecedentes");
    assert_eq!(bg_meta.name_en, "Backgrounds");
    assert_eq!(bg_meta.count, "33");

    let sphere_meta = get_section_meta(CompendiumSection::Spheres);
    assert_eq!(sphere_meta.icon, "🔮");
    assert_eq!(sphere_meta.name_pt, "Esferas da Mágika");
    assert_eq!(sphere_meta.name_en, "Spheres of Magick");
    assert_eq!(sphere_meta.count, "13");

    let weapon_meta = get_section_meta(CompendiumSection::Weapons);
    assert_eq!(weapon_meta.icon, "⚔️");
    assert_eq!(weapon_meta.name_pt, "Armas & Manobras");
    assert_eq!(weapon_meta.name_en, "Weapons & Maneuvers");
    assert_eq!(weapon_meta.count, "126");
}

#[test]
fn test_cyclic_navigation_indices() {
    // 1. Navegação para frente a partir do início
    let first_idx = 0; // Practices
    let next_from_first = (first_idx + 1) % ALL_SECTIONS.len();
    assert_eq!(ALL_SECTIONS[next_from_first].section, CompendiumSection::Instruments);

    // 2. Navegação para trás a partir do início (deve ir para a última seção)
    let prev_from_first = if first_idx == 0 { ALL_SECTIONS.len() - 1 } else { first_idx - 1 };
    assert_eq!(ALL_SECTIONS[prev_from_first].section, CompendiumSection::MeritsFlaws);

    // 3. Navegação para frente a partir do fim (deve dar a volta para o início)
    let last_idx = ALL_SECTIONS.len() - 1; // MeritsFlaws
    let next_from_last = (last_idx + 1) % ALL_SECTIONS.len();
    assert_eq!(ALL_SECTIONS[next_from_last].section, CompendiumSection::Practices);
}

#[test]
fn test_bilingual_titles_no_empty_or_truncated() {
    for meta in ALL_SECTIONS {
        assert!(meta.name_pt.len() >= 5, "Nome PT não pode ser excessivamente curto ou vazio: {}", meta.name_pt);
        assert!(meta.name_en.len() >= 5, "Nome EN não pode ser excessivamente curto ou vazio: {}", meta.name_en);
        assert!(!meta.name_pt.contains("..."), "Nome PT não deve conter truncamento: {}", meta.name_pt);
        assert!(!meta.name_en.contains("..."), "Nome EN não deve conter truncamento: {}", meta.name_en);
    }
}
