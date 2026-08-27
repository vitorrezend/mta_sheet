/// Reproduz um efeito sonoro de dados rolando / colidindo usando Web Audio API nativa.
pub fn play_dice_roll_sound() {
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::AudioContext;

        if let Ok(ctx) = AudioContext::new() {
            let current_time = ctx.current_time();
            
            // Simula múltiplos impactos e estalidos de dados colidindo na mesa de jogo
            let impacts = [
                (0.00, 520.0, 0.05, 0.22),
                (0.05, 640.0, 0.04, 0.18),
                (0.11, 420.0, 0.05, 0.26),
                (0.18, 580.0, 0.04, 0.20),
                (0.26, 460.0, 0.06, 0.30),
                (0.35, 510.0, 0.08, 0.35),
            ];

            for (delay, freq, duration, gain_val) in impacts {
                if let (Ok(osc), Ok(gain)) = (ctx.create_oscillator(), ctx.create_gain()) {
                    let start_t = current_time + delay;
                    let stop_t = start_t + duration;

                    osc.set_type(web_sys::OscillatorType::Triangle);
                    let _ = osc.frequency().set_value_at_time(freq, start_t);
                    let _ = osc.frequency().exponential_ramp_to_value_at_time(100.0, stop_t);

                    let _ = gain.gain().set_value_at_time(gain_val, start_t);
                    let _ = gain.gain().exponential_ramp_to_value_at_time(0.001, stop_t);

                    if osc.connect_with_audio_node(&gain).is_ok() && gain.connect_with_audio_node(&ctx.destination()).is_ok() {
                        let _ = osc.start_with_when(start_t);
                        let _ = osc.stop_with_when(stop_t);
                    }
                }
            }
        }
    }
}
