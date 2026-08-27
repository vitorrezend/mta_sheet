#![allow(unexpected_cfgs)]

pub mod components;
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
    pub user: Signal<Option<crate::auth::UserInfo>>,
    pub refetch: Callback<()>,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let user_resource = create_local_resource(|| (), |_| async move { crate::auth::get_current_user().await });
    let user = Signal::derive(move || {
        user_resource.get().and_then(|r| r.ok()).flatten()
    });
    let refetch = Callback::new(move |_| {
        user_resource.refetch();
    });
    provide_context(AuthContext { user, refetch });

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        let apply_mobile_scale = move || {
            if let Some(window) = web_sys::window() {
                if let Ok(width_val) = window.inner_width() {
                    if let Some(width) = width_val.as_f64() {
                        let scale = (width / 793.0).min(1.0);
                        if let Some(doc) = window.document() {
                            if let Some(doc_elem) = doc.document_element() {
                                if let Ok(html_elem) = doc_elem.dyn_into::<web_sys::HtmlElement>() {
                                    let style = html_elem.style();
                                    let _ = style.set_property("--mobile-scale", &scale.to_string());
                                    let _ = style.set_property("--sheet-visual-height", &format!("{}px", scale * 1122.0));
                                }
                            }
                        }
                    }
                }
            }
        };

        apply_mobile_scale();
        let _ = window_event_listener(ev::resize, move |_| {
            apply_mobile_scale();
        });
    }

    view! {
        <Stylesheet id="leptos" href="/pkg/mta_sheet.css"/>
        <Title text="MTA Character Manager"/>
        <Router>
            <Routes>
                <Route path="/" view=crate::components::Home />
                <Route path="/sheet/:id" view=crate::components::CharacterSheet />
                <Route path="/login" view=crate::components::AuthPage />
                <Route path="/rooms" view=crate::components::RoomsPage />
                <Route path="/room/:id" view=crate::components::RoomView />
                <Route path="/logs" view=crate::components::LogsPage />
            </Routes>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
