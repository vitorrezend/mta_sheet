//! M20 (Mage: The Ascension 20th Anniversary Edition) - Compendium of Weapons & Combat
//!
//! Canonical reference text and tables from Chapter 9 (Combat & Storytelling, pp. 450-453),
//! with full bilingual support (English & Portuguese), page numbers, categories, stats
//! (Difficulty, Damage/Type, Range, Rate, Clip, Conceal), Melee Combat Rule Notes (#1 to #10),
//! and Ranged Firearms/Bows Rule Notes (#1 to #12).

use serde::{Serialize, Deserialize};
use crate::i18n::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum WeaponClass {
    Melee,
    Ranged,
    Thrown,
}

impl WeaponClass {
    pub fn name(&self, lang: Language) -> &'static str {
        match (self, lang) {
            (WeaponClass::Melee, Language::PtBr) => "Armas Brancas / Corpo a Corpo",
            (WeaponClass::Melee, Language::EnUs) => "Melee Weapons",
            (WeaponClass::Ranged, Language::PtBr) => "Armas de Fogo & Distância",
            (WeaponClass::Ranged, Language::EnUs) => "Ranged Weapons & Firearms",
            (WeaponClass::Thrown, Language::PtBr) => "Armas de Arremesso",
            (WeaponClass::Thrown, Language::EnUs) => "Thrown Weapons",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            WeaponClass::Melee => "⚔️",
            WeaponClass::Ranged => "🔫",
            WeaponClass::Thrown => "🎯",
        }
    }
}

pub const ALL_WEAPON_CLASSES: [WeaponClass; 3] = [
    WeaponClass::Melee,
    WeaponClass::Ranged,
    WeaponClass::Thrown,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum WeaponMainGroup {
    Melee,
    Ranged,
}

impl WeaponMainGroup {
    pub fn name(&self, lang: Language) -> &'static str {
        match (self, lang) {
            (WeaponMainGroup::Melee, Language::PtBr) => "Arma Branca & Corpo a Corpo",
            (WeaponMainGroup::Melee, Language::EnUs) => "Melee & Close Combat",
            (WeaponMainGroup::Ranged, Language::PtBr) => "Armas de Fogo & À Distância",
            (WeaponMainGroup::Ranged, Language::EnUs) => "Firearms & Ranged Weapons",
        }
    }

    pub fn short_name(&self, lang: Language) -> &'static str {
        match (self, lang) {
            (WeaponMainGroup::Melee, Language::PtBr) => "Arma Branca",
            (WeaponMainGroup::Melee, Language::EnUs) => "Melee",
            (WeaponMainGroup::Ranged, Language::PtBr) => "À Distância",
            (WeaponMainGroup::Ranged, Language::EnUs) => "Ranged",
        }
    }

    pub fn id_str(&self) -> &'static str {
        match self {
            WeaponMainGroup::Melee => "melee",
            WeaponMainGroup::Ranged => "ranged",
        }
    }

    pub fn from_id_str(s: &str) -> Option<Self> {
        match s {
            "melee" => Some(WeaponMainGroup::Melee),
            "ranged" => Some(WeaponMainGroup::Ranged),
            _ => None,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            WeaponMainGroup::Melee => "⚔️",
            WeaponMainGroup::Ranged => "🔫",
        }
    }
}

pub const ALL_WEAPON_MAIN_GROUPS: [WeaponMainGroup; 2] = [
    WeaponMainGroup::Melee,
    WeaponMainGroup::Ranged,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum WeaponCategory {
    // --- Subcategorias de Corpo a Corpo / Melee (M20 pp. 450-451) ---
    Knives,             // Facas & Adagas (3)
    Swords,             // Espadas (5)
    Axes,               // Machados & Hastes (5)
    FistExtension,      // Soqueiras & Punhos (8)
    Clubbing,           // Porretes & Bastões (9)
    WhipsAndChains,     // Chicotes & Correntes (8)
    Improvised,         // Armas Improvisadas (4)

    // --- Subcategorias de Armas de Fogo & Distância (M20 pp. 452-453) ---
    Pistols,            // Pistolas & Revólveres (4)
    RiflesAndSmgs,      // Fuzis & Submetralhadoras (4)
    Shotguns,           // Espingardas (4)
    Bows,               // Arcos & Bestas (6)
    HeavyAndMilitary,   // Lança-Foguetes & Armas Pesadas (7)
    TechnocracySidearms,// Armas da Tecnocracia (6)
    NonLethal,          // Não-Letais & Imobilização (3)
    ThrownWeapons,      // Armas de Arremesso (6)
}

impl WeaponCategory {
    pub fn main_group(&self) -> WeaponMainGroup {
        match self {
            WeaponCategory::Knives
            | WeaponCategory::Swords
            | WeaponCategory::Axes
            | WeaponCategory::FistExtension
            | WeaponCategory::Clubbing
            | WeaponCategory::WhipsAndChains
            | WeaponCategory::Improvised => WeaponMainGroup::Melee,

            WeaponCategory::Pistols
            | WeaponCategory::RiflesAndSmgs
            | WeaponCategory::Shotguns
            | WeaponCategory::Bows
            | WeaponCategory::HeavyAndMilitary
            | WeaponCategory::TechnocracySidearms
            | WeaponCategory::NonLethal
            | WeaponCategory::ThrownWeapons => WeaponMainGroup::Ranged,
        }
    }

    pub fn class(&self) -> WeaponClass {
        match self {
            WeaponCategory::Knives
            | WeaponCategory::Swords
            | WeaponCategory::Axes
            | WeaponCategory::FistExtension
            | WeaponCategory::Clubbing
            | WeaponCategory::WhipsAndChains
            | WeaponCategory::Improvised => WeaponClass::Melee,

            WeaponCategory::Pistols
            | WeaponCategory::RiflesAndSmgs
            | WeaponCategory::Shotguns
            | WeaponCategory::Bows
            | WeaponCategory::HeavyAndMilitary
            | WeaponCategory::TechnocracySidearms
            | WeaponCategory::NonLethal => WeaponClass::Ranged,

            WeaponCategory::ThrownWeapons => WeaponClass::Thrown,
        }
    }

    pub fn weapon_class(&self) -> WeaponClass {
        self.class()
    }

    pub fn name(&self, lang: Language) -> &'static str {
        match (self, lang) {
            // Melee
            (WeaponCategory::Knives, Language::PtBr) => "Facas & Adagas",
            (WeaponCategory::Knives, Language::EnUs) => "Knives & Daggers",
            (WeaponCategory::Swords, Language::PtBr) => "Espadas",
            (WeaponCategory::Swords, Language::EnUs) => "Swords",
            (WeaponCategory::Axes, Language::PtBr) => "Machados & Hastes",
            (WeaponCategory::Axes, Language::EnUs) => "Axes & Polearms",
            (WeaponCategory::FistExtension, Language::PtBr) => "Soqueiras & Punhos",
            (WeaponCategory::FistExtension, Language::EnUs) => "Knuckles & Claws",
            (WeaponCategory::Clubbing, Language::PtBr) => "Porretes & Bastões",
            (WeaponCategory::Clubbing, Language::EnUs) => "Clubs & Staves",
            (WeaponCategory::WhipsAndChains, Language::PtBr) => "Chicotes & Correntes",
            (WeaponCategory::WhipsAndChains, Language::EnUs) => "Whips & Chains",
            (WeaponCategory::Improvised, Language::PtBr) => "Armas Improvisadas",
            (WeaponCategory::Improvised, Language::EnUs) => "Improvised Weapons",

            // Ranged
            (WeaponCategory::Pistols, Language::PtBr) => "Pistolas & Revólveres",
            (WeaponCategory::Pistols, Language::EnUs) => "Pistols & Revolvers",
            (WeaponCategory::RiflesAndSmgs, Language::PtBr) => "Fuzis & Submetralhadoras",
            (WeaponCategory::RiflesAndSmgs, Language::EnUs) => "Rifles & SMGs",
            (WeaponCategory::Shotguns, Language::PtBr) => "Espingardas",
            (WeaponCategory::Shotguns, Language::EnUs) => "Shotguns",
            (WeaponCategory::Bows, Language::PtBr) => "Arcos & Bestas",
            (WeaponCategory::Bows, Language::EnUs) => "Bows & Crossbows",
            (WeaponCategory::HeavyAndMilitary, Language::PtBr) => "Lança-Foguetes & Armas Pesadas",
            (WeaponCategory::HeavyAndMilitary, Language::EnUs) => "Rocket Launchers & Heavy Weapons",
            (WeaponCategory::TechnocracySidearms, Language::PtBr) => "Armas da Tecnocracia",
            (WeaponCategory::TechnocracySidearms, Language::EnUs) => "Technocracy Sidearms",
            (WeaponCategory::NonLethal, Language::PtBr) => "Não-Letais & Imobilização",
            (WeaponCategory::NonLethal, Language::EnUs) => "Non-Lethal / Pacification",
            (WeaponCategory::ThrownWeapons, Language::PtBr) => "Armas de Arremesso",
            (WeaponCategory::ThrownWeapons, Language::EnUs) => "Thrown Weapons",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            WeaponCategory::Knives => "🗡️",
            WeaponCategory::Swords => "⚔️",
            WeaponCategory::Axes => "🪓",
            WeaponCategory::FistExtension => "🥊",
            WeaponCategory::Clubbing => "🏏",
            WeaponCategory::WhipsAndChains => "⛓️",
            WeaponCategory::Improvised => "🪑",
            WeaponCategory::Pistols => "🔫",
            WeaponCategory::RiflesAndSmgs => "🎯",
            WeaponCategory::Shotguns => "💥",
            WeaponCategory::Bows => "🏹",
            WeaponCategory::HeavyAndMilitary => "🚀",
            WeaponCategory::TechnocracySidearms => "⚡",
            WeaponCategory::NonLethal => "🛡️",
            WeaponCategory::ThrownWeapons => "💣",
        }
    }

    pub fn id_str(&self) -> &'static str {
        match self {
            WeaponCategory::Knives => "knives",
            WeaponCategory::Swords => "swords",
            WeaponCategory::Axes => "axes",
            WeaponCategory::FistExtension => "fist_extension",
            WeaponCategory::Clubbing => "clubbing",
            WeaponCategory::WhipsAndChains => "whips_and_chains",
            WeaponCategory::Improvised => "improvised",
            WeaponCategory::Pistols => "pistols",
            WeaponCategory::RiflesAndSmgs => "rifles_and_smgs",
            WeaponCategory::Shotguns => "shotguns",
            WeaponCategory::Bows => "bows",
            WeaponCategory::HeavyAndMilitary => "heavy_and_military",
            WeaponCategory::TechnocracySidearms => "technocracy_sidearms",
            WeaponCategory::NonLethal => "non_lethal",
            WeaponCategory::ThrownWeapons => "thrown_weapons",
        }
    }

    pub fn from_id_str(s: &str) -> Option<Self> {
        ALL_WEAPON_CATEGORIES.iter().copied().find(|c| c.id_str() == s)
    }
}

pub const MELEE_CATEGORIES: &[WeaponCategory] = &[
    WeaponCategory::Knives,
    WeaponCategory::Swords,
    WeaponCategory::Axes,
    WeaponCategory::FistExtension,
    WeaponCategory::Clubbing,
    WeaponCategory::WhipsAndChains,
    WeaponCategory::Improvised,
];

pub const RANGED_CATEGORIES: &[WeaponCategory] = &[
    WeaponCategory::Pistols,
    WeaponCategory::RiflesAndSmgs,
    WeaponCategory::Shotguns,
    WeaponCategory::Bows,
    WeaponCategory::HeavyAndMilitary,
    WeaponCategory::TechnocracySidearms,
    WeaponCategory::NonLethal,
    WeaponCategory::ThrownWeapons,
];

pub const ALL_WEAPON_CATEGORIES: &[WeaponCategory] = &[
    WeaponCategory::Knives,
    WeaponCategory::Swords,
    WeaponCategory::Axes,
    WeaponCategory::FistExtension,
    WeaponCategory::Clubbing,
    WeaponCategory::WhipsAndChains,
    WeaponCategory::Improvised,
    WeaponCategory::Pistols,
    WeaponCategory::RiflesAndSmgs,
    WeaponCategory::Shotguns,
    WeaponCategory::Bows,
    WeaponCategory::HeavyAndMilitary,
    WeaponCategory::TechnocracySidearms,
    WeaponCategory::NonLethal,
    WeaponCategory::ThrownWeapons,
];

// ============================================================================
// Regras Canônicas de Combate & Notas (#1 a #10 Melee, #1 a #12 Ranged)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WeaponRuleNote {
    pub code: &'static str,
    pub title: &'static str,
    pub title_pt: &'static str,
    pub description: &'static str,
    pub description_pt: &'static str,
}

impl WeaponRuleNote {
    pub fn title(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.title_pt,
            Language::EnUs => self.title,
        }
    }

    pub fn description(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.description_pt,
            Language::EnUs => self.description,
        }
    }
}

// --- Notas de Armas Brancas / Melee (M20, pp. 450-451) ---
pub const NOTE_1: WeaponRuleNote = WeaponRuleNote {
    code: "#1",
    title: "Two-Handed Heavy Weapon",
    title_pt: "Arma Pesada de Duas Mãos",
    description: "Two-handed weapon; very heavy – requires min. Strength 3 to employ.",
    description_pt: "Arma de duas mãos; muito pesada – requer Força mín. 3 para empunhar.",
};

pub const NOTE_2: WeaponRuleNote = WeaponRuleNote {
    code: "#2",
    title: "Two Hands Required",
    title_pt: "Exigência de Duas Mãos",
    description: "Requires two hands to employ properly; +2 difficulty if used one-handed.",
    description_pt: "Requer duas mãos para usar adequadamente; +2 de dificuldade se usada com uma mão.",
};

pub const NOTE_3: WeaponRuleNote = WeaponRuleNote {
    code: "#3",
    title: "Stabbing / Impaling",
    title_pt: "Estocada / Empalação",
    description: "May be used for stabbing or impaling at +1 difficulty.",
    description_pt: "Pode ser usada para perfurar ou empalar com +1 de dificuldade.",
};

pub const NOTE_4: WeaponRuleNote = WeaponRuleNote {
    code: "#4",
    title: "Slicing / Slashing",
    title_pt: "Corte / Fatiamento",
    description: "May be used for slicing or slashing attacks at -1 difficulty.",
    description_pt: "Pode ser usada para ataques de corte ou golpe deslizante com -1 de dificuldade.",
};

pub const NOTE_5: WeaponRuleNote = WeaponRuleNote {
    code: "#5",
    title: "Disarm Maneuver",
    title_pt: "Manobra de Desarme",
    description: "May be used to disarm an opponent without botch penalties.",
    description_pt: "Pode ser usada para desarmar o oponente sem penalidade de falha crítica.",
};

pub const NOTE_6: WeaponRuleNote = WeaponRuleNote {
    code: "#6",
    title: "Sweep / Trip Maneuver",
    title_pt: "Manobra de Rasteira / Derrubada",
    description: "May be used to execute a sweep or trip maneuver.",
    description_pt: "Pode ser usada para executar rasteiras ou manobras de derrubada.",
};

pub const NOTE_7: WeaponRuleNote = WeaponRuleNote {
    code: "#7",
    title: "Thrown Weapon Option",
    title_pt: "Opção de Arma Arremessável",
    description: "Can be thrown up to (Strength x 3) yards at difficulty 6.",
    description_pt: "Pode ser arremessada a uma distância de até (Força x 3) jardas com dificuldade 6.",
};

pub const NOTE_8: WeaponRuleNote = WeaponRuleNote {
    code: "#8",
    title: "Shove / Knockdown",
    title_pt: "Empurrão / Derrubada Contundente",
    description: "Allows a shove or knockdown maneuver inflicting standard bashing damage.",
    description_pt: "Permite empurrões ou manobras de derrubada causando dano contundente padrão.",
};

pub const NOTE_9: WeaponRuleNote = WeaponRuleNote {
    code: "#9",
    title: "Self-Harm on Botch",
    title_pt: "Dano Próprio em Falha Crítica",
    description: "On a botch, user inflicts damage upon self.",
    description_pt: "Em uma falha crítica, o usuário inflige dano a si mesmo.",
};

pub const NOTE_10: WeaponRuleNote = WeaponRuleNote {
    code: "#10",
    title: "Entangle Limb",
    title_pt: "Prender Membro",
    description: "May be used to entangle an enemy’s limb at +1 difficulty.",
    description_pt: "Pode ser usado para prender um membro do inimigo com +1 de dificuldade.",
};

pub const ALL_MELEE_RULE_NOTES: &[WeaponRuleNote] = &[
    NOTE_1, NOTE_2, NOTE_3, NOTE_4, NOTE_5,
    NOTE_6, NOTE_7, NOTE_8, NOTE_9, NOTE_10,
];

pub const ALL_RULE_NOTES: &[WeaponRuleNote] = ALL_MELEE_RULE_NOTES;

// --- Notas de Armas de Fogo, Arcos & Militares (M20, pp. 452-453) ---
pub const RANGED_NOTE_1: WeaponRuleNote = WeaponRuleNote {
    code: "#1",
    title: "Full-Auto, Bursts & Strafing Sprays",
    title_pt: "Fogo Automático, Rajadas de 3 Tiros & Disparos em Varredura",
    description: "Gun may fire full-auto, three-round bursts, and strafing sprays (see Firearms Maneuvers, pp. 434-436).",
    description_pt: "A arma pode disparar em modo totalmente automático, rajadas de três tiros e fogo em varredura (ver Manobras de Armas de Fogo, pp. 434-436).",
};

pub const RANGED_NOTE_2: WeaponRuleNote = WeaponRuleNote {
    code: "#2",
    title: "Technocracy Laser Sights & Citizen Clearance",
    title_pt: "Mira Laser da Tecnocracia & Autorização de Cidadão",
    description: "All category weapons except chain-gun feature laser sights and can be used by Sleepers and extraordinary citizens; issued only to Technocracy personnel of extraordinary citizen level or higher.",
    description_pt: "Todas as armas desta categoria (exceto a chain-gun) possuem miras laser integradas e podem ser operadas por Adormecidos; emitidas apenas para agentes da Tecnocracia com classificação de cidadão extraordinário ou superior.",
};

pub const RANGED_NOTE_3: WeaponRuleNote = WeaponRuleNote {
    code: "#3",
    title: "Composite Revolver & Rapid Chambering",
    title_pt: "Revólver de Compósito & Carregamento Seletivo",
    description: "Revolver; at the touch of a button, can chamber any type of ammunition loaded in the cylinder. Both Model R and Model A made of composite materials – invisible to metal detectors.",
    description_pt: "Revólver; ao simples toque de um botão, carrega na câmara qualquer munição disposta no tambor. Modelos R e A são forjados em compósito de polímero/carbono — invisíveis a detectores de metal comuns.",
};

pub const RANGED_NOTE_4: WeaponRuleNote = WeaponRuleNote {
    code: "#4",
    title: "Enlightened Consensus Sensitivity",
    title_pt: "Sensibilidade ao Consenso / Risco para Não-Iluminados",
    description: "Rarely issued to unEnlightened personnel, as it tends to jam on a failed roll during unEnlightened use; still, more attuned to Consensus than it was years ago.",
    description_pt: "Raramente distribuída a pessoal Não-Iluminado, pois tende a emperrar em falhas de jogadas quando empunhada por adormecidos; ainda assim, encontra-se mais sintonizada com o Consenso do que em décadas passadas.",
};

pub const RANGED_NOTE_5: WeaponRuleNote = WeaponRuleNote {
    code: "#5",
    title: "Bolan Mk. 13 Heavy Hybrid System",
    title_pt: "Sistema Híbrido Pesado Bolan Mk. 13",
    description: "Combines a submachine gun main weapon (as per AK-47; 50-shot clip) with auto-shotgun capabilities (as MPS AA-12; 20-shot clip) through the main barrel, plus a grenade launcher under that barrel (as a MP-79/ six-shot magazine). Laser sights. Heavy; requires Strength 3 or better to employ but has relatively low recoil.",
    description_pt: "Combina fuzil/submetralhadora principal (estilo AK-47; pente 50 tiros) com espingarda automática (estilo MPS AA-12; pente 20 tiros) pelo cano principal, e lança-granadas acoplado inferior (estilo MP-79; pente 6 tiros). Miras laser. Muito pesada; requer Força 3+ para empunhar com recuo moderado.",
};

pub const RANGED_NOTE_6: WeaponRuleNote = WeaponRuleNote {
    code: "#6",
    title: "Draw, Nock & Reload Automatic Action",
    title_pt: "Ação de Armar, Puxar & Recarregar de Arcos/Bestas",
    description: "Long and short bows take an automatic action to nock and draw; crossbows require two automatic actions to reload. A character with Archery 3 or higher can nock, draw, and loose an arrow as a single action but cannot do the same thing with a crossbow.",
    description_pt: "Arcos longos e curtos consomem uma ação automática para encaixar e puxar a flecha; bestas requerem duas ações automáticas para recarregar. Personagens com Arquearia 3+ podem armar, puxar e disparar em uma única ação, mas o mesmo benefício não se aplica a bestas mecânicas.",
};

pub const RANGED_NOTE_7: WeaponRuleNote = WeaponRuleNote {
    code: "#7",
    title: "Collapsible Stealth Frame",
    title_pt: "Estrutura Tática Retrátil / Dobrável",
    description: "Collapsible; requires one turn to unfold from storage configuration, plus one action to load once it has been unfolded.",
    description_pt: "Dobrável; requer um turno para desdobrar da maleta ou configuração oculta de transporte, mais uma ação para municiar assim que montada.",
};

pub const RANGED_NOTE_8: WeaponRuleNote = WeaponRuleNote {
    code: "#8",
    title: "Bashing Damage & Dice Pool Debuff",
    title_pt: "Dano Contundente & Penalidade de Parada de Dados",
    description: "Bashing damage, not lethal; adds no extra damage from successes scored. Tear gas and Technocratic pacification spray also reduce target’s dice pools by two dice for one turn per success.",
    description_pt: "Causa dano contundente, não letal; sucessos extras não aumentam o dano da parada. Gás lacrimogêneo e spray de pacificação da Tecnocracia reduzem a parada de dados da vítima em 2 dados durante 1 turno por sucesso obtido.",
};

pub const RANGED_NOTE_9: WeaponRuleNote = WeaponRuleNote {
    code: "#9",
    title: "Military License & Heavy Weapons Specialization",
    title_pt: "Licença Militar Classe C & Especialização em Armas Pesadas",
    description: "Requires Firearms specialization in Heavy Weapons and a Class C license to own legally in U.S.",
    description_pt: "Exige especialização de Armas de Fogo em 'Armas Pesadas' e uma licença governamental Classe C para posse/porte legal em território civil.",
};

pub const RANGED_NOTE_10: WeaponRuleNote = WeaponRuleNote {
    code: "#10",
    title: "Vehicle-Mounted Only (Non-Portable)",
    title_pt: "Montagem em Veículo / Tripé (Não-Portátil a Pé)",
    description: "Not man-portable; must be mounted on vehicle or fixed heavy tripod.",
    description_pt: "Incapaz de ser disparada por um indivíduo solto a pé; necessita ser instalada sobre torre veicular, afuste ou tripé fixo de solo.",
};

pub const RANGED_NOTE_11: WeaponRuleNote = WeaponRuleNote {
    code: "#11",
    title: "HIT Mark Chain-Gun (Halves Armor)",
    title_pt: "Metralhadora Rotativa HIT Mark (Perfura Armadura pela Metade)",
    description: "Chain-gun; not normally man-portable, but carried by some HIT Marks and heavy-duty cyborgs. Halves armor protection due to high-speed “drilling” effect.",
    description_pt: "Metralhadora rotativa (chain-gun); não manuseável por pessoas normais, empunhada por ciborgues pesados e HIT Marks. Corta a proteção da armadura do alvo pela metade devido ao efeito de perfuração em vórtice contínuo.",
};

pub const RANGED_NOTE_12: WeaponRuleNote = WeaponRuleNote {
    code: "#12",
    title: "Explosive Blast / Area Effect",
    title_pt: "Dano Explosivo em Área (Tabela de Explosivos)",
    description: "See Explosives chart (M20 pp. 454-455). Explodes upon impact inflicting area shockwave damage.",
    description_pt: "Consulte a tabela de Explosivos (M20 pp. 454-455). Detona com o impacto causando onda de choque e estilhaços em raio esférico.",
};

pub const ALL_RANGED_RULE_NOTES: &[WeaponRuleNote] = &[
    RANGED_NOTE_1, RANGED_NOTE_2, RANGED_NOTE_3, RANGED_NOTE_4,
    RANGED_NOTE_5, RANGED_NOTE_6, RANGED_NOTE_7, RANGED_NOTE_8,
    RANGED_NOTE_9, RANGED_NOTE_10, RANGED_NOTE_11, RANGED_NOTE_12,
];

