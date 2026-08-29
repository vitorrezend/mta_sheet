pub mod cost_breakdown_modal;
pub mod sheet_top_bar;
pub mod sheet_tabs;
pub mod quiz_modal;
pub mod pdf_export_modal;

pub use cost_breakdown_modal::CostBreakdownModal;
pub use sheet_top_bar::{SheetTopBar, SaveStatus};
pub use sheet_tabs::{SheetTabs, SheetPageTab};
pub use quiz_modal::QuizModal;
pub use pdf_export_modal::PdfExportModal;

use leptos::*;
use crate::state::DotOrigin;

#[derive(Clone, Copy)]
pub struct ActiveDotOriginContext {
    pub origin: ReadSignal<DotOrigin>,
    #[allow(dead_code)]
    pub set_origin: WriteSignal<DotOrigin>,
}
