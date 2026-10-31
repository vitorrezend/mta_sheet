//! Testes da seção de Tradições e da União Tecnocrática do Compêndio (M20, pp. 147-195): textos integrais,
//! navegação `mta://traditions/<id>` e validade de todos os links internos.

use mta_sheet::compendium::abilities::{find_ability, find_ability_theory_rule};
use mta_sheet::compendium::practices::find_practice;
use mta_sheet::compendium::spheres::find_sphere;
use mta_sheet::compendium::traditions::{
    find_convention, find_disparate, find_faction, find_tradition, ALL_CONVENTIONS, ALL_DISPARATES, ALL_TRADITIONS,
    TRADITION_DESCRIPTIONS,
};
use mta_sheet::components::compendium::navigation::CompendiumTarget;
use mta_sheet::components::compendium::{resolve_compendium_query, CompendiumSection};
use mta_sheet::i18n::Language;

fn assert_links_resolve(source: &str, text: &str) {
    let mut remaining = text;
    while let Some(start) = remaining.find("mta://") {
        let after = &remaining[start..];
        let end = after.find(')').unwrap_or(after.len());
        let uri = &after[..end];
        let target = CompendiumTarget::from_uri(uri).unwrap_or_else(|| panic!("URI inválida em {}: {}", source, uri));
        let exists = match &target {
            CompendiumTarget::Sphere(id) => find_sphere(id).is_some(),
            CompendiumTarget::Practice(id) => find_practice(id).is_some(),
            CompendiumTarget::Tradition(id) => find_faction(id).is_some(),
            CompendiumTarget::Ability(id) => find_ability(id).is_some() || find_ability_theory_rule(id).is_some(),
            other => panic!("Link inesperado em {}: {:?}", source, other),
        };
        assert!(exists, "Link para verbete inexistente em {}: {}", source, uri);
        remaining = &after[end..];
    }
}

#[test]
fn test_traditions_full_texts_and_links() {
    assert_eq!(ALL_TRADITIONS.len(), 9);
    for t in ALL_TRADITIONS {
        assert!(t.content(Language::EnUs).is_some());
        assert!(t.content(Language::PtBr).is_some(), "Texto em português deve existir para {}", t.id);
        for section in ["#### Organization", "#### Initiation", "#### Focus", "#### Stereotypes", "**Source:** M20"] {
            assert!(t.content.contains(section), "{} sem {}", t.id, section);
        }
        for section in ["#### Organização", "#### Iniciação", "#### Foco", "#### Estereótipos", "**Fonte:** Livro Básico M20"] {
            assert!(t.content_pt.contains(section), "{} sem {} em PT", t.id, section);
        }
        for sid in t.affinity_spheres {
            assert!(find_sphere(sid).is_some(), "Esfera de afinidade inexistente em {}: {}", t.id, sid);
        }
        assert_links_resolve(t.id, t.content);
        assert_links_resolve(t.id, t.content_pt);
    }
    assert_links_resolve(TRADITION_DESCRIPTIONS.id, TRADITION_DESCRIPTIONS.content);
    assert_links_resolve(TRADITION_DESCRIPTIONS.id, TRADITION_DESCRIPTIONS.content_pt);
}

#[test]
fn test_traditions_navigation_and_query() {
    let target = CompendiumTarget::from_uri("mta://traditions/order_of_hermes").expect("URI deve ser válida");
    assert_eq!(target, CompendiumTarget::Tradition("order_of_hermes".to_string()));
    assert_eq!(target.section(), CompendiumSection::Traditions);
    assert_eq!(target.label(Language::EnUs), "Order of Hermes");
    assert_eq!(target.label(Language::PtBr), "Ordem de Hermes");

    let intro = CompendiumTarget::Tradition(TRADITION_DESCRIPTIONS.id.to_string());
    assert_eq!(intro.label(Language::EnUs), "Tradition Descriptions");
    assert_eq!(intro.label(Language::PtBr), "Descrição das Tradições");

    let loc = resolve_compendium_query(CompendiumSection::Practices, "Verbena").expect("Deve achar a Tradição");
    assert_eq!(loc.section, CompendiumSection::Traditions);
    assert_eq!(loc.item_id, "verbena");

    let loc_pt = resolve_compendium_query(CompendiumSection::Practices, "Ordem de Hermes").expect("Deve achar a Tradição em PT");
    assert_eq!(loc_pt.section, CompendiumSection::Traditions);
    assert_eq!(loc_pt.item_id, "order_of_hermes");
}

