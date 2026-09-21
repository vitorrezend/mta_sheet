use leptos::*;
use leptos::html::{Input, Textarea};
use super::callback::Callback;

/// Auxiliar que insere 2 espaços na posição do cursor considerando unidades de código UTF-16 do DOM.
fn insert_tab_spaces(current: &str, start_u16: u32, end_u16: u32) -> (String, u32) {
    let utf16: Vec<u16> = current.encode_utf16().collect();
    let s = (start_u16 as usize).min(utf16.len());
    let e = (end_u16 as usize).min(utf16.len());

    let mut new_utf16 = Vec::with_capacity(utf16.len() + 2);
    new_utf16.extend_from_slice(&utf16[..s]);
    new_utf16.push(0x0020);
    new_utf16.push(0x0020);
    new_utf16.extend_from_slice(&utf16[e..]);

    let new_string = String::from_utf16_lossy(&new_utf16);
    let new_pos = (s + 2) as u32;
    (new_string, new_pos)
}

/// Componente de Textarea Inteligente com Bloqueio de Foco.
/// Previne travamentos e saltos de cursor ao isolar a digitação do usuário
/// dos ciclos de renderização reativa do Leptos.
#[component]
pub fn StableTextArea(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into, optional)] placeholder: MaybeSignal<String>,
    #[prop(optional)] class: &'static str,
    #[prop(default = false)] allow_tab: bool,
    #[prop(default = false)] auto_resize: bool,
    #[prop(optional)] node_ref: Option<NodeRef<Textarea>>,
) -> impl IntoView {
    let internal_ref = create_node_ref::<Textarea>();
    let target_ref = node_ref.unwrap_or(internal_ref);
    let is_focused = std::rc::Rc::new(std::cell::Cell::new(false));
    let last_synced_value = std::rc::Rc::new(std::cell::RefCell::new(String::new()));

    let is_focused_focus = is_focused.clone();
    let is_focused_blur = is_focused.clone();
    let last_synced_blur = last_synced_value.clone();
    let last_synced_effect = last_synced_value.clone();
    let is_focused_effect = is_focused.clone();

    let adjust_height_for_elem = move |elem: &web_sys::HtmlTextAreaElement| {
        let style = elem.style();
        let _ = style.set_property("height", "auto");
        let scroll_h = elem.scroll_height();
        if scroll_h > 0 {
            let _ = style.set_property("height", &format!("{}px", scroll_h));
        }
    };

    create_render_effect(move |_| {
        let val = value.get();
        if !is_focused_effect.get() {
            if let Some(elem) = target_ref.get() {
                elem.set_value(&val);
                if auto_resize {
                    adjust_height_for_elem(&elem);
                }
            }
            *last_synced_effect.borrow_mut() = val;
        }
    });

    let on_change_tab = on_change.clone();
    let last_synced_tab = last_synced_value.clone();

    view! {
        <textarea
            node_ref=target_ref
            class=class
            placeholder=move || placeholder.get()
            prop:value=move || value.get()
            on:focus=move |_| { is_focused_focus.set(true); }
            on:input=move |_| {
                if auto_resize {
                    if let Some(elem) = target_ref.get() {
                        adjust_height_for_elem(&elem);
                    }
                }
            }
            on:keydown=move |ev: web_sys::KeyboardEvent| {
                if allow_tab && ev.key() == "Tab" && !ev.shift_key() && !ev.ctrl_key() && !ev.alt_key() {
                    ev.prevent_default();
                    if let Some(elem) = target_ref.get() {
                        if let (Ok(Some(start)), Ok(Some(end))) = (elem.selection_start(), elem.selection_end()) {
                            let current_val = elem.value();
                            let (new_val, new_pos) = insert_tab_spaces(&current_val, start, end);
                            elem.set_value(&new_val);
                            let _ = elem.set_selection_range(new_pos, new_pos);
                            *last_synced_tab.borrow_mut() = new_val.clone();
                            on_change_tab.call(new_val);
                            if auto_resize {
                                adjust_height_for_elem(&elem);
                            }
                        }
                    }
                } else if ev.key() == "Escape" {
                    if let Some(elem) = target_ref.get() {
                        let _ = elem.blur();
                    }
                }
            }
            on:blur=move |_| {
                is_focused_blur.set(false);
                if let Some(elem) = target_ref.get() {
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
            allow_tab: false,
            auto_resize: false,
            node_ref: None,
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

    #[test]
    fn test_insert_tab_spaces_utf16_safety() {
        // Teste de inserção normal no início
        let (res1, pos1) = insert_tab_spaces("Hello", 0, 0);
        assert_eq!(res1, "  Hello");
        assert_eq!(pos1, 2);

        // Teste com caracteres multibyte (emojis e acentos)
        let text = "Mágica ✨ Feitiço";
        let utf16_len = text.encode_utf16().count() as u32;
        let (res2, pos2) = insert_tab_spaces(text, utf16_len, utf16_len);
        assert_eq!(res2, format!("{}  ", text));
        assert_eq!(pos2, utf16_len + 2);

        // Teste de substituição de seleção
        let (res3, pos3) = insert_tab_spaces("ABCD", 1, 3);
        assert_eq!(res3, "A  D");
        assert_eq!(pos3, 3);
    }
}
