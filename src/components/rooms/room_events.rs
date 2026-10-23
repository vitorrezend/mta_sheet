//! Conexão única de eventos em tempo real (SSE) por sala, compartilhada pelos componentes.
//!
//! Antes, chat e iniciativa abriam cada um o seu `EventSource` e interpretavam todos os
//! eventos. Agora a `RoomView` abre uma conexão e cada componente se inscreve com
//! [`on_room_event`], recebendo eventos tipados.
//!
//! Reconexão: o `EventSource` já reconecta sozinho após quedas de rede, mas desiste se a
//! resposta for um erro HTTP; nesse caso tentamos de novo com espera crescente. No iPhone,
//! o Safari derruba a conexão quando a aba vai para segundo plano; ao voltar, reconectamos
//! e emitimos [`RoomStreamEvent::Reconnected`] para quem precisar recarregar o estado.

use std::rc::Rc;
use leptos::*;
use crate::rooms::RoomBroadcastEvent;

/// Preferência "sons da sala silenciados", compartilhada entre chat e iniciativa.
#[derive(Clone, Copy)]
pub struct RoomSoundMuted(pub RwSignal<bool>);

#[derive(Clone, Debug)]
pub enum RoomStreamEvent {
    /// Evento enviado pelo servidor
    Event(RoomBroadcastEvent),
    /// A conexão voltou depois de cair: eventos podem ter sido perdidos
    Reconnected,
}

type Listener = Rc<dyn Fn(&RoomStreamEvent)>;

#[derive(Clone, Copy)]
pub struct RoomEvents {
    listeners: StoredValue<Vec<(u64, Listener)>>,
    next_id: StoredValue<u64>,
    /// `true` enquanto a conexão SSE está aberta
    pub connected: RwSignal<bool>,
}

impl RoomEvents {
    fn subscribe(&self, listener: Listener) -> u64 {
        let id = self.next_id.get_value();
        self.next_id.set_value(id + 1);
        self.listeners.update_value(|list| list.push((id, listener)));
        id
    }

    fn unsubscribe(&self, id: u64) {
        self.listeners.update_value(|list| list.retain(|(lid, _)| *lid != id));
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn dispatch(&self, event: &RoomStreamEvent) {
        // Copia a lista antes de chamar: um ouvinte pode se inscrever/desinscrever durante o despacho
        let listeners: Vec<Listener> = self.listeners.with_value(|list| list.iter().map(|(_, l)| l.clone()).collect());
        for listener in listeners {
            listener(event);
        }
    }
}

/// Inscreve o componente atual nos eventos da sala; a inscrição é removida quando ele sai da tela.
pub fn on_room_event(listener: impl Fn(&RoomStreamEvent) + 'static) {
    let Some(events) = use_context::<RoomEvents>() else { return };
    let id = events.subscribe(Rc::new(listener));
    on_cleanup(move || events.unsubscribe(id));
}

/// Abre a conexão de eventos da sala e a disponibiliza via contexto para os filhos.
pub fn provide_room_events(room_id: Signal<String>) -> RoomEvents {
    let events = RoomEvents {
        listeners: store_value(Vec::new()),
        next_id: store_value(0),
        connected: create_rw_signal(false),
    };
    provide_context(events);

    #[cfg(target_arch = "wasm32")]
    create_effect(move |_| {
        let id = room_id.get();
        if id.is_empty() {
            return;
        }
        let connection = client::Connection::open(id, events);
        on_cleanup(move || connection.close());
    });
    #[cfg(not(target_arch = "wasm32"))]
    let _ = room_id;

    events
}

#[cfg(target_arch = "wasm32")]
mod client {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use leptos::*;
    use wasm_bindgen::{closure::Closure, JsCast};
    use web_sys::{EventSource, MessageEvent};
    use super::{RoomEvents, RoomStreamEvent};
    use crate::rooms::RoomBroadcastEvent;

    const MAX_BACKOFF_MS: u32 = 30_000;

    struct State {
        room_id: String,
        events: RoomEvents,
        source: RefCell<Option<EventSource>>,
        /// Houve queda desde a última conexão aberta (para emitir `Reconnected`)
        dropped: Cell<bool>,
        backoff_ms: Cell<u32>,
        closed: Cell<bool>,
        retry_timer: RefCell<Option<gloo_timers::callback::Timeout>>,
        visibility_listener: RefCell<Option<Closure<dyn FnMut()>>>,
    }

    pub struct Connection(Rc<State>);

