use leptos::*;
use super::header::GodsAndMonstersHeader;
use super::attributes::GodsAndMonstersAttributes;
use super::abilities::GodsAndMonstersAbilities;
use super::advantages::GodsAndMonstersAdvantages;

#[component]
pub fn GodsAndMonstersPage1() -> impl IntoView {
    view! {
        <div class="sheet-page-layout page-1-layout gods-sheet-page gods-page-1">
            <GodsAndMonstersHeader />
            <GodsAndMonstersAttributes />
            <GodsAndMonstersAbilities />
            <GodsAndMonstersAdvantages />
        </div>
    }
}
