use leptos::*;

mod expanded_backgrounds;
mod possessions;
mod chantry;

pub use expanded_backgrounds::*;
pub use possessions::*;
pub use chantry::*;

#[component]
pub fn PageExpandedBackgroundsPossessions() -> impl IntoView {
    view! {
        <div class="sheet-page-content page-expanded-content">
            <ExpandedBackgrounds />
            <Possessions />
            <Chantry />
        </div>
    }
}
