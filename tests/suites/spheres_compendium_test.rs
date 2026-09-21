//! Testes Automatizados do Compêndio Canônico de Esferas (M20).
//!
//! Valida contagem (13), unicidade de IDs, carregamento integral dos arquivos Markdown bilíngues,
//! navegação pelo protocolo `mta://spheres/<id>` e busca inteligente com tolerância a acentos.

use mta_sheet::compendium::spheres::{
    find_sphere, find_unabridged_text, get_all_spheres, ALL_SPHERES,
};
use mta_sheet::components::compendium::navigation::CompendiumTarget;
use mta_sheet::components::compendium::CompendiumSection;
use mta_sheet::i18n::Language;
use std::collections::HashSet;

#[test]
fn test_all_spheres_count_and_uniqueness() {
    assert_eq!(
        ALL_SPHERES.len(),
        13,
        "M20 deve conter exatamente 13 esferas e variantes no catálogo canônico completo"
    );

    let mut ids = HashSet::new();
    for s in ALL_SPHERES {
        assert!(ids.insert(s.id), "ID duplicado encontrado no catálogo de esferas: {}", s.id);
        assert!(
            s.page_ref.starts_with("M20, p"),
            "Referência de página inválida para {}: {}",
            s.name,
            s.page_ref
        );
        assert!(!s.name.is_empty(), "Nome em EN não pode ser vazio para {}", s.id);
        assert!(!s.name_pt.is_empty(), "Nome em PT não pode ser vazio para {}", s.id);
        assert!(!s.subtitle.is_empty(), "Subtítulo em EN não pode ser vazio para {}", s.id);
        assert!(!s.subtitle_pt.is_empty(), "Subtítulo em PT não pode ser vazio para {}", s.id);
        assert!(!s.specialties.is_empty(), "Especializações em EN não podem ser vazias para {}", s.id);
        assert!(!s.specialties_pt.is_empty(), "Especializações em PT não podem ser vazias para {}", s.id);
        assert!(!s.description.is_empty(), "Descrição em EN não pode ser vazia para {}", s.id);
        assert!(!s.description_pt.is_empty(), "Descrição em PT não pode ser vazia para {}", s.id);

        if !s.is_optional_rule {
            assert_eq!(
                s.ranks.len(),
                5,
                "A esfera {} deve conter exatamente 5 postos canônicos",
                s.name
            );
            for (i, r) in s.ranks.iter().enumerate() {
                assert_eq!(r.rank, (i + 1) as i32, "Graduação do posto incorreta para {}", s.name);
                assert!(!r.name.is_empty(), "Nome do posto em EN vazio para {} {}", s.name, r.rank);
                assert!(!r.name_pt.is_empty(), "Nome do posto em PT vazio para {} {}", s.name, r.rank);
                assert!(!r.description.is_empty(), "Descrição do posto em EN vazia para {} {}", s.name, r.rank);
                assert!(!r.description_pt.is_empty(), "Descrição do posto em PT vazia para {} {}", s.name, r.rank);
            }
        }
    }
}

#[test]
fn test_all_spheres_unabridged_markdown_files_load_and_not_empty() {
    for s in get_all_spheres() {
        let pt_text = find_unabridged_text(s.id, Language::PtBr);
        assert!(
            pt_text.is_some(),
            "Texto integral em PT-BR não encontrado para a esfera {}",
            s.id
        );
        let pt = pt_text.unwrap();
        assert!(
            pt.len() > 200,
            "Texto integral em PT-BR muito curto para {}: {} bytes",
            s.id,
            pt.len()
        );

        let en_text = find_unabridged_text(s.id, Language::EnUs);
        assert!(
            en_text.is_some(),
            "Texto integral em EN-US não encontrado para a esfera {}",
            s.id
        );
        let en = en_text.unwrap();
        assert!(
            en.len() > 200,
            "Texto integral em EN-US muito curto para {}: {} bytes",
            s.id,
            en.len()
        );
    }
}

