use leptos::*;

#[component]
pub fn LabelField(
    label: &'static str,
    value: Signal<String>,
    on_change: impl Fn(String) + 'static,
) -> impl IntoView {
    let on_change = store_value(on_change);

    view! {
        <div class="label-field">
            <span class="label-text">{label}</span>
            <div class="tooltip-container" style="flex: 1; min-width: 0;">
                <input 
                    type="text" 
                    class="label-input" 
                    maxlength="30"
                    prop:value=value
                    on:input=move |ev| on_change.with_value(|cb| cb(event_target_value(&ev)))
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
