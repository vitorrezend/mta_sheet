//! Instrumentos Expandidos do M20 Book of Secrets, Capítulo Três: Matters of Focus
//! (BoS, pp. 205-209), com o quadro "Prepped and Ready" Instrumental Operations.
//!
//! Texto integral em inglês em data/compendium/instruments/bos/*.en.md e tradução
//! integral em *.pt.md.

use super::instruments::{InstrumentDefinition, InstrumentTheoryArticle};

/// Introdução "Expanded Instruments" (BoS, p. 205).
pub const THEORY_EXPANDED_INSTRUMENTS: InstrumentTheoryArticle = InstrumentTheoryArticle {
    id: "expanded_instruments",
    title: "Expanded Instruments",
    title_pt: "Instrumentos Expandidos",
    page_ref: "BoS, p. 205",
    content: include_str!("../../../../data/compendium/instruments/bos/expanded_instruments.en.md"),
    content_pt: include_str!("../../../../data/compendium/instruments/bos/expanded_instruments.pt.md"),
};

/// Quadro "Prepped and Ready" Instrumental Operations (BoS, p. 206).
pub const THEORY_PREPPED_AND_READY: InstrumentTheoryArticle = InstrumentTheoryArticle {
    id: "prepped_and_ready",
    title: "“Prepped and Ready” Instrumental Operations",
    title_pt: "Operações Instrumentais “Preparadas e Prontas”",
    page_ref: "BoS, p. 206",
    content: include_str!("../../../../data/compendium/instruments/bos/prepped_and_ready.en.md"),
    content_pt: include_str!("../../../../data/compendium/instruments/bos/prepped_and_ready.pt.md"),
};

pub const BOS_BODY_MODIFICATION: InstrumentDefinition = InstrumentDefinition {
    id: "body_modification",
    name: "Body Modification",
    name_pt: "Modificação Corporal",
    aliases: &[r#"Body Modification"#, r#"Modificação Corporal"#, r#"Body Mod"#, r#"Body Modifications"#, r#"Tattoos"#, r#"Piercings"#, r#"Tatuagens"#],
    page_ref: "BoS, pp. 205-206",
    category: "bos",
    description: include_str!("../../../../data/compendium/instruments/bos/body_modification.en.md"),
    description_pt: include_str!("../../../../data/compendium/instruments/bos/body_modification.pt.md"),
};

pub const BOS_CANNIBALISM: InstrumentDefinition = InstrumentDefinition {
    id: "cannibalism",
    name: "Cannibalism",
    name_pt: "Canibalismo",
    aliases: &[r#"Cannibalism"#, r#"Canibalismo"#],
    page_ref: "BoS, pp. 206-207",
    category: "bos",
    description: include_str!("../../../../data/compendium/instruments/bos/cannibalism.en.md"),
    description_pt: include_str!("../../../../data/compendium/instruments/bos/cannibalism.pt.md"),
};

pub const BOS_CYBERNETIC_IMPLANTS: InstrumentDefinition = InstrumentDefinition {
    id: "cybernetic_implants",
    name: "Cybernetic Implants",
    name_pt: "Implantes Cibernéticos",
    aliases: &[r#"Cybernetic Implants"#, r#"Implantes Cibernéticos"#, r#"Cybernetics Implants"#, r#"Implants"#, r#"Implantes"#],
    page_ref: "BoS, pp. 207-208",
    category: "bos",
    description: include_str!("../../../../data/compendium/instruments/bos/cybernetic_implants.en.md"),
    description_pt: include_str!("../../../../data/compendium/instruments/bos/cybernetic_implants.pt.md"),
};

pub const BOS_GENETIC_MANIPULATION: InstrumentDefinition = InstrumentDefinition {
    id: "genetic_manipulation",
    name: "Genetic Manipulation",
    name_pt: "Manipulação Genética",
    aliases: &[r#"Genetic Manipulation"#, r#"Manipulação Genética"#, r#"Gene Therapy"#, r#"Genetic Engineering"#, r#"Engenharia Genética"#],
    page_ref: "BoS, p. 208",
    category: "bos",
    description: include_str!("../../../../data/compendium/instruments/bos/genetic_manipulation.en.md"),
    description_pt: include_str!("../../../../data/compendium/instruments/bos/genetic_manipulation.pt.md"),
};

pub const BOS_INTERNET_ACTIVITY: InstrumentDefinition = InstrumentDefinition {
    id: "internet_activity",
    name: "Internet Activity",
    name_pt: "Atividade na Internet",
    aliases: &[r#"Internet Activity"#, r#"Atividade na Internet"#, r#"Internet"#, r#"Online Activity"#, r#"Atividade Online"#],
    page_ref: "BoS, pp. 208-209",
    category: "bos",
    description: include_str!("../../../../data/compendium/instruments/bos/internet_activity.en.md"),
    description_pt: include_str!("../../../../data/compendium/instruments/bos/internet_activity.pt.md"),
};

pub const BOS_MEDICAL_PROCEDURES: InstrumentDefinition = InstrumentDefinition {
    id: "medical_procedures",
    name: "Medical Procedures",
    name_pt: "Procedimentos Médicos",
    aliases: &[r#"Medical Procedures"#, r#"Procedimentos Médicos"#, r#"Medical Procedure"#, r#"Surgery"#, r#"Cirurgia"#, r#"Procedimento Médico"#],
    page_ref: "BoS, p. 209",
    category: "bos",
    description: include_str!("../../../../data/compendium/instruments/bos/medical_procedures.en.md"),
    description_pt: include_str!("../../../../data/compendium/instruments/bos/medical_procedures.pt.md"),
};

pub const BOS_TRANSGRESSION: InstrumentDefinition = InstrumentDefinition {
    id: "transgression",
    name: "Transgression",
    name_pt: "Transgressão",
    aliases: &[r#"Transgression"#, r#"Transgressão"#, r#"Antinomianism"#, r#"Antinomianismo"#],
    page_ref: "BoS, p. 209",
    category: "bos",
    description: include_str!("../../../../data/compendium/instruments/bos/transgression.en.md"),
    description_pt: include_str!("../../../../data/compendium/instruments/bos/transgression.pt.md"),
};