#[test]
fn test_technocratic_conventions_texts_and_links() {
    assert_eq!(ALL_CONVENTIONS.len(), 5);
    for c in ALL_CONVENTIONS {
        assert!(c.content(Language::EnUs).is_some());
        assert!(c.content(Language::PtBr).is_some(), "Texto em português deve existir para Convenção {}", c.id);
        for section in ["#### Organization", "#### Initiation", "#### Focus", "#### Stereotypes", "**Source:** M20", "**Fellow Conventions:**"] {
            assert!(c.content.contains(section), "{} sem {}", c.id, section);
        }
        for section in ["#### Organização", "#### Iniciação", "#### Foco", "#### Estereótipos", "**Fonte:** Livro Básico M20", "**Convenções Companheiras:**"] {
            assert!(c.content_pt.contains(section), "{} sem {} em PT", c.id, section);
        }
        for sid in c.affinity_spheres {
            assert!(find_sphere(sid).is_some(), "Esfera de afinidade inexistente em {}: {}", c.id, sid);
        }
        assert!(find_tradition(c.id).is_none(), "Convenção não deve aparecer entre as Tradições: {}", c.id);
        assert_links_resolve(c.id, c.content);
        assert_links_resolve(c.id, c.content_pt);
    }

    let target = CompendiumTarget::from_uri("mta://technocracy/void_engineers").expect("URI deve ser válida");
    assert_eq!(target.section(), CompendiumSection::Traditions);
    assert_eq!(target.label(Language::EnUs), "Void Engineers");
    assert_eq!(target.label(Language::PtBr), "Engenheiros do Vazio");

    assert_eq!(find_convention("NWO").unwrap().id, "new_world_order");
    let loc = resolve_compendium_query(CompendiumSection::Practices, "Sindicato").expect("Deve achar a Convenção");
    assert_eq!(loc.section, CompendiumSection::Traditions);
    assert_eq!(loc.item_id, "syndicate");
}

#[test]
fn test_disparate_sects_texts_and_links() {
    assert_eq!(ALL_DISPARATES.len(), 11);
    for d in ALL_DISPARATES {
        assert!(d.content(Language::EnUs).is_some());
        assert!(d.content(Language::PtBr).is_some(), "Texto em português deve existir para {}", d.id);
        for section in ["#### Organization", "#### Initiation", "#### Focus", "#### Stereotypes", "**Source:** M20", "**The Technocracy:**"] {
            assert!(d.content.contains(section), "{} sem {}", d.id, section);
        }
        for section in ["#### Organização", "#### Iniciação", "#### Foco", "#### Estereótipos", "**Fonte:** Livro Básico M20", "**A Tecnocracia:**"] {
            assert!(d.content_pt.contains(section), "{} sem {} em PT", d.id, section);
        }
        for sid in d.affinity_spheres {
            assert!(find_sphere(sid).is_some(), "Esfera de afinidade inexistente em {}: {}", d.id, sid);
        }
        assert!(find_tradition(d.id).is_none() && find_convention(d.id).is_none(), "Seita fora do grupo certo: {}", d.id);
        assert_links_resolve(d.id, d.content);
        assert_links_resolve(d.id, d.content_pt);
    }

    let target = CompendiumTarget::from_uri("mta://disparates/hollow_ones").expect("URI deve ser válida");
    assert_eq!(target.section(), CompendiumSection::Traditions);
    assert_eq!(target.label(Language::EnUs), "Hollow Ones");
    assert_eq!(target.label(Language::PtBr), "Ocos");

    assert_eq!(find_disparate("Solificati").unwrap().id, "children_of_knowledge");
    assert_eq!(find_faction("Batini").unwrap().id, "ahl_i_batin");
    let loc = resolve_compendium_query(CompendiumSection::Practices, "Wu Lung").expect("Deve achar a seita");
    assert_eq!(loc.section, CompendiumSection::Traditions);
    assert_eq!(loc.item_id, "wu_lung");
}