pub fn explain_note(code: &str, lang: Language) -> Option<(&'static str, &'static str)> {
    let trimmed = code.trim();
    if let Some(n) = ALL_MELEE_RULE_NOTES.iter().find(|n| n.code == trimmed) {
        return Some((n.title(lang), n.description(lang)));
    }
    if let Some(n) = ALL_RANGED_RULE_NOTES.iter().find(|n| n.code == trimmed) {
        return Some((n.title(lang), n.description(lang)));
    }
    None
}

pub fn explain_weapon_note(code: &str, lang: Language, w_class: WeaponClass) -> Option<(&'static str, &'static str)> {
    let trimmed = code.trim();
    match w_class {
        WeaponClass::Ranged => {
            if let Some(n) = ALL_RANGED_RULE_NOTES.iter().find(|n| n.code == trimmed) {
                return Some((n.title(lang), n.description(lang)));
            }
        }
        _ => {
            if let Some(n) = ALL_MELEE_RULE_NOTES.iter().find(|n| n.code == trimmed) {
                return Some((n.title(lang), n.description(lang)));
            }
        }
    }
    explain_note(code, lang)
}

// ============================================================================
// Estrutura Canônica de Armamento (M20, pp. 450-453)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WeaponDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub name_pt: &'static str,
    pub aliases: &'static [&'static str],
    pub category: WeaponCategory,
    pub difficulty: u8,
    pub damage: &'static str,
    pub damage_pt: &'static str,
    pub range: &'static str,
    pub rate: &'static str,
    pub clip: &'static str,
    pub conceal: &'static str,
    pub notes: &'static [&'static str],
    pub page_ref: &'static str,
    pub description: &'static str,
    pub description_pt: &'static str,
}

impl WeaponDefinition {
    pub fn name(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt,
            Language::EnUs => self.name,
        }
    }

    pub fn damage(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.damage_pt,
            Language::EnUs => self.damage,
        }
    }

    pub fn description(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.description_pt,
            Language::EnUs => self.description,
        }
    }

    pub fn weapon_class(&self) -> WeaponClass {
        self.category.weapon_class()
    }

    pub fn format_notes_str(&self, lang: Language) -> String {
        let mut list = Vec::new();
        for &n in self.notes {
            if n == "Used as pair" {
                match lang {
                    Language::PtBr => list.push("Usada em pares".to_string()),
                    Language::EnUs => list.push("Used as pair".to_string()),
                }
            } else {
                list.push(n.to_string());
            }
        }
        list.join(", ")
    }
}

// ============================================================================
// 1. MELEE WEAPONS (ARMAS BRANCAS - M20, pp. 450-451) - 42 ARMAS
// ============================================================================

// --- AXES (MACHADOS) ---
pub const HATCHET: WeaponDefinition = WeaponDefinition {
    id: "hatchet",
    name: "Hatchet",
    name_pt: "Machadinha",
    aliases: &["Hatchet", "Machadinha", "Pequeno Machado", "Hand Axe"],
    category: WeaponCategory::Axes,
    difficulty: 6,
    damage: "Strength +1/L",
    damage_pt: "Força +1/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "J",
    notes: &[],
    page_ref: "M20, p. 450",
    description: "A small, one-handed axe designed for chopping wood and close-quarters utility or combat.",
    description_pt: "Um pequeno machado de uma mão projetado para cortar madeira, utilidades em acampamentos ou combate corpo a corpo.",
};

pub const TOMAHAWK: WeaponDefinition = WeaponDefinition {
    id: "tomahawk",
    name: "Tomahawk",
    name_pt: "Tomahawk (Machado Tático)",
    aliases: &["Tomahawk", "Machadinha Indígena", "Tactical Tomahawk"],
    category: WeaponCategory::Axes,
    difficulty: 6,
    damage: "Strength +1/L",
    damage_pt: "Força +1/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "J",
    notes: &["#7"],
    page_ref: "M20, p. 450",
    description: "A lightweight, balanced axe traditionally crafted for throwing and vicious close-range skirmishing.",
    description_pt: "Um machado leve e equilibrado, forjado tradicionalmente para arremessos precisos e combate veloz a curta distância.",
};

pub const AXE: WeaponDefinition = WeaponDefinition {
    id: "axe",
    name: "Axe",
    name_pt: "Machado de Batalha",
    aliases: &["Axe", "Machado", "Battleaxe", "Machado Médio"],
    category: WeaponCategory::Axes,
    difficulty: 7,
    damage: "Strength +3/L",
    damage_pt: "Força +3/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "T",
    notes: &[],
    page_ref: "M20, p. 450",
    description: "A standard combat or woodsman axe with a heavy head capable of sundering armor and bone.",
    description_pt: "Um machado padrão de lenhador ou combate com lâmina pesada capaz de partir armaduras e ossos.",
};

pub const GREAT_AXE: WeaponDefinition = WeaponDefinition {
    id: "great_axe",
    name: "Great Axe",
    name_pt: "Machado Grande / Duas Mãos",
    aliases: &["Great Axe", "Machado de Duas Mãos", "Machado de Guerra", "Greataxe"],
    category: WeaponCategory::Axes,
    difficulty: 7,
    damage: "Strength +6/L",
    damage_pt: "Força +6/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "N",
    notes: &["#1"],
    page_ref: "M20, p. 450",
    description: "A massive, broad-bladed two-handed axe requiring immense muscular strength to cleave through foes.",
    description_pt: "Um machado colossal de lâmina larga e cabo longo exigindo grande poder muscular para desferir golpes devastadores.",
};

pub const POLEARM: WeaponDefinition = WeaponDefinition {
    id: "polearm",
    name: "Polearm",
    name_pt: "Arma de Haste / Alabarda",
    aliases: &["Polearm", "Alabarda", "Halberd", "Lança com Machado", "Glaive"],
    category: WeaponCategory::Axes,
    difficulty: 7,
    damage: "Strength +4/L",
    damage_pt: "Força +4/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "N",
    notes: &["#2"],
    page_ref: "M20, p. 450",
    description: "A combined axe and spear blade mounted on an eight-foot shaft, dominating range in melee combat.",
    description_pt: "Uma cabeça de machado combinada com ponta perfurante montada sobre uma haste longa, dominando o alcance corpo a corpo.",
};

// --- KNIVES & DAGGERS (FACAS & ADAGAS) ---
pub const STILETTO: WeaponDefinition = WeaponDefinition {
    id: "stiletto",
    name: "Stiletto",
    name_pt: "Estilete",
    aliases: &["Stiletto", "Estilete Fino", "Punhal Fino"],
    category: WeaponCategory::Knives,
    difficulty: 4,
    damage: "Strength +1/L",
    damage_pt: "Força +1/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "P",
    notes: &["#3"],
    page_ref: "M20, p. 450",
    description: "A slender needle-pointed dagger specifically forged for sliding between armor joints and ribs.",
    description_pt: "Uma adaga estreita com ponta de agulha feita para penetrar frestas de armaduras e cavidades vitais.",
};

pub const KNIFE: WeaponDefinition = WeaponDefinition {
    id: "knife",
    name: "Knife",
    name_pt: "Faca de Combate / Adaga",
    aliases: &["Knife", "Faca", "Adaga", "Dagger", "Bowie Knife"],
    category: WeaponCategory::Knives,
    difficulty: 5,
    damage: "Strength +1/L",
    damage_pt: "Força +1/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "P",
    notes: &["#3", "#7"],
    page_ref: "M20, p. 450",
    description: "A practical single- or double-edged blade ubiquitous in modern utility and street fights.",
    description_pt: "Uma lâmina prática de corte e perfuração, amplamente portada nas ruas e em kits de sobrevivência.",
};

pub const SAI: WeaponDefinition = WeaponDefinition {
    id: "sai",
    name: "Sai",
    name_pt: "Sai",
    aliases: &["Sai", "Tridente de Mão", "Okinawan Sai"],
    category: WeaponCategory::Knives,
    difficulty: 5,
    damage: "Strength +1/L",
    damage_pt: "Força +1/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "J",
    notes: &["#5", "#6"],
    page_ref: "M20, p. 450",
    description: "A pronged truncheon with twin curved guards, famous for trapping enemy blades and disarming strikes.",
    description_pt: "Uma arma tridente de ponta afiada e guarda curvada dupla, perfeita para prender lâminas inimigas e desarmar agressores.",
};

// --- SWORDS (ESPADAS) ---
pub const SHORT_SWORD: WeaponDefinition = WeaponDefinition {
    id: "short_sword",
    name: "Short Sword",
    name_pt: "Espada Curta (Gládio)",
    aliases: &["Short Sword", "Espada Curta", "Gládio", "Gladius", "Wakizashi"],
    category: WeaponCategory::Swords,
    difficulty: 5,
    damage: "Strength +2/L",
    damage_pt: "Força +2/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "J",
    notes: &["#3"],
    page_ref: "M20, p. 450",
    description: "A compact thrusting and slashing sword, agile in close corridors and shielded formations.",
    description_pt: "Uma espada ágil com comprimento moderado, excelente para investidas rápidas em corredores e espaços confinados.",
};

pub const SWORD: WeaponDefinition = WeaponDefinition {
    id: "sword",
    name: "Sword",
    name_pt: "Espada Longa / Sabre",
    aliases: &["Sword", "Espada", "Espada Longa", "Longsword", "Sabre", "Scimitar"],
    category: WeaponCategory::Swords,
    difficulty: 6,
    damage: "Strength +2/L",
    damage_pt: "Força +2/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "T",
    notes: &["#4"],
    page_ref: "M20, p. 450",
    description: "The classic one-handed martial sword, honed for slicing maneuvers and dueling finesse.",
    description_pt: "A clássica lâmina de combate de uma mão, balanceada para manobras de corte e esgrima refinada.",
};

pub const KATANA: WeaponDefinition = WeaponDefinition {
    id: "katana",
    name: "Katana",
    name_pt: "Katana",
    aliases: &["Katana", "Espada Samurai", "Nihonto"],
    category: WeaponCategory::Swords,
    difficulty: 6,
    damage: "Strength +3/L",
    damage_pt: "Força +3/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "T",
    notes: &["#4"],
    page_ref: "M20, p. 450",
    description: "A curved, single-edged Japanese steel blade celebrated for razor lethality and precise cutting arcs.",
    description_pt: "A consagrada lâmina curvada japonesa de fio único, lendária pela precisão cirúrgica e letalidade em arcos de corte.",
};

pub const GREAT_SWORD: WeaponDefinition = WeaponDefinition {
    id: "great_sword",
    name: "Great Sword",
    name_pt: "Espada de Duas Mãos (Montante)",
    aliases: &["Great Sword", "Montante", "Claymore", "Zweihander", "Espadão"],
    category: WeaponCategory::Swords,
    difficulty: 7,
    damage: "Strength +5/L",
    damage_pt: "Força +5/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "N",
    notes: &["#1"],
    page_ref: "M20, p. 450",
    description: "A towering two-handed battlefield sword capable of keeping multiple foes at bay.",
    description_pt: "Uma imponente espada de campo de batalha com quase dois metros de extensão, exigindo duas mãos para manobras de varredura.",
};

pub const HOOK_SWORD: WeaponDefinition = WeaponDefinition {
    id: "hook_sword",
    name: "Hook Sword(s)",
    name_pt: "Espadas Gancho (Shuang Gou)",
    aliases: &["Hook Sword", "Espadas Gancho", "Shuang Gou", "Tiger Hook"],
    category: WeaponCategory::Swords,
    difficulty: 7,
    damage: "Strength +2/L",
    damage_pt: "Força +2/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "T",
    notes: &["Used as pair", "#2", "#5", "#6"],
    page_ref: "M20, p. 450",
    description: "Twin martial blades tipped with hooking ends and crescent handguards for trapping, sweeping, and disarming.",
    description_pt: "Pares de espadas orientais com pontas curvadas em gancho e punhais crescentes, especializadas em rasteiras e desarmes.",
};

// --- CLUBBING WEAPONS (ARMAS CONTUNDENTES) ---
pub const RIOT_BATON: WeaponDefinition = WeaponDefinition {
    id: "riot_baton",
    name: "Riot Baton",
    name_pt: "Cassetete / Tonfa Policial",
    aliases: &["Riot Baton", "Tonfa", "Cassetete", "Bastão Policial", "Nightstick"],
    category: WeaponCategory::Clubbing,
    difficulty: 5,
    damage: "Strength +1/B",
    damage_pt: "Força +1/C",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "J",
    notes: &[],
    page_ref: "M20, p. 450",
    description: "A sturdy police-issue polycarbonate baton with side-handle grips for non-lethal crowd subdual.",
    description_pt: "Um bastão de policarbonato reforçado com pega lateral, padrão policial para controle de multidões e autodefesa.",
};

pub const BASEBALL_BAT: WeaponDefinition = WeaponDefinition {
    id: "baseball_bat",
    name: "Baseball Bat",
    name_pt: "Taco de Beisebol",
    aliases: &["Baseball Bat", "Taco", "Taco de Beisebol", "Bastão de Madeira"],
    category: WeaponCategory::Clubbing,
    difficulty: 6,
    damage: "Strength +2/B",
    damage_pt: "Força +2/C",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "T",
    notes: &[],
    page_ref: "M20, p. 450",
    description: "Ash wood or aluminum sports bat, providing excellent leverage and blunt kinetic force.",
    description_pt: "Taco esportivo de madeira nobre ou alumínio, oferecendo excelente alavancagem para golpes de concussão contundente.",
};

pub const CROWBAR: WeaponDefinition = WeaponDefinition {
    id: "crowbar",
    name: "Crowbar",
    name_pt: "Pé-de-Cabra",
    aliases: &["Crowbar", "Pé de Cabra", "Alavanca de Ferro"],
    category: WeaponCategory::Clubbing,
    difficulty: 6,
    damage: "Strength +2/B",
    damage_pt: "Força +2/C",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "T",
    notes: &[],
    page_ref: "M20, p. 450",
    description: "A heavy forged-steel utility pry bar that doubles as an agonizingly brutal bludgeon.",
    description_pt: "Uma barra de aço forjado com ponta chanfrada usada para forçar acessos e arrombamento, brutal como porrete.",
};

pub const STAFF: WeaponDefinition = WeaponDefinition {
    id: "staff",
    name: "Staff",
    name_pt: "Cajado / Bordão (Bo)",
    aliases: &["Staff", "Cajado", "Bordão", "Bo", "Quarterstaff"],
    category: WeaponCategory::Clubbing,
    difficulty: 6,
    damage: "Strength +2/B",
    damage_pt: "Força +2/C",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "N",
    notes: &["#2", "#6"],
    page_ref: "M20, p. 450",
    description: "A six-foot seasoned hardwood staff, classic weapon of martial monks, wanderers, and mages.",
    description_pt: "Um bastão cilíndrico de madeira densa com quase dois metros, arma clássica de monges guerreiros e magos peregrinos.",
};

pub const IRON_STAFF: WeaponDefinition = WeaponDefinition {
    id: "iron_staff",
    name: "Iron Staff",
    name_pt: "Bordão de Ferro (Tetsubo)",
    aliases: &["Iron Staff", "Bordão de Ferro", "Bastão de Aço", "Tetsubo"],
    category: WeaponCategory::Clubbing,
    difficulty: 7,
    damage: "Strength +4/B",
    damage_pt: "Força +4/C",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "N",
    notes: &["#1", "#2"],
    page_ref: "M20, p. 450",
    description: "A solid iron or steel-clad staff designed to crack shields, helmets, and vehicular frames.",
    description_pt: "Uma barra maciça de ferro ou madeira revestida em aço, capaz de esmagar armaduras pesadas e estruturas de carros.",
};

pub const MACE: WeaponDefinition = WeaponDefinition {
    id: "mace",
    name: "Mace",
    name_pt: "Maça de Armas",
    aliases: &["Mace", "Maça", "Flanged Mace", "Maça Medieval"],
    category: WeaponCategory::Clubbing,
    difficulty: 6,
    damage: "Strength +3/B",
    damage_pt: "Força +3/C",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "T",
    notes: &[],
    page_ref: "M20, p. 450",
    description: "Flanged iron head on a reinforced handle engineered to pulverize bones beneath plate armor.",
    description_pt: "Cabeça de ferro aletada sobre punho reforçado, criada nos campos medievais para estilhaçar ossos através de armaduras.",
};

pub const NUNCHAKU: WeaponDefinition = WeaponDefinition {
    id: "nunchaku",
    name: "Nunchaku",
    name_pt: "Nunchaku",
    aliases: &["Nunchaku", "Nunchuk", "Bastões Articulados"],
    category: WeaponCategory::Clubbing,
    difficulty: 7,
    damage: "Strength +2/B",
    damage_pt: "Força +2/C",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "J",
    notes: &["#5", "#7"],
    page_ref: "M20, p. 450",
    description: "Two hardwood sticks connected by a short chain or cord, multiplying rotational velocity.",
    description_pt: "Dois pequenos bastões unidos por corrente ou cordame, multiplicando a velocidade cinética em golpes rápidos e desarmes.",
};

pub const SPIKED_CLUB: WeaponDefinition = WeaponDefinition {
    id: "spiked_club",
    name: "Spiked Club",
    name_pt: "Clava com Cravos (Morgenstern)",
    aliases: &["Spiked Club", "Clava com Pregos", "Clava com Cravos", "Morningstar"],
    category: WeaponCategory::Clubbing,
    difficulty: 6,
    damage: "Strength +2/L",
    damage_pt: "Força +2/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "T",
    notes: &[],
    page_ref: "M20, p. 450",
    description: "Heavy club studded with metal spikes or nails, turning crushing blows into lethal lacerations.",
    description_pt: "Porrete pesado cravado de pregos ou puas metálicas, transformando o impacto contundente em lacerações letais.",
};

pub const HUGE_SPIKED_CLUB: WeaponDefinition = WeaponDefinition {
    id: "huge_spiked_club",
    name: "Huge Spiked Club",
    name_pt: "Grande Clava Cravada (Duas Mãos)",
    aliases: &["Huge Spiked Club", "Grande Clava", "Clava de Duas Mãos", "Kanabo"],
    category: WeaponCategory::Clubbing,
    difficulty: 7,
    damage: "Strength +5/L",
    damage_pt: "Força +5/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "N",
    notes: &["#1"],
    page_ref: "M20, p. 450",
    description: "A terrifyingly huge, spiked wooden trunk or kanabo, inflicting traumatic bone-shattering wounds.",
    description_pt: "Um tronco maciço coberto de espigões ou kanabo samurai, exigindo força descomunal para esmagar grupos inteiros.",
};

// --- FIST-EXTENSION WEAPONS (EXTENSÕES DE PUNHO) ---
pub const SAP: WeaponDefinition = WeaponDefinition {
    id: "sap",
    name: "Sap / Black-Jack",
    name_pt: "Cachaporra / Black-Jack (Sap)",
    aliases: &["Sap", "Black-Jack", "Cachaporra", "Porrete de Chumbo", "Slapjack"],
    category: WeaponCategory::FistExtension,
    difficulty: 5,
    damage: "Strength +1/B",
    damage_pt: "Força +1/C",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "P",
    notes: &[],
    page_ref: "M20, p. 450",
    description: "A palm-sized leather pouch loaded with powdered lead, favored for discreet, concussive ambushes.",
    description_pt: "Uma tira de couro cheia de chumbo maleável que cabe na palma da mão, excelente para nocautes discretos e emboscadas.",
};

pub const BRASS_KNUCKLES: WeaponDefinition = WeaponDefinition {
    id: "brass_knuckles",
    name: "Brass Knuckles",
    name_pt: "Soco-Inglês",
    aliases: &["Brass Knuckles", "Soco-Inglês", "Soco Inglês", "Knuckleduster"],
    category: WeaponCategory::FistExtension,
    difficulty: 6,
    damage: "Strength +1/B",
    damage_pt: "Força +1/C",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "P",
    notes: &[],
    page_ref: "M20, p. 450",
    description: "Cast metal guard molded to the fingers, focusing punching force into sharp fracturing impact points.",
    description_pt: "Armação de metal fundido ajustada aos dedos, concentrando a força do murro em pontos fraturantes de impacto.",
};

pub const SPIKED_GAUNTLET: WeaponDefinition = WeaponDefinition {
    id: "spiked_gauntlet",
    name: "Spiked Gauntlet",
    name_pt: "Manopla com Cravos",
    aliases: &["Spiked Gauntlet", "Manopla", "Luva com Espigões"],
    category: WeaponCategory::FistExtension,
    difficulty: 6,
    damage: "Strength +2/L",
    damage_pt: "Força +2/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "T",
    notes: &[],
    page_ref: "M20, p. 450",
    description: "An armored glove fortified with hardened steel knuckles and spikes for deadly bare-knuckle violence.",
    description_pt: "Uma luva de couro pesado e placas de aço com garras e pontas que tornam os golpes de punho letais e dilacerantes.",
};

pub const HAND_CLAWS_SMALL: WeaponDefinition = WeaponDefinition {
    id: "hand_claws_small",
    name: "Hand Claws (Small / Shuko)",
    name_pt: "Garras de Mão Pequenas (Shuko)",
    aliases: &["Hand Claws Small", "Shuko", "Garras Ninja", "Garras de Mão"],
    category: WeaponCategory::FistExtension,
    difficulty: 5,
    damage: "Strength +1/L",
    damage_pt: "Força +1/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "P",
    notes: &[],
    page_ref: "M20, p. 450",
    description: "Concealed metal palm claws used for wall scaling, parrying strikes, and raking flesh.",
    description_pt: "Pequenas garras de metal usadas na palma das mãos para escaladas, aparar golpes e desferir cortes rasgantes.",
};

pub const HAND_CLAWS_LARGE: WeaponDefinition = WeaponDefinition {
    id: "hand_claws_large",
    name: "Hand Claws (Large / Tekko-Kagi)",
    name_pt: "Garras de Mão Grandes (Tekko-Kagi)",
    aliases: &["Hand Claws Large", "Tekko-Kagi", "Garras Longas", "Bagh Nakh"],
    category: WeaponCategory::FistExtension,
    difficulty: 6,
    damage: "Strength +2/L",
    damage_pt: "Força +2/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "J",
    notes: &[],
    page_ref: "M20, p. 450",
    description: "Extended steel talons mounted on forearm bracers that butcher flesh and catch swords.",
    description_pt: "Garras de aço compridas montadas sobre o antebraço e nós dos dedos, rasgando carne com ferocidade primal.",
};

pub const KATAR: WeaponDefinition = WeaponDefinition {
    id: "katar",
    name: "Katar",
    name_pt: "Katar (Adaga de Empunhadura H)",
    aliases: &["Katar", "Adaga de Soco", "Push Dagger", "Adaga Indiana"],
    category: WeaponCategory::FistExtension,
    difficulty: 6,
    damage: "Strength +2/L",
    damage_pt: "Força +2/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "J",
    notes: &["#3"],
    page_ref: "M20, p. 450",
    description: "Indian push dagger with an H-shaped horizontal grip allowing full body weight behind punch-thrusts.",
    description_pt: "Adaga indiana com punho transversal em formato de 'H', descarregando todo o peso do corpo na estocada frontal.",
};

pub const WAR_FAN: WeaponDefinition = WeaponDefinition {
    id: "war_fan",
    name: "War Fan (Tessen)",
    name_pt: "Leque de Guerra (Tessen)",
    aliases: &["War Fan", "Tessen", "Leque de Aço", "Gunsen"],
    category: WeaponCategory::FistExtension,
    difficulty: 6,
    damage: "Strength +1/L",
    damage_pt: "Força +1/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "P",
    notes: &["#5"],
    page_ref: "M20, p. 450",
    description: "An elegant folding fan built with sharpened steel ribs, passing as high-society accessory until striking.",
    description_pt: "Leque dobrável japonês com varetas de aço afiado, disfarçado como objeto nobre de vestuário até o bote mortal.",
};

pub const WIND_AND_FIRE_WHEEL: WeaponDefinition = WeaponDefinition {
    id: "wind_and_fire_wheel",
    name: "Wind and Fire Wheel",
    name_pt: "Rodas de Vento e Fogo",
    aliases: &["Wind and Fire Wheel", "Rodas de Vento e Fogo", "Feng Huo Lun"],
    category: WeaponCategory::FistExtension,
    difficulty: 6,
    damage: "Strength +2/L",
    damage_pt: "Força +2/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "J",
    notes: &["#5"],
    page_ref: "M20, p. 450",
    description: "Crescent metal rings surrounded by stabbing points, excelling in parrying and close-quarters infighting.",
    description_pt: "Anéis metálicos chineses circundados por lâminas e chamas pontiagudas, desenhados para aprisionar armas e cortar.",
};

