#![allow(unused_imports)]

pub mod page1;
pub mod page2;
pub mod page3;
pub mod page4;
pub mod page5;
pub mod page6;
pub mod sheet;

// Re-export Page 1 components
pub use page1::{Abilities, AdvantagesMta, Attributes, InfoHeader, Spheres};

// Re-export Page 2 components
pub use page2::{CombatSection, ImageModal, MagicSection, MeritsFlaws, OtherTraits, PageMagicCombat, WonderCard};

// Re-export Page 3 components
pub use page3::{Chantry, ExpandedBackgrounds, PageExpandedBackgroundsPossessions, Possessions};

// Re-export Page 4 components
pub use page4::{DescriptionSection, HistorySection, PageHistoryDescriptionVisuals, VisualsSection};

// Re-export Page 5 components
pub use page5::PageGrimoire;

// Re-export Page 6 components
pub use page6::PageNotes;

// Re-export Sheet orchestration components
pub use sheet::{ActiveDotOriginContext, CostBreakdownModal, SaveStatus, SheetPageTab, SheetTabs, SheetTopBar};
