//! Sons da sala (rolagem de dados e nova mensagem) sintetizados com Web Audio.
//!
//! Regras dos navegadores, em especial Safari/iOS:
//! - só existe UM `AudioContext` por página (criar um por som esgota o limite do navegador);
//! - o contexto nasce "suspenso" e só pode ser liberado dentro de um gesto do usuário
//!   (toque, clique, tecla). [`install_audio_unlock`] faz isso no primeiro gesto, para que
//!   sons disparados depois por eventos da sala (sem gesto) consigam tocar;
//! - no iPhone, o Web Audio respeita a chave de silencioso do aparelho.

/// Chave do localStorage com a preferência de som mudo das salas.
const MUTE_STORAGE_KEY: &str = "mta_mute_dice_sound";

#[cfg(target_arch = "wasm32")]
mod engine {
    use std::cell::{Cell, RefCell};
    use wasm_bindgen::{closure::Closure, JsCast, JsValue};
    use web_sys::{AudioContext, AudioContextState, OscillatorType};

    thread_local! {
        static CONTEXT: RefCell<Option<AudioContext>> = const { RefCell::new(None) };
        static UNLOCK_INSTALLED: Cell<bool> = const { Cell::new(false) };
        static UNLOCKED: Cell<bool> = const { Cell::new(false) };
    }

    /// Cria o contexto uma única vez. Safari antigo (< 14.1) só expõe `webkitAudioContext`.
    fn context() -> Option<AudioContext> {
        CONTEXT.with(|slot| {
            if let Some(ctx) = slot.borrow().as_ref() {
                return Some(ctx.clone());
            }
            let ctx = AudioContext::new().ok().or_else(|| {
                let window = web_sys::window()?;
                let ctor = js_sys::Reflect::get(&window, &JsValue::from_str("webkitAudioContext")).ok()?;
                let ctor: js_sys::Function = ctor.dyn_into().ok()?;
                js_sys::Reflect::construct(&ctor, &js_sys::Array::new()).ok().map(JsCast::unchecked_into)
            })?;
            *slot.borrow_mut() = Some(ctx.clone());
            Some(ctx)
        })
    }

    fn running_context() -> Option<AudioContext> {
        let ctx = context()?;
        if ctx.state() == AudioContextState::Suspended {
            let _ = ctx.resume();
        }
        Some(ctx)
    }

    /// Toca um buffer silencioso de 1 amostra: é o "destravamento" que o iOS exige.
    fn unlock() {
        if UNLOCKED.with(Cell::get) {
            return;
        }
        let Some(ctx) = running_context() else { return };
        if let (Ok(buffer), Ok(source)) = (ctx.create_buffer(1, 1, 22050.0), ctx.create_buffer_source()) {
            source.set_buffer(Some(&buffer));
            if source.connect_with_audio_node(&ctx.destination()).is_ok() {
                let _ = source.start();
            }
        }
        UNLOCKED.with(|u| u.set(ctx.state() == AudioContextState::Running));
    }

    pub fn install_unlock() {
        if UNLOCK_INSTALLED.with(|i| i.replace(true)) {
            return;
        }
        let Some(window) = web_sys::window() else { return };
        // Os ouvintes vivem durante toda a página e viram no-op depois do desbloqueio
        let handler = Closure::<dyn FnMut(web_sys::Event)>::new(move |_: web_sys::Event| unlock());
        for event in ["pointerdown", "touchend", "keydown", "click"] {
            let _ = window.add_event_listener_with_callback_and_bool(event, handler.as_ref().unchecked_ref(), true);
        }
        handler.forget();
    }

    /// Uma nota: (atraso, frequência inicial, frequência final, duração, volume, forma de onda)
    type Note = (f64, f32, f32, f64, f32, OscillatorType);

    pub fn play(notes: &[Note]) {
        let Some(ctx) = running_context() else { return };
        let now = ctx.current_time();
        for &(delay, freq_start, freq_end, duration, volume, wave) in notes {
            let (Ok(osc), Ok(gain)) = (ctx.create_oscillator(), ctx.create_gain()) else { continue };
            let start = now + delay;
            let stop = start + duration;
            osc.set_type(wave);
            let _ = osc.frequency().set_value_at_time(freq_start, start);
            let _ = osc.frequency().exponential_ramp_to_value_at_time(freq_end, stop);
            let _ = gain.gain().set_value_at_time(volume, start);
            let _ = gain.gain().exponential_ramp_to_value_at_time(0.001, stop);
            if osc.connect_with_audio_node(&gain).is_ok() && gain.connect_with_audio_node(&ctx.destination()).is_ok() {
                let _ = osc.start_with_when(start);
                let _ = osc.stop_with_when(stop);
            }
        }
    }
}

/// Prepara o áudio para ser liberado no primeiro toque/clique/tecla. Pode ser chamada várias vezes.
pub fn install_audio_unlock() {
    #[cfg(target_arch = "wasm32")]
    engine::install_unlock();
}

/// Som de dados colidindo na mesa.
pub fn play_dice_roll_sound() {
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::OscillatorType::Triangle;
        engine::play(&[
            (0.00, 520.0, 100.0, 0.05, 0.22, Triangle),
            (0.05, 640.0, 100.0, 0.04, 0.18, Triangle),
            (0.11, 420.0, 100.0, 0.05, 0.26, Triangle),
            (0.18, 580.0, 100.0, 0.04, 0.20, Triangle),
            (0.26, 460.0, 100.0, 0.06, 0.30, Triangle),
            (0.35, 510.0, 100.0, 0.08, 0.35, Triangle),
        ]);
    }
}

/// "Plim" curto e suave para nova mensagem de outra pessoa.
pub fn play_message_sound() {
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::OscillatorType::Sine;
        engine::play(&[
            (0.00, 880.0, 870.0, 0.14, 0.12, Sine),
            (0.09, 1320.0, 1300.0, 0.22, 0.10, Sine),
        ]);
    }
}

/// Lê do navegador se os sons das salas estão silenciados.
pub fn load_sound_muted() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        let storage = web_sys::window().and_then(|w| w.local_storage().ok()).flatten();
        if let Some(Ok(Some(value))) = storage.map(|s| s.get_item(MUTE_STORAGE_KEY)) {
            return value == "true";
        }
    }
    false
}

pub fn save_sound_muted(muted: bool) {
    #[cfg(target_arch = "wasm32")]
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok()).flatten() {
        let _ = storage.set_item(MUTE_STORAGE_KEY, if muted { "true" } else { "false" });
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (muted, MUTE_STORAGE_KEY);
}
