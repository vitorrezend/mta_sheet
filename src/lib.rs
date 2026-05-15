mod components;
mod state;
pub mod database;

use leptos::*;
use leptos_router::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    // Mobile Scaling Script from index.html
    let mobile_scale_script = "
        var A4_PX = 793;
        function applyMobileScale() {
            var scale = Math.min(1, window.innerWidth / A4_PX);
            document.documentElement.style.setProperty('--mobile-scale', scale);
            document.documentElement.style.setProperty(
                '--sheet-visual-height',
                (scale * 1122) + 'px'
            );
        }
        applyMobileScale();
        window.addEventListener('resize', applyMobileScale);
    ";

    view! {
        <Stylesheet id="leptos" href="/pkg/mta_sheet.css"/>
        <Title text="MTA Sheet - RPG Character Sheet"/>
        <Script>{mobile_scale_script}</Script>

        <Router>
            <main>
                <Routes>
                    <Route path="/" view=crate::components::Home />
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
