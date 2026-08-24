#![allow(unexpected_cfgs)]

mod components;
pub mod state;
pub mod database;
pub mod auth;
pub mod rooms;
pub mod logging;

#[cfg(test)]
mod compliance_tests;

use leptos::*;
use leptos_router::*;
use leptos_meta::*;

#[derive(Clone, Copy)]
pub struct AuthContext {
    pub user: ReadSignal<Option<crate::auth::UserInfo>>,
    pub set_user: WriteSignal<Option<crate::auth::UserInfo>>,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let user_resource = create_local_resource(|| (), |_| async move { crate::auth::get_current_user().await });
    let (user, set_user) = create_signal(Option::<crate::auth::UserInfo>::None);
    provide_context(AuthContext { user, set_user });

    create_effect(move |_| {
        if let Some(Ok(u)) = user_resource.get() {
            set_user.set(u);
        }
    });

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

    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/mta_sheet.css"/>
        <Title text="MTA Sheet - RPG Character Sheet"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Script>{mobile_scale_script}</Script>

        <Router>
            <main>
                <Routes>
                    <Route path="/" view=crate::components::Home />
                    <Route path="/sheet/:id" view=crate::components::CharacterSheet />
                    <Route path="/login" view=crate::components::AuthPage />
                    <Route path="/rooms" view=crate::components::RoomsPage />
                    <Route path="/room/:id" view=crate::components::RoomView />
                    <Route path="/logs" view=crate::components::LogsPage />
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
