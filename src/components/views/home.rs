use wasm_bindgen::JsCast;
use leptos::*;
use leptos_router::*;
use crate::state::{get_sheets, get_public_sheets, set_sheet_visibility, create_sheet, delete_sheet, import_sheet, CharacterSummary};
use crate::components::{Callback, Navbar, PatchNotesModal};
use crate::components::common::patch_notes_data::CURRENT_VERSION;
use crate::AuthContext;


#[component]
pub fn Home() -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let auth = use_context::<AuthContext>();
    let user = auth.map(|a| a.user).unwrap_or_else(|| Signal::derive(|| None));

    let (home_tab, set_home_tab) = create_signal("my_sheets");
    let sheets = create_local_resource(|| (), |_| async move { get_sheets().await });
    let public_sheets = create_local_resource(
        move || home_tab.get(),
        |tab| async move {
            if tab == "public_sheets" {
                get_public_sheets().await
            } else {
                Ok(Vec::new())
            }
        }
    );

    let (name, set_name) = create_signal(String::new());
    let (error_msg, set_error_msg) = create_signal(Option::<String>::None);
    let (sheet_to_delete, set_sheet_to_delete) = create_signal(Option::<CharacterSummary>::None);
    let (is_creating, set_is_creating) = create_signal(false);
    let (selected_sheet_type, set_selected_sheet_type) = create_signal("mage".to_string());
    let (show_patch_notes, set_show_patch_notes) = create_signal(false);
    let (is_importing, set_is_importing) = create_signal(false);
    let import_home_input_ref = create_node_ref::<html::Input>();
    let navigate = use_navigate();

    let on_home_file_import = Callback::new(move |ev: ev::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Some(file_list) = target.files() {
            if let Some(file) = file_list.get(0) {
                let file_reader = web_sys::FileReader::new().ok();
                if let Some(fr) = file_reader {
                    let fr_clone = fr.clone();
                    let nav = navigate.clone();
                    set_is_importing.set(true);
                    let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::ProgressEvent| {
                        if let Ok(result) = fr_clone.result() {
                            if let Some(text) = result.as_string() {
                                match crate::components::common::parse_and_sanitize_sheet_json(&text) {
                                    Ok(sanitized) => {
                                        let nav_clone = nav.clone();
                                        spawn_local(async move {
                                            match import_sheet(sanitized).await {
                                                Ok(new_id) => {
                                                    nav_clone(&format!("/sheet/{}", new_id), Default::default());
                                                }
                                                Err(e) => {
                                                    set_is_importing.set(false);
                                                    set_error_msg.set(Some(format!("Erro ao importar ficha: {}", e)));
                                                }
                                            }
                                        });
                                    }
                                    Err(err) => {
                                        set_is_importing.set(false);
                                        set_error_msg.set(Some(format!("Arquivo JSON inválido: {}", err)));
                                    }
                                }
                            }
                        }
                    }) as Box<dyn FnMut(_)>);

                    fr.set_onload(Some(onload.as_ref().unchecked_ref()));
                    onload.forget();
                    let _ = fr.read_as_text(&file);
                }
            }
        }
        target.set_value("");
    });
    let on_home_import_cb = on_home_file_import.clone();

    let on_create = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if is_creating.get() { return; }
        
        let raw_name = name.get().trim().to_string();
        let s_type = selected_sheet_type.get();
        let default_name = if s_type == "gods_and_monsters" { "Novo Familiar / Monstro" } else { "Novo Mago" };
        let final_name = if raw_name.is_empty() { default_name.to_string() } else { raw_name };
        
        set_is_creating.set(true);
        set_error_msg.set(None);
        
        let type_param = Some(s_type);
        spawn_local(async move {
            match create_sheet(final_name, type_param).await {
                Ok(id) => {
                    use_navigate()(&format!("/sheet/{}", id), Default::default());
                }
                Err(e) => {
                    log::error!("Error creating sheet: {:?}", e);
                    set_error_msg.set(Some(format!("Erro ao criar ficha: {}", e)));
                    set_is_creating.set(false);
                }
            }
        });
    };

    let toggle_privacy = move |sheet_id: String, current_public: bool| {
        spawn_local(async move {
            match set_sheet_visibility(sheet_id, !current_public).await {
                Ok(_) => {
                    sheets.refetch();
                    if home_tab.get_untracked() == "public_sheets" {
                        public_sheets.refetch();
                    }
                }
                Err(e) => {
                    set_error_msg.set(Some(format!("Erro ao alterar privacidade: {}", e)));
                }
            }
        });
    };

    let confirm_delete = move || {
        if let Some(target) = sheet_to_delete.get() {
            let id = target.id;
            set_sheet_to_delete.set(None);
            set_error_msg.set(None);
            spawn_local(async move {
                match delete_sheet(id).await {
                    Ok(_) => {
                        sheets.refetch();
                        if home_tab.get_untracked() == "public_sheets" {
                            public_sheets.refetch();
                        }
                    }
                    Err(e) => {
                        log::error!("Error deleting sheet: {:?}", e);
                        set_error_msg.set(Some(format!("Erro ao excluir ficha: {}", e)));
                    }
                }
            });
        }
    };

    let cancel_delete = move |_| {
        set_sheet_to_delete.set(None);
    };

    view! {
        <div class="home-page">
            <Navbar />
            <div class="home-container">
                <header class="home-header">
                <h1>{move || crate::i18n::tr("home_header_title", lang())}</h1>
                <p>{move || crate::i18n::tr("home_header_subtitle", lang())}</p>
            </header>

            {move || error_msg.get().map(|msg| view! {
                <div class="alert-box alert-error">
                    <span>{msg}</span>
                    <button class="alert-close" on:click=move |_| set_error_msg.set(None)>"×"</button>
                </div>
            })}

            {
                let on_home_import_cb = on_home_import_cb.clone();
                move || match user.get() {
                Some(_) => {
                    let on_import = on_home_import_cb.clone();
                    view! {
                        <section class="create-section">
                            <h2>{move || crate::i18n::tr("home_create_title", lang())}</h2>
                            
                            <div class="sheet-type-selector">
                                <button
                                    type="button"
                                    class="type-pill-btn"
                                    class:active=move || selected_sheet_type.get() == "mage"
                                    on:click=move |_| set_selected_sheet_type.set("mage".to_string())
                                >
                                    {move || crate::i18n::tr("home_type_mage", lang())}
                                </button>
                                <button
                                    type="button"
                                    class="type-pill-btn"
                                    class:active=move || selected_sheet_type.get() == "gods_and_monsters"
                                    on:click=move |_| set_selected_sheet_type.set("gods_and_monsters".to_string())
                                >
                                    {move || crate::i18n::tr("home_type_gm", lang())}
                                </button>
                            </div>

                            <form on:submit=move |ev| on_create(ev) class="create-form">
                                <input
                                    type="text"
                                    placeholder=move || if selected_sheet_type.get() == "gods_and_monsters" {
                                        crate::i18n::tr("home_name_ph_gm", lang())
                                    } else {
                                        crate::i18n::tr("home_name_ph_mage", lang())
                                    }
                                    on:input=move |ev| set_name.set(event_target_value(&ev))
                                    prop:value=name
                                    class="name-input"
                                    disabled=is_creating
                                />

                                <div class="create-actions-group">
                                    <input 
                                        type="file" 
                                        accept=".json,application/json" 
                                        node_ref=import_home_input_ref 
                                        style="display: none;" 
                                        on:change={
                                             let cb = on_import.clone();
                                            move |ev| cb.call(ev)
                                        }
                                    />
                                    <button type="submit" class="create-btn" disabled=move || is_creating.get() || is_importing.get()>
                                        {move || if is_creating.get() { crate::i18n::tr("home_btn_creating", lang()) } else { crate::i18n::tr("home_btn_create", lang()) }}
                                    </button>
                                    <button 
                                        type="button" 
                                        class="import-json-home-btn" 
                                        disabled=move || is_creating.get() || is_importing.get()
                                        on:click=move |_| {
                                            if let Some(input) = import_home_input_ref.get() {
                                                input.click();
                                            }
                                        }
                                        title=move || crate::i18n::tr("home_import_tooltip", lang())
                                    >
                                        {move || if is_importing.get() { crate::i18n::tr("home_btn_importing", lang()) } else { crate::i18n::tr("home_btn_import", lang()) }}
                                    </button>
                                </div>
                            </form>
                        </section>
                    }.into_view()
                },
                None => view! {
                    <div class="visitor-banner">
                        <div class="visitor-banner-content">
                            <span class="visitor-banner-icon">"🔮"</span>
                            <div class="visitor-banner-text">
                                <h3>{move || crate::i18n::tr("home_visitor_title", lang())}</h3>
                                <p>{move || crate::i18n::tr("home_visitor_desc", lang())}</p>
                            </div>
                        </div>
                        <A href="/login" class="visitor-login-btn">{move || crate::i18n::tr("home_visitor_btn", lang())}</A>
                    </div>
                }.into_view(),
            }}

            <div class="home-tabs-container">
                {move || if user.get().is_some() {
                    Some(view! {
                        <button
                            class="home-tab-btn"
                            class:active=move || home_tab.get() == "my_sheets"
                            on:click=move |_| set_home_tab.set("my_sheets")
                        >
                            {move || crate::i18n::tr("home_tab_my_sheets", lang())}
                        </button>
                    })
                } else {
                    None
                }}
                <button
                    class="home-tab-btn"
                    class:active=move || home_tab.get() == "public_sheets" || user.get().is_none()
                    on:click=move |_| set_home_tab.set("public_sheets")
                >
                    {move || crate::i18n::tr("home_tab_public_sheets", lang())}
                </button>
            </div>

            <section class="list-section">
                {move || {
                    let current_lang = lang();
                    match home_tab.get() {
                        "public_sheets" => match public_sheets.get() {
                            None => view! { <p class="loading-msg">{crate::i18n::tr("home_loading_pub", current_lang)}</p> }.into_view(),
                            Some(Ok(data)) if data.is_empty() => view! {
                                <p class="empty-msg">{crate::i18n::tr("home_empty_pub", current_lang)}</p>
                            }.into_view(),
                            Some(Ok(data)) => render_character_grid(data, set_sheet_to_delete, toggle_privacy, current_lang),
                            Some(Err(e)) => view! {
                                <div class="alert-box alert-error">
                                    <p>"Erro ao carregar fichas públicas: " {e.to_string()}</p>
                                </div>
                            }.into_view(),
                        },
                        _ => match sheets.get() {
                            None => view! { <p class="loading-msg">{crate::i18n::tr("home_loading_my", current_lang)}</p> }.into_view(),
                            Some(Ok(data)) if data.is_empty() => view! { 
                                <p class="empty-msg">{crate::i18n::tr("home_empty_my", current_lang)}</p> 
                            }.into_view(),
                            Some(Ok(data)) => render_character_grid(data, set_sheet_to_delete, toggle_privacy, current_lang),
                            Some(Err(e)) => view! { 
                                <div class="alert-box alert-error">
                                    <p>"Erro ao carregar fichas: " {e.to_string()}</p>
                                </div> 
                            }.into_view(),
                        },
                    }
                }}
            </section>

            // Delete Confirmation Modal
            {move || sheet_to_delete.get().map(|target| view! {
                <div class="modal-overlay" on:click=cancel_delete>
                    <div class="modal-card" on:click=move |ev| ev.stop_propagation()>
                        <h3 class="modal-title">{move || crate::i18n::tr("home_delete_title", lang())}</h3>
                        <p class="modal-text">
                            {move || crate::i18n::tr("home_delete_prompt", lang())}
                            <strong>{target.name}</strong>"?"
                        </p>
                        <p class="modal-subtext">{move || crate::i18n::tr("home_delete_sub", lang())}</p>
                        <div class="modal-actions">
                            <button class="modal-btn btn-cancel" on:click=cancel_delete>{move || crate::i18n::tr("home_btn_cancel", lang())}</button>
                            <button class="modal-btn btn-danger" on:click=move |_| confirm_delete()>{move || crate::i18n::tr("home_btn_confirm_delete", lang())}</button>
                        </div>
                    </div>
                </div>
            })}

            <footer class="home-footer">
                <span class="home-footer-text">{move || crate::i18n::tr("home_footer_copyright", lang())}</span>
                <button
                    type="button"
                    class="version-pill-badge"
                    on:click=move |_| set_show_patch_notes.set(true)
                    title="Ver Notas de Atualização & Versões"
                >
                    <span class="version-pill-sparkle">"✨"</span>
                    <span>{format!("v{}", CURRENT_VERSION)}</span>
                    <span>{move || format!(" {}", crate::i18n::tr("home_footer_patch_notes", lang()))}</span>
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

fn render_character_grid<F>(
    data: Vec<CharacterSummary>,
    set_sheet_to_delete: WriteSignal<Option<CharacterSummary>>,
    toggle_privacy: F,
    current_lang: crate::i18n::Language,
) -> View
where
    F: Fn(String, bool) + Copy + 'static,
{
    view! {
        <div class="character-cards-grid">
            {data.into_iter().map(|summary| {
                let summary_clone = summary.clone();
                let id = summary.id.clone();
                let id_vis = id.clone();
                let is_pub_val = summary.is_public;
                let is_owner = summary.is_owner;
                let photo = summary.photo_url.clone();
                let is_gm = summary.sheet_type == "gods_and_monsters";
                let has_photo = !photo.is_empty();
                let tradition_display = if !summary.tradition.is_empty() {
                    summary.tradition.clone()
                } else if is_gm {
                    "Familiar / Bygone".to_string()
                } else {
                    crate::i18n::tr("card_tradition_undefined", current_lang).to_string()
                };
                let essence_display = if !summary.essence.is_empty() {
                    summary.essence.clone()
                } else if is_gm {
                    "Gods & Monsters".to_string()
                } else {
                    crate::i18n::tr("card_essence_awakened", current_lang).to_string()
                };
                let arete_val = summary.arete;
                let wp_val = summary.willpower;
                let date_display = summary.updated_at.split(' ').next().unwrap_or(&summary.updated_at).to_string();
                let updated_at_full = summary.updated_at.clone();
                let id_nav = id.clone();

                view! {
                    <div
                        class="character-card"
                        on:click=move |_| use_navigate()(&format!("/sheet/{}", id_nav), Default::default())
                    >
                        <div class="card-portrait-box">
                            {if has_photo {
                                let img_style = format!("object-position: {}% {}%;", summary.photo_focus_x, summary.photo_focus_y);
                                view! {
                                    <img
                                        src=photo
                                        alt=summary.name.clone()
                                        class="card-portrait-img"
                                        style=img_style
                                    />
                                }.into_view()
                            } else {
                                view! {
                                    <div class="card-portrait-placeholder">
                                        <span class="placeholder-icon">{if is_gm { "🐉" } else { "🔮" }}</span>
                                        <span class="placeholder-tag">{if is_gm { "Gods & Monsters".to_string() } else { crate::i18n::tr("card_no_image", current_lang).to_string() }}</span>
                                    </div>
                                }.into_view()
                            }}
                            <div class="card-portrait-gradient"></div>
                            {if is_owner {
                                view! {
                                    <button
                                        class="card-delete-floating-btn"
                                        on:click=move |ev: ev::MouseEvent| {
                                            ev.stop_propagation();
                                            set_sheet_to_delete.set(Some(summary_clone.clone()));
                                        }
                                        title=crate::i18n::tr("card_delete_tooltip", current_lang)
                                    >
                                        "🗑️"
                                    </button>
                                }.into_view()
                            } else {
                                ().into_view()
                            }}
                        </div>

                        <div class="card-content">
                            <div class="card-header-info">
                                <h3 class="card-name" title=summary.name.clone()>{summary.name.clone()}</h3>
                                <div class="card-meta-tags">
                                    {if is_gm {
                                        view! {
                                            <span class="meta-tag type-badge-gm">{crate::i18n::tr("card_tag_gm", current_lang)}</span>
                                        }.into_view()
                                    } else {
                                        view! {
                                            <span class="meta-tag type-badge-mage">{crate::i18n::tr("card_tag_mage", current_lang)}</span>
                                        }.into_view()
                                    }}
                                    <span class="meta-tag tradition-tag">{tradition_display}</span>
                                    <span class="meta-tag essence-tag">{essence_display}</span>
                                    {if is_owner {
                                        view! {
                                            <button
                                                class=if is_pub_val { "meta-tag vis-tag vis-public" } else { "meta-tag vis-tag vis-private" }
                                                title=if is_pub_val { crate::i18n::tr("card_vis_public_tt", current_lang) } else { crate::i18n::tr("card_vis_private_tt", current_lang) }
                                                on:click=move |ev: ev::MouseEvent| {
                                                    ev.stop_propagation();
                                                    toggle_privacy(id_vis.clone(), is_pub_val);
                                                }
                                            >
                                                {if is_pub_val { crate::i18n::tr("card_vis_public", current_lang) } else { crate::i18n::tr("card_vis_private", current_lang) }}
                                            </button>
                                        }.into_view()
                                    } else {
                                        view! {
                                            <span class="meta-tag vis-tag vis-public">
                                                {crate::i18n::tr("card_vis_public", current_lang)}
                                            </span>
                                        }.into_view()
                                    }}
                                </div>
                            </div>

                            <div class="card-stats-preview">
                                <div class="card-stat-item">
                                    <span class="stat-label">{if is_gm { crate::i18n::tr("card_gnosis", current_lang) } else { crate::i18n::tr("card_arete", current_lang) }}</span>
                                    <div class="stat-dots arete-dots">
                                        {(1..=if is_gm { 10 } else { 5 }).map(|idx| {
                                            let filled = idx <= arete_val;
                                            view! {
                                                <span class=if filled { "stat-dot filled-arete" } else { "stat-dot empty-dot" }></span>
                                            }
                                        }).collect_view()}
                                    </div>
                                    <span class="stat-number">{arete_val}</span>
                                </div>

                                <div class="card-stat-item">
                                    <span class="stat-label">{crate::i18n::tr("card_willpower", current_lang)}</span>
                                    <div class="stat-dots wp-dots">
                                        {(1..=10).map(|idx| {
                                            let filled = idx <= wp_val;
                                            view! {
                                                <span class=if filled { "stat-dot filled-wp" } else { "stat-dot empty-dot" }></span>
                                            }
                                        }).collect_view()}
                                    </div>
                                    <span class="stat-number">{wp_val}</span>
                                </div>
                            </div>

                            {if !is_gm {
                                view! {
                                    <div class="card-spheres-preview">
                                        <div class="spheres-header-row">
                                            <span class="spheres-label">{crate::i18n::tr("card_spheres_title", current_lang)}</span>
                                        </div>
                                        <div class="spheres-9-grid">
                                            {summary.spheres.iter().map(|(sphere_name, lvl)| {
                                                let s_name = crate::i18n::tr_sphere(sphere_name, current_lang).to_string();
                                                let s_lvl = *lvl;
                                                let is_active = s_lvl > 0;
                                                let level_label = match current_lang {
                                                    crate::i18n::Language::PtBr => "nível",
                                                    crate::i18n::Language::EnUs => "level",
                                                };
                                                view! {
                                                    <div
                                                        class=if is_active { "sphere-item-active" } else { "sphere-item-inactive" }
                                                        title=format!("{}: {} {}", s_name, level_label, s_lvl)
                                                    >
                                                        <span class="sphere-mini-name">{s_name}</span>
                                                        <div class="sphere-mini-dots">
                                                            {(1..=5).map(|dot_i| {
                                                                let filled = dot_i <= s_lvl;
                                                                view! {
                                                                    <span class=if filled { "stat-dot filled-sphere" } else { "stat-dot empty-dot" }></span>
                                                                }
                                                            }).collect_view()}
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <div class="card-gm-badge-footer">
                                        <span class="gm-creature-desc">{crate::i18n::tr("card_gm_footer_desc", current_lang)}</span>
                                    </div>
                                }.into_view()
                            }}

                            <div class="card-footer">
                                <span class="card-date" title=format!("{}: {}", match current_lang { crate::i18n::Language::PtBr => "Última alteração", crate::i18n::Language::EnUs => "Last update" }, updated_at_full)>
                                    {crate::i18n::tr("card_updated", current_lang)} " " {date_display}
                                </span>
                                <span class="card-cta">{crate::i18n::tr("card_open_cta", current_lang)}</span>
                            </div>
                        </div>
                    </div>
                }
            }).collect_view()}
        </div>
    }.into_view()
}
