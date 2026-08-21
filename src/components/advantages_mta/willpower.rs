use leptos::*;
use crate::state::CharacterData;

#[component]
pub fn Willpower() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    let total_name = "willpower_total";
    let current_name = "willpower_current";

    let total = Signal::derive(move || {
        let val = data.with(|d| d.attributes.get(total_name).map(|a| a.level).unwrap_or(0));
        if val == 0 { 5 } else { val }
    });

    let current = Signal::derive(move || {
        data.with(|d| d.attributes.get(current_name).map(|a| a.level).unwrap_or(0))
    });

    let update_total = move |new_val: i32| {
        let val = new_val.max(1);
        set_data.update(|s| {
            s.attributes.entry(total_name.to_string()).or_default().level = val;
            if let Some(c_attr) = s.attributes.get_mut(current_name) {
                if c_attr.level > val {
                    c_attr.level = val;
                }
            }
        });
    };

    let update_current = move |new_val: i32| {
        let val = new_val.clamp(0, total.get());
        set_data.update(|s| {
            s.attributes.entry(current_name.to_string()).or_default().level = val;
        });
    };

    view! {
        <div class="willpower-container" style="margin-top: 1.2rem;">
            <h3 class="column-title">"Força de Vontade"</h3>
            
            <div class="dots-container" style="justify-content: center; margin-bottom: 0.6rem; gap: 6px;">
                {(1..=10).map(|i| {
                    let is_filled = move || total.get() >= i;
                    view! {
                        <span 
                            class="dot"
                            class:filled=is_filled
                            on:click=move |_| {
                                let current_total = total.get();
                                let new_val = if i == current_total {
                                    (i - 1).max(1)
                                } else {
                                    i
                                };
                                update_total(new_val);
                            }
                        ></span>
                    }
                }).collect_view()}
            </div>

            <div class="dots-container" style="justify-content: center; gap: 6px;">
                {(1..=10).map(|i| {
                    let is_filled = move || current.get() >= i;
                    view! {
                        <span 
                            class="square"
                            class:filled=is_filled
                            on:click=move |_| {
                                let cur = current.get();
                                let new_val = if i == cur {
                                    i - 1
                                } else {
                                    i
                                };
                                update_current(new_val);
                            }
                        ></span>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