#[test]
fn test_find_sphere_bilingual_and_variants() {
    // 9 Esferas Tradicionais em Inglês
    assert_eq!(find_sphere("Correspondence").unwrap().id, "correspondence");
    assert_eq!(find_sphere("Entropy").unwrap().id, "entropy");
    assert_eq!(find_sphere("Forces").unwrap().id, "forces");
    assert_eq!(find_sphere("Life").unwrap().id, "life");
    assert_eq!(find_sphere("Matter").unwrap().id, "matter");
    assert_eq!(find_sphere("Mind").unwrap().id, "mind");
    assert_eq!(find_sphere("Prime").unwrap().id, "prime");
    assert_eq!(find_sphere("Spirit").unwrap().id, "spirit");
    assert_eq!(find_sphere("Time").unwrap().id, "time");

    // 9 Esferas Tradicionais em Português com e sem acentos
    assert_eq!(find_sphere("Correspondência").unwrap().id, "correspondence");
    assert_eq!(find_sphere("correspondencia").unwrap().id, "correspondence");
    assert_eq!(find_sphere("Entropia").unwrap().id, "entropy");
    assert_eq!(find_sphere("Forças").unwrap().id, "forces");
    assert_eq!(find_sphere("forcas").unwrap().id, "forces");
    assert_eq!(find_sphere("Vida").unwrap().id, "life");
    assert_eq!(find_sphere("Matéria").unwrap().id, "matter");
    assert_eq!(find_sphere("materia").unwrap().id, "matter");
    assert_eq!(find_sphere("Mente").unwrap().id, "mind");
    assert_eq!(find_sphere("Primórdio").unwrap().id, "prime");
    assert_eq!(find_sphere("primordio").unwrap().id, "prime");
    assert_eq!(find_sphere("Espírito").unwrap().id, "spirit");
    assert_eq!(find_sphere("espirito").unwrap().id, "spirit");
    assert_eq!(find_sphere("Tempo").unwrap().id, "time");

    // 3 Variantes da Tecnocracia (EN e PT)
    assert_eq!(find_sphere("Data").unwrap().id, "data");
    assert_eq!(find_sphere("Dados").unwrap().id, "data");
    assert_eq!(find_sphere("Dimensional Science").unwrap().id, "dimensional_science");
    assert_eq!(find_sphere("Ciência Dimensional").unwrap().id, "dimensional_science");
    assert_eq!(find_sphere("ciencia dimensional").unwrap().id, "dimensional_science");
    assert_eq!(find_sphere("Primal Utility").unwrap().id, "primal_utility");
    assert_eq!(find_sphere("Utilidade Primordial").unwrap().id, "primal_utility");

    // 1 Regra Opcional
    assert_eq!(find_sphere("Wild Talent").unwrap().id, "wild_talent");
    assert_eq!(find_sphere("Talento Selvagem").unwrap().id, "wild_talent");
}

#[test]
fn test_mta_protocol_navigation_spheres() {
    let target1 = CompendiumTarget::from_uri("mta://spheres/forces").expect("URI deve ser válida");
    assert_eq!(target1, CompendiumTarget::Sphere("forces".to_string()));
    assert_eq!(target1.section(), CompendiumSection::Spheres);
    assert_eq!(target1.label(Language::PtBr), "Forças");
    assert_eq!(target1.label(Language::EnUs), "Forces");

    let target2 = CompendiumTarget::from_uri("mta://esferas/correspondence").expect("URI deve ser válida");
    assert_eq!(target2, CompendiumTarget::Sphere("correspondence".to_string()));
    assert_eq!(target2.label(Language::PtBr), "Correspondência");
    assert_eq!(target2.label(Language::EnUs), "Correspondence");

    let target3 = CompendiumTarget::from_uri("mta://sphere/primal_utility").expect("URI deve ser válida");
    assert_eq!(target3, CompendiumTarget::Sphere("primal_utility".to_string()));
    assert_eq!(target3.label(Language::PtBr), "Utilidade Primordial");
    assert_eq!(target3.label(Language::EnUs), "Primal Utility");
}

