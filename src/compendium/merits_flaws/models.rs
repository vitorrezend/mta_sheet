//! Modelos de Dados para o Compêndio de Qualidades, Defeitos e Perturbações (M20).
//!
//! Capítulo 6: Criação do Personagem (pp. 253-256) e Apêndice II (pp. 642-650).

use serde::Serialize;
use crate::i18n::Language;

/// Tipo de Trait: Qualidade (Merit) ou Defeito (Flaw).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TraitType {
    Merit,
    Flaw,
}

impl TraitType {
    pub fn name(&self, lang: Language) -> &'static str {
        match (self, lang) {
            (TraitType::Merit, Language::PtBr) => "Qualidade",
            (TraitType::Merit, Language::EnUs) => "Merit",
            (TraitType::Flaw, Language::PtBr) => "Defeito",
            (TraitType::Flaw, Language::EnUs) => "Flaw",
        }
    }

    pub fn id_str(&self) -> &'static str {
        match self {
            TraitType::Merit => "merits",
            TraitType::Flaw => "flaws",
        }
    }
}

/// Categoria do Trait (Físico, Mental, Social ou Sobrenatural).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TraitCategory {
    Physical,
    Mental,
    Social,
    Supernatural,
}

impl TraitCategory {
    pub fn name(&self, lang: Language) -> &'static str {
        match (self, lang) {
            (TraitCategory::Physical, Language::PtBr) => "Físico",
            (TraitCategory::Physical, Language::EnUs) => "Physical",
            (TraitCategory::Mental, Language::PtBr) => "Mental",
            (TraitCategory::Mental, Language::EnUs) => "Mental",
            (TraitCategory::Social, Language::PtBr) => "Social",
            (TraitCategory::Social, Language::EnUs) => "Social",
            (TraitCategory::Supernatural, Language::PtBr) => "Sobrenatural",
            (TraitCategory::Supernatural, Language::EnUs) => "Supernatural",
        }
    }

    pub fn id_str(&self) -> &'static str {
        match self {
            TraitCategory::Physical => "physical",
            TraitCategory::Mental => "mental",
            TraitCategory::Social => "social",
            TraitCategory::Supernatural => "supernatural",
        }
    }
}

/// Definição Canônica de uma Qualidade ou Defeito M20.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct MeritFlawDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub name_pt: &'static str,
    pub technocracy_name: Option<&'static str>,
    pub technocracy_name_pt: Option<&'static str>,
    pub trait_type: TraitType,
    pub category: TraitCategory,
    pub points_str: &'static str,
    pub points_str_pt: &'static str,
    pub available_costs: &'static [i32],
    pub page_ref: &'static str,
    pub description: &'static str,
    pub description_pt: &'static str,
    pub system: &'static str,
    pub system_pt: &'static str,
}

impl MeritFlawDefinition {
    pub fn name(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt,
            Language::EnUs => self.name,
        }
    }

    pub fn secondary_name(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name,
            Language::EnUs => self.name_pt,
        }
    }

    pub fn technocracy_name(&self, lang: Language) -> Option<&'static str> {
        match lang {
            Language::PtBr => self.technocracy_name_pt.or(self.technocracy_name),
            Language::EnUs => self.technocracy_name,
        }
    }

    pub fn points_display(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.points_str_pt,
            Language::EnUs => self.points_str,
        }
    }

    pub fn description(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.description_pt,
            Language::EnUs => self.description,
        }
    }

    pub fn system(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.system_pt,
            Language::EnUs => self.system,
        }
    }
}

/// Definição Canônica de uma Perturbação Mental (Derangement) de M20.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DerangementDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub name_pt: &'static str,
    pub page_ref: &'static str,
    pub description: &'static str,
    pub description_pt: &'static str,
    pub game_effects: &'static str,
    pub game_effects_pt: &'static str,
}

impl DerangementDefinition {
    pub fn name(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt,
            Language::EnUs => self.name,
        }
    }

    pub fn description(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.description_pt,
            Language::EnUs => self.description,
        }
    }

    pub fn effects(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.game_effects_pt,
            Language::EnUs => self.game_effects,
        }
    }
}