// --- IMPROVISED WEAPONS (ARMAS IMPROVISADAS) ---
pub const BROKEN_BOTTLE: WeaponDefinition = WeaponDefinition {
    id: "broken_bottle",
    name: "Broken Bottle",
    name_pt: "Garrafa Quebrada",
    aliases: &["Broken Bottle", "Garrafa Quebrada", "Gargalo Quebrado", "Vidro Quebrado"],
    category: WeaponCategory::Improvised,
    difficulty: 6,
    damage: "Strength +1/L",
    damage_pt: "Força +1/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "P",
    notes: &["#8"],
    page_ref: "M20, p. 451",
    description: "A jagged glass bottle neck improvised in barroom brawls, causing nasty lacerating puncture wounds.",
    description_pt: "Gargalo de vidro denteado improvisado em brigas de taverna ou becos, causando lacerações feias.",
};

pub const CHAIR: WeaponDefinition = WeaponDefinition {
    id: "chair",
    name: "Chair",
    name_pt: "Cadeira de Madeira",
    aliases: &["Chair", "Cadeira", "Banqueta"],
    category: WeaponCategory::Improvised,
    difficulty: 7,
    damage: "Strength +2/B",
    damage_pt: "Força +2/C",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "N",
    notes: &["#2", "#8"],
    page_ref: "M20, p. 451",
    description: "A common wooden dining chair or stool swung with two hands to push back and floor opponents.",
    description_pt: "Cadeira ou banqueta comum de madeira empunhada com ambas as mãos para repelir agressores e derrubá-los.",
};

pub const CHAINSAW: WeaponDefinition = WeaponDefinition {
    id: "chainsaw",
    name: "Chainsaw",
    name_pt: "Motosserra",
    aliases: &["Chainsaw", "Motosserra", "Serra Elétrica"],
    category: WeaponCategory::Improvised,
    difficulty: 8,
    damage: "Strength +7/L",
    damage_pt: "Força +7/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "N",
    notes: &["#9"],
    page_ref: "M20, p. 451",
    description: "Gasoline-powered logging saw with a spinning toothed chain, inflicting horrific, mangling carnage.",
    description_pt: "Serra mecânica de corte florestal com dentes giratórios de aço em alta rotação, infligindo carnificina horrível.",
};

pub const TABLE: WeaponDefinition = WeaponDefinition {
    id: "table",
    name: "Table",
    name_pt: "Mesa Pesada",
    aliases: &["Table", "Mesa", "Tampo de Mesa"],
    category: WeaponCategory::Improvised,
    difficulty: 8,
    damage: "Strength +4/B",
    damage_pt: "Força +4/C",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "N",
    notes: &["#1", "#8"],
    page_ref: "M20, p. 451",
    description: "Heavy furniture uprooted and heaved as an improvised bludgeoning shield or battering ram.",
    description_pt: "Móvel pesado de madeira maciça arrancado e arremessado como aríete improvisado para esmagar alvos.",
};

// --- WHIPS AND CHAINS (CHICOTES E CORRENTES) ---
pub const CHAIN: WeaponDefinition = WeaponDefinition {
    id: "chain",
    name: "Chain",
    name_pt: "Corrente de Aço",
    aliases: &["Chain", "Corrente", "Corrente de Aço", "Iron Chain"],
    category: WeaponCategory::WhipsAndChains,
    difficulty: 6,
    damage: "Strength +1/B",
    damage_pt: "Força +1/C",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "J",
    notes: &["#6", "#10"],
    page_ref: "M20, p. 451",
    description: "A three- to five-foot heavy steel link chain, delivering bone-cracking strikes and entangling limbs.",
    description_pt: "Três a cinco pés de elos pesados de aço, proporcionando chicotadas contundentes e laços de imobilização.",
};

pub const CHAIN_WHIP: WeaponDefinition = WeaponDefinition {
    id: "chain_whip",
    name: "Chain Whip",
    name_pt: "Chicote de Nove Elos (Jiujiebian)",
    aliases: &["Chain Whip", "Chicote de Aço", "Nine-Section Whip", "Jiujiebian"],
    category: WeaponCategory::WhipsAndChains,
    difficulty: 7,
    damage: "Strength +2/L",
    damage_pt: "Força +2/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "J",
    notes: &["#9", "#10"],
    page_ref: "M20, p. 451",
    description: "Articulated steel rod segments ending in a dart, demanding sublime acrobatics to avoid self-injury.",
    description_pt: "Segmentos articulados de hastes de aço finalizados por dardo perfurante, exigindo maestria para não ferir o próprio portador.",
};

pub const KUSARIGAMA: WeaponDefinition = WeaponDefinition {
    id: "kusarigama",
    name: "Kusarigama",
    name_pt: "Kusarigama (Foice com Corrente)",
    aliases: &["Kusarigama", "Foice com Corrente", "Kama Chain"],
    category: WeaponCategory::WhipsAndChains,
    difficulty: 7,
    damage: "Strength +2/L",
    damage_pt: "Força +2/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "T",
    notes: &["#2", "#6", "#10"],
    page_ref: "M20, p. 451",
    description: "Sickle attached to a weighted iron chain, entangling swords from afar before delivering the sickle coup de grâce.",
    description_pt: "Foice de mão acoplada a uma longa corrente de ferro com peso de chumbo, enredando lâminas à distância.",
};

pub const MANRIKI_GUSARI: WeaponDefinition = WeaponDefinition {
    id: "manriki_gusari",
    name: "Manriki-Gusari",
    name_pt: "Manriki-Gusari (Corrente com Pesos)",
    aliases: &["Manriki-Gusari", "Manriki", "Kusari", "Corrente com Pesos"],
    category: WeaponCategory::WhipsAndChains,
    difficulty: 6,
    damage: "Strength +1/B",
    damage_pt: "Força +1/C",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "P",
    notes: &["#5", "#6", "#10"],
    page_ref: "M20, p. 451",
    description: "Pocket-sized chain with weighted iron ends used for blocking blades, trapping wrists, and choking.",
    description_pt: "Corrente compacta de elos com pesos cônicos de ferro em cada extremidade, excelente para desarmes e estrangulamentos.",
};

pub const FLOGGER: WeaponDefinition = WeaponDefinition {
    id: "flogger",
    name: "Flogger",
    name_pt: "Chicote Multi-Tiras (Flogger)",
    aliases: &["Flogger", "Chicote de Tiras", "Açoite de Tiras"],
    category: WeaponCategory::WhipsAndChains,
    difficulty: 5,
    damage: "Strength/B",
    damage_pt: "Força/C",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "P",
    notes: &[],
    page_ref: "M20, p. 451",
    description: "Multi-tailed leather scourge that stings bare skin and leaves welts without breaking major bones.",
    description_pt: "Açoite com dezenas de tiras flexíveis de couro macio, desenhado para dor lancinante superficial e marcas.",
};

pub const BARBED_CAT: WeaponDefinition = WeaponDefinition {
    id: "barbed_cat",
    name: "Cat-o'-Nine-Tails (Barbed)",
    name_pt: "Gato de Nove Caudas Farpado",
    aliases: &["Cat-o'-Nine-Tails", "Gato de Nove Caudas", "Chicote Farpado"],
    category: WeaponCategory::WhipsAndChains,
    difficulty: 6,
    damage: "Strength +1/L",
    damage_pt: "Força +1/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "J",
    notes: &[],
    page_ref: "M20, p. 451",
    description: "Brutal nine-tailed scourge tipped with knotted metal barbs, ripping away ribbons of muscle with every strike.",
    description_pt: "Chicote com nó de nove cordas entrelaçadas encimadas por farpas afiadas de aço, estraçalhando músculos.",
};

pub const WHIP: WeaponDefinition = WeaponDefinition {
    id: "whip",
    name: "Whip",
    name_pt: "Chicote Padrão",
    aliases: &["Whip", "Chicote", "Chicote Trançado"],
    category: WeaponCategory::WhipsAndChains,
    difficulty: 6,
    damage: "Strength/L",
    damage_pt: "Força/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "J",
    notes: &["#10"],
    page_ref: "M20, p. 451",
    description: "A braided leather whip capable of cracking beyond the sound barrier, slicing open bare skin and tangling limbs.",
    description_pt: "Um chicote trançado de couro cru cujo estalo quebra a barreira do som, talhando a pele e enroscando membros distantes.",
};

pub const BULLWHIP: WeaponDefinition = WeaponDefinition {
    id: "bullwhip",
    name: "Bullwhip",
    name_pt: "Chicote de Couro (Bullwhip)",
    aliases: &["Bullwhip", "Chicote de Couro", "Chicote Pesado", "Chicote de Boiadeiro"],
    category: WeaponCategory::WhipsAndChains,
    difficulty: 7,
    damage: "Strength +1/L",
    damage_pt: "Força +1/L",
    range: "—",
    rate: "—",
    clip: "—",
    conceal: "J",
    notes: &["#10"],
    page_ref: "M20, p. 451",
    description: "A heavy, long-reach leather whip with an integrated stiff handle that delivers bone-deep lashes across significant distances.",
    description_pt: "Um chicote longo e denso com empunhadura rígida embutida, oferecendo alcance superior e impacto dilacerante profundo.",
};

pub const ALL_MELEE_WEAPONS: &[WeaponDefinition] = &[
    HATCHET, TOMAHAWK, AXE, GREAT_AXE, POLEARM,
    STILETTO, KNIFE, SHORT_SWORD, SWORD, KATANA, GREAT_SWORD, SAI, HOOK_SWORD,
    RIOT_BATON, BASEBALL_BAT, CROWBAR, STAFF, IRON_STAFF, MACE, NUNCHAKU, SPIKED_CLUB, HUGE_SPIKED_CLUB,
    SAP, BRASS_KNUCKLES, SPIKED_GAUNTLET, HAND_CLAWS_SMALL, HAND_CLAWS_LARGE, KATAR, WAR_FAN, WIND_AND_FIRE_WHEEL,
    BROKEN_BOTTLE, CHAIR, CHAINSAW, TABLE,
    CHAIN, CHAIN_WHIP, KUSARIGAMA, MANRIKI_GUSARI, FLOGGER, BARBED_CAT, WHIP, BULLWHIP,
];

// ============================================================================
// 2. RANGED WEAPONS (ARMAS DE FOGO & DISTÂNCIA - M20, pp. 452-453) - 34 ARMAS
// ============================================================================

// --- PISTOLS & REVOLVERS (PISTOLAS & REVÓLVERES) ---
pub const REVOLVER_LT: WeaponDefinition = WeaponDefinition {
    id: "revolver_lt",
    name: "Revolver, Lt. (SW M640 .38 Special)",
    name_pt: "Revólver Leve (SW M640 .38 Special)",
    aliases: &["SW M640", "Revolver Lt", "Revólver Leve", "Revolver Leve", "38 Special"],
    category: WeaponCategory::Pistols,
    difficulty: 6,
    damage: "4/L",
    damage_pt: "4/L",
    range: "12",
    rate: "3",
    clip: "6",
    conceal: "P",
    notes: &[],
    page_ref: "M20, p. 452",
    description: "A compact snub-nosed 5- or 6-shot .38 revolver, easily tucked away in a pocket or ankle holster.",
    description_pt: "Revólver compacto de cano curto calibre .38, facilmente ocultável em bolsos ou coldres de tornozelo.",
};

pub const REVOLVER_HVY: WeaponDefinition = WeaponDefinition {
    id: "revolver_hvy",
    name: "Revolver, Hvy. (Colt Anaconda .44 Magnum)",
    name_pt: "Revólver Pesado (Colt Anaconda .44 Magnum)",
    aliases: &["Colt Anaconda", "Revolver Hvy", "Revólver Pesado", "44 Magnum"],
    category: WeaponCategory::Pistols,
    difficulty: 6,
    damage: "6/L",
    damage_pt: "6/L",
    range: "35",
    rate: "2",
    clip: "6",
    conceal: "J",
    notes: &[],
    page_ref: "M20, p. 452",
    description: "A massive, roaring magnum revolver capable of cracking engine blocks and punching through doors.",
    description_pt: "Um revólver magnum maciço de alto calibre com poder de perfurar portas e parar veículos com recuo imponente.",
};

pub const PISTOL_LT: WeaponDefinition = WeaponDefinition {
    id: "pistol_lt",
    name: "Semi-Automatic Pistol, Lt. (Glock 17 9mm)",
    name_pt: "Pistola Semi-Automática Leve (Glock 17 9mm)",
    aliases: &["Glock 17", "Light Pistol", "Pistola Leve", "Semi-Automatic Pistol Lt", "9mm"],
    category: WeaponCategory::Pistols,
    difficulty: 6,
    damage: "4/L",
    damage_pt: "4/L",
    range: "20",
    rate: "4",
    clip: "17+1",
    conceal: "P",
    notes: &[],
    page_ref: "M20, p. 452",
    description: "Reliable, polymer-framed standard 9mm sidearm with high magazine capacity and smooth cycling.",
    description_pt: "Pistola de polímero padrão 9mm extremamente confiável, com grande capacidade de carregador e ciclo veloz.",
};

pub const PISTOL_HVY: WeaponDefinition = WeaponDefinition {
    id: "pistol_hvy",
    name: "Semi-Automatic Pistol, Hvy. (Desert Eagle .50 AE)",
    name_pt: "Pistola Semi-Automática Pesada (Desert Eagle .50 AE)",
    aliases: &["Desert Eagle", "Heavy Pistol", "Pistola Pesada", "Desert Eagle 50", "Semi-Automatic Pistol Hvy"],
    category: WeaponCategory::Pistols,
    difficulty: 6,
    damage: "5/L",
    damage_pt: "5/L",
    range: "30",
    rate: "3",
    clip: "7+1",
    conceal: "J",
    notes: &[],
    page_ref: "M20, p. 452",
    description: "A gas-operated hand cannon chambered in .50 Action Express, terrifying in sound and terminal impact.",
    description_pt: "Um canhão de mão a gás disparando projéteis .50 Action Express, apavorante no estampido e no poder de parada.",
};

// --- RIFLES & SUBMACHINE GUNS (FUZIS & SUBMETRALHADORAS) ---
pub const SMG_SMALL: WeaponDefinition = WeaponDefinition {
    id: "smg_small",
    name: "SMG, Small (Ingram Mac-10 9mm)",
    name_pt: "Submetralhadora Pequena (Mac-10 9mm)",
    aliases: &["Mac-10", "Mac 10", "Uzi", "Micro-Uzi", "SMG Small", "Submetralhadora Pequena"],
    category: WeaponCategory::RiflesAndSmgs,
    difficulty: 6,
    damage: "4/L",
    damage_pt: "4/L",
    range: "25",
    rate: "3",
    clip: "30+1",
    conceal: "J",
    notes: &["#1"],
    page_ref: "M20, p. 452",
    description: "Compact open-bolt submachine gun with high cyclic rate of fire, capable of emptying magazines in seconds.",
    description_pt: "Submetralhadora de ferrolho aberto ultracompacta, esvaziando pentes inteiros em rajadas devastadoras a curta distância.",
};

pub const SMG_LARGE: WeaponDefinition = WeaponDefinition {
    id: "smg_large",
    name: "SMG, Large (HK MP-5 9mm)",
    name_pt: "Submetralhadora Grande (HK MP-5 9mm)",
    aliases: &["HK MP5", "MP5", "SMG Large", "Submetralhadora Grande"],
    category: WeaponCategory::RiflesAndSmgs,
    difficulty: 6,
    damage: "4/L",
    damage_pt: "4/L",
    range: "50",
    rate: "3",
    clip: "30+1",
    conceal: "T",
    notes: &["#1"],
    page_ref: "M20, p. 452",
    description: "Tactical police and special forces roller-delayed blowback submachine gun, celebrated for accuracy.",
    description_pt: "A consagrada arma de forças especiais da SWAT e SAS, com recuo controlado em rajadas de precisão cirúrgica.",
};

pub const RIFLE_HUNTING: WeaponDefinition = WeaponDefinition {
    id: "rifle_hunting",
    name: "Rifle (Remington M-700 30.06)",
    name_pt: "Rifle de Precisão / Caça (Remington M-700 30.06)",
    aliases: &["Remington 700", "Rifle", "Fuzil de Caça", "Sniper Rifle"],
    category: WeaponCategory::RiflesAndSmgs,
    difficulty: 6,
    damage: "8/L",
    damage_pt: "8/L",
    range: "200",
    rate: "1",
    clip: "5+1",
    conceal: "N",
    notes: &[],
    page_ref: "M20, p. 452",
    description: "A bolt-action hunting and marksman rifle providing supreme accuracy across vast distances.",
    description_pt: "Fuzil de ferrolho para caça e atiradores de elite com precisão terminal em centenas de metros.",
};

pub const ASSAULT_RIFLE: WeaponDefinition = WeaponDefinition {
    id: "assault_rifle",
    name: "Assault Rifle (AK-47 7.62x39mm)",
    name_pt: "Fuzil de Assalto (AK-47 7.62x39mm)",
    aliases: &["AK-47", "AK47", "Assault Rifle", "Fuzil de Assalto", "M-16", "AR-15"],
    category: WeaponCategory::RiflesAndSmgs,
    difficulty: 6,
    damage: "7/L",
    damage_pt: "7/L",
    range: "150",
    rate: "3",
    clip: "42+1",
    conceal: "N",
    notes: &["#1"],
    page_ref: "M20, p. 452",
    description: "Rugged, gas-operated military assault rifle that operates flawlessly in mud, sand, and blizzards.",
    description_pt: "Fuzil militar de combate padrão com alta penetração balística de 7.62mm, lendário por sua confiabilidade extrema.",
};

// --- SHOTGUNS (ESPINGARDAS) ---
pub const SHOTGUN_SAWED_OFF: WeaponDefinition = WeaponDefinition {
    id: "shotgun_sawed_off",
    name: "Shotgun, Sawed Off (Winchester Model 24 12G)",
    name_pt: "Espingarda de Cano Serrado (Winchester 12)",
    aliases: &["Sawed Off Shotgun", "Cano Serrado", "Shotgun Sawed Off"],
    category: WeaponCategory::Shotguns,
    difficulty: 6,
    damage: "8/L",
    damage_pt: "8/L",
    range: "10",
    rate: "2",
    clip: "2",
    conceal: "J",
    notes: &[],
    page_ref: "M20, p. 452",
    description: "A double-barreled 12-gauge with cut-down barrels, creating a deadly cone of lead at point-blank range.",
    description_pt: "Espingarda de cano duplo serrada, gerando uma nuvem de chumbo impiedosa à queima-roupa e fácil de esconder em jaquetas.",
};

pub const SHOTGUN_PUMP: WeaponDefinition = WeaponDefinition {
    id: "shotgun_pump",
    name: "Shotgun (Ithaca M-37 12 Gauge)",
    name_pt: "Espingarda Pump-Action (Ithaca M-37 12G)",
    aliases: &["Shotgun", "Espingarda", "Ithaca 37", "Remington 870", "Pump Shotgun"],
    category: WeaponCategory::Shotguns,
    difficulty: 6,
    damage: "8/L",
    damage_pt: "8/L",
    range: "20",
    rate: "1",
    clip: "5+1",
    conceal: "T",
    notes: &[],
    page_ref: "M20, p. 452",
    description: "Classic manual pump-action 12-gauge shotgun, trusted for home defense and law enforcement.",
    description_pt: "Espingarda clássica de ação por bombeamento com cartuchos calibre 12 de alto poder destrutivo.",
};

pub const SHOTGUN_SEMI_AUTO: WeaponDefinition = WeaponDefinition {
    id: "shotgun_semi_auto",
    name: "Shotgun, Semi-Automatic (Benelli M4 12G)",
    name_pt: "Espingarda Semi-Automática (Benelli M4 12G)",
    aliases: &["Benelli M4", "Semi-Auto Shotgun", "Shotgun Semi-Automatic", "Benelli"],
    category: WeaponCategory::Shotguns,
    difficulty: 6,
    damage: "8/L",
    damage_pt: "8/L",
    range: "25",
    rate: "3",
    clip: "6+1",
    conceal: "T",
    notes: &[],
    page_ref: "M20, p. 452",
    description: "Combat gas-regulated semi-automatic shotgun providing rapid follow-up shots without pumping.",
    description_pt: "Espingarda tática semi-automática militar permitindo múltiplos disparos de calibre 12 sem necessidade de armação manual.",
};

pub const SHOTGUN_ASSAULT: WeaponDefinition = WeaponDefinition {
    id: "shotgun_assault",
    name: "Shotgun, Assault (MPS AA-12 12 Gauge)",
    name_pt: "Espingarda de Assalto (MPS AA-12 12G)",
    aliases: &["AA-12", "AA12", "MPS AA-12", "Shotgun Assault", "Espingarda de Assalto"],
    category: WeaponCategory::Shotguns,
    difficulty: 6,
    damage: "8/L",
    damage_pt: "8/L",
    range: "50",
    rate: "3",
    clip: "32+1",
    conceal: "N",
    notes: &["#1"],
    page_ref: "M20, p. 452",
    description: "Full-auto combat shotgun feeding from 32-round drum magazines, delivering terrifying area suppression.",
    description_pt: "Espingarda de combate totalmente automática alimentada por tambor de 32 tiros, varrendo salas inteiras em segundos.",
};

// --- TECHNOCRACY SIDEARMS (ARMAS DA TECNOCRACIA) ---
pub const BIGGS_X5_MODEL_R: WeaponDefinition = WeaponDefinition {
    id: "biggs_x5_model_r",
    name: "Biggs X-5 Model R Protector",
    name_pt: "Protetor Biggs X-5 Modelo R",
    aliases: &["Biggs X-5 Model R", "Biggs R", "Model R Protector"],
    category: WeaponCategory::TechnocracySidearms,
    difficulty: 6,
    damage: "5/L",
    damage_pt: "5/L",
    range: "40",
    rate: "4",
    clip: "12",
    conceal: "J",
    notes: &["#2", "#3"],
    page_ref: "M20, p. 452",
    description: "Technocratic composite cylinder sidearm with laser sight and rapid electronic chambering selector.",
    description_pt: "Arma curta da Tecnocracia em compósito impermeável a detectores de metal, com mira laser e seleção eletrônica de munição.",
};

pub const BIGGS_X5_MODEL_A: WeaponDefinition = WeaponDefinition {
    id: "biggs_x5_model_a",
    name: "Biggs X-5 Model A Protector",
    name_pt: "Protetor Biggs X-5 Modelo A",
    aliases: &["Biggs X-5 Model A", "Biggs A", "Model A Protector"],
    category: WeaponCategory::TechnocracySidearms,
    difficulty: 6,
    damage: "5/L",
    damage_pt: "5/L",
    range: "40",
    rate: "4",
    clip: "12+1",
    conceal: "J",
    notes: &["#1", "#2", "#3"],
    page_ref: "M20, p. 452",
    description: "Automatic-fire variant of the composite Biggs X-5 with burst fire capability and integral laser targeting.",
    description_pt: "Versão com capacidade de rajada do Biggs X-5 em compósito, com mira laser e câmara para disparos múltiplos.",
};

pub const BIGGS_MJOLLNER_MK4: WeaponDefinition = WeaponDefinition {
    id: "biggs_mjollner_mk4",
    name: "Biggs Mjollner Mk. IV",
    name_pt: "Biggs Mjollner Mk. IV (Canhão de Mão)",
    aliases: &["Mjollner", "Biggs Mjollner", "Mjollner Mk IV"],
    category: WeaponCategory::TechnocracySidearms,
    difficulty: 6,
    damage: "10/L",
    damage_pt: "10/L",
    range: "100",
    rate: "1",
    clip: "10",
    conceal: "T",
    notes: &["#2", "#4"],
    page_ref: "M20, p. 452",
    description: "Heavy hypertech particle pistol with catastrophic kinetic transfer, reserved strictly for Enlightened operatives.",
    description_pt: "Pistola de partículas pesadas hipertecnológica com impacto cinético avassalador, reservada a operativos Iluminados.",
};

pub const CASTLE_GRAVES_WW3: WeaponDefinition = WeaponDefinition {
    id: "castle_graves_ww3",
    name: "Castle-Graves WW-3",
    name_pt: "Castle-Graves WW-3 (Fuzil Tecnocrata)",
    aliases: &["Castle-Graves", "WW-3", "Castle Graves WW3"],
    category: WeaponCategory::TechnocracySidearms,
    difficulty: 6,
    damage: "8/L",
    damage_pt: "8/L",
    range: "200",
    rate: "3",
    clip: "50+1",
    conceal: "N",
    notes: &["#1", "#2"],
    page_ref: "M20, p. 452",
    description: "Hypertech squad combat rifle with 50-round high-density magazine and target-tracking optics.",
    description_pt: "Fuzil hipertecnológico de esquadrão com carregador de 50 projéteis e ótica com aquisição de alvos em tempo real.",
};

