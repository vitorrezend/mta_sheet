use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    PtBr,
    EnUs,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::PtBr => "pt",
            Language::EnUs => "en",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::PtBr => "Português",
            Language::EnUs => "English",
        }
    }

    pub fn flag(&self) -> &'static str {
        match self {
            Language::PtBr => "🇧🇷",
            Language::EnUs => "🇺🇸",
        }
    }
}

pub fn detect_browser_language() -> Language {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            // 1. Check user preference stored in localStorage
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(saved)) = storage.get_item("mta_sheet_lang") {
                    if saved.eq_ignore_ascii_case("en") {
                        return Language::EnUs;
                    } else if saved.eq_ignore_ascii_case("pt") {
                        return Language::PtBr;
                    }
                }
            }
            // 2. Check browser navigator language
            let nav = window.navigator();
            if let Some(lang_str) = nav.language() {
                if lang_str.to_lowercase().starts_with("pt") {
                    return Language::PtBr;
                } else {
                    return Language::EnUs;
                }
            }
        }
    }
    Language::PtBr
}

#[allow(unused_variables)]
pub fn save_language_preference(lang: Language) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("mta_sheet_lang", lang.code());
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct LanguageContext {
    pub lang: ReadSignal<Language>,
    pub set_lang: WriteSignal<Language>,
}

impl LanguageContext {
    pub fn new(lang: ReadSignal<Language>, set_lang: WriteSignal<Language>) -> Self {
        Self { lang, set_lang }
    }

    pub fn toggle(&self) {
        let current = self.lang.get();
        let next = match current {
            Language::PtBr => Language::EnUs,
            Language::EnUs => Language::PtBr,
        };
        self.set_lang.set(next);
        save_language_preference(next);
    }
}

/// Translates standard attributes
pub fn tr_attr(name: &str, lang: Language) -> &str {
    match lang {
        Language::PtBr => name,
        Language::EnUs => match name {
            "Força" => "Strength",
            "Destreza" => "Dexterity",
            "Vigor" => "Stamina",
            "Carisma" => "Charisma",
            "Manipulação" => "Manipulation",
            "Aparência" => "Appearance",
            "Percepção" => "Perception",
            "Inteligência" => "Intelligence",
            "Raciocínio" => "Wits",
            _ => name,
        },
    }
}

/// Translates standard abilities
pub fn tr_ability(name: &str, lang: Language) -> &str {
    match lang {
        Language::PtBr => name,
        Language::EnUs => match name {
            // Talentos / Talents
            "Prontidão" => "Alertness",
            "Esportes" => "Athletics",
            "Briga" => "Brawl",
            "Esquiva" => "Dodge",
            "Consciência" => "Awareness",
            "Expressão" => "Expression",
            "Intimidação" => "Intimidation",
            "Liderança" => "Leadership",
            "Manha" => "Streetwise",
            "Lábia" => "Subterfuge",

            // Perícias / Skills
            "Ofícios" => "Crafts",
            "Condução" => "Drive",
            "Etiqueta" => "Etiquette",
            "Armas de Fogo" => "Firearms",
            "Meditação" => "Meditation",
            "Armas Brancas" => "Melee",
            "Performance" => "Performance",
            "Furtividade" => "Stealth",
            "Sobrevivência" => "Survival",
            "Tecnologia" => "Technology",

            // Conhecimentos / Knowledges
            "Acadêmicos" => "Academics",
            "Computador" => "Computer",
            "Cosmologia" => "Cosmology",
            "Enigmas" => "Enigmas",
            "Investigação" => "Investigation",
            "Direito" => "Law",
            "Medicina" => "Medicine",
            "Ocultismo" => "Occult",
            "Esotérica" => "Esoterica",
            "Ciência" => "Science",

            _ => name,
        },
    }
}

/// Translates standard spheres
pub fn tr_sphere(name: &str, lang: Language) -> &str {
    match lang {
        Language::PtBr => name,
        Language::EnUs => match name {
            "Correspondência" => "Correspondence",
            "Entropia" => "Entropy",
            "Forças" => "Forces",
            "Vida" => "Life",
            "Matéria" => "Matter",
            "Mente" => "Mind",
            "Primórdio" => "Prime",
            "Espírito" => "Spirit",
            "Tempo" => "Time",
            _ => name,
        },
    }
}

/// Translates health levels
pub fn tr_health(name: &str, lang: Language) -> &str {
    match lang {
        Language::PtBr => name,
        Language::EnUs => match name {
            "Escoriado" => "Bruised",
            "Machucado" => "Hurt",
            "Ferido" => "Injured",
            "Ferido Gravemente" => "Wounded",
            "Espancado" => "Mauled",
            "Aleijado" => "Crippled",
            "Incapacitado" => "Incapacitated",
            _ => name,
        },
    }
}

/// General UI translations
/// Translates info header field labels
pub fn tr_header_label(key: &str, lang: Language) -> &str {
    match lang {
        Language::PtBr => match key {
            "Nome" => "Nome",
            "Jogador" => "Jogador",
            "Cronica" | "Crônica" => "Crônica",
            "Natureza" => "Natureza",
            "Essencia" | "Essência" => "Essência",
            "Comportamento" => "Comportamento",
            "Tradicao" | "Tradição" => "Tradição",
            "Conceito" => "Conceito",
            "Cabala" => "Cabala",
            _ => key,
        },
        Language::EnUs => match key {
            "Nome" => "Name",
            "Jogador" => "Player",
            "Cronica" | "Crônica" => "Chronicle",
            "Natureza" => "Nature",
            "Essencia" | "Essência" => "Essence",
            "Comportamento" => "Demeanor",
            "Tradicao" | "Tradição" => "Tradition",
            "Conceito" => "Concept",
            "Cabala" => "Cabal",
            _ => key,
        },
    }
}

