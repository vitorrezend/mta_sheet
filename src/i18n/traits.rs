use super::Language;

/// Translates standard attributes
pub fn tr_attr(name: &str, lang: Language) -> &str {
    match lang {
        Language::PtBr => match name {
            "Strength" | "Força" => "Força",
            "Dexterity" | "Destreza" => "Destreza",
            "Stamina" | "Vigor" => "Vigor",
            "Charisma" | "Carisma" => "Carisma",
            "Manipulation" | "Manipulação" => "Manipulação",
            "Appearance" | "Aparência" => "Aparência",
            "Perception" | "Percepção" => "Percepção",
            "Intelligence" | "Inteligência" => "Inteligência",
            "Wits" | "Raciocínio" => "Raciocínio",
            _ => name,
        },
        Language::EnUs => match name {
            "Strength" | "Força" => "Strength",
            "Dexterity" | "Destreza" => "Dexterity",
            "Stamina" | "Vigor" => "Stamina",
            "Charisma" | "Carisma" => "Charisma",
            "Manipulation" | "Manipulação" => "Manipulation",
            "Appearance" | "Aparência" => "Appearance",
            "Perception" | "Percepção" => "Perception",
            "Intelligence" | "Inteligência" => "Intelligence",
            "Wits" | "Raciocínio" => "Wits",
            _ => name,
        },
    }
}

/// Translates standard abilities
pub fn tr_ability(name: &str, lang: Language) -> &str {
    match lang {
        Language::PtBr => match name {
            // Talentos / Talents
            "Alertness" | "Prontidão" => "Prontidão",
            "Art" | "Arte" => "Arte",
            "Athletics" | "Esportes" => "Esportes",
            "Brawl" | "Briga" => "Briga",
            "Dodge" | "Esquiva" => "Esquiva",
            "Awareness" | "Consciência" => "Consciência",
            "Empathy" | "Empatia" => "Empatia",
            "Expression" | "Expressão" => "Expressão",
            "Intimidação" | "Intimidation" => "Intimidação",
            "Leadership" | "Liderança" => "Liderança",
            "Streetwise" | "Manha" => "Manha",
            "Subterfuge" | "Lábia" => "Lábia",

            // Perícias / Skills
            "Crafts" | "Ofícios" => "Ofícios",
            "Drive" | "Condução" => "Condução",
            "Etiquette" | "Etiqueta" => "Etiqueta",
            "Firearms" | "Armas de Fogo" => "Armas de Fogo",
            "Martial Arts" | "Artes Marciais" => "Artes Marciais",
            "Meditation" | "Meditação" => "Meditação",
            "Melee" | "Armas Brancas" => "Armas Brancas",
            "Performance" => "Performance",
            "Research" | "Pesquisa" => "Pesquisa",
            "Stealth" | "Furtividade" => "Furtividade",
            "Survival" | "Sobrevivência" => "Sobrevivência",
            "Technology" | "Tecnologia" => "Tecnologia",

            // Conhecimentos / Knowledges
            "Academics" | "Acadêmicos" => "Acadêmicos",
            "Computer" | "Computador" => "Computador",
            "Cosmology" | "Cosmologia" => "Cosmologia",
            "Enigmas" => "Enigmas",
            "Esoterica" | "Esotérica" => "Esotérica",
            "Investigation" | "Investigação" => "Investigação",
            "Law" | "Direito" => "Direito",
            "Medicine" | "Medicina" => "Medicina",
            "Occult" | "Ocultismo" => "Ocultismo",
            "Politics" | "Política" => "Política",
            "Science" | "Ciência" => "Ciência",

            _ => name,
        },
        Language::EnUs => match name {
            // Talentos / Talents
            "Alertness" | "Prontidão" => "Alertness",
            "Art" | "Arte" => "Art",
            "Athletics" | "Esportes" => "Athletics",
            "Brawl" | "Briga" => "Brawl",
            "Dodge" | "Esquiva" => "Dodge",
            "Awareness" | "Consciência" => "Awareness",
            "Empathy" | "Empatia" => "Empathy",
            "Expression" | "Expressão" => "Expression",
            "Intimidação" | "Intimidation" => "Intimidation",
            "Leadership" | "Liderança" => "Leadership",
            "Streetwise" | "Manha" => "Streetwise",
            "Subterfuge" | "Lábia" => "Subterfuge",

            // Perícias / Skills
            "Crafts" | "Ofícios" => "Crafts",
            "Drive" | "Condução" => "Drive",
            "Etiquette" | "Etiqueta" => "Etiquette",
            "Firearms" | "Armas de Fogo" => "Firearms",
            "Martial Arts" | "Artes Marciais" => "Martial Arts",
            "Meditation" | "Meditação" => "Meditation",
            "Melee" | "Armas Brancas" => "Melee",
            "Performance" => "Performance",
            "Research" | "Pesquisa" => "Research",
            "Stealth" | "Furtividade" => "Stealth",
            "Survival" | "Sobrevivência" => "Survival",
            "Technology" | "Tecnologia" => "Technology",

            // Conhecimentos / Knowledges
            "Academics" | "Acadêmicos" => "Academics",
            "Computer" | "Computador" => "Computer",
            "Cosmology" | "Cosmologia" => "Cosmology",
            "Enigmas" => "Enigmas",
            "Esoterica" | "Esotérica" => "Esoterica",
            "Investigation" | "Investigação" => "Investigation",
            "Law" | "Direito" => "Law",
            "Medicine" | "Medicina" => "Medicine",
            "Occult" | "Ocultismo" => "Occult",
            "Politics" | "Política" => "Politics",
            "Science" | "Ciência" => "Science",

            _ => name,
        },
    }
}

/// Translates standard spheres and technocratic variants
pub fn tr_sphere(name: &str, lang: Language) -> &str {
    match lang {
        Language::PtBr => match name {
            "Data" => "Dados",
            "Primal Utility" => "Utilidade Primordial",
            "Dimensional Science" => "Ciência Dimensional",
            _ => name,
        },
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
            "Dados" => "Data",
            "Utilidade Primordial" => "Primal Utility",
            "Ciência Dimensional" => "Dimensional Science",
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

/// Translates info header field labels
pub fn tr_header_label(key: &str, lang: Language) -> &str {
    match lang {
        Language::PtBr => match key {
            "Nome" | "Name" => "Nome",
            "Jogador" | "Player" => "Jogador",
            "Cronica" | "Crônica" | "Chronicle" => "Crônica",
            "Natureza" | "Nature" => "Natureza",
            "Essencia" | "Essência" | "Essence" => "Essência",
            "Comportamento" | "Demeanor" => "Comportamento",
            "Tradicao" | "Tradição" | "Tradition" => "Tradição",
            "Conceito" | "Concept" => "Conceito",
            "Cabala" | "Cabal" => "Cabala",
            "Tipo" | "Type" => "Tipo",
            _ => key,
        },
        Language::EnUs => match key {
            "Nome" | "Name" => "Name",
            "Jogador" | "Player" => "Player",
            "Cronica" | "Crônica" | "Chronicle" => "Chronicle",
            "Natureza" | "Nature" => "Nature",
            "Essencia" | "Essência" | "Essence" => "Essence",
            "Comportamento" | "Demeanor" => "Demeanor",
            "Tradicao" | "Tradição" | "Tradition" => "Tradition",
            "Conceito" | "Concept" => "Concept",
            "Cabala" | "Cabal" => "Cabal",
            "Tipo" | "Type" => "Type",
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
