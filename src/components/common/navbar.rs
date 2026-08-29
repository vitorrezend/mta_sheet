use leptos::*;
use leptos_router::*;
use crate::auth::logout;
use crate::AuthContext;
use crate::components::common::patch_notes_data::CURRENT_VERSION;
use crate::components::common::patch_notes_modal::PatchNotesModal;

#[component]
pub fn Navbar() -> impl IntoView {
    let auth = use_context::<AuthContext>();
    let user = auth.map(|a| a.user).unwrap_or_else(|| Signal::derive(|| None));
    let (show_patch_notes, set_show_patch_notes) = create_signal(false);

    let on_logout = move |_| {
        spawn_local(async move {
            let _ = logout().await;
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(window) = web_sys::window() {
                    let _ = window.location().set_href("/");
                }
            }
        });
    };

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    view! {
        <nav class="main-navbar">
            <div class="navbar-container">
                <div class="navbar-brand">
                    <A href="/" class="brand-link">
                        <span class="brand-icon">"🔮"</span>
                        <span class="brand-title">"MTA Sheet"</span>
                    </A>
                    <button
                        type="button"
                        class="version-pill-badge"
                        on:click=move |_| set_show_patch_notes.set(true)
                        title="Ver Notas de Atualização & Versões"
                    >
                        <span class="version-pill-sparkle">"✨"</span>
                        <span>{format!("v{}", CURRENT_VERSION)}</span>
                    </button>
                </div>

                <div class="navbar-links">
                    <A href="/" class="nav-link" exact=true>{move || crate::i18n::tr("character_sheets", lang())}</A>
                    <A href="/rooms" class="nav-link">{move || crate::i18n::tr("game_rooms", lang())}</A>
                    {move || user.get().and_then(|u| {
                        if u.is_admin {
                            Some(view! { <A href="/logs" class="nav-link">{move || crate::i18n::tr("logs", lang())}</A> })
                        } else {
                            None
                        }
                    })}
                </div>

                <div class="navbar-auth">
                    <button
                        type="button"
                        class="lang-toggle-btn"
                        on:click=move |_| {
                            if let Some(ctx) = lang_ctx {
                                ctx.toggle();
                            }
                        }
                        title=move || match lang() {
                            crate::i18n::Language::PtBr => "Idioma: Português (Clique para mudar para English)",
                            crate::i18n::Language::EnUs => "Language: English (Click to switch to Português)",
                        }
                    >
                        {move || match lang() {
                            crate::i18n::Language::PtBr => "🇧🇷 PT",
                            crate::i18n::Language::EnUs => "🇺🇸 EN",
                        }}
                    </button>

                    {move || match user.get() {
                        Some(u) => view! {
                            <div class="user-pill">
                                <span class="user-greeting">"🧙 " {u.username}</span>
                                <button class="logout-btn" on:click=on_logout title="Sair da conta">{move || crate::i18n::tr("logout", lang())}</button>
                            </div>
                        }.into_view(),
                        None => view! {
                            <A href="/login" class="login-link">"Entrar / Cadastrar"</A>
                        }.into_view(),
                    }}
                </div>
            </div>
        </nav>
        <PatchNotesModal
            is_open=show_patch_notes
            on_close=crate::components::common::SafeCallback::new(move |_| set_show_patch_notes.set(false))
        />
    }
}

