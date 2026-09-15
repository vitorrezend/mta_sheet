use leptos::*;
use super::SheetFrame;

#[component]
pub fn Sheet(children: Children) -> impl IntoView {
    view! {
        <main class="sheet-container">
            <SheetFrame />
            {children()}
        </main>
    }
}