    impl Connection {
        pub fn open(room_id: String, events: RoomEvents) -> Self {
            let state = Rc::new(State {
                room_id,
                events,
                source: RefCell::new(None),
                dropped: Cell::new(false),
                backoff_ms: Cell::new(1_000),
                closed: Cell::new(false),
                retry_timer: RefCell::new(None),
                visibility_listener: RefCell::new(None),
            });
            connect(&state);
            watch_visibility(&state);
            Self(state)
        }

        pub fn close(&self) {
            let state = &self.0;
            state.closed.set(true);
            state.retry_timer.borrow_mut().take();
            if let Some(source) = state.source.borrow_mut().take() {
                source.close();
            }
            if let (Some(document), Some(listener)) = (document_opt(), state.visibility_listener.borrow_mut().take()) {
                let _ = document.remove_event_listener_with_callback("visibilitychange", listener.as_ref().unchecked_ref());
            }
            state.events.connected.set(false);
        }
    }

    fn document_opt() -> Option<web_sys::Document> {
        web_sys::window().and_then(|w| w.document())
    }

    fn connect(state: &Rc<State>) {
        if state.closed.get() {
            return;
        }
        let Ok(source) = EventSource::new(&format!("/api/room_events/{}", state.room_id)) else {
            schedule_retry(state);
            return;
        };

        let on_open = {
            let state = Rc::downgrade(state);
            Closure::<dyn FnMut()>::new(move || {
                let Some(state) = state.upgrade() else { return };
                state.backoff_ms.set(1_000);
                state.events.connected.set(true);
                if state.dropped.replace(false) {
                    state.events.dispatch(&RoomStreamEvent::Reconnected);
                }
            })
        };

        let on_message = {
            let state = Rc::downgrade(state);
            Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
                let Some(state) = state.upgrade() else { return };
                let Some(text) = e.data().as_string() else { return };
                match serde_json::from_str::<RoomBroadcastEvent>(&text) {
                    Ok(event) => state.events.dispatch(&RoomStreamEvent::Event(event)),
                    Err(err) => log::warn!("Evento de sala inválido: {}", err),
                }
            })
        };

        let on_error = {
            let state = Rc::downgrade(state);
            Closure::<dyn FnMut()>::new(move || {
                let Some(state) = state.upgrade() else { return };
                state.dropped.set(true);
                state.events.connected.set(false);
                // CONNECTING: o navegador já está tentando de novo. CLOSED: desistiu, então tentamos nós.
                let gave_up = state.source.borrow().as_ref().is_some_and(|s| s.ready_state() == EventSource::CLOSED);
                if gave_up {
                    if let Some(source) = state.source.borrow_mut().take() {
                        source.close();
                    }
                    schedule_retry(&state);
                }
            })
        };

        source.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        source.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        source.set_onerror(Some(on_error.as_ref().unchecked_ref()));
        // As closures precisam viver enquanto o EventSource existir; são pequenas e
        // referenciam o estado só por Weak, então não prendem a sala na memória.
        on_open.forget();
        on_message.forget();
        on_error.forget();

        if let Some(old) = state.source.borrow_mut().replace(source) {
            old.close();
        }
    }

    fn schedule_retry(state: &Rc<State>) {
        if state.closed.get() {
            return;
        }
        let delay = state.backoff_ms.get();
        state.backoff_ms.set((delay * 2).min(MAX_BACKOFF_MS));
        let weak = Rc::downgrade(state);
        let timer = gloo_timers::callback::Timeout::new(delay, move || {
            if let Some(state) = weak.upgrade() {
                state.retry_timer.borrow_mut().take();
                connect(&state);
            }
        });
        *state.retry_timer.borrow_mut() = Some(timer);
    }

    /// Ao voltar para a aba (iPhone desbloqueado, app trocado), reconecta na hora se a conexão caiu.
    fn watch_visibility(state: &Rc<State>) {
        let Some(document) = document_opt() else { return };
        let weak = Rc::downgrade(state);
        let listener = Closure::<dyn FnMut()>::new(move || {
            let Some(state) = weak.upgrade() else { return };
            let visible = document_opt().is_some_and(|d| d.visibility_state() == web_sys::VisibilityState::Visible);
            let is_open = state.source.borrow().as_ref().is_some_and(|s| s.ready_state() == EventSource::OPEN);
            if visible && !is_open && !state.closed.get() {
                state.dropped.set(true);
                state.retry_timer.borrow_mut().take();
                state.backoff_ms.set(1_000);
                connect(&state);
            }
        });
        let _ = document.add_event_listener_with_callback("visibilitychange", listener.as_ref().unchecked_ref());
        *state.visibility_listener.borrow_mut() = Some(listener);
    }
}
