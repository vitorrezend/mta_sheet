use leptos::*;
use leptos::html::{Input, Textarea};

/// Componente de Textarea Inteligente com Bloqueio de Foco.
/// Previne travamentos e saltos de cursor ao isolar a digitação do usuário
/// dos ciclos de renderização reativa do Leptos.
#[component]
pub fn StableTextArea(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(optional)] placeholder: &'static str,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let textarea_ref = create_node_ref::<Textarea>();
    let (is_focused, set_is_focused) = create_signal(false);

    // Sincroniza o valor do sinal externo APENAS quando o usuário NÃO está digitando no campo
    create_effect(move |_| {
        let val = value.get();
        if !is_focused.get() {
            if let Some(elem) = textarea_ref.get() {
                elem.set_value(&val);
            }
        }
    });

    view! {
        <textarea
            node_ref=textarea_ref
            class=class
            placeholder=placeholder
            on:focus=move |_| set_is_focused.set(true)
            on:blur=move |_| {
                set_is_focused.set(false);
                if let Some(elem) = textarea_ref.get() {
                    let current_val = elem.value();
                    if current_val != value.get_untracked() {
                        on_change.call(current_val);
                    }
                }
            }
        ></textarea>
    }
}

/// Componente de Input de Texto Inteligente com Bloqueio de Foco.
#[component]
pub fn StableTextInput(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(optional)] placeholder: &'static str,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let input_ref = create_node_ref::<Input>();
    let (is_focused, set_is_focused) = create_signal(false);

    // Sincroniza o valor do sinal externo APENAS quando o usuário NÃO está focado no campo
    create_effect(move |_| {
        let val = value.get();
        if !is_focused.get() {
            if let Some(elem) = input_ref.get() {
                elem.set_value(&val);
            }
        }
    });

    view! {
        <input
            type="text"
            node_ref=input_ref
            class=class
            placeholder=placeholder
            on:focus=move |_| set_is_focused.set(true)
            on:blur=move |_| {
                set_is_focused.set(false);
                if let Some(elem) = input_ref.get() {
                    let current_val = elem.value();
                    if current_val != value.get_untracked() {
                        on_change.call(current_val);
                    }
                }
            }
        />
    }
}
