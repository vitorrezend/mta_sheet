use leptos::*;

#[component]
pub fn Sheet(children: Children) -> impl IntoView {
    view! {
        <main class="sheet-container">
            {children()}
        </main>
    }
}