#[test]
fn test_sphere_theory_rules_loaded_and_valid() {
    use mta_sheet::compendium::spheres::SPHERE_THEORY_RULES;

    assert_eq!(SPHERE_THEORY_RULES.id, "theory_sphere_rules");
    assert_eq!(SPHERE_THEORY_RULES.page_ref, "M20, pp. 511-512");
    assert!(!SPHERE_THEORY_RULES.title(Language::PtBr).is_empty());
    assert!(!SPHERE_THEORY_RULES.title(Language::EnUs).is_empty());
    assert!(SPHERE_THEORY_RULES.content(Language::PtBr).len() > 500);
    assert!(SPHERE_THEORY_RULES.content(Language::EnUs).len() > 500);

    let pt_unabridged = find_unabridged_text("theory_sphere_rules", Language::PtBr);
    assert!(pt_unabridged.is_some(), "Texto integral PT-BR de theory_sphere_rules deve existir");
    assert!(pt_unabridged.unwrap().len() > 500);

    let en_unabridged = find_unabridged_text("theory_sphere_rules", Language::EnUs);
    assert!(en_unabridged.is_some(), "Texto integral EN-US de theory_sphere_rules deve existir");
    assert!(en_unabridged.unwrap().len() > 500);

    // Teste de navegação pelo protocolo mta://
    let target = CompendiumTarget::from_uri("mta://spheres/theory_sphere_rules").expect("URI de regras deve ser válida");
    assert_eq!(target, CompendiumTarget::Sphere("theory_sphere_rules".to_string()));
    assert_eq!(target.label(Language::PtBr), "Regras Gerais das Esferas");
    assert_eq!(target.label(Language::EnUs), "General Sphere Rules");
}

#[test]
fn test_sphere_canonical_page_references() {
    assert_eq!(find_sphere("correspondence").unwrap().page_ref, "M20, pp. 512-513");
    assert_eq!(find_sphere("entropy").unwrap().page_ref, "M20, pp. 514-515");
    assert_eq!(find_sphere("forces").unwrap().page_ref, "M20, pp. 515-516");
    assert_eq!(find_sphere("life").unwrap().page_ref, "M20, pp. 516-517");
    assert_eq!(find_sphere("matter").unwrap().page_ref, "M20, pp. 517-519");
    assert_eq!(find_sphere("mind").unwrap().page_ref, "M20, pp. 519-520");
    assert_eq!(find_sphere("prime").unwrap().page_ref, "M20, pp. 520-521");
    assert_eq!(find_sphere("spirit").unwrap().page_ref, "M20, pp. 521-522");
    assert_eq!(find_sphere("time").unwrap().page_ref, "M20, pp. 522-523");
    assert_eq!(find_sphere("data").unwrap().page_ref, "M20, pp. 524-525");
    assert_eq!(find_sphere("dimensional_science").unwrap().page_ref, "M20, pp. 525-526");
    assert_eq!(find_sphere("primal_utility").unwrap().page_ref, "M20, pp. 526-527");
    assert_eq!(find_sphere("wild_talent").unwrap().page_ref, "M20, pp. 527-528");
}

#[test]
fn test_sphere_suggested_specialties() {
    use mta_sheet::compendium::spheres::get_suggested_specialties;

    // Correspondência / Correspondence
    let corr_pt = get_suggested_specialties("correspondence", Language::PtBr);
    assert!(!corr_pt.is_empty(), "Especialidades de Correspondência em PT não podem ser vazias");
    assert!(corr_pt.contains(&"Teletransporte"));

    let corr_en = get_suggested_specialties("Correspondence", Language::EnUs);
    assert!(!corr_en.is_empty(), "Especialidades de Correspondence em EN não podem ser vazias");
    assert!(corr_en.contains(&"Teleportation"));

    // Forças / Forces
    let forces_pt = get_suggested_specialties("Forças", Language::PtBr);
    assert!(forces_pt.contains(&"Eletricidade"));
    assert!(forces_pt.contains(&"Fogo"));

    let forces_en = get_suggested_specialties("forces", Language::EnUs);
    assert!(forces_en.contains(&"Electricity"));
    assert!(forces_en.contains(&"Fire"));

    // Vida / Life
    let life_pt = get_suggested_specialties("Vida", Language::PtBr);
    assert!(life_pt.contains(&"Clonagem"));
    assert!(life_pt.contains(&"Cura"));

    // Dados / Data
    let data_pt = get_suggested_specialties("Dados", Language::PtBr);
    assert!(data_pt.contains(&"Criptografia"));

    // Esfera inexistente retorna vazio
    let none_specs = get_suggested_specialties("non_existent_sphere", Language::PtBr);
    assert!(none_specs.is_empty());
}

