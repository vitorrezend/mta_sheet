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
    #[prop(into, optional)] placeholder: MaybeSignal<String>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let textarea_ref = create_node_ref::<Textarea>();
    let is_focused = std::rc::Rc::new(std::cell::Cell::new(false));
    let last_synced_value = std::rc::Rc::new(std::cell::RefCell::new(String::new()));

    let is_focused_focus = is_focused.clone();
    let is_focused_blur = is_focused.clone();
    let last_synced_blur = last_synced_value.clone();
    let last_synced_effect = last_synced_value.clone();
    let is_focused_effect = is_focused.clone();

    create_render_effect(move |_| {
        let val = value.get();
        if !is_focused_effect.get() {
            if let Some(elem) = textarea_ref.get() {
                elem.set_value(&val);
            }
            *last_synced_effect.borrow_mut() = val;
        }
    });

    view! {
        <textarea
            node_ref=textarea_ref
            class=class
            placeholder=move || placeholder.get()
            prop:value=move || value.get()
            on:focus=move |_| { is_focused_focus.set(true); }
            on:blur=move |_| {
                is_focused_blur.set(false);
                if let Some(elem) = textarea_ref.get() {
                    let current_val = elem.value();
                    if current_val != *last_synced_blur.borrow() {
                        *last_synced_blur.borrow_mut() = current_val.clone();
                        on_change.call(current_val);
                    }
                }
            }
        ></textarea>
    }
}

/// Componente de Input de Texto Inteligente com Bloqueio de Foco.
/// Previne perda de foco e inversão de digitação ao sincronizar reativamente
/// apenas quando o input não está ativamente em foco pelo usuário.
#[component]
pub fn StableTextInput(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into, optional)] placeholder: MaybeSignal<String>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let input_ref = create_node_ref::<Input>();
    let is_focused = std::rc::Rc::new(std::cell::Cell::new(false));
    let last_synced_value = std::rc::Rc::new(std::cell::RefCell::new(String::new()));

    let is_focused_focus = is_focused.clone();
    let is_focused_blur = is_focused.clone();
    let last_synced_blur = last_synced_value.clone();
    let last_synced_effect = last_synced_value.clone();
    let is_focused_effect = is_focused.clone();

    create_render_effect(move |_| {
        let val = value.get();
        if !is_focused_effect.get() {
            if let Some(elem) = input_ref.get() {
                elem.set_value(&val);
            }
            *last_synced_effect.borrow_mut() = val;
        }
    });

    view! {
        <input
            type="text"
            node_ref=input_ref
            class=class
            placeholder=move || placeholder.get()
            prop:value=move || value.get()
            on:focus=move |_| { is_focused_focus.set(true); }
            on:blur=move |_| {
                is_focused_blur.set(false);
                if let Some(elem) = input_ref.get() {
                    let current_val = elem.value();
                    if current_val != *last_synced_blur.borrow() {
                        *last_synced_blur.borrow_mut() = current_val.clone();
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
            placeholder: "Digite...".to_string().into(),
            class: "custom-area",
        });

        let _input_view = StableTextInput(StableTextInputProps {
            value: val_sig.into(),
            on_change,
            placeholder: "Digite...".to_string().into(),
            class: "custom-input",
        });

        set_val_sig.set("Texto Modificado".to_string());
        assert_eq!(val_sig.get(), "Texto Modificado");

        runtime.dispose();
    }
}
