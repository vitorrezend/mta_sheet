//! Criação de ficha na tela inicial: tipo, nome, pasta de destino e importação de JSON.

use leptos::*;
use wasm_bindgen::JsCast;
use crate::i18n::tr;
use crate::state::{create_sheet, import_sheet};
use super::store::{use_home, use_lang};

/// Lê o arquivo escolhido, valida o JSON e cria a ficha importada.
fn import_file(ev: ev::Event, on_done: impl Fn(Result<String, String>) + 'static) -> bool {
    let input = event_target::<web_sys::HtmlInputElement>(&ev);
    let file = input.files().and_then(|files| files.get(0));
    input.set_value("");
    let (Some(file), Ok(reader)) = (file, web_sys::FileReader::new()) else { return false };
    let reader_clone = reader.clone();
    let on_done = std::rc::Rc::new(on_done);
    let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::ProgressEvent| {
        let Some(text) = reader_clone.result().ok().and_then(|r| r.as_string()) else { return };
        let on_done = on_done.clone();
        match crate::components::common::parse_and_sanitize_sheet_json(&text) {
            Ok(sanitized) => spawn_local(async move {
                on_done(import_sheet(sanitized).await.map_err(|e| format!("Erro ao importar ficha: {}", e)));
            }),
            Err(err) => on_done(Err(format!("Arquivo JSON inválido: {}", err))),
        }
    }) as Box<dyn FnMut(_)>);
    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
    onload.forget();
    reader.read_as_text(&file).is_ok()
}

#[component]
pub fn CreateSheetSection() -> impl IntoView {
    let home = use_home();
    let lang = use_lang();
    let name = create_rw_signal(String::new());
    let sheet_type = create_rw_signal("mage".to_string());
    let is_creating = create_rw_signal(false);
    let is_importing = create_rw_signal(false);
    let is_open = create_rw_signal(false);
    let import_ref = create_node_ref::<html::Input>();
    let busy = move || is_creating.get() || is_importing.get();
    let is_gm = move || sheet_type.get() == "gods_and_monsters";

    let on_create = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if is_creating.get_untracked() {
            return;
        }
        let typed = name.get_untracked().trim().to_string();
        let kind = sheet_type.get_untracked();
        let default_name = if kind == "gods_and_monsters" { "Novo Familiar / Monstro" } else { "Novo Mago" };
        let final_name = if typed.is_empty() { default_name.to_string() } else { typed };
        let folder = home.selected_folder.get_untracked();
        is_creating.set(true);
        home.error.set(None);
        spawn_local(async move {
            match create_sheet(final_name, Some(kind), folder).await {
                Ok(id) => leptos_router::use_navigate()(&format!("/sheet/{}", id), Default::default()),
                Err(e) => {
                    log::error!("Error creating sheet: {:?}", e);
                    home.error.set(Some(format!("Erro ao criar ficha: {}", e)));
                    is_creating.set(false);
                }
            }
        });
    };

    let navigate = leptos_router::use_navigate();
    let on_import = move |ev: ev::Event| {
        let nav = navigate.clone();
        let started = import_file(ev, move |result| match result {
            Ok(id) => nav(&format!("/sheet/{}", id), Default::default()),
            Err(msg) => {
                is_importing.set(false);
                home.error.set(Some(msg));
            }
        });
        if started {
            is_importing.set(true);
        }
    };

    // Pasta aberta: a nova ficha será criada dentro dela
    let target_folder = move || {
        let fid = home.selected_folder.get()?;
        home.folder_list().into_iter().find(|f| f.id == fid)
    };

    view! {
        <section class="create-section">
            <div class="create-section-header" on:click=move |_| is_open.update(|v| *v = !*v)>
                <h2>{move || tr("home_create_title", lang.get())}</h2>
                <button
                    type="button"
                    class="mobile-create-toggle-btn"
                    class:active=move || is_open.get()
                    title=move || if is_open.get() { "Recolher criação de ficha" } else { "Expandir criação de ficha" }
                >
                    <span class="toggle-text">
                        {move || if is_open.get() { "Recolher ▲" } else { "➕ Nova Ficha ▼" }}
                    </span>
                </button>
            </div>

            <div class="create-section-body" class:mobile-open=move || is_open.get()>
                <div class="sheet-type-selector">
                    <button type="button" class="type-pill-btn" class:active=move || !is_gm() on:click=move |_| sheet_type.set("mage".to_string())>
                        {move || tr("home_type_mage", lang.get())}
                    </button>
                    <button type="button" class="type-pill-btn" class:active=is_gm on:click=move |_| sheet_type.set("gods_and_monsters".to_string())>
                        {move || tr("home_type_gm", lang.get())}
                    </button>
                </div>

                {move || target_folder().map(|f| view! {
                    <div class="drive-creating-badge">
                        <span class="drive-creating-icon">"📁"</span>
                        <span class="drive-creating-label">{tr("drive_creating_in_folder", lang.get())}</span>
                        <span class="drive-creating-target">{f.icon} " " {f.name}</span>
                        <button
                            type="button"
                            class="drive-creating-clear"
                            title=tr("drive_move_to_root", lang.get())
                            on:click=move |_| home.selected_folder.set(None)
                        >
                            "✕"
                        </button>
                    </div>
                })}

                <form on:submit=on_create class="create-form">
                    <input
                        type="text"
                        placeholder=move || tr(if is_gm() { "home_name_ph_gm" } else { "home_name_ph_mage" }, lang.get())
                        on:input=move |ev| name.set(event_target_value(&ev))
                        prop:value=name
                        class="name-input"
                        disabled=is_creating
                    />

                    <div class="create-actions-group">
                        <input type="file" accept=".json,application/json" node_ref=import_ref style="display: none;" on:change=on_import />
                        <button type="submit" class="create-btn" disabled=busy>
                            {move || tr(if is_creating.get() { "home_btn_creating" } else { "home_btn_create" }, lang.get())}
                        </button>
                        <button
                            type="button"
                            class="import-json-home-btn"
                            disabled=busy
                            on:click=move |_| {
                                if let Some(input) = import_ref.get() {
                                    input.click();
                                }
                            }
                            title=move || tr("home_import_tooltip", lang.get())
                        >
                            {move || tr(if is_importing.get() { "home_btn_importing" } else { "home_btn_import" }, lang.get())}
                        </button>
                    </div>
                </form>
            </div>
        </section>
    }
}
