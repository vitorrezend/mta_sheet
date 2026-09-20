//! Textos Integrais M20 das 13 Esferas e Variantes Canônicas.
//!
//! Submódulo modularizado em conformidade com o teto de 1.000 linhas
//! e com suporte a hyperlinks internos do protocolo `mta://`.

pub mod part1;
pub mod part2;

use crate::i18n::Language;

/// Retorna o texto integral da esfera M20 no idioma especificado, se disponível.
pub fn find_unabridged_text(id: &str, lang: Language) -> Option<&'static str> {
    match id {
        // Parte 1 (Correspondência a Mente)
        "correspondence" => Some(part1::correspondence(lang)),
        "entropy" => Some(part1::entropy(lang)),
        "forces" => Some(part1::forces(lang)),
        "life" => Some(part1::life(lang)),
        "matter" => Some(part1::matter(lang)),
        "mind" => Some(part1::mind(lang)),

        // Parte 2 (Primórdio a Talento Selvagem)
        "prime" => Some(part2::prime(lang)),
        "spirit" => Some(part2::spirit(lang)),
        "time" => Some(part2::time(lang)),
        "data" => Some(part2::data_sphere(lang)),
        "dimensional_science" => Some(part2::dimensional_science(lang)),
        "primal_utility" => Some(part2::primal_utility(lang)),
        "wild_talent" => Some(part2::wild_talent(lang)),
        "theory_sphere_rules" => Some(part2::theory_sphere_rules(lang)),

        _ => None,
    }
}