pub const BOLAN_MK13_SYSTEM: WeaponDefinition = WeaponDefinition {
    id: "bolan_mk13_system",
    name: "Bolan Mk. 13 Weapons System",
    name_pt: "Sistema de Armas Bolan Mk. 13",
    aliases: &["Bolan Mk 13", "Bolan Weapons System", "Bolan Mk. 13"],
    category: WeaponCategory::TechnocracySidearms,
    difficulty: 6,
    damage: "Special",
    damage_pt: "Especial",
    range: "Special",
    rate: "Special",
    clip: "Special",
    conceal: "N",
    notes: &["#1", "#2", "#5"],
    page_ref: "M20, p. 452",
    description: "Modular combat rig combining SMG, automatic shotgun, and underbarrel grenade launcher in one heavy frame.",
    description_pt: "Plataforma integrada combinando fuzil de assalto, espingarda automática e lança-granadas sob o mesmo cano reforçado.",
};

pub const HIT_MARK_CHAIN_GUN: WeaponDefinition = WeaponDefinition {
    id: "hit_mark_chain_gun",
    name: "HIT Mark Chain-Gun",
    name_pt: "Metralhadora Rotativa HIT Mark (Chain-Gun)",
    aliases: &["HIT Mark Chain-Gun", "Chain-Gun", "Chain Gun", "Metralhadora HIT Mark"],
    category: WeaponCategory::TechnocracySidearms,
    difficulty: 6,
    damage: "8/L",
    damage_pt: "8/L",
    range: "150",
    rate: "3",
    clip: "200",
    conceal: "N",
    notes: &["#1", "#2", "#11"],
    page_ref: "M20, p. 452",
    description: "Cyborg-mounted multi-barrel rotary cannon that shreds armor plating and biological targets in milliseconds.",
    description_pt: "Canhão rotativo multivias operado por ciborgues HIT Mark, cortando a armadura inimiga pela metade com saraivadas de alta velocidade.",
};

// --- BOWS AND CROSSBOWS (ARCOS E BESTAS) ---
pub const SHORT_BOW: WeaponDefinition = WeaponDefinition {
    id: "short_bow",
    name: "Short Bow",
    name_pt: "Arco Curto",
    aliases: &["Short Bow", "Arco Curto", "Arco Hípico"],
    category: WeaponCategory::Bows,
    difficulty: 6,
    damage: "4/L",
    damage_pt: "4/L",
    range: "60",
    rate: "1",
    clip: "1",
    conceal: "N",
    notes: &["#6"],
    page_ref: "M20, p. 452",
    description: "Lightweight flexible wooden bow agile on horseback or in dense foliage.",
    description_pt: "Arco ágil de madeira flexível com manuseio rápido a cavalo ou entre folhagens fechadas.",
};

pub const HUNTING_BOW: WeaponDefinition = WeaponDefinition {
    id: "hunting_bow",
    name: "Hunting Bow",
    name_pt: "Arco de Caça (Composto)",
    aliases: &["Hunting Bow", "Arco de Caça", "Compound Bow", "Arco Composto"],
    category: WeaponCategory::Bows,
    difficulty: 6,
    damage: "5/L",
    damage_pt: "5/L",
    range: "100",
    rate: "1",
    clip: "1",
    conceal: "N",
    notes: &["#6"],
    page_ref: "M20, p. 452",
    description: "Modern compound bow using pulleys and fiberglass limbs for maximum draw retention and penetration.",
    description_pt: "Arco composto moderno com roldanas excêntricas e hastes de fibra de vidro para tração e penetração superior.",
};

pub const LONG_BOW: WeaponDefinition = WeaponDefinition {
    id: "long_bow",
    name: "Long Bow",
    name_pt: "Arco Longo (Longbow)",
    aliases: &["Long Bow", "Arco Longo", "English Longbow"],
    category: WeaponCategory::Bows,
    difficulty: 6,
    damage: "5/L",
    damage_pt: "5/L",
    range: "120",
    rate: "1",
    clip: "1",
    conceal: "N",
    notes: &["#6"],
    page_ref: "M20, p. 452",
    description: "Traditional yew-wood bow of great height, sending heavy war shafts across hundreds of yards.",
    description_pt: "Arco histórico de teixo de grande envergadura, disparando flechas de guerra a distâncias impressionantes.",
};

pub const CROSSBOW_COMMANDO: WeaponDefinition = WeaponDefinition {
    id: "crossbow_commando",
    name: "Crossbow, Commando",
    name_pt: "Besta Tática Comando (Pistol Crossbow)",
    aliases: &["Crossbow Commando", "Besta Comando", "Pistol Crossbow", "Besta de Pistola"],
    category: WeaponCategory::Bows,
    difficulty: 6,
    damage: "3/L",
    damage_pt: "3/L",
    range: "20",
    rate: "1",
    clip: "1",
    conceal: "J",
    notes: &["#6", "#7"],
    page_ref: "M20, p. 452",
    description: "Compact tactical one-hand crossbow with folding limbs designed for covert close-range hits.",
    description_pt: "Besta de punho compacta com arco dobrável para assassinatos discretos a curta distância.",
};

pub const CROSSBOW: WeaponDefinition = WeaponDefinition {
    id: "crossbow",
    name: "Crossbow",
    name_pt: "Besta Padrão",
    aliases: &["Crossbow", "Besta", "Balestra"],
    category: WeaponCategory::Bows,
    difficulty: 6,
    damage: "5/L",
    damage_pt: "5/L",
    range: "90",
    rate: "1",
    clip: "1",
    conceal: "T",
    notes: &["#6"],
    page_ref: "M20, p. 452",
    description: "Mechanical stock crossbow that releases bolts with devastating kinetic punch without requiring high muscle strength.",
    description_pt: "Besta mecânica com coronha e gatilho disparando virotes com imenso impacto sem depender da Força do atirador.",
};

pub const CROSSBOW_HVY: WeaponDefinition = WeaponDefinition {
    id: "crossbow_hvy",
    name: "Crossbow, Hvy.",
    name_pt: "Besta Pesada (Arbaleste)",
    aliases: &["Crossbow Hvy", "Besta Pesada", "Arbaleste", "Arbalest"],
    category: WeaponCategory::Bows,
    difficulty: 6,
    damage: "6/L",
    damage_pt: "6/L",
    range: "100",
    rate: "1",
    clip: "1",
    conceal: "N",
    notes: &["#6"],
    page_ref: "M20, p. 452",
    description: "Heavy steel-limbed arbalest cocked with a crank or windlass, delivering armor-splitting bolts.",
    description_pt: "Arbaleste pesada com arco de aço armada por manivela mecânica, perfurando cotas de malha e blindagens.",
};

// --- NON-LETHAL AND PACIFICATION WEAPONS (NÃO-LETAIS) ---
pub const TASER: WeaponDefinition = WeaponDefinition {
    id: "taser",
    name: "Taser",
    name_pt: "Taser (Pistola de Choque)",
    aliases: &["Taser", "Pistola de Choque", "Stun Gun"],
    category: WeaponCategory::NonLethal,
    difficulty: 6,
    damage: "5/B",
    damage_pt: "5/C",
    range: "5",
    rate: "1",
    clip: "1",
    conceal: "P",
    notes: &["#8"],
    page_ref: "M20, p. 452",
    description: "Shoots twin dart electrodes delivering 50,000 volts of neuromuscular incapacitation.",
    description_pt: "Dispara dois dardos condutores descarregando 50.000 volts que paralisam os impulsos neuromusculares do alvo.",
};

pub const TEAR_GAS: WeaponDefinition = WeaponDefinition {
    id: "tear_gas",
    name: "Tear Gas",
    name_pt: "Gás Lacrimogêneo",
    aliases: &["Tear Gas", "Gás Lacrimogêneo", "Gas Lacrimogeneo", "CS Gas"],
    category: WeaponCategory::NonLethal,
    difficulty: 6,
    damage: "3/B",
    damage_pt: "3/C",
    range: "3",
    rate: "1",
    clip: "5",
    conceal: "P",
    notes: &["#8"],
    page_ref: "M20, p. 452",
    description: "Chemical irritant canister provoking blinding pain, choking, and severe dice pool penalties.",
    description_pt: "Composto químico lacrimogêneo provocando tosse asfixiante, cegueira temporária e perda severa de dados de ação.",
};

pub const PACIFICATION_SPRAY: WeaponDefinition = WeaponDefinition {
    id: "pacification_spray",
    name: "Pacification Spray",
    name_pt: "Spray de Pacificação Tecnocrática",
    aliases: &["Pacification Spray", "Spray de Pacificação", "Technocracy Spray"],
    category: WeaponCategory::NonLethal,
    difficulty: 6,
    damage: "5/B",
    damage_pt: "5/C",
    range: "3",
    rate: "1",
    clip: "5",
    conceal: "P",
    notes: &["#8"],
    page_ref: "M20, p. 452",
    description: "High-grade Technocratic chemical aerosol inducing immediate disorientation and docile stupor.",
    description_pt: "Aerossol neural de grau avançado desenvolvido pela Tecnocracia para induzir choque e desorientação instantânea.",
};

// --- HEAVY & MILITARY WEAPONS (LANÇA-FOGUETES & ARMAS PESADAS) ---
pub const MACHINE_GUN_30: WeaponDefinition = WeaponDefinition {
    id: "machine_gun_30",
    name: ".30 Caliber Machine Gun",
    name_pt: "Metralhadora Calibre .30",
    aliases: &["30 Caliber Machine Gun", "Metralhadora .30", "M1919 Browning", "M60"],
    category: WeaponCategory::HeavyAndMilitary,
    difficulty: 6,
    damage: "12/L",
    damage_pt: "12/L",
    range: "800",
    rate: "5",
    clip: "100",
    conceal: "N",
    notes: &["#1", "#9"],
    page_ref: "M20, p. 452",
    description: "Belt-fed medium machine gun delivering relentless sustained automatic fire across infantry battlefields.",
    description_pt: "Metralhadora militar média alimentada por fita de 100 tiros, despejando fogo de saturação contínuo.",
};

pub const MACHINE_GUN_50: WeaponDefinition = WeaponDefinition {
    id: "machine_gun_50",
    name: ".50 Caliber Machine Gun",
    name_pt: "Metralhadora Pesada Calibre .50",
    aliases: &["50 Caliber Machine Gun", "Metralhadora .50", "Browning M2", "Ponto 50"],
    category: WeaponCategory::HeavyAndMilitary,
    difficulty: 6,
    damage: "16/L",
    damage_pt: "16/L",
    range: "1000",
    rate: "5",
    clip: "200",
    conceal: "N",
    notes: &["#1", "#9", "#10"],
    page_ref: "M20, p. 452",
    description: "Immense vehicle-mounted heavy machine gun tearing through light armor, cover, and aircraft.",
    description_pt: "Canhão automático antiveicular de calibre .50 montado em blindados, rasgando paredes de concreto e viaturas.",
};

pub const CANNON_30MM: WeaponDefinition = WeaponDefinition {
    id: "cannon_30mm",
    name: "30 mm Cannon",
    name_pt: "Canhão Automático 30 mm",
    aliases: &["30 mm Cannon", "Canhão 30mm", "Autocannon 30mm", "30mm"],
    category: WeaponCategory::HeavyAndMilitary,
    difficulty: 6,
    damage: "15/L",
    damage_pt: "15/L",
    range: "1200",
    rate: "8",
    clip: "100",
    conceal: "N",
    notes: &["#1", "#9", "#11"],
    page_ref: "M20, p. 452",
    description: "Rotary autocannon mounted on armored fighting vehicles and gunships, devastating hardened bunkers.",
    description_pt: "Canhão giratório montado em aeronaves de ataque e tanques blindados, aniquilando fortificações inteiras.",
};

pub const GRENADE_LAUNCHER_M79: WeaponDefinition = WeaponDefinition {
    id: "grenade_launcher_m79",
    name: "M-79 Grenade Launcher",
    name_pt: "Lança-Granadas M-79",
    aliases: &["M-79", "M79", "Grenade Launcher M79", "Lança Granadas M79"],
    category: WeaponCategory::HeavyAndMilitary,
    difficulty: 6,
    damage: "Explosive (#12)",
    damage_pt: "Explosivo (#12)",
    range: "8",
    rate: "1",
    clip: "1",
    conceal: "N",
    notes: &["#9", "#12"],
    page_ref: "M20, p. 452",
    description: "Single-shot break-action 40mm grenade launcher lobbing explosive shells into enemy positions.",
    description_pt: "Lança-granadas monotiro basculante de 40mm, disparando ogivas de alto impacto explosivo e fragmentação.",
};

pub const GRENADE_LAUNCHER_M19: WeaponDefinition = WeaponDefinition {
    id: "grenade_launcher_m19",
    name: "M-19 Grenade Launcher",
    name_pt: "Lança-Granadas Automático M-19",
    aliases: &["M-19", "M19", "Mk 19", "Automatic Grenade Launcher"],
    category: WeaponCategory::HeavyAndMilitary,
    difficulty: 6,
    damage: "Explosive (#12)",
    damage_pt: "Explosivo (#12)",
    range: "2",
    rate: "1",
    clip: "1",
    conceal: "N",
    notes: &["#9", "#10", "#12"],
    page_ref: "M20, p. 452",
    description: "Belt-fed automatic grenade launcher delivering continuous high-explosive area barrages.",
    description_pt: "Lança-granadas automático montado em tripé ou veículos, saturando setores inteiros com munição de 40mm.",
};

pub const FLAMETHROWER: WeaponDefinition = WeaponDefinition {
    id: "flamethrower",
    name: "Flamethrower",
    name_pt: "Lança-Chamas",
    aliases: &["Flamethrower", "Lança-Chamas", "Lanca Chamas", "Lança Chamas"],
    category: WeaponCategory::HeavyAndMilitary,
    difficulty: 6,
    damage: "Napalm (#12)",
    damage_pt: "Napalm (#12)",
    range: "60",
    rate: "1",
    clip: "Special",
    conceal: "N",
    notes: &["#9", "#12"],
    page_ref: "M20, p. 452",
    description: "Projector tank system shooting streams of ignited thickened fuel that stick and consume everything.",
    description_pt: "Arma com tanques dorsais e projetor ejetando jatos viscosos de combustível inflamado em chamas contínuas.",
};

pub const ROCKET_LAUNCHER: WeaponDefinition = WeaponDefinition {
    id: "rocket_launcher",
    name: "Rocket Launcher (RPG / LAW)",
    name_pt: "Lança-Foguetes (RPG / LAW)",
    aliases: &["Rocket Launcher", "RPG", "LAW", "Bazooka", "Lança Foguetes"],
    category: WeaponCategory::HeavyAndMilitary,
    difficulty: 6,
    damage: "12-16/L (#12)",
    damage_pt: "12-16/L (#12)",
    range: "500+",
    rate: "1",
    clip: "1",
    conceal: "N",
    notes: &["#9", "#12"],
    page_ref: "M20, p. 452",
    description: "Shoulder-fired rocket weapon firing shaped charges designed to destroy battle tanks and bunkers.",
    description_pt: "Lança-rojão disparado sobre o ombro com ogiva de carga oca antitanque, pulverizando alvos e paredes.",
};

pub const ALL_RANGED_WEAPONS: &[WeaponDefinition] = &[
    REVOLVER_LT, REVOLVER_HVY, PISTOL_LT, PISTOL_HVY, RIFLE_HUNTING,
    SMG_SMALL, SMG_LARGE, ASSAULT_RIFLE, SHOTGUN_SAWED_OFF, SHOTGUN_PUMP, SHOTGUN_SEMI_AUTO, SHOTGUN_ASSAULT,
    BIGGS_X5_MODEL_R, BIGGS_X5_MODEL_A, BIGGS_MJOLLNER_MK4, CASTLE_GRAVES_WW3, BOLAN_MK13_SYSTEM, HIT_MARK_CHAIN_GUN,
    SHORT_BOW, HUNTING_BOW, LONG_BOW, CROSSBOW_COMMANDO, CROSSBOW, CROSSBOW_HVY,
    TASER, TEAR_GAS, PACIFICATION_SPRAY,
    MACHINE_GUN_30, MACHINE_GUN_50, CANNON_30MM, GRENADE_LAUNCHER_M79, GRENADE_LAUNCHER_M19, FLAMETHROWER, ROCKET_LAUNCHER,
];

// ============================================================================
// 3. THROWN WEAPONS (ARMAS DE ARREMESSO - M20, p. 452) - 6 ARMAS
// ============================================================================

pub const THROWN_KNIFE: WeaponDefinition = WeaponDefinition {
    id: "thrown_knife",
    name: "Knife (Thrown)",
    name_pt: "Faca de Arremesso",
    aliases: &["Knife (Thrown)", "Faca de Arremesso", "Adaga de Arremesso", "Throwing Knife"],
    category: WeaponCategory::ThrownWeapons,
    difficulty: 6,
    damage: "Strength/L",
    damage_pt: "Força/L",
    range: "Strength x 5",
    rate: "1",
    clip: "1",
    conceal: "P",
    notes: &["#7 (Melee)"],
    page_ref: "M20, p. 452",
    description: "Aerodynamically balanced blade weighted for spin and target penetration when cast by hand.",
    description_pt: "Lâmina com balanceamento aerodinâmico no cabo, equilibrada para rotação estável e penetração letal.",
};

pub const SHURIKEN: WeaponDefinition = WeaponDefinition {
    id: "shuriken",
    name: "Shuriken",
    name_pt: "Shuriken (Estrela Ninja)",
    aliases: &["Shuriken", "Estrela Ninja", "Ninja Star"],
    category: WeaponCategory::ThrownWeapons,
    difficulty: 7,
    damage: "3/L",
    damage_pt: "3/L",
    range: "Strength x 5",
    rate: "2",
    clip: "1",
    conceal: "P",
    notes: &[],
    page_ref: "M20, p. 452",
    description: "Flat multi-pointed steel throwing stars concealed in sleeves for rapid, distracting bleeding cuts.",
    description_pt: "Estrela metálica plana de múltiplas pontas afiadas usada em arremessos velozes e distração tática.",
};

pub const THROWN_SPEAR: WeaponDefinition = WeaponDefinition {
    id: "thrown_spear",
    name: "Spear (Thrown / Javelin)",
    name_pt: "Lança de Arremesso (Dardo / Zagaia)",
    aliases: &["Spear (Thrown)", "Lança de Arremesso", "Dardo", "Javelin", "Zagaia"],
    category: WeaponCategory::ThrownWeapons,
    difficulty: 6,
    damage: "Strength +1/L",
    damage_pt: "Força +1/L",
    range: "Strength x 4",
    rate: "1",
    clip: "1",
    conceal: "N",
    notes: &[],
    page_ref: "M20, p. 452",
    description: "A pointed shaft weighted towards the iron tip for maximum ballistic force over medium ranges.",
    description_pt: "Haste com ponta de aço balanceada para arremesso à distância com peso balístico de alto poder de perfuração.",
};

pub const STONE: WeaponDefinition = WeaponDefinition {
    id: "stone",
    name: "Stone",
    name_pt: "Pedra",
    aliases: &["Stone", "Pedra", "Pedregulho"],
    category: WeaponCategory::ThrownWeapons,
    difficulty: 5,
    damage: "Strength/B",
    damage_pt: "Força/C",
    range: "Strength x 6",
    rate: "1",
    clip: "1",
    conceal: "varies",
    notes: &[],
    page_ref: "M20, p. 452",
    description: "A fist-sized rock readily found on the street or countryside, hurled as a blunt missile.",
    description_pt: "Pedra do tamanho do punho encontrada com facilidade, arremessada como projétil contundente imediato.",
};

pub const STONE_LARGE: WeaponDefinition = WeaponDefinition {
    id: "stone_large",
    name: "Stone, Large",
    name_pt: "Pedra Grande / Paralelepípedo",
    aliases: &["Stone Large", "Pedra Grande", "Paralelepípedo", "Rocha"],
    category: WeaponCategory::ThrownWeapons,
    difficulty: 5,
    damage: "Strength +3/B",
    damage_pt: "Força +3/C",
    range: "Strength x 3",
    rate: "1",
    clip: "1",
    conceal: "N",
    notes: &[],
    page_ref: "M20, p. 452",
    description: "A heavy paving stone or boulder requiring two hands or brute force to crush an adversary.",
    description_pt: "Um paralelepípedo ou bloco pesado de alvenaria arremessado para esmagar com impacto demolidor.",
};

pub const THROWN_TOMAHAWK: WeaponDefinition = WeaponDefinition {
    id: "thrown_tomahawk",
    name: "Tomahawk (Thrown)",
    name_pt: "Tomahawk de Arremesso",
    aliases: &["Tomahawk (Thrown)", "Tomahawk de Arremesso", "Machadinha de Arremesso"],
    category: WeaponCategory::ThrownWeapons,
    difficulty: 6,
    damage: "Strength +1/L",
    damage_pt: "Força +1/L",
    range: "Strength x 4",
    rate: "1",
    clip: "1",
    conceal: "J",
    notes: &[],
    page_ref: "M20, p. 452",
    description: "Hand axe thrown with a spinning arc that embeds the heavy steel bit deeply into target targets.",
    description_pt: "Machadinha balanceada arremessada em arco giratório com a lâmina cravando com violência no alvo.",
};

pub const ALL_THROWN_WEAPONS: &[WeaponDefinition] = &[
    THROWN_KNIFE, SHURIKEN, THROWN_SPEAR, STONE, STONE_LARGE, THROWN_TOMAHAWK,
];

// ============================================================================
// Catálogo Unificado Canônico de M20 (Exatamente 82 Armas)
// ============================================================================

pub const ALL_WEAPONS: &[WeaponDefinition] = &[
    // --- Melee (42) ---
    // Axes (5)
    HATCHET, TOMAHAWK, AXE, GREAT_AXE, POLEARM,
    // Blades (8)
    STILETTO, KNIFE, SHORT_SWORD, SWORD, KATANA, GREAT_SWORD, SAI, HOOK_SWORD,
    // Clubbing (9)
    RIOT_BATON, BASEBALL_BAT, CROWBAR, STAFF, IRON_STAFF, MACE, NUNCHAKU, SPIKED_CLUB, HUGE_SPIKED_CLUB,
    // Fist-Extension (8)
    SAP, BRASS_KNUCKLES, SPIKED_GAUNTLET, HAND_CLAWS_SMALL, HAND_CLAWS_LARGE, KATAR, WAR_FAN, WIND_AND_FIRE_WHEEL,
    // Improvised (4)
    BROKEN_BOTTLE, CHAIR, CHAINSAW, TABLE,
    // Whips and Chains (8)
    CHAIN, CHAIN_WHIP, KUSARIGAMA, MANRIKI_GUSARI, FLOGGER, BARBED_CAT, WHIP, BULLWHIP,

    // --- Ranged (34) ---
    // Conventional Guns (12)
    REVOLVER_LT, REVOLVER_HVY, PISTOL_LT, PISTOL_HVY, RIFLE_HUNTING,
    SMG_SMALL, SMG_LARGE, ASSAULT_RIFLE, SHOTGUN_SAWED_OFF, SHOTGUN_PUMP, SHOTGUN_SEMI_AUTO, SHOTGUN_ASSAULT,
    // Technocracy Sidearms (6)
    BIGGS_X5_MODEL_R, BIGGS_X5_MODEL_A, BIGGS_MJOLLNER_MK4, CASTLE_GRAVES_WW3, BOLAN_MK13_SYSTEM, HIT_MARK_CHAIN_GUN,
    // Bows & Crossbows (6)
    SHORT_BOW, HUNTING_BOW, LONG_BOW, CROSSBOW_COMMANDO, CROSSBOW, CROSSBOW_HVY,
    // Non-Lethal (3)
    TASER, TEAR_GAS, PACIFICATION_SPRAY,
    // Military Weapons (7)
    MACHINE_GUN_30, MACHINE_GUN_50, CANNON_30MM, GRENADE_LAUNCHER_M79, GRENADE_LAUNCHER_M19, FLAMETHROWER, ROCKET_LAUNCHER,

    // --- Thrown (6) ---
    THROWN_KNIFE, SHURIKEN, THROWN_SPEAR, STONE, STONE_LARGE, THROWN_TOMAHAWK,
];

pub fn find_weapon(query: &str) -> Option<&'static WeaponDefinition> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return None;
    }

    // 1. Match exato por id
    if let Some(w) = ALL_WEAPONS.iter().find(|w| w.id.eq_ignore_ascii_case(&q)) {
        return Some(w);
    }

    // 2. Match exato por name ou name_pt
    if let Some(w) = ALL_WEAPONS.iter().find(|w| {
        w.name.to_lowercase() == q || w.name_pt.to_lowercase() == q
    }) {
        return Some(w);
    }

    // 3. Match em aliases
    if let Some(w) = ALL_WEAPONS.iter().find(|w| {
        w.aliases.iter().any(|alias| alias.to_lowercase() == q)
    }) {
        return Some(w);
    }

    // 4. Substring no nome
    ALL_WEAPONS.iter().find(|w| {
        w.name.to_lowercase().contains(&q)
            || w.name_pt.to_lowercase().contains(&q)
            || w.aliases.iter().any(|alias| alias.to_lowercase().contains(&q))
    })
}

