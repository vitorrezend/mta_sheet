use mta_sheet::compendium::backgrounds::*;
use mta_sheet::components::compendium::navigation::CompendiumTarget;
use mta_sheet::components::compendium::rich_text::{parse_rich_document, InlineSpan, RichBlock};
use mta_sheet::compendium::archetypes::find_archetype;
use mta_sheet::compendium::attributes::find_attribute;
use mta_sheet::compendium::instruments::{find_instrument, find_theory_article};
use mta_sheet::compendium::practices::find_practice;
use mta_sheet::compendium::spheres::find_sphere;
use mta_sheet::compendium::weapons::find_combat_entity;
use mta_sheet::i18n::Language;

#[test]
fn test_compendium_target_uri_parser() {
    // 1. Antecedentes
    let bg_target = CompendiumTarget::from_uri("mta://backgrounds/allies").expect("Deveria parsear antecedentes");
    assert_eq!(bg_target, CompendiumTarget::Background("allies".to_string()));
    assert_eq!(bg_target.label(Language::PtBr), "Aliados");
    assert_eq!(bg_target.label(Language::EnUs), "Allies");

    let rule_target = CompendiumTarget::from_uri("mta://backgrounds/theory_background_rules").expect("Deveria parsear teoria");
    assert_eq!(rule_target, CompendiumTarget::Background("theory_background_rules".to_string()));

    // 2. Armas e Manobras
    let w_target = CompendiumTarget::from_uri("mta://weapons/weapons/katana").expect("Deveria parsear armas");
    assert_eq!(w_target, CompendiumTarget::Weapon("katana".to_string()));

    let m_target = CompendiumTarget::from_uri("mta://weapons/maneuver/do").expect("Deveria parsear manobras");
    assert_eq!(m_target, CompendiumTarget::Maneuver("do".to_string()));

    // 3. Práticas
    let p_target = CompendiumTarget::from_uri("mta://practices/alchemy").expect("Deveria parsear práticas");
    assert_eq!(p_target, CompendiumTarget::Practice("alchemy".to_string()));

    // 4. Instrumentos
    let i_target = CompendiumTarget::from_uri("mta://instruments/tools_of_focus").expect("Deveria parsear instrumentos");
    assert_eq!(i_target, CompendiumTarget::Instrument("tools_of_focus".to_string()));

    // 5. Atributos
    let a_target = CompendiumTarget::from_uri("mta://attributes/strength").expect("Deveria parsear atributos");
    assert_eq!(a_target, CompendiumTarget::Attribute("strength".to_string()));

    // 6. Arquétipos
    let arc_target = CompendiumTarget::from_uri("mta://archetypes/activist").expect("Deveria parsear arquétipos");
    assert_eq!(arc_target, CompendiumTarget::Archetype("activist".to_string()));

    // 7. Casos inválidos
    assert!(CompendiumTarget::from_uri("http://example.com").is_none());
    assert!(CompendiumTarget::from_uri("mta://").is_none());
    assert!(CompendiumTarget::from_uri("mta://invalid_section").is_none());
}

#[test]
fn test_all_unabridged_background_texts_exist_and_are_bilingual() {
    let all = get_all_backgrounds();
    assert_eq!(all.len(), 33);

    for bg in all {
        let pt_text = find_unabridged_text(bg.id, Language::PtBr);
        assert!(pt_text.is_some(), "Texto integral em PT deve existir para {}", bg.id);
        let pt = pt_text.unwrap();
        assert!(!pt.trim().is_empty(), "Texto PT não pode ser vazio para {}", bg.id);
        assert!(pt.contains("### "), "Texto PT deve conter título markdown para {}", bg.id);

        let en_text = find_unabridged_text(bg.id, Language::EnUs);
        assert!(en_text.is_some(), "Texto integral em EN deve existir para {}", bg.id);
        let en = en_text.unwrap();
        assert!(!en.trim().is_empty(), "Texto EN não pode ser vazio para {}", bg.id);
        assert!(en.contains("### "), "Texto EN deve conter título markdown para {}", bg.id);
    }
}

