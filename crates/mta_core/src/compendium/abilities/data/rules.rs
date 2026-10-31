//! Artigos Canônicos de Regras Opcionais e Teoria de Habilidades de M20.
//!
//! Textos completos sobre Controle Corporal, Habilidades Mínimas, O Artesão Habilidoso,
//! Perícias Profissionais/Conhecimentos Especialistas e Regras de Ciência Inspirada.

use super::super::models::AbilityTheoryArticle;

pub const RULE_BODY_CONTROL: AbilityTheoryArticle = AbilityTheoryArticle {
    id: "rule_body_control",
    title: "Optional Rule: Body Control",
    title_pt: "Regra Opcional: Controle Corporal",
    page_ref: "M20, p. 285",
    content: include_str!("../../../../../../data/compendium/abilities/rules/rule_body_control.en.md"),
    content_pt: include_str!("../../../../../../data/compendium/abilities/rules/rule_body_control.pt.md"),
};

pub const RULE_MINIMUM_ABILITIES: AbilityTheoryArticle = AbilityTheoryArticle {
    id: "rule_minimum_abilities",
    title: "Optional Rule: Minimum Abilities",
    title_pt: "Regra Opcional: Habilidades Mínimas",
    page_ref: "M20, p. 275",
    content: include_str!("../../../../../../data/compendium/abilities/rules/rule_minimum_abilities.en.md"),
    content_pt: include_str!("../../../../../../data/compendium/abilities/rules/rule_minimum_abilities.pt.md"),
};

pub const RULE_WELL_SKILLED_CRAFTSMAN: AbilityTheoryArticle = AbilityTheoryArticle {
    id: "rule_well_skilled_craftsman",
    title: "Optional Rule: The Well-Skilled Craftsman",
    title_pt: "Regra Opcional: O Artesão Habilidoso",
    page_ref: "M20, p. 279",
    content: include_str!("../../../../../../data/compendium/abilities/rules/rule_well_skilled_craftsman.en.md"),
    content_pt: include_str!("../../../../../../data/compendium/abilities/rules/rule_well_skilled_craftsman.pt.md"),
};

pub const RULE_PROFESSIONAL_EXPERT: AbilityTheoryArticle = AbilityTheoryArticle {
    id: "rule_professional_expert",
    title: "Optional Rule: Professional Skills and Expert Knowledges",
    title_pt: "Regra Opcional: Perícias Profissionais & Conhecimentos Especialistas",
    page_ref: "M20, p. 277",
    content: include_str!("../../../../../../data/compendium/abilities/rules/rule_professional_expert.en.md"),
    content_pt: include_str!("../../../../../../data/compendium/abilities/rules/rule_professional_expert.pt.md"),
};

pub const RULE_SCIENCE_THEORY: AbilityTheoryArticle = AbilityTheoryArticle {
    id: "rule_science_theory",
    title: "SCIENCE!!!",
    title_pt: "CIÊNCIA!!! Teoria e Mágika Coincidente",
    page_ref: "M20, p. 290",
    content: include_str!("../../../../../../data/compendium/abilities/rules/rule_science_theory.en.md"),
    content_pt: include_str!("../../../../../../data/compendium/abilities/rules/rule_science_theory.pt.md"),
};

pub const SIDEBAR_DO_MARTIAL_ARTS: AbilityTheoryArticle = AbilityTheoryArticle {
    id: "sidebar_do_martial_arts",
    title: "Do and Other Martial Arts",
    title_pt: "Dô e Outras Artes Marciais",
    page_ref: "M20, p. 281",
    content: include_str!("../../../../../../data/compendium/abilities/rules/sidebar_do_martial_arts.en.md"),
    content_pt: include_str!("../../../../../../data/compendium/abilities/rules/sidebar_do_martial_arts.pt.md"),
};

pub const SIDEBAR_SCIENTIFIC_SPECIALTIES: AbilityTheoryArticle = AbilityTheoryArticle {
    id: "sidebar_scientific_specialties",
    title: "Scientific Specialties",
    title_pt: "Especializações Científicas",
    page_ref: "M20, pp. 288-289",
    content: include_str!("../../../../../../data/compendium/abilities/rules/sidebar_scientific_specialties.en.md"),
    content_pt: include_str!("../../../../../../data/compendium/abilities/rules/sidebar_scientific_specialties.pt.md"),
};

pub const ABILITY_THEORY_RULES: &[AbilityTheoryArticle] = &[
    RULE_MINIMUM_ABILITIES,
    RULE_PROFESSIONAL_EXPERT,
    RULE_WELL_SKILLED_CRAFTSMAN,
    SIDEBAR_DO_MARTIAL_ARTS,
    RULE_BODY_CONTROL,
    SIDEBAR_SCIENTIFIC_SPECIALTIES,
    RULE_SCIENCE_THEORY,
];
