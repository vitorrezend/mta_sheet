use leptos::*;
use crate::components::Callback;
use crate::rules::{RollRequest, MAX_DICE, MAX_DIFFICULTY, MIN_DICE, MIN_DIFFICULTY};

/// Painel do rolador rápido de d10.
#[component]
pub fn QuickRollPanel(#[prop(into)] on_roll: Callback<RollRequest>, #[prop(into)] disabled: Signal<bool>) -> impl IntoView {
    let dice = create_rw_signal(5i32);
    let difficulty = create_rw_signal(6i32);
    let specialty = create_rw_signal(false);
    let label = create_rw_signal(String::new());

    let submit = move |_| {
        on_roll.call(RollRequest::new(dice.get(), difficulty.get(), specialty.get(), &label.get()));
        label.set(String::new());
    };

    view! {
        <div class="chat-quick-roll-panel">
            <div class="quick-roll-row">
                <Stepper title="Quantidade de Dados (D10)" value=dice min=MIN_DICE max=MAX_DICE />
                <Stepper title="Dificuldade (3-10)" value=difficulty min=MIN_DIFFICULTY max=MAX_DIFFICULTY />
                <div class="quick-field specialty-toggle">
                    <label class="checkbox-label">
                        <input
                            type="checkbox"
                            prop:checked=move || specialty.get()
                            on:change=move |ev| specialty.set(event_target_checked(&ev))
                        />
                        <span>"⭐ Especialidade (10s dobram)"</span>
                    </label>
                </div>
            </div>

            <div class="quick-roll-row label-row">
                <input
                    type="text"
                    class="quick-label-input"
                    maxlength="120"
                    placeholder="Rótulo da ação (ex: Força + Briga, Arete...)"
                    prop:value=move || label.get()
                    on:input=move |ev| label.set(event_target_value(&ev))
                />
                <button type="button" class="quick-roll-submit-btn" on:click=submit disabled=disabled>
                    "🎲 Rolar " {move || dice.get()} "d10"
                </button>
            </div>
        </div>
    }
}

#[component]
fn Stepper(title: &'static str, value: RwSignal<i32>, min: i32, max: i32) -> impl IntoView {
    view! {
        <div class="quick-field">
            <label>{title}</label>
            <div class="quick-number-stepper">
                <button type="button" aria-label="Diminuir" on:click=move |_| value.update(|v| *v = (*v - 1).max(min))>"-"</button>
                <span class="stepper-val">{move || value.get()}</span>
                <button type="button" aria-label="Aumentar" on:click=move |_| value.update(|v| *v = (*v + 1).min(max))>"+"</button>
            </div>
        </div>
    }
}