pub fn get_weapons_by_category(cat: WeaponCategory) -> Vec<&'static WeaponDefinition> {
    ALL_WEAPONS.iter().filter(|w| w.category == cat).collect()
}

pub fn get_weapons_by_class(w_class: WeaponClass) -> Vec<&'static WeaponDefinition> {
    ALL_WEAPONS.iter().filter(|w| w.category.weapon_class() == w_class).collect()
}

pub fn get_weapons_by_main_group(group: WeaponMainGroup) -> Vec<&'static WeaponDefinition> {
    ALL_WEAPONS.iter().filter(|w| w.category.main_group() == group).collect()
}

pub fn get_categories_by_main_group(group: WeaponMainGroup) -> &'static [WeaponCategory] {
    match group {
        WeaponMainGroup::Melee => MELEE_CATEGORIES,
        WeaponMainGroup::Ranged => RANGED_CATEGORIES,
    }
}

// ============================================================================
// Manobras de Combate Corpo a Corpo (Briga, Luta Suja & Regras Especiais)
// Canônicas de M20 (Capítulo 9: Combat & Storytelling, pp. 450-453)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManeuverCategory {
    General,       // Manobras Gerais de Corpo a Corpo
    DirtyFighting, // Luta Suja (Briga 3+)
    MartialArts,   // Artes Marciais (M20 pp. 423-426)
    Do,            // Dô (Irmandade de Akashayana / M20)
    SpecialRules,  // Regras Especiais (Duas Armas, etc.)
}

impl ManeuverCategory {
    pub fn name(&self, lang: Language) -> &'static str {
        match (self, lang) {
            (ManeuverCategory::General, Language::PtBr) => "Manobras Gerais",
            (ManeuverCategory::General, Language::EnUs) => "General Maneuvers",
            (ManeuverCategory::DirtyFighting, Language::PtBr) => "Luta Suja (Briga 3+)",
            (ManeuverCategory::DirtyFighting, Language::EnUs) => "Dirty Fighting (Brawl 3+)",
            (ManeuverCategory::MartialArts, Language::PtBr) => "Artes Marciais",
            (ManeuverCategory::MartialArts, Language::EnUs) => "Martial Arts",
            (ManeuverCategory::Do, Language::PtBr) => "Dô (Akashayana)",
            (ManeuverCategory::Do, Language::EnUs) => "Do (Akashic Brotherhood)",
            (ManeuverCategory::SpecialRules, Language::PtBr) => "Regras Especiais",
            (ManeuverCategory::SpecialRules, Language::EnUs) => "Special Rules",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ManeuverCategory::General => "👊",
            ManeuverCategory::DirtyFighting => "🥊",
            ManeuverCategory::MartialArts => "🥋",
            ManeuverCategory::Do => "🪷",
            ManeuverCategory::SpecialRules => "⚡",
        }
    }
}

pub const ALL_MANEUVER_CATEGORIES: [ManeuverCategory; 5] = [
    ManeuverCategory::General,
    ManeuverCategory::DirtyFighting,
    ManeuverCategory::MartialArts,
    ManeuverCategory::Do,
    ManeuverCategory::SpecialRules,
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CombatManeuver {
    pub id: &'static str,
    pub name: &'static str,
    pub name_pt: &'static str,
    pub category: ManeuverCategory,
    pub roll: &'static str,
    pub roll_pt: &'static str,
    pub damage: &'static str,
    pub damage_pt: &'static str,
    pub difficulty: &'static str,
    pub actions: i32,
    pub requirement: &'static str,
    pub requirement_pt: &'static str,
    pub description: &'static str,
    pub description_pt: &'static str,
}

impl CombatManeuver {
    pub fn name(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt,
            Language::EnUs => self.name,
        }
    }

    pub fn roll(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.roll_pt,
            Language::EnUs => self.roll,
        }
    }

    pub fn damage(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.damage_pt,
            Language::EnUs => self.damage,
        }
    }

    pub fn requirement(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.requirement_pt,
            Language::EnUs => self.requirement,
        }
    }

    pub fn description(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.description_pt,
            Language::EnUs => self.description,
        }
    }

    pub fn difficulty(&self, lang: Language) -> &'static str {
        match lang {
            Language::EnUs => self.difficulty,
            Language::PtBr => match self.difficulty {
                "7 / Willpower + 3 / 8" => "7 / Vontade + 3 / 8",
                "hard 5 / soft 6" => "duro 5 / suave 6",
                "hard 6 / soft 5" => "duro 6 / suave 5",
                "hard 7 / soft 6" => "duro 7 / suave 6",
                "+1 usual / 8 (bare hands) / 7 (Martial Arts 3+)" => "+1 habitual / 8 (mãos nuas) / 7 (Artes Marciais 3+)",
                "-1 (Flank) / -2 (Rear)" => "-1 (Flanco) / -2 (Retaguarda)",
                "Normal (main hand) / +1 (off hand)" => "Normal (mão hábil) / +1 (mão inábil)",
                "7 (deflect) / 9 (catch & throw)" => "7 (desviar) / 9 (apanhar e arremessar)",
                "6 (or 7 if attack)" => "6 (ou 7 se ataque)",
                "As weapon / 6" => "Como a arma / 6",
                other => other,
            },
        }
    }

    pub fn to_weapon_item(&self, lang: Language) -> crate::state::WeaponItem {
        crate::state::WeaponItem {
            name: self.name(lang).to_string(),
            diff: self.difficulty(lang).to_string(),
            damage: self.damage(lang).to_string(),
            range: match lang {
                Language::PtBr => "Corpo a corpo".to_string(),
                Language::EnUs => "Close".to_string(),
            },
            rate: self.actions.to_string(),
            clip: "—".to_string(),
            conceal: "—".to_string(),
            notes: self.requirement(lang).to_string(),
        }
    }
}

