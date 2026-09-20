//! Modelos de Dados e Tipos do Compêndio de Armas & Manobras de Combate (M20)

use serde::{Serialize, Deserialize};
use crate::i18n::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatSubTab {
    Weapons,
    Maneuvers,
}

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
    Knives,
    Swords,
    Axes,
    FistExtension,
    Clubbing,
    WhipsAndChains,
    Improvised,

    // --- Subcategorias de Armas de Fogo & Distância (M20 pp. 452-453) ---
    Pistols,
    RiflesAndSmgs,
    Shotguns,
    Bows,
    HeavyAndMilitary,
    TechnocracySidearms,
    NonLethal,
    ThrownWeapons,
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

    pub fn weapon_class(&self) -> WeaponClass {
        match self {
            WeaponCategory::Knives
            | WeaponCategory::Swords
            | WeaponCategory::Axes
            | WeaponCategory::FistExtension
            | WeaponCategory::Clubbing
            | WeaponCategory::WhipsAndChains
            | WeaponCategory::Improvised => WeaponClass::Melee,

            WeaponCategory::ThrownWeapons => WeaponClass::Thrown,

            WeaponCategory::Pistols
            | WeaponCategory::RiflesAndSmgs
            | WeaponCategory::Shotguns
            | WeaponCategory::Bows
            | WeaponCategory::HeavyAndMilitary
            | WeaponCategory::TechnocracySidearms
            | WeaponCategory::NonLethal => WeaponClass::Ranged,
        }
    }

    pub fn class(&self) -> WeaponClass {
        self.weapon_class()
    }

    pub fn name(&self, lang: Language) -> &'static str {
        match (self, lang) {
            (WeaponCategory::Knives, Language::PtBr) => "Facas & Adagas",
            (WeaponCategory::Knives, Language::EnUs) => "Knives & Daggers",
            (WeaponCategory::Swords, Language::PtBr) => "Espadas & Lâminas",
            (WeaponCategory::Swords, Language::EnUs) => "Swords & Blades",
            (WeaponCategory::Axes, Language::PtBr) => "Machados",
            (WeaponCategory::Axes, Language::EnUs) => "Axes",
            (WeaponCategory::FistExtension, Language::PtBr) => "Extensões de Punho",
            (WeaponCategory::FistExtension, Language::EnUs) => "Fist Extensions",
            (WeaponCategory::Clubbing, Language::PtBr) => "Clavas & Concussão",
            (WeaponCategory::Clubbing, Language::EnUs) => "Clubbing & Bludgeons",
            (WeaponCategory::WhipsAndChains, Language::PtBr) => "Chicotes & Correntes",
            (WeaponCategory::WhipsAndChains, Language::EnUs) => "Whips & Chains",
            (WeaponCategory::Improvised, Language::PtBr) => "Armas Improvisadas",
            (WeaponCategory::Improvised, Language::EnUs) => "Improvised Weapons",
            (WeaponCategory::Pistols, Language::PtBr) => "Pistolas & Revólveres",
            (WeaponCategory::Pistols, Language::EnUs) => "Pistols & Revolvers",
            (WeaponCategory::RiflesAndSmgs, Language::PtBr) => "Rifles & Submetralhadoras",
            (WeaponCategory::RiflesAndSmgs, Language::EnUs) => "Rifles & SMGs",
            (WeaponCategory::Shotguns, Language::PtBr) => "Espingardas (Shotguns)",
            (WeaponCategory::Shotguns, Language::EnUs) => "Shotguns",
            (WeaponCategory::Bows, Language::PtBr) => "Arcos & Bestas",
            (WeaponCategory::Bows, Language::EnUs) => "Bows & Crossbows",
            (WeaponCategory::HeavyAndMilitary, Language::PtBr) => "Armamento Pesado & Militar",
            (WeaponCategory::HeavyAndMilitary, Language::EnUs) => "Heavy & Military Weapons",
            (WeaponCategory::TechnocracySidearms, Language::PtBr) => "Armamento Tecnocrata",
            (WeaponCategory::TechnocracySidearms, Language::EnUs) => "Technocracy Firearms",
            (WeaponCategory::NonLethal, Language::PtBr) => "Não-Letais & Imobilização",
            (WeaponCategory::NonLethal, Language::EnUs) => "Non-Lethal & Stun",
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
            WeaponCategory::Improvised => "🪵",
            WeaponCategory::Pistols => "🔫",
            WeaponCategory::RiflesAndSmgs => "🎯",
            WeaponCategory::Shotguns => "💥",
            WeaponCategory::Bows => "🏹",
            WeaponCategory::HeavyAndMilitary => "💣",
            WeaponCategory::TechnocracySidearms => "⚡",
            WeaponCategory::NonLethal => "🔋",
            WeaponCategory::ThrownWeapons => "🎲",
        }
    }

    pub fn id_str(&self) -> &'static str {
        match self {
            WeaponCategory::Knives => "knives",
            WeaponCategory::Swords => "swords",
            WeaponCategory::Axes => "axes",
            WeaponCategory::FistExtension => "fist_extension",
            WeaponCategory::Clubbing => "clubbing",
            WeaponCategory::WhipsAndChains => "whips_chains",
            WeaponCategory::Improvised => "improvised",
            WeaponCategory::Pistols => "pistols",
            WeaponCategory::RiflesAndSmgs => "rifles_smgs",
            WeaponCategory::Shotguns => "shotguns",
            WeaponCategory::Bows => "bows",
            WeaponCategory::HeavyAndMilitary => "heavy_military",
            WeaponCategory::TechnocracySidearms => "technocracy",
            WeaponCategory::NonLethal => "non_lethal",
            WeaponCategory::ThrownWeapons => "thrown",
        }
    }

    pub fn from_id_str(s: &str) -> Option<Self> {
        match s {
            "knives" => Some(WeaponCategory::Knives),
            "swords" => Some(WeaponCategory::Swords),
            "axes" => Some(WeaponCategory::Axes),
            "fist_extension" => Some(WeaponCategory::FistExtension),
            "clubbing" => Some(WeaponCategory::Clubbing),
            "whips_chains" => Some(WeaponCategory::WhipsAndChains),
            "improvised" => Some(WeaponCategory::Improvised),
            "pistols" => Some(WeaponCategory::Pistols),
            "rifles_smgs" => Some(WeaponCategory::RiflesAndSmgs),
            "shotguns" => Some(WeaponCategory::Shotguns),
            "bows" => Some(WeaponCategory::Bows),
            "heavy_military" => Some(WeaponCategory::HeavyAndMilitary),
            "technocracy" => Some(WeaponCategory::TechnocracySidearms),
            "non_lethal" => Some(WeaponCategory::NonLethal),
            "thrown" => Some(WeaponCategory::ThrownWeapons),
            _ => None,
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
                Language::PtBr => "C/C".to_string(),
                Language::EnUs => "Close".to_string(),
            },
            rate: self.actions.to_string(),
            clip: "—".to_string(),
            conceal: "—".to_string(),
            notes: self.requirement(lang).to_string(),
        }
    }
}

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
