#![allow(unused_imports)]

pub mod common;
pub mod page1;
pub mod page2;
pub mod profile;
pub mod rooms;
pub mod sheet;
pub mod views;

// Re-export common UI components
pub use common::{LabelColumn, LabelField, Navbar, Sheet, StableTextArea, StableTextInput, ValueField};

// Re-export Page 1 components
pub use page1::{Abilities, AdvantagesMta, Attributes, InfoHeader, Spheres};

// Re-export Page 2 components
pub use page2::{CombatSection, MagicSection, MeritsFlaws, OtherTraits, PageMagicCombat};

// Re-export Profile components
pub use profile::CharacterProfile;

// Re-export Rooms components
pub use rooms::{RoomView, RoomsPage};

// Re-export Sheet orchestration components
pub use sheet::{ActiveDotOriginContext, CostBreakdownModal, SaveStatus, SheetPageTab, SheetTabs, SheetTopBar};

// Re-export Views / Top-level Pages
pub use views::{AuthPage, CharacterSheet, Home, LogsPage};

// Compatibility module for legacy imports
pub mod character_sheet {
    pub use crate::components::sheet::ActiveDotOriginContext;
    pub use crate::components::views::character_sheet::*;
}
