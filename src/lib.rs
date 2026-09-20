#![allow(unexpected_cfgs)]

pub mod components;
pub mod state;
pub mod database;
pub mod auth;
pub mod rooms;
pub mod logging;
pub mod i18n;
pub mod rules;
pub mod compendium;
pub mod repositories;
pub mod settings;
#[cfg(feature = "ssr")]
pub mod server;

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

    let (lang, set_lang) = create_signal(crate::i18n::detect_browser_language());
    let lang_ctx = crate::i18n::LanguageContext::new(lang, set_lang);
    provide_context(lang_ctx);

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
        <Html lang=move || match lang.get() {
            crate::i18n::Language::PtBr => "pt-BR",
            crate::i18n::Language::EnUs => "en-US",
        }/>
        <Stylesheet id="leptos" href="/pkg/mta_sheet.css"/>
        <Title text="MTA Sheet — Ficha de RPG Mago: A Ascensão (M20) & Gods and Monsters"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="description" content="Gerenciador completo e ficha interativa para Mago: A Ascensão (20th Anniversary Edition - M20) e Gods & Monsters. Automação de Pontos de Bônus e XP, Grimório de Rotes, Compêndio de Dô e Salas de Jogo com HUD da Cabala em tempo real."/>
        <Meta name="keywords" content="Mago A Ascensao, Mage The Ascension, M20, World of Darkness, White Wolf, RPG de Mesa, Ficha de RPG, Gods and Monsters, Grimorio, Do Akashico, Storyteller, Storytelling"/>
        <Meta name="author" content="MTA Sheet Community"/>
        <Meta name="robots" content="index, follow"/>
        <Meta name="theme-color" content="#1e1b4b"/>

        // Preload de recursos críticos para otimização de FCP (First Contentful Paint)
        <Link rel="preload" href="/pkg/mta_sheet.css" as_="style"/>
        <Link rel="preload" href="/pkg/mta_sheet.wasm" as_="fetch" type_="application/wasm" crossorigin="anonymous"/>
        <Link rel="preload" href="/fonts/cinzel-latin.woff2" as_="font" type_="font/woff2" crossorigin="anonymous"/>

        <Meta property="og:type" content="website"/>
        <Meta property="og:title" content="MTA Sheet — Mago: A Ascensão (M20) & Gods and Monsters"/>
        <Meta property="og:description" content="Crie, automatize e jogue com fichas canônicas de M20 e Gods & Monsters, compêndio completo de Dô e Armas, e salas multijogador em tempo real."/>
        <Meta property="og:site_name" content="MTA Sheet"/>
        <Meta property="og:url" content="/"/>
        <Meta property="og:image" content="/banner_og.jpg"/>
        <Meta property="og:image:secure_url" content="/banner_og.jpg"/>
        <Meta property="og:image:type" content="image/jpeg"/>
        <Meta property="og:image:width" content="1200"/>
        <Meta property="og:image:height" content="675"/>
        <Meta property="og:image:alt" content="MTA Sheet — World of Darkness M20 & Gods and Monsters"/>

        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content="MTA Sheet — Ficha Interativa M20 & Gods and Monsters"/>
        <Meta name="twitter:description" content="Gerenciador completo de fichas para Mago: A Ascensão 20 Anos e Gods & Monsters."/>
        <Meta name="twitter:image" content="/banner_og.jpg"/>

        <Link rel="icon" type_="image/svg+xml" href="/favicon.svg"/>
        <Link rel="canonical" href="/"/>

        <Script type_="application/ld+json">
            {r#"{
  "@context": "https://schema.org",
  "@type": "WebApplication",
  "name": "MTA Sheet",
  "url": "/",
  "description": "Gerenciador e automação de fichas de personagens para Mago: A Ascensão (M20) e Gods & Monsters.",
  "applicationCategory": "GameApplication",
  "operatingSystem": "All",
  "offers": {
    "@type": "Offer",
    "price": "0",
    "priceCurrency": "BRL"
  },
  "featureList": [
    "Ficha interativa M20 de 6 páginas",
    "Suplemento Gods & Monsters",
    "Automação de Pontos de Bônus e XP",
    "Compêndio canônico de Dô e Armas",
    "Salas de Jogo e HUD da Cabala em tempo real"
  ]
}"#}
        </Script>
        <Router>
            <Routes>
                <Route path="" view=AppLayout>
                    <Route path="" view=crate::components::Home />
                    <Route path="feed" view=crate::components::FeedPage />
                    <Route path="profile" view=crate::components::ProfilePage />
                    <Route path="user/:username" view=crate::components::ProfilePage />
                    <Route path="rooms" view=crate::components::RoomsPage />
                    <Route path="room/:id" view=crate::components::RoomView />
                    <Route path="logs" view=crate::components::LogsPage />
                    <Route path="login" view=crate::components::AuthPage />
                    <Route path="about" view=crate::components::AboutPage />
                    <Route path="colabore" view=crate::components::AboutPage />
                </Route>
                <Route path="/sheet/:id" view=crate::components::CharacterSheet />
            </Routes>
        </Router>
    }
}

#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <div class="app-shell">
            <crate::components::Navbar />
            <main class="app-main-content">
                <Outlet />
            </main>
        </div>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
