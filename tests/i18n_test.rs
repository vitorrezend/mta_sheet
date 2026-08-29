use mta_sheet::i18n::{tr, tr_ability, tr_attr, tr_health, tr_sphere, Language};
use mta_sheet::state::{STANDARD_ATTRIBUTES, STANDARD_KNOWLEDGES, STANDARD_SKILLS, STANDARD_SPHERES, STANDARD_TALENTS};

#[test]
fn test_language_enum_properties() {
    assert_eq!(Language::PtBr.code(), "pt");
    assert_eq!(Language::EnUs.code(), "en");
    assert_eq!(Language::PtBr.display_name(), "Português");
    assert_eq!(Language::EnUs.display_name(), "English");
    assert_eq!(Language::PtBr.flag(), "🇧🇷");
    assert_eq!(Language::EnUs.flag(), "🇺🇸");
}

#[test]
fn test_attributes_translation() {
    let pt_attrs = ["Força", "Destreza", "Vigor", "Carisma", "Manipulação", "Aparência", "Percepção", "Inteligência", "Raciocínio"];
    let en_attrs = ["Strength", "Dexterity", "Stamina", "Charisma", "Manipulation", "Appearance", "Perception", "Intelligence", "Wits"];

    for i in 0..9 {
        assert_eq!(tr_attr(pt_attrs[i], Language::PtBr), pt_attrs[i]);
        assert_eq!(tr_attr(pt_attrs[i], Language::EnUs), en_attrs[i]);
    }

    for attr in STANDARD_ATTRIBUTES {
        let en = tr_attr(attr, Language::EnUs);
        assert_ne!(en, "");
        assert_ne!(en, attr); // Must translate to English
    }
}

#[test]
fn test_abilities_translation() {
    // 1. Talentos
    assert_eq!(tr_ability("Consciência", Language::PtBr), "Consciência");
    assert_eq!(tr_ability("Consciência", Language::EnUs), "Awareness");
    assert_eq!(tr_ability("Prontidão", Language::EnUs), "Alertness");
    assert_eq!(tr_ability("Esportes", Language::EnUs), "Athletics");
    assert_eq!(tr_ability("Briga", Language::EnUs), "Brawl");
    assert_eq!(tr_ability("Esquiva", Language::EnUs), "Dodge");

    // 2. Perícias
    assert_eq!(tr_ability("Ofícios", Language::PtBr), "Ofícios");
    assert_eq!(tr_ability("Ofícios", Language::EnUs), "Crafts");
    assert_eq!(tr_ability("Armas de Fogo", Language::EnUs), "Firearms");
    assert_eq!(tr_ability("Furtividade", Language::EnUs), "Stealth");
    assert_eq!(tr_ability("Tecnologia", Language::EnUs), "Technology");

    // 3. Conhecimentos
    assert_eq!(tr_ability("Esotérica", Language::PtBr), "Esotérica");
    assert_eq!(tr_ability("Esotérica", Language::EnUs), "Esoterica");
    assert_eq!(tr_ability("Computador", Language::EnUs), "Computer");
    assert_eq!(tr_ability("Investigação", Language::EnUs), "Investigation");
    assert_eq!(tr_ability("Ocultismo", Language::EnUs), "Occult");

    // All standard abilities have English translations
    for t in STANDARD_TALENTS {
        assert_ne!(tr_ability(t, Language::EnUs), "");
    }
    for s in STANDARD_SKILLS {
        assert_ne!(tr_ability(s, Language::EnUs), "");
    }
    for k in STANDARD_KNOWLEDGES {
        assert_ne!(tr_ability(k, Language::EnUs), "");
    }
}

#[test]
fn test_spheres_translation() {
    let pt_spheres = ["Correspondência", "Entropia", "Forças", "Vida", "Matéria", "Mente", "Primórdio", "Espírito", "Tempo"];
    let en_spheres = ["Correspondence", "Entropy", "Forces", "Life", "Matter", "Mind", "Prime", "Spirit", "Time"];

    for i in 0..9 {
        assert_eq!(tr_sphere(pt_spheres[i], Language::PtBr), pt_spheres[i]);
        assert_eq!(tr_sphere(pt_spheres[i], Language::EnUs), en_spheres[i]);
    }

    for sph in STANDARD_SPHERES {
        let en = tr_sphere(sph, Language::EnUs);
        assert_ne!(en, "");
        assert_ne!(en, sph);
    }
}

