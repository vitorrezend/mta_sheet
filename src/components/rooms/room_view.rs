use leptos::*;
use leptos_router::*;
use crate::rooms::{
    get_room_details, assign_sheet_to_room, remove_sheet_from_room,
    toggle_sheet_room_visibility, update_room_chantry, update_room_chronicle_notes,
    clone_and_assign_sheet_to_member, update_room_settings, update_room_sheet_stats,
    update_room_sheet_order,
    QuickStatAction, ChantryPoolData, RoomMemberInfo, RoomDetails,
};
use crate::state::get_sheets;
use crate::components::Navbar;
use crate::components::rooms::{InitiativeDrawer, BattleGrid, PartyCardDesktop, PartyCardMobile};
use crate::rooms::RoomMapData;

#[component]
pub fn RoomView() -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let params = use_params_map();
    let room_id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    let room_resource = create_local_resource(room_id, |id| async move {
        if id.is_empty() {
            return Err(ServerFnError::new("ID da sala não fornecido"));
        }
        get_room_details(id).await
    });

    #[cfg(target_arch = "wasm32")]
    {
        let is_active = std::rc::Rc::new(std::cell::Cell::new(true));
        let is_active_cleanup = is_active.clone();

        let interval = gloo_timers::callback::Interval::new(4_000, move || {
            if is_active.get() {
                room_resource.refetch();
            }
        });

        on_cleanup(move || {
            is_active_cleanup.set(false);
            drop(interval);
        });
    }

    let my_sheets_resource = create_local_resource(|| (), |_| async move { get_sheets().await });

    let (room_data, set_room_data) = create_signal(Option::<RoomDetails>::None);
    let (is_loaded, set_is_loaded) = create_signal(false);
    let (load_error, set_load_error) = create_signal(Option::<String>::None);

    let (active_tab, set_active_tab) = create_signal("party");
    let (show_assign_modal, set_show_assign_modal) = create_signal(false);
    let (show_initiative, set_show_initiative) = create_signal(false);
    let (settings_is_public, set_settings_is_public) = create_signal(false);
    let (settings_new_password, set_settings_new_password) = create_signal(String::new());
    let (settings_remove_password, set_settings_remove_password) = create_signal(false);
    let (is_saving_settings, set_is_saving_settings) = create_signal(false);
    let (settings_initialized, set_settings_initialized) = create_signal(false);

    let (selected_sheet_id, set_selected_sheet_id) = create_signal(String::new());
    let (dragged_sheet_id, set_dragged_sheet_id) = create_signal(Option::<String>::None);
    let (drag_over_id, set_drag_over_id) = create_signal(Option::<String>::None);
    let (target_clone_member, set_target_clone_member) = create_signal(Option::<RoomMemberInfo>::None);
    let (selected_clone_sheet_id, set_selected_clone_sheet_id) = create_signal(String::new());
    let (is_cloning, set_is_cloning) = create_signal(false);
    let (copied_code, set_copied_code) = create_signal(false);
    let (copied_link, set_copied_link) = create_signal(false);
    let (feedback_msg, set_feedback_msg) = create_signal(Option::<String>::None);
    let (error_msg, set_error_msg) = create_signal(Option::<String>::None);

    let (chantry_loc, set_chantry_loc) = create_signal(String::new());
    let (chantry_node, set_chantry_node) = create_signal(1);
    let (chantry_library, set_chantry_library) = create_signal(1);
    let (chantry_quint, set_chantry_quint) = create_signal(0);
    let (chantry_max_quint, set_chantry_max_quint) = create_signal(20);
    let (chantry_notes, set_chantry_notes) = create_signal(String::new());
    let (chantry_initialized, set_chantry_initialized) = create_signal(false);

    let (chronicle_text, set_chronicle_text) = create_signal(String::new());
    let (chronicle_initialized, set_chronicle_initialized) = create_signal(false);

    let (map_data, set_map_data) = create_signal(RoomMapData::default());
    let (map_initialized, set_map_initialized) = create_signal(false);

    let features_resource = create_local_resource(|| (), |_| async move { crate::settings::get_system_feature_flags().await });
    let is_map_enabled = Signal::derive(move || {
        features_resource.get().and_then(|r| r.ok()).map(|flags| {
            flags.iter().find(|f| f.key == "feature_tactical_grid").map(|f| f.is_enabled_for_user).unwrap_or(false)
        }).unwrap_or(false)
    });

    create_effect(move |_| {
        if !is_map_enabled.get() && active_tab.get() == "map" {
            set_active_tab.set("party");
        }
    });

    create_effect(move |_| {
        if let Some(res) = room_resource.get() {
            match res {
                Ok(room) => {
                    if !chantry_initialized.get_untracked() {
                        set_chantry_loc.set(room.chantry.location_name.clone());
                        set_chantry_node.set(room.chantry.node_rating);
                        set_chantry_library.set(room.chantry.library_rating);
                        set_chantry_quint.set(room.chantry.quintessence_pool);
                        set_chantry_max_quint.set(if room.chantry.max_quintessence > 0 { room.chantry.max_quintessence } else { 20 });
                        set_chantry_notes.set(room.chantry.notes.clone());
                        set_chantry_initialized.set(true);
                    }
                    if !chronicle_initialized.get_untracked() {
                        set_chronicle_text.set(room.chronicle_notes.clone());
                        set_chronicle_initialized.set(true);
                    }
                    if !map_initialized.get_untracked() {
                        set_map_data.set(room.map_data.clone());
                        set_map_initialized.set(true);
                    }
                    if !settings_initialized.get_untracked() {
                        set_settings_is_public.set(room.is_public);
                        set_settings_initialized.set(true);
                    }
                    if dragged_sheet_id.get_untracked().is_none() {
                        set_room_data.set(Some(room));
                    }
                    set_is_loaded.set(true);
                    set_load_error.set(None);
                }
                Err(e) => {
                    if !is_loaded.get_untracked() {
                        set_load_error.set(Some(e.to_string()));
                    }
                }
            }
        }
    });

    let on_assign_sheet = move |_| {
        let s_id = selected_sheet_id.get();
        let r_id = room_id();
        if s_id.is_empty() || r_id.is_empty() {
            return;
        }

        spawn_local(async move {
            match assign_sheet_to_room(s_id, r_id).await {
                Ok(_) => {
                    let _ = set_show_assign_modal.try_set(false);
                    let _ = set_feedback_msg.try_set(Some("Ficha vinculada à mesa com sucesso!".to_string()));
                    room_resource.refetch();
                }
                Err(e) => {
                    let _ = set_error_msg.try_set(Some(e.to_string()));
                }
            }
        });
    };

    let on_clone_and_assign = move |_| {
        let member = target_clone_member.get();
        let s_id = selected_clone_sheet_id.get();
        let r_id = room_id();

        if let Some(target) = member {
            if s_id.is_empty() || r_id.is_empty() {
                return;
            }

            let target_name = target.username.clone();
            let target_id = target.user_id.clone();
            set_is_cloning.set(true);

            spawn_local(async move {
                match clone_and_assign_sheet_to_member(r_id, s_id, target_id).await {
                    Ok(_) => {
                        let _ = set_is_cloning.try_set(false);
                        let _ = set_target_clone_member.try_set(None);
                        let _ = set_selected_clone_sheet_id.try_set(String::new());
                        let _ = set_feedback_msg.try_set(Some(format!("Ficha clonada e entregue com sucesso para {}!", target_name)));
                        room_resource.refetch();
                    }
                    Err(e) => {
                        let _ = set_is_cloning.try_set(false);
                        let _ = set_error_msg.try_set(Some(e.to_string()));
                    }
                }
            });
        }
    };

    let on_remove_sheet = move |sheet_id: String| {
        spawn_local(async move {
            match remove_sheet_from_room(sheet_id).await {
                Ok(_) => {
                    set_feedback_msg.set(Some("Ficha desvinculada da mesa.".to_string()));
                    room_resource.refetch();
                }
                Err(e) => {
                    set_error_msg.set(Some(e.to_string()));
                }
            }
        });
    };

    let on_toggle_visibility = move |sheet_id: String, current_hidden: bool| {
        spawn_local(async move {
            match toggle_sheet_room_visibility(sheet_id, !current_hidden).await {
                Ok(_) => {
                    set_feedback_msg.set(Some(if !current_hidden {
                        "Ficha agora está OCULTA para os outros jogadores (visível apenas para você/narrador).".to_string()
                    } else {
                        "Ficha agora está VISÍVEL para toda a mesa!".to_string()
                    }));
                    room_resource.refetch();
                }
                Err(e) => {
                    set_error_msg.set(Some(e.to_string()));
                }
            }
        });
    };

    let on_save_chantry = move |_| {
        let r_id = room_id();
        if r_id.is_empty() { return; }

        let chantry_data = ChantryPoolData {
            quintessence_pool: chantry_quint.get(),
            max_quintessence: chantry_max_quint.get(),
            node_rating: chantry_node.get(),
            library_rating: chantry_library.get(),
            location_name: chantry_loc.get(),
            notes: chantry_notes.get(),
        };

        spawn_local(async move {
            match update_room_chantry(r_id, chantry_data).await {
                Ok(_) => {
                    set_feedback_msg.set(Some("Recursos da Capela atualizados com sucesso!".to_string()));
                    room_resource.refetch();
                }
                Err(e) => {
                    set_error_msg.set(Some(e.to_string()));
                }
            }
        });
    };

    let on_save_chronicle = move |_| {
        let r_id = room_id();
        if r_id.is_empty() { return; }

        let text = chronicle_text.get();
        spawn_local(async move {
            match update_room_chronicle_notes(r_id, text).await {
                Ok(_) => {
                    set_feedback_msg.set(Some("Diário da Crônica salvo com sucesso!".to_string()));
                    room_resource.refetch();
                }
                Err(e) => {
                    set_error_msg.set(Some(e.to_string()));
                }
            }
        });
    };

    let on_quick_stat = move |sheet_id: String, action: QuickStatAction| {
        let r_id = room_id();
        if r_id.is_empty() { return; }

        spawn_local(async move {
            match update_room_sheet_stats(r_id, sheet_id.clone(), action).await {
                Ok(updated_summary) => {
                    set_room_data.update(|data| {
                        if let Some(room) = data {
                            if let Some(s) = room.sheets.iter_mut().find(|s| s.id == sheet_id) {
                                *s = updated_summary;
                            }
                        }
                    });
                }
                Err(e) => {
                    set_error_msg.set(Some(e.to_string()));
                }
            }
        });
    };

    let on_drag_start = move |sheet_id: String| {
        set_dragged_sheet_id.set(Some(sheet_id));
    };

    let on_drag_over = move |target_id: String| {
        let current_dragged = dragged_sheet_id.get_untracked();
        if let Some(source_id) = current_dragged {
            if source_id != target_id {
                set_drag_over_id.set(Some(target_id.clone()));
                set_room_data.update(|data| {
                    if let Some(room) = data {
                        let from_pos = room.sheets.iter().position(|s| s.id == source_id);
                        let to_pos = room.sheets.iter().position(|s| s.id == target_id);
                        if let (Some(from), Some(to)) = (from_pos, to_pos) {
                            let item = room.sheets.remove(from);
                            room.sheets.insert(to, item);
                        }
                    }
                });
            }
        }
    };

    let on_drag_end = move || {
        let current_dragged = dragged_sheet_id.get_untracked();
        set_dragged_sheet_id.set(None);
        set_drag_over_id.set(None);

        if current_dragged.is_some() {
            let r_id = room_id();
            if !r_id.is_empty() {
                let current_order: Vec<String> = room_data.with_untracked(|data| {
                    data.as_ref()
                        .map(|r| r.sheets.iter().map(|s| s.id.clone()).collect())
                        .unwrap_or_default()
                });
                if !current_order.is_empty() {
                    spawn_local(async move {
                        let _ = update_room_sheet_order(r_id, current_order).await;
                    });
                }
            }
        }
    };

    let on_drop = move |_target_id: String| {
        on_drag_end();
    };

    let on_touch_move_coord = move |client_x: i32, client_y: i32| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(el) = document.element_from_point(client_x as f32, client_y as f32) {
                        if let Ok(Some(card)) = el.closest("[data-sheet-id]") {
                            if let Some(target_id) = card.get_attribute("data-sheet-id") {
                                on_drag_over(target_id);
                            }
                        }
                    }
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (client_x, client_y);
        }
    };


    let is_gm_sig = Signal::derive(move || room_data.with(|r| r.as_ref().map(|x| x.is_gm).unwrap_or(false)));
    let room_name_sig = Signal::derive(move || room_data.with(|r| r.as_ref().map(|x| x.name.clone()).unwrap_or_default()));
    let room_code_sig = Signal::derive(move || room_data.with(|r| r.as_ref().map(|x| x.code.clone()).unwrap_or_default()));
    let gm_username_sig = Signal::derive(move || room_data.with(|r| r.as_ref().map(|x| x.gm_username.clone()).unwrap_or_default()));
    let description_sig = Signal::derive(move || room_data.with(|r| r.as_ref().map(|x| x.description.clone()).unwrap_or_default()));
    let is_public_sig = Signal::derive(move || room_data.with(|r| r.as_ref().map(|x| x.is_public).unwrap_or(false)));
    let has_password_sig = Signal::derive(move || room_data.with(|r| r.as_ref().map(|x| x.has_password).unwrap_or(false)));
    let room_sheets_sig = Signal::derive(move || room_data.with(|r| r.as_ref().map(|x| x.sheets.clone()).unwrap_or_default()));
    let room_members_sig = Signal::derive(move || room_data.with(|r| r.as_ref().map(|x| x.members.clone()).unwrap_or_default()));
    let initiative_sig = Signal::derive(move || room_data.with(|r| r.as_ref().map(|x| x.initiative.clone()).unwrap_or_default()));
    let room_id_sig = Signal::derive(move || room_id());

    let on_copy = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let code = room_code_sig.get();
            if let Some(window) = web_sys::window() {
                let _ = window.navigator().clipboard().write_text(&code);
            }
        }
        set_copied_code.set(true);
    };

    let on_copy_link = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(href) = window.location().href() {
                    let _ = window.navigator().clipboard().write_text(&href);
                }
            }
        }
        set_copied_link.set(true);
    };

    view! {
        <div class="room-view-page">
            <div class="room-view-container">
            {move || if !is_loaded.get() {
                if let Some(err) = load_error.get() {
                    view! {
                        <div class="error-container">
                            <p>{err}</p>
                            <A href="/rooms">"Voltar para Salas"</A>
                        </div>
                    }.into_view()
                } else {
                    view! { <p class="loading-msg">"Carregando mesa da crônica..."</p> }.into_view()
                }
            } else {
                view! {
                    <div class="room-dashboard">
                        <header class="room-top-header">
                            <div class="header-left">
                                <A href="/rooms" class="back-link">"← Voltar para Minhas Salas"</A>
                                <div class="room-title-row">
                                    <h1 class="room-title">{move || room_name_sig.get()}</h1>
                                    <div class="room-badge-group">
                                        {move || if is_public_sig.get() {
                                            view! { <span class="room-privacy-tag public">"🌐 Pública"</span> }.into_view()
                                        } else {
                                            view! { <span class="room-privacy-tag private">"🔒 Privada"</span> }.into_view()
                                        }}
                                        {move || if has_password_sig.get() {
                                            view! { <span class="room-pwd-tag" title="Mesa Protegida por Senha">"🔑 Com Senha"</span> }.into_view()
                                        } else {
                                            view! {}.into_view()
                                        }}
                                    </div>
                                </div>
                                <p class="room-subtitle">
                                    "Narrador: " <strong>{move || gm_username_sig.get()}</strong>
                                    {move || {
                                        let desc = description_sig.get();
                                        if !desc.is_empty() {
                                            format!(" • {}", desc)
                                        } else {
                                            "".to_string()
                                        }
                                    }}
                                </p>
                            </div>

                            <div class="header-right">
                                <div class="invite-code-pill">
                                    <span class="invite-label">"Código:"</span>
                                    <span class="invite-code">{move || room_code_sig.get()}</span>
                                    <button class="copy-btn" on:click=on_copy title="Copiar Código de Entrada">{move || if copied_code.get() { "✓" } else { "Copiar" }}</button>
                                </div>
                                <button
                                    class="copy-btn copy-room-link-btn"
                                    on:click=on_copy_link
                                    title="Copiar link direto de compartilhamento da mesa"
                                >
                                    {move || if copied_link.get() { "✓ Link Copiado" } else { "🔗 Copiar Link" }}
                                </button>
                                <button
                                    class="initiative-top-bar-btn"
                                    on:click=move |_| set_show_initiative.update(|v| *v = !*v)
                                    title="Abrir Rastreador de Iniciativa de Combate"
                                >
                                    "⚔️ Iniciativa"
                                </button>
                                <button class="add-sheet-to-room-btn" on:click=move |_| set_show_assign_modal.set(true)>"+ Adicionar Ficha"</button>
                            </div>
                        </header>

                        {move || feedback_msg.get().map(|msg| view! { <div class="alert-box alert-success"><span>{msg}</span><button class="alert-close" on:click=move |_| set_feedback_msg.set(None)>"×"</button></div> })}
                        {move || error_msg.get().map(|msg| view! { <div class="alert-box alert-error"><span>{msg}</span><button class="alert-close" on:click=move |_| set_error_msg.set(None)>"×"</button></div> })}

                        <nav class="room-tabs-nav">
                            <button class="room-tab-btn" class:active=move || active_tab.get() == "party" on:click=move |_| set_active_tab.set("party")>
                                "👥 Personagens & HUD (" {move || room_sheets_sig.get().len()} ")"
                            </button>
                            {move || if is_map_enabled.get() {
                                view! {
                                    <button class="room-tab-btn" class:active=move || active_tab.get() == "map" on:click=move |_| set_active_tab.set("map")>
                                        "🗺️ Mapa & Grid Tático"
                                    </button>
                                }.into_view()
                            } else {
                                view! {}.into_view()
                            }}
                            <button class="room-tab-btn" class:active=move || active_tab.get() == "chantry" on:click=move |_| set_active_tab.set("chantry")>
                                "🏛️ Capela & Recursos"
                            </button>
                            <button class="room-tab-btn" class:active=move || active_tab.get() == "chronicle" on:click=move |_| set_active_tab.set("chronicle")>
                                "📜 Diário & Mural"
                            </button>
                            <button class="room-tab-btn" class:active=move || active_tab.get() == "members" on:click=move |_| set_active_tab.set("members")>
                                "🧙 Membros (" {move || room_members_sig.get().len()} ")"
                            </button>
                            {move || if is_gm_sig.get() {
                                view! {
                                    <button
                                        class="room-tab-btn"
                                        class:active=move || active_tab.get() == "settings"
                                        on:click=move |_| set_active_tab.set("settings")
                                    >
                                        "⚙️ Configurações da Mesa"
                                    </button>
                                }.into_view()
                            } else {
                                view! {}.into_view()
                            }}
                        </nav>

                        <div class="room-tab-content">
                            {move || if active_tab.get() == "party" {
                                let current_lang = lang();
                                view! {
                                    <section class="room-characters-section">
                                        <div class="section-header">
                                            <h2>"HUD da Cabala & Personagens"</h2>
                                            <span class="count-badge">{move || format!("{} personagens", room_sheets_sig.get().len())}</span>
                                        </div>
                                        {move || {
                                            let sheets = room_sheets_sig.get();
                                            let current_dragged = dragged_sheet_id.get();
                                            let current_over = drag_over_id.get();
                                            if sheets.is_empty() {
                                                view! { <div class="empty-room-sheets"><h3>"Nenhum personagem aqui"</h3><button class="action-link-btn" on:click=move |_| set_show_assign_modal.set(true)>"+ Vincular Ficha"</button></div> }.into_view()
                                            } else {
                                                view! {
                                                    <div class="character-cards-grid party-grid">
                                                        {sheets.into_iter().map(|sheet| {
                                                            let is_gm = is_gm_sig.get();
                                                            let is_dragging = current_dragged.as_deref() == Some(&sheet.id);
                                                            let is_drag_over = current_over.as_deref() == Some(&sheet.id);
                                                            let sheet_m = sheet.clone();
                                                            view! {
                                                                <PartyCardDesktop
                                                                    sheet=sheet
                                                                    is_gm=is_gm
                                                                    current_lang=current_lang
                                                                    is_dragging=is_dragging
                                                                    is_drag_over=is_drag_over
                                                                    on_quick_stat=on_quick_stat
                                                                    on_toggle_visibility=on_toggle_visibility
                                                                    on_remove_sheet=on_remove_sheet
                                                                    on_drag_start=on_drag_start
                                                                    on_drag_over=on_drag_over
                                                                    on_drop=on_drop
                                                                    on_drag_end=on_drag_end
                                                                />
                                                                <PartyCardMobile
                                                                    sheet=sheet_m
                                                                    is_gm=is_gm
                                                                    current_lang=current_lang
                                                                    is_dragging=is_dragging
                                                                    is_drag_over=is_drag_over
                                                                    on_quick_stat=on_quick_stat
                                                                    on_toggle_visibility=on_toggle_visibility
                                                                    on_remove_sheet=on_remove_sheet
                                                                    on_drag_start=on_drag_start
                                                                    on_drag_over=on_drag_over
                                                                    on_drop=on_drop
                                                                    on_drag_end=on_drag_end
                                                                    on_touch_move_coord=on_touch_move_coord
                                                                />
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                }.into_view()
                                            }
                                        }}
                                    </section>
                                }.into_view()
                            } else if active_tab.get() == "map" && is_map_enabled.get() {
                                view! {
                                    <section class="room-map-tab-section">
                                        <BattleGrid
                                            room_id=room_id_sig
                                            is_gm=is_gm_sig
                                            map_data=map_data.into()
                                            set_map_data=set_map_data
                                            room_sheets=room_sheets_sig
                                        />
                                    </section>
                                }.into_view()
                            } else if active_tab.get() == "chantry" {
                                view! {
                                    <section class="room-chantry-section">
                                        <div class="section-header"><h2>"Santuário da Cabala"</h2><button class="save-chantry-btn" on:click=on_save_chantry>"Salvar Capela"</button></div>
                                        <div class="chantry-grid">
                                            <div class="chantry-card">
                                                <h3>"Quintessência"</h3>
                                                <div class="pool-numbers">{move || chantry_quint.get()}"/"{move || chantry_max_quint.get()}</div>
                                                <div class="pool-actions">
                                                    <button on:click=move |_| set_chantry_quint.update(|q| *q = (*q - 1).max(0))>"-"</button>
                                                    <button on:click=move |_| set_chantry_quint.update(|q| *q = (*q + 1).min(chantry_max_quint.get()))>"+"</button>
                                                </div>
                                            </div>
                                            <div class="chantry-card">
                                                <h3>"Localização / Nome"</h3>
                                                <input type="text" prop:value=move || chantry_loc.get() on:input=move |e| set_chantry_loc.set(event_target_value(&e))/>
                                            </div>
                                            <div class="chantry-card chantry-card-full">
                                                <h3>"Notas e Recursos Comuns"</h3>
                                                <textarea rows="5" class="chantry-textarea" prop:value=move || chantry_notes.get() on:input=move |e| set_chantry_notes.set(event_target_value(&e))></textarea>
                                            </div>
                                        </div>
                                    </section>
                                }.into_view()
                            } else if active_tab.get() == "chronicle" {
                                view! {
                                    <section class="room-chronicle-section">
                                        <div class="section-header">
                                            <h2>"Diário da Crônica & Mural"</h2>
                                            <button class="save-chantry-btn" on:click=on_save_chronicle>"Salvar Diário"</button>
                                        </div>
                                        <textarea rows="14" class="chronicle-textarea" prop:value=move || chronicle_text.get() on:input=move |e| set_chronicle_text.set(event_target_value(&e))></textarea>
                                    </section>
                                }.into_view()
                            } else if active_tab.get() == "settings" && is_gm_sig.get() {
                                let r_id = room_id();
                                let on_save_settings = move |ev: ev::SubmitEvent| {
                                    ev.prevent_default();
                                    if r_id.is_empty() { return; }
                                    let is_pub = settings_is_public.get();
                                    let pwd_val = settings_new_password.get().trim().to_string();
                                    let pwd_opt = if pwd_val.is_empty() { None } else { Some(pwd_val) };
                                    let remove_pwd = settings_remove_password.get();

                                    set_is_saving_settings.set(true);
                                    let r_id_clone = r_id.clone();
                                    spawn_local(async move {
                                        match update_room_settings(r_id_clone, is_pub, pwd_opt, remove_pwd).await {
                                            Ok(_) => {
                                                set_feedback_msg.set(Some("Configurações da crônica salvas com sucesso!".to_string()));
                                                set_is_saving_settings.set(false);
                                                set_settings_new_password.set(String::new());
                                                set_settings_remove_password.set(false);
                                                room_resource.refetch();
                                            }
                                            Err(e) => {
                                                set_error_msg.set(Some(e.to_string()));
                                                set_is_saving_settings.set(false);
                                            }
                                        }
                                    });
                                };

                                view! {
                                    <section class="room-settings-section">
                                        <div class="section-header">
                                            <h2>"⚙️ Configurações da Mesa"</h2>
                                            <p class="section-subtitle">"Gerencie a visibilidade na comunidade e a senha de proteção desta crônica."</p>
                                        </div>

                                        <div class="settings-card-container">
                                            <form on:submit=on_save_settings class="room-settings-tab-form">
                                                <div class="settings-option-card">
                                                    <div class="settings-option-info">
                                                        <strong class="settings-option-title">"🌐 Visibilidade no Mural Público"</strong>
                                                        <p class="settings-option-desc">
                                                            "Quando ativado, sua mesa é listada na aba 'Explorar Mesas Públicas' para qualquer jogador poder encontrá-la."
                                                        </p>
                                                    </div>
                                                    <label class="settings-toggle-label">
                                                        <input
                                                            type="checkbox"
                                                            class="room-checkbox-styled"
                                                            checked=settings_is_public
                                                            on:change=move |ev| set_settings_is_public.set(event_target_checked(&ev))
                                                            disabled=is_saving_settings
                                                        />
                                                        <span class="toggle-text">
                                                            {move || if settings_is_public.get() { "Mesa Pública (Visível)" } else { "Mesa Privada (Oculta)" }}
                                                        </span>
                                                    </label>
                                                </div>

                                                <div class="settings-option-card">
                                                    <div class="settings-option-info">
                                                        <strong class="settings-option-title">"🔒 Senha de Acesso à Mesa"</strong>
                                                        <p class="settings-option-desc">
                                                            {move || if has_password_sig.get() {
                                                                "Esta mesa está protegida por senha (criptografia bcrypt). Novos membros precisam inseri-la para entrar."
                                                            } else {
                                                                "Esta mesa não possui senha ativa no momento. Qualquer pessoa com o código ou via lista pública pode ingressar."
                                                            }}
                                                        </p>
                                                    </div>
                                                    <div class="settings-pwd-inputs">
                                                        <input
                                                            type="password"
                                                            class="room-input settings-tab-pwd-input"
                                                            placeholder="Digite nova senha para a sala (ou deixe em branco)..."
                                                            prop:value=settings_new_password
                                                            on:input=move |ev| set_settings_new_password.set(event_target_value(&ev))
                                                            disabled=is_saving_settings
                                                        />
                                                        {move || if has_password_sig.get() {
                                                            view! {
                                                                <label class="room-checkbox-label remove-pwd-chk">
                                                                    <input
                                                                        type="checkbox"
                                                                        class="room-checkbox"
                                                                        checked=settings_remove_password
                                                                        on:change=move |ev| set_settings_remove_password.set(event_target_checked(&ev))
                                                                        disabled=is_saving_settings
                                                                    />
                                                                    <span>"🔓 Remover senha atual da mesa"</span>
                                                                </label>
                                                            }.into_view()
                                                        } else {
                                                            view! {}.into_view()
                                                        }}
                                                    </div>
                                                </div>

                                                <div class="settings-submit-box">
                                                    <button type="submit" class="save-chantry-btn settings-save-btn" disabled=is_saving_settings>
                                                        {move || if is_saving_settings.get() { "Salvando..." } else { "💾 Salvar Configurações da Mesa" }}
                                                    </button>
                                                </div>
                                            </form>
                                        </div>
                                    </section>
                                }.into_view()
                            } else {
                                view! {
                                    <section class="room-members-section">
                                        <h2>"Membros da Crônica"</h2>
                                        <div class="members-pills">
                                            {move || {
                                                let members = room_members_sig.get();
                                                let is_user_gm = is_gm_sig.get();
                                                members.into_iter().map(|m| {
                                                    let is_narrator = m.role == "gm";
                                                    let m_member = m.clone();
                                                    view! {
                                                        <div class="member-pill" class:gm=is_narrator>
                                                            <span class="member-avatar">{if is_narrator { "👑" } else { "🧙" }}</span>
                                                            <span class="member-name">{m.username.clone()}</span>
                                                            <span class="member-role">{if is_narrator { "Narrador" } else { "Jogador" }}</span>
                                                            {if is_user_gm {
                                                                view! {
                                                                    <button
                                                                        type="button"
                                                                        class="gift-sheet-btn"
                                                                        on:click=move |_| {
                                                                            set_selected_clone_sheet_id.set(String::new());
                                                                            set_target_clone_member.set(Some(m_member.clone()));
                                                                        }
                                                                        title=format!("Clonar e entregar uma das suas fichas para {}", m.username)
                                                                    >
                                                                        "🎁 Entregar Ficha"
                                                                    </button>
                                                                }.into_view()
                                                            } else {
                                                                view! {}.into_view()
                                                            }}
                                                        </div>
                                                    }
                                                }).collect_view()
                                            }}
                                        </div>
                                    </section>
                                }.into_view()
                            }}
                        </div>
                    </div>
                }.into_view()
            }}

            {move || if show_assign_modal.get() {
                view! {
                    <div class="modal-overlay" on:click=move |_| set_show_assign_modal.set(false)>
                        <div class="modal-content" on:click=move |e| e.stop_propagation()>
                            <h3>"Vincular Ficha à Mesa"</h3>
                            {move || match my_sheets_resource.get() {
                                None => view! { <p>"Carregando suas fichas..."</p> }.into_view(),
                                Some(Ok(sheets)) => sheets.into_iter().map(|s| {
                                    let s_id = s.id.clone();
                                    view! {
                                        <div class="sheet-item" on:click=move |_| set_selected_sheet_id.set(s_id.clone())>
                                            <strong>{s.name}</strong>" - "{s.tradition}
                                        </div>
                                    }
                                }).collect_view(),
                                Some(Err(_)) => view! {}.into_view(),
                            }}
                            <div style="margin-top: 1rem; display: flex; gap: 0.5rem; justify-content: flex-end;">
                                <button class="btn-secondary" on:click=move |_| set_show_assign_modal.set(false)>"Cancelar"</button>
                                <button class="save-chantry-btn" on:click=on_assign_sheet>"Confirmar"</button>
                            </div>
                        </div>
                    </div>
                }.into_view()
            } else { view! {}.into_view() }}

            // Modal de Clonagem e Entrega de Ficha pelo Narrador
            {move || if let Some(target) = target_clone_member.get() {
                let target_name = target.username.clone();
                view! {
                    <div class="modal-overlay" on:click=move |_| set_target_clone_member.set(None)>
                        <div class="modal-content" on:click=move |e| e.stop_propagation()>
                            <h3>"🎁 Entregar Ficha para " {target_name.clone()}</h3>
                            <p style="color: #64748b; font-size: 0.88rem; margin: 0.25rem 0 1rem 0;">
                                "Selecione uma das suas fichas para clonar e entregar uma cópia independente na conta deste jogador:"
                            </p>
                            <div class="sheets-selection-list">
                                {move || match my_sheets_resource.get() {
                                    None => view! { <p>"Carregando suas fichas..."</p> }.into_view(),
                                    Some(Ok(sheets)) if sheets.is_empty() => view! {
                                        <p style="color: #94a3b8; font-style: italic;">"Você ainda não possui fichas criadas para clonar."</p>
                                    }.into_view(),
                                    Some(Ok(sheets)) => sheets.into_iter().map(|s| {
                                        let s_id = s.id.clone();
                                        let s_id_active = s_id.clone();
                                        let s_id_check = s_id.clone();
                                        let s_id_click = s_id.clone();
                                        let sheet_type_label = if s.tradition.is_empty() { s.sheet_type } else { s.tradition };
                                        view! {
                                            <div 
                                                class="sheet-item"
                                                class:active=move || selected_clone_sheet_id.get() == s_id_active
                                                on:click=move |_| set_selected_clone_sheet_id.set(s_id_click.clone())
                                            >
                                                <div class="sheet-item-info">
                                                    <strong>{s.name}</strong>
                                                    <span class="sheet-item-meta">{sheet_type_label}</span>
                                                </div>
                                                {move || if selected_clone_sheet_id.get() == s_id_check {
                                                    view! { <span class="selected-check">"✓ Selecionada"</span> }.into_view()
                                                } else {
                                                    view! {}.into_view()
                                                }}
                                            </div>
                                        }
                                    }).collect_view(),
                                    Some(Err(e)) => view! { <p style="color: #ef4444;">"Erro ao listar fichas: " {e.to_string()}</p> }.into_view(),
                                }}
                            </div>
                            <div style="margin-top: 1rem; display: flex; gap: 0.5rem; justify-content: flex-end;">
                                <button class="btn-secondary" on:click=move |_| set_target_clone_member.set(None)>"Cancelar"</button>
                                <button 
                                    class="save-chantry-btn" 
                                    disabled=move || selected_clone_sheet_id.get().is_empty() || is_cloning.get()
                                    on:click=on_clone_and_assign
                                >
                                    {move || if is_cloning.get() { "Clonando..." } else { "Confirmar Entrega" }}
                                </button>
                            </div>
                        </div>
                    </div>
                }.into_view()
            } else { view! {}.into_view() }}

            <InitiativeDrawer
                is_open=show_initiative
                set_is_open=set_show_initiative
                room_id=room_id_sig
                initiative=initiative_sig
                sheets=room_sheets_sig
                is_gm=is_gm_sig
            />
            </div>
        </div>
    }
}
