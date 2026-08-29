use leptos::*;
use crate::state::CharacterData;

/// Retorna as coordenadas (x, y) no espaço do SVG (170x170) para cada um dos 20 quadrados da roda.
/// O ponto de origem (separador vazio) fica exatamente na posição das 9 horas (180° / à esquerda).
/// - Índices 0..=9: Arco superior (sentido horário / para cima a partir das 9h em direção às 3h).
/// - Índices 19..=10: Arco inferior (sentido anti-horário / para baixo a partir das 9h em direção às 3h).
fn get_box_coords(index: usize) -> (f64, f64) {
    let cx = 85.0;
    let cy = 85.0;
    let r = 67.0;
    let angle_deg: f64 = if index < 10 {
        // Arco Superior (0..9): index 0 = 171°, index 9 = 9°
        180.0 - (index as f64 + 0.5) * 18.0
    } else {
        // Arco Inferior (19..10): index 19 = 189°, index 10 = 351°
        let step = (19 - index) as f64;
        180.0 + (step + 0.5) * 18.0
    };
    let rad = angle_deg.to_radians();
    let x = cx + r * rad.cos() - 6.5;
    let y = cy - r * rad.sin() - 6.5;
    (x, y)
}

#[component]
pub fn QuintessenceParadox() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    let states_key = crate::state::models::keys::KEY_QUINTESSENCE_PARADOX;

    let states = Signal::derive(move || {
        let val = data.with(|d| d.labels.get(states_key).cloned().unwrap_or_default());
        if val.len() == 20 { val } else { "0".repeat(20) }
    });

    let quintessence_total = Signal::derive(move || {
        states.get().chars().filter(|&c| c == '1').count() as i32
    });
    let paradox_total = Signal::derive(move || {
        states.get().chars().filter(|&c| c == '2').count() as i32
    });

    let add_q = move |_| {
        set_data.update(|s| s.add_quintessence());
    };

    let remove_q = move |_| {
        set_data.update(|s| s.remove_quintessence());
    };

    let add_p = move |_| {
        set_data.update(|s| s.add_paradox());
    };

    let remove_p = move |_| {
        set_data.update(|s| s.remove_paradox());
    };

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let render_box = move |i: usize| {
        let (x, y) = get_box_coords(i);
        let state = move || states.get().chars().nth(i).unwrap_or('0');
        let is_quint = move || state() == '1';
        let is_paradox = move || state() == '2';

        let box_tooltip = move || {
            let s = state();
            let is_top = i < 10;
            let current_lang = lang();
            let q_label = crate::i18n::tr("quintessence", current_lang);
            let p_label = crate::i18n::tr("paradox", current_lang);

            match s {
                '1' => format!("✦ {} (Slot #{})", q_label, i + 1),
                '2' => format!("⚡ {} (Slot #{})", p_label, 20 - i),
                _ => {
                    if is_top {
                        format!("○ Slot Livre (Sentido Horário - {})", q_label)
                    } else {
                        format!("○ Slot Livre (Sentido Anti-horário - {})", p_label)
                    }
                }
            }
        };

        view! {
            <rect
                x=format!("{:.1}", x)
                y=format!("{:.1}", y)
                width="13"
                height="13"
                rx="2"
                ry="2"
                class="qp-wheel-box"
                class:qp-box-quintessence=is_quint
                class:qp-box-paradox=is_paradox
                on:click=move |_| {
                    set_data.update(|s| s.cycle_quintessence_paradox_box(i));
                }
                on:contextmenu=move |ev: ev::MouseEvent| {
                    ev.prevent_default();
                    set_data.update(|s| s.set_quintessence_paradox_box(i, '0'));
                }
            >
                <title>{box_tooltip}</title>
            </rect>
        }
    };

    view! {
        <div class="quintessence-paradox-container">
            <h3 class="column-title">
                {move || format!("{} / {}", crate::i18n::tr("quintessence", lang()), crate::i18n::tr("paradox", lang()))}
            </h3>

            <div class="qp-wheel-wrapper">
                <svg
                    class="qp-wheel-svg"
                    viewBox="0 0 170 170"
                    width="170"
                    height="170"
                >
                    // Círculo de trilho pontilhado sutil ao fundo
                    <circle
                        cx="85"
                        cy="85"
                        r="67"
                        class="qp-track-ring"
                    />

                    // Marcador de Origem às 9 Horas (Esquerda / Linha de Separação)
                    <g class="qp-origin-marker" title="Ponto de Origem (9h)">
                        <line x1="9" y1="85" x2="20" y2="85" class="qp-origin-line" />
                        <circle cx="8" cy="85" r="2" class="qp-origin-dot" />
                        <text x="1" y="87.5" class="qp-origin-text">"9h"</text>
                    </g>

                    // Renderização dos 20 Quadrados distribuídos na circunferência
                    // 0..10: Arco superior (Quintessência)
                    {(0..10).map(|i| render_box(i)).collect_view()}
                    // 10..20: Arco inferior (Paradoxo)
                    {(10..20).map(|i| render_box(i)).collect_view()}
                </svg>

                // Núcleo Central Interativo com Botões de Ação Rápida
                <div class="qp-center-hub">
                    // Controle de Quintessência (Arco Superior)
                    <div class="qp-hub-row qp-hub-quint">
                        <span class="qp-hub-title">{move || crate::i18n::tr("quintessence", lang())}</span>
                        <div class="qp-hub-controls">
                            <button
                                type="button"
                                class="qp-btn qp-btn-minus qp-btn-quint"
                                on:click=remove_q
                                title="Remover 1 ponto de Quintessência"
                            >
                                "−"
                            </button>
                            <span class="qp-count-badge qp-badge-quint">
                                {quintessence_total}
                            </span>
                            <button
                                type="button"
                                class="qp-btn qp-btn-plus qp-btn-quint"
                                on:click=add_q
                                title="Adicionar 1 ponto de Quintessência (sentido horário / para cima)"
                            >
                                "+"
                            </button>
                        </div>
                    </div>

                    <div class="qp-hub-divider"></div>

                    // Controle de Paradoxo (Arco Inferior)
                    <div class="qp-hub-row qp-hub-paradox">
                        <span class="qp-hub-title">{move || crate::i18n::tr("paradox", lang())}</span>
                        <div class="qp-hub-controls">
                            <button
                                type="button"
                                class="qp-btn qp-btn-minus qp-btn-paradox"
                                on:click=remove_p
                                title="Remover 1 ponto de Paradoxo"
                            >
                                "−"
                            </button>
                            <span class="qp-count-badge qp-badge-paradox">
                                {paradox_total}
                            </span>
                            <button
                                type="button"
                                class="qp-btn qp-btn-plus qp-btn-paradox"
                                on:click=add_p
                                title="Adicionar 1 ponto de Paradoxo (sentido anti-horário / para baixo)"
                            >
                                "+"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
