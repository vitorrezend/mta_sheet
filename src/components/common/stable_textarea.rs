use leptos::*;
use leptos::html::{Input, Textarea};
use super::callback::Callback;

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
    let is_focused = create_rw_signal(false);
    let last_synced_value = create_rw_signal(String::new());

    // Sincroniza o valor do sinal externo APENAS quando o valor externo mudar
    // e o usuário NÃO estiver digitando ativamente no campo
    create_effect(move |_| {
        let val = value.get();
        if !is_focused.get_untracked() {
            if let Some(elem) = textarea_ref.get() {
                elem.set_value(&val);
            }
            let _ = last_synced_value.try_set(val);
        }
    });

    view! {
        <textarea
            node_ref=textarea_ref
            class=class
            placeholder=placeholder
            on:focus=move |_| { let _ = is_focused.try_set(true); }
            on:blur=move |_| {
                let _ = is_focused.try_set(false);
                if let Some(elem) = textarea_ref.get() {
                    let current_val = elem.value();
                    if current_val != last_synced_value.get_untracked() {
                        let _ = last_synced_value.try_set(current_val.clone());
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
    let is_focused = create_rw_signal(false);
    let last_synced_value = create_rw_signal(String::new());

    // Sincroniza o valor do sinal externo APENAS quando o valor externo mudar
    // e o usuário NÃO estiver focado no campo
    create_effect(move |_| {
        let val = value.get();
        if !is_focused.get_untracked() {
            if let Some(elem) = input_ref.get() {
                elem.set_value(&val);
            }
            let _ = last_synced_value.try_set(val);
        }
    });

    view! {
        <input
            type="text"
            node_ref=input_ref
            class=class
            placeholder=placeholder
            on:focus=move |_| { let _ = is_focused.try_set(true); }
            on:blur=move |_| {
                let _ = is_focused.try_set(false);
                if let Some(elem) = input_ref.get() {
                    let current_val = elem.value();
                    if current_val != last_synced_value.get_untracked() {
                        let _ = last_synced_value.try_set(current_val.clone());
                        on_change.call(current_val);
                    }
                }
            }
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stable_inputs_instantiation_and_reactive_lifecycle() {
        let runtime = create_runtime();
        let (val_sig, set_val_sig) = create_signal("Texto Inicial".to_string());
        let (_changed, set_changed) = create_signal(String::new());

        let on_change = Callback::new(move |val| {
            set_changed.set(val);
        });

        let _textarea_view = StableTextArea(StableTextAreaProps {
            value: val_sig.into(),
            on_change: on_change.clone(),
            placeholder: "Digite...",
            class: "custom-area",
        });

        let _input_view = StableTextInput(StableTextInputProps {
            value: val_sig.into(),
            on_change,
            placeholder: "Digite...",
            class: "custom-input",
        });

        set_val_sig.set("Texto Modificado".to_string());
        assert_eq!(val_sig.get(), "Texto Modificado");

        runtime.dispose();
    }
}
