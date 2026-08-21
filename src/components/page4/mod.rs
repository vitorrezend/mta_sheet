use leptos::*;

mod history_section;
mod description_section;
mod visuals_section;

pub use history_section::*;
pub use description_section::*;
pub use visuals_section::*;

#[component]
pub fn PageHistoryDescriptionVisuals() -> impl IntoView {
    view! {
        <div class="sheet-page-content page-history-visuals-content">
            <HistorySection />
            <DescriptionSection />
            <VisualsSection />
        </div>
    }
}
