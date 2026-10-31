//! Testes da seção Aliados & Antagonistas (M20, Apêndice I, pp. 618-641): textos,
//! navegação `mta://bestiary/<id>`, busca e leitura dos números de cada ficha.

use mta_sheet::compendium::bestiary::{find_bestiary_entry, BestiaryGroup, ALL_BESTIARY};
use mta_sheet::components::compendium::navigation::CompendiumTarget;
use mta_sheet::components::compendium::{resolve_compendium_query, CompendiumSection};
use mta_sheet::i18n::Language;

#[test]
fn test_bestiary_entries_have_text_pages_and_stats() {
    for e in ALL_BESTIARY {
        assert!(e.page_ref.starts_with("M20, p"), "{} com página inválida: {}", e.id, e.page_ref);
        assert!(!e.name_pt.is_empty(), "{} sem nome em PT", e.id);
        assert!(e.has_pt_text(), "{} sem tradução para PT", e.id);
        assert!(e.content(Language::PtBr).contains("**Fonte:** Livro Básico M20"), "{} sem cabeçalho de fonte PT", e.id);
        if !e.is_article {
            assert!(e.content.starts_with("### "), "{} sem título EN", e.id);
            assert!(e.content_pt.starts_with("### "), "{} sem título PT", e.id);
            if e.group != BestiaryGroup::Totems && e.id != "sendings" && e.id != "memophores" {
                assert!(!e.stat_lines().is_empty(), "{} sem ficha EN", e.id);
                assert!(!e.stat_lines_pt().is_empty(), "{} sem ficha PT", e.id);
            }
        }
    }
    // Toda ficha de pessoa ou animal com Destreza e Raciocínio tem base de iniciativa.
    for id in ["thug", "beat_cop", "black_suit", "hit_mark_x", "wolf", "alligator", "pillory", "hive"] {
        let q = find_bestiary_entry(id).unwrap().quick_stats();
        assert!(q.initiative_base().is_some(), "{} sem iniciativa", id);
        assert!(q.willpower.is_some(), "{} sem Força de Vontade", id);
        assert!(!q.health_levels.is_empty(), "{} sem vitalidade", id);
    }
    // Espíritos trazem Rage, Gnosis e Essence.
    for e in ALL_BESTIARY.iter().filter(|e| e.group == BestiaryGroup::Elementals && !e.is_article) {
        let q = e.quick_stats();
        assert!(q.rage.is_some() && q.gnosis.is_some() && q.essence.is_some(), "{} sem traços de espírito", e.id);
    }
}

#[test]
fn test_bestiary_navigation_and_query() {
    let target = CompendiumTarget::from_uri("mta://bestiary/black_suit").expect("URI deve ser válida");
    assert_eq!(target.section(), CompendiumSection::Bestiary);
    assert_eq!(target.label(Language::EnUs), "Black Suit");
    assert_eq!(target.label(Language::PtBr), "Terno Preto");

    let loc = resolve_compendium_query(CompendiumSection::Practices, "Thug").expect("Deve achar a ficha");
    assert_eq!(loc.section, CompendiumSection::Bestiary);
    assert_eq!(loc.item_id, "thug");
}
