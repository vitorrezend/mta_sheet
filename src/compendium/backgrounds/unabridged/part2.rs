//! Textos Integrais M20: Antecedentes (Parte 2: Capela a Sonho).
//!
//! M20 pp. 307-311. Textos carregados de data/compendium/backgrounds/*.md.

use crate::i18n::Language;

pub fn chantry(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/chantry.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/chantry.en.md"),
    }
}

pub fn contacts(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/contacts.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/contacts.en.md"),
    }
}

pub fn cult(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/cult.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/cult.en.md"),
    }
}

pub fn demesne(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/demesne.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/demesne.en.md"),
    }
}

pub fn destiny(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/destiny.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/destiny.en.md"),
    }
}

pub fn dream(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/dream.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/dream.en.md"),
    }
}
