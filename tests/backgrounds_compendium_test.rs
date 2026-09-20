use mta_sheet::compendium::backgrounds::*;
use mta_sheet::i18n::Language;
use std::collections::HashSet;

#[test]
fn test_canonical_backgrounds_count_and_uniqueness() {
    let all = get_all_backgrounds();
    assert_eq!(all.len(), 33, "M20 deve conter exatamente 33 antecedentes canônicos no compêndio completo");
    assert_eq!(ALL_BACKGROUNDS.len(), 33);

    let mut ids = HashSet::new();
    for bg in all {
        assert!(ids.insert(bg.id), "ID duplicado encontrado: {}", bg.id);
        assert!(bg.page_ref.starts_with("M20, p"), "Referência de página deve ser do livro M20: {}", bg.page_ref);
        assert!(!bg.ratings.is_empty(), "O antecedente {} deve conter níveis de pontuação", bg.name);
        assert!(!bg.name.is_empty(), "Nome em EN não pode ser vazio");
        assert!(!bg.name_pt.is_empty(), "Nome em PT não pode ser vazio");
        assert!(!bg.description.is_empty(), "Descrição em EN não pode ser vazia");
        assert!(!bg.description_pt.is_empty(), "Descrição em PT não pode ser vazia");
        assert!(!bg.system.is_empty(), "Sistema em EN não pode ser vazio");
        assert!(!bg.system_pt.is_empty(), "Sistema em PT não pode ser vazio");

        // Validação de pontuações máximas canônicas
        assert!(bg.max_dots == 5 || bg.max_dots == 10, "Pontuação máxima deve ser 5 ou 10: {}", bg.max_dots);

        // Validação de ratings
        for r in bg.ratings {
            assert!(!r.description(Language::EnUs).is_empty(), "Rating EN não pode ser vazio para {}", bg.name);
            assert!(!r.description(Language::PtBr).is_empty(), "Rating PT não pode ser vazio para {}", bg.name_pt);
        }
    }
}

#[test]
fn test_background_traits_over_five_canonical_limits() {
    // 10 antecedentes que permitem até 10 bolinhas (M20 pp. 301, 303, 305, 307, 309, 311, 313, 316, 317, 321)
    let ten_dot_ids = [
        "allies", "backup", "chantry", "contacts", "influence",
        "library", "node", "requisitions", "resources", "spies",
    ];
    for id in ten_dot_ids {
        let bg = find_background(id).expect(&format!("Antecedente {} deve existir", id));
        assert_eq!(bg.max_dots, 10, "Antecedente {} deve permitir até 10 pontos", bg.name);
    }

    // 23 antecedentes com teto em 5 bolinhas
    let five_dot_ids = [
        "alternate_identity", "arcane", "avatar", "blessing",
        "certification", "cult", "demesne", "destiny", "dream",
        "enhancement", "fame", "familiar", "legend", "mentor",
        "past_lives", "patron", "rank", "retainers", "sanctum",
        "secret_weapons", "status", "totem", "wonder",
    ];
    for id in five_dot_ids {
        let bg = find_background(id).expect(&format!("Antecedente {} deve existir", id));
        assert_eq!(bg.max_dots, 5, "Antecedente {} deve ter limite de 5 pontos", bg.name);
    }
}

