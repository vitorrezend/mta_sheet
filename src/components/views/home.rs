use wasm_bindgen::JsCast;
use leptos::*;
use leptos_router::*;
use crate::state::{
    get_sheets, get_public_sheets, set_sheet_visibility, create_sheet, delete_sheet, import_sheet,
    get_folders, create_folder, update_folder, delete_folder, move_sheet_to_folder, move_folder,
    get_folder_acls, grant_folder_acl, revoke_folder_acl,
    CharacterSummary, SheetFolder, FolderAclEntry,
};
use crate::components::{Callback, Navbar, PatchNotesModal};
use crate::components::common::patch_notes_data::CURRENT_VERSION;
use crate::AuthContext;

const FOLDER_ICON_PRESETS: &[&str] = &[
    "📁", "🔮", "🧙", "🧛", "🐺", "👻", "⚰️", "🎯", "🐉", "🗡️", "📜", "🎭", "🏛️", "⚡", "🏰", "💀", "👁️", "✨", "🩸", "🌿", "🏺", "🧪", "🌌",
];

#[component]
pub fn Home() -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let auth = use_context::<AuthContext>();
    let user = auth.map(|a| a.user).unwrap_or_else(|| Signal::derive(|| None));

    let (home_tab, set_home_tab) = create_signal("my_sheets");
    let sheets = create_local_resource(|| (), |_| async move { get_sheets().await });
    let folders = create_local_resource(
        move || (home_tab.get(), user.get().is_some()),
        |(tab, is_logged)| async move {
            if tab == "my_sheets" && is_logged {
                get_folders().await
            } else {
                Ok(Vec::new())
            }
        }
    );
    let (selected_folder_id, set_selected_folder_id) = create_signal(Option::<String>::None);
    let (root_view_mode, set_root_view_mode) = create_signal("unorganized");
    let (dragged_sheet_id, set_dragged_sheet_id) = create_signal(Option::<String>::None);
    let (dragged_folder_id, set_dragged_folder_id) = create_signal(Option::<String>::None);
    let (drag_over_folder_id, set_drag_over_folder_id) = create_signal(Option::<String>::None);

    let (is_create_folder_open, set_is_create_folder_open) = create_signal(false);
    let (create_folder_parent, set_create_folder_parent) = create_signal(Option::<String>::None);
    let (folder_to_edit, set_folder_to_edit) = create_signal(Option::<SheetFolder>::None);
    let (folder_to_delete, set_folder_to_delete) = create_signal(Option::<SheetFolder>::None);
    let (sheet_to_move, set_sheet_to_move) = create_signal(Option::<CharacterSummary>::None);
    let (folder_input_name, set_folder_input_name) = create_signal(String::new());
    let (folder_input_icon, set_folder_input_icon) = create_signal("📁".to_string());
    let (folder_edit_name, set_folder_edit_name) = create_signal(String::new());
    let (folder_edit_icon, set_folder_edit_icon) = create_signal("📁".to_string());
    let (is_saving_folder, set_is_saving_folder) = create_signal(false);

    // ACL Signals
    let (folder_to_share, set_folder_to_share) = create_signal(Option::<SheetFolder>::None);
    let (folder_acls_list, set_folder_acls_list) = create_signal(Vec::<FolderAclEntry>::new());
    let (is_loading_acls, set_is_loading_acls) = create_signal(false);
    let (share_grantee_type, set_share_grantee_type) = create_signal("user".to_string());
    let (share_target_input, set_share_target_input) = create_signal(String::new());
    let (share_permission_level, set_share_permission_level) = create_signal("read".to_string());
    let (is_granting_acl, set_is_granting_acl) = create_signal(false);
    let (acl_error_msg, set_acl_error_msg) = create_signal(Option::<String>::None);


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
        let folder_param = selected_folder_id.get();
        spawn_local(async move {
            match create_sheet(final_name, type_param, folder_param).await {
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
                        folders.refetch();
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

    let cancel_create_folder = move |_| set_is_create_folder_open.set(false);
    let cancel_edit_folder = move |_| set_folder_to_edit.set(None);
    let cancel_delete_folder = move |_| set_folder_to_delete.set(None);
    let cancel_move_sheet = move |_| set_sheet_to_move.set(None);

    let handle_create_folder = {
        let name_sig = folder_input_name;
        let icon_sig = folder_input_icon;
        move |ev: ev::SubmitEvent| {
            ev.prevent_default();
            if is_saving_folder.get() { return; }
            let f_name = name_sig.get().trim().to_string();
            if f_name.is_empty() { return; }
            let f_icon = icon_sig.get();
            let parent_id = create_folder_parent.get();
            set_is_saving_folder.set(true);
            spawn_local(async move {
                match create_folder(f_name, Some(f_icon), None, parent_id).await {
                    Ok(_) => {
                        folders.refetch();
                        set_is_create_folder_open.set(false);
                        set_folder_input_name.set(String::new());
                        set_is_saving_folder.set(false);
                    }
                    Err(e) => {
                        set_error_msg.set(Some(format!("Erro ao criar pasta: {}", e)));
                        set_is_saving_folder.set(false);
                    }
                }
            });
        }
    };

    let handle_update_folder = {
        let name_sig = folder_edit_name;
        let icon_sig = folder_edit_icon;
        move |ev: ev::SubmitEvent| {
            ev.prevent_default();
            if is_saving_folder.get() { return; }
            let target = match folder_to_edit.get() {
                Some(f) => f,
                None => return,
            };
            let f_name = name_sig.get().trim().to_string();
            if f_name.is_empty() { return; }
            let f_icon = icon_sig.get();
            set_is_saving_folder.set(true);
            spawn_local(async move {
                match update_folder(target.id, f_name, Some(f_icon), None).await {
                    Ok(_) => {
                        folders.refetch();
                        sheets.refetch();
                        set_folder_to_edit.set(None);
                        set_is_saving_folder.set(false);
                    }
                    Err(e) => {
                        set_error_msg.set(Some(format!("Erro ao atualizar pasta: {}", e)));
                        set_is_saving_folder.set(false);
                    }
                }
            });
        }
    };

    let handle_delete_folder = move || {
        let target = match folder_to_delete.get() {
            Some(f) => f,
            None => return,
        };
        let target_id = target.id.clone();
        spawn_local(async move {
            match delete_folder(target.id).await {
                Ok(_) => {
                    folders.refetch();
                    sheets.refetch();
                    if selected_folder_id.get_untracked().as_deref() == Some(target_id.as_str()) {
                        set_selected_folder_id.set(None);
                    }
                    set_folder_to_delete.set(None);
                }
                Err(e) => {
                    set_error_msg.set(Some(format!("Erro ao excluir pasta: {}", e)));
                    set_folder_to_delete.set(None);
                }
            }
        });
    };

    let handle_move_sheet = move |sheet_id: String, target_folder: Option<String>| {
        spawn_local(async move {
            match move_sheet_to_folder(sheet_id, target_folder).await {
                Ok(_) => {
                    sheets.refetch();
                    folders.refetch();
                    set_sheet_to_move.set(None);
                }
                Err(e) => {
                    set_error_msg.set(Some(format!("Erro ao mover ficha: {}", e)));
                    set_sheet_to_move.set(None);
                }
            }
        });
    };

    let handle_move_folder = move |folder_id: String, target_parent: Option<String>| {
        spawn_local(async move {
            match move_folder(folder_id, target_parent).await {
                Ok(_) => {
                    folders.refetch();
                }
                Err(e) => {
                    set_error_msg.set(Some(format!("Erro ao mover pasta: {}", e)));
                }
            }
        });
    };

    let load_acls = move |folder_id: String| {
        set_is_loading_acls.set(true);
        set_acl_error_msg.set(None);
        spawn_local(async move {
            match get_folder_acls(folder_id).await {
                Ok(entries) => {
                    set_folder_acls_list.set(entries);
                    set_is_loading_acls.set(false);
                }
                Err(e) => {
                    set_acl_error_msg.set(Some(format!("Erro ao carregar permissões: {}", e)));
                    set_is_loading_acls.set(false);
                }
            }
        });
    };

    let open_share_modal = move |folder: SheetFolder| {
        let fid = folder.id.clone();
        set_folder_to_share.set(Some(folder));
        set_share_target_input.set(String::new());
        set_acl_error_msg.set(None);
        load_acls(fid);
    };

    let cancel_share_folder = move |_| {
        set_folder_to_share.set(None);
        set_acl_error_msg.set(None);
    };

    let handle_grant_acl = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if is_granting_acl.get() { return; }
        let f = match folder_to_share.get() {
            Some(f) => f,
            None => return,
        };
        let g_type = share_grantee_type.get();
        let target = share_target_input.get().trim().to_string();
        if g_type != "public" && target.is_empty() {
            set_acl_error_msg.set(Some("Por favor, preencha o destinatário.".to_string()));
            return;
        }
        let perm = share_permission_level.get();
        set_is_granting_acl.set(true);
        set_acl_error_msg.set(None);
        let fid = f.id.clone();
        spawn_local(async move {
            match grant_folder_acl(fid.clone(), g_type, target, perm).await {
                Ok(_) => {
                    set_is_granting_acl.set(false);
                    set_share_target_input.set(String::new());
                    load_acls(fid);
                }
                Err(e) => {
                    set_is_granting_acl.set(false);
                    set_acl_error_msg.set(Some(format!("Erro ao conceder permissão: {}", e)));
                }
            }
        });
    };

    let handle_revoke_acl = move |acl_id: String| {
        let f = match folder_to_share.get() {
            Some(f) => f,
            None => return,
        };
        let fid = f.id.clone();
        spawn_local(async move {
            match revoke_folder_acl(acl_id).await {
                Ok(_) => {
                    load_acls(fid);
                }
                Err(e) => {
                    set_acl_error_msg.set(Some(format!("Erro ao revogar permissão: {}", e)));
                }
            }
        });
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

                            {move || {
                                if let Some(fid) = selected_folder_id.get() {
                                    let folder_list = folders.get().and_then(|r| r.ok()).unwrap_or_default();
                                    if let Some(current_f) = folder_list.iter().find(|f| f.id == fid) {
                                        view! {
                                            <div class="drive-creating-badge">
                                                <span class="drive-creating-icon">"📁"</span>
                                                <span class="drive-creating-label">{crate::i18n::tr("drive_creating_in_folder", lang())}</span>
                                                <span class="drive-creating-target">{current_f.icon.clone()} " " {current_f.name.clone()}</span>
                                                <button
                                                    type="button"
                                                    class="drive-creating-clear"
                                                    title=crate::i18n::tr("drive_move_to_root", lang())
                                                    on:click=move |_| set_selected_folder_id.set(None)
                                                >
                                                    "✕"
                                                </button>
                                            </div>
                                        }.into_view()
                                    } else {
                                        ().into_view()
                                    }
                                } else {
                                    ().into_view()
                                }
                            }}

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
                            Some(Ok(data)) => render_character_grid(
                                data,
                                set_sheet_to_delete,
                                set_sheet_to_move,
                                toggle_privacy,
                                std::collections::HashMap::new(),
                                current_lang,
                                dragged_sheet_id,
                                set_dragged_sheet_id,
                                set_drag_over_folder_id,
                            ),
                            Some(Err(e)) => view! {
                                <div class="alert-box alert-error">
                                    <p>"Erro ao carregar fichas públicas: " {e.to_string()}</p>
                                </div>
                            }.into_view(),
                        },
                        _ => match sheets.get() {
                            None => view! { <p class="loading-msg">{crate::i18n::tr("home_loading_my", current_lang)}</p> }.into_view(),
                            Some(Err(e)) => view! { 
                                <div class="alert-box alert-error">
                                    <p>"Erro ao carregar fichas: " {e.to_string()}</p>
                                </div> 
                            }.into_view(),
                            Some(Ok(all_sheets)) => {
                                let folder_list = folders.get().and_then(|r| r.ok()).unwrap_or_default();
                                let total_count = all_sheets.len();
                                let unorganized_count = all_sheets.iter().filter(|s| s.folder_id.is_none()).count();
                                
                                let mut folder_counts = std::collections::HashMap::new();
                                for s in &all_sheets {
                                    if let Some(ref fid) = s.folder_id {
                                        *folder_counts.entry(fid.clone()).or_insert(0usize) += 1;
                                    }
                                }

                                let folders_info: std::collections::HashMap<String, (String, String)> = folder_list
                                    .iter()
                                    .map(|f| (f.id.clone(), (f.name.clone(), f.icon.clone())))
                                    .collect();

                                let active_fid = selected_folder_id.get();

                                match active_fid {
                                    Some(current_fid) => {
                                        // INSIDE A FOLDER
                                        let current_folder = folder_list.iter().find(|f| f.id == current_fid).cloned();
                                        let folder_sheets: Vec<CharacterSummary> = all_sheets
                                            .into_iter()
                                            .filter(|s| s.folder_id.as_deref() == Some(current_fid.as_str()))
                                            .collect();
                                        let sheets_count = folder_sheets.len();

                                        let current_folder = match current_folder {
                                            Some(f) => f,
                                            None => {
                                                set_selected_folder_id.set(None);
                                                return view! { <div class="drive-container"></div> }.into_view();
                                            }
                                        };
                                        let f_parent_id = current_folder.parent_id.clone();
                                        let f_for_edit = current_folder.clone();
                                        let f_for_del = current_folder.clone();
                                        let f_for_share = current_folder.clone();

                                        // Compute ancestry path for multi-level breadcrumbs
                                        let mut ancestry = Vec::new();
                                        let mut curr_id = Some(current_fid.clone());
                                        while let Some(cid) = curr_id {
                                            if let Some(f) = folder_list.iter().find(|item| item.id == cid) {
                                                ancestry.push(f.clone());
                                                curr_id = f.parent_id.clone();
                                            } else {
                                                break;
                                            }
                                        }
                                        ancestry.reverse();

                                        // Subfolders
                                        let subfolders: Vec<SheetFolder> = folder_list
                                            .iter()
                                            .filter(|f| f.parent_id.as_deref() == Some(current_fid.as_str()))
                                            .cloned()
                                            .collect();
                                        let has_subfolders = !subfolders.is_empty();

                                        view! {
                                            <div class="drive-container">
                                                <div class="drive-nav-bar">
                                                    <div class="drive-breadcrumbs">
                                                        <button
                                                            type="button"
                                                            class="drive-breadcrumb-item clickable"
                                                            class:drag-over=move || drag_over_folder_id.get().as_deref() == Some("__root__")
                                                            class:drag-targetable=move || dragged_sheet_id.get().is_some() || dragged_folder_id.get().is_some()
                                                            on:click=move |_| set_selected_folder_id.set(None)
                                                            on:dragover=move |ev: ev::DragEvent| {
                                                                ev.prevent_default();
                                                                if let Some(dt) = ev.data_transfer() {
                                                                    let _ = dt.set_drop_effect("move");
                                                                }
                                                            }
                                                            on:dragenter=move |ev: ev::DragEvent| {
                                                                ev.prevent_default();
                                                                set_drag_over_folder_id.set(Some("__root__".to_string()));
                                                            }
                                                            on:dragleave=move |_| {
                                                                if drag_over_folder_id.get_untracked().as_deref() == Some("__root__") {
                                                                    set_drag_over_folder_id.set(None);
                                                                }
                                                            }
                                                            on:drop=move |ev: ev::DragEvent| {
                                                                ev.prevent_default();
                                                                set_drag_over_folder_id.set(None);
                                                                if let Some(s_id) = dragged_sheet_id.get_untracked() {
                                                                    handle_move_sheet(s_id, None);
                                                                    set_dragged_sheet_id.set(None);
                                                                } else if let Some(f_id) = dragged_folder_id.get_untracked() {
                                                                    handle_move_folder(f_id, None);
                                                                    set_dragged_folder_id.set(None);
                                                                }
                                                            }
                                                            title=crate::i18n::tr("drive_move_to_root", current_lang)
                                                        >
                                                            <span class="drive-bc-icon">"📁"</span>
                                                            <span>{crate::i18n::tr("drive_my_sheets", current_lang)}</span>
                                                        </button>

                                                        {
                                                            let total_anc = ancestry.len();
                                                            ancestry.into_iter().enumerate().map(|(idx, anc_f)| {
                                                                let is_last = idx == total_anc - 1;
                                                                let anc_id = anc_f.id.clone();
                                                                let anc_id_drop = anc_f.id.clone();
                                                                let anc_id_over = anc_f.id.clone();
                                                                let anc_icon = anc_f.icon.clone();
                                                                let anc_name = anc_f.name.clone();

                                                                if is_last {
                                                                    view! {
                                                                        <span class="drive-breadcrumb-sep">"/"</span>
                                                                        <span class="drive-breadcrumb-item active">
                                                                            <span class="drive-bc-icon">{anc_icon}</span>
                                                                            <span>{anc_name}</span>
                                                                        </span>
                                                                    }.into_view()
                                                                } else {
                                                                    view! {
                                                                        <span class="drive-breadcrumb-sep">"/"</span>
                                                                        <button
                                                                            type="button"
                                                                            class="drive-breadcrumb-item clickable"
                                                                            class:drag-over={
                                                                                let chk = anc_id_over.clone();
                                                                                move || drag_over_folder_id.get().as_deref() == Some(chk.as_str())
                                                                            }
                                                                            class:drag-targetable=move || dragged_sheet_id.get().is_some() || dragged_folder_id.get().is_some()
                                                                            on:click={
                                                                                let cid = anc_id.clone();
                                                                                move |_| set_selected_folder_id.set(Some(cid.clone()))
                                                                            }
                                                                            on:dragover=move |ev: ev::DragEvent| {
                                                                                ev.prevent_default();
                                                                                if let Some(dt) = ev.data_transfer() {
                                                                                    let _ = dt.set_drop_effect("move");
                                                                                }
                                                                            }
                                                                            on:dragenter={
                                                                                let eid = anc_id_drop.clone();
                                                                                move |ev: ev::DragEvent| {
                                                                                    ev.prevent_default();
                                                                                    set_drag_over_folder_id.set(Some(eid.clone()));
                                                                                }
                                                                            }
                                                                            on:dragleave={
                                                                                let lid = anc_id_drop.clone();
                                                                                move |_| {
                                                                                    if drag_over_folder_id.get_untracked().as_deref() == Some(lid.as_str()) {
                                                                                        set_drag_over_folder_id.set(None);
                                                                                    }
                                                                                }
                                                                            }
                                                                            on:drop={
                                                                                let did = anc_id_drop.clone();
                                                                                move |ev: ev::DragEvent| {
                                                                                    ev.prevent_default();
                                                                                    set_drag_over_folder_id.set(None);
                                                                                    if let Some(s_id) = dragged_sheet_id.get_untracked() {
                                                                                        handle_move_sheet(s_id, Some(did.clone()));
                                                                                        set_dragged_sheet_id.set(None);
                                                                                    } else if let Some(f_id) = dragged_folder_id.get_untracked() {
                                                                                        if f_id != did {
                                                                                            handle_move_folder(f_id, Some(did.clone()));
                                                                                        }
                                                                                        set_dragged_folder_id.set(None);
                                                                                    }
                                                                                }
                                                                            }
                                                                        >
                                                                            <span class="drive-bc-icon">{anc_icon}</span>
                                                                            <span>{anc_name}</span>
                                                                        </button>
                                                                    }.into_view()
                                                                }
                                                            }).collect_view()
                                                        }
                                                    </div>

                                                    <div class="drive-toolbar-actions">
                                                        <button
                                                            type="button"
                                                            class="drive-primary-btn"
                                                            on:click={
                                                                let cf_id = current_fid.clone();
                                                                move |_| {
                                                                    set_create_folder_parent.set(Some(cf_id.clone()));
                                                                    set_folder_input_name.set(String::new());
                                                                    set_folder_input_icon.set("📁".to_string());
                                                                    set_is_create_folder_open.set(true);
                                                                }
                                                            }
                                                        >
                                                            <span class="btn-icon">"➕"</span>
                                                            <span>{crate::i18n::tr("drive_btn_new_subfolder", current_lang)}</span>
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="drive-action-btn btn-folder-share"
                                                            title=crate::i18n::tr("drive_share_folder", current_lang)
                                                            on:click={
                                                                let fs = f_for_share.clone();
                                                                move |_| open_share_modal(fs.clone())
                                                            }
                                                        >
                                                            <span class="btn-icon">"👥"</span>
                                                            <span class="btn-label">{crate::i18n::tr("drive_share_folder", current_lang)}</span>
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="drive-action-btn btn-folder-rename"
                                                            title=crate::i18n::tr("folder_modal_edit_title", current_lang)
                                                            on:click={
                                                                let fe = f_for_edit.clone();
                                                                move |_| {
                                                                    set_folder_edit_name.set(fe.name.clone());
                                                                    set_folder_edit_icon.set(fe.icon.clone());
                                                                    set_folder_to_edit.set(Some(fe.clone()));
                                                                }
                                                            }
                                                        >
                                                            <span class="btn-icon">"✏️"</span>
                                                            <span class="btn-label">{crate::i18n::tr("folder_modal_edit_title", current_lang)}</span>
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="drive-action-btn btn-folder-delete"
                                                            title=crate::i18n::tr("folder_modal_delete_title", current_lang)
                                                            on:click={
                                                                let fd = f_for_del.clone();
                                                                move |_| {
                                                                    set_folder_to_delete.set(Some(fd.clone()));
                                                                }
                                                            }
                                                        >
                                                            <span class="btn-icon">"🗑️"</span>
                                                            <span class="btn-label">{crate::i18n::tr("folder_modal_delete_title", current_lang)}</span>
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="drive-action-btn btn-drive-back"
                                                            on:click={
                                                                let p_id = f_parent_id.clone();
                                                                move |_| set_selected_folder_id.set(p_id.clone())
                                                            }
                                                        >
                                                            <span>{crate::i18n::tr("drive_btn_back", current_lang)}</span>
                                                        </button>
                                                    </div>
                                                </div>

                                                // Subfolders Section (if any subfolders exist)
                                                {if has_subfolders {
                                                    let f_counts = folder_counts.clone();
                                                    view! {
                                                        <div class="drive-section">
                                                            <div class="drive-section-header">
                                                                <h3 class="drive-section-title">
                                                                    <span>{crate::i18n::tr("drive_subfolders_title", current_lang)}</span>
                                                                    <span class="drive-count-badge">{subfolders.len()}</span>
                                                                </h3>
                                                            </div>
                                                            <div class="drive-folders-grid">
                                                                {subfolders.into_iter().map(|f| {
                                                                    let count = f_counts.get(&f.id).copied().unwrap_or(0);
                                                                    let item_label = if count == 1 {
                                                                        crate::i18n::tr("drive_item_count_single", current_lang)
                                                                    } else {
                                                                        crate::i18n::tr("drive_item_count_plural", current_lang)
                                                                    };
                                                                    render_single_folder_card(
                                                                        f,
                                                                        count,
                                                                        item_label,
                                                                        current_lang,
                                                                        dragged_sheet_id,
                                                                        dragged_folder_id,
                                                                        drag_over_folder_id,
                                                                        set_dragged_sheet_id,
                                                                        set_dragged_folder_id,
                                                                        set_drag_over_folder_id,
                                                                        set_selected_folder_id,
                                                                        set_folder_edit_name,
                                                                        set_folder_edit_icon,
                                                                        set_folder_to_edit,
                                                                        set_folder_to_delete,
                                                                        open_share_modal,
                                                                        handle_move_sheet,
                                                                        handle_move_folder,
                                                                    )
                                                                }).collect_view()}
                                                            </div>
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    ().into_view()
                                                }}

                                                {if folder_sheets.is_empty() && !has_subfolders {
                                                    view! {
                                                        <div class="drive-empty-folder-box">
                                                            <span class="drive-empty-icon">"📂"</span>
                                                            <p class="drive-empty-title">{crate::i18n::tr("folder_empty_in_folder", current_lang)}</p>
                                                            <p class="drive-empty-desc">{crate::i18n::tr("drive_folder_empty", current_lang)}</p>
                                                        </div>
                                                    }.into_view()
                                                } else if !folder_sheets.is_empty() {
                                                    view! {
                                                        <div class="drive-section">
                                                            <div class="drive-section-header">
                                                                <h3 class="drive-section-title">
                                                                    <span>{crate::i18n::tr("drive_sheets_title", current_lang)}</span>
                                                                    <span class="drive-count-badge">{sheets_count}</span>
                                                                </h3>
                                                            </div>
                                                            {render_character_grid(
                                                                folder_sheets,
                                                                set_sheet_to_delete,
                                                                set_sheet_to_move,
                                                                toggle_privacy,
                                                                folders_info,
                                                                current_lang,
                                                                dragged_sheet_id,
                                                                set_dragged_sheet_id,
                                                                set_drag_over_folder_id,
                                                            )}
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    ().into_view()
                                                }}
                                            </div>
                                        }.into_view()
                                    },
                                    None => {
                                        // AT ROOT ("Minhas Fichas")
                                        let root_folders: Vec<SheetFolder> = folder_list.iter().filter(|f| f.parent_id.is_none()).cloned().collect();
                                        let has_folders = !root_folders.is_empty();
                                        let view_mode = root_view_mode.get();

                                        let root_sheets: Vec<CharacterSummary> = if view_mode == "all" {
                                            all_sheets
                                        } else {
                                            all_sheets.into_iter().filter(|s| s.folder_id.is_none()).collect()
                                        };
                                        let current_sheets_count = root_sheets.len();

                                        view! {
                                            <div class="drive-container">
                                                <div class="drive-nav-bar">
                                                    <div class="drive-breadcrumbs">
                                                        <span
                                                            class="drive-breadcrumb-item active"
                                                            class:drag-over=move || drag_over_folder_id.get().as_deref() == Some("__root__")
                                                            on:dragover=move |ev: ev::DragEvent| {
                                                                ev.prevent_default();
                                                                if let Some(dt) = ev.data_transfer() {
                                                                    let _ = dt.set_drop_effect("move");
                                                                }
                                                            }
                                                            on:dragenter=move |ev: ev::DragEvent| {
                                                                ev.prevent_default();
                                                                set_drag_over_folder_id.set(Some("__root__".to_string()));
                                                            }
                                                            on:dragleave=move |_| {
                                                                if drag_over_folder_id.get_untracked().as_deref() == Some("__root__") {
                                                                    set_drag_over_folder_id.set(None);
                                                                }
                                                            }
                                                            on:drop=move |ev: ev::DragEvent| {
                                                                ev.prevent_default();
                                                                set_drag_over_folder_id.set(None);
                                                                if let Some(sheet_id) = dragged_sheet_id.get_untracked() {
                                                                    handle_move_sheet(sheet_id, None);
                                                                    set_dragged_sheet_id.set(None);
                                                                } else if let Some(folder_id) = dragged_folder_id.get_untracked() {
                                                                    handle_move_folder(folder_id, None);
                                                                    set_dragged_folder_id.set(None);
                                                                }
                                                            }
                                                        >
                                                            <span class="drive-bc-icon">"📁"</span>
                                                            <span>{crate::i18n::tr("drive_my_sheets", current_lang)}</span>
                                                        </span>
                                                    </div>

                                                    <div class="drive-toolbar-actions">
                                                        <button
                                                            type="button"
                                                            class="drive-primary-btn"
                                                            on:click=move |_| {
                                                                set_create_folder_parent.set(None);
                                                                set_folder_input_name.set(String::new());
                                                                set_folder_input_icon.set("📁".to_string());
                                                                set_is_create_folder_open.set(true);
                                                            }
                                                        >
                                                            <span class="btn-icon">"➕"</span>
                                                            <span>{crate::i18n::tr("drive_btn_new_folder", current_lang)}</span>
                                                        </button>
                                                    </div>
                                                </div>

                                                // Folders Grid (if any root folders exist)
                                                {if has_folders {
                                                    let f_counts = folder_counts.clone();
                                                    view! {
                                                        <div class="drive-section">
                                                            <div class="drive-section-header">
                                                                <h3 class="drive-section-title">
                                                                    <span>{crate::i18n::tr("drive_folders_title", current_lang)}</span>
                                                                    <span class="drive-count-badge">{root_folders.len()}</span>
                                                                </h3>
                                                            </div>
                                                            <div class="drive-folders-grid">
                                                                {root_folders.into_iter().map(|f| {
                                                                    let count = f_counts.get(&f.id).copied().unwrap_or(0);
                                                                    let item_label = if count == 1 {
                                                                        crate::i18n::tr("drive_item_count_single", current_lang)
                                                                    } else {
                                                                        crate::i18n::tr("drive_item_count_plural", current_lang)
                                                                    };
                                                                    render_single_folder_card(
                                                                        f,
                                                                        count,
                                                                        item_label,
                                                                        current_lang,
                                                                        dragged_sheet_id,
                                                                        dragged_folder_id,
                                                                        drag_over_folder_id,
                                                                        set_dragged_sheet_id,
                                                                        set_dragged_folder_id,
                                                                        set_drag_over_folder_id,
                                                                        set_selected_folder_id,
                                                                        set_folder_edit_name,
                                                                        set_folder_edit_icon,
                                                                        set_folder_to_edit,
                                                                        set_folder_to_delete,
                                                                        open_share_modal,
                                                                        handle_move_sheet,
                                                                        handle_move_folder,
                                                                    )
                                                                }).collect_view()}
                                                            </div>
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    ().into_view()
                                                }}

                                                // Sheets Section
                                                <div class="drive-section">
                                                    <div class="drive-section-header">
                                                        <h3 class="drive-section-title">
                                                            <span>{if view_mode == "all" {
                                                                crate::i18n::tr("drive_all_sheets", current_lang)
                                                            } else {
                                                                crate::i18n::tr("drive_root_sheets", current_lang)
                                                            }}</span>
                                                            <span class="drive-count-badge">{current_sheets_count}</span>
                                                        </h3>

                                                        {if has_folders {
                                                            view! {
                                                                <div class="drive-view-mode-toggle">
                                                                    <button
                                                                        type="button"
                                                                        class="drive-toggle-btn"
                                                                        class:active=move || root_view_mode.get() == "unorganized"
                                                                        class:drag-over=move || drag_over_folder_id.get().as_deref() == Some("__root__")
                                                                        on:click=move |_| set_root_view_mode.set("unorganized")
                                                                        on:dragover=move |ev: ev::DragEvent| {
                                                                            ev.prevent_default();
                                                                            if let Some(dt) = ev.data_transfer() {
                                                                                let _ = dt.set_drop_effect("move");
                                                                            }
                                                                        }
                                                                        on:dragenter=move |ev: ev::DragEvent| {
                                                                            ev.prevent_default();
                                                                            set_drag_over_folder_id.set(Some("__root__".to_string()));
                                                                        }
                                                                        on:dragleave=move |_| {
                                                                            if drag_over_folder_id.get_untracked().as_deref() == Some("__root__") {
                                                                                set_drag_over_folder_id.set(None);
                                                                            }
                                                                        }
                                                                        on:drop=move |ev: ev::DragEvent| {
                                                                            ev.prevent_default();
                                                                            set_drag_over_folder_id.set(None);
                                                                            if let Some(sheet_id) = dragged_sheet_id.get_untracked() {
                                                                                handle_move_sheet(sheet_id, None);
                                                                                set_dragged_sheet_id.set(None);
                                                                            } else if let Some(folder_id) = dragged_folder_id.get_untracked() {
                                                                                handle_move_folder(folder_id, None);
                                                                                set_dragged_folder_id.set(None);
                                                                            }
                                                                        }
                                                                    >
                                                                        {format!("{} ({})", crate::i18n::tr("drive_root_sheets", current_lang), unorganized_count)}
                                                                    </button>
                                                                    <button
                                                                        type="button"
                                                                        class="drive-toggle-btn"
                                                                        class:active=move || root_view_mode.get() == "all"
                                                                        on:click=move |_| set_root_view_mode.set("all")
                                                                    >
                                                                        {format!("{} ({})", crate::i18n::tr("drive_all_sheets", current_lang), total_count)}
                                                                    </button>
                                                                </div>
                                                            }.into_view()
                                                        } else {
                                                            ().into_view()
                                                        }}
                                                    </div>

                                                    {if root_sheets.is_empty() {
                                                        view! {
                                                            <p class="empty-msg">{crate::i18n::tr("home_empty_my", current_lang)}</p>
                                                        }.into_view()
                                                    } else {
                                                        render_character_grid(
                                                            root_sheets,
                                                            set_sheet_to_delete,
                                                            set_sheet_to_move,
                                                            toggle_privacy,
                                                            folders_info,
                                                            current_lang,
                                                            dragged_sheet_id,
                                                            set_dragged_sheet_id,
                                                            set_drag_over_folder_id,
                                                        )
                                                    }}
                                                </div>
                                            </div>
                                        }.into_view()
                                    }
                                }
                            }
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

            // Create Folder Modal
            {move || if is_create_folder_open.get() {
                Some(view! {
                    <div class="modal-overlay" on:click=cancel_create_folder>
                        <div class="modal-card folder-modal-card" on:click=move |ev| ev.stop_propagation()>
                            <h3 class="modal-title">{move || crate::i18n::tr("folder_modal_create_title", lang())}</h3>
                            <form on:submit=handle_create_folder class="folder-form">
                                <div class="folder-form-field">
                                    <label class="folder-field-label">{move || crate::i18n::tr("folder_name_label", lang())}</label>
                                    <input
                                        type="text"
                                        class="folder-input"
                                        placeholder=move || crate::i18n::tr("folder_name_ph", lang())
                                        prop:value=folder_input_name
                                        on:input=move |ev| set_folder_input_name.set(event_target_value(&ev))
                                        required
                                        autofocus
                                    />
                                </div>
                                <div class="folder-form-field">
                                    <label class="folder-field-label">{move || crate::i18n::tr("folder_icon_label", lang())}</label>
                                    <div class="folder-icon-presets">
                                        {FOLDER_ICON_PRESETS.iter().map(|&icon| {
                                            let icon_str = icon.to_string();
                                            let icon_val = icon_str.clone();
                                            let is_active = move || folder_input_icon.get() == icon_val;
                                            view! {
                                                <button
                                                    type="button"
                                                    class="folder-icon-preset-btn"
                                                    class:active=is_active
                                                    on:click=move |_| set_folder_input_icon.set(icon_str.clone())
                                                >
                                                    {icon}
                                                </button>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                                <div class="modal-actions">
                                    <button type="button" class="modal-btn btn-cancel" on:click=cancel_create_folder>
                                        {move || crate::i18n::tr("home_btn_cancel", lang())}
                                    </button>
                                    <button type="submit" class="modal-btn btn-primary" disabled=is_saving_folder>
                                        {move || crate::i18n::tr("folder_create", lang())}
                                    </button>
                                </div>
                            </form>
                        </div>
                    </div>
                })
            } else {
                None
            }}

            // Edit Folder Modal
            {move || folder_to_edit.get().map(|_target| view! {
                <div class="modal-overlay" on:click=cancel_edit_folder>
                    <div class="modal-card folder-modal-card" on:click=move |ev| ev.stop_propagation()>
                        <h3 class="modal-title">{move || crate::i18n::tr("folder_modal_edit_title", lang())}</h3>
                        <form on:submit=handle_update_folder class="folder-form">
                            <div class="folder-form-field">
                                <label class="folder-field-label">{move || crate::i18n::tr("folder_name_label", lang())}</label>
                                <input
                                    type="text"
                                    class="folder-input"
                                    placeholder=move || crate::i18n::tr("folder_name_ph", lang())
                                    prop:value=folder_edit_name
                                    on:input=move |ev| set_folder_edit_name.set(event_target_value(&ev))
                                    required
                                    autofocus
                                />
                            </div>
                            <div class="folder-form-field">
                                <label class="folder-field-label">{move || crate::i18n::tr("folder_icon_label", lang())}</label>
                                <div class="folder-icon-presets">
                                    {FOLDER_ICON_PRESETS.iter().map(|&icon| {
                                        let icon_str = icon.to_string();
                                        let icon_val = icon_str.clone();
                                        let is_active = move || folder_edit_icon.get() == icon_val;
                                        view! {
                                            <button
                                                type="button"
                                                class="folder-icon-preset-btn"
                                                class:active=is_active
                                                on:click=move |_| set_folder_edit_icon.set(icon_str.clone())
                                            >
                                                {icon}
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                            <div class="modal-actions">
                                <button type="button" class="modal-btn btn-cancel" on:click=cancel_edit_folder>
                                    {move || crate::i18n::tr("home_btn_cancel", lang())}
                                </button>
                                <button type="submit" class="modal-btn btn-primary" disabled=is_saving_folder>
                                    {move || crate::i18n::tr("folder_save", lang())}
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            })}

            // Delete Folder Modal
            {move || folder_to_delete.get().map(|target| view! {
                <div class="modal-overlay" on:click=cancel_delete_folder>
                    <div class="modal-card" on:click=move |ev| ev.stop_propagation()>
                        <h3 class="modal-title">{move || crate::i18n::tr("folder_modal_delete_title", lang())}</h3>
                        <p class="modal-text">
                            {move || crate::i18n::tr("home_delete_prompt", lang())}
                            <strong>{target.name}</strong>"?"
                        </p>
                        <p class="modal-subtext">{move || crate::i18n::tr("folder_modal_delete_desc", lang())}</p>
                        <div class="modal-actions">
                            <button type="button" class="modal-btn btn-cancel" on:click=cancel_delete_folder>
                                {move || crate::i18n::tr("home_btn_cancel", lang())}
                            </button>
                            <button type="button" class="modal-btn btn-danger" on:click=move |_| handle_delete_folder()>
                                {move || crate::i18n::tr("home_btn_confirm_delete", lang())}
                            </button>
                        </div>
                    </div>
                </div>
            })}

            // Move Sheet Modal
            {move || sheet_to_move.get().map(|target| {
                let sheet_id = target.id.clone();
                let current_fid = target.folder_id.clone();
                let folder_list = folders.get().and_then(|r| r.ok()).unwrap_or_default();
                let all_f_modal = folder_list.clone();

                view! {
                    <div class="modal-overlay" on:click=cancel_move_sheet>
                        <div class="modal-card folder-modal-card" on:click=move |ev| ev.stop_propagation()>
                            <h3 class="modal-title">{move || crate::i18n::tr("folder_modal_move_title", lang())}</h3>
                            <p class="modal-text">
                                "Mover "<strong>{target.name.clone()}</strong>" para:"
                            </p>
                            <div class="move-folder-options-list">
                                {
                                    let sid = sheet_id.clone();
                                    let is_curr = current_fid.is_none();
                                    view! {
                                        <button
                                            type="button"
                                            class="move-folder-opt-btn"
                                            class:active=is_curr
                                            on:click=move |_| handle_move_sheet(sid.clone(), None)
                                        >
                                            <span class="move-opt-icon">"📁"</span>
                                            <span class="move-opt-name">{move || crate::i18n::tr("drive_move_to_root", lang())}</span>
                                            {if is_curr {
                                                view! { <span class="move-opt-curr">"✓ Atual"</span> }.into_view()
                                            } else {
                                                ().into_view()
                                            }}
                                        </button>
                                    }
                                }

                                {folder_list.into_iter().map(|f| {
                                    let sid = sheet_id.clone();
                                    let fid = f.id.clone();
                                    let is_curr = current_fid.as_deref() == Some(fid.as_str());
                                    let display_path = get_folder_path(&fid, &all_f_modal);
                                    view! {
                                        <button
                                            type="button"
                                            class="move-folder-opt-btn"
                                            class:active=is_curr
                                            on:click=move |_| handle_move_sheet(sid.clone(), Some(fid.clone()))
                                        >
                                            <span class="move-opt-icon">{f.icon.clone()}</span>
                                            <span class="move-opt-name">{display_path}</span>
                                            {if is_curr {
                                                view! { <span class="move-opt-curr">"✓ Atual"</span> }.into_view()
                                            } else {
                                                ().into_view()
                                            }}
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                            <div class="modal-actions">
                                <button type="button" class="modal-btn btn-cancel" on:click=cancel_move_sheet>
                                    {move || crate::i18n::tr("home_btn_cancel", lang())}
                                </button>
                            </div>
                        </div>
                    </div>
                }
            })}

            // Share Folder Modal (ACL)
            {move || folder_to_share.get().map(|target_folder| {
                let tf_name = target_folder.name.clone();
                let tf_icon = target_folder.icon.clone();
                let current_lang = lang();

                view! {
                    <div class="modal-overlay" on:click=cancel_share_folder>
                        <div class="modal-card share-modal-card" on:click=move |ev| ev.stop_propagation()>
                            <div class="share-modal-header">
                                <div class="share-modal-header-info">
                                    <h3 class="modal-title">{crate::i18n::tr("drive_share_modal_title", current_lang)}</h3>
                                    <div class="share-modal-subtitle">
                                        <span>{tf_icon}</span>
                                        <strong>{tf_name}</strong>
                                    </div>
                                </div>
                                <button type="button" class="share-close-x-btn" on:click=cancel_share_folder title="Fechar">
                                    "✕"
                                </button>
                            </div>

                            {move || acl_error_msg.get().map(|msg| view! {
                                <div class="alert-box alert-error" style="margin-bottom: 0.75rem; padding: 0.5rem 0.75rem; font-size: 0.85rem;">
                                    <span>{msg}</span>
                                </div>
                            })}

                            // Grantee type selector
                            <div class="share-type-tabs">
                                <button
                                    type="button"
                                    class="share-type-tab-btn"
                                    class:active=move || share_grantee_type.get() == "user"
                                    on:click=move |_| set_share_grantee_type.set("user".to_string())
                                >
                                    <span>"👤"</span>
                                    <span>{crate::i18n::tr("drive_share_with_user", current_lang)}</span>
                                </button>
                                <button
                                    type="button"
                                    class="share-type-tab-btn"
                                    class:active=move || share_grantee_type.get() == "room"
                                    on:click=move |_| set_share_grantee_type.set("room".to_string())
                                >
                                    <span>"🏰"</span>
                                    <span>{crate::i18n::tr("drive_share_with_room", current_lang)}</span>
                                </button>
                                <button
                                    type="button"
                                    class="share-type-tab-btn"
                                    class:active=move || share_grantee_type.get() == "public"
                                    on:click=move |_| set_share_grantee_type.set("public".to_string())
                                >
                                    <span>"🌐"</span>
                                    <span>"Público"</span>
                                </button>
                            </div>

                            // Add Access Form
                            <form on:submit=handle_grant_acl class="share-grant-form">
                                {move || {
                                    let gtype = share_grantee_type.get();
                                    if gtype == "public" {
                                        view! {
                                            <div class="share-public-banner">
                                                <span class="share-public-icon">"🌐"</span>
                                                <div class="share-public-text">
                                                    <strong>"Acesso Público Geral"</strong>
                                                    <span>"Qualquer usuário cadastrado no sistema poderá acessar esta pasta."</span>
                                                </div>
                                            </div>
                                        }.into_view()
                                    } else {
                                        let (label_text, placeholder) = if gtype == "user" {
                                            ("Nome do Jogador / Usuário", "Digite o nome exato do usuário (ex: vitor, narrador)...")
                                        } else {
                                            ("Sala de Crônica (ID ou Código)", "Digite o código da sala (ex: MTA-AB12) ou ID...")
                                        };
                                        view! {
                                            <div class="share-field-group">
                                                <label class="share-field-label">{label_text}</label>
                                                <input class="share-target-input"
                                                    type="text"
                                                    placeholder=placeholder
                                                    prop:value=share_target_input
                                                    on:input=move |ev| set_share_target_input.set(event_target_value(&ev))
                                                    required
                                                />
                                            </div>
                                        }.into_view()
                                    }
                                }}

                                <div class="share-actions-row">
                                    <div class="share-perm-select-col">
                                        <label class="share-field-label">"Nível de Permissão"</label>
                                        <select
                                            class="share-perm-select"
                                            on:change=move |ev| set_share_permission_level.set(event_target_value(&ev))
                                            prop:value=share_permission_level
                                        >
                                            <option value="read">{crate::i18n::tr("drive_share_permission_read", current_lang)}</option>
                                            <option value="write">{crate::i18n::tr("drive_share_permission_write", current_lang)}</option>
                                            <option value="admin">{crate::i18n::tr("drive_share_permission_admin", current_lang)}</option>
                                        </select>
                                    </div>
                                    <button
                                        type="submit"
                                        class="share-add-btn"
                                        disabled=is_granting_acl
                                    >
                                        <span>"➕"</span>
                                        <span>{crate::i18n::tr("drive_share_add_btn", current_lang)}</span>
                                    </button>
                                </div>
                            </form>

                            <div class="share-inherited-note">
                                {crate::i18n::tr("drive_share_inherited_note", current_lang)}
                            </div>

                            <div class="share-section-heading">"Acessos Ativos"</div>

                            <div class="share-entries-list">
                                // Owner entry
                                <div class="share-entry-item">
                                    <div class="share-entry-left">
                                        <span class="share-entry-icon">"👑"</span>
                                        <span class="share-entry-name">"Você (Proprietário)"</span>
                                    </div>
                                    <div class="share-entry-right">
                                        <span class="share-perm-badge badge-owner">{crate::i18n::tr("drive_share_owner", current_lang)}</span>
                                    </div>
                                </div>

                                // Other ACL entries
                                {move || {
                                    if is_loading_acls.get() {
                                        view! { <p style="font-size: 0.85rem; color: #64748b; padding: 0.5rem 0;">"Carregando permissões..."</p> }.into_view()
                                    } else {
                                        let acls = folder_acls_list.get();
                                        if acls.is_empty() {
                                            view! { <p style="font-size: 0.85rem; color: #94a3b8; font-style: italic; padding: 0.5rem 0;">{crate::i18n::tr("drive_share_empty", current_lang)}</p> }.into_view()
                                        } else {
                                            acls.into_iter().map(|entry| {
                                                let acl_id = entry.id.clone();
                                                let type_icon = match entry.grantee_type.as_str() {
                                                    "user" => "👤",
                                                    "room" => "🏰",
                                                    _ => "🌐",
                                                };
                                                let badge_class = match entry.permission.as_str() {
                                                    "admin" => "badge-admin",
                                                    "write" => "badge-write",
                                                    _ => "badge-read",
                                                };
                                                let perm_label = match entry.permission.as_str() {
                                                    "admin" => crate::i18n::tr("drive_share_permission_admin", current_lang),
                                                    "write" => crate::i18n::tr("drive_share_permission_write", current_lang),
                                                    _ => crate::i18n::tr("drive_share_permission_read", current_lang),
                                                };
                                                let aid_del = acl_id.clone();
                                                view! {
                                                    <div class="share-entry-item">
                                                        <div class="share-entry-left">
                                                            <span class="share-entry-icon">{type_icon}</span>
                                                            <span class="share-entry-name">{entry.grantee_name}</span>
                                                        </div>
                                                        <div class="share-entry-right">
                                                            <span class=format!("share-perm-badge {}", badge_class)>{perm_label}</span>
                                                            <button
                                                                type="button"
                                                                class="share-revoke-btn"
                                                                title=crate::i18n::tr("drive_share_revoke_btn", current_lang)
                                                                on:click=move |_| handle_revoke_acl(aid_del.clone())
                                                            >
                                                                "✕"
                                                            </button>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view()
                                        }
                                    }
                                }}
                            </div>

                            <div class="modal-actions">
                                <button type="button" class="modal-btn btn-cancel" on:click=cancel_share_folder>
                                    {crate::i18n::tr("drive_share_close", current_lang)}
                                </button>
                            </div>
                        </div>
                    </div>
                }
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
    set_sheet_to_move: WriteSignal<Option<CharacterSummary>>,
    toggle_privacy: F,
    folders_info: std::collections::HashMap<String, (String, String)>,
    current_lang: crate::i18n::Language,
    dragged_sheet_id: ReadSignal<Option<String>>,
    set_dragged_sheet_id: WriteSignal<Option<String>>,
    set_drag_over_folder_id: WriteSignal<Option<String>>,
) -> View
where
    F: Fn(String, bool) + Copy + 'static,
{
    view! {
        <div class="character-cards-grid">
            {data.into_iter().map(|summary| {
                let summary_clone = summary.clone();
                let id = summary.id.clone();
                let id_card = id.clone();
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

                let is_dragging = move || dragged_sheet_id.get().as_deref() == Some(id_card.as_str());

                view! {
                    <div
                        class="character-card"
                        class:is-dragging=is_dragging
                        draggable=if is_owner { "true" } else { "false" }
                        on:dragstart={
                            let id_drag = id.clone();
                            move |ev: ev::DragEvent| {
                                set_dragged_sheet_id.set(Some(id_drag.clone()));
                                if let Some(dt) = ev.data_transfer() {
                                    let _ = dt.set_data("text/plain", &id_drag);
                                    let _ = dt.set_effect_allowed("move");
                                }
                            }
                        }
                        on:dragend=move |_| {
                            set_dragged_sheet_id.set(None);
                            set_drag_over_folder_id.set(None);
                        }
                        on:click=move |_| {
                            if dragged_sheet_id.get_untracked().is_some() { return; }
                            use_navigate()(&format!("/sheet/{}", id_nav), Default::default())
                        }
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
                                let summary_for_move = summary_clone.clone();
                                let summary_for_del = summary_clone.clone();
                                view! {
                                    <div class="card-floating-actions" draggable="false">
                                        <button
                                            type="button"
                                            draggable="false"
                                            class="card-float-action-btn card-move-floating-btn"
                                            on:click=move |ev: ev::MouseEvent| {
                                                ev.stop_propagation();
                                                set_sheet_to_move.set(Some(summary_for_move.clone()));
                                            }
                                            title=crate::i18n::tr("card_move_tooltip", current_lang)
                                        >
                                            "📁"
                                        </button>
                                        <button
                                            type="button"
                                            draggable="false"
                                            class="card-float-action-btn card-delete-floating-btn"
                                            on:click=move |ev: ev::MouseEvent| {
                                                ev.stop_propagation();
                                                set_sheet_to_delete.set(Some(summary_for_del.clone()));
                                            }
                                            title=crate::i18n::tr("card_delete_tooltip", current_lang)
                                        >
                                            "🗑️"
                                        </button>
                                    </div>
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
                                    {summary.folder_id.as_ref().and_then(|fid| folders_info.get(fid)).map(|(fname, ficon)| {
                                        view! {
                                            <span class="meta-tag folder-tag" title=format!("Pasta: {}", fname)>
                                                {ficon.clone()} " " {fname.clone()}
                                            </span>
                                        }
                                    })}
                                    <span class="meta-tag tradition-tag" title=tradition_display.clone()>{tradition_display}</span>
                                    <span class="meta-tag essence-tag" title=essence_display.clone()>{essence_display}</span>
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

pub fn get_folder_path(fid: &str, all_folders: &[SheetFolder]) -> String {
    let mut names = Vec::new();
    let mut curr = Some(fid.to_string());
    while let Some(c) = curr {
        if let Some(f) = all_folders.iter().find(|item| item.id == c) {
            names.push(f.name.clone());
            curr = f.parent_id.clone();
        } else {
            break;
        }
    }
    names.reverse();
    names.join(" / ")
}

fn render_single_folder_card<FShare, FMoveSheet, FMoveFolder>(
    f: SheetFolder,
    count: usize,
    item_label: &'static str,
    current_lang: crate::i18n::Language,
    dragged_sheet_id: ReadSignal<Option<String>>,
    dragged_folder_id: ReadSignal<Option<String>>,
    drag_over_folder_id: ReadSignal<Option<String>>,
    set_dragged_sheet_id: WriteSignal<Option<String>>,
    set_dragged_folder_id: WriteSignal<Option<String>>,
    set_drag_over_folder_id: WriteSignal<Option<String>>,
    set_selected_folder_id: WriteSignal<Option<String>>,
    set_folder_edit_name: WriteSignal<String>,
    set_folder_edit_icon: WriteSignal<String>,
    set_folder_to_edit: WriteSignal<Option<SheetFolder>>,
    set_folder_to_delete: WriteSignal<Option<SheetFolder>>,
    open_share_modal: FShare,
    handle_move_sheet: FMoveSheet,
    handle_move_folder: FMoveFolder,
) -> View
where
    FShare: Fn(SheetFolder) + Copy + 'static,
    FMoveSheet: Fn(String, Option<String>) + Copy + 'static,
    FMoveFolder: Fn(String, Option<String>) + Copy + 'static,
{
    let fid = f.id.clone();
    let f_for_edit = f.clone();
    let f_for_del = f.clone();
    let f_for_share = f.clone();
    let fid_drag_chk = fid.clone();
    let fid_over_chk = fid.clone();
    let fid_target_chk = fid.clone();
    let fid_drag = fid.clone();
    let fid_enter = fid.clone();
    let fid_leave = fid.clone();
    let fid_drop = fid.clone();

    let is_dragging = move || dragged_folder_id.get().as_deref() == Some(fid_drag_chk.as_str());
    let is_drag_over = move || drag_over_folder_id.get().as_deref() == Some(fid_over_chk.as_str());
    let is_targetable = move || dragged_sheet_id.get().is_some() || (dragged_folder_id.get().is_some() && dragged_folder_id.get().as_deref() != Some(fid_target_chk.as_str()));

    view! {
        <div
            class="drive-folder-card"
            class:drag-over=is_drag_over
            class:drag-targetable=is_targetable
            class:is-dragging=is_dragging
            draggable="true"
            on:dragstart={
                let fid_d = fid_drag.clone();
                move |ev: ev::DragEvent| {
                    set_dragged_folder_id.set(Some(fid_d.clone()));
                    if let Some(dt) = ev.data_transfer() {
                        let _ = dt.set_data("text/plain", &fid_d);
                        let _ = dt.set_effect_allowed("move");
                    }
                }
            }
            on:dragend=move |_| {
                set_dragged_folder_id.set(None);
                set_drag_over_folder_id.set(None);
            }
            on:click=move |_| {
                if dragged_folder_id.get_untracked().is_some() || dragged_sheet_id.get_untracked().is_some() { return; }
                set_selected_folder_id.set(Some(fid.clone()))
            }
            on:dragover=move |ev: ev::DragEvent| {
                ev.prevent_default();
                if let Some(dt) = ev.data_transfer() {
                    let _ = dt.set_drop_effect("move");
                }
            }
            on:dragenter={
                let fid_e = fid_enter.clone();
                move |ev: ev::DragEvent| {
                    ev.prevent_default();
                    set_drag_over_folder_id.set(Some(fid_e.clone()));
                }
            }
            on:dragleave={
                let fid_l = fid_leave.clone();
                move |_| {
                    if drag_over_folder_id.get_untracked().as_deref() == Some(fid_l.as_str()) {
                        set_drag_over_folder_id.set(None);
                    }
                }
            }
            on:drop={
                let fid_dp = fid_drop.clone();
                move |ev: ev::DragEvent| {
                    ev.prevent_default();
                    set_drag_over_folder_id.set(None);
                    if let Some(sheet_id) = dragged_sheet_id.get_untracked() {
                        handle_move_sheet(sheet_id, Some(fid_dp.clone()));
                        set_dragged_sheet_id.set(None);
                    } else if let Some(folder_id) = dragged_folder_id.get_untracked() {
                        if folder_id != fid_dp {
                            handle_move_folder(folder_id, Some(fid_dp.clone()));
                        }
                        set_dragged_folder_id.set(None);
                    }
                }
            }
        >
            <div class="drive-folder-icon-badge">
                <span class="drive-folder-emoji">{f.icon.clone()}</span>
            </div>
            <div class="drive-folder-info">
                <span class="drive-folder-name" title=f.name.clone()>{f.name.clone()}</span>
                <span class="drive-folder-count">{format!("{} {}", count, item_label)}</span>
            </div>
            <div class="drive-folder-menu-actions">
                <button
                    type="button"
                    class="drive-folder-menu-btn"
                    title=crate::i18n::tr("drive_share_folder", current_lang)
                    on:click={
                        let fsh = f_for_share.clone();
                        move |ev: ev::MouseEvent| {
                            ev.stop_propagation();
                            open_share_modal(fsh.clone());
                        }
                    }
                >
                    "👥"
                </button>
                <button
                    type="button"
                    class="drive-folder-menu-btn"
                    title=crate::i18n::tr("folder_modal_edit_title", current_lang)
                    on:click={
                        let fe = f_for_edit.clone();
                        move |ev: ev::MouseEvent| {
                            ev.stop_propagation();
                            set_folder_edit_name.set(fe.name.clone());
                            set_folder_edit_icon.set(fe.icon.clone());
                            set_folder_to_edit.set(Some(fe.clone()));
                        }
                    }
                >
                    "✏️"
                </button>
                <button
                    type="button"
                    class="drive-folder-menu-btn"
                    title=crate::i18n::tr("folder_modal_delete_title", current_lang)
                    on:click={
                        let fd = f_for_del.clone();
                        move |ev: ev::MouseEvent| {
                            ev.stop_propagation();
                            set_folder_to_delete.set(Some(fd.clone()));
                        }
                    }
                >
                    "🗑️"
                </button>
            </div>
        </div>
    }.into_view()
}
