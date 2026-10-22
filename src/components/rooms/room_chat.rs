use leptos::*;
use web_sys::wasm_bindgen::{closure::Closure, JsCast};
use crate::rooms::{
    RoomChatMessage, ChatRollData, RoomSheetSummary, RoomBroadcastEvent,
    get_room_chat_messages, send_room_chat_message, roll_room_dice, clear_room_chat,
};
use crate::components::common::play_dice_roll_sound;

#[component]
pub fn RoomChat(
    room_id: Signal<String>,
    sheets: Signal<Vec<RoomSheetSummary>>,
    is_gm: Signal<bool>,
) -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let _lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let messages = create_rw_signal(Vec::<RoomChatMessage>::new());
    let (input_text, set_input_text) = create_signal(String::new());
    let (selected_identity, set_selected_identity) = create_signal("player".to_string());
    let (show_quick_roll, set_show_quick_roll) = create_signal(false);
    let (is_sending, set_is_sending) = create_signal(false);

    // Estado do Rolador Rápido
    let (quick_dice, set_quick_dice) = create_signal(5i32);
    let (quick_diff, set_quick_diff) = create_signal(6i32);
    let (quick_specialty, set_quick_specialty) = create_signal(false);
    let (quick_label, set_quick_label) = create_signal(String::new());

    let chat_container_ref = create_node_ref::<html::Div>();

    // Scroll suave para a última mensagem
    let scroll_to_bottom = move || {
        request_animation_frame(move || {
            if let Some(el) = chat_container_ref.get() {
                el.set_scroll_top(el.scroll_height());
            }
        });
    };

    // 1. Carregamento inicial do histórico do chat
    let r_id_for_res = room_id.clone();
    let initial_msgs = create_local_resource(
        move || r_id_for_res.get(),
        |id| async move {
            if id.is_empty() {
                return Ok(Vec::new());
            }
            get_room_chat_messages(id).await
        }
    );

    create_effect(move |_| {
        if let Some(Ok(msgs)) = initial_msgs.get() {
            messages.set(msgs);
            scroll_to_bottom();
        }
    });

    // 2. Conexão SSE em tempo real para sincronização instantânea
    #[cfg(target_arch = "wasm32")]
    {
        create_effect(move |_| {
            let r_id = room_id.get();
            if r_id.is_empty() {
                return;
            }

            if let Ok(es) = web_sys::EventSource::new(&format!("/api/room_events/{}", r_id)) {
                let msgs_sig = messages;
                let onmessage_cb = Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |e: web_sys::MessageEvent| {
                    if let Some(text) = e.data().as_string() {
                        if let Ok(event) = serde_json::from_str::<RoomBroadcastEvent>(&text) {
                            if event.event_type == "CHAT_MESSAGE" || event.event_type == "DICE_ROLLED" {
                                if let Some(new_msg) = event.chat_message {
                                    msgs_sig.update(|list| {
                                        // Evita duplicação caso já exista por ID
                                        if !list.iter().any(|m| m.id == new_msg.id) {
                                            list.push(new_msg);
                                        }
                                    });
                                    if event.play_sound {
                                        play_dice_roll_sound();
                                    }
                                    request_animation_frame(move || {
                                        if let Some(el) = chat_container_ref.get() {
                                            el.set_scroll_top(el.scroll_height());
                                        }
                                    });
                                }
                            } else if event.event_type == "CHAT_CLEARED" {
                                msgs_sig.set(Vec::new());
                            }
                        }
                    }
                });

                es.set_onmessage(Some(onmessage_cb.as_ref().unchecked_ref()));
                onmessage_cb.forget();

                let es_clone = es.clone();
                on_cleanup(move || {
                    es_clone.close();
                });
            }
        });
    }

    // Helper para obter avatar e nome do personagem selecionado
    let get_current_sender_identity = move || {
        let ident = selected_identity.get();
        if ident == "player" {
            (None, None)
        } else {
            let found = sheets.get().into_iter().find(|s| s.id == ident);
            match found {
                Some(s) => {
                    let avatar = if s.photo_url.is_empty() { None } else { Some(s.photo_url) };
                    (Some(s.name), avatar)
                }
                None => (None, None),
            }
        }
    };

    // Ação: Enviar Mensagem de Texto ou Comando de Rolagem
    let handle_send_message = move || {
        let text = input_text.get().trim().to_string();
        if text.is_empty() || is_sending.get() {
            return;
        }

        let r_id = room_id.get();
        if r_id.is_empty() {
            return;
        }

        set_is_sending.set(true);
        let (char_name, char_avatar) = get_current_sender_identity();

        // Checa se é comando de rolagem: /r ou /roll
        if text.starts_with("/r ") || text.starts_with("/roll ") {
            let rest = if text.starts_with("/r ") { &text[3..] } else { &text[6..] }.trim();
            // Parser de rolagem simples: Ex: "5d10", "5d10 dif 7", "4d10 esp"
            let mut parts = rest.split_whitespace();
            let mut dice_count = 5i32;
            let mut diff = 6i32;
            let mut is_spec = false;
            let mut roll_label = String::new();

            if let Some(d_part) = parts.next() {
                let clean_d = d_part.to_lowercase().replace("d10", "").replace("d", "");
                if let Ok(cnt) = clean_d.parse::<i32>() {
                    dice_count = cnt;
                }
            }

            while let Some(arg) = parts.next() {
                let lower = arg.to_lowercase();
                if lower == "dif" || lower == "diff" {
                    if let Some(diff_val) = parts.next().and_then(|v| v.parse::<i32>().ok()) {
                        diff = diff_val;
                    }
                } else if lower == "esp" || lower == "spec" {
                    is_spec = true;
                } else {
                    if !roll_label.is_empty() {
                        roll_label.push(' ');
                    }
                    roll_label.push_str(arg);
                }
            }

            if roll_label.is_empty() {
                roll_label = format!("{}d10 vs Dif {}", dice_count, diff);
            }

            spawn_local(async move {
                let _ = roll_room_dice(r_id, dice_count, diff, is_spec, roll_label, char_name, char_avatar).await;
                set_input_text.set(String::new());
                set_is_sending.set(false);
            });
        } else {
            let msg_type = if char_name.is_some() { "ic".to_string() } else { "ooc".to_string() };
            spawn_local(async move {
                let _ = send_room_chat_message(r_id, text, char_name, char_avatar, msg_type).await;
                set_input_text.set(String::new());
                set_is_sending.set(false);
            });
        }
    };

    // Ação: Rolar via Interface do Rolador Rápido
    let handle_quick_roll = move |_| {
        let r_id = room_id.get();
        if r_id.is_empty() {
            return;
        }

        let count = quick_dice.get();
        let diff = quick_diff.get();
        let spec = quick_specialty.get();
        let label = quick_label.get();
        let (char_name, char_avatar) = get_current_sender_identity();

        spawn_local(async move {
            let _ = roll_room_dice(r_id, count, diff, spec, label, char_name, char_avatar).await;
            set_show_quick_roll.set(false);
            set_quick_label.set(String::new());
        });
    };

    view! {
        <div class="room-chat-wrapper">
            // ==========================================
            // HEADER DO CHAT / FERRAMENTAS
            // ==========================================
            <div class="room-chat-header">
                <div class="chat-header-left">
                    <span class="chat-title">"💬 Feed da Crônica & Rolagens"</span>
                    <span class="chat-count">
                        {move || format!("({} registros)", messages.get().len())}
                    </span>
                </div>
                <div class="chat-header-right">
                    <button 
                        type="button" 
                        class="chat-quick-roll-toggle-btn"
                        class:active=move || show_quick_roll.get()
                        on:click=move |_| set_show_quick_roll.update(|v| *v = !*v)
                    >
                        "🎲 Rolar Dados"
                    </button>
                    {move || if is_gm.get() {
                        view! {
                            <button 
                                type="button" 
                                class="chat-clear-btn"
                                title="Limpar histórico de mensagens da sala"
                                on:click=move |_| {
                                    let r_id = room_id.get();
                                    spawn_local(async move {
                                        let _ = clear_room_chat(r_id).await;
                                    });
                                }
                            >
                                "🗑️"
                            </button>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }}
                </div>
            </div>

            // ==========================================
            // GAVETA / POPOVER DO ROLADOR RÁPIDO
            // ==========================================
            {move || if show_quick_roll.get() {
                view! {
                    <div class="chat-quick-roll-panel">
                        <div class="quick-roll-row">
                            <div class="quick-field">
                                <label>"Quantidade de Dados (D10)"</label>
                                <div class="quick-number-stepper">
                                    <button type="button" on:click=move |_| set_quick_dice.update(|v| *v = (*v - 1).max(1))>"-"</button>
                                    <span class="stepper-val">{move || quick_dice.get()}</span>
                                    <button type="button" on:click=move |_| set_quick_dice.update(|v| *v = (*v + 1).min(30))>"+"</button>
                                </div>
                            </div>

                            <div class="quick-field">
                                <label>"Dificuldade Padrão (3-10)"</label>
                                <div class="quick-number-stepper">
                                    <button type="button" on:click=move |_| set_quick_diff.update(|v| *v = (*v - 1).max(3))>"-"</button>
                                    <span class="stepper-val">{move || quick_diff.get()}</span>
                                    <button type="button" on:click=move |_| set_quick_diff.update(|v| *v = (*v + 1).min(10))>"+"</button>
                                </div>
                            </div>

                            <div class="quick-field specialty-toggle">
                                <label class="checkbox-label">
                                    <input 
                                        type="checkbox" 
                                        prop:checked=move || quick_specialty.get()
                                        on:change=move |ev| set_quick_specialty.set(event_target_checked(&ev))
                                    />
                                    <span>"⭐ Especialidade (10s dobram)"</span>
                                </label>
                            </div>
                        </div>

                        <div class="quick-roll-row label-row">
                            <input 
                                type="text"
                                class="quick-label-input"
                                placeholder="Rótulo da ação (ex: Força + Briga, Arete, Percepção...)"
                                prop:value=move || quick_label.get()
                                on:input=move |ev| set_quick_label.set(event_target_value(&ev))
                            />
                            <button type="button" class="quick-roll-submit-btn" on:click=handle_quick_roll>
                                "🎲 Rolar D10"
                            </button>
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}

            // ==========================================
            // CONTAINER PRINCIPAL DE MENSAGENS / FEED
            // ==========================================
            <div class="room-chat-feed" node_ref=chat_container_ref>
                {move || {
                    let msgs = messages.get();
                    if msgs.is_empty() {
                        view! {
                            <div class="chat-empty-state">
                                <span class="empty-icon">"📜"</span>
                                <h4>"Nenhuma mensagem ou rolagem nesta sessão ainda."</h4>
                                <p>"Envie uma mensagem abaixo ou use o rolador de dados para inaugurar o feed da crônica!"</p>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <For
                                each=move || messages.get()
                                key=|m| m.id.clone()
                                children=move |msg: RoomChatMessage| {
                                    let is_roll = msg.message_type == "roll";
                                    let is_ic = msg.message_type == "ic";
                                                                        view! {
                                        <div 
                                            class="chat-bubble"
                                            class:bubble-roll=is_roll
                                            class:bubble-ic=is_ic
                                            class:bubble-ooc=!is_roll && !is_ic
                                        >
                                            <div class="bubble-header">
                                                <div class="sender-identity-group">
                                                    {if let Some(avatar) = msg.sender_avatar {
                                                        view! { <img class="sender-avatar" src=avatar alt="avatar" /> }.into_view()
                                                    } else {
                                                        view! { <div class="sender-avatar-placeholder">"👤"</div> }.into_view()
                                                    }}
                                                    <div class="sender-names">
                                                        {if let Some(char_name) = msg.sender_character {
                                                            view! {
                                                                <span class="char-name">{char_name}</span>
                                                                <span class="player-sub">"(" {msg.sender_name} ")"</span>
                                                            }.into_view()
                                                        } else {
                                                            view! {
                                                                <span class="player-name">{msg.sender_name}</span>
                                                            }.into_view()
                                                        }}
                                                    </div>
                                                </div>

                                                <div class="bubble-meta">
                                                    {if is_roll {
                                                        view! { <span class="msg-type-pill pill-roll">"🎲 ROLAGEM"</span> }.into_view()
                                                    } else if is_ic {
                                                        view! { <span class="msg-type-pill pill-ic">"🎭 EM CENA"</span> }.into_view()
                                                    } else {
                                                        view! { <span class="msg-type-pill pill-ooc">"💬 OFF"</span> }.into_view()
                                                    }}
                                                </div>
                                            </div>

                                            // Conteúdo ou Card de Rolagem
                                            {if let Some(roll) = msg.roll_data {
                                                view! {
                                                    <div class="roll-result-card" class:botch-card=roll.is_botch>
                                                        <div class="roll-header-line">
                                                            <span class="roll-action-label">{roll.label}</span>
                                                            <span class="roll-diff-badge">"Dif " {roll.difficulty}</span>
                                                        </div>

                                                        // Dados individuais rolados
                                                        <div class="dice-grid">
                                                            {roll.dice_results.iter().map(|&die| {
                                                                let is_ten = die == 10;
                                                                let is_success = die >= roll.difficulty;
                                                                let is_one = die == 1;

                                                                view! {
                                                                    <div 
                                                                        class="die-box"
                                                                        class:die-ten=is_ten
                                                                        class:die-success=is_success && !is_ten
                                                                        class:die-one=is_one
                                                                        class:die-fail=!is_success && !is_one
                                                                    >
                                                                        {die}
                                                                    </div>
                                                                }
                                                            }).collect_view()}
                                                        </div>

                                                        // Desfecho final
                                                        <div class="roll-outcome-banner" class:outcome-botch=roll.is_botch>
                                                            {if roll.is_botch {
                                                                view! { <span class="outcome-text">"💥 FALHA CRÍTICA (BOTCH)!"</span> }.into_view()
                                                            } else if roll.successes > 0 {
                                                                view! { <span class="outcome-text text-success">"✨ " {roll.successes} " Sucesso(s)!"</span> }.into_view()
                                                            } else {
                                                                view! { <span class="outcome-text text-neutral">"Falha Simples (0 sucessos)"</span> }.into_view()
                                                            }}
                                                        </div>
                                                    </div>
                                                }.into_view()
                                            } else {
                                                view! {
                                                    <div class="bubble-text-content">
                                                        {msg.content}
                                                    </div>
                                                }.into_view()
                                            }}
                                        </div>
                                    }
                                }
                            />
                        }.into_view()
                    }
                }}
            </div>

            // ==========================================
            // BARRA DE ENVIO DE MENSAGEM
            // ==========================================
            <div class="room-chat-input-bar">
                <div class="identity-picker">
                    <select 
                        class="identity-select"
                        on:change=move |ev| set_selected_identity.set(event_target_value(&ev))
                    >
                        <option value="player">"Como Jogador (OOC)"</option>
                        {sheets.get().into_iter().map(|s| {
                            view! {
                                <option value=s.id.clone()>
                                    "Como " {s.name} " (IC)"
                                </option>
                            }
                        }).collect_view()}
                    </select>
                </div>

                <input 
                    type="text"
                    class="chat-text-input"
                    placeholder="Digite sua mensagem ou comando (ex: /r 5d10 dif 6)..."
                    prop:value=move || input_text.get()
                    on:input=move |ev| set_input_text.set(event_target_value(&ev))
                    on:keydown=move |ev| {
                        if ev.key() == "Enter" {
                            handle_send_message();
                        }
                    }
                />

                <button 
                    type="button" 
                    class="chat-send-btn" 
                    on:click=move |_| handle_send_message()
                    disabled=move || input_text.get().trim().is_empty() || is_sending.get()
                >
                    "Enviar"
                </button>
            </div>
        </div>
    }
}