#[test]
fn test_bilingual_lookups_and_diacritics_resilience() {
    // 1. Nomes canônicos em inglês
    assert_eq!(find_background("Allies").unwrap().id, "allies");
    assert_eq!(find_background("Alternate Identity").unwrap().id, "alternate_identity");
    assert_eq!(find_background("Arcane").unwrap().id, "arcane");
    assert_eq!(find_background("Avatar").unwrap().id, "avatar");
    assert_eq!(find_background("Backup").unwrap().id, "backup");
    assert_eq!(find_background("Blessing").unwrap().id, "blessing");
    assert_eq!(find_background("Certification").unwrap().id, "certification");
    assert_eq!(find_background("Chantry").unwrap().id, "chantry");
    assert_eq!(find_background("Contacts").unwrap().id, "contacts");
    assert_eq!(find_background("Cult").unwrap().id, "cult");
    assert_eq!(find_background("Demesne").unwrap().id, "demesne");
    assert_eq!(find_background("Destiny").unwrap().id, "destiny");
    assert_eq!(find_background("Dream").unwrap().id, "dream");
    assert_eq!(find_background("Enhancement").unwrap().id, "enhancement");
    assert_eq!(find_background("Fame").unwrap().id, "fame");
    assert_eq!(find_background("Familiar").unwrap().id, "familiar");
    assert_eq!(find_background("Influence").unwrap().id, "influence");
    assert_eq!(find_background("Legend").unwrap().id, "legend");
    assert_eq!(find_background("Library").unwrap().id, "library");
    assert_eq!(find_background("Mentor").unwrap().id, "mentor");
    assert_eq!(find_background("Node").unwrap().id, "node");
    assert_eq!(find_background("Past Lives").unwrap().id, "past_lives");
    assert_eq!(find_background("Patron").unwrap().id, "patron");
    assert_eq!(find_background("Rank").unwrap().id, "rank");
    assert_eq!(find_background("Requisitions").unwrap().id, "requisitions");
    assert_eq!(find_background("Resources").unwrap().id, "resources");
    assert_eq!(find_background("Retainers").unwrap().id, "retainers");
    assert_eq!(find_background("Sanctum").unwrap().id, "sanctum");
    assert_eq!(find_background("Secret Weapons").unwrap().id, "secret_weapons");
    assert_eq!(find_background("Spies").unwrap().id, "spies");
    assert_eq!(find_background("Status").unwrap().id, "status");
    assert_eq!(find_background("Totem").unwrap().id, "totem");
    assert_eq!(find_background("Wonder").unwrap().id, "wonder");

    // 2. Nomes em português com acentuação correta
    assert_eq!(find_background("Aliados").unwrap().id, "allies");
    assert_eq!(find_background("Identidade Alternativa").unwrap().id, "alternate_identity");
    assert_eq!(find_background("Arcano").unwrap().id, "arcane");
    assert_eq!(find_background("Reforço").unwrap().id, "backup");
    assert_eq!(find_background("Bênção").unwrap().id, "blessing");
    assert_eq!(find_background("Certificação").unwrap().id, "certification");
    assert_eq!(find_background("Capela").unwrap().id, "chantry");
    assert_eq!(find_background("Contatos").unwrap().id, "contacts");
    assert_eq!(find_background("Culto").unwrap().id, "cult");
    assert_eq!(find_background("Domínio").unwrap().id, "demesne");
    assert_eq!(find_background("Destino").unwrap().id, "destiny");
    assert_eq!(find_background("Sonho").unwrap().id, "dream");
    assert_eq!(find_background("Aprimoramento").unwrap().id, "enhancement");
    assert_eq!(find_background("Fama").unwrap().id, "fame");
    assert_eq!(find_background("Familiar").unwrap().id, "familiar");
    assert_eq!(find_background("Influência").unwrap().id, "influence");
    assert_eq!(find_background("Lenda").unwrap().id, "legend");
    assert_eq!(find_background("Biblioteca").unwrap().id, "library");
    assert_eq!(find_background("Mentor").unwrap().id, "mentor");
    assert_eq!(find_background("Nó").unwrap().id, "node");
    assert_eq!(find_background("Vidas Passadas").unwrap().id, "past_lives");
    assert_eq!(find_background("Patrono").unwrap().id, "patron");
    assert_eq!(find_background("Posto").unwrap().id, "rank");
    assert_eq!(find_background("Requisições").unwrap().id, "requisitions");
    assert_eq!(find_background("Recursos").unwrap().id, "resources");
    assert_eq!(find_background("Lacaios").unwrap().id, "retainers");
    assert_eq!(find_background("Santuário").unwrap().id, "sanctum");
    assert_eq!(find_background("Armas Secretas").unwrap().id, "secret_weapons");
    assert_eq!(find_background("Espiões").unwrap().id, "spies");
    assert_eq!(find_background("Status").unwrap().id, "status");
    assert_eq!(find_background("Totem").unwrap().id, "totem");
    assert_eq!(find_background("Maravilha").unwrap().id, "wonder");

    // 3. Nomes em português normalizados sem acentuação e minúsculos
    assert_eq!(find_background("reforco").unwrap().id, "backup");
    assert_eq!(find_background("bencao").unwrap().id, "blessing");
    assert_eq!(find_background("certificacao").unwrap().id, "certification");
    assert_eq!(find_background("dominio").unwrap().id, "demesne");
    assert_eq!(find_background("identidade alternativa").unwrap().id, "alternate_identity");
    assert_eq!(find_background("aprimoramento").unwrap().id, "enhancement");
    assert_eq!(find_background("influencia").unwrap().id, "influence");
    assert_eq!(find_background("no").unwrap().id, "node");
    assert_eq!(find_background("requisicoes").unwrap().id, "requisitions");
    assert_eq!(find_background("recursos").unwrap().id, "resources");
    assert_eq!(find_background("lacaios").unwrap().id, "retainers");
    assert_eq!(find_background("santuario").unwrap().id, "sanctum");
    assert_eq!(find_background("armas secretas").unwrap().id, "secret_weapons");
    assert_eq!(find_background("espioes").unwrap().id, "spies");
    assert_eq!(find_background("maravilha").unwrap().id, "wonder");
}

