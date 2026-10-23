pub mod cost_breakdown_modal;
pub mod sheet_top_bar;
pub mod sheet_tabs;
pub mod quiz_modal;
pub mod pdf_export_modal;
pub mod sheet_share_modal;
pub mod xp_ledger_modal;
pub mod dice_roller;

pub use cost_breakdown_modal::CostBreakdownModal;
pub use sheet_top_bar::{SaveStatus, SheetTopBar, SheetTopBarActions, SheetTopBarState};
pub use sheet_tabs::{SheetTabs, SheetPageTab};
pub use quiz_modal::QuizModal;
pub use pdf_export_modal::PdfExportModal;
pub use sheet_share_modal::SheetShareModal;
pub use xp_ledger_modal::XpLedgerModal;
pub use dice_roller::SheetDiceRoller;

use leptos::*;
use crate::state::DotOrigin;

#[derive(Clone, Copy)]
pub struct ActiveDotOriginContext {
    pub origin: ReadSignal<DotOrigin>,
    #[allow(dead_code)]
    pub set_origin: WriteSignal<DotOrigin>,
}