/// Translates resonance trait names
pub fn tr_resonance(key: &str, lang: Language) -> &str {
    match lang {
        Language::PtBr => match key {
            "res_dynamic" | "Dinâmica" | "Dinâmico" | "Dynamic" => "Dinâmica",
            "res_static" | "Estática" | "Estático" | "Static" => "Estática",
            "res_entropic" | "Entrópica" | "Entrópico" | "Entropic" => "Entrópica",
            _ => key,
        },
        Language::EnUs => match key {
            "res_dynamic" | "Dinâmica" | "Dinâmico" | "Dynamic" => "Dynamic",
            "res_static" | "Estática" | "Estático" | "Static" => "Static",
            "res_entropic" | "Entrópica" | "Entrópico" | "Entropic" => "Entropic",
            _ => key,
        },
    }
}

/// Translates authentication and network error messages
pub fn tr_auth_error(msg: &str, lang: Language) -> String {
    let clean = msg.trim_start_matches("error: ").trim();
    match lang {
        Language::PtBr => match clean {
            "Usuário ou senha incorretos" | "Invalid username or password" => "Usuário ou senha incorretos".to_string(),
            "Preencha todos os campos obrigatórios" | "Please fill in all required fields" => "Preencha todos os campos obrigatórios".to_string(),
            "As senhas não conferem" | "Passwords do not match" => "As senhas não conferem".to_string(),
            "Usuário e senha são obrigatórios" | "Username and password are required" => "Usuário e senha são obrigatórios".to_string(),
            "Nome de usuário deve ter no mínimo 3 caracteres" | "Username must be at least 3 characters" => "Nome de usuário deve ter no mínimo 3 caracteres".to_string(),
            "A senha deve ter no mínimo 4 caracteres" | "Password must be at least 4 characters" => "A senha deve ter no mínimo 4 caracteres".to_string(),
            "Este nome de usuário já está em uso" | "This username is already taken" => "Este nome de usuário já está em uso".to_string(),
            "Falha temporária ao conectar ao banco de dados" | "Temporary database connection error" => "Falha temporária ao conectar ao banco de dados".to_string(),
            "Conexão com o banco de dados indisponível" | "Database connection unavailable" => "Conexão com o banco de dados indisponível".to_string(),
            "Erro ao verificar credenciais" | "Error verifying credentials" => "Erro ao verificar credenciais".to_string(),
            _ => clean.to_string(),
        },
        Language::EnUs => match clean {
            "Usuário ou senha incorretos" | "Invalid username or password" => "Invalid username or password".to_string(),
            "Preencha todos os campos obrigatórios" | "Please fill in all required fields" => "Please fill in all required fields".to_string(),
            "As senhas não conferem" | "Passwords do not match" => "Passwords do not match".to_string(),
            "Usuário e senha são obrigatórios" | "Username and password are required" => "Username and password are required".to_string(),
            "Nome de usuário deve ter no mínimo 3 caracteres" | "Username must be at least 3 characters" => "Username must be at least 3 characters".to_string(),
            "A senha deve ter no mínimo 4 caracteres" | "Password must be at least 4 characters" => "Password must be at least 4 characters".to_string(),
            "Este nome de usuário já está em uso" | "This username is already taken" => "This username is already taken".to_string(),
            "Falha temporária ao conectar ao banco de dados" | "Temporary database connection error" => "Temporary database connection error".to_string(),
            "Conexão com o banco de dados indisponível" | "Database connection unavailable" => "Database connection unavailable".to_string(),
            "Erro ao verificar credenciais" | "Error verifying credentials" => "Error verifying credentials".to_string(),
            _ => clean.to_string(),
        },
    }
}

/// Translates Dossier / Quiz question titles
pub fn tr_quiz_title(id: &str, lang: Language) -> &str {
    match lang {
        Language::PtBr => match id {
            "q_char_age" => "Qual É A Sua Idade?",
            "q_char_different" => "Quando Você Percebeu Que Era... Diferente?",
            "q_char_skills" => "Como Você Desenvolveu Suas Habilidades?",
            "q_char_important_people" => "Quem É Importante Para Você?",
            "q_char_first_magick" => "Quando Você Encontrou A Mágika Pela Primeira Vez?",
            "q_char_mentor" => "Quem Era O Seu Mentor?",
            "q_char_cabala" => "Como Você Conheceu Os Outros na Sua Cabala?",
            "q_char_mundane_life" => "Você Mantém Uma Vida Comum?",
            "q_player_what_to_do" => "O Que Você, o Jogador, Quer Fazer?",
            "q_player_destiny_pursued" => "Que Destino Ele Persegue?",
            "q_player_destiny_view" => "Como o Mago Vê o Seu Destino?",
            "q_player_avatar_nature" => "Como é o Avatar do Personagem?",
            "q_player_avatar_relation" => "Como o Mago se Relaciona Com o seu Avatar?",
            "q_player_conflicts" => "Que Conflitos Podem Surgir ao Longo do Caminho?",
            _ => id,
        },
        Language::EnUs => match id {
            "q_char_age" => "How Old Are You?",
            "q_char_different" => "When Did You Realize You Were... Different?",
            "q_char_skills" => "How Did You Develop Your Abilities?",
            "q_char_important_people" => "Who Is Important To You?",
            "q_char_first_magick" => "When Did You First Encounter Magick?",
            "q_char_mentor" => "Who Was Your Mentor?",
            "q_char_cabala" => "How Did You Meet The Others in Your Cabal?",
            "q_char_mundane_life" => "Do You Maintain a Mundane Life?",
            "q_player_what_to_do" => "What Do You, The Player, Want To Do?",
            "q_player_destiny_pursued" => "What Destiny Do You Pursue?",
            "q_player_destiny_view" => "How Does The Mage View Their Destiny?",
            "q_player_avatar_nature" => "What Is The Nature of Your Avatar?",
            "q_player_avatar_relation" => "How Do You Relate To Your Avatar?",
            "q_player_conflicts" => "What Conflicts May Arise Along The Way?",
            _ => id,
        },
    }
}

