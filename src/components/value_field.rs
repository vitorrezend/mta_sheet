use leptos::*;

#[component]
pub fn ValueField(
    label: Signal<String>,
    level: Signal<i32>,
    modifier: Signal<String>,
    on_level_change: impl Fn(i32) + 'static,
    on_modifier_change: impl Fn(String) + 'static,
    #[prop(default = 0)] min_level: i32,
    #[prop(optional)] max_chars: Option<usize>,
    on_remove: Option<Callback<()>>,
    #[prop(default = false)] is_editable: bool,
    on_label_change: Option<Callback<String>>,
) -> impl IntoView {
    let on_level_change = store_value(on_level_change);
    let on_modifier_change = store_value(on_modifier_change);

    let display_label = move || {
        let l = label.get();
        if let Some(max) = max_chars {
            if l.chars().count() > max {
                format!("{}...", l.chars().take(max).collect::<String>())
            } else {
                l
            }
        } else {
            l
        }
    };

    let (editing_label, set_editing_label) = create_signal(is_editable && label.get().is_empty());
    let (local_label, set_local_label) = create_signal(label.get());

    // Sincroniza o valor local quando entra em modo de edição
    create_effect(move |_| {
        if editing_label.get() {
            set_local_label.set(label.get());
        }
    });

    view! {
        <div class="attribute-row">
            <div class="tooltip-container">
                {move || if editing_label.get() {
                    view! {
                        <input 
                            type="text" 
                            class="label-edit-input"
                            placeholder="..."
                            maxlength="32"
                            prop:value=local_label
                            on:input=move |ev| set_local_label.set(event_target_value(&ev))
                            on:blur=move |_| {
                                let val = local_label.get();
                                if !val.trim().is_empty() {
                                    if let Some(cb) = on_label_change {
                                        cb.call(val);
                                    }
                                    set_editing_label.set(false);
                                }
                            }
                            on:keydown=move |ev| {
                                if ev.key() == "Enter" {
                                    let val = local_label.get();
                                    if !val.trim().is_empty() {
                                        if let Some(cb) = on_label_change {
                                            cb.call(val);
                                        }
                                        set_editing_label.set(false);
                                    }
                                }
                            }
                            on:mount=move |el: web_sys::HtmlInputElement| { 
                                let _ = el.focus(); 
                            }
                        />
                    }.into_view()
                } else {
                    view! {
                        <span 
                            class="attribute-label"
                            on:dblclick=move |_| if is_editable { set_editing_label.set(true) }
                        >
                            {display_label}
                        </span>
                    }.into_view()
                }}
                <span class="tooltip-text">{label}</span>
            </div>

            <div class="modifier-container tooltip-container">
                <input 
                    type="text" 
                    class="field-modifier" 
                    placeholder="..."
                    maxlength="30"
                    prop:value=modifier
                    on:input=move |ev| on_modifier_change.with_value(|cb| cb(event_target_value(&ev)))
                />
                <span class="tooltip-text" 
                    class:hidden=move || modifier.get().is_empty()
                >
                    {modifier}
                </span>
            </div>

            <div class="dots-container">
                {move || (1..=5).map(|i| {
                    let is_filled = move || level.get() >= i;
                    view! {
                        <span 
                            class="dot"
                            class:filled=is_filled
                            on:click=move |_| {
                                let current = level.get();
                                let new_val = if i == current {
                                    (i - 1).max(min_level)
                                } else {
                                    i
                                };
                                on_level_change.with_value(|cb| cb(new_val))
                            }
                        ></span>
                    }
                }).collect_view()}
            </div>
            
            <div class="action-area">
                {on_remove.map(|cb| view! {
                    <button class="remove-btn" on:click=move |_| cb.call(())>"×"</button>
                })}
            </div>
        </div>
    }
}