#[test]
fn test_technocratic_sphere_specialty_flipping() {
    use mta_sheet::compendium::spheres::get_suggested_specialties;

    // Correspondência vs Dados
    let corr = get_suggested_specialties("Correspondência", Language::PtBr);
    let data = get_suggested_specialties("Dados", Language::PtBr);
    assert!(corr.contains(&"Teletransporte"));
    assert!(!corr.contains(&"Criptografia"));
    assert!(data.contains(&"Criptografia"));
    assert!(!data.contains(&"Teletransporte"));

    // Primórdio vs Utilidade Primordial
    let prime = get_suggested_specialties("Primórdio", Language::PtBr);
    let primal_util = get_suggested_specialties("Utilidade Primordial", Language::PtBr);
    assert!(prime.contains(&"Canalização"));
    assert!(!prime.contains(&"Geração de Capital Energético"));
    assert!(primal_util.contains(&"Geração de Capital Energético"));
    assert!(!primal_util.contains(&"Canalização"));

    // Espírito vs Ciência Dimensional
    let spirit = get_suggested_specialties("Espírito", Language::PtBr);
    let dim_sci = get_suggested_specialties("Ciência Dimensional", Language::PtBr);
    assert!(spirit.contains(&"Viagem Umbral"));
    assert!(!spirit.contains(&"Relações com EDEs"));
    assert!(dim_sci.contains(&"Relações com EDEs"));
    assert!(!dim_sci.contains(&"Viagem Umbral"));
}

#[test]
fn test_rich_document_rank_headings_and_clean_paragraphs() {
    use mta_sheet::components::compendium::rich_text::{parse_rich_document, RichBlock};

    let sample_md = r#"### Correspondência (Correspondence)
*Conexões e Dimensões*

> **Especializações:** Teletransporte

---

#### • Percepções Espaciais Imediatas / Paisagem da Mente
A compreensão espacial básica permite ao mago pressentir eventos em sua vizinhança.

#### •• Sentir, Tocar, Espessar & Alcançar Através do Espaço
O mago agora estende seus sentidos através do espaço intermediário.
"#;

    let blocks = parse_rich_document(sample_md);

    // Deve conter Heading, Quote, Divider, RankHeading(•), Paragraph, RankHeading(••), Paragraph
    assert!(blocks.len() >= 6, "Esperava pelo menos 6 blocos estruturados, obteve {}", blocks.len());

    // Verifica que o primeiro RankHeading foi detectado
    let rank1 = blocks.iter().find(|b| matches!(b, RichBlock::RankHeading { dots, .. } if dots == "•"));
    assert!(rank1.is_some(), "RankHeading com '•' deve ser detectado");
    if let Some(RichBlock::RankHeading { dots, title }) = rank1 {
        assert_eq!(dots, "•");
        assert_eq!(title, "Percepções Espaciais Imediatas / Paisagem da Mente");
        assert!(!title.contains('#'), "Título não pode conter hashes #");
    }

    // Verifica que o segundo RankHeading foi detectado
    let rank2 = blocks.iter().find(|b| matches!(b, RichBlock::RankHeading { dots, .. } if dots == "••"));
    assert!(rank2.is_some(), "RankHeading com '••' deve ser detectado");
    if let Some(RichBlock::RankHeading { dots, title }) = rank2 {
        assert_eq!(dots, "••");
        assert_eq!(title, "Sentir, Tocar, Espessar & Alcançar Através do Espaço");
    }

    // Verifica que nenhum bloco contém marcas brutas de cabeçalho nos textos
    for block in &blocks {
        match block {
            RichBlock::Heading { text, .. } => assert!(!text.starts_with('#')),
            RichBlock::RankHeading { dots, title } => {
                assert!(!dots.starts_with('#'));
                assert!(!title.starts_with('#'));
            }
            RichBlock::Paragraph(spans) => {
                for span in spans {
                    if let mta_sheet::components::compendium::rich_text::InlineSpan::Text(t) = span {
                        assert!(!t.starts_with("####"), "Parágrafo não deve começar com #### bruto: {}", t);
                    }
                }
            }
            _ => {}
        }
    }
}

