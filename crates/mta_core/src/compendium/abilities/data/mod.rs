//! M20 Catálogo Unificado de Habilidades (Abilities).

pub mod talents;
pub mod skills;
pub mod knowledges;
pub mod secondary;
pub mod rules;

pub use talents::CORE_TALENTS;
pub use skills::CORE_SKILLS;
pub use knowledges::CORE_KNOWLEDGES;
pub use secondary::SECONDARY_ABILITIES;
pub use rules::ABILITY_THEORY_RULES;
