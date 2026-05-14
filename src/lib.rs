mod components;
mod state;
pub mod database;

use leptos::*;
use leptos_router::*;
use crate::components::Sheet; // We will refactor this later

#[component]
pub fn App() -> impl IntoView {
    use crate::components::Home;

    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/sheet/:id" view=crate::components::CharacterSheet />
                </Routes>
            </main>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
