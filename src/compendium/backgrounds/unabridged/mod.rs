//! Textos Integrais M20 dos 33 Antecedentes Canônicos.
//!
//! Submódulo modularizado em conformidade com o limite de 1.000 linhas
//! e com suporte a hyperlinks internos do protocolo `mta://`.

pub mod part1;
pub mod part2;
pub mod part3;
pub mod part4;
pub mod part5;

use crate::i18n::Language;

/// Retorna o texto integral do antecedente M20 no idioma especificado, se disponível.
pub fn find_unabridged_text(id: &str, lang: Language) -> Option<&'static str> {
    match id {
        // Parte 1 (Aliados a Certificação)
        "allies" => Some(part1::allies(lang)),
        "alternate_identity" => Some(part1::alternate_identity(lang)),
        "arcane" => Some(part1::arcane(lang)),
        "avatar" => Some(part1::avatar(lang)),
        "backup" => Some(part1::backup(lang)),
        "blessing" => Some(part1::blessing(lang)),
        "certification" => Some(part1::certification(lang)),

        // Parte 2 (Capela a Sonho)
        "chantry" => Some(part2::chantry(lang)),
        "contacts" => Some(part2::contacts(lang)),
        "cult" => Some(part2::cult(lang)),
        "demesne" => Some(part2::demesne(lang)),
        "destiny" => Some(part2::destiny(lang)),
        "dream" => Some(part2::dream(lang)),

        // Parte 3 (Aprimoramento a Biblioteca)
        "enhancement" => Some(part3::enhancement(lang)),
        "fame" => Some(part3::fame(lang)),
        "familiar" => Some(part3::familiar(lang)),
        "influence" => Some(part3::influence(lang)),
        "legend" => Some(part3::legend(lang)),
        "library" => Some(part3::library(lang)),

        // Parte 4 (Mentor a Requisições)
        "mentor" => Some(part4::mentor(lang)),
        "node" => Some(part4::node(lang)),
        "past_lives" => Some(part4::past_lives(lang)),
        "patron" => Some(part4::patron(lang)),
        "rank" => Some(part4::rank(lang)),
        "requisitions" => Some(part4::requisitions(lang)),

        // Parte 5 (Recursos a Maravilha)
        "resources" => Some(part5::resources(lang)),
        "retainers" => Some(part5::retainers(lang)),
        "sanctum" => Some(part5::sanctum(lang)),
        "secret_weapons" => Some(part5::secret_weapons(lang)),
        "spies" => Some(part5::spies(lang)),
        "status" => Some(part5::status(lang)),
        "totem" => Some(part5::totem(lang)),
        "wonder" => Some(part5::wonder(lang)),

        _ => None,
    }
}
