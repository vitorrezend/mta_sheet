//! Textos Integrais M20: Antecedentes (Parte 4: Mentor a Requisições).
//!
//! M20 pp. 314-317. Textos carregados de data/compendium/backgrounds/*.md.

use crate::i18n::Language;

pub fn mentor(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/mentor.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/mentor.en.md"),
    }
}

pub fn node(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/node.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/node.en.md"),
    }
}

pub fn past_lives(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/past_lives.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/past_lives.en.md"),
    }
}

pub fn patron(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/patron.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/patron.en.md"),
    }
}

pub fn rank(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/rank.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/rank.en.md"),
    }
}

pub fn requisitions(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/requisitions.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/requisitions.en.md"),
    }
}