#[test]
fn test_health_levels_translation() {
    assert_eq!(tr_health("Escoriado", Language::PtBr), "Escoriado");
    assert_eq!(tr_health("Escoriado", Language::EnUs), "Bruised");
    assert_eq!(tr_health("Machucado", Language::EnUs), "Hurt");
    assert_eq!(tr_health("Ferido", Language::EnUs), "Injured");
    assert_eq!(tr_health("Ferido Gravemente", Language::EnUs), "Wounded");
    assert_eq!(tr_health("Espancado", Language::EnUs), "Mauled");
    assert_eq!(tr_health("Aleijado", Language::EnUs), "Crippled");
    assert_eq!(tr_health("Incapacitado", Language::EnUs), "Incapacitated");
}

#[test]
fn test_general_ui_translations() {
    assert_eq!(tr("mode", Language::PtBr), "Modo:");
    assert_eq!(tr("mode", Language::EnUs), "Mode:");

    assert_eq!(tr("mode_base", Language::PtBr), "Criação");
    assert_eq!(tr("mode_base", Language::EnUs), "Creation");

    assert_eq!(tr("mode_bonus", Language::PtBr), "Bônus");
    assert_eq!(tr("mode_bonus", Language::EnUs), "Freebies");

    assert_eq!(tr("save", Language::PtBr), "Salvar");
    assert_eq!(tr("save", Language::EnUs), "Save");

    assert_eq!(tr("export", Language::PtBr), "Exportar");
    assert_eq!(tr("export", Language::EnUs), "Export");

    assert_eq!(tr("import", Language::PtBr), "Importar");
    assert_eq!(tr("import", Language::EnUs), "Import");

    assert_eq!(tr("attributes", Language::PtBr), "Atributos");
    assert_eq!(tr("attributes", Language::EnUs), "Attributes");

    assert_eq!(tr("abilities", Language::PtBr), "Habilidades");
    assert_eq!(tr("abilities", Language::EnUs), "Abilities");

    assert_eq!(tr("backgrounds", Language::PtBr), "Antecedentes");
    assert_eq!(tr("backgrounds", Language::EnUs), "Backgrounds");

    assert_eq!(tr("willpower", Language::PtBr), "Força de Vontade");
    assert_eq!(tr("willpower", Language::EnUs), "Willpower");
}

#[test]
fn test_header_labels_translation() {
    use mta_sheet::i18n::tr_header_label;

    assert_eq!(tr_header_label("Nome", Language::PtBr), "Nome");
    assert_eq!(tr_header_label("Nome", Language::EnUs), "Name");
    assert_eq!(tr_header_label("Jogador", Language::EnUs), "Player");
    assert_eq!(tr_header_label("Crônica", Language::EnUs), "Chronicle");
    assert_eq!(tr_header_label("Natureza", Language::EnUs), "Nature");
    assert_eq!(tr_header_label("Essência", Language::EnUs), "Essence");
    assert_eq!(tr_header_label("Comportamento", Language::EnUs), "Demeanor");
    assert_eq!(tr_header_label("Tradição", Language::EnUs), "Tradition");
    assert_eq!(tr_header_label("Conceito", Language::EnUs), "Concept");
    assert_eq!(tr_header_label("Cabala", Language::EnUs), "Cabal");
}

#[test]
fn test_quiz_dossier_translations() {
    use mta_sheet::i18n::{tr_quiz_prompt, tr_quiz_title};

    // Check all default quiz question IDs
    let ids = [
        "q_char_age", "q_char_different", "q_char_skills", "q_char_important_people",
        "q_char_first_magick", "q_char_mentor", "q_char_cabala", "q_char_mundane_life",
        "q_player_what_to_do", "q_player_destiny_pursued", "q_player_destiny_view",
        "q_player_avatar_nature", "q_player_avatar_relation", "q_player_conflicts",
    ];

    for id in ids {
        let pt_title = tr_quiz_title(id, Language::PtBr);
        let en_title = tr_quiz_title(id, Language::EnUs);
        assert_ne!(pt_title, id);
        assert_ne!(en_title, id);
        assert_ne!(pt_title, en_title);

        let pt_prompt = tr_quiz_prompt(id, Language::PtBr);
        let en_prompt = tr_quiz_prompt(id, Language::EnUs);
        assert_ne!(pt_prompt, id);
        assert_ne!(en_prompt, id);
        assert_ne!(pt_prompt, en_prompt);
    }
}