/// Translates Dossier / Quiz question prompts
pub fn tr_quiz_prompt(id: &str, lang: Language) -> &str {
    match lang {
        Language::PtBr => match id {
            "q_char_age" => "Quantos anos têm o seu personagem? Por quanto tempo ele estudou mágika? A sua aparência reflete a sua idade? Que acontecimentos foram importantes para o seu personagem (se a crônica se passa em 1996 e o seu personagem tem 25 anos, acontecimentos como a queda do Muro de Berlim terão efeito direto; se tem 40 anos, a Guerra do Vietnã terá influência marcante)?",
            "q_char_different" => "A maioria dos magos tem infâncias estranhas com eventos inexplicáveis e coincidências bizarras. Esses acontecimentos passaram despercebidos ou tiveram consequências tremendas? Uma infância feliz pode induzir no mago um sentimento de dever com os menos favorecidos, enquanto outro que passou sua infância marginalizado pode ter dificuldade de se relacionar com os outros.",
            "q_char_skills" => "Ninguém aparece do nada, completo com perícias e um lugar na sociedade. Quem era você? Onde você cresceu? Como você aprendeu aquilo que sabe? Estas perguntas dão mais profundidade e sugerem linhas de enredo que o Narrador possa tecer na crônica.",
            "q_char_important_people" => "Nenhum homem é uma ilha. Ele tem amigos superficiais ou um pequeno círculo bem unido? Como se relaciona com sua família? Alguma dessas pessoas sabe sobre suas habilidades de balançar os pilares do Céu? Como reagem a isso? O seu Despertar o afastou de todos que eram importantes? Quem é importante para ele agora?",
            "q_char_first_magick" => "Os humanos têm uma grande capacidade de ignorar ou desprezar coisas que não se encaixam nos seus modelos de mundo. Que acontecimentos superaram essa habilidade? Como descobriu que havia mistérios que não podiam ser solucionados? Ficou com medo, surpreso, louco pelo poder ou teve um colapso? Ou simplesmente nunca perdeu a habilidade infantil de acreditar em tudo?",
            "q_char_mentor" => "Magos da Tradição normalmente começam com algum tipo de mentor. Como conheceu essa pessoa? Você o procurou ou ele veio até você? Ele foi atencioso ou duro? Ele explicou tudo, ou simplesmente fez perguntas e observou? O mentor agiu como professor, pai, irmão mais velho ou força fundamental na personalidade?",
            "q_char_cabala" => "A cabala normalmente se refere aos personagens dos outros jogadores. Como você os conheceu e interagiu com eles? Preveniram alguma catástrofe na Teia, salvaram alguém de um ataque Nefandi ou foram reunidos por uma causa maior?",
            "q_char_mundane_life" => "Você tem uma 'identidade secreta'? Você continua a interagir com os Adormecidos, escondendo os aspectos sobrenaturais da sua existência, ou você deixou sua vida antiga para trás?",
            "q_player_what_to_do" => "As motivações são muito importantes, tanto as suas quanto as do seu personagem. Que tipo de pessoa você quer representar (cientista louco, mago enigmático, socialite)? Que tipo de coisas você gostaria de realizar no jogo (alimentar famintos, lutar, ficar rico, vingar alguém)? Lembre-se de que Mago é sobre encontrar verdades maiores.",
            "q_player_destiny_pursued" => "O que o místiko vê no seu destino? Morrer por um bem maior? Derrubar a Tecnocracia? Acabar com o mal, aperfeiçoar seu eu interior ou escrever os evangelhos do século XXI? Que visões guiam a busca do destino?",
            "q_player_destiny_view" => "A maioria dos magos, especialmente os novos, não querem pensar sobre o fim do caminho. No entanto, todo mago tem alguma ideia do que o destino reservou para ele. Isso o assusta ou intriga? O que ele sente sobre isso, e o que fará para persegui-lo... ou evitá-lo?",
            "q_player_avatar_nature" => "O que está nos planos do Avatar (tornar-se uno com todos, devolver a mágika ao mundo, eliminar preconceitos, completar negócios inacabados de vidas passadas)? Como ele aparece (um amigo imaginário, um surto de inspiração, um anjo com as mãos sangrentas)?",
            "q_player_avatar_relation" => "O seu mago está em conflito com o seu eu mágiko? Como eles se relacionam? O Avatar atormenta o místiko com poder/conhecimento, senta num canto ou o arrasta através das Procuras até abrir seus olhos ou ficar louco? O mago quer ser um mago ou preferiria voltar à vida antiga? Lembre-se: harmonia perfeita é tediosa!",
            "q_player_conflicts" => "O Caminho da Ascensão verdadeira nunca foi suave. Que tipos de distrações podem tirar o mago do seu caminho (amor verdadeiro, desilusões, vingança, traição, insanidade, orgulho)? Descobrindo isso, você terá uma ideia de como seu personagem reagirá quando tais coisas acontecerem.",
            _ => id,
        },
        Language::EnUs => match id {
            "q_char_age" => "How old is your character? How long have they studied magick? Does their appearance reflect their age? What historical events shaped their life (e.g. the fall of the Berlin Wall, the Vietnam War, the rise of the Internet)?",
            "q_char_different" => "Most mages have unusual childhoods filled with strange coincidences. Did these go unnoticed or cause profound consequences? A happy upbringing may foster responsibility, while an alienated youth might make bonding difficult.",
            "q_char_skills" => "Nobody appears out of nowhere with skills and a place in society. Who were you? Where did you grow up? How did you learn what you know? These roots provide rich story hooks for the Storyteller.",
            "q_char_important_people" => "No mage is an island. Do you have casual friends or a tight-knit circle? How is your relationship with your family? Does anyone know about your ability to shake the pillars of Heaven?",
            "q_char_first_magick" => "Humans excel at rationalizing anomalies. What event broke through that filter? How did you realize there were mysteries science couldn't explain? Were you terrified, amazed, or hungry for power?",
            "q_char_mentor" => "Tradition mages usually begin with a mentor. How did you meet? Did you seek them out or were you found? Were they nurturing or strict? Did they act as teacher, parent, or a formative influence?",
            "q_char_cabala" => "Your cabal connects you with the other player characters. How did you meet? Did you prevent a disaster, survive a Nephandi ambush, or answer a higher calling together?",
            "q_char_mundane_life" => "Do you keep a 'secret identity'? Do you still walk among the Sleepers hiding your supernatural nature, or have you left your old mortal life completely behind?",
            "q_player_what_to_do" => "Player motivations matter just as much as character goals. What archetype do you wish to portray (mad scientist, hermit mystic, high-society investigator)? What deeds do you want to accomplish?",
            "q_player_destiny_pursued" => "What destiny calls to your mage? Dying for the greater good? Bringing down the Technocracy? Perfecting your inner self? What vision guides your path to Ascension?",
            "q_player_destiny_view" => "Few mages want to dwell on the end of the road. Yet every mage senses what fate holds. Does your destiny excite or frighten you? Will you embrace it or fight it?",
            "q_player_avatar_nature" => "What does your Avatar desire? How does it manifest (an imaginary friend, a sudden flash of genius, a crowned angelic vision)? What cosmic role does it expect you to fulfill?",
            "q_player_avatar_relation" => "Are you in harmony or conflict with your Avatar? Does it guide you patiently or drag you kicking and screaming through Seekings? Perfect harmony is rare and boring!",
            "q_player_conflicts" => "The Path of Ascension is fraught with peril. What flaws or temptations could derail your mage (pride, tragic love, vengeance, obsession, insanity)? How will you face them?",
            _ => id,
        },
    }
}