pub const ALL_COMBAT_MANEUVERS: &[CombatManeuver] = &[
    // ------------------------------------------------------------------------
    // Manobras Gerais de Corpo a Corpo (General Hand-to-Hand)
    // ------------------------------------------------------------------------
    CombatManeuver {
        id: "bite",
        name: "Bite",
        name_pt: "Mordida",
        category: ManeuverCategory::General,
        roll: "Dexterity + Brawl",
        roll_pt: "Destreza + Briga",
        damage: "Strength + 1 / B or L",
        damage_pt: "Força + 1 / C ou L",
        difficulty: "5",
        actions: 1,
        requirement: "Creatures with teeth",
        requirement_pt: "Criaturas com dentes",
        description: "Creatures with teeth can bite. Human jaws normally inflict Strength in bashing damage, but a shapechanged mage’s teeth can deal out lethal harm instead. Certain Night-Folk inflict aggravated damage with their bites, but a mage – even in a more dangerous form – inflicts lethal damage only. Typically, a bite attack causes Strength +1 damage; an especially large mouth or teeth, however, may add +2 or even +3 to that attack.",
        description_pt: "Criaturas com dentes podem morder. Mandíbulas humanas normalmente causam Força em dano contundente, mas os dentes de um mago metamorfo podem causar dano letal. Certos Membros do Povo da Noite causam dano agravado com suas mordidas, mas um mago – mesmo em forma mais perigosa – causa apenas dano letal. Tipicamente, um ataque de mordida causa dano de Força +1; bocas ou dentes excepcionalmente grandes, contudo, podem somar +2 ou +3 a esse ataque.",
    },
    CombatManeuver {
        id: "claw",
        name: "Claw",
        name_pt: "Garras",
        category: ManeuverCategory::General,
        roll: "Dexterity + Brawl",
        roll_pt: "Destreza + Briga",
        damage: "Strength + 1 to + 3 / L or A",
        damage_pt: "Força + 1 a + 3 / L ou A",
        difficulty: "6",
        actions: 1,
        requirement: "Claws, implants or shapechanged",
        requirement_pt: "Garras, implantes ou metamorfoseado",
        description: "Cyborgs, beasts, shapechanged mages, and folks wearing surgically implanted claws can slash their enemies. Again, this inflicts lethal, not aggravated, damage unless the attacker has some sort of special advantage like Primium claws or Quintessence enhancement. Normal claws inflict Strength +1; Wolverine-style claws dish out Strength +2, and terrifying monster claws deal out Strength +3.",
        description_pt: "Ciborgues, feras, magos metamorfos e indivíduos com garras implantadas cirurgicamente podem retalhar seus inimigos. Novamente, isso causa dano letal (não agravado), a menos que o atacante possua uma vantagem especial como garras de Primium ou aprimoramento por Quintessência. Garras normais causam Força +1; garras estilo Wolverine causam Força +2, e garras monstruosas aterrorizantes podem infligir Força +3.",
    },
    CombatManeuver {
        id: "disarm",
        name: "Disarm",
        name_pt: "Desarmar",
        category: ManeuverCategory::General,
        roll: "Dexterity + Melee (or Brawl / Martial Arts)",
        roll_pt: "Destreza + Armas Brancas (ou Briga / Artes Marciais)",
        damage: "Special (Drops Weapon)",
        damage_pt: "Especial (Derruba a Arma)",
        difficulty: "+1 usual / 8 (bare hands) / 7 (Martial Arts 3+)",
        actions: 1,
        requirement: "None",
        requirement_pt: "Nenhum",
        description: "With a clever strike and twist, the fighter removes her opponent’s weapon from his hand. To succeed, the attacker rolls Dexterity + Melee at +1 to her usual difficulty; if her successes exceed her opponent’s Strength score, then he drops that weapon. If she doesn’t score enough successes to disarm the opponent, she still inflicts her usual damage. If she botches that roll, she loses her own weapon instead.\n\nA brave or skillful attacker can try to disarm an armed opponent with her bare hands. In this case, the roll is Dexterity + Brawl, the difficulty is 8, and she subtracts one die from her usual attack dice pool. A character with at least three dots in the Martial Arts Skill can perform this bare-handed disarm maneuver at difficulty 7 instead; in this case, the roll is Dexterity + Martial Arts, not Brawl.",
        description_pt: "Com um golpe hábil e uma torção, o lutador arranca a arma da mão do oponente. Para ter sucesso, o atacante rola Destreza + Armas Brancas com +1 na dificuldade habitual; se seus sucessos superarem a Força do oponente, este solta a arma. Se não obtiver sucessos suficientes para desarmar, ainda causa o dano normal do golpe. Se tiver uma falha crítica, perde sua própria arma.\n\nUm atacante corajoso ou habilidoso pode tentar desarmar um inimigo armado com as próprias mãos desarmadas. Nesse caso, o teste é Destreza + Briga, dificuldade 8, e subtrai um dado de sua parada. Um personagem com pelo menos três pontos em Artes Marciais pode realizar esse desarme desarmado na dificuldade 7 (Destreza + Artes Marciais).",
    },
    CombatManeuver {
        id: "flank_rear",
        name: "Flank or Rear Attacks",
        name_pt: "Ataques de Flanco ou Retaguarda",
        category: ManeuverCategory::General,
        roll: "Normal attack roll",
        roll_pt: "Teste normal de ataque",
        damage: "Normal",
        damage_pt: "Normal",
        difficulty: "-1 (Flank) / -2 (Rear)",
        actions: 1,
        requirement: "Tactical positioning",
        requirement_pt: "Posicionamento tático vantajoso",
        description: "By attacking an opponent’s flank, an attacker reduces his difficulty by -1; by attacking from the rear, he reduces it by -2.",
        description_pt: "Ao atacar o flanco de um oponente, o atacante reduz a dificuldade de seu ataque em -1; ao atacar pelas costas (retaguarda), a dificuldade é reduzida em -2.",
    },
    CombatManeuver {
        id: "grapple",
        name: "Grapple",
        name_pt: "Agarrar / Imobilizar (Clinch)",
        category: ManeuverCategory::General,
        roll: "Strength + Brawl, Martial Arts, or Do",
        roll_pt: "Força + Briga, Artes Marciais ou Dô",
        damage: "Strength or none / B (L with spikes/claws)",
        damage_pt: "Força ou nenhum / C (L com espinhos/garras)",
        difficulty: "6",
        actions: 1,
        requirement: "Limbs free",
        requirement_pt: "Mãos livres",
        description: "Grabbing hold of her antagonist, the attacker tries to either immobilize him (a hold) or crush him (a clinch). The latter option requires strength, but Life Sphere magick makes many things possible.\n\nTo grapple, the attacker rolls Strength + Brawl, Martial Arts, or Do. If she wants to inflict damage, the player can roll Strength for damage, beginning on the next turn and continuing until her opponent breaks free (who can soak normally). If instead she holds her opponent still, he’s stuck in her grip until his next action.\n\nTo escape, the opponent can break free in a resisted Strength + Brawl roll, or reverse the hold with two successes more. An agile character can use Dexterity to escape; Acrobatics, Martial Arts, or Do can replace Brawl.",
        description_pt: "Segurando seu antagonista, o atacante tenta imobilizá-lo (uma chave) ou esmagá-lo (clinch). Esta última opção exige força bruta, mas a mágica da Esfera da Vida torna muitas coisas possíveis.\n\nPara agarrar, o atacante faz um teste de Força + Briga, Artes Marciais ou Dô. Se desejar infligir dor, o jogador pode rolar Força para causar dano a partir do turno seguinte e continuamente até que o oponente se solte (podendo absorver normalmente). Se optar por imobilizar, o alvo fica preso até sua próxima ação.\n\nPara escapar, o oponente pode se soltar vencendo um teste resistido de Força + Briga, ou reverter a chave obtendo dois sucessos a mais que o atacante. Personagens ágeis podem usar Destreza, Acrobacia, Artes Marciais ou Dô.",
    },
    CombatManeuver {
        id: "kick",
        name: "Kick",
        name_pt: "Chute",
        category: ManeuverCategory::General,
        roll: "Dexterity + Brawl",
        roll_pt: "Destreza + Briga",
        damage: "Strength + 1 / B",
        damage_pt: "Força + 1 / C",
        difficulty: "7",
        actions: 1,
        requirement: "None",
        requirement_pt: "Nenhum",
        description: "Lashing out with a leg, the character hits an opponent where it hurts. Kicks normally inflict bashing damage. A cyborg, HIT Mark, or other massive attacker, though, may deal out lethal damage from sheer density.",
        description_pt: "Desferindo um golpe com a perna, o personagem atinge o oponente onde mais dói. Chutes normalmente causam dano contundente (Força +1). Ciborgues, HIT Marks ou atacantes de grande densidade podem causar dano letal pelo puro peso do impacto.",
    },
    CombatManeuver {
        id: "punch",
        name: "Punch",
        name_pt: "Soco",
        category: ManeuverCategory::General,
        roll: "Dexterity + Brawl",
        roll_pt: "Destreza + Briga",
        damage: "Strength / B",
        damage_pt: "Força / C",
        difficulty: "6",
        actions: 1,
        requirement: "None",
        requirement_pt: "Nenhum",
        description: "One fist, one face, one very basic attack. Most punches inflict bashing damage, but punches by metal-handed cyborgs or reinforced gauntlets can do lethal harm instead.",
        description_pt: "Um punho, um rosto, o ataque mais elementar. A maioria dos socos causa dano contundente equivalente à Força, mas golpes de ciborgues com punhos de aço ou manoplas reforçadas podem infligir dano letal.",
    },
    CombatManeuver {
        id: "reading_opponent",
        name: "Reading an Opponent",
        name_pt: "Ler um Oponente",
        category: ManeuverCategory::General,
        roll: "Perception + Combat Trait (vs Wits + Combat Trait)",
        roll_pt: "Percepção + Traço de Combate (vs Raciocínio + Traço)",
        damage: "N/A (+1 die per success on next action)",
        damage_pt: "N/A (+1 dado por sucesso na próxima ação)",
        difficulty: "6",
        actions: 1,
        requirement: "Perception & Combat Trait (max Wits times/scene)",
        requirement_pt: "Percepção & Combate (máx vezes na cena = Raciocínio)",
        description: "Mages specialize in out-thinking their opponents. With the Storyteller’s permission, a hand-to-hand combatant may try to read an opponent’s intentions with a roll of Perception + either Brawling, Do, or Martial Arts. This becomes a resisted roll, pitting the observer’s Perception + combat Trait against her opponent’s Wits + combat Trait. If the observer succeeds, she adds one die per success to the dice pool to perform her next action.\n\nDemands one action. The character may attempt to do this only once per combat scene for each dot she has in Wits. Beyond 20 feet, this does not work without Correspondence or Mind magick.",
        description_pt: "Magos se especializam em superar seus oponentes pela inteligência. Um combatente corpo a corpo pode tentar antecipar as intenções do inimigo com um teste de Percepção + Briga, Dô ou Artes Marciais, resistido por Raciocínio + Traço de Combate do alvo. Se o observador vencer, ganha +1 dado em sua próxima ação por sucesso obtido.\n\nExige 1 ação. O personagem só pode tentar essa manobra um número de vezes por cena igual aos seus pontos em Raciocínio. A distâncias superiores a 6 metros, exige mágica de Correspondência ou Mente.",
    },
    CombatManeuver {
        id: "sweep",
        name: "Sweep",
        name_pt: "Rasteira",
        category: ManeuverCategory::General,
        roll: "Dexterity + Brawl or Melee",
        roll_pt: "Destreza + Briga ou Armas Brancas",
        damage: "None / B (Knocks to ground)",
        damage_pt: "Nenhum / C (Derruba no chão)",
        difficulty: "8",
        actions: 1,
        requirement: "Legs or appropriate weapon",
        requirement_pt: "Pernas ou arma apropriada",
        description: "With a weapon (Melee) or leg (Brawl, Martial Arts, or Do), our fighter knocks his enemy’s own legs out from under him. If the blow succeeds, the antagonist winds up on the ground that turn placing him at a serious disadvantage.",
        description_pt: "Com uma arma (Armas Brancas) ou perna (Briga, Artes Marciais ou Dô), o lutador ceifa a base do oponente por baixo dele. Se acertar, o adversário vai ao chão imediatamente, ficando vulnerável e em grande desvantagem no combate.",
    },
    CombatManeuver {
        id: "tackle",
        name: "Tackle",
        name_pt: "Investida (Derrubar)",
        category: ManeuverCategory::General,
        roll: "Dexterity + Brawl or Athletics",
        roll_pt: "Destreza + Briga ou Atletismo",
        damage: "Strength / B or L",
        damage_pt: "Força / C ou L",
        difficulty: "7",
        actions: 1,
        requirement: "At least 2 yards of running distance",
        requirement_pt: "Ao menos 2 metros de corrida livre",
        description: "Employing a mix of weight and momentum, the attacker throws herself into her opponent. A tackle requires at least two yards of distance. Each combatant must make a successful Dexterity + Athletics roll (diff 6 for attacker, 6 + attacker’s successes for target), or else wind up sprawled on the ground afterward.\n\nFor the most part, tackle damage is bashing. A person crushed beneath a flying cyborg or mass-enhanced body (Life/Forces), however, inflicts lethal damage instead.",
        description_pt: "Usando uma combinação de massa e aceleração, o atacante se arremessa contra o oponente para contê-lo ou impedir sua fuga. Exige pelo menos 2 metros de distância de corrida. Ambos os lutadores devem fazer um teste de Destreza + Atletismo (dificuldade 6 para o atacante; 6 + sucessos do atacante para o alvo), sob pena de caírem esparramados no chão.\n\nO dano é contundente, tornando-se letal caso o atacante seja um ciborgue pesado ou tenha sua massa/momento potencializados por mágica de Vida ou Forças.",
    },

    // ------------------------------------------------------------------------
    // Luta Suja (Dirty Fighting — Requer Briga 3+)
    // ------------------------------------------------------------------------
    CombatManeuver {
        id: "blinding",
        name: "Blinding",
        name_pt: "Cegamento",
        category: ManeuverCategory::DirtyFighting,
        roll: "Dexterity + Subterfuge",
        roll_pt: "Destreza + Subterfúgio",
        damage: "Special (Blinded 1 turn/success, -2 dice)",
        damage_pt: "Especial (Cego 1 turno/sucesso, -2 dados)",
        difficulty: "9",
        actions: 1,
        requirement: "Brawl 3+ and street background",
        requirement_pt: "Briga 3+ e vivência de rua",
        description: "Scratching an enemy’s eyes, throwing dirt in his face, spraying him with chemicals or using some similar attack, the brawler tries to render her opponent sightless. If she succeeds, the enemy gets blinded for one turn per success and loses two dice from his dice pools until he recovers. Five successes or more may destroy the eyes completely.",
        description_pt: "Arranhando os olhos do inimigo, jogando terra ou areia na face, borrifando produtos químicos ou golpeando sorrateiramente, o lutador tenta cegar o oponente. Com sucesso, o adversário fica cego por 1 turno por sucesso e perde 2 dados de todas as suas paradas de ação até se recuperar. Cinco ou mais sucessos podem destruir a visão permanentemente.",
    },
    CombatManeuver {
        id: "body_slam",
        name: "Body Slam",
        name_pt: "Bate-Estaca",
        category: ManeuverCategory::DirtyFighting,
        roll: "Dexterity + Brawl",
        roll_pt: "Destreza + Briga",
        damage: "Strength + 2 / B or L",
        damage_pt: "Força + 2 / C ou L",
        difficulty: "6",
        actions: 2,
        requirement: "Brawl 3+, requires successful grapple",
        requirement_pt: "Briga 3+, exige agarrão bem-sucedido",
        description: "Lifting his opponent up over his head, a strong fighter can use brute strength and momentum to burst various important innards. Requires a successful grappling attack. If the enemy can’t break free within that turn, the brawler can use his next action to slam his foe into the nearest and most painful surface (Strength +2 damage).",
        description_pt: "Erguendo o oponente acima da cabeça, o lutador usa força bruta e gravidade para arrebentar o corpo do adversário contra o solo ou paredes. Exige um agarrão anterior bem-sucedido. Se o alvo não se soltar, a próxima ação do lutador é arremessá-lo violentamente contra a superfície mais dura e dolorosa próxima (Força +2 de dano).",
    },
    CombatManeuver {
        id: "curbstomp",
        name: "Curbstomp",
        name_pt: "Pisoteamento",
        category: ManeuverCategory::DirtyFighting,
        roll: "Strength + Brawl",
        roll_pt: "Força + Briga",
        damage: "Strength + 2 / L",
        damage_pt: "Força + 2 / L",
        difficulty: "6",
        actions: 2,
        requirement: "Brawl 3+, target must be stunned or immobilized",
        requirement_pt: "Briga 3+, alvo atordoado ou imobilizado no chão",
        description: "An ugly way of rearranging someone’s face involves propping said face against a curb or hard surface and stomping down on the back of his head. Requires a stunned or immobilized target, plus one turn to place his face into position. Inflicts lethal damage (Strength +2) and lasting horrific injuries (broken teeth, shattered jaw, ruined face).",
        description_pt: "Uma tática brutal das ruas: apoiar a face do alvo contra o meio-fio ou superfície de concreto e desferir um pisão impiedoso na nuca. Exige que o oponente esteja atordoado ou imobilizado no chão e consome 1 turno para posicioná-lo. Causa dano letal (Força +2) e traumas grotescos (maxilar quebrado, dentes destruídos, deformidade facial).",
    },
    CombatManeuver {
        id: "haymaker",
        name: "Haymaker",
        name_pt: "Cruzado Selvagem",
        category: ManeuverCategory::DirtyFighting,
        roll: "Dexterity + Brawl",
        roll_pt: "Destreza + Briga",
        damage: "Strength + 2 / B (Knockback)",
        damage_pt: "Força + 2 / C (Empurrão)",
        difficulty: "7",
        actions: 1,
        requirement: "Brawl 3+",
        requirement_pt: "Briga 3+",
        description: "Making like Popeye the Sailor, the brawler winds up, spots an opening, and hurls everything she has into a jaw-cracking punch. If successful, this blow knocks the opponent back a yard or so unless he makes a Strength roll (difficulty 8); if he botches, he hits the ground.",
        description_pt: "Como um golpe fulminante de marinheiro, o lutador gira o tronco, aproveita uma brecha na guarda e lança todo o peso de seu corpo em um soco devastador. Em caso de sucesso, empurra o oponente 1 metro para trás a menos que ele passe num teste de Força (dif 8); se falhar criticamente, beija o asfalto.",
    },
    CombatManeuver {
        id: "head_butt",
        name: "Head Butt",
        name_pt: "Cabeçada",
        category: ManeuverCategory::DirtyFighting,
        roll: "Dexterity + Brawl",
        roll_pt: "Destreza + Briga",
        damage: "Strength + 1 / B",
        damage_pt: "Força + 1 / C",
        difficulty: "6",
        actions: 1,
        requirement: "Brawl 3+, close clinch range",
        requirement_pt: "Briga 3+, combate agarrado (corpo a corpo colado)",
        description: "In a close-clinch situation, the attacker can smash her head into the face or stomach of an opponent. Benefit comes from surprise. Inflicts bashing damage. If the target soaks all damage, however, the attacker is stunned for one turn. Head butting a cyborg causes the attacker to suffer the damage herself!",
        description_pt: "Em distância de clinch ou agarramento, o lutador desfere uma cabeçada violenta no nariz ou estômago do oponente aproveitando o fator surpresa. Causa dano contundente de Força +1. Atenção: se o alvo absorver todo o dano, o próprio atacante fica atordoado por 1 turno! Dar cabeçada em ciborgues faz o atacante sofrer o próprio dano.",
    },
    CombatManeuver {
        id: "jab_pistol",
        name: "Jab Pistol",
        name_pt: "Disparo à Queima-Roupa",
        category: ManeuverCategory::DirtyFighting,
        roll: "Dexterity + Brawl",
        roll_pt: "Destreza + Briga",
        damage: "As per firearm + 2 / L",
        damage_pt: "Conforme arma de fogo + 2 / L",
        difficulty: "6",
        actions: 1,
        requirement: "Brawl 3+, pistol in hand, close combat",
        requirement_pt: "Briga 3+, pistola empunhada em combate corpo a corpo",
        description: "Jamming a firearm directly into his opponent’s body, the attacker blasts a rather large hole in her. Risky and messy, this move forces the attacker to get as close as possible. Because the gun is used hand-to-hand, the maneuver employs Brawl instead of Firearms. Only pistols can be used this way, dealing firearm damage +2.",
        description_pt: "Pressionando o cano da arma de fogo diretamente contra o peito ou abdômen do oponente em plena briga física, o atirador puxa o gatilho abrindo um rombo devastador. Como a arma é engatilhada colada na luta corporal, usa-se Briga no lugar de Armas de Fogo. Apenas pistolas podem ser usadas assim, adicionando +2 ao dano padrão da arma.",
    },
    CombatManeuver {
        id: "low_blow",
        name: "Low Blow",
        name_pt: "Golpe Baixo",
        category: ManeuverCategory::DirtyFighting,
        roll: "Dexterity + Brawl",
        roll_pt: "Destreza + Briga",
        damage: "Strength + Stun / B or L",
        damage_pt: "Força + Atordoamento / C ou L",
        difficulty: "7",
        actions: 1,
        requirement: "Brawl 3+",
        requirement_pt: "Briga 3+",
        description: "Going for the takedown, a brawler aims a shot at some location – kidneys, septum, genitals – that’s sure to hurt a lot. If successful, the target winds up stunned for one turn for each health level inflicted after soak (can resist with Stamina roll diff 8).",
        description_pt: "Mirando pontos hipersensíveis — genitais, rim, septo nasal ou plexo solar —, o lutador busca a incapacitação imediata pela dor. O alvo fica atordoado por 1 turno para cada nível de vitalidade sofrido após a absorção, podendo tentar resistir com um teste de Vigor (dificuldade 8).",
    },
    CombatManeuver {
        id: "pistol_whip",
        name: "Pistol Whip",
        name_pt: "Coronhada",
        category: ManeuverCategory::DirtyFighting,
        roll: "Dexterity + Melee",
        roll_pt: "Destreza + Armas Brancas",
        damage: "Strength + 2 / L",
        damage_pt: "Força + 2 / L",
        difficulty: "7",
        actions: 1,
        requirement: "Brawl 3+, gun or heavy object in hand",
        requirement_pt: "Briga 3+, arma de fogo ou objeto pesado em mãos",
        description: "Adding emphasis to injury, the brawler cracks his opponent across the head or face with a gun or other small, hard, heavy object (crowbar, mug, laptop). Inflicts lethal damage (Strength +2) and may stun the opponent for one turn on failed Stamina roll (diff 8) or two turns on botch.",
        description_pt: "Golpeando o rosto ou têmpora do oponente com a coronha de metal da pistola (ou objeto pesado como pé de cabra ou notebook), o lutador desfere um traumatismo violento. Causa dano letal de Força +2 e atordoa por 1 turno caso o alvo falhe em teste de Vigor (dif 8), ou por 2 turnos se tiver falha crítica.",
    },

    // ------------------------------------------------------------------------
    // Artes Marciais Canônicas (Martial Arts - M20 pp. 423-426)
    // ------------------------------------------------------------------------
    CombatManeuver {
        id: "counter_throw",
        name: "Counter Throw",
        name_pt: "Contra-Arremesso",
        category: ManeuverCategory::MartialArts,
        roll: "Dexterity + Martial Arts (resisted vs attack)",
        roll_pt: "Destreza + Artes Marciais (resistido vs ataque)",
        damage: "As Throw",
        damage_pt: "Como Arremesso",
        difficulty: "6",
        actions: 1,
        requirement: "Martial Arts 1, Athletics 1 (soft style)",
        requirement_pt: "Artes Marciais 1, Atletismo 1 (estilo suave)",
        description: "Directing an attacker’s momentum against him, the martial artist sends him flying into the nearest wall… or the ground… or worse. A basic soft-style maneuver, this remains an effective technique.\n\nWhen the martial artist is attacked, make a resisted roll of Dexterity + Martial Arts against the attacker’s Dexterity + Brawl, Melee, or Martial Arts (depending on the Ability the opponent uses). If you score more successes than your attacker, you deflect the attack and may – immediately and without having to divide your dice pool – try to throw him (as per Throw). If you fail the throw, you still sidestep his attack.",
        description_pt: "Direcionando o impulso do atacante contra ele mesmo, o artista marcial o arremessa voando contra a parede mais próxima... ou o chão... ou pior. Uma técnica clássica e extremamente eficaz do estilo suave.\n\nAo ser atacado, faça um teste resistido de Destreza + Artes Marciais contra a Destreza + Briga, Armas Brancas ou Artes Marciais do adversário. Se obtiver mais sucessos que o agressor, você deflete o golpe e pode — imediatamente e sem precisar dividir sua parada de dados — tentar arremessá-lo. Se falhar no arremesso, ainda assim consegue se esquivar do golpe.",
    },
    CombatManeuver {
        id: "death_strike",
        name: "Death Strike",
        name_pt: "Golpe Mortal",
        category: ManeuverCategory::MartialArts,
        roll: "Dexterity + Martial Arts",
        roll_pt: "Destreza + Artes Marciais",
        damage: "Strength + 2 / L",
        damage_pt: "Força + 2 / L",
        difficulty: "5",
        actions: 1,
        requirement: "Martial Arts 3 (hard style)",
        requirement_pt: "Artes Marciais 3 (estilo duro)",
        description: "Aiming a rigid hand at an organ, joint, or other incapacitating location, the attacker directs devastating force at that target. This strike inflicts lethal damage.",
        description_pt: "Mirando uma mão rígida como ponta de lança contra um órgão interno, juntura ou ponto incapacitante, o atacante canaliza uma energia devastadora no alvo. Este golpe inflige dano letal.",
    },
    CombatManeuver {
        id: "deflecting_block",
        name: "Deflecting Block",
        name_pt: "Bloqueio Defletor",
        category: ManeuverCategory::MartialArts,
        roll: "Dexterity + Martial Arts",
        roll_pt: "Destreza + Artes Marciais",
        damage: "Attacker’s Strength / B",
        damage_pt: "Força do atacante / C",
        difficulty: "6",
        actions: 1,
        requirement: "Martial Arts 2 (soft style), Athletics or Acrobatics 2",
        requirement_pt: "Artes Marciais 2 (estilo suave), Atletismo ou Acrobacia 2",
        description: "Evading an attacker’s blow, the martial artist redirects the attacker’s force against her. In game terms, the defender rolls Dexterity + Martial Arts in Phase Two: Defense. Each success subtracts one success from the attacker’s roll. If the defender scores more successes than his attacker, then the attacker must roll Dexterity (difficulty 8) or else fall to the ground (or smash into a nearby surface), taking her own Strength in bashing damage.",
        description_pt: "Esquivando-se do golpe do agressor, o artista marcial redireciona o vetor de força do atacante contra ele. O defensor rola Destreza + Artes Marciais na Fase de Defesa. Cada sucesso subtrai um sucesso do teste do atacante. Se o defensor obtiver mais sucessos que o agressor, o atacante deve rolar Destreza (dificuldade 8) ou cairá esparramado no chão (ou contra uma parede próxima), sofrendo sua própria Força em dano contundente.",
    },
    CombatManeuver {
        id: "dragon_tail_sweep",
        name: "Dragon Tail Sweep",
        name_pt: "Rasteira Cauda de Dragão (Varredura)",
        category: ManeuverCategory::MartialArts,
        roll: "Dexterity + Martial Arts",
        roll_pt: "Destreza + Artes Marciais",
        damage: "Opponent’s Strength / B",
        damage_pt: "Força do oponente / C",
        difficulty: "8",
        actions: 1,
        requirement: "Martial Arts 1 (hard style), Athletics or Acrobatics 1",
        requirement_pt: "Artes Marciais 1 (estilo duro), Atletismo ou Acrobacia 1",
        description: "A spinning leg sweep that knocks an opponent sprawling. In game terms, the effects work like a throw, but the martial artist doesn’t need to grab her opponent first.",
        description_pt: "Uma rasteira circular giratória com a perna que derruba o oponente com estrondo. Em regras, os efeitos funcionam exatamente como uma projeção/arremesso, mas o lutador não precisa agarrar o oponente antes de executá-la.",
    },
    CombatManeuver {
        id: "elbow_knee_strike",
        name: "Elbow/Knee Strike",
        name_pt: "Golpe de Cotovelo ou Joelhada",
        category: ManeuverCategory::MartialArts,
        roll: "Dexterity + Martial Arts",
        roll_pt: "Destreza + Artes Marciais",
        damage: "Strength + 1 / B",
        damage_pt: "Força + 1 / C",
        difficulty: "5",
        actions: 1,
        requirement: "Martial Arts 1 (any style)",
        requirement_pt: "Artes Marciais 1 (qualquer estilo)",
        description: "A quick, brutal blow with an elbow or kneecap, usually directed at a vulnerable spot at close range.",
        description_pt: "Um impacto rápido e contundente desferido com a ponta do cotovelo ou rótula do joelho, mirando áreas vulneráveis do tronco ou rosto em combate corpo a corpo colado.",
    },
    CombatManeuver {
        id: "flying_kick",
        name: "Flying Kick",
        name_pt: "Chute Voador",
        category: ManeuverCategory::MartialArts,
        roll: "Dexterity + Martial Arts",
        roll_pt: "Destreza + Artes Marciais",
        damage: "Strength + 2 / B",
        damage_pt: "Força + 2 / C",
        difficulty: "7",
        actions: 1,
        requirement: "Martial Arts 1 (hard style), Athletics or Acrobatics 1",
        requirement_pt: "Artes Marciais 1 (estilo duro), Atletismo ou Acrobacia 1",
        description: "Leaping through the air, the martial artist combines her body weight and momentum into a powerful blow.",
        description_pt: "Saltando pelos ares, o artista marcial combina sua massa corporal e momento cinético em um impacto aéreo formidável.",
    },
    CombatManeuver {
        id: "hard_soft_strike",
        name: "Hard Strike/Soft Strike",
        name_pt: "Golpe Duro ou Golpe Suave",
        category: ManeuverCategory::MartialArts,
        roll: "Dexterity + Martial Arts",
        roll_pt: "Destreza + Artes Marciais",
        damage: "Strength + 1 / B",
        damage_pt: "Força + 1 / C",
        difficulty: "hard 5 / soft 6",
        actions: 1,
        requirement: "Martial Arts 1 (any style)",
        requirement_pt: "Artes Marciais 1 (qualquer estilo)",
        description: "Here’s a basic combat blow, focused with more skill than a wild punch. The hard variant involves a clenched fist or rigid fingers (the Tiger Claw technique), whereas a soft blow typically involves tripping the opponent or otherwise using his own force to hurt him.",
        description_pt: "O golpe básico refinado de combate marcial, desferido com disciplina incomparável a um soco comum. A variante dura utiliza punhos cerrados ou dedos rígidos (Garra de Tigre), enquanto o golpe suave utiliza alavancas, tropeços e o próprio ímpeto do inimigo para feri-lo.",
    },
    CombatManeuver {
        id: "joint_lock",
        name: "Joint Lock",
        name_pt: "Chave de Articulação",
        category: ManeuverCategory::MartialArts,
        roll: "Dexterity + Martial Arts",
        roll_pt: "Destreza + Artes Marciais",
        damage: "Successes / L",
        damage_pt: "Sucessos / L",
        difficulty: "5",
        actions: 1,
        requirement: "Martial Arts 2 (any style), Athletics or Acrobatics 2",
        requirement_pt: "Artes Marciais 2 (qualquer estilo), Atletismo ou Acrobacia 2",
        description: "Having already grappled his opponent, the martial artist applies pressure to joints and pressure points. If he chooses to exert force against that spot, he can dislocate or break limbs, joints, or possibly even the neck.\n\nThe player rolls Dexterity + Martial Arts to grapple his opponent, then may immediately (without dividing his dice pool between attacks) roll Dexterity + Martial Arts to inflict damage. Each success rolled inflicts one health level’s worth of lethal injury.",
        description_pt: "Após prender o oponente em um agarrão anterior, o artista marcial aplica torção e pressão extrema em articulações e juntas anatômicas. Se optar por exercer pressão destrutiva, pode deslocar ou fraturar braços, pernas, ombros ou até mesmo quebrar o pescoço.\n\nO jogador rola Destreza + Artes Marciais para travar o agarrão e, em seguida, pode imediatamente (sem dividir a parada de dados) rolar Destreza + Artes Marciais para dano. Cada sucesso obtido causa diretamente 1 nível de vitalidade em dano letal.",
    },
    CombatManeuver {
        id: "nerve_strike",
        name: "Nerve/Pressure Point Strike",
        name_pt: "Ponto de Pressão ou Golpe no Nervo",
        category: ManeuverCategory::MartialArts,
        roll: "Dexterity + Martial Arts",
        roll_pt: "Destreza + Artes Marciais",
        damage: "Strength + 3 / B (Stun)",
        damage_pt: "Força + 3 / C (Atordoamento)",
        difficulty: "7",
        actions: 1,
        requirement: "Martial Arts 3 (any style)",
        requirement_pt: "Artes Marciais 3 (qualquer estilo)",
        description: "Targeting a vital location, the martial artist directs precision force against it. If the attacker scores at least three successes to hit his opponent, then the opponent must make a Stamina roll (difficulty 8) or else be stunned on the next turn. A botch on that roll stuns her for three turns instead.",
        description_pt: "Mirando feixes nervosos ou centros vitais precisos, o lutador canaliza um choque focalizado. Se obtiver ao menos 3 sucessos no teste de ataque, a vítima deve fazer um teste de Vigor (dificuldade 8) ou ficará atordoada durante o turno seguinte. Uma falha crítica atordoa o oponente por 3 turnos inteiros.",
    },
    CombatManeuver {
        id: "snake_step",
        name: "Snake Step",
        name_pt: "Passo da Serpente",
        category: ManeuverCategory::MartialArts,
        roll: "Dexterity + Martial Arts",
        roll_pt: "Destreza + Artes Marciais",
        damage: "N/A (+3 dice to dodge)",
        damage_pt: "N/A (+3 dados de esquiva)",
        difficulty: "hard 6 / soft 5",
        actions: 1,
        requirement: "Martial Arts 1 (any style), Athletics or Acrobatics 1",
        requirement_pt: "Artes Marciais 1 (qualquer estilo), Atletismo ou Acrobacia 1",
        description: "Shifting away from the blow, our martial artist deftly sidesteps a close or ranged attack. Naturally, he must be able to see it coming first. A successful roll acts as a dodge attempt with an additional three-dice bonus.",
        description_pt: "Deslizando o corpo com fluidez serpenteante, o artista marcial esquiva graciosamente de golpes corpo a corpo ou disparos balísticos (desde que perceba o ataque a tempo). Um sucesso atua como manobra defensiva de esquiva conferindo um bônus de +3 dados adicionais à parada.",
    },
    CombatManeuver {
        id: "snap_kick",
        name: "Snap Kick",
        name_pt: "Chute Rápido (Chicote)",
        category: ManeuverCategory::MartialArts,
        roll: "Dexterity + Martial Arts",
        roll_pt: "Destreza + Artes Marciais",
        damage: "Strength + 1 / B",
        damage_pt: "Força + 1 / C",
        difficulty: "5",
        actions: 1,
        requirement: "Martial Arts 1 (hard style)",
        requirement_pt: "Artes Marciais 1 (estilo duro)",
        description: "Focusing her energy on a simple yet effective kick, the martial artist directs her lower body strength into the blow.",
        description_pt: "Concentrando energia em um golpe frontal veloz, o lutador estala a canela e ponta do pé como um chicote, desferindo um chute limpo com grande rapidez.",
    },
    CombatManeuver {
        id: "spinning_kick",
        name: "Spinning Kick",
        name_pt: "Chute Giratório",
        category: ManeuverCategory::MartialArts,
        roll: "Dexterity + Martial Arts",
        roll_pt: "Destreza + Artes Marciais",
        damage: "Strength + 3 / B",
        damage_pt: "Força + 3 / C",
        difficulty: "6",
        actions: 1,
        requirement: "Martial Arts 2 (hard style), Athletics or Acrobatics 2",
        requirement_pt: "Artes Marciais 2 (estilo duro), Atletismo ou Acrobacia 2",
        description: "Our martial artist spins around and plants a solid kick into the object of his attention.",
        description_pt: "O praticante gira velozmente sobre a base e crava o calcanhar com ímpeto rotacional no alvo, descarregando grande energia de impacto.",
    },
    CombatManeuver {
        id: "throw",
        name: "Throw",
        name_pt: "Arremesso (Projeção)",
        category: ManeuverCategory::MartialArts,
        roll: "Dexterity + Martial Arts",
        roll_pt: "Destreza + Artes Marciais",
        damage: "Strength + movement / B",
        damage_pt: "Força + movimento / C",
        difficulty: "hard 7 / soft 6",
        actions: 1,
        requirement: "Martial Arts 2 (any style)",
        requirement_pt: "Artes Marciais 2 (qualquer estilo)",
        description: "Having grappled her opponent (or used a counter throw), the martial artist slams him into a convenient surface. The attacker needs a Dexterity + Martial Arts dice pool equal to, or higher than, her opponent’s Dexterity. With a successful roll, the martial artist can throw her opponent up to one yard for each success she scores on her attack roll.\n\nWhen he lands, he takes damage based on her strength. If he’d been charging her at the time he was thrown, the impact also inflicts an additional +1 die for every 10 feet he’d been traveling at the time (+10 dice maximum).\n\nIf the impact inflicts more than three health levels in damage (after soaking), the opponent must make a Stamina roll (difficulty 8) or else be stunned for one turn. Should the martial artist throw one opponent into another one, the flying enemy inflicts one die bashing damage for each point of Stamina he has, plus one additional die for momentum (+2 difficulty to the throw roll).",
        description_pt: "Tendo agarrado o adversário (ou empregado um contra-arremesso), a lutadora projeta o corpo dele contra o chão ou paredes. A atacante precisa ter parada de Destreza + Artes Marciais igual ou superior à Destreza do alvo. Em caso de sucesso, projeta o oponente a até 1 metro por sucesso obtido.\n\nAo aterrissar, ele sofre dano baseado na Força da praticante. Se ele estiver investindo em corrida no momento, o choque causa +1 dado extra para cada 3 metros percorridos (máx +10 dados). Se sofrer mais de 3 níveis de dano após absorção, deve passar em teste de Vigor (dif 8) ou fica atordoado por 1 turno.\n\nArremessar um inimigo contra outro: causa 1 dado de dano contundente por ponto de Vigor do corpo lançado mais bônus de impulso (adiciona +2 à dificuldade do teste de arremesso).",
    },
    CombatManeuver {
        id: "thunder_kick",
        name: "Thunder Kick",
        name_pt: "Chute do Trovão",
        category: ManeuverCategory::MartialArts,
        roll: "Dexterity + Martial Arts",
        roll_pt: "Destreza + Artes Marciais",
        damage: "Strength + 3 + successes / B",
        damage_pt: "Força + 3 + sucessos / C",
        difficulty: "7",
        actions: 1,
        requirement: "Martial Arts 3 (hard style), Athletics or Acrobatics 2 (1/5 turns)",
        requirement_pt: "Artes Marciais 3 (estilo duro), Atletismo ou Acrobacia 2 (1x a cada 5 turnos)",
        description: "A devastating flying kick. Hurling himself through the air, the martial artist focuses his chi and mass into a strike potent enough to end most fights immediately. In game terms, each success adds one additional die to the kick’s damage. A character cannot use the Thunder Kick more than once every five turns, however, as it demands intense focus and commitment to the blow.",
        description_pt: "Um salto acrobático lendário de impacto ensurdecedor. Projetando-se no ar, o artista marcial canaliza seu chi e todo o peso corporal em uma colisão devastadora feita para encerrar o combate no ato. Cada sucesso obtido no ataque adiciona um dado extra ao dano contundente. Não pode ser executado mais de uma vez a cada 5 turnos devido à colossal exigência de foco e fôlego.",
    },
    CombatManeuver {
        id: "vital_strike",
        name: "Vital Strike",
        name_pt: "Golpe Vital",
        category: ManeuverCategory::MartialArts,
        roll: "Dexterity + Martial Arts",
        roll_pt: "Destreza + Artes Marciais",
        damage: "Strength / L",
        damage_pt: "Força / L",
        difficulty: "7",
        actions: 1,
        requirement: "Martial Arts 2 (hard style)",
        requirement_pt: "Artes Marciais 2 (estilo duro)",
        description: "A sharp-handed blow to an organ or joint inflicts lethal injury.",
        description_pt: "Um golpe de lâmina de mão ou dedos cerrados em ponta aplicado diretamente em tendões, traqueia ou órgãos vulneráveis, causando ferimentos de natureza letal.",
    },
    CombatManeuver {
        id: "withering_grasp",
        name: "Withering Grasp",
        name_pt: "Aperto Decomponedor (Imobilização Dilacerante)",
        category: ManeuverCategory::MartialArts,
        roll: "Dexterity + Martial Arts",
        roll_pt: "Destreza + Artes Marciais",
        damage: "Strength / B (Disarm)",
        damage_pt: "Força / C (Desarme)",
        difficulty: "7",
        actions: 1,
        requirement: "Martial Arts 3 (soft style)",
        requirement_pt: "Artes Marciais 3 (estilo suave)",
        description: "Grabbing his foe in a painful hold, the martial artist inflicts damage and may disarm her as well. To get the weapon from an enemy’s hand, the martial artist must score at least three successes; with five successes, he can take it away from her and use it himself in a following turn.",
        description_pt: "Apanhando a mão ou membro do adversário em uma chave de torção dolorosíssima, o lutador causa dano e desestabiliza a posse de armas. Com 3 sucessos, arranca e derruba a arma da mão do inimigo no chão; com 5 sucessos, desarma o oponente tomando a arma diretamente para si para usá-la no turno seguinte.",
    },

    // ------------------------------------------------------------------------
    // Regras Especiais de Combate (Special Rules)
    // ------------------------------------------------------------------------
    CombatManeuver {
        id: "two_weapons",
        name: "Two Weapons (Two-Fisted Gunslinger)",
        name_pt: "Duas Armas (Empunhadura Dupla)",
        category: ManeuverCategory::SpecialRules,
        roll: "Dexterity + Firearms or Melee (divided pool)",
        roll_pt: "Destreza + Armas de Fogo ou Armas Brancas (parada dividida)",
        damage: "As per each weapon",
        damage_pt: "Conforme cada arma individual",
        difficulty: "Normal (main hand) / +1 (off hand)",
        actions: 1,
        requirement: "Two one-handed weapons",
        requirement_pt: "Empunhar duas armas de uma mão",
        description: "If you wanna get your John Woo on, the two-fisted gunslinger stunt is an old favorite. Outside the movies, though, it’s far more difficult than it looks. A shooter who employs two guns at once needs to divide her Dexterity + Firearms dice pool between both weapons and also add a +1 difficulty penalty to the gun in her off hand. That said, a skillful combatant can lay down some serious damage with a pair of large pistols and the will to use them.\n\nThe same mechanics apply to dual-wielding melee weapons (dividing Dexterity + Melee with +1 difficulty to the secondary hand).",
        description_pt: "Para quem deseja canalizar o estilo John Woo, atirar com duas armas simultâneas é um clássico. Fora dos cinemas, contudo, é muito mais difícil do que parece. Um atirador que empunha duas pistolas ao mesmo tempo precisa dividir sua parada de dados de Destreza + Armas de Fogo entre os dois disparos e ainda somar uma penalidade de +1 na dificuldade para o tiro com a mão inábil.\n\nA mesma regra se aplica ao combate corpo a corpo com duas armas brancas (divide Destreza + Armas Brancas com +1 de dificuldade na mão secundária).",
    },

    // ------------------------------------------------------------------------
    // Técnicas Especiais de Dô (Akashic Brotherhood / M20 pp. 423-426, 448-450)
    // ------------------------------------------------------------------------
    CombatManeuver {
        id: "arrow_cutting",
        name: "Arrow Cutting",
        name_pt: "Aparar Flechas",
        category: ManeuverCategory::Do,
        roll: "Dexterity + Do",
        roll_pt: "Destreza + Dô",
        damage: "As Weapon",
        damage_pt: "Conforme Arma",
        difficulty: "7 (deflect) / 9 (catch & throw)",
        actions: 1,
        requirement: "Do 2+, Akashic Tradition",
        requirement_pt: "Dô 2+, Tradição Akashiana",
        description: "With blinding speed, the Tao-shih catches arrows and other projectile weapons (no bullets), knocks them out of the air, and sometimes even throws them back at her attackers. At difficulty 7, the devotee can deflect an incoming missile of arrow speed or slower; at difficulty 9, she can catch it and throw it back with her next action.",
        description_pt: "Com velocidade relampejante, o Tao-shih apara flechas e outras armas de projétil (não balas de fogo), rebatendo-as no ar e podendo inclusive arremessá-las de volta contra os atacantes. Na dificuldade 7, deflete um projétil com velocidade de flecha ou inferior; na dificuldade 9, agarra o projétil no ar e o lança de volta com sua próxima ação.",
    },
    CombatManeuver {
        id: "hurricane_throw",
        name: "Hurricane Throw",
        name_pt: "Arremesso Furacão",
        category: ManeuverCategory::Do,
        roll: "Dexterity + Do",
        roll_pt: "Destreza + Dô",
        damage: "Strength + 3 + successes / B",
        damage_pt: "Força + 3 + sucessos / C",
        difficulty: "8",
        actions: 1,
        requirement: "Do 2+, Akashic Tradition",
        requirement_pt: "Dô 2+, Tradição Akashiana",
        description: "The remarkable power of Do allows a practitioner to catch or grab his opponent and then throw her with incredible force. Essentially, this is a typical martial arts throw with the damage and difficulty listed below; each success, however, adds one die to the Hurricane Throw’s damage pool. This damage, unlike that of other unarmed attacks, is not lethal… though it can be quite significant.",
        description_pt: "O extraordinário poder do Dô permite ao praticante agarrar o oponente e projetá-lo com força colossal. Essencialmente, trata-se de um arremesso marcial clássico com dificuldade 8; cada sucesso obtido na rolagem adiciona um dado à parada de dano do arremesso. Ao contrário de outros ataques desarmados de Dô, este dano é contundente, e não letal — embora seja devastador.",
    },
    CombatManeuver {
        id: "iron_shirt",
        name: "Iron Shirt",
        name_pt: "Camisa de Ferro",
        category: ManeuverCategory::Do,
        roll: "Adds Do Trait to soak roll",
        roll_pt: "Soma nível de Dô à rolagem de absorção",
        damage: "N/A (Soak Bonus)",
        damage_pt: "N/A (Bônus de Absorção)",
        difficulty: "N/A",
        actions: 0,
        requirement: "Do 2+, Akashic Tradition",
        requirement_pt: "Dô 2+, Tradição Akashiana",
        description: "Through intense conditioning and focused chi, a practitioner can withstand terrible blows. In game terms, he gains one bonus soak die for every dot he has in Do. Although this Iron Shirt cannot soak lethal damage (unless you’re using the Cinematic Damage option, p. 412), it can grant fantastic stamina to a practitioner of Do.\n\nSadly, this technique does not confer total protection from harm. The character will always take one health level of damage from a blow that exceeds her Stamina Trait. If Lucy Hark gets hit for five levels of damage and she has a Stamina of 2, then she’ll still take one health level from that attack, despite her Iron Shirt technique.",
        description_pt: "Através de condicionamento físico extremo e chi canalizado, o praticante suporta impactos terríveis. O personagem recebe um dado adicional de absorção para cada ponto em Dô. Embora a Camisa de Ferro absorva apenas dano contundente (a menos que use a opção de Dano Cinematográfico, p. 412), concede uma resistência monumental ao Tao-shih.\n\nEssa técnica não confere imunidade total: o personagem sempre sofre ao menos 1 nível de dano se o ataque infligir dano superior ao seu Vigor (ex: sofrer 5 níveis de dano com Vigor 2 sempre causará ao menos 1 nível de vitalidade, independentemente da absorção).",
    },
    CombatManeuver {
        id: "kiaijutsu",
        name: "Kiaijutsu (Iron Shout)",
        name_pt: "Kiaijutsu (Grito de Ferro)",
        category: ManeuverCategory::Do,
        roll: "Stamina, Manipulation or Charisma + Do",
        roll_pt: "Vigor, Manipulação ou Carisma + Dô",
        damage: "Special (Stamina bonus / Terror / Eloquence)",
        damage_pt: "Especial (Bônus Vigor / Terror / Eloquência)",
        difficulty: "7 / Willpower + 3 / 8",
        actions: 1,
        requirement: "Do 2+, Akashic Tradition (1/combat)",
        requirement_pt: "Dô 2+, Tradição Akashiana (1x por combate)",
        description: "Also known as the Iron Shout, kiaijutsu channels the practitioner’s chi through his voice, granting him phenomenal powers of intimidation and force. Once per combat, the Akashic may focus his inner energy into a terrifying kiai, which may have one of the following effects:\n\n• Stamina: Toughens against blows, giving a three-turn Stamina bonus of +1 dot for every success on Stamina + Do (difficulty 7).\n\n• Terror: Frightens enemies by rolling Manipulation + Do (difficulty target’s Willpower + 3). Each success adds +1 difficulty to target's next action (max +3). If successes exceed target’s Willpower, enemy is stunned or flees. Against weak-willed opponents (Willpower 4 or less), one enemy flees per success.\n\n• Eloquence: Channeling toward a softer voice, rolls Charisma + Do (difficulty 8); each success adds +1 success to next Expression, Leadership, Intimidation, or performance Art roll.\n\nMust declare at beginning of action, or split pool as second action.",
        description_pt: "Também conhecido como o Grito de Ferro, o kiaijutsu canaliza o chi através da voz com poder estarrecedor. Uma vez por combate, o akashiano pode disparar um kiai interior com um dos seguintes efeitos:\n\n• Vigor: Endurece o corpo contra traumas, concedendo bônus de +1 no Vigor por sucesso em Vigor + Dô (dificuldade 7) com duração de 3 turnos.\n\n• Terror: Aterroriza inimigos rolando Manipulação + Dô (dificuldade: Vontade do alvo + 3). Cada sucesso adiciona +1 na dificuldade da próxima ação do oponente (máx +3). Se os sucessos superarem a Vontade do alvo, ele fica paralisado de pavor ou foge. Contra oponentes de mente fraca (Vontade 4 ou menos), 1 inimigo foge por sucesso.\n\n• Eloquência: Canalizando o grito para modulação persuasiva, rola Carisma + Dô (dificuldade 8); cada sucesso adiciona 1 sucesso automático no próximo teste de Expressão, Liderança, Intimidação ou Arte de performance.\n\nDeve ser declarado no início do turno ou usando divisão de parada para segunda ação.",
    },
    CombatManeuver {
        id: "plum_flower_blossom",
        name: "Plum Flower Blossom",
        name_pt: "Desabrochar da Flor de Ameixeira",
        category: ManeuverCategory::Do,
        roll: "Dexterity + Do",
        roll_pt: "Destreza + Dô",
        damage: "Special (+2 damage dice if attack)",
        damage_pt: "Especial (+2 dados de dano se for ataque)",
        difficulty: "6 (or 7 if attack)",
        actions: 1,
        requirement: "Do 2+, Akashic Tradition",
        requirement_pt: "Dô 2+, Tradição Akashiana",
        description: "Through intense balance training on stumps, pillars, and poles, an Akashic learns to perform incredible feats of acrobatic prowess. By rolling Dexterity + Do (difficulty 6), a practitioner may either double her jumping distance or bounce from object to object for one turn, plus another turn for every two successes scored.\n\nIf one of those leaps gets used as an attack, the difficulty rises by +1 but adds two extra damage dice to that attack. Leaping and attacking are still considered two different actions, so it demands the usual divided dice pool. Especially if it’s enhanced by the Correspondence and/or Forces Spheres, the Plum Flower technique can be quite amazing.",
        description_pt: "Através de treinamento rigoroso de equilíbrio sobre tocos, pilares e postes estreitos, o akashiano realiza acrobacias inacreditáveis. Rolando Destreza + Dô (dificuldade 6), dobra sua distância de salto ou salta de objeto em objeto no ar durante 1 turno, mais 1 turno extra para cada dois sucessos.\n\nSe um desses saltos for usado para desferir um ataque, a dificuldade sobe em +1, mas adiciona +2 dados de dano ao ataque. Saltar e atacar contam como duas ações distintas (exigindo parada dividida). Especialmente potencializado por Correspondência e/ou Forças, o Desabrochar da Ameixeira atinge níveis lendários.",
    },
    CombatManeuver {
        id: "soft_fist",
        name: "Soft Fist (Jou Chuan / Redirection)",
        name_pt: "Punho Suave (Jou Chuan / Redirecionamento)",
        category: ManeuverCategory::Do,
        roll: "Dexterity + Do",
        roll_pt: "Destreza + Dô",
        damage: "Attacker's Strength + weapon + extra successes",
        damage_pt: "Força do atacante + arma + sucessos excedentes",
        difficulty: "7",
        actions: 1,
        requirement: "Do 2+, Akashic Tradition",
        requirement_pt: "Dô 2+, Tradição Akashiana",
        description: "Also known as Jou Chuan or the Redirection technique, the Soft Fist guides a hand-to-hand attack back against those who wish to harm the devotee. If the Akashic using this technique scores more successes than his attacker, he catches the blow and sends it into a nearby character – probably the one who attacked him, or possibly a different enemy.\n\nDamage-wise, Jou Chuan inflicts the attacker’s Strength + weapon, plus one die for every success scored by the Akashic beyond what he needed to catch the attack. If, for instance, Raging Eagle needs three successes to catch his opponent’s attack, and he gets six successes, then the redirected attack inflicts the opponent’s Strength + weapon + three more dice.",
        description_pt: "Também conhecido como Jou Chuan ou Técnica de Redirecionamento, o Punho Suave guia o ataque corpo a corpo do inimigo contra ele mesmo ou outro oponente próximo. Se o akashiano obtiver mais sucessos que o agressor, intercepta o golpe e desvia a força cinética de volta para o agressor ou contra outro inimigo adjacente.\n\nEm dano, o Jou Chuan inflige a Força do atacante + dano da arma dele, mais um dado extra para cada sucesso excedente obtido pelo akashiano além do necessário para interceptar o ataque.",
    },
    CombatManeuver {
        id: "ten_thousand_weapons",
        name: "Ten Thousand Weapons",
        name_pt: "Dez Mil Armas",
        category: ManeuverCategory::Do,
        roll: "Dexterity + Do",
        roll_pt: "Destreza + Dô",
        damage: "Do dice (B) / Lethal / +1 die (L)",
        damage_pt: "Dano = Dô (C) / Letal / +1 dado (L)",
        difficulty: "As weapon / 6",
        actions: 1,
        requirement: "Do 2+, Akashic Tradition",
        requirement_pt: "Dô 2+, Tradição Akashiana",
        description: "In the hands of an especially skillful Warring Fist, anything becomes a weapon. With this technique, the Tao-shih can use any object – a coin, a wallet, a newspaper, whatever – to inflict damage. If the object isn’t normally a weapon, it deals out one die of bashing damage for every dot in the character’s Do Trait; if it’s a weapon that normally inflicts bashing damage, it does lethal damage; and if it normally does lethal damage, it inflicts one extra die of damage when this character employs it.",
        description_pt: "Nas mãos de um Punho Guerreiro experiente, qualquer objeto torna-se uma arma fatal. O Tao-shih pode usar uma moeda, carteira, jornal enrolado ou garrafa para ferir. Se o objeto não for uma arma convencional, causa 1 dado de dano contundente para cada ponto em Dô; se for uma arma que normalmente causa dano contundente, passa a causar dano letal; e se for uma arma que já causa dano letal, adiciona +1 dado extra de dano letal.",
    },
    CombatManeuver {
        id: "typhoon_kick",
        name: "Typhoon Kick",
        name_pt: "Chute Tufão",
        category: ManeuverCategory::Do,
        roll: "Dexterity + Do",
        roll_pt: "Destreza + Dô",
        damage: "Strength + 5 + successes / B or L",
        damage_pt: "Força + 5 + sucessos / C ou L",
        difficulty: "8",
        actions: 1,
        requirement: "Do 2+, Akashic Tradition (1/5 turns)",
        requirement_pt: "Dô 2+, Tradição Akashiana (1x a cada 5 turnos)",
        description: "Perhaps the most devastating strike in the Akashic technique arsenal, the Typhoon Kick directs chi, momentum, and supreme focus into a blow that can shatter stone and kill most human beings. Like the martial arts maneuver Thunder Kick, this attack requires absolute concentration and may be done no more than once every five turns. The practitioner can choose to use either bashing or lethal damage with this kick.\n\nIf she directs it against a solid object (a car, wall, stone elemental, etc.), or a character who manages to soak all the damage, then she must also make a successful soak roll or else take half of the damage, rounded down, herself. (A successful soak roll means no damage.)",
        description_pt: "Provavelmente o golpe mais devastador do arsenal marcial akashiano, o Chute Tufão concentra chi, momento angular e foco absoluto em um impacto capaz de partir pedras e aniquilar oponentes. Exige concentração máxima e só pode ser executado 1 vez a cada 5 turnos. O praticante escolhe causar dano contundente ou letal.\n\nSe desferido contra um obstáculo maciço (carro, parede de concreto, elemental de pedra) ou contra um alvo que absorva todo o dano, o próprio lutador deve rolar absorção ou sofrer metade do dano pretendido (arredondado para baixo).",
    },
    CombatManeuver {
        id: "weapon_art",
        name: "Weapon Art",
        name_pt: "Arte das Armas",
        category: ManeuverCategory::Do,
        roll: "Dexterity + Do or Melee",
        roll_pt: "Destreza + Dô ou Armas Brancas",
        damage: "As Weapon (Lethal option)",
        damage_pt: "Conforme Arma (Opção Letal)",
        difficulty: "Normal - 1",
        actions: 1,
        requirement: "Do 2+, Melee 1+, Dex + Do >= Dex + Melee (no guns)",
        requirement_pt: "Dô 2+, Armas Brancas 1+, Destreza + Dô >= Destreza + Armas Brancas (sem armas de fogo)",
        description: "Applying the mastery of Do into the use of hand-to-hand weaponry, a devotee can use certain weapons with brilliant efficiency. In game terms, the character gets one familiar weapon for every dot he has in the Melee Trait; with those weapons only, he reduces the normal difficulty by -1. If the weapon normally does bashing damage, the Akashic may inflict lethal damage with it instead. He may also use the Arrow Cutting technique with those weapons as well, though he cannot use a weapon to catch projectiles.\n\nIn order to employ this technique, the devotee must have a Dexterity + Do dice pool that’s at least as high as his Dexterity + Melee pool, if not higher. This technique does not apply to guns, though it may be used with bows or thrown weapons.",
        description_pt: "Aplicando a maestria do Dô no manejo de armas brancas e tradicionais, o devoto atinge uma eficácia transcendente. O personagem escolhe uma arma familiar para cada ponto em Armas Brancas; exclusivamente com essas armas, reduz a dificuldade em -1. Se a arma causar dano contundente, pode optar por causar dano letal. Permite também empregar Aparar Flechas com essas armas (embora não possa agarrar projéteis usando a lâmina).\n\nExige que a parada Destreza + Dô seja igual ou superior a Destreza + Armas Brancas. Não se aplica a armas de fogo, mas é válido para arcos e armas de arremesso.",
    },
];

