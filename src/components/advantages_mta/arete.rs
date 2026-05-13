use leptos::*;
use crate::state::AttributeValue;

#[component]
pub fn Arete() -> impl IntoView {
    let name = "Arete";
    let (level, set_level) = create_signal(AttributeValue::load_individual(name).level.max(1));

    let update_level = move |new_val: i32| {
        let val = new_val.max(1); // Mínimo 1 conforme solicitado
        set_level.set(val);
        let mut attr = AttributeValue::load_individual(name);
        attr.level = val;
        attr.save_individual(name);
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
