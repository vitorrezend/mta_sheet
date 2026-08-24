#![allow(unused_imports)]

pub mod common;
pub mod gods_and_monsters;
pub mod mta_sheet;
pub mod profile;
pub mod rooms;
pub mod views;

// Re-export common UI components
pub use common::{
    Callback, LabelColumn, LabelField, Navbar, SafeCallback, Sheet, StableTextArea,
    StableTextInput, ValueField,
};

// Re-export MTA sheet components
pub use mta_sheet::{
    page1, page2, page3, page4, page5, sheet,
    Abilities, ActiveDotOriginContext, AdvantagesMta, Attributes, Chantry, CombatSection,
    CostBreakdownModal, DescriptionSection, ExpandedBackgrounds, HistorySection, InfoHeader,
    MagicSection, MeritsFlaws, OtherTraits, PageExpandedBackgroundsPossessions,
    PageHistoryDescriptionVisuals, PageMagicCombat, Possessions, SaveStatus, SheetPageTab,
    SheetTabs, SheetTopBar, Spheres, VisualsSection, PageGrimoire,
};

// Re-export Profile components
pub use profile::CharacterProfile;

// Re-export Rooms components
pub use rooms::{RoomView, RoomsPage};

// Re-export Views / Top-level Pages
pub use views::{AuthPage, CharacterSheet, Home, LogsPage};

// Compatibility module for legacy imports
pub mod character_sheet {
    pub use crate::components::mta_sheet::sheet::ActiveDotOriginContext;
    pub use crate::components::views::character_sheet::*;
}
