//! Modelos de Dados para o Compêndio de Esferas (Spheres) de M20.
//!
//! Capítulo 10: O Livro das Esferas (pp. 504-533).

use serde::Serialize;
use crate::i18n::Language;

/// Posto / Graduação de uma Esfera (de 1 a 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SphereRank {
    pub rank: i32,
    pub name: &'static str,
    pub name_pt: &'static str,
    pub description: &'static str,
    pub description_pt: &'static str,
}

impl SphereRank {
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
}

/// Definição Canônica de uma Esfera M20.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SphereDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub name_pt: &'static str,
    pub subtitle: &'static str,
    pub subtitle_pt: &'static str,
    pub page_ref: &'static str,
    pub specialties: &'static str,
    pub specialties_pt: &'static str,
    pub description: &'static str,
    pub description_pt: &'static str,
    pub ranks: &'static [SphereRank],
    pub technocracy_equivalent: Option<&'static str>,
    pub technocracy_equivalent_pt: Option<&'static str>,
    pub mystic_equivalent: Option<&'static str>,
    pub mystic_equivalent_pt: Option<&'static str>,
    pub is_optional_rule: bool,
}

impl SphereDefinition {
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

    pub fn subtitle(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.subtitle_pt,
            Language::EnUs => self.subtitle,
        }
    }

    pub fn specialties(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.specialties_pt,
            Language::EnUs => self.specialties,
        }
    }

    pub fn suggested_specialties(&self, lang: Language) -> Vec<&'static str> {
        let raw = self.specialties(lang);
        raw.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
    }

    pub fn description(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.description_pt,
            Language::EnUs => self.description,
        }
    }

    pub fn equivalent_note(&self, lang: Language) -> Option<(&'static str, &'static str)> {
        if let (Some(eq_en), Some(eq_pt)) = (self.technocracy_equivalent, self.technocracy_equivalent_pt) {
            let label = match lang {
                Language::PtBr => "Equivalente Tecnocrático: ",
                Language::EnUs => "Technocratic Equivalent: ",
            };
            let val = match lang {
                Language::PtBr => eq_pt,
                Language::EnUs => eq_en,
            };
            Some((label, val))
        } else if let (Some(eq_en), Some(eq_pt)) = (self.mystic_equivalent, self.mystic_equivalent_pt) {
            let label = match lang {
                Language::PtBr => "Equivalente Místico: ",
                Language::EnUs => "Mystic Equivalent: ",
            };
            let val = match lang {
                Language::PtBr => eq_pt,
                Language::EnUs => eq_en,
            };
            Some((label, val))
        } else {
            None
        }
    }
}

/// Artigo Teórico e Regras Gerais das Esferas (M20, pp. 511-512).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SphereTheoryArticle {
    pub id: &'static str,
    pub title: &'static str,
    pub title_pt: &'static str,
    pub page_ref: &'static str,
    pub content: &'static str,
    pub content_pt: &'static str,
}

impl SphereTheoryArticle {
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