#[test]
fn test_pages_and_sections_translation() {
    // Page 2
    assert_eq!(tr("merits_flaws", Language::PtBr), "Qualidades & Defeitos");
    assert_eq!(tr("merits_flaws", Language::EnUs), "Merits & Flaws");
    assert_eq!(tr("combat_title", Language::PtBr), "COMBATE & ARMAMENTO");
    assert_eq!(tr("combat_title", Language::EnUs), "COMBAT & WEAPONRY");

    // Page 3
    assert_eq!(tr("allies", Language::PtBr), "ALIADOS");
    assert_eq!(tr("allies", Language::EnUs), "ALLIES");
    assert_eq!(tr("chantry_title", Language::PtBr), "CAPELA");
    assert_eq!(tr("chantry_title", Language::EnUs), "CHANTRY");

    // Page 4
    assert_eq!(tr("description_title", Language::PtBr), "DESCRIÇÃO FÍSICA");
    assert_eq!(tr("description_title", Language::EnUs), "PHYSICAL DESCRIPTION");
    assert_eq!(tr("history_title", Language::PtBr), "HISTÓRIA");
    assert_eq!(tr("history_title", Language::EnUs), "HISTORY");

    // Page 5
    assert_eq!(tr("grimoire_page_title", Language::PtBr), "GRIMÓRIO & ROTINAS MÁGICAS");
    assert_eq!(tr("grimoire_page_title", Language::EnUs), "GRIMOIRE & MAGIC ROTES");
    assert_eq!(tr("coincident", Language::PtBr), "Coincidente");
    assert_eq!(tr("coincident", Language::EnUs), "Coincident");

    // Page 6
    assert_eq!(tr("session_notes", Language::PtBr), "NOTAS DE SESSÃO");
    assert_eq!(tr("session_notes", Language::EnUs), "SESSION NOTES");
}

#[test]
fn test_resonance_translation() {
    use mta_sheet::i18n::tr_resonance;

    // Dynamic
    assert_eq!(tr_resonance("res_dynamic", Language::PtBr), "Dinâmica");
    assert_eq!(tr_resonance("res_dynamic", Language::EnUs), "Dynamic");
    assert_eq!(tr_resonance("Dinâmico", Language::EnUs), "Dynamic");
    assert_eq!(tr_resonance("Dinâmica", Language::EnUs), "Dynamic");

    // Static
    assert_eq!(tr_resonance("res_static", Language::PtBr), "Estática");
    assert_eq!(tr_resonance("res_static", Language::EnUs), "Static");
    assert_eq!(tr_resonance("Estático", Language::EnUs), "Static");
    assert_eq!(tr_resonance("Estática", Language::EnUs), "Static");

    // Entropic
    assert_eq!(tr_resonance("res_entropic", Language::PtBr), "Entrópica");
    assert_eq!(tr_resonance("res_entropic", Language::EnUs), "Entropic");
    assert_eq!(tr_resonance("Entrópico", Language::EnUs), "Entropic");
    assert_eq!(tr_resonance("Entrópica", Language::EnUs), "Entropic");
}

#[test]
fn test_auth_errors_translation() {
    use mta_sheet::i18n::tr_auth_error;

    assert_eq!(tr_auth_error("Usuário ou senha incorretos", Language::PtBr), "Usuário ou senha incorretos");
    assert_eq!(tr_auth_error("Usuário ou senha incorretos", Language::EnUs), "Invalid username or password");
    assert_eq!(tr_auth_error("Preencha todos os campos obrigatórios", Language::EnUs), "Please fill in all required fields");
    assert_eq!(tr_auth_error("As senhas não conferem", Language::EnUs), "Passwords do not match");
    assert_eq!(tr_auth_error("Falha temporária ao conectar ao banco de dados", Language::EnUs), "Temporary database connection error");
}

#[test]
fn test_outside_pages_ui_translations() {
    // Auth
    assert_eq!(tr("auth_login_title", Language::PtBr), "Entrar no MTA Sheet");
    assert_eq!(tr("auth_login_title", Language::EnUs), "Sign in to MTA Sheet");
    assert_eq!(tr("auth_submit_login", Language::PtBr), "Entrar");
    assert_eq!(tr("auth_submit_login", Language::EnUs), "Sign In");

    // Home & Cards
    assert_eq!(tr("home_create_title", Language::PtBr), "Criar Nova Ficha");
    assert_eq!(tr("home_create_title", Language::EnUs), "Create New Sheet");
    assert_eq!(tr("card_vis_public", Language::PtBr), "🌐 Pública");
    assert_eq!(tr("card_vis_public", Language::EnUs), "🌐 Public");
    assert_eq!(tr("card_vis_private", Language::PtBr), "🔒 Privada");
    assert_eq!(tr("card_vis_private", Language::EnUs), "🔒 Private");
    assert_eq!(tr("card_arete", Language::PtBr), "Arete");
    assert_eq!(tr("card_arete", Language::EnUs), "Arete");
    assert_eq!(tr("card_willpower", Language::PtBr), "Vontade");
    assert_eq!(tr("card_willpower", Language::EnUs), "Willpower");
    assert_eq!(tr("card_open_cta", Language::PtBr), "Abrir Ficha →");
    assert_eq!(tr("card_open_cta", Language::EnUs), "Open Sheet →");
}