#[test]
fn test_technocratic_sphere_variant_swap_sheet_sync_and_costs() {
    use mta_sheet::state::{CharacterData, DotOrigin};

    let mut sheet = CharacterData::new("test_swap".to_string(), "Hermético".to_string());

    // 1. Configura Correspondência com 2 pontos (1 Base, 1 Bônus) e marca como Esfera de Afinidade
    sheet.set_attribute_with_origin("Correspondência", Some(2), Some("Teletransporte".to_string()), DotOrigin::Base);
    sheet.set_attribute_dot_origin("Correspondência", 1, DotOrigin::Bonus);
    sheet.set_affinity_sphere(Some("Correspondência".to_string()));

    assert_eq!(sheet.get_attribute_level("Correspondência", 0), 2);
    assert_eq!(sheet.get_affinity_sphere(), Some("Correspondência".to_string()));
    assert_eq!(sheet.get_active_spheres()[0], "correspondence");

    // 2. Alterna para a variante Tecnocrática "data" (Dados)
    let swapped = sheet.swap_sphere_variant("data");
    assert!(swapped, "swap_sphere_variant deve retornar true para 'data'");

    // Atributo anterior deve ter sido removido e migrado para "Dados"
    assert!(!sheet.attributes.contains_key("Correspondência"));
    assert!(sheet.attributes.contains_key("Dados"));
    assert_eq!(sheet.get_attribute_level("Dados", 0), 2);
    assert_eq!(sheet.get_attribute_modifier("Dados"), "Teletransporte");
    assert_eq!(sheet.labels.get("sphere_slot_correspondence").map(|s| s.as_str()), Some("Dados"));

    // Afinidade deve ter sido atualizada para "Dados"
    assert_eq!(sheet.get_affinity_sphere(), Some("Dados".to_string()));

    // Active spheres deve listar "data"
    assert_eq!(sheet.get_active_spheres()[0], "data");

    // Summary deve refletir "Dados"
    let summary = sheet.to_summary("now".to_string(), false, true);
    let data_sph = summary.spheres.iter().find(|(s, _)| s == "Dados");
    assert!(data_sph.is_some(), "Summary deve conter a esfera 'Dados'");
    assert_eq!(data_sph.unwrap().1, 2);

    // Custos devem calcular sobre "Dados" com afinidade (1 dot bônus = 7 pts)
    let costs = sheet.calculate_costs();
    assert_eq!(costs.total_bonus_spent, 7);
    assert!(costs.items.iter().any(|item| item.name == "Dados"));

    // 3. Alterna de volta para "correspondence"
    let swapped_back = sheet.swap_sphere_variant("correspondence");
    assert!(swapped_back, "swap_sphere_variant deve retornar true para 'correspondence'");

    assert!(!sheet.attributes.contains_key("Dados"));
    assert!(sheet.attributes.contains_key("Correspondência"));
    assert_eq!(sheet.get_attribute_level("Correspondência", 0), 2);
    assert_eq!(sheet.get_attribute_modifier("Correspondência"), "Teletransporte");
    assert_eq!(sheet.labels.get("sphere_slot_correspondence").map(|s| s.as_str()), Some("Correspondência"));
    assert_eq!(sheet.get_affinity_sphere(), Some("Correspondência".to_string()));
    assert_eq!(sheet.get_active_spheres()[0], "correspondence");

    // 4. Testa os outros dois pares (Primórdio <-> Utilidade Primordial, Espírito <-> Ciência Dimensional)
    sheet.set_attribute("Primórdio", Some(3), None);
    sheet.swap_sphere_variant("primal_utility");
    assert_eq!(sheet.get_attribute_level("Utilidade Primordial", 0), 3);
    assert_eq!(sheet.get_active_spheres()[6], "primal_utility");

    sheet.set_attribute("Espírito", Some(4), None);
    sheet.swap_sphere_variant("dimensional_science");
    assert_eq!(sheet.get_attribute_level("Ciência Dimensional", 0), 4);
    assert_eq!(sheet.get_active_spheres()[7], "dimensional_science");
}



