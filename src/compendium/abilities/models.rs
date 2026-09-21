//! Modelos de Dados para o Compêndio de Habilidades (Abilities) de M20.
//!
//! Capítulo 6: Criação do Personagem (pp. 275-301) e Habilidades Secundárias.

use serde::Serialize;
use crate::i18n::Language;

/// Categoria principal da Habilidade (Talentos, Perícias ou Conhecimentos).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AbilityCategory {
    Talents,
    Skills,
    Knowledges,
}

impl AbilityCategory {
    pub fn name(&self, lang: Language) -> &'static str {
        match (self, lang) {
            (AbilityCategory::Talents, Language::PtBr) => "Talentos",
            (AbilityCategory::Talents, Language::EnUs) => "Talents",
            (AbilityCategory::Skills, Language::PtBr) => "Perícias",
            (AbilityCategory::Skills, Language::EnUs) => "Skills",
            (AbilityCategory::Knowledges, Language::PtBr) => "Conhecimentos",
            (AbilityCategory::Knowledges, Language::EnUs) => "Knowledges",
        }
    }

    pub fn id_str(&self) -> &'static str {
        match self {
            AbilityCategory::Talents => "talents",
            AbilityCategory::Skills => "skills",
            AbilityCategory::Knowledges => "knowledges",
        }
    }
}

/// Escopo da Habilidade: Básica/Canônica da ficha ou Secundária/Opcional.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AbilityScope {
    Core,
    Secondary,
}

impl AbilityScope {
    pub fn label(&self, lang: Language) -> &'static str {
        match (self, lang) {
            (AbilityScope::Core, Language::PtBr) => "Básica (Ficha)",
            (AbilityScope::Core, Language::EnUs) => "Core (Sheet)",
            (AbilityScope::Secondary, Language::PtBr) => "Secundária",
            (AbilityScope::Secondary, Language::EnUs) => "Secondary",
        }
    }

    pub fn name(&self, lang: Language) -> &'static str {
        self.label(lang)
    }
}

/// Nível de pontuação de uma Habilidade (de 1 a 5 pontos).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct AbilityRating {
    pub dots: i32,
    pub title: &'static str,
    pub title_pt: &'static str,
    pub description: &'static str,
    pub description_pt: &'static str,
}

impl AbilityRating {
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

/// Definição Canônica de uma Habilidade M20.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct AbilityDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub name_pt: &'static str,
    pub category: AbilityCategory,
    pub scope: AbilityScope,
    pub page_ref: &'static str,
    pub description: &'static str,
    pub description_pt: &'static str,
    pub ratings: &'static [AbilityRating],
    pub possessed_by: &'static str,
    pub possessed_by_pt: &'static str,
    pub suggested_specialties: &'static [&'static str],
    pub suggested_specialties_pt: &'static [&'static str],
}

impl AbilityDefinition {
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

    pub fn description(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.description_pt,
            Language::EnUs => self.description,
        }
    }

    pub fn possessed_by(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.possessed_by_pt,
            Language::EnUs => self.possessed_by,
        }
    }

    pub fn specialties(&self, lang: Language) -> &'static [&'static str] {
        match lang {
            Language::PtBr => self.suggested_specialties_pt,
            Language::EnUs => self.suggested_specialties,
        }
    }
}

/// Artigo Canônico de Regra Opcional relacionada a Habilidades.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct AbilityTheoryArticle {
    pub id: &'static str,
    pub title: &'static str,
    pub title_pt: &'static str,
    pub page_ref: &'static str,
    pub content: &'static str,
    pub content_pt: &'static str,
}

impl AbilityTheoryArticle {
    pub fn title(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.title_pt,
            Language::EnUs => self.title,
        }
    }

    pub fn content(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.content_pt,
            Language::EnUs => self.content,
        }
    }
}