#[test]
fn test_compendium_cross_links_integrity() {
    // Extrai e valida todos os links mta:// presentes nos textos integrais
    let all = get_all_backgrounds();

    for bg in all {
        for lang in [Language::PtBr, Language::EnUs] {
            if let Some(text) = find_unabridged_text(bg.id, lang) {
                let mut remaining = text;
                while let Some(start) = remaining.find("mta://") {
                    let after = &remaining[start..];
                    let end = after.find(')').unwrap_or(after.len());
                    let uri = &after[..end];

                    let target = CompendiumTarget::from_uri(uri)
                        .expect(&format!("URI inválida encontrada em {} ({:?}): {}", bg.id, lang, uri));

                    // Validação de existência real do alvo
                    match &target {
                        CompendiumTarget::Background(id) => {
                            let exists = id == "theory_background_rules" || find_background(id).is_some();
                            assert!(exists, "Link para antecedente inexistente em {} ({:?}): {}", bg.id, lang, id);
                        }
                        CompendiumTarget::Weapon(id) | CompendiumTarget::Maneuver(id) => {
                            let exists = find_combat_entity(id).is_some();
                            assert!(exists, "Link para combate/arma/manobra inexistente em {} ({:?}): {}", bg.id, lang, id);
                        }
                        CompendiumTarget::Practice(id) => {
                            let exists = find_practice(id).is_some();
                            assert!(exists, "Link para prática inexistente em {} ({:?}): {}", bg.id, lang, id);
                        }
                        CompendiumTarget::Instrument(id) => {
                            let exists = find_instrument(id).is_some() || find_theory_article(id).is_some();
                            assert!(exists, "Link para instrumento inexistente em {} ({:?}): {}", bg.id, lang, id);
                        }
                        CompendiumTarget::Attribute(id) => {
                            let exists = id == "rule_specialties" || find_attribute(id).is_some();
                            assert!(exists, "Link para atributo inexistente em {} ({:?}): {}", bg.id, lang, id);
                        }
                        CompendiumTarget::Archetype(id) => {
                            let exists = id == "theory_nature_demeanor" || find_archetype(id).is_some();
                            assert!(exists, "Link para arquétipo inexistente em {} ({:?}): {}", bg.id, lang, id);
                        }
                        CompendiumTarget::Sphere(id) => {
                            let exists = id == "theory_sphere_rules" || find_sphere(id).is_some();
                            assert!(exists, "Link para esfera inexistente em {} ({:?}): {}", bg.id, lang, id);
                        }
                    }

                    // Valida que o label retorna texto legível
                    assert!(!target.label(lang).is_empty());

                    remaining = &after[end..];
                }
            }
        }
    }
}

#[test]
fn test_rich_document_parser_blocks() {
    let doc = "### Título Principal\n\n\
               Este é um parágrafo com **negrito** e *itálico* e [Link](mta://backgrounds/allies).\n\n\
               > Uma citação de regras importante.\n\n\
               - Item 1\n\
               - Item 2";

    let blocks = parse_rich_document(doc);
    assert_eq!(blocks.len(), 4);

    match &blocks[0] {
        RichBlock::Heading { text, .. } => assert_eq!(text, "Título Principal"),
        _ => panic!("Esperado Heading"),
    }

    match &blocks[1] {
        RichBlock::Paragraph(spans) => {
            assert!(spans.iter().any(|s| matches!(s, InlineSpan::Bold(b) if b == "negrito")));
            assert!(spans.iter().any(|s| matches!(s, InlineSpan::Italic(i) if i == "itálico")));
            assert!(spans.iter().any(|s| matches!(s, InlineSpan::Link { url, is_internal, .. } if url == "mta://backgrounds/allies" && *is_internal)));
        }
        _ => panic!("Esperado Paragraph"),
    }

    match &blocks[2] {
        RichBlock::Quote(_) => {}
        _ => panic!("Esperado Quote"),
    }

    match &blocks[3] {
        RichBlock::BulletList(items) => assert_eq!(items.len(), 2),
        _ => panic!("Esperado BulletList"),
    }
}
