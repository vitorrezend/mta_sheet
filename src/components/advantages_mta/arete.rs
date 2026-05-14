use leptos::*;
use crate::state::{CharacterData, AttributeValue};

#[component]
pub fn Arete() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    let name = "Arete";
    let level = Signal::derive(move || data.get().attributes.get(name).map(|a| a.level).unwrap_or(1).max(1));

    let update_level = move |new_val: i32| {
        let val = new_val.max(1);
        set_data.update(|s| {
            s.attributes.entry(name.to_string()).or_default().level = val;
        });
    };

    view! {
        <div class="arete-container">
            <h3 class="column-title">"Arete"</h3>
            <div class="dots-container" style="justify-content: center; margin-bottom: 0.5rem;">
                {(1..=10).map(|i| {
                    let is_filled = move || level.get() >= i;
                    view! {
                        <span 
                            class="dot"
                            class:filled=is_filled
                            on:click=move |_| {
                                let current = level.get();
                                let new_val = if i == current {
                                    (i - 1).max(1)
                                } else {
                                    i
                                };
                                update_level(new_val);
                            }
                        ></span>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
