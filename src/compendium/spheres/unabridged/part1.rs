//! Textos Integrais M20: Esferas (Parte 1: Correspondência a Mente).
//!
//! M20 pp. 504-523. Textos carregados de data/compendium/spheres/*.md.

use crate::i18n::Language;

pub fn correspondence(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/spheres/correspondence.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/spheres/correspondence.en.md"),
    }
}

pub fn entropy(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/spheres/entropy.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/spheres/entropy.en.md"),
    }
}

pub fn forces(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/spheres/forces.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/spheres/forces.en.md"),
    }
}

pub fn life(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/spheres/life.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/spheres/life.en.md"),
    }
}

pub fn matter(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/spheres/matter.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/spheres/matter.en.md"),
    }
}

pub fn mind(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/spheres/mind.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/spheres/mind.en.md"),
    }
}
