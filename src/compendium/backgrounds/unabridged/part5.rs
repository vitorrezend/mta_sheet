//! Textos Integrais M20: Antecedentes (Parte 5: Recursos a Maravilha).
//!
//! M20 pp. 317-322. Textos carregados de data/compendium/backgrounds/*.md.

use crate::i18n::Language;

pub fn resources(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/resources.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/resources.en.md"),
    }
}

pub fn retainers(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/retainers.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/retainers.en.md"),
    }
}

pub fn sanctum(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/sanctum.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/sanctum.en.md"),
    }
}

pub fn secret_weapons(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/secret_weapons.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/secret_weapons.en.md"),
    }
}

pub fn spies(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/spies.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/spies.en.md"),
    }
}

pub fn status(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/status.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/status.en.md"),
    }
}

pub fn totem(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/totem.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/totem.en.md"),
    }
}

pub fn wonder(lang: Language) -> &'static str {
    match lang {
        Language::PtBr => include_str!("../../../../data/compendium/backgrounds/wonder.pt.md"),
        Language::EnUs => include_str!("../../../../data/compendium/backgrounds/wonder.en.md"),
    }
}
