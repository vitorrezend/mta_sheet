//! Textos Integrais M20: Antecedentes (Parte 3: Aprimoramento a Biblioteca).
//!
//! M20 pp. 311-314. Textos carregados de data/compendium/backgrounds/*.md.

use crate::i18n::Language;

pub fn enhancement(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/enhancement.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/enhancement.en.md"),
    }
}

pub fn fame(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/fame.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/fame.en.md"),
    }
}

pub fn familiar(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/familiar.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/familiar.en.md"),
    }
}

pub fn influence(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/influence.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/influence.en.md"),
    }
}

pub fn legend(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/legend.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/legend.en.md"),
    }
}

pub fn library(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/library.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/library.en.md"),
    }
}
