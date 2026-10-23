//! Célula do nome: estrela de afinidade, botão de variante, nome (fixo ou editável) e menu do botão direito.

use leptos::*;
use super::super::callback::Callback;
use super::summary::{tooltip_label, truncate_label};
use super::{with_origins, TraitField};

/// Estrela da Esfera de Afinidade.
#[component]
pub fn AffinityStar(is_starred: Option<Signal<bool>>, on_star: Callback<()>, tooltip: Option<&'static str>) -> impl IntoView {
    let is_active = move || is_starred.map(|s| s.get()).unwrap_or(false);
    view! {
        <button
            type="button"
            class="affinity-star-btn"
            class:active=is_active
            on:click=move |ev| {
                ev.stop_propagation();
                on_star.call(());
            }
            title=move || if is_active() {
                tooltip.unwrap_or("Esfera de Afinidade ativa (Custo XP: Atual × 7). Clique para desmarcar.")
            } else {
                "Clique para definir como Esfera de Afinidade (Custo XP: Atual × 7)"
            }
        >
            <span class="affinity-star-icon" class:active=is_active>
                {move || if is_active() { "★" } else { "☆" }}
            </span>
        </button>
    }
}

/// Alterna a variante Tecnocrática / Mística da Esfera.
#[component]
pub fn FlipButton(on_flip: Callback<()>, tooltip: Option<Signal<String>>) -> impl IntoView {
    view! {
        <button
            type="button"
            class="sphere-flip-btn"
            style="background: transparent; border: none; cursor: pointer; padding: 0 3px; font-size: 0.85rem; color: var(--text-secondary, #64748b); transition: color 0.15s, transform 0.15s; line-height: 1; display: inline-flex; align-items: center;"
            on:click=move |ev| {
                ev.stop_propagation();
                on_flip.call(());
            }
            title=move || tooltip.map(|t| t.get()).unwrap_or_else(|| "Alternar variante Tecnocrática / Mística".to_string())
        >
            "⇄"
        </button>
    }
}

/// Nome editável com focus-lock: o valor externo só entra no campo quando ele não está em foco.
#[component]
pub fn EditableName(label: Signal<String>, on_change: Option<Callback<String>>) -> impl IntoView {
    let label_ref = create_node_ref::<html::Input>();
    let is_focused = create_rw_signal(false);
    create_effect(move |_| {
        let val = label.get();
        if !is_focused.get_untracked() && let Some(elem) = label_ref.get() {
            elem.set_value(&val);
        }
    });
    view! {
        <input
            type="text"
            node_ref=label_ref
            class="label-edit-input"
            placeholder="Nome..."
            maxlength="32"
            on:focus=move |_| { let _ = is_focused.try_set(true); }
            on:blur=move |_| {
                let _ = is_focused.try_set(false);
                if let (Some(elem), Some(cb)) = (label_ref.get(), on_change.as_ref()) {
                    cb.call(elem.value());
                }
            }
        />
    }
}

/// Nome fixo, cortado em `max_chars`, com a dica de origem dos pontos.
#[component]
pub fn StaticName(field: TraitField, max_chars: Option<usize>, is_starred: Option<Signal<bool>>) -> impl IntoView {
    let label = field.label;
    view! {
        <span class="attribute-label" class:affinity-active=move || is_starred.map(|s| s.get()).unwrap_or(false)>
            {move || truncate_label(label.get(), max_chars)}
        </span>
    }
}

/// Dica que aparece sobre o nome.
#[component]
pub fn NameTooltip(field: TraitField) -> impl IntoView {
    let TraitField { label, level, origins, .. } = field;
    view! {
        <span class="tooltip-text">
            {move || with_origins(origins, |o| tooltip_label(&label.get(), level.get(), o))}
        </span>
    }
}

/// Menu do botão direito: escala sobrenatural, voltar ao nível 5 e compêndio.
#[component]
pub fn SupernaturalMenu(
    field: TraitField,
    is_supernatural: Option<Signal<bool>>,
    on_toggle: Callback<()>,
    show_menu: RwSignal<bool>,
    show_scale: RwSignal<bool>,
    lang: Signal<crate::i18n::Language>,
) -> impl IntoView {
    let TraitField { level, on_level_change, on_open_compendium, .. } = field;
    let is_sup = move || is_supernatural.map(|s| s.get()).unwrap_or(false) || level.get() >= 6;
    let close = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        show_menu.set(false);
    };
    view! {
        <div class="context-menu-wrapper" class:hidden=move || !show_menu.get()>
            <div class="context-menu-backdrop" on:click=close on:contextmenu=close />
            <div class="label-context-menu" on:click=move |ev| ev.stop_propagation()>
                <button
                    type="button"
                    class="label-context-item"
                    on:click=move |_| {
                        show_menu.set(false);
                        show_scale.set(true);
                    }
                >
                    <span class="context-item-icon">"✦"</span>
                    "Escala Sobrenatural (6 a 10)"
                </button>
                <button
                    type="button"
                    class="label-context-item"
                    class:hidden=move || !is_sup()
                    on:click=move |_| {
                        show_menu.set(false);
                        on_level_change.call(5);
                        on_toggle.call(());
                    }
                >
                    <span class="context-item-icon">"↺"</span>
                    "Resetar para Nível 5 (Mortal)"
                </button>
                {on_open_compendium.map(|open| view! {
                    <button
                        type="button"
                        class="label-context-item"
                        on:click=move |_| {
                            show_menu.set(false);
                            open.call(());
                        }
                    >
                        <span class="context-item-icon">"📖"</span>
                        {move || match lang.get() {
                            crate::i18n::Language::PtBr => "Compêndio M20 (Atributo)",
                            crate::i18n::Language::EnUs => "M20 Compendium (Attribute)",
                        }}
                    </button>
                })}
            </div>
        </div>
    }
}