#[test]
fn test_technocracy_variants_and_aliases() {
    // Arcane -> Cloaking / Camuflagem
    assert_eq!(find_background("Cloaking").unwrap().id, "arcane");
    assert_eq!(find_background("cloaking").unwrap().id, "arcane");
    assert_eq!(find_background("Camuflagem").unwrap().id, "arcane");
    assert_eq!(find_background("camuflagem").unwrap().id, "arcane");

    // Avatar -> Genius / Gênio
    assert_eq!(find_background("Genius").unwrap().id, "avatar");
    assert_eq!(find_background("genius").unwrap().id, "avatar");
    assert_eq!(find_background("Gênio").unwrap().id, "avatar");
    assert_eq!(find_background("genio").unwrap().id, "avatar");

    // Chantry -> Construct / Constructo
    assert_eq!(find_background("Construct").unwrap().id, "chantry");
    assert_eq!(find_background("construct").unwrap().id, "chantry");
    assert_eq!(find_background("Constructo").unwrap().id, "chantry");
    assert_eq!(find_background("constructo").unwrap().id, "chantry");

    // Dream -> Hypercram / Hiperaprendizado
    assert_eq!(find_background("Hypercram").unwrap().id, "dream");
    assert_eq!(find_background("hypercram").unwrap().id, "dream");
    assert_eq!(find_background("Hiperaprendizado").unwrap().id, "dream");
    assert_eq!(find_background("hiperaprendizado").unwrap().id, "dream");

    // Outros antecedentes Tecnocráticos
    assert_eq!(find_background("Biotech").unwrap().id, "enhancement");
    assert_eq!(find_background("Companion").unwrap().id, "familiar");
    assert_eq!(find_background("Data Retrieval").unwrap().id, "library");
    assert_eq!(find_background("Supervisor").unwrap().id, "mentor");
    assert_eq!(find_background("Power Plant").unwrap().id, "node");
    assert_eq!(find_background("Shadow Benefactor").unwrap().id, "patron");
    assert_eq!(find_background("Clearance").unwrap().id, "rank");
    assert_eq!(find_background("Capital").unwrap().id, "resources");
    assert_eq!(find_background("Assistants").unwrap().id, "retainers");
    assert_eq!(find_background("Laboratory").unwrap().id, "sanctum");
    assert_eq!(find_background("Laboratório").unwrap().id, "sanctum");
    assert_eq!(find_background("Q Division").unwrap().id, "secret_weapons");
    assert_eq!(find_background("Surveillance Grid").unwrap().id, "spies");
    assert_eq!(find_background("Device").unwrap().id, "wonder");
    assert_eq!(find_background("Dispositivo").unwrap().id, "wonder");
}

#[test]
fn test_backup_special_sample_teams_table() {
    let backup = find_background("backup").unwrap();
    assert!(backup.backup_teams.is_some());
    let teams = backup.backup_teams.unwrap();
    assert_eq!(teams.len(), 15, "M20 pp. 305-306 lista exatamente 15 equipes de reforço de amostra");

    // Verificação de arquétipos conhecidos
    let cyborgs = teams.iter().find(|t| t.archetype.contains("Cyborgs")).expect("Cyborgs / Techs deve existir");
    assert_eq!(cyborgs.archetype_pt, "Ciborgues & Engenheiros");
    assert!(cyborgs.members.contains("cybernetic"));
    assert!(cyborgs.members_pt.contains("cibernéticos"));

    let authority = teams.iter().find(|t| t.archetype.contains("Agents of Authority")).expect("Agents of Authority deve existir");
    assert_eq!(authority.archetype_pt, "Agentes da Autoridade");
    assert!(authority.members.contains("Cops"));
    assert!(authority.members_pt.contains("Policiais"));

    let scientists = teams.iter().find(|t| t.archetype.contains("Scientists")).expect("Scientists deve existir");
    assert_eq!(scientists.archetype_pt, "Cientistas da Tecnocracia");
    assert!(scientists.members.contains("robots"));
    assert!(scientists.members_pt.contains("robóticos"));

    for team in teams {
        assert!(!team.archetype(Language::EnUs).is_empty());
        assert!(!team.archetype(Language::PtBr).is_empty());
        assert!(!team.members(Language::EnUs).is_empty());
        assert!(!team.members(Language::PtBr).is_empty());
    }
}