/// Translates general UI strings, section headers, and tooltips
pub fn tr(key: &str, lang: Language) -> &str {
    match lang {
        Language::PtBr => match key {
            "mode" => "Modo:",
            "mode_base" => "Criação",
            "mode_bonus" => "Bônus",
            "mode_xp" => "XP",
            "mode_temp" => "Buff / Magia",
            "home" => "← Início",
            "logs" => "📊 Logs",
            "statement" => "📊 Extrato",
            "dossier" => "📂 Dossiê",
            "export" => "Exportar",
            "import" => "Importar",
            "save" => "Salvar",
            "saving" => "Salvando...",
            "saved" => "Salvo",
            "pending" => "Pendente",
            "attributes" => "Atributos",
            "physical" => "Físicos",
            "social" => "Sociais",
            "mental" => "Mentais",
            "abilities" => "Habilidades",
            "talents" => "Talentos",
            "skills" => "Perícias",
            "knowledges" => "Conhecimentos",
            "advantages" => "Vantagens",
            "spheres" => "Esferas",
            "backgrounds" => "Antecedentes",
            "arete" => "Arete",
            "willpower" => "Força de Vontade",
            "quintessence" => "Quintessência",
            "paradox" => "Paradoxo",
            "resonance" => "Ressonância",
            "vitality" => "Vitalidade",
            "experience" => "Experiência",
            "affinity_sphere" => "Esfera de Afinidade",
            "add_field" => "+ Adicionar Campo",
            "character_sheets" => "📜 Fichas",
            "game_rooms" => "🏰 Salas de Jogo",
            "logout" => "Sair",
            "dynamic" => "Dinâmica",
            "static" => "Estática",
            "entropic" => "Entrópica",

            // Page 2: Magia & Combate
            "page2_title" => "MAGIA & COMBATE",
            "page2_subtitle" => "Qualidades & Defeitos • Outras Características • Maravilhas • Rotes • Armamento",
            "merits_flaws" => "Qualidades & Defeitos",
            "merits" => "Qualidades",
            "flaws" => "Defeitos",
            "new_merit" => "Nova Qualidade",
            "new_flaw" => "Novo Defeito",
            "add_merit" => "Adicionar Qualidade",
            "add_flaw" => "Adicionar Defeito",
            "other_traits" => "Outras Características",
            "other_trait_slot" => "Outro Traço",
            "wonders_title" => "MARAVILHAS & ARTEFATOS",
            "wonder" => "Maravilha",
            "wonder_name_placeholder" => "Nome da Maravilha / Artefato / Wonder...",
            "wonder_rating" => "Nível",
            "wonder_powers" => "Poderes & Efeitos",
            "wonder_powers_placeholder" => "Descrição, esferas infundidas, poderes místicos e gatilho...",
            "add_wonder" => "+ Adicionar Maravilha",
            "remove_wonder" => "Remover Maravilha",
            "upload_image" => "Enviar Imagem",
            "combat_title" => "COMBATE & ARMAMENTO",
            "weapon_header" => "Arma / Golpe",
            "diff_header" => "Dif",
            "dmg_header" => "Dano",
            "range_header" => "Alcance",
            "rate_header" => "Cadência",
            "clip_header" => "Pente",
            "conceal_header" => "Ocult.",
            "armor_title" => "Armadura & Proteção",
            "armor_class" => "Classe",
            "armor_rating" => "Absorção",
            "armor_penalty" => "Penalidade",
            "combat_maneuvers" => "Manobras de Combate",
            "add_weapon" => "+ Adicionar Arma",

            // Page 3: Antecedentes Expandidos, Posses & Capela
            "expanded_bg_title" => "ANTECEDENTES EXPANDIDOS",
            "allies" => "ALIADOS",
            "contacts" => "CONTATOS",
            "fame" => "FAMA",
            "influence" => "INFLUÊNCIA",
            "library" => "BIBLIOTECA",
            "node" => "NODO",
            "resources" => "RECURSOS",
            "retainers" => "LACAIOS",
            "sanctum" => "SANTUÁRIO",
            "other_bg" => "OUTROS ANTECEDENTES",
            "possessions_title" => "POSSES & ITENS",
            "gear_carried" => "EQUIPAMENTO (Carregado)",
            "gear_carried_sub" => "Itens, ferramentas e objetos nos bolsos",
            "equipment_owned" => "EQUIPAMENTO (Possuído)",
            "equipment_owned_sub" => "Veículos, cofres e posses no refúgio",
            "foci_title" => "FOCOS",
            "foci_sub" => "Focos de paradigmas e instrumentos místicos",
            "familiar_title" => "FAMILIAR",
            "familiar_sub" => "Espírito guardião, animal companheiro ou construto",
            "grimoire_title" => "GRIMÓRIO",
            "grimoire_sub" => "Textos sagrados, tomos e anotações mágicas",
            "chantry_title" => "CAPELA",
            "chantry_sub" => "Capela / Fundação / Domínio Místico",
            "add_location" => "+ Adicionar Local",
            "location_header" => "LOCALIZAÇÃO (Espaço / Sala)",
            "description_header" => "DESCRIÇÃO (Função & Proteções)",

            // Page 4: Descrição, História & Visuais
            "description_title" => "DESCRIÇÃO FÍSICA",
            "age" => "Idade:",
            "apparent_age" => "Idade Aparente:",
            "date_of_birth" => "Data de Nascimento:",
            "age_of_awakening" => "Idade do Despertar:",
            "hair" => "Cabelos:",
            "eyes" => "Olhos:",
            "race_ethnicity" => "Etnia / Raça:",
            "nationality" => "Nacionalidade:",
            "height" => "Altura:",
            "weight" => "Peso:",
            "gender" => "Sexo / Gênero:",
            "history_title" => "HISTÓRIA",
            "char_history_label" => "HISTÓRICO DO PERSONAGEM",
            "goals_destiny_label" => "OBJETIVOS & DESTINO",
            "seekings_label" => "BUSCAS DE AVATAR (Seekings)",
            "quiets_label" => "SILÊNCIOS & LOUCURA MÍSTICA (Quiets)",
            "visuals_title" => "VISUAIS & ARQUIVOS",
            "portrait_title" => "Retrato do Personagem",
            "cabal_chart_title" => "Organograma da Cabala",

            // Page 5: Grimório & Rotinas
            "grimoire_page_title" => "GRIMÓRIO & ROTINAS MÁGICAS",
            "paradigm_label" => "PARADIGMA MÁGICO",
            "practices_label" => "PRÁTICAS MÁGICAS",
            "instruments_label" => "INSTRUMENTOS & FOCOS",
            "rotes_section_title" => "ROTINAS MÁGICAS (ROTES)",
            "add_rote" => "+ Nova Rotina Mágica",
            "rote_name" => "Nome da Rotina",
            "highest_sphere" => "Esfera Mais Alta",
            "coincident" => "Coincidente",
            "vulgar" => "Vulgar",
            "vulgar_witness" => "Vulgar c/ Testemunha",

            // Page 6: Notas
            "notes_page_title" => "ANOTAÇÕES & DIÁRIO DA CRÔNICA",
            "session_notes" => "NOTAS DE SESSÃO",
            "campaign_journal" => "DIÁRIO DE CAMPANHA",
            "visual_docs" => "DOCUMENTOS VISUAIS & EVIDÊNCIAS",

            // Dossiê & Extrato
            "dossier_modal_title" => "📂 Dossiê do Personagem — Questionário de Criação",
            "dossier_modal_sub" => "Material Suplementar & Guia de Interpretação (Mago: A Ascensão)",
            "quiz_char_section" => "👤 Perguntas para o Personagem (Histórico & Identidade)",
            "quiz_char_desc" => "Perguntas essenciais sobre quem era o personagem antes do Despertar, infância, mentor, cabala e vida comum.",
            "quiz_player_section" => "🎲 Dicas & Perguntas para os Jogadores sobre o Caminho",
            "quiz_player_desc" => "Foco nas motivações do jogador, visão do destino, dinâmica com o Avatar e conflitos da Ascensão.",
            "close" => "Fechar",
            "clear" => "Limpar",

            // Auth Page
            "auth_login_title" => "Entrar no MTA Sheet",
            "auth_register_title" => "Criar Conta de Mago",
            "auth_login_desc" => "Acesse suas fichas e salas de jogo",
            "auth_register_desc" => "Crie sua conta para gerenciar suas crônicas e salas",
            "auth_tab_login" => "Entrar",
            "auth_tab_register" => "Cadastrar",
            "auth_username_label" => "Usuário",
            "auth_username_placeholder" => "Seu nome de usuário",
            "auth_password_label" => "Senha",
            "auth_password_placeholder" => "Sua senha",
            "auth_confirm_password_label" => "Confirmar Senha",
            "auth_confirm_password_placeholder" => "Confirme sua senha",
            "auth_submit_login" => "Entrar",
            "auth_submit_register" => "Criar Minha Conta",
            "auth_submitting" => "Processando...",
            "auth_guest_link" => "← Continuar como Convidado",

            // Home Page
            "home_header_title" => "MTA Character Manager",
            "home_header_subtitle" => "Gerencie suas fichas de Mago: A Ascensão e Gods & Monsters com total privacidade",
            "home_create_title" => "Criar Nova Ficha",
            "home_type_mage" => "🧙‍♂️ Mago: A Ascensão (6 Págs)",
            "home_type_gm" => "🐉 Gods & Monsters (2 Págs)",
            "home_name_ph_mage" => "🧙‍♂️ Nome do Personagem (ex: Hermes Trismegisto)",
            "home_name_ph_gm" => "🐉 Nome do Familiar / Monstro (ex: Quimera de Hermes)",
            "home_btn_create" => "+ Criar Ficha",
            "home_btn_creating" => "✨ Criando...",
            "home_btn_import" => "📥 Importar JSON",
            "home_btn_importing" => "📥 Importando...",
            "home_import_tooltip" => "Importar uma ficha salva em arquivo .json",
            "home_visitor_title" => "Modo Visitante",
            "home_visitor_desc" => "Suas fichas agora são 100% privadas. Conecte-se para criar, editar e acessar suas fichas salvas com segurança.",
            "home_visitor_btn" => "Entrar / Cadastrar",
            "home_tab_my_sheets" => "📜 Minhas Fichas",
            "home_tab_public_sheets" => "🌐 Fichas Públicas da Comunidade",
            "home_loading_my" => "Carregando suas fichas...",
            "home_loading_pub" => "Carregando fichas públicas...",
            "home_empty_my" => "Nenhuma ficha privada encontrada. Crie uma nova ficha acima!",
            "home_empty_pub" => "Nenhuma ficha pública encontrada na comunidade no momento.",
            "home_delete_title" => "Confirmar Exclusão",
            "home_delete_prompt" => "Tem certeza que deseja excluir permanentemente a ficha de ",
            "home_delete_sub" => "Esta ação não pode ser desfeita.",
            "home_btn_cancel" => "Cancelar",
            "home_btn_confirm_delete" => "Sim, Excluir",
            "home_footer_copyright" => "MTA Sheet © 2026 — Mago: A Ascensão (M20) & Deuses e Monstros",
            "home_footer_patch_notes" => "— Notas de Atualização",

            // Character Card
            "card_no_image" => "Sem Imagem",
            "card_delete_tooltip" => "Excluir ficha",
            "card_tag_mage" => "🧙‍♂️ Mago",
            "card_tag_gm" => "🐉 Gods & Monsters",
            "card_tradition_undefined" => "Tradição não definida",
            "card_essence_awakened" => "Mago Desperto",
            "card_vis_public" => "🌐 Pública",
            "card_vis_private" => "🔒 Privada",
            "card_vis_public_tt" => "Visível para a comunidade. Clique para tornar privada.",
            "card_vis_private_tt" => "Privada para você. Clique para tornar pública.",
            "card_gnosis" => "Gnose",
            "card_arete" => "Arete",
            "card_willpower" => "Vontade",
            "card_spheres_title" => "9 Esferas",
            "card_gm_footer_desc" => "🐉 Entidade Sobrenatural (Familiar / Bygone / Espírito)",
            "card_updated" => "Atualizado:",
            "card_last_update_tt" => "Última alteração:",
            "card_open_cta" => "Abrir Ficha →",

            // Rooms Page
            "rooms_title" => "Salas de Jogo & Crônicas",
            "rooms_subtitle" => "Crie ou participe de mesas virtuais em tempo real com rastreador de iniciativa e dados compartilhados",
            "rooms_tab_my" => "🏰 Minhas Salas",
            "rooms_tab_pub" => "🌐 Salas Públicas",
            "rooms_create_title" => "Criar Nova Sala",
            "rooms_join_title" => "Entrar por Código",
            "rooms_name_label" => "Nome da Sala / Crônica",
            "rooms_name_ph" => "ex: Crônica dos Filhos do Éter",
            "rooms_desc_label" => "Descrição (opcional)",
            "rooms_desc_ph" => "ex: Mesa semanal nas sextas-feiras...",
            "rooms_pwd_label" => "Senha de Acesso (opcional)",
            "rooms_pwd_ph" => "Deixe em branco para sala aberta",
            "rooms_is_pub_label" => "Tornar sala visível publicamente",
            "rooms_btn_create" => "+ Criar Sala",
            "rooms_btn_join" => "Entrar na Sala",
            "rooms_code_label" => "Código da Sala (6 dígitos)",
            "rooms_code_ph" => "ex: 7X9K2P",
            "rooms_enter_btn" => "Entrar na Mesa →",
            _ => key,
        },
        Language::EnUs => match key {
            "mode" => "Mode:",
            "mode_base" => "Creation",
            "mode_bonus" => "Freebies",
            "mode_xp" => "XP",
            "mode_temp" => "Buff / Magic",
            "home" => "← Home",
            "logs" => "📊 Logs",
            "statement" => "📊 Statement",
            "dossier" => "📂 Dossier",
            "export" => "Export",
            "import" => "Import",
            "save" => "Save",
            "saving" => "Saving...",
            "reset" => "Reset",
            "saved" => "Saved!",
            "pending" => "Pending",
            "attributes" => "Attributes",
            "physical" => "Physical",
            "social" => "Social",
            "mental" => "Mental",
            "abilities" => "Abilities",
            "talents" => "Talents",
            "skills" => "Skills",
            "knowledges" => "Knowledges",
            "advantages" => "Advantages",
            "spheres" => "Spheres",
            "backgrounds" => "Backgrounds",
            "arete" => "Arete",
            "willpower" => "Willpower",
            "quintessence" => "Quintessence",
            "paradox" => "Paradox",
            "resonance" => "Resonance",
            "vitality" => "Health",
            "experience" => "Experience",
            "affinity_sphere" => "Affinity Sphere",
            "add_field" => "+ Add Field",
            "character_sheets" => "📜 Sheets",
            "game_rooms" => "🏰 Game Rooms",
            "logout" => "Logout",
            "dynamic" => "Dynamic",
            "static" => "Static",
            "entropic" => "Entropic",

            // Page 2: Magic & Combat
            "page2_title" => "MAGIC & COMBAT",
            "page2_subtitle" => "Merits & Flaws • Other Traits • Wonders • Rotes • Combat Equipment",
            "merits_flaws" => "Merits & Flaws",
            "merits" => "Merits",
            "flaws" => "Flaws",
            "new_merit" => "New Merit",
            "new_flaw" => "New Flaw",
            "add_merit" => "Add Merit",
            "add_flaw" => "Add Flaw",
            "other_traits" => "Other Traits",
            "other_trait_slot" => "Other Trait",
            "wonders_title" => "WONDERS & ARTIFACTS",
            "wonder" => "Wonder",
            "wonder_name_placeholder" => "Wonder / Artifact Name...",
            "wonder_rating" => "Rating",
            "wonder_powers" => "Powers & Effects",
            "wonder_powers_placeholder" => "Description, infused spheres, magical powers and triggers...",
            "add_wonder" => "+ Add Wonder",
            "remove_wonder" => "Remove Wonder",
            "upload_image" => "Upload Image",
            "combat_title" => "COMBAT & WEAPONRY",
            "weapon_header" => "Weapon / Attack",
            "diff_header" => "Diff",
            "dmg_header" => "Damage",
            "range_header" => "Range",
            "rate_header" => "Rate",
            "clip_header" => "Clip",
            "conceal_header" => "Conceal",
            "armor_title" => "Armor & Protection",
            "armor_class" => "Class",
            "armor_rating" => "Soak",
            "armor_penalty" => "Penalty",
            "combat_maneuvers" => "Combat Maneuvers",
            "add_weapon" => "+ Add Weapon",

            // Page 3: Expanded Backgrounds, Possessions & Chantry
            "expanded_bg_title" => "EXPANDED BACKGROUNDS",
            "allies" => "ALLIES",
            "contacts" => "CONTACTS",
            "fame" => "FAME",
            "influence" => "INFLUENCE",
            "library" => "LIBRARY",
            "node" => "NODE",
            "resources" => "RESOURCES",
            "retainers" => "RETAINERS",
            "sanctum" => "SANCTUM",
            "other_bg" => "OTHER BACKGROUNDS",
            "possessions_title" => "POSSESSIONS & GEAR",
            "gear_carried" => "GEAR (Carried)",
            "gear_carried_sub" => "Everyday items and carried gear",
            "equipment_owned" => "EQUIPMENT (Owned)",
            "equipment_owned_sub" => "Vehicles, safes, and sanctum property",
            "foci_title" => "FOCI",
            "foci_sub" => "Paradigm foci and mystical instruments",
            "familiar_title" => "FAMILIAR",
            "familiar_sub" => "Spirit guardian, animal companion, or construct",
            "grimoire_title" => "GRIMOIRE",
            "grimoire_sub" => "Sacred writings, tomes, and mystical notes",
            "chantry_title" => "CHANTRY",
            "chantry_sub" => "Chantry / Foundation / Mystical Demesne",
            "add_location" => "+ Add Location",
            "location_header" => "LOCATION (Space / Room)",
            "description_header" => "DESCRIPTION (Function & Wards)",

            // Page 4: Description, History & Visuals
            "description_title" => "PHYSICAL DESCRIPTION",
            "age" => "Age:",
            "apparent_age" => "Apparent Age:",
            "date_of_birth" => "Date of Birth:",
            "age_of_awakening" => "Age of Awakening:",
            "hair" => "Hair:",
            "eyes" => "Eyes:",
            "race_ethnicity" => "Ethnicity / Race:",
            "nationality" => "Nationality:",
            "height" => "Height:",
            "weight" => "Weight:",
            "gender" => "Sex / Gender:",
            "history_title" => "HISTORY",
            "char_history_label" => "CHARACTER HISTORY",
            "goals_destiny_label" => "GOALS & DESTINY",
            "seekings_label" => "AVATAR SEEKINGS",
            "quiets_label" => "MYSTICAL MADNESS (QUIETS)",
            "visuals_title" => "VISUALS & MEDIA",
            "portrait_title" => "Character Portrait",
            "cabal_chart_title" => "Cabal Hierarchy & Chart",

            // Page 5: Grimoire & Rotes
            "grimoire_page_title" => "GRIMOIRE & MAGIC ROTES",
            "paradigm_label" => "MAGICAL PARADIGM",
            "practices_label" => "MAGICAL PRACTICES",
            "instruments_label" => "INSTRUMENTS & FOCI",
            "rotes_section_title" => "MAGIC ROTES",
            "add_rote" => "+ New Magic Rote",
            "rote_name" => "Rote Name",
            "highest_sphere" => "Highest Sphere",
            "coincident" => "Coincident",
            "vulgar" => "Vulgar",
            "vulgar_witness" => "Vulgar w/ Witness",

            // Page 6: Notes
            "notes_page_title" => "SESSION NOTES & CAMPAIGN JOURNAL",
            "session_notes" => "SESSION NOTES",
            "campaign_journal" => "CAMPAIGN JOURNAL",
            "visual_docs" => "VISUAL DOCUMENTS & EVIDENCE",

            // Dossier & Cost Modal
            "dossier_modal_title" => "📂 Character Dossier — Creation Questionnaire",
            "dossier_modal_sub" => "Supplementary Roleplaying Material (Mage: The Ascension)",
            "quiz_char_section" => "👤 Character Questions (History & Identity)",
            "quiz_char_desc" => "Key questions regarding your mortal life before Awakening, childhood, mentor, cabal, and lifestyle.",
            "quiz_player_section" => "🎲 Player Questions (Goals, Destiny & Avatar)",
            "quiz_player_desc" => "Player intentions, vision of fate, relationship with Avatar, and dramatic Ascension conflicts.",
            "close" => "Close",
            "clear" => "Clear",

            // Auth Page
            "auth_login_title" => "Sign in to MTA Sheet",
            "auth_register_title" => "Create Mage Account",
            "auth_login_desc" => "Access your character sheets and game rooms",
            "auth_register_desc" => "Create your account to manage your chronicles and rooms",
            "auth_tab_login" => "Sign In",
            "auth_tab_register" => "Sign Up",
            "auth_username_label" => "Username",
            "auth_username_placeholder" => "Your username",
            "auth_password_label" => "Password",
            "auth_password_placeholder" => "Your password",
            "auth_confirm_password_label" => "Confirm Password",
            "auth_confirm_password_placeholder" => "Confirm your password",
            "auth_submit_login" => "Sign In",
            "auth_submit_register" => "Create Account",
            "auth_submitting" => "Processing...",
            "auth_guest_link" => "← Continue as Guest",

            // Home Page
            "home_header_title" => "MTA Character Manager",
            "home_header_subtitle" => "Manage your Mage: The Ascension and Gods & Monsters sheets with total privacy",
            "home_create_title" => "Create New Sheet",
            "home_type_mage" => "🧙‍♂️ Mage: The Ascension (6 Pages)",
            "home_type_gm" => "🐉 Gods & Monsters (2 Pages)",
            "home_name_ph_mage" => "🧙‍♂️ Character Name (e.g. Hermes Trismegistus)",
            "home_name_ph_gm" => "🐉 Familiar / Monster Name (e.g. Chimera of Hermes)",
            "home_btn_create" => "+ Create Sheet",
            "home_btn_creating" => "✨ Creating...",
            "home_btn_import" => "📥 Import JSON",
            "home_btn_importing" => "📥 Importing...",
            "home_import_tooltip" => "Import a character sheet saved as a .json file",
            "home_visitor_title" => "Guest Mode",
            "home_visitor_desc" => "Your sheets are 100% private. Sign in to securely create, edit, and access your saved sheets.",
            "home_visitor_btn" => "Sign In / Register",
            "home_tab_my_sheets" => "📜 My Sheets",
            "home_tab_public_sheets" => "🌐 Community Public Sheets",
            "home_loading_my" => "Loading your character sheets...",
            "home_loading_pub" => "Loading public sheets...",
            "home_empty_my" => "No private character sheets found. Create a new sheet above!",
            "home_empty_pub" => "No public character sheets found in the community right now.",
            "home_delete_title" => "Confirm Deletion",
            "home_delete_prompt" => "Are you sure you want to permanently delete the sheet of ",
            "home_delete_sub" => "This action cannot be undone.",
            "home_btn_cancel" => "Cancel",
            "home_btn_confirm_delete" => "Yes, Delete",
            "home_footer_copyright" => "MTA Sheet © 2026 — Mage: The Ascension (M20) & Gods and Monsters",
            "home_footer_patch_notes" => "— Patch Notes",

            // Character Card
            "card_no_image" => "No Image",
            "card_delete_tooltip" => "Delete sheet",
            "card_tag_mage" => "🧙‍♂️ Mage",
            "card_tag_gm" => "🐉 Gods & Monsters",
            "card_tradition_undefined" => "Tradition not set",
            "card_essence_awakened" => "Awakened Mage",
            "card_vis_public" => "🌐 Public",
            "card_vis_private" => "🔒 Private",
            "card_vis_public_tt" => "Visible to community. Click to make private.",
            "card_vis_private_tt" => "Private to you. Click to make public.",
            "card_gnosis" => "Gnosis",
            "card_arete" => "Arete",
            "card_willpower" => "Willpower",
            "card_spheres_title" => "9 Spheres",
            "card_gm_footer_desc" => "🐉 Supernatural Entity (Familiar / Bygone / Spirit)",
            "card_updated" => "Updated:",
            "card_last_update_tt" => "Last updated:",
            "card_open_cta" => "Open Sheet →",

            // Rooms Page
            "rooms_title" => "Game Rooms & Chronicles",
            "rooms_subtitle" => "Create or join virtual tables in real time with initiative tracking and shared dice",
            "rooms_tab_my" => "🏰 My Rooms",
            "rooms_tab_pub" => "🌐 Public Rooms",
            "rooms_create_title" => "Create New Room",
            "rooms_join_title" => "Join by Code",
            "rooms_name_label" => "Room / Chronicle Name",
            "rooms_name_ph" => "e.g. Sons of Ether Chronicle",
            "rooms_desc_label" => "Description (optional)",
            "rooms_desc_ph" => "e.g. Weekly Friday night game...",
            "rooms_pwd_label" => "Access Password (optional)",
            "rooms_pwd_ph" => "Leave blank for open room",
            "rooms_is_pub_label" => "Make room publicly visible",
            "rooms_btn_create" => "+ Create Room",
            "rooms_btn_join" => "Join Room",
            "rooms_code_label" => "Room Code (6 digits)",
            "rooms_code_ph" => "e.g. 7X9K2P",
            "rooms_enter_btn" => "Enter Table →",
            _ => key,
        },
    }
}
