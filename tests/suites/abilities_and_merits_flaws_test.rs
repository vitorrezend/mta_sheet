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
fn test_book_of_secrets_physical_and_mental_merits_flaws() {
    use mta_sheet::compendium::merits_flaws::{find_merit_flaw_unabridged_text, BOS_FLAWS, BOS_MERITS};

    // BoS pp. 35-96: Físicos, Mentais, Sociais e Sobrenaturais
    // (Sterile, Cyclic Magick e Spirit Magnet contam como Qualidade e como Defeito)
    assert_eq!(BOS_MERITS.len(), 101);
    assert_eq!(BOS_FLAWS.len(), 126);
    let mut ids = std::collections::HashSet::new();
    for mf in ALL_MERITS_FLAWS.iter() {
        assert!(ids.insert(mf.id), "ID repetido: {}", mf.id);
    }
    for mf in BOS_MERITS.iter().chain(BOS_FLAWS.iter()) {
        assert!(mf.page_ref.starts_with("BoS, p"), "Página sem BoS em {}", mf.id);
        // Texto integral = descrição do livro com os links automáticos entre verbetes
        let base = mf.id.strip_suffix("_flaw").unwrap_or(mf.id);
        let paired = format!("{base}_flaw");
        let self_ids = [mf.id, base, paired.as_str()];
        for lang in [Language::PtBr, Language::EnUs] {
            let expected = mta_sheet::compendium::merits_flaws::autolink::autolink(mf.description(lang), &self_ids);
            assert_eq!(find_merit_flaw_unabridged_text(mf.id, lang), Some(expected.as_str()), "Texto integral de {}", mf.id);
        }
        assert!(!mf.name(Language::PtBr).trim().is_empty(), "Nome PT ausente em {}", mf.id);
        assert_ne!(mf.name(Language::PtBr), "", "Nome PT vazio em {}", mf.id);
        assert!(!mf.description(Language::PtBr).trim().is_empty(), "Descrição PT ausente em {}", mf.id);
        assert!(!mf.points_display(Language::PtBr).trim().is_empty(), "Pontos PT ausente em {}", mf.id);
    }
    // Os que o livro só remete ao M20 não se repetem
    assert_eq!(find_merit_flaw("acute_senses").unwrap().page_ref, "M20, pp. 642-643");
    assert_eq!(find_merit_flaw("Iron Will").unwrap().available_costs, &[3]);
    assert_eq!(find_merit_flaw("Vontade de Ferro").unwrap().available_costs, &[3]);
    assert_eq!(find_merit_flaw("aging").unwrap().available_costs, &[2, 4, 6, 8, 10]);
    assert_eq!(find_merit_flaw("Envelhecimento").unwrap().available_costs, &[2, 4, 6, 8, 10]);
    assert_eq!(find_merit_flaw("sterile").unwrap().trait_type, TraitType::Merit);
    assert_eq!(find_merit_flaw("sterile_flaw").unwrap().trait_type, TraitType::Flaw);
    // O Defeito Amnesia não colide com a Perturbação de mesmo nome
    assert_eq!(find_merit_flaw("amnesia_flaw").unwrap().page_ref, "BoS, p. 48");
    assert!(find_derangement("amnesia").is_some());
    // Sociais: os que remetem ao M20 (Dark Triad, Ties, Enemy) continuam com a página do M20
    assert_eq!(find_merit_flaw("enemy").unwrap().page_ref, "M20, p. 647");
    assert_eq!(find_merit_flaw("hideaway_safehouse").unwrap().available_costs, &[2, 4, 6]);
    assert_eq!(find_merit_flaw("infamy").unwrap().page_ref, "BoS, pp. 62-63");
    assert_eq!(find_merit_flaw("ward").unwrap().page_ref, "BoS, p. 66");
    assert_eq!(find_merit_flaw("naive").unwrap().name, "Naïve");
    assert_eq!(find_merit_flaw("Lealdade").unwrap().id, "loyalty");
    assert_eq!(find_merit_flaw("Amor Verdadeiro").unwrap().id, "true_love");
    assert_eq!(find_merit_flaw("Na Lista Negra").unwrap().id, "blacklisted");
    assert_eq!(find_merit_flaw("Mago Renegado").unwrap().id, "rogue");
    // Sobrenaturais: os que remetem ao M20 continuam com a página do M20
    assert_eq!(find_merit_flaw("true_faith").unwrap().page_ref, "M20, pp. 644-646");
    assert_eq!(find_merit_flaw("spirit_magnet").unwrap().available_costs, &[3, 4, 5, 6, 7]);
    assert_eq!(find_merit_flaw("spirit_magnet_flaw").unwrap().trait_type, TraitType::Flaw);
    assert_eq!(find_merit_flaw("cyclic_magick_flaw").unwrap().page_ref, "BoS, p. 70");
    assert_eq!(find_merit_flaw("immortal").unwrap().available_costs, &[5, 7]);
    assert_eq!(find_merit_flaw("clear_sighted").unwrap().page_ref, "BoS, pp. 75-76");
    // Defeitos Sobrenaturais (os que remetem ao M20 continuam com a página do M20)
    assert_eq!(find_merit_flaw("cursed").unwrap().page_ref, "M20, p. 646");
    assert_eq!(find_merit_flaw("jinx_infernal_contraption").unwrap().available_costs, &[2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(find_merit_flaw("dark_fate").unwrap().trait_type, TraitType::Flaw);
    assert_eq!(find_merit_flaw("phylactery").unwrap().page_ref, "BoS, pp. 94-96");
    assert_eq!(find_merit_flaw("Filactério").unwrap().id, "phylactery");
    assert_eq!(find_merit_flaw("Fera Interior").unwrap().id, "beast_within");
    // Links automáticos entre verbetes (o livro cita "the Flaw: Echoes", "Antecedente: Maravilha"…)
    let bound = find_merit_flaw_unabridged_text("beast_within", Language::EnUs).unwrap();
    assert!(bound.contains("[Feral Mind](mta://merits_flaws/feral_mind)"));
    let jinx_pt = find_merit_flaw_unabridged_text("jinx_infernal_contraption", Language::PtBr).unwrap();
    assert!(jinx_pt.contains("[Maravilha](mta://backgrounds/wonder)"));
    // Sobrenaturais traduzidos: nome, custo e texto integral em português
    assert_eq!(find_merit_flaw("Esfera Natural").unwrap().id, "sphere_natural");
    assert_eq!(find_merit_flaw("Magia Cíclica").unwrap().trait_type, TraitType::Merit);
    assert_eq!(find_merit_flaw("cyclic_magick_flaw").unwrap().name(Language::PtBr), "Magia Cíclica");
    assert_eq!(find_merit_flaw("spirit_magnet").unwrap().points_display(Language::PtBr), "3 a 7 pts.");
    for mf in BOS_MERITS.iter().chain(BOS_FLAWS.iter()).filter(|m| m.category == TraitCategory::Supernatural) {
        assert!(!mf.name_pt.is_empty(), "Nome PT ausente em {}", mf.id);
        assert!(!mf.description_pt.trim().is_empty(), "Texto PT ausente em {}", mf.id);
        assert_ne!(mf.description_pt, mf.description, "Texto PT igual ao inglês em {}", mf.id);
    }
}

#[test]
fn test_book_of_secrets_secondary_abilities() {
    use mta_sheet::compendium::abilities::{find_ability_unabridged_text, BOS_SECONDARY_ABILITIES};

    // BoS pp. 18-34: 11 Talentos, 23 Perícias e 11 Conhecimentos
    assert_eq!(BOS_SECONDARY_ABILITIES.len(), 45);
    let count = |c: AbilityCategory| BOS_SECONDARY_ABILITIES.iter().filter(|a| a.category == c).count();
    assert_eq!(count(AbilityCategory::Talents), 11);
    assert_eq!(count(AbilityCategory::Skills), 23);
    assert_eq!(count(AbilityCategory::Knowledges), 11);

    let mut ids = std::collections::HashSet::new();
    for a in ALL_ABILITIES.iter() {
        assert!(ids.insert(a.id), "ID de Habilidade repetido: {}", a.id);
    }
    for a in BOS_SECONDARY_ABILITIES.iter() {
        assert_eq!(a.scope, AbilityScope::Secondary);
        assert!(a.page_ref.starts_with("BoS, p"), "Página sem BoS em {}", a.id);
        assert_eq!(a.ratings.len(), 5, "{} deve ter 5 níveis", a.id);
        assert!(!a.name_pt.is_empty() && !a.possessed_by_pt.is_empty(), "Tradução ausente em {}", a.id);
        assert_eq!(a.suggested_specialties.len(), a.suggested_specialties_pt.len(), "Especializações EN/PT de {}", a.id);
        for lang in [Language::EnUs, Language::PtBr] {
            let text = find_ability_unabridged_text(a.id, lang).expect("texto integral");
            assert!(!text.trim().is_empty(), "Texto vazio em {}", a.id);
        }
        assert_ne!(a.description(Language::PtBr), a.description(Language::EnUs), "PT igual ao EN em {}", a.id);
    }
    // Estilo e Pilotagem agora vêm do livro, com a página certa
    assert_eq!(find_ability("style").unwrap().page_ref, "BoS, p. 22");
    assert_eq!(find_ability("Pilotagem").unwrap().page_ref, "BoS, p. 28");
    assert_eq!(find_ability("Culinária").unwrap().id, "cooking");
    assert_eq!(find_ability("fortune_telling_assessment_analysis").unwrap().ratings[4].title, "Master");
    assert_eq!(find_ability("history").unwrap().ratings[0].title_pt, "Curioso");
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


#[test]
fn test_abilities_unabridged_texts_and_links() {
    use mta_sheet::compendium::abilities::find_ability_unabridged_text;
    use mta_sheet::compendium::attributes::find_attribute;
    use mta_sheet::compendium::backgrounds::find_background;
    use mta_sheet::components::compendium::navigation::CompendiumTarget;

    // Toda habilidade do M20 (pp. 275-300) tem texto integral em inglês; Style e Pilot não estão no livro
    for ab in ALL_ABILITIES.iter().filter(|a| a.id != "style" && a.id != "pilot") {
        let en = find_ability_unabridged_text(ab.id, Language::EnUs);
        assert!(en.is_some_and(|t| !t.trim().is_empty()), "Texto integral EN ausente para {}", ab.id);
        let pt = find_ability_unabridged_text(ab.id, Language::PtBr);
        assert!(pt.is_some_and(|t| !t.trim().is_empty()), "Texto integral PT ausente para {}", ab.id);
    }

    let texts = ALL_ABILITIES
        .iter()
        .flat_map(|a| [
            (a.id, find_ability_unabridged_text(a.id, Language::EnUs)),
            (a.id, find_ability_unabridged_text(a.id, Language::PtBr)),
        ])
        .filter_map(|(id, opt)| opt.map(|t| (id, t)))
        .chain(ABILITY_THEORY_RULES.iter().flat_map(|r| [(r.id, r.content(Language::EnUs)), (r.id, r.content(Language::PtBr))]));

    for (owner, text) in texts {
        let mut remaining = text;
        while let Some(start) = remaining.find("mta://") {
            let after = &remaining[start..];
            let end = after.find(')').unwrap_or(after.len());
            let uri = &after[..end];
            let exists = match CompendiumTarget::from_uri(uri) {
                Some(CompendiumTarget::Ability(id)) => find_ability(&id).is_some() || find_ability_theory_rule(&id).is_some(),
                Some(CompendiumTarget::Background(id)) => find_background(&id).is_some(),
                Some(CompendiumTarget::Attribute(id)) => find_attribute(&id).is_some(),
                _ => false,
            };
            assert!(exists, "Link inválido em {}: {}", owner, uri);
            remaining = &after[end..];
        }
    }
}


#[test]
fn test_merits_flaws_unabridged_texts_and_links() {
    use mta_sheet::compendium::abilities::find_ability;
    use mta_sheet::compendium::attributes::find_attribute;
    use mta_sheet::compendium::backgrounds::find_background;
    use mta_sheet::compendium::merits_flaws::{
        find_derangement_unabridged_text, find_merit_flaw_article, find_merit_flaw_unabridged_text, MERIT_FLAW_ARTICLES,
    };
    use mta_sheet::components::compendium::navigation::CompendiumTarget;

    // Toda Qualidade, Defeito e Perturbação do Apêndice II tem texto integral bilíngue (EN e PT-BR)
    for mf in ALL_MERITS_FLAWS.iter() {
        let en = find_merit_flaw_unabridged_text(mf.id, Language::EnUs);
        assert!(en.is_some_and(|t| !t.trim().is_empty()), "Texto integral EN ausente para {}", mf.id);
        let pt = find_merit_flaw_unabridged_text(mf.id, Language::PtBr);
        assert!(pt.is_some_and(|t| !t.trim().is_empty()), "Texto integral PT ausente para {}", mf.id);
    }
    for d in ALL_DERANGEMENTS.iter() {
        let en = find_derangement_unabridged_text(d.id, Language::EnUs);
        assert!(en.is_some_and(|t| !t.trim().is_empty()), "Texto integral EN ausente para {}", d.id);
        let pt = find_derangement_unabridged_text(d.id, Language::PtBr);
        assert!(pt.is_some_and(|t| !t.trim().is_empty()), "Texto integral PT ausente para {}", d.id);
    }
    // 4 quadros do M20 (Apêndice II) + 6 do Book of Secrets
    assert_eq!(MERIT_FLAW_ARTICLES.len(), 10);
    for art in MERIT_FLAW_ARTICLES.iter() {
        assert!(!art.content.trim().is_empty(), "Conteúdo EN ausente para artigo {}", art.id);
        // Quadros ainda sem tradução (title_pt vazio) caem no inglês
        if !art.title_pt.is_empty() {
            assert!(!art.content_pt.trim().is_empty(), "Conteúdo PT ausente para artigo {}", art.id);
        }
        assert!(!art.title(Language::PtBr).trim().is_empty(), "Título PT ausente para artigo {}", art.id);
        assert!(!art.title(Language::EnUs).trim().is_empty(), "Título EN ausente para artigo {}", art.id);
    }

    let texts = ALL_MERITS_FLAWS
        .iter()
        .filter_map(|m| find_merit_flaw_unabridged_text(m.id, Language::EnUs).map(|t| (m.id, t)))
        .chain(ALL_MERITS_FLAWS.iter().filter_map(|m| find_merit_flaw_unabridged_text(m.id, Language::PtBr).map(|t| (m.id, t))))
        .chain(ALL_DERANGEMENTS.iter().filter_map(|d| find_derangement_unabridged_text(d.id, Language::EnUs).map(|t| (d.id, t))))
        .chain(ALL_DERANGEMENTS.iter().filter_map(|d| find_derangement_unabridged_text(d.id, Language::PtBr).map(|t| (d.id, t))))
        .chain(MERIT_FLAW_ARTICLES.iter().map(|a| (a.id, a.content)))
        .chain(MERIT_FLAW_ARTICLES.iter().map(|a| (a.id, a.content_pt)));

    for (owner, text) in texts {
        let mut remaining = text;
        while let Some(start) = remaining.find("mta://") {
            let after = &remaining[start..];
            let end = after.find(')').unwrap_or(after.len());
            let uri = &after[..end];
            let exists = match CompendiumTarget::from_uri(uri) {
                Some(CompendiumTarget::MeritFlaw(id)) => find_merit_flaw(&id).is_some() || find_merit_flaw_article(&id).is_some(),
                Some(CompendiumTarget::Derangement(id)) => find_derangement(&id).is_some(),
                Some(CompendiumTarget::Ability(id)) => find_ability(&id).is_some(),
                Some(CompendiumTarget::Background(id)) => find_background(&id).is_some(),
                Some(CompendiumTarget::Attribute(id)) => find_attribute(&id).is_some(),
                _ => false,
            };
            assert!(exists, "Link inválido em {}: {}", owner, uri);
            remaining = &after[end..];
        }
    }
}
