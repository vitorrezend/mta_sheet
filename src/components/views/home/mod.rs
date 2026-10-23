use leptos::*;
use leptos_router::*;
use crate::components::{Callback, PatchNotesModal};
use crate::components::common::patch_notes_data::CURRENT_VERSION;
use crate::i18n::{tr, Language};
use crate::state::CharacterSummary;
use crate::AuthContext;

pub mod types;
pub mod store;
pub mod folder_card;
pub mod sheet_card;
pub mod folder_modals;
pub mod folder_acl_modal;
pub mod sheet_modals;
pub mod drive_views;
pub mod create_section;
pub mod showcase;

pub use types::*;
pub use showcase::*;
use create_section::CreateSheetSection;
use drive_views::{FolderView, RootDriveView};
use folder_acl_modal::FolderAclModal;
use folder_modals::{DeleteFolderModal, FolderFormModal, MoveFolderModal, MoveSheetModal};
use sheet_card::CharacterGrid;
use sheet_modals::DeleteSheetModal;
use store::{use_lang, HomeStore};

#[component]
pub fn Home() -> impl IntoView {
    let lang = use_lang();
    let auth = use_context::<AuthContext>();
    let user = auth.map(|a| a.user).unwrap_or_else(|| Signal::derive(|| None));
    let is_logged_in = Signal::derive(move || user.get().is_some());
    let home = HomeStore::new(is_logged_in);
    provide_context(home);
    let home_tab = home.tab;
    let (show_patch_notes, set_show_patch_notes) = create_signal(false);

    view! {
        <div class="home-page">
            <div class="home-container">
            {move || if user.get().is_none() {
                Some(view! {
                    <HomeShowcase lang=lang is_logged_in=is_logged_in />
                })
            } else {
                None
            }}

            {move || home.error.get().map(|msg| view! {
                <div class="alert-box alert-error">
                    <span>{msg}</span>
                    <button class="alert-close" on:click=move |_| home.error.set(None)>"×"</button>
                </div>
            })}

            {move || match user.get() {
                Some(_) => view! { <CreateSheetSection /> }.into_view(),
                None => view! {
                    <div class="visitor-banner">
                        <div class="visitor-banner-content">
                            <span class="visitor-banner-icon">"🔮"</span>
                            <div class="visitor-banner-text">
                                <h3>{move || tr("home_visitor_title", lang.get())}</h3>
                                <p>{move || tr("home_visitor_desc", lang.get())}</p>
                            </div>
                        </div>
                        <A href="/login" class="visitor-login-btn">{move || tr("home_visitor_btn", lang.get())}</A>
                    </div>
                }.into_view(),
            }}

            <div class="home-tabs-container">
                {move || if user.get().is_some() {
                    Some(view! {
                        <button
                            class="home-tab-btn"
                            class:active=move || home_tab.get() == "my_sheets"
                            on:click=move |_| home_tab.set("my_sheets")
                        >
                            {move || tr("home_tab_my_sheets", lang.get())}
                        </button>
                    })
                } else {
                    None
                }}
                <button
                    class="home-tab-btn"
                    class:active=move || home_tab.get() == "public_sheets" || (user.get().is_none() && home_tab.get() != "showcase")
                    on:click=move |_| home_tab.set("public_sheets")
                >
                    {move || tr("home_tab_public_sheets", lang.get())}
                </button>
            </div>

            <section class="list-section">
                {move || {
                    let current_lang = lang.get();
                    match home_tab.get() {
                        "showcase" => view! { <HomeShowcase lang=lang is_logged_in=is_logged_in /> }.into_view(),
                        "public_sheets" => public_list(home, current_lang),
                        _ if user.get().is_none() => public_list(home, current_lang),
                        _ => my_drive(home, current_lang),
                    }
                }}
            </section>

            <DeleteSheetModal />
            <FolderFormModal />
            <DeleteFolderModal />
            <MoveSheetModal />
            <MoveFolderModal />
            <FolderAclModal />

            <footer class="home-footer">
                <span class="home-footer-text">{move || tr("home_footer_copyright", lang.get())}</span>
                <A href="/about" class="home-footer-about-link">
                    {move || tr("about_collab", lang.get())}
                </A>
                <button
                    type="button"
                    class="version-pill-badge"
                    on:click=move |_| set_show_patch_notes.set(true)
                    title="Ver Notas de Atualização & Versões"
                >
                    <span class="version-pill-sparkle">"✨"</span>
                    <span>{format!("v{}", CURRENT_VERSION)}</span>
                    <span>{move || format!(" {}", tr("home_footer_patch_notes", lang.get()))}</span>
                </button>
            </footer>
            </div>
            <PatchNotesModal
                is_open=show_patch_notes
                on_close=Callback::new(move |_| set_show_patch_notes.set(false))
            />
        </div>
    }
}

fn load_error(prefix: &str, e: &ServerFnError) -> View {
    view! {
        <div class="alert-box alert-error">
            <p>{prefix.to_string()} {e.to_string()}</p>
        </div>
    }
    .into_view()
}

/// Aba de fichas públicas (também o que visitantes veem).
fn public_list(home: HomeStore, lang: Language) -> View {
    match home.public_sheets.get() {
        None => view! { <p class="loading-msg">{tr("home_loading_pub", lang)}</p> }.into_view(),
        Some(Ok(sheets)) if sheets.is_empty() => view! { <p class="empty-msg">{tr("home_empty_pub", lang)}</p> }.into_view(),
        Some(Ok(sheets)) => view! { <CharacterGrid sheets=sheets lang=lang /> }.into_view(),
        Some(Err(e)) => load_error("Erro ao carregar fichas públicas: ", &e),
    }
}

/// Minhas fichas: a raiz do drive ou a pasta aberta.
fn my_drive(home: HomeStore, lang: Language) -> View {
    let sheets: Vec<CharacterSummary> = match home.sheets.get() {
        None => return view! { <p class="loading-msg">{tr("home_loading_my", lang)}</p> }.into_view(),
        Some(Err(e)) => return load_error("Erro ao carregar fichas: ", &e),
        Some(Ok(sheets)) => sheets,
    };
    let folders = home.folder_list();
    match home.selected_folder.get() {
        Some(fid) => view! { <FolderView fid=fid folders=folders sheets=sheets lang=lang /> }.into_view(),
        None => {
            let show_all = home.root_view_mode.get() == "all";
            view! { <RootDriveView folders=folders sheets=sheets show_all=show_all lang=lang /> }.into_view()
        }
    }
}
