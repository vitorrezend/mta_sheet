use leptos::*;
use std::rc::Rc;

#[component]
pub fn LabelField(
    #[prop(into)] label: MaybeSignal<String>,
    value: Signal<String>,
    on_change: impl Fn(String) + 'static,
    #[prop(optional)] on_lookup: Option<crate::components::Callback<()>>,
    #[prop(into, optional)] lookup_title: Option<MaybeSignal<String>>,
) -> impl IntoView {
    let on_change = Rc::new(on_change);
    let on_change_blur = on_change.clone();
    let on_change_input = on_change.clone();

    let input_ref = create_node_ref::<html::Input>();
    let is_focused = create_rw_signal(false);
    let last_synced_value = create_rw_signal(String::new());

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
        <div class="label-field">
            <div class="label-header-group">
                <span class="label-text">{move || label.get()}</span>
                {on_lookup.map(|cb| {
                    let title_sig = lookup_title.clone();
                    view! {
                        <button
                            type="button"
                            class="field-compendium-lookup-btn"
                            title=move || title_sig.as_ref().map(|t| t.get()).unwrap_or_else(|| "Consultar no Compêndio M20".to_string())
                            on:click=move |ev| {
                                ev.prevent_default();
                                ev.stop_propagation();
                                cb.call(());
                            }
                        >
                            "🎭"
                        </button>
                    }
                })}
            </div>
            <div class="tooltip-container" style="flex: 1; min-width: 0;">
                <input 
                    type="text" 
                    node_ref=input_ref
                    class="label-input" 
                    maxlength="30"
                    on:focus=move |_| { let _ = is_focused.try_set(true); }
                    on:change=move |_| {
                        if let Some(elem) = input_ref.get() {
                            let current_val = elem.value();
                            if current_val != last_synced_value.get_untracked() {
                                let _ = last_synced_value.try_set(current_val.clone());
                                on_change_input(current_val);
                            }
                        }
                    }
                    on:blur=move |_| {
                        let _ = is_focused.try_set(false);
                        if let Some(elem) = input_ref.get() {
                            let current_val = elem.value();
                            if current_val != last_synced_value.get_untracked() {
                                let _ = last_synced_value.try_set(current_val.clone());
                                on_change_blur(current_val);
                            }
                        }
                    }
                />
                <span class="tooltip-text" 
                    class:hidden=move || value.get().is_empty()
                >
                    {value}
                </span>
            </div>
        </div>
    }
}
