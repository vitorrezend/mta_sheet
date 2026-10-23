//! Estado do chat da sala, vivo enquanto a sala está aberta (e não só na aba do chat).
//!
//! Assim os sons de notificação e o contador de não lidas funcionam mesmo quando o
//! jogador está olhando a aba de personagens ou o mapa.

use leptos::*;
use crate::components::common::{play_dice_roll_sound, play_message_sound};
use crate::components::rooms::room_events::{on_room_event, RoomStreamEvent};
use crate::rooms::{get_room_chat_messages, ChatMessageKind, RoomBroadcastEvent, RoomChatMessage};

#[derive(Clone, Copy)]
pub struct ChatStore {
    pub messages: RwSignal<Vec<RoomChatMessage>>,
    /// Mensagens de outras pessoas recebidas com a aba do chat fechada
    pub unread: RwSignal<u32>,
    /// A aba do chat está visível (controlado pelo componente `RoomChat`)
    pub is_open: RwSignal<bool>,
    pub is_loading: RwSignal<bool>,
    pub load_error: RwSignal<Option<String>>,
    /// Sons das salas silenciados (compartilhado com a iniciativa)
    pub sound_muted: RwSignal<bool>,
    room_id: Signal<String>,
    my_user_id: Signal<Option<String>>,
}

impl ChatStore {
    /// Insere mantendo a ordem e sem duplicar (a própria mensagem chega pela resposta
    /// do servidor e de novo pelo SSE).
    pub fn insert(&self, msg: RoomChatMessage) -> bool {
        let mut inserted = false;
        self.messages.update(|list| {
            if !list.iter().any(|m| m.id == msg.id) {
                list.push(msg);
                inserted = true;
            }
        });
        inserted
    }

    pub fn is_mine(&self, msg: &RoomChatMessage) -> bool {
        self.my_user_id.get_untracked().as_deref() == Some(msg.sender_id.as_str())
    }

    pub fn play_sound_for(&self, msg: &RoomChatMessage) {
        if self.sound_muted.get_untracked() {
            return;
        }
        if msg.message_type == ChatMessageKind::Roll {
            play_dice_roll_sound();
        } else {
            play_message_sound();
        }
    }

    /// Recarrega o histórico completo do servidor (abertura da sala ou após reconexão).
    pub fn reload(&self) {
        let room_id = self.room_id.get_untracked();
        if room_id.is_empty() {
            return;
        }
        let store = *self;
        store.is_loading.set(true);
        spawn_local(async move {
            match get_room_chat_messages(room_id).await {
                Ok(history) => {
                    store.messages.set(history);
                    store.load_error.set(None);
                }
                Err(e) => store.load_error.set(Some(clean_error(&e.to_string()))),
            }
            store.is_loading.set(false);
        });
    }

    fn on_event(&self, event: &RoomStreamEvent) {
        match event {
            RoomStreamEvent::Event(RoomBroadcastEvent::ChatMessage(msg)) => {
                if self.insert(msg.clone()) && !self.is_mine(msg) {
                    self.play_sound_for(msg);
                    if !self.is_open.get_untracked() {
                        self.unread.update(|n| *n += 1);
                    }
                }
            }
            RoomStreamEvent::Event(RoomBroadcastEvent::ChatCleared) => self.messages.set(Vec::new()),
            RoomStreamEvent::Event(RoomBroadcastEvent::Resync) | RoomStreamEvent::Reconnected => self.reload(),
            RoomStreamEvent::Event(RoomBroadcastEvent::InitiativeUpdated { .. }) => {}
        }
    }
}

/// Remove o prefixo técnico que o Leptos adiciona às mensagens de erro do servidor.
pub fn clean_error(raw: &str) -> String {
    raw.trim_start_matches("error running server function: ")
        .trim_start_matches("ServerFnError: ")
        .to_string()
}

/// Cria o estado do chat da sala, inscreve nos eventos em tempo real e o disponibiliza via contexto.
/// Deve ser chamado depois de `provide_room_events`.
pub fn provide_chat_store(room_id: Signal<String>, my_user_id: Signal<Option<String>>, sound_muted: RwSignal<bool>) -> ChatStore {
    let store = ChatStore {
        messages: create_rw_signal(Vec::new()),
        unread: create_rw_signal(0),
        is_open: create_rw_signal(false),
        is_loading: create_rw_signal(false),
        load_error: create_rw_signal(None),
        sound_muted,
        room_id,
        my_user_id,
    };
    provide_context(store);

    create_effect(move |_| {
        if !room_id.get().is_empty() {
            store.messages.set(Vec::new());
            store.unread.set(0);
            store.reload();
        }
    });

    on_room_event(move |event| store.on_event(event));
    store
}
