//! M20 Rules Compendium
//!
//! Provides static reference and lookup structures for Paradigms, Practices, Instruments,
//! and rules from Mage: The Ascension 20th Anniversary Edition.

pub mod practices;
pub mod instruments;
pub mod attributes;
pub mod weapons;
pub mod archetypes;
pub mod backgrounds;
pub mod spheres;
pub mod abilities;
pub mod merits_flaws;

pub use practices::*;
pub use instruments::*;
pub use attributes::*;
pub use weapons::*;
pub use archetypes::*;
pub use backgrounds::{
    find_background, get_all_backgrounds, ALL_BACKGROUNDS, BACKGROUND_THEORY_RULES,
    BackgroundDefinition, BackgroundRating, BackgroundTheoryArticle, BackupTeamSample, ChantryPoolLevel,
};
pub use spheres::{
    find_sphere, get_all_spheres, ALL_SPHERES, SphereDefinition, SphereRank,
};
pub use abilities::{
    find_ability, find_ability_theory_rule, ALL_ABILITIES, ABILITY_THEORY_RULES,
    AbilityCategory, AbilityDefinition, AbilityRating, AbilityScope, AbilityTheoryArticle,
};
pub use merits_flaws::{
    find_derangement, find_merit_flaw, ALL_DERANGEMENTS, ALL_FLAWS, ALL_MERITS, ALL_MERITS_FLAWS,
    DerangementDefinition, MeritFlawDefinition, TraitCategory, TraitType,
};

