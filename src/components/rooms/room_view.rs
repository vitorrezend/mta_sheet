use leptos::*;
use leptos_router::*;
use crate::rooms::{get_room_details, assign_sheet_to_room, remove_sheet_from_room};
use crate::state::{get_sheets, CharacterSummary};
use crate::components::Navbar;

#[component]
pub fn RoomView() -> impl IntoView {
    let params = use_params_map();
    let room_id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    let room_resource = create_resource(room_id, |id| async move {
        if id.is_empty() {
            return Err(ServerFnError::new("ID da sala não fornecido"));
        }
        get_room_details(id).await
    });

    let my_sheets_resource = create_resource(|| (), |_| async move { get_sheets().await });

    let (show_assign_modal, set_show_assign_modal) = create_signal(false);
    let (selected_sheet_id, set_selected_sheet_id) = create_signal(String::new());
    let (copied_code, set_copied_code) = create_signal(false);
    let (error_msg, set_error_msg) = create_signal(Option::<String>::None);

    let on_assign_sheet = move |_| {
        let s_id = selected_sheet_id.get();
        let r_id = room_id();
        if s_id.is_empty() || r_id.is_empty() {
            return;
        }

        spawn_local(async move {
            match assign_sheet_to_room(s_id, r_id).await {
                Ok(_) => {
                    set_show_assign_modal.set(false);
                    room_resource.refetch();
                }
                Err(e) => {
                    set_error_msg.set(Some(e.to_string()));
                }
            }
        });
    };

    let on_remove_sheet = move |sheet_id: String| {
        spawn_local(async move {
            match remove_sheet_from_room(sheet_id).await {
                Ok(_) => {
                    room_resource.refetch();
                }
                Err(e) => {
                    set_error_msg.set(Some(e.to_string()));
                }
            }
        });
    };

    view! {
        <link rel="stylesheet" href="/style.css"/>
        <Navbar />
        <div class="room-view-container">
            <Suspense fallback=move || view! { <p class="loading-msg">"Carregando mesa..."</p> }>
                {move || room_resource.get().map(|res| match res {
                    Ok(room) => {
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
                        view! {
                            <div class="room-dashboard">
                                <header class="room-top-header">
                                    <div class="header-left">
                                        <A href="/rooms" class="back-link">"← Voltar para Salas"</A>
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
                                            <span class="invite-label">"Código de Convite:"</span>
                                            <span class="invite-code">{room.code.clone()}</span>
                                            <button 
                                                class="copy-btn" 
                                                on:click=on_copy
                                                title="Copiar código para compartilhar com os jogadores"
                                            >
                                                {move || if copied_code.get() { "✓ Copiado!" } else { "Copiar" }}
                                            </button>
                                        </div>

                                        <button class="add-sheet-to-room-btn" on:click=move |_| set_show_assign_modal.set(true)>
                                            "+ Adicionar Ficha à Mesa"
                                        </button>
                                    </div>
                                </header>

                                {move || error_msg.get().map(|msg| view! {
                                    <div class="alert-box alert-error">
                                        <span>{msg}</span>
                                        <button class="alert-close" on:click=move |_| set_error_msg.set(None)>"×"</button>
                                    </div>
                                })}

                                // Characters in the room
                                <section class="room-characters-section">
                                    <div class="section-header">
                                        <h2>"Personagens da Mesa"</h2>
                                        <span class="count-badge">{room.sheets.len()} " personagens"</span>
                                    </div>

                                    {if room.sheets.is_empty() {
                                        view! {
                                            <div class="empty-room-sheets">
                                                <p>"Nenhum personagem vinculado a esta crônica ainda."</p>
                                                <button class="action-link-btn" on:click=move |_| set_show_assign_modal.set(true)>
                                                    "Clique aqui para vincular uma ficha existente"
                                                </button>
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! {
                                            <div class="party-grid">
                                                {room.sheets.into_iter().map(|sheet| {
                                                    let sheet_id = sheet.id.clone();
                                                    let remove_id = sheet.id.clone();
                                                    view! {
                                                        <div class="party-card">
                                                            <div class="party-card-header">
                                                                <h3 class="char-name">{sheet.name}</h3>
                                                                <span class="char-player">{if sheet.player_name.is_empty() { "Jogador".to_string() } else { sheet.player_name }}</span>
                                                            </div>

                                                            <div class="party-card-tags">
                                                                {if !sheet.tradition.is_empty() {
                                                                    view! { <span class="tag tag-tradition">{sheet.tradition}</span> }.into_view()
                                                                } else {
                                                                    view! { <span class="tag tag-empty">"Sem Tradição"</span> }.into_view()
                                                                }}
                                                                {if !sheet.concept.is_empty() {
                                                                    view! { <span class="tag tag-concept">{sheet.concept}</span> }.into_view()
                                                                } else {
                                                                    view! { <span></span> }.into_view()
                                                                }}
                                                            </div>

                                                            <div class="party-stats-grid">
                                                                <div class="stat-box">
                                                                    <span class="stat-label">"Arete"</span>
                                                                    <span class="stat-value">{sheet.arete}</span>
                                                                </div>
                                                                <div class="stat-box">
                                                                    <span class="stat-label">"F. Vontade"</span>
                                                                    <span class="stat-value">{sheet.willpower_current} "/" {sheet.willpower_total}</span>
                                                                </div>
                                                                <div class="stat-box">
                                                                    <span class="stat-label">"Quint / Par"</span>
                                                                    <span class="stat-value">{sheet.quintessence} "/" {sheet.paradox}</span>
                                                                </div>
                                                            </div>

                                                            <div class="party-card-footer">
                                                                <A href=format!("/sheet/{}", sheet_id) class="open-sheet-btn">"Abrir Ficha ↗"</A>
                                                                <button 
                                                                    class="unlink-btn" 
                                                                    on:click=move |_| on_remove_sheet(remove_id.clone())
                                                                    title="Desvincular ficha desta mesa"
                                                                >
                                                                    "Desvincular"
                                                                </button>
                                                            </div>
                                                        </div>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        }.into_view()
                                    }}
                                </section>

                                // Room members
                                <section class="room-members-section">
                                    <h3>"Membros da Crônica (" {room.members.len()} ")"</h3>
                                    <div class="members-pills">
                                        {room.members.into_iter().map(|m| {
                                            let is_gm = m.role == "gm";
                                            view! {
                                                <div class="member-pill" class:gm=is_gm>
                                                    <span class="member-avatar">{if is_gm { "👑" } else { "🧙" }}</span>
                                                    <span class="member-name">{m.username}</span>
                                                    <span class="member-role">{if is_gm { "Narrador" } else { "Jogador" }}</span>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                </section>
                            </div>
                        }.into_view()
                    },
                    Err(e) => view! {
                        <div class="error-container">
                            <p class="error-title">"Erro ao carregar a sala"</p>
                            <p class="error-detail">{e.to_string()}</p>
                            <A href="/rooms" class="back-home-btn">"Voltar para a lista de salas"</A>
                        </div>
                    }.into_view(),
                })}
            </Suspense>

            // Assign sheet modal
            {move || show_assign_modal.get().then(|| view! {
                <div class="modal-overlay" on:click=move |_| set_show_assign_modal.set(false)>
                    <div class="modal-card" on:click=move |ev| ev.stop_propagation()>
                        <h3 class="modal-title">"Vincular Ficha a esta Mesa"</h3>
                        <p class="modal-text">"Selecione uma das suas fichas para torná-la visível na mesa:"</p>
                        
                        <Suspense fallback=move || view! { <p>"Carregando suas fichas..."</p> }>
                            {move || my_sheets_resource.get().map(|res| match res {
                                Ok(sheets) if sheets.is_empty() => view! {
                                    <p class="modal-subtext">"Você não possui nenhuma ficha criada ainda."</p>
                                }.into_view(),
                                Ok(sheets) => view! {
                                    <div class="form-group" style="margin: 1rem 0;">
                                        <select 
                                            class="form-input"
                                            on:change=move |ev| set_selected_sheet_id.set(event_target_value(&ev))
                                        >
                                            <option value="">"-- Selecione uma Ficha --"</option>
                                            {sheets.into_iter().map(|s: CharacterSummary| {
                                                view! {
                                                    <option value=s.id.clone()>{s.name}</option>
                                                }
                                            }).collect_view()}
                                        </select>
                                    </div>
                                }.into_view(),
                                Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_view(),
                            })}
                        </Suspense>

                        <div class="modal-actions">
                            <button class="modal-btn btn-cancel" on:click=move |_| set_show_assign_modal.set(false)>"Cancelar"</button>
                            <button 
                                class="modal-btn btn-primary" 
                                on:click=on_assign_sheet
                                disabled=move || selected_sheet_id.get().is_empty()
                            >
                                "Confirmar Vinculação"
                            </button>
                        </div>
                    </div>
                </div>
            })}
        </div>
    }
}