pub fn find_maneuver(query: &str) -> Option<&'static CombatManeuver> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return None;
    }

    // 1. Id match
    if let Some(m) = ALL_COMBAT_MANEUVERS.iter().find(|m| m.id.eq_ignore_ascii_case(&q)) {
        return Some(m);
    }

    // 2. Exact match name or name_pt
    if let Some(m) = ALL_COMBAT_MANEUVERS.iter().find(|m| {
        m.name.to_lowercase() == q || m.name_pt.to_lowercase() == q
    }) {
        return Some(m);
    }

    // 3. Substring match
    if let Some(m) = ALL_COMBAT_MANEUVERS.iter().find(|m| {
        m.name.to_lowercase().contains(&q) || m.name_pt.to_lowercase().contains(&q)
    }) {
        return Some(m);
    }

    // 4. Common aliases & variants
    match q.as_str() {
        "rasteira cauda de dragão" | "rasteira de dragão" | "rasteira do dragão" | "dragon sweep" => {
            ALL_COMBAT_MANEUVERS.iter().find(|m| m.id == "dragon_tail_sweep")
        }
        "chute trovão" | "chute do trovao" | "chute trovao" => {
            ALL_COMBAT_MANEUVERS.iter().find(|m| m.id == "thunder_kick")
        }
        "thunder punch" | "soco trovão" | "soco trovao" => {
            ALL_COMBAT_MANEUVERS.iter().find(|m| m.id == "punch")
        }
        "strike vital point" | "golpe em ponto vital" | "ponto vital" => {
            ALL_COMBAT_MANEUVERS.iter().find(|m| m.id == "vital_strike")
        }
        "roundhouse" | "roundhouse kick" | "chute circular" => {
            ALL_COMBAT_MANEUVERS.iter().find(|m| m.id == "spinning_kick")
        }
        "nerve strike" | "pressure point strike" | "pressure point" | "golpe no nervo" => {
            ALL_COMBAT_MANEUVERS.iter().find(|m| m.id == "nerve_strike")
        }
        "elbow strike" | "knee strike" | "golpe de cotovelo" | "joelhada" => {
            ALL_COMBAT_MANEUVERS.iter().find(|m| m.id == "elbow_knee_strike")
        }
        "hard strike" | "soft strike" | "golpe duro" | "golpe suave" => {
            ALL_COMBAT_MANEUVERS.iter().find(|m| m.id == "hard_soft_strike")
        }
        _ => None,
    }
}

pub fn get_maneuvers_by_category(cat: Option<ManeuverCategory>) -> Vec<&'static CombatManeuver> {
    match cat {
        Some(c) => ALL_COMBAT_MANEUVERS.iter().filter(|m| m.category == c).collect(),
        None => ALL_COMBAT_MANEUVERS.iter().collect(),
    }
}

// ============================================================================
// REGRA EXPANDIDA / BOX DO LIVRO M20: TRUQUE DE MAGO (MAGE TRICK)
// (M20 Core Rulebook, p. 449, Capítulo 9: Combat & Storytelling)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MageTrickArticle {
    pub id: &'static str,
    pub title: &'static str,
    pub title_pt: &'static str,
    pub page_ref: &'static str,
    pub spheres_summary: &'static str,
    pub spheres_summary_pt: &'static str,
    pub difficulty_rule: &'static str,
    pub difficulty_rule_pt: &'static str,
    pub damage_rule: &'static str,
    pub damage_rule_pt: &'static str,
    pub backlash_rule: &'static str,
    pub backlash_rule_pt: &'static str,
    pub paragraphs: &'static [&'static str],
    pub paragraphs_pt: &'static [&'static str],
    pub sphere_tags: &'static [&'static str],
    pub sphere_tags_pt: &'static [&'static str],
}

impl MageTrickArticle {
    pub fn title(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.title_pt,
            Language::EnUs => self.title,
        }
    }

    pub fn spheres_summary(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.spheres_summary_pt,
            Language::EnUs => self.spheres_summary,
        }
    }

    pub fn difficulty_rule(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.difficulty_rule_pt,
            Language::EnUs => self.difficulty_rule,
        }
    }

    pub fn damage_rule(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.damage_rule_pt,
            Language::EnUs => self.damage_rule,
        }
    }

    pub fn backlash_rule(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.backlash_rule_pt,
            Language::EnUs => self.backlash_rule,
        }
    }

    pub fn paragraphs(&self, lang: Language) -> &'static [&'static str] {
        match lang {
            Language::PtBr => self.paragraphs_pt,
            Language::EnUs => self.paragraphs,
        }
    }

    pub fn sphere_tags(&self, lang: Language) -> &'static [&'static str] {
        match lang {
            Language::PtBr => self.sphere_tags_pt,
            Language::EnUs => self.sphere_tags,
        }
    }
}

/// Box Oficial do Livro M20 (p. 449): "Mage Trick: The Thunder Punch"
pub const THUNDER_PUNCH_TRICK: MageTrickArticle = MageTrickArticle {
    id: "thunder_punch",
    title: "Mage Trick: The Thunder Punch",
    title_pt: "Truque de Mago: O Golpe Trovão",
    page_ref: "M20, p. 449",
    spheres_summary: "Correspondence 1, Entropy 1-2, Forces 2, Life 3, Mind 2, Matter 2, Prime 2-3, or Time 2",
    spheres_summary_pt: "Correspondência 1, Entropia 1-2, Forças 2, Vida 3, Mente 2, Matéria 2, Primórdio 2-3 ou Tempo 2",
    difficulty_rule: "Each success reduces attack difficulty by -1 (maximum adjustment: -3)",
    difficulty_rule_pt: "Cada sucesso reduz a dificuldade do ataque em -1 (ajuste máximo: -3)",
    damage_rule: "Coincidental: Bashing damage. Life 3 or Prime 3: Aggravated damage (Pattern assault). Vulgar if visibly disproportionate to physique.",
    damage_rule_pt: "Coincidente: Dano Contundente. Vida 3 ou Primórdio 3: Dano Agravado (ataque ao Padrão). Vulgar se visivelmente desproporcional à compleição.",
    backlash_rule: "If target soaks ALL damage, attacker takes the full intended damage (Bashing; Lethal if striking walls/steel armor).",
    backlash_rule_pt: "Se o alvo absorver TODO o dano, o próprio mago sofre o dano pretendido (Contundente; Letal contra paredes/armadura de aço).",
    paragraphs: &[
        "Throwing Enlightened force behind a punch, kick, or other hand-to-hand attack, a fighting mage delivers an unexpectedly powerful blow. Story-wise, the trick looks like a perfectly executed strike. In game terms, the mage employs either Correspondence 1 (to judge the perfect spot to hit), Entropy 1 or 2 (to either find a weak spot or adjust probability to the perfect place and time), Forces 2 (to increase velocity), Life 3 (to boost Strength or to damage the enemy internally), Mind 2 (to send an impulse to surrender), Matter 2 (to break inert materials), Prime 2 (to directly attack the target’s Pattern), or Time 2 (to note the perfect opening). As with most other tricks, each success reduces the difficulty of the attack roll by -1 per success, to a maximum adjustment of -3.",
        "In most cases, this “thunder punch” – which can also be performed with hand-to-hand weapons – is coincidental and inflicts bashing damage. A Life 3 or Prime 3 Pattern assault, however, inflicts aggravated damage instead, using the blow to focus an all-out metaphysical attack. Truly powerful blows may slip into vulgar magick if the mage appears to be too weak to have inflicted such a powerful strike… like, say, the proverbial skinny geek smashing every bone in the body of a towering muscle-thug. Oh, yeah – the thunder punch can kill people if you’re not careful, so it’s best to save this trick for Night-Folk and other such opponents unless your mage wants to face charges for manslaughter or murder.",
        "Another flipside to the blow is obvious: you can hurt yourself while doing it. If the target manages to soak every level of damage inflicted by a thunder punch, then the attacker hurts himself; the damage he would have inflicted on the target gets inflicted on the mage instead (yes, he can try to soak it too). In the case of the Pattern attacks, the mage simply suffers bashing damage, not aggravated harm. If, however, the mage tries to thunder punch a wall, steel armor, solid stone, and so forth, he might take lethal damage instead, possibly shattering his hand or foot… or, if he’s using a weapon to strike, breaking his weapon against that unyielding surface.",
    ],
    paragraphs_pt: &[
        "Colocando força Iluminada por trás de um soco, chute ou outro ataque corpo a corpo, um mago lutador desfere um golpe inesperadamente poderoso. Narrativamente, o truque parece um golpe perfeitamente executado. Em termos de jogo, o mago emprega Correspondência 1 (para julgar o ponto de impacto perfeito), Entropia 1 ou 2 (para encontrar um ponto fraco ou ajustar a probabilidade ao momento e local ideais), Forças 2 (para aumentar a velocidade), Vida 3 (para aumentar a Força ou lesionar internamente o inimigo), Mente 2 (para enviar um impulso de rendição), Matéria 2 (para quebrar materiais inertes), Primórdio 2 (para atacar diretamente o Padrão do alvo) ou Tempo 2 (para notar a brecha perfeita). Como na maioria dos outros truques, cada sucesso reduz a dificuldade da rolagem de ataque em -1 por sucesso, até um ajuste máximo de -3.",
        "Na maioria dos casos, este \"golpe trovão\" — que também pode ser executado com armas brancas — é coincidente e causa dano contundente. Um ataque ao Padrão com Vida 3 ou Primórdio 3, contudo, causa dano agravado, usando o impacto físico para focar um ataque metafísico devastador. Golpes verdadeiramente poderosos podem deslizar para mágica vulgar se o mago parecer fraco demais para ter causado tal impacto... como o lendário geek franzino quebrando todos os ossos do corpo de um brutamontes imenso. E sim — o golpe trovão pode matar se você não tiver cuidado, portanto é melhor guardar este truque para o Povo da Noite e oponentes semelhantes, a menos que seu mago queira responder por homicídio culposo ou assassinato.",
        "O outro lado da moeda é evidente: você pode se machucar ao usá-lo. Se o alvo conseguir absorver cada nível de dano infligido pelo golpe trovão, o próprio atacante se fere; o dano que ele teria infligido ao alvo é aplicado no próprio mago (sim, ele também pode tentar absorver). No caso dos ataques ao Padrão, o mago sofre apenas dano contundente, e não dano agravado. Se, no entanto, o mago tentar desferir um golpe trovão contra uma parede, armadura de aço, pedra maciça e afins, ele pode sofrer dano letal, possivelmente esfacelando sua mão ou pé... ou, se estiver empunhando uma arma para golpear, quebrando-a contra a superfície inquebrável.",
    ],
    sphere_tags: &[
        "Correspondence 1",
        "Entropy 1-2",
        "Forces 2",
        "Life 3",
        "Mind 2",
        "Matter 2",
        "Prime 2-3",
        "Time 2",
    ],
    sphere_tags_pt: &[
        "Correspondência 1",
        "Entropia 1-2",
        "Forças 2",
        "Vida 3",
        "Mente 2",
        "Matéria 2",
        "Primórdio 2-3",
        "Tempo 2",
    ],
};

