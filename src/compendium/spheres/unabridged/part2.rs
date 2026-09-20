//! Textos Integrais M20: Esferas (Parte 2: Primórdio a Talento Selvagem).
//!
//! M20 pp. 523-534. Textos carregados de data/compendium/spheres/*.md.

use crate::i18n::Language;

pub fn prime(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/spheres/prime.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/spheres/prime.en.md"),
    }
}

pub fn spirit(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/spheres/spirit.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/spheres/spirit.en.md"),
    }
}

pub fn time(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/spheres/time.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/spheres/time.en.md"),
    }
}

pub fn data_sphere(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/spheres/data.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/spheres/data.en.md"),
    }
}

pub fn dimensional_science(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/spheres/dimensional_science.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/spheres/dimensional_science.en.md"),
    }
}

pub fn primal_utility(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/spheres/primal_utility.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/spheres/primal_utility.en.md"),
    }
}

pub fn wild_talent(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/spheres/wild_talent.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/spheres/wild_talent.en.md"),
    }
}

pub fn theory_sphere_rules(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/spheres/theory_sphere_rules.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/spheres/theory_sphere_rules.en.md"),
    }
}
