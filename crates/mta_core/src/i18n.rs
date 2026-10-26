//! Idioma da interface e dos textos do compêndio.

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
