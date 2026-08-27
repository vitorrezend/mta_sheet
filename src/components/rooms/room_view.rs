use leptos::*;
use leptos_router::*;
use crate::rooms::{
    get_room_details, assign_sheet_to_room, remove_sheet_from_room,
    toggle_sheet_room_visibility, update_room_chantry, update_room_chronicle_notes,
    clone_and_assign_sheet_to_member, ChantryPoolData, RoomMemberInfo,
};
use crate::state::get_sheets;
use crate::components::Navbar;

#[component]
pub fn RoomView() -> impl IntoView {
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

        let interval = gloo_timers::callback::Interval::new(15_000, move || {
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

    let (active_tab, set_active_tab) = create_signal("party");
    let (show_assign_modal, set_show_assign_modal) = create_signal(false);
    let (selected_sheet_id, set_selected_sheet_id) = create_signal(String::new());
    let (target_clone_member, set_target_clone_member) = create_signal(Option::<RoomMemberInfo>::None);
    let (selected_clone_sheet_id, set_selected_clone_sheet_id) = create_signal(String::new());
    let (is_cloning, set_is_cloning) = create_signal(false);
    let (copied_code, set_copied_code) = create_signal(false);
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

    create_effect(move |_| {
        if let Some(Ok(room)) = room_resource.get() {
            if !chantry_initialized.get() {
                set_chantry_loc.set(room.chantry.location_name.clone());
                set_chantry_node.set(room.chantry.node_rating);
                set_chantry_library.set(room.chantry.library_rating);
                set_chantry_quint.set(room.chantry.quintessence_pool);
                set_chantry_max_quint.set(if room.chantry.max_quintessence > 0 { room.chantry.max_quintessence } else { 20 });
                set_chantry_notes.set(room.chantry.notes.clone());
                set_chantry_initialized.set(true);
            }
            if !chronicle_initialized.get() {
                set_chronicle_text.set(room.chronicle_notes.clone());
                set_chronicle_initialized.set(true);
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

    view! {
        <div class="room-view-page">
            <Navbar />
            <div class="room-view-container">
            {move || match room_resource.get() {
                None => view! { <p class="loading-msg">"Carregando mesa da crônica..."</p> }.into_view(),
                Some(Ok(room)) => {
                    let _code_copy = room.code.clone();
                    let on_copy = move |_| {
                        #[cfg(target_arch = "wasm32")]
                        {
                            let code = _code_copy.clone();
                            if let Some(window) = web_sys::window() {
                                let _ = window.navigator().clipboard().write_text(&code);
                            }
                        }
                        set_copied_code.set(true);
                    };

                    let is_gm = room.is_gm;
                    let member_count = room.members.len();
                    let sheet_count = room.sheets.len();

                    view! {
                        <div class="room-dashboard">
                            <header class="room-top-header">
                                <div class="header-left">
                                    <A href="/rooms" class="back-link">"← Voltar para Minhas Salas"</A>
                                    <h1 class="room-title">{room.name.clone()}</h1>
                                    <p class="room-subtitle">
                                        "Narrador: " <strong>{room.gm_username.clone()}</strong>
                                        {if !room.description.is_empty() {
                                            format!(" • {}", room.description)
                                        } else {
                                            "".to_string()
                                        }}
                                    </p>
                                </div>

                                <div class="header-right">
                                    <div class="invite-code-pill">
                                        <span class="invite-label">"Código:"</span>
                                        <span class="invite-code">{room.code.clone()}</span>
                                        <button class="copy-btn" on:click=on_copy>{move || if copied_code.get() { "✓" } else { "Copiar" }}</button>
                                    </div>
                                    <button class="add-sheet-to-room-btn" on:click=move |_| set_show_assign_modal.set(true)>"+ Adicionar Ficha"</button>
                                </div>
                            </header>

                            {move || feedback_msg.get().map(|msg| view! { <div class="alert-box alert-success"><span>{msg}</span><button class="alert-close" on:click=move |_| set_feedback_msg.set(None)>"×"</button></div> })}
                            {move || error_msg.get().map(|msg| view! { <div class="alert-box alert-error"><span>{msg}</span><button class="alert-close" on:click=move |_| set_error_msg.set(None)>"×"</button></div> })}

                            <nav class="room-tabs-nav">
                                <button class="room-tab-btn" class:active=move || active_tab.get() == "party" on:click=move |_| set_active_tab.set("party")>"👥 Personagens & HUD (" {sheet_count} ")"</button>
                                <button class="room-tab-btn" class:active=move || active_tab.get() == "chantry" on:click=move |_| set_active_tab.set("chantry")>"🏛️ Capela & Recursos"</button>
                                <button class="room-tab-btn" class:active=move || active_tab.get() == "chronicle" on:click=move |_| set_active_tab.set("chronicle")>"📜 Diário & Mural"</button>
                                <button class="room-tab-btn" class:active=move || active_tab.get() == "members" on:click=move |_| set_active_tab.set("members")>"🧙 Membros (" {member_count} ")"</button>
                            </nav>

                            <div class="room-tab-content">
                                {move || if active_tab.get() == "party" {
                                    view! {
                                        <section class="room-characters-section">
                                            <div class="section-header">
                                                <h2>"HUD da Cabala & Personagens"</h2>
                                                <span class="count-badge">{room.sheets.len()} " personagens"</span>
                                            </div>
                                            {if room.sheets.is_empty() {
                                                view! { <div class="empty-room-sheets"><h3>"Nenhum personagem aqui"</h3><button class="action-link-btn" on:click=move |_| set_show_assign_modal.set(true)>"+ Vincular Ficha"</button></div> }.into_view()
                                            } else {
                                                view! {
                                                    <div class="party-grid">
                                                        {room.sheets.iter().map(|sheet| {
                                                            let s_id = sheet.id.clone();
                                                            let remove_id = sheet.id.clone();
                                                            let toggle_id = sheet.id.clone();
                                                            let is_hidden = sheet.is_hidden;
                                                            let can_toggle = is_gm || sheet.is_owner;
                                                            let badge_cls = format!("party-health-badge {}", sheet.health_badge_class);
                                                            view! {
                                                                <div class="party-card" class:party-card-hidden=is_hidden>
                                                                    <div class="party-card-header"><h3 class="char-name">{sheet.name.clone()}</h3><span class="char-player">{sheet.player_name.clone()}</span></div>
                                                                    <div class=badge_cls><span>"Saúde: " {sheet.health_label.clone()}</span></div>
                                                                    <div class="party-stats-grid"><div class="stat-box"><span>"Arete"</span><span>{sheet.arete}</span></div><div class="stat-box"><span>"Vontade"</span><span>{sheet.willpower_current}"/"{sheet.willpower_total}</span></div></div>
                                                                    <div class="party-card-footer">
                                                                        <A href=format!("/sheet/{}", s_id) class="open-sheet-btn">"Abrir Ficha"</A>
                                                                        {if can_toggle { view! { <button class="visibility-toggle-btn" on:click=move |_| on_toggle_visibility(toggle_id.clone(), is_hidden)>{if is_hidden { "Revelar" } else { "Ocultar" }}</button> }.into_view() } else { view! {}.into_view() }}
                                                                        {if is_gm || sheet.is_owner { view! { <button class="unlink-btn" on:click=move |_| on_remove_sheet(remove_id.clone())>"Desvincular"</button> }.into_view() } else { view! {}.into_view() }}
                                                                    </div>
                                                                </div>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                }.into_view()
                                            }}
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
                                } else {
                                    view! {
                                        <section class="room-members-section">
                                            <h2>"Membros da Crônica"</h2>
                                            <div class="members-pills">
                                                {room.members.iter().map(|m| {
                                                    let is_narrator = m.role == "gm";
                                                    let m_member = m.clone();
                                                    view! {
                                                        <div class="member-pill" class:gm=is_narrator>
                                                            <span class="member-avatar">{if is_narrator { "👑" } else { "🧙" }}</span>
                                                            <span class="member-name">{m.username.clone()}</span>
                                                            <span class="member-role">{if is_narrator { "Narrador" } else { "Jogador" }}</span>
                                                            {if is_gm {
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
                                                }).collect_view()}
                                            </div>
                                        </section>
                                    }.into_view()
                                }}
                            </div>
                        </div>
                    }.into_view()
                },
                Some(Err(e)) => view! { <div class="error-container"><p>{e.to_string()}</p><A href="/rooms">"Voltar para Salas"</A></div> }.into_view(),
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
            </div>
        </div>
    }
}
