//! M20 (Mage: The Ascension 20th Anniversary Edition) - Compendium of Weapons & Combat
//!
//! Canonical reference text and tables from Chapter 9 (Combat & Storytelling, pp. 450-453),
//! with full bilingual support (English & Portuguese), page numbers, categories, stats
//! (Difficulty, Damage/Type, Range, Rate, Clip, Conceal), Melee Combat Rule Notes (#1 to #10),
//! and Ranged Firearms/Bows Rule Notes (#1 to #12).

use serde::Serialize;
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
