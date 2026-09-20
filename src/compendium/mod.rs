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

