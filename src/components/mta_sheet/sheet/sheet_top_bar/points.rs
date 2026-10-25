//! Faixa de modos de pontuação: em que conta entram as bolinhas marcadas a partir de agora.
//!
//! Os quatro modos ficam sempre à vista, com o saldo de cada um, para quem usa entender
//! o que está fazendo sem abrir menus; o botão ⓘ explica cada modo.

use leptos::*;
use crate::components::Callback;
use crate::i18n::Language;
use crate::state::{CostSummary, DotOrigin};
use super::pick;

/// Um modo de pontuação e como apresentá-lo.
struct Mode {
    origin: DotOrigin,
    class: &'static str,
    label: (&'static str, &'static str),
    help: (&'static str, &'static str),
}

static MODES: [Mode; 4] = [
    Mode {
        origin: DotOrigin::Base,
        class: "dot-base",
        label: ("Criação", "Creation"),
        help: (
            "Pontos iniciais da criação: 7/5/3 em Atributos, 13/9/5 em Habilidades e 6 em Esferas.",
            "Starting dots: 7/5/3 Attributes, 13/9/5 Abilities and 6 Spheres.",
        ),
    },
    Mode {
        origin: DotOrigin::Bonus,
        class: "dot-bonus",
        label: ("Bônus", "Freebies"),
        help: (
            "15 pontos de bônus. Custo por ponto: Atributo 5, Habilidade 2, Esfera 7, Arete 4, Força de Vontade e Antecedente 1.",
            "15 freebie points. Cost per dot: Attribute 5, Ability 2, Sphere 7, Arete 4, Willpower and Background 1.",
        ),
    },
    Mode {
        origin: DotOrigin::Experience,
        class: "dot-xp",
        label: ("XP", "XP"),
        help: (
            "Experiência gasta ao longo da crônica, somada no extrato.",
            "Experience spent during the chronicle, totaled in the statement.",
        ),
    },
    Mode {
        origin: DotOrigin::Temporary,
        class: "dot-temp",
        label: ("Buff", "Buff"),
        help: (
            "Efeitos passageiros (feitiços, maravilhas, bênçãos). Não custam pontos.",
            "Temporary effects (spells, wonders, blessings). They cost no points.",
        ),
    },
];

/// Saldo mostrado ao lado do nome do modo, e se ele passou do limite.
fn mode_counter(origin: DotOrigin, costs: &CostSummary) -> (Option<String>, bool) {
    match origin {
        DotOrigin::Base => (None, costs.creation_points.has_any_overflow),
        DotOrigin::Bonus => (Some(format!("{}/15", costs.total_bonus_spent)), costs.total_bonus_spent > 15),
        DotOrigin::Experience => (Some(costs.total_xp_spent.to_string()), false),
        DotOrigin::Temporary => (None, false),
    }
}

#[component]
pub fn PointsModeBar(
    active_origin: ReadSignal<DotOrigin>,
    set_active_origin: WriteSignal<DotOrigin>,
    costs: Memo<CostSummary>,
    lang: Signal<Language>,
    on_open_breakdown: Callback<()>,
) -> impl IntoView {
    let show_help = create_rw_signal(false);

    view! {
        <div class="points-strip">
            <div class="points-strip-row">
                <span class="points-strip-label">
                    {move || pick(lang.get(), "Novos pontos contam como", "New dots count as")}
                </span>
                <div class="points-segmented" role="radiogroup" aria-label="Modo de pontuação">
                    {MODES.iter().map(|mode| {
                        let origin = mode.origin;
                        let selected = move || active_origin.get() == origin;
                        view! {
                            <button
                                type="button"
                                role="radio"
                                class="points-segment"
                                class:selected=selected
                                class:over-limit=move || costs.with(|c| mode_counter(origin, c).1)
                                aria-checked=move || if selected() { "true" } else { "false" }
                                title=move || pick(lang.get(), mode.help.0, mode.help.1)
                                on:click=move |_| set_active_origin.set(origin)
                            >
                                <span class=format!("points-segment-dot {}", mode.class)></span>
                                <span class="points-segment-label">{move || pick(lang.get(), mode.label.0, mode.label.1)}</span>
                                {move || costs.with(|c| mode_counter(origin, c)).0.map(|n| view! {
                                    <span class="points-segment-count">{n}</span>
                                })}
                                <span class="points-segment-alert" aria-hidden="true">"⚠"</span>
                            </button>
                        }
                    }).collect_view()}
                </div>
                <div class="points-strip-tools">
                    <button
                        type="button"
                        class="points-tool-btn"
                        class:active=move || show_help.get()
                        aria-expanded=move || if show_help.get() { "true" } else { "false" }
                        title=move || pick(lang.get(), "Como funcionam os modos de pontos", "How point modes work")
                        on:click=move |_| show_help.update(|v| *v = !*v)
                    >
                        "ⓘ"
                    </button>
                    <button
                        type="button"
                        class="points-tool-btn points-breakdown-btn"
                        title=move || pick(lang.get(), "Extrato completo de custos e XP", "Full cost and XP statement")
                        on:click=move |_| on_open_breakdown.call(())
                    >
                        <span>"📊"</span>
                        <span class="points-tool-text">{move || pick(lang.get(), "Extrato", "Statement")}</span>
                    </button>
                </div>
            </div>
            <div class="points-help" class:open=move || show_help.get()>
                <p class="points-help-intro">
                    {move || pick(
                        lang.get(),
                        "Escolha o modo antes de marcar bolinhas: cada ponto guarda de onde veio (a cor da bolinha) e entra no custo daquele modo. Clique com o botão direito numa bolinha para trocar a origem depois.",
                        "Pick a mode before filling dots: each dot remembers where it came from (its color) and counts toward that mode's cost. Right-click a dot to change its origin later.",
                    )}
                </p>
                <ul class="points-help-list">
                    {MODES.iter().map(|mode| view! {
                        <li>
                            <span class=format!("points-segment-dot {}", mode.class)></span>
                            <strong>{move || pick(lang.get(), mode.label.0, mode.label.1)}</strong>
                            <span>{move || pick(lang.get(), mode.help.0, mode.help.1)}</span>
                        </li>
                    }).collect_view()}
                </ul>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counters_flag_spending_over_the_limit() {
        let mut costs = CostSummary::default();
        costs.total_bonus_spent = 16;
        costs.total_xp_spent = 9;
        assert_eq!(mode_counter(DotOrigin::Bonus, &costs), (Some("16/15".into()), true));
        assert_eq!(mode_counter(DotOrigin::Experience, &costs), (Some("9".into()), false));
        assert_eq!(mode_counter(DotOrigin::Temporary, &costs), (None, false));
    }
}
