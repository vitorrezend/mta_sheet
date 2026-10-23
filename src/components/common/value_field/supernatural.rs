//! Espaço da escala sobrenatural ao lado das bolinhas: losango no nível 5, selo "+N" acima disso.

use leptos::*;
use super::super::callback::Callback;
use super::super::supernatural_modal::SupernaturalModal;
use super::summary::supernatural_badge_class;
use super::{with_origins, TraitField};

#[component]
pub fn SupernaturalSlot(field: TraitField, show_scale: RwSignal<bool>, lang: Signal<crate::i18n::Language>) -> impl IntoView {
    let level = field.level;
    let origins = field.origins;
    let badge_class = move || with_origins(origins, |o| supernatural_badge_class(level.get(), o));
    let has_badge = move |class: &'static str| badge_class() == class;

    move || {
        let cur_lvl = level.get();
        let is_open = show_scale.get();
        // Abaixo do nível 5 e sem o painel aberto, não ocupa espaço
        if cur_lvl < 5 && !is_open {
            return view! {}.into_view();
        }

        let marker = if cur_lvl >= 6 {
            let extra = cur_lvl - 5;
            view! {
                <span class="supernatural-separator">"|"</span>
                <button
                    type="button"
                    class="supernatural-badge"
                    class=("badge-origin-base", move || has_badge("badge-origin-base"))
                    class=("badge-origin-bonus", move || has_badge("badge-origin-bonus"))
                    class=("badge-origin-xp", move || has_badge("badge-origin-xp"))
                    class=("badge-origin-temp", move || has_badge("badge-origin-temp"))
                    on:click=move |ev| {
                        ev.stop_propagation();
                        show_scale.update(|v| *v = !*v);
                    }
                    title=format!("Nível Total: {} (+{} Sobrenatural) • Clique para alterar o valor", cur_lvl, extra)
                >
                    {format!("+{}", extra)}
                </button>
            }
            .into_view()
        } else if cur_lvl == 5 {
            let on_level_change = field.on_level_change.clone();
            view! {
                <span class="supernatural-separator">"|"</span>
                <span
                    class="dot dot-supernatural"
                    on:click:undelegated=move |ev| {
                        ev.stop_propagation();
                        on_level_change.call(6);
                        show_scale.set(true);
                    }
                    title="Clique para abrir a escala sobrenatural (6 a 10)"
                ></span>
            }
            .into_view()
        } else {
            view! {}.into_view()
        };

        let panel = is_open.then(|| view! {
            <SupernaturalModal field=field.clone() lang=lang on_close=Callback::new(move |_| show_scale.set(false)) />
        });

        view! {
            <div class="supernatural-container supernatural-slot" class:supernatural-empty=move || level.get() == 5>
                {marker}
                {panel}
            </div>
        }
        .into_view()
    }
}
