//! Modularized re-export of the unified MTA Compendium Modal.
//! All views (Practices, Instruments, Archetypes, Attributes, Weapons) are implemented
//! under `src/components/compendium/` and unified in `CompendiumModal`.

pub use crate::components::compendium::{
    ArchetypeTarget, CompendiumModal as PracticeCompendiumModal, CompendiumSection,
};
