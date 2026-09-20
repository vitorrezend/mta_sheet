//! Textos Integrais M20: Antecedentes (Parte 1: Aliados a Certificação).
//!
//! M20 pp. 303-307. Textos carregados de data/compendium/backgrounds/*.md.

use crate::i18n::Language;

pub fn allies(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/allies.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/allies.en.md"),
    }
}

pub fn alternate_identity(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/alternate_identity.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/alternate_identity.en.md"),
    }
}

pub fn arcane(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/arcane.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/arcane.en.md"),
    }
}

pub fn avatar(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/avatar.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/avatar.en.md"),
    }
}

pub fn backup(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/backup.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/backup.en.md"),
    }
}

pub fn blessing(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/blessing.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/blessing.en.md"),
    }
}

pub fn certification(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/certification.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/certification.en.md"),
    }
}
