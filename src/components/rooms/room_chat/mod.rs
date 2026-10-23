//! Chat da sala: mensagens em tempo real, rolagens de dados e sons de notificação.
//!
//! - [`store`]: estado vivo durante toda a sala (histórico, não lidas, sons);
//! - [`message`]: balão de mensagem e cartão de rolagem;
//! - [`quick_roll`]: painel do rolador rápido.

pub mod message;
pub mod quick_roll;
pub mod store;

use leptos::*;
use crate::components::common::{play_dice_roll_sound, save_sound_muted};
use crate::rooms::{clear_room_chat, roll_room_dice, send_room_chat_message, RoomChatMessage, RoomSheetSummary, CHAT_MAX_CHARS};
use crate::rules::RollRequest;
use message::ChatMessageItem;
use quick_roll::QuickRollPanel;
pub use store::{provide_chat_store, ChatStore};

/// Distância (px) do fim da lista abaixo da qual consideramos que o usuário está "no fim".
const NEAR_BOTTOM_PX: i32 = 80;

#[component]
pub fn RoomChat(
    room_id: Signal<String>,
    sheets: Signal<Vec<RoomSheetSummary>>,
    is_gm: Signal<bool>,
) -> impl IntoView {
    let Some(store) = use_context::<ChatStore>() else {
        return view! { <p class="chat-load-error">"Chat indisponível nesta página."</p> }.into_view();
    };

    // Aba aberta: zera as não lidas e evita contar enquanto o usuário está lendo
    store.is_open.set(true);
    store.unread.set(0);
    on_cleanup(move || store.is_open.set(false));

    let input_text = create_rw_signal(String::new());
    let selected_sheet = create_rw_signal(String::new());
    let show_quick_roll = create_rw_signal(false);
    let is_sending = create_rw_signal(false);
    let send_error = create_rw_signal(Option::<String>::None);
    let has_new_below = create_rw_signal(false);
    let feed_ref = create_node_ref::<html::Div>();

    // Personagens disponíveis para falar "em cena": os próprios (o mestre pode usar todos)
    let speakable_sheets = Signal::derive(move || {
        let gm = is_gm.get();
        sheets.get().into_iter().filter(|s| gm || s.is_owner).collect::<Vec<_>>()
    });
    // Se o personagem escolhido sair da sala, volta a falar como jogador
    create_effect(move |_| {
        let current = selected_sheet.get();
        if !current.is_empty() && !speakable_sheets.with(|list| list.iter().any(|s| s.id == current)) {
            selected_sheet.set(String::new());
        }
    });
    let sheet_choice = move || Some(selected_sheet.get_untracked()).filter(|s| !s.is_empty());

    let is_near_bottom = move || {
        feed_ref.get_untracked().is_some_and(|el| el.scroll_height() - el.scroll_top() - el.client_height() < NEAR_BOTTOM_PX)
    };
    let scroll_to_bottom = move || {
        request_animation_frame(move || {
            if let Some(el) = feed_ref.get_untracked() {
                el.set_scroll_top(el.scroll_height());
            }
            has_new_below.set(false);
        });
    };

    // Rola para o fim só se o usuário já estava lá; senão mostra o aviso "novas mensagens"
    create_effect(move |prev_len: Option<usize>| {
        let len = store.messages.with(Vec::len);
        let last_is_mine = store.messages.with(|list| list.last().is_some_and(|m| store.is_mine(m)));
        match prev_len {
            None | Some(0) => scroll_to_bottom(),
            Some(prev) if len > prev => {
                if last_is_mine || is_near_bottom() {
                    scroll_to_bottom();
                } else {
                    has_new_below.set(true);
                }
            }
            _ => {}
        }
        len
    });

    let after_send = move |result: Result<RoomChatMessage, ServerFnError>, clear_input: bool| {
        match result {
            Ok(msg) => {
                if msg.roll_data.is_some() && !store.sound_muted.get_untracked() {
                    play_dice_roll_sound();
                }
                store.insert(msg);
                send_error.set(None);
                if clear_input {
                    input_text.set(String::new());
                }
            }
            // Mantém o texto digitado para o usuário tentar de novo
            Err(e) => send_error.set(Some(store::clean_error(&e.to_string()))),
        }
        is_sending.set(false);
    };

    let send_message = move || {
        let text = input_text.get_untracked().trim().to_string();
        if text.is_empty() || is_sending.get_untracked() {
            return;
        }
        is_sending.set(true);
        let (r_id, sheet_id) = (room_id.get_untracked(), sheet_choice());
        spawn_local(async move {
            after_send(send_room_chat_message(r_id, text, sheet_id).await, true);
        });
    };

    let quick_roll = move |req: RollRequest| {
        if is_sending.get_untracked() {
            return;
        }
        is_sending.set(true);
        show_quick_roll.set(false);
        let (r_id, sheet_id) = (room_id.get_untracked(), sheet_choice());
        spawn_local(async move {
            let result = roll_room_dice(r_id, req.dice_count, req.difficulty, req.specialty, req.label, sheet_id).await;
            after_send(result, false);
        });
    };

    let clear_history = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let confirmed = web_sys::window()
                .and_then(|w| w.confirm_with_message("Apagar todo o histórico do chat desta sala?").ok())
                .unwrap_or(false);
            if !confirmed {
                return;
            }
        }
        let r_id = room_id.get_untracked();
        spawn_local(async move {
            if let Err(e) = clear_room_chat(r_id).await {
                send_error.set(Some(store::clean_error(&e.to_string())));
            }
        });
    };

    let toggle_sound = move |_| {
        store.sound_muted.update(|m| *m = !*m);
        save_sound_muted(store.sound_muted.get_untracked());
    };

    view! {
        <div class="room-chat-wrapper">
            <div class="room-chat-header">
                <div class="chat-header-left">
                    <span class="chat-title">"💬 Feed da Crônica & Rolagens"</span>
                    <span class="chat-count">{move || format!("({} registros)", store.messages.with(Vec::len))}</span>
                </div>
                <div class="chat-header-right">
                    <button
                        type="button"
                        class="chat-sound-toggle-btn"
                        class:muted=move || store.sound_muted.get()
                        on:click=toggle_sound
                        title=move || if store.sound_muted.get() { "Sons da sala desligados (clique para ligar)" } else { "Sons da sala ligados (clique para silenciar)" }
                    >
                        {move || if store.sound_muted.get() { "🔕" } else { "🔔" }}
                    </button>
                    <button
                        type="button"
                        class="chat-quick-roll-toggle-btn"
                        class:active=move || show_quick_roll.get()
                        on:click=move |_| show_quick_roll.update(|v| *v = !*v)
                    >
                        "🎲 Rolar Dados"
                    </button>
                    <Show when=move || is_gm.get()>
                        <button type="button" class="chat-clear-btn" title="Limpar histórico de mensagens da sala" on:click=clear_history>
                            "🗑️"
                        </button>
                    </Show>
                </div>
            </div>

            <Show when=move || show_quick_roll.get()>
                <QuickRollPanel on_roll=quick_roll disabled=Signal::derive(move || is_sending.get()) />
            </Show>

            <div class="room-chat-feed-wrap">
                <div class="room-chat-feed" node_ref=feed_ref role="log" aria-live="polite"
                    on:scroll=move |_| if is_near_bottom() { has_new_below.set(false) }
                >
                    <Show when=move || store.load_error.get().is_some()>
                        <div class="chat-load-error">
                            {move || store.load_error.get().unwrap_or_default()}
                            <button type="button" on:click=move |_| store.reload()>"Tentar de novo"</button>
                        </div>
                    </Show>
                    <Show when=move || store.messages.with(Vec::is_empty) && !store.is_loading.get()>
                        <div class="chat-empty-state">
                            <span class="empty-icon">"📜"</span>
                            <h4>"Nenhuma mensagem ou rolagem nesta sessão ainda."</h4>
                            <p>"Envie uma mensagem abaixo ou use o rolador de dados para inaugurar o feed da crônica!"</p>
                        </div>
                    </Show>
                    <For
                        each=move || store.messages.get()
                        key=|m| m.id.clone()
                        children=move |msg: RoomChatMessage| {
                            let mine = store.is_mine(&msg);
                            view! { <ChatMessageItem msg=msg is_mine=mine /> }
                        }
                    />
                </div>
                <Show when=move || has_new_below.get()>
                    <button type="button" class="chat-new-below-btn" on:click=move |_| scroll_to_bottom()>
                        "⬇ Novas mensagens"
                    </button>
                </Show>
            </div>

            <Show when=move || send_error.get().is_some()>
                <div class="chat-send-error" role="alert">
                    {move || send_error.get().unwrap_or_default()}
                </div>
            </Show>

            <form class="room-chat-input-bar" on:submit=move |ev| { ev.prevent_default(); send_message(); }>
                <select
                    class="identity-select"
                    aria-label="Falar como"
                    prop:value=move || selected_sheet.get()
                    on:change=move |ev| selected_sheet.set(event_target_value(&ev))
                >
                    <option value="">"Como Jogador (OFF)"</option>
                    <For
                        each=move || speakable_sheets.get()
                        key=|s| s.id.clone()
                        children=move |s: RoomSheetSummary| view! { <option value=s.id>"Como " {s.name} " (Em cena)"</option> }
                    />
                </select>

                <input
                    type="text"
                    class="chat-text-input"
                    enterkeyhint="send"
                    autocomplete="off"
                    maxlength=CHAT_MAX_CHARS.to_string()
                    placeholder="Mensagem ou comando (ex: /r 5d10 dif 6)"
                    prop:value=move || input_text.get()
                    on:input=move |ev| input_text.set(event_target_value(&ev))
                />

                <button
                    type="submit"
                    class="chat-send-btn"
                    disabled=move || input_text.with(|t| t.trim().is_empty()) || is_sending.get()
                >
                    {move || if is_sending.get() { "Enviando…" } else { "Enviar" }}
                </button>
            </form>
        </div>
    }.into_view()
}
