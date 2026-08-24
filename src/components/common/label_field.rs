use leptos::*;
use std::rc::Rc;

#[component]
pub fn LabelField(
    label: &'static str,
    value: Signal<String>,
    on_change: impl Fn(String) + 'static,
) -> impl IntoView {
    let on_change = Rc::new(on_change);
    let on_change_input = on_change.clone();

    view! {
        <div class="label-field">
            <span class="label-text">{label}</span>
            <div class="tooltip-container" style="flex: 1; min-width: 0;">
                <input 
                    type="text" 
                    class="label-input" 
                    maxlength="30"
                    prop:value=value
                    on:input=move |ev| on_change_input(event_target_value(&ev))
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