#[test]
fn test_chantry_special_pools_table() {
    let chantry = find_background("chantry").unwrap();
    assert!(chantry.chantry_pools.is_some());
    let pools = chantry.chantry_pools.unwrap();
    assert_eq!(pools.len(), 5, "M20 p. 308 lista exatamente 5 escalas de piscina de construção de Capela");

    assert_eq!(pools[0].pool_range, "10-20 pts");
    assert_eq!(pools[0].name_pt, "Ocupação Segura");

    assert_eq!(pools[1].pool_range, "21-30 pts");
    assert_eq!(pools[1].name_pt, "Pequeno Santuário");

    assert_eq!(pools[2].pool_range, "30-70 pts");
    assert_eq!(pools[2].name_pt, "Capela Mística / Constructo");

    assert_eq!(pools[3].pool_range, "71-100 pts");
    assert_eq!(pools[3].name_pt, "Fortaleza Avançada");

    assert_eq!(pools[4].pool_range, "101+ pts");
    assert_eq!(pools[4].name_pt, "Centro de Poder");

    for pool in pools {
        assert!(!pool.name(Language::EnUs).is_empty());
        assert!(!pool.name(Language::PtBr).is_empty());
        assert!(!pool.description(Language::EnUs).is_empty());
        assert!(!pool.description(Language::PtBr).is_empty());
    }
}

#[test]
fn test_background_theory_rules_article() {
    let rules = &BACKGROUND_THEORY_RULES;
    assert_eq!(rules.id, "theory_background_rules");
    assert_eq!(rules.page_ref, "M20, pp. 301-303");
    assert_eq!(rules.title(Language::EnUs), "Background Rules: Traits Over Five & Pooling Resources");
    assert_eq!(rules.title(Language::PtBr), "Regras de Antecedentes: Pontuações Acima de 5 & Agrupamento na Cabala");

    // Conteúdo em EN
    let content_en = rules.content(Language::EnUs);
    assert!(content_en.contains("TRAITS OVER FIVE"));
    assert!(content_en.contains("POOLING BACKGROUNDS"));
    assert!(content_en.contains("DIFFERENTIAL BACKGROUNDS"));

    // Conteúdo em PT
    let content_pt = rules.content(Language::PtBr);
    assert!(content_pt.contains("PONTUAÇÕES ACIMA DE 5 PONTOS"));
    assert!(content_pt.contains("AGRUPAMENTO DE ANTECEDENTES NA CABALA"));
    assert!(content_pt.contains("ANTECEDENTES DIFERENCIAIS"));
}

#[test]
fn test_requisitions_special_chart_table() {
    let req = find_background("requisitions").unwrap();
    assert!(req.requisitions_chart.is_some(), "Requisições deve conter a tabela de dificuldade por lealdade");
    let chart = req.requisitions_chart.unwrap();
    assert_eq!(chart.len(), 5, "Tabela de Requisições contém 5 níveis de lealdade");

    assert_eq!(chart[0].relationship, "Doubtful Loyalty");
    assert_eq!(chart[0].relationship_pt, "Lealdade Duvidosa");
    assert_eq!(chart[0].difficulty, 9);

    assert_eq!(chart[1].relationship, "Questionable Loyalty");
    assert_eq!(chart[1].relationship_pt, "Lealdade Questionável");
    assert_eq!(chart[1].difficulty, 8);

    assert_eq!(chart[2].relationship, "Assumed Loyalty");
    assert_eq!(chart[2].relationship_pt, "Lealdade Presumida");
    assert_eq!(chart[2].difficulty, 7);

    assert_eq!(chart[3].relationship, "Assured Loyalty");
    assert_eq!(chart[3].relationship_pt, "Lealdade Assegurada");
    assert_eq!(chart[3].difficulty, 6);

    assert_eq!(chart[4].relationship, "Total Loyalty");
    assert_eq!(chart[4].relationship_pt, "Lealdade Total");
    assert_eq!(chart[4].difficulty, 5);

    for item in chart {
        assert!(!item.relationship(Language::EnUs).is_empty());
        assert!(!item.relationship(Language::PtBr).is_empty());
        assert!(item.difficulty >= 3 && item.difficulty <= 10);
    }
}

#[test]
fn test_unknown_query_returns_none() {
    assert!(find_background("").is_none());
    assert!(find_background("   ").is_none());
    assert!(find_background("antecedente_inexistente_xyz_12345").is_none());
}