// ============================================================================
// REGRA EXPANDIDA / BOX DO LIVRO M20: OS OITO MEMBROS DA MAESTRIA (EIGHT LIMBS)
// (Akashic Dharma Sutra • M20 Livro Básico Capítulo 6 & Livro dos Segredos)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimbDefinition {
    pub name: &'static str,
    pub title: &'static str,
    pub title_pt: &'static str,
    pub description: &'static str,
    pub description_pt: &'static str,
    pub abilities: &'static [&'static str],
    pub abilities_pt: &'static [&'static str],
}

impl LimbDefinition {
    pub fn title(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.title_pt,
            Language::EnUs => self.title,
        }
    }

    pub fn description(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.description_pt,
            Language::EnUs => self.description,
        }
    }

    pub fn abilities(&self, lang: Language) -> &'static [&'static str] {
        match lang {
            Language::PtBr => self.abilities_pt,
            Language::EnUs => self.abilities,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EightLimbsArticle {
    pub id: &'static str,
    pub title: &'static str,
    pub title_pt: &'static str,
    pub subtitle: &'static str,
    pub subtitle_pt: &'static str,
    pub page_ref: &'static str,
    pub intro: &'static str,
    pub intro_pt: &'static str,
    pub limbs: &'static [LimbDefinition],
    pub progression_rule: &'static str,
    pub progression_rule_pt: &'static str,
    pub peaceful_way_title: &'static str,
    pub peaceful_way_title_pt: &'static str,
    pub peaceful_way_rule: &'static str,
    pub peaceful_way_rule_pt: &'static str,
}

impl EightLimbsArticle {
    pub fn title(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.title_pt,
            Language::EnUs => self.title,
        }
    }

    pub fn subtitle(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.subtitle_pt,
            Language::EnUs => self.subtitle,
        }
    }

    pub fn intro(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.intro_pt,
            Language::EnUs => self.intro,
        }
    }

    pub fn progression_rule(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.progression_rule_pt,
            Language::EnUs => self.progression_rule,
        }
    }

    pub fn peaceful_way_title(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.peaceful_way_title_pt,
            Language::EnUs => self.peaceful_way_title,
        }
    }

    pub fn peaceful_way_rule(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.peaceful_way_rule_pt,
            Language::EnUs => self.peaceful_way_rule,
        }
    }
}

pub const EIGHT_LIMBS_ARTICLE: EightLimbsArticle = EightLimbsArticle {
    id: "eight_limbs",
    title: "Eight Limbs of Expertise",
    title_pt: "Os Oito Membros da Maestria",
    subtitle: "Akashic Dharma Sutra • Chapter Six & Book of Secrets",
    subtitle_pt: "Dharma Sutra Akashiano • Capítulo Seis & Livro dos Segredos",
    page_ref: "M20 Ch. 6, p. 580",
    intro: "According to the Akashic Dharma Sutra, the Way is divided into eight fields of expertise. And although Do is often regarded as a martial art, its devotees recognize that a truly harmonious human being understands at least a little bit about all of them.",
    intro_pt: "De acordo com o Dharma Sutra Akashiano, o Caminho é dividido em oito campos de maestria. E embora o Dô seja frequentemente considerado uma arte marcial, seus devotos reconhecem que um ser humano verdadeiramente harmonioso compreende ao menos um pouco sobre todos eles.",
    limbs: &[
        LimbDefinition {
            name: "Dharmamukti",
            title: "Dharmamukti (The Dharma Clasped Hand)",
            title_pt: "Dharmamukti (A Mão Unida do Dharma)",
            description: "Unarmed combat techniques of the Dharma Clasped Hand. Both adapting and inspiring other martial arts techniques, this limb comprises the most obvious form of Do, but only one of its many applications.",
            description_pt: "Técnicas de combate desarmado da Mão Unida do Dharma. Tanto adaptando quanto inspirando outras técnicas marciais, este membro compreende a forma mais evidente do Dô, mas é apenas uma de suas inúmeras aplicações.",
            abilities: &["Alertness", "Athletics", "Do"],
            abilities_pt: &["Prontidão", "Atletismo", "Dô"],
        },
        LimbDefinition {
            name: "Dhyana",
            title: "Dhyana (The Limb of Meditation)",
            title_pt: "Dhyana (O Membro da Meditação)",
            description: "The Limb of Meditation, focused upon calming an excitable mind. Refined as a way to escape the clutches of egotism and attachment, Dhyana employs mandalas, the Five Asian Elements, meditative movement practices, and the greater realm beyond Self.",
            description_pt: "O Membro da Meditação, voltado para aquietar uma mente agitada. Refinado como uma senda para escapar das amarras do egoísmo e do apego, Dhyana emprega mandalas, os Cinco Elementos Orientais, práticas de movimento meditativo e a transcendência além do Eu.",
            abilities: &["Awareness", "Enigmas", "Meditation"],
            abilities_pt: &["Consciência", "Enigmas", "Meditação"],
        },
        LimbDefinition {
            name: "Jivahasta",
            title: "Jivahasta (The Hand of Life)",
            title_pt: "Jivahasta (A Mão da Vida)",
            description: "The Hand of Life, a study of healing arts and bodywork.",
            description_pt: "A Mão da Vida, um estudo aprofundado das artes de cura, terapias corporais e equilíbrio biológico.",
            abilities: &["Esoterica (Bodywork, Herbalism, Yoga)", "Medicine (Alternate Healing)", "Survival"],
            abilities_pt: &["Esotérica (Trabalho Corporal, Herbalismo, Ioga)", "Medicina (Técnicas Alternativas de Cura)", "Sobrevivência"],
        },
        LimbDefinition {
            name: "Karma",
            title: "Karma (Devotion to Humble Labor)",
            title_pt: "Karma (Devoção ao Labor Humilde)",
            description: "Devotion to humble labor, the perfect focus of small things, and one’s place within an infinite cosmos, epitomized by the saying, “Before enlightenment, chopping wood and carrying water. After enlightenment, chopping wood and carrying water.”",
            description_pt: "Devoção ao trabalho humilde, foco perfeito nas pequenas coisas e consciência do próprio lugar dentro de um cosmos infinito, sintetizado no provérbio: \"Antes da iluminação, cortar lenha e carregar água. Depois da iluminação, cortar lenha e carregar água\".",
            abilities: &["Art (all kinds)", "Crafts (non-martial)", "Etiquette"],
            abilities_pt: &["Arte (todas as formas)", "Ofícios (especialidades não-marciais)", "Etiqueta"],
        },
        LimbDefinition {
            name: "Prajna",
            title: "Prajna (Study of Ethics and Philosophy)",
            title_pt: "Prajna (Estudo da Ética e Filosofia)",
            description: "The study of ethics and philosophy. Just as a devotee of the Way refines his body for war, he must also refine his mind with kindness, compassion, and the stillness of perfect harmony.",
            description_pt: "O estudo da ética e da filosofia. Assim como um devoto do Caminho refina seu corpo para o combate, ele também deve refinar sua mente com bondade, compaixão e a serenidade da harmonia perfeita.",
            abilities: &["Academics (Philosophy)", "Belief Systems", "Cosmology"],
            abilities_pt: &["Instrução (Filosofia)", "Sistemas de Crença", "Cosmologia"],
        },
        LimbDefinition {
            name: "Shastamarga",
            title: "Shastamarga (The Way of Weapons)",
            title_pt: "Shastamarga (O Caminho das Armas)",
            description: "The Way of Weapons, intended to expand bodily awareness into the tools that extend the body’s reach. By focusing on weapons, the craftsmanship involved, and the awful consequences of their use and abuse, a devotee learns to judge the implications of her own actions upon the greater whole.",
            description_pt: "O Caminho das Armas, destinado a expandir a consciência corporal através dos instrumentos que prolongam o alcance físico. Ao focar nas armas, no artesanato e nas terríveis consequências de seu uso e abuso, o devoto aprende a ponderar as implicações de seus atos sobre a totalidade.",
            abilities: &["Academics (Strategy)", "Crafts (Weaponsmithing)", "Melee"],
            abilities_pt: &["Instrução (Estratégia)", "Ofícios (Armaria / Forja)", "Armas Brancas"],
        },
        LimbDefinition {
            name: "Sunyakaya",
            title: "Sunyakaya (The Limb of the Empty Body)",
            title_pt: "Sunyakaya (O Membro do Corpo Vazio)",
            description: "The Limb of the Empty Body, cultivating the ability to disappear. Based as it is upon the removal of ego through the concealment of identity, this limb has more practical uses as well.",
            description_pt: "O Membro do Corpo Vazio, cultivando a capacidade de desaparecer. Fundamentado na dissolução do ego através da ocultação da identidade, este membro possui também aplicações práticas e de furtividade suprema.",
            abilities: &["Meditation", "Stealth", "Subterfuge"],
            abilities_pt: &["Meditação", "Furtividade", "Subterfúgio"],
        },
        LimbDefinition {
            name: "Tricanmarga",
            title: "Tricanmarga (The Way of the Triple Struggle)",
            title_pt: "Tricanmarga (O Caminho da Tripla Luta)",
            description: "The Way of the Triple Struggle, balancing out the forces of Dynamism, Stasis, and Entropy (seen as Tiger, Phoenix, and Dragon) within one’s own self and then – by extension – reaching greater harmony with the world beyond that self.",
            description_pt: "O Caminho da Tripla Luta, equilibrando as forças do Dinamismo, Estase e Entropia (representadas como o Tigre, a Fênix e o Dragão) dentro de si mesmo e, por extensão, alcançando maior harmonia com o mundo além do Eu.",
            abilities: &["Acrobatics", "Athletics", "Esoterica (Body Control, Yoga)"],
            abilities_pt: &["Acrobacia", "Atletismo", "Esotérica (Controle Corporal, Ioga)"],
        },
    ],
    progression_rule: "In game terms, you must have at least two dots in limb-related Abilities other than Dharmamukti for every dot in Do. Before you can add another dot of Do, you must first add two other dots in limb-related fields. To go, for example, from Do 2 to Do 3, your character would also have to learn at least two dots in one of the Abilities on the sidebar. Ideally, he’ll eventually learn at least one dot in all eight limbs.",
    progression_rule_pt: "Em termos de sistema, você deve possuir ao menos dois pontos em Habilidades ligadas aos membros (exceto Dharmamukti) para cada ponto em Dô. Antes de adquirir um novo ponto em Dô, o mago precisa primeiro adquirir dois outros pontos nos campos dos membros. Para evoluir, por exemplo, de Dô 2 para Dô 3, o personagem precisa desenvolver ao menos dois pontos em uma das Habilidades listadas. O ideal é eventualmente possuir ao menos um ponto em todos os oito membros.",
    peaceful_way_title: "Optional Rule: The Peaceful Way",
    peaceful_way_title_pt: "Regra Opcional: O Caminho Pacífico",
    peaceful_way_rule: "Do is not merely a martial art. By applying the Way to peaceful tasks, a Tao-shih may focus its principles to great success. In story terms, the Akashic character meditates upon the Way as he performs whatever task he has set his mind to doing.\n\nIn game terms, the player spends one Willpower point and then rolls his Do Trait as a dice pool against difficulty 8. Each success he rolls adds one success to a mundane task associated with one of the eight limbs listed above. To perform the task itself, the player rolls the associated Attribute + Ability, at whatever difficulty seems to fit the task at hand.\n\nOnly the Associated Abilities listed above may be enhanced by this optional rule, with the exceptions of Do combat maneuvers and the Abilities associated with the Shastamarga limb. Such violent tasks do not get the benefit of this rule; this is, after all, the Peaceful Way, and so martial applications are not appropriate to its purity.",
    peaceful_way_rule_pt: "O Dô não é apenas uma arte marcial. Aplicando o Caminho a tarefas pacíficas, um Tao-shih pode canalizar seus princípios com enorme sucesso. Narrativamente, o personagem akashiano medita sobre o Caminho enquanto realiza a tarefa a que se propôs.\n\nEm termos de jogo, o jogador gasta um ponto de Força de Vontade e rola sua parada de Dô contra dificuldade 8. Cada sucesso obtido adiciona um sucesso automático a uma tarefa mundana associada a um dos oito membros listados acima. Para realizar a tarefa em si, o jogador rola o Atributo + Habilidade correspondente na dificuldade apropriada à situação.\n\nApenas as Habilidades Associadas listadas acima podem ser beneficiadas por esta regra opcional, com a exceção expressa de manobras de combate de Dô e das Habilidades associadas ao membro Shastamarga (Armas). Tarefas violentas não recebem o benefício desta regra; trata-se, afinal, do Caminho Pacífico, e aplicações de combate não são condizentes com sua pureza.",
};

// ============================================================================
// REGRAS & FILOSOFIA DE TREINAMENTO DO DÔ (M20)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoAdvantage {
    pub code: &'static str,
    pub title: &'static str,
    pub title_pt: &'static str,
    pub icon: &'static str,
    pub rule: &'static str,
    pub rule_pt: &'static str,
}

impl DoAdvantage {
    pub fn title(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.title_pt,
            Language::EnUs => self.title,
        }
    }

    pub fn rule(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.rule_pt,
            Language::EnUs => self.rule,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoRulesArticle {
    pub id: &'static str,
    pub title: &'static str,
    pub title_pt: &'static str,
    pub subtitle: &'static str,
    pub subtitle_pt: &'static str,
    pub page_ref: &'static str,
    pub overview: &'static str,
    pub overview_pt: &'static str,
    pub commitment: &'static str,
    pub commitment_pt: &'static str,
    pub advantages: &'static [DoAdvantage],
}

impl DoRulesArticle {
    pub fn title(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.title_pt,
            Language::EnUs => self.title,
        }
    }

    pub fn subtitle(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.subtitle_pt,
            Language::EnUs => self.subtitle,
        }
    }

    pub fn overview(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.overview_pt,
            Language::EnUs => self.overview,
        }
    }

    pub fn commitment(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.commitment_pt,
            Language::EnUs => self.commitment,
        }
    }
}

pub const DO_RULES_ARTICLE: DoRulesArticle = DoRulesArticle {
    id: "do_rules",
    title: "Do: The Way of Life & Fighting Prowess",
    title_pt: "Dô: O Caminho da Vida & Destreza Marcial",
    subtitle: "Akashic Brotherhood Canonical Combat Rules • M20",
    subtitle_pt: "Regras Canônicas de Combate da Irmandade de Akashayana • M20",
    page_ref: "M20 Ch. 6, pp. 580-581",
    overview: "The quintessential martial art from which all other disciplines supposedly descend, Do is far more than a mere collection of fighting techniques. As shown by its Trait listing in Chapter Six, Do is a way of life… or, more literally, the Way of life. When directed toward peaceful ends, Do allows a person to refine enormous potential. And when directed, by sad necessity, toward the ends of violence, Do is legendary in its martial applications.",
    overview_pt: "A arte marcial quintessencial da qual todas as outras disciplinas supostamente descendem, o Dô é muito mais do que uma simples coleção de técnicas de combate. O Dô é um modo de vida... ou, mais literalmente, o Caminho da vida. Quando direcionado para fins pacíficos, o Dô permite a uma pessoa refinar um potencial colossal. E quando direcionado, por triste necessidade, para a violência, o Dô é lendário em suas aplicações marciais.",
    commitment: "A starting character cannot begin play with more than two dots in Do. Further expertise must be earned over the course of the game. Only Akashics and their closest, most trusted allies may learn it, and that discipline is a lifetime commitment. A Tao-shih (Way-scholar/warrior) must spend at least one hour every day practicing Do; otherwise, his skills diminish. In game terms, he loses dots temporarily until regular training resumes – roughly one dot for every two weeks without practice.",
    commitment_pt: "Um personagem iniciante não pode começar o jogo com mais de dois pontos em Dô. Maestria adicional deve ser conquistada ao longo das crônicas. Apenas akashianos e seus aliados mais íntimos e leais podem aprendê-lo, e essa disciplina é um compromisso para toda a vida. Um Tao-shih (guerreiro/sábio do Caminho) precisa dedicar ao menos uma hora todos os dias à prática de Dô; do contrário, suas habilidades diminuem. Em termos de jogo, ele perde pontos temporariamente até que retome o treino regular — aproximadamente um ponto para cada duas semanas sem prática.",
    advantages: &[
        DoAdvantage {
            code: "secret_teachings",
            title: "Secret Teachings",
            title_pt: "Ensinamentos Secretos",
            icon: "📜",
            rule: "Only the most skilled Akashayana (Do 3+) may teach this Art. A teacher passes on one fighting maneuver for each dot of Do she possesses, plus one more for each dot in Intelligence. A student cannot learn Do without studying the eight limbs of Do.",
            rule_pt: "Apenas akashianos mestres (Dô 3+) podem ensinar esta Arte. Um instrutor transmite uma manobra de combate para cada ponto de Dô que possui, mais uma por ponto em Inteligência. O aprendiz não pode avançar em Dô sem estudar os oito membros.",
        },
        DoAdvantage {
            code: "flexibility",
            title: "Flexibility",
            title_pt: "Flexibilidade",
            icon: "🥋",
            rule: "A Do-using character may employ any general or martial arts combat maneuver (1 dot in Do satisfies 1 dot required in Martial Arts). Dirty fighting techniques cannot be used – they are brutal, undisciplined, and inharmonious.",
            rule_pt: "Um praticante de Dô pode utilizar qualquer manobra de combate geral ou marcial (1 ponto em Dô substitui 1 ponto exigido em Artes Marciais). Não pode empregar manobras de Luta Suja — brutais, indisciplinadas e desarmônicas.",
        },
        DoAdvantage {
            code: "precision",
            title: "Precision (Well-Trained Advantage)",
            title_pt: "Precisão (Vantagem de Bem Treinado)",
            icon: "🎯",
            rule: "When employing general combat maneuvers, subtracts -1 from the usual difficulty; a punch, for instance, is difficulty 5, not 6. Maneuvers from Do or Martial Arts use normal difficulty.",
            rule_pt: "Ao empregar manobras gerais de combate corporal, subtrai -1 na dificuldade habitual; um soco, por exemplo, tem dificuldade 5, e não 6. Manobras próprias de Dô ou Artes Marciais usam as dificuldades normais.",
        },
        DoAdvantage {
            code: "martial_mastery",
            title: "Martial Mastery",
            title_pt: "Maestria Marcial",
            icon: "⚡",
            rule: "Select two martial arts maneuvers for every dot in Do, plus one special Do technique for every dot above the first. A character with both Do and Martial Arts adds all maneuvers together.",
            rule_pt: "O jogador escolhe duas manobras de artes marciais para cada ponto em Dô, mais uma técnica especial de Dô para cada ponto acima do primeiro. Tendo Dô e Artes Marciais, soma todas as manobras.",
        },
        DoAdvantage {
            code: "differences_in_mastery",
            title: "Differences in Mastery",
            title_pt: "Diferença nas Paradas de Dados",
            icon: "⚖️",
            rule: "Uses Dexterity + Martial Arts for maneuvers acquired through Martial Arts, and Dexterity + Do for those learned through Do. Once Do exceeds Martial Arts, use Do for all combat maneuver dice pools.",
            rule_pt: "Usa Destreza + Artes Marciais para as manobras aprendidas via Artes Marciais, e Destreza + Dô para as aprendidas através de Dô. Quando o valor de Dô superar Artes Marciais, usa Dô para todas as rolagens!",
        },
        DoAdvantage {
            code: "lethal_damage",
            title: "Lethal Damage",
            title_pt: "Dano Letal Desarmado",
            icon: "⚔️",
            rule: "Thanks to intense focus and precision, unarmed attacks inflict lethal damage. The Akashic may choose to use bashing damage reflexively at any time before rolling to hit.",
            rule_pt: "Graças ao foco e precisão supremos, ataques desarmados causam dano letal. O akashiano pode optar reflexivamente por causar dano contundente a qualquer instante antes da rolagem de ataque.",
        },
        DoAdvantage {
            code: "hardened_defense",
            title: "Hardened Defense",
            title_pt: "Defesa Endurecida",
            icon: "🛡️",
            rule: "An unarmed Do practitioner can block Brawl or Melee attacks that cause lethal damage using bare palms and limbs. (Cannot stop projectiles without special techniques like Arrow Cutting).",
            rule_pt: "Um praticante desarmado de Dô pode bloquear e aparar ataques de Briga ou Armas Brancas que causem dano letal usando as mãos nuas e antebraços. (Não detém projéteis sem técnicas como Aparar Flechas).",
        },
    ],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thunder_punch_trick_integrity() {
        assert_eq!(THUNDER_PUNCH_TRICK.id, "thunder_punch");
        assert_eq!(THUNDER_PUNCH_TRICK.page_ref, "M20, p. 449");
        assert_eq!(THUNDER_PUNCH_TRICK.paragraphs.len(), 3);
        assert_eq!(THUNDER_PUNCH_TRICK.paragraphs_pt.len(), 3);
        assert_eq!(THUNDER_PUNCH_TRICK.sphere_tags.len(), 8);
        assert_eq!(THUNDER_PUNCH_TRICK.sphere_tags_pt.len(), 8);

        // Termos canônicos em inglês
        assert!(THUNDER_PUNCH_TRICK.paragraphs[0].contains("Correspondence 1"));
        assert!(THUNDER_PUNCH_TRICK.paragraphs[0].contains("maximum adjustment of -3"));
        assert!(THUNDER_PUNCH_TRICK.paragraphs[1].contains("Life 3 or Prime 3 Pattern assault"));
        assert!(THUNDER_PUNCH_TRICK.paragraphs[2].contains("If the target manages to soak every level of damage"));

        // Termos canônicos em português
        assert!(THUNDER_PUNCH_TRICK.paragraphs_pt[0].contains("Correspondência 1"));
        assert!(THUNDER_PUNCH_TRICK.paragraphs_pt[0].contains("ajuste máximo de -3"));
        assert!(THUNDER_PUNCH_TRICK.paragraphs_pt[1].contains("Vida 3 ou Primórdio 3"));
        assert!(THUNDER_PUNCH_TRICK.paragraphs_pt[2].contains("Se o alvo conseguir absorver cada nível de dano"));
    }
}


