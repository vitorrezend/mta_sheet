use leptos::*;
use crate::state::{AttributeValue};

#[component]
pub fn Willpower() -> impl IntoView {
    let total_name = "willpower_total";
    let current_name = "willpower_current";

    // Carrega o total com padrão 5 se for 0
    let initial_total = {
        let val = AttributeValue::load_individual(total_name).level;
        if val == 0 { 5 } else { val }
    };

    let (total, set_total) = create_signal(initial_total);
    let (current, set_current) = create_signal(AttributeValue::load_individual(current_name).level);

    let update_total = move |new_val: i32| {
        let val = new_val.max(1);
        set_total.set(val);
        let mut attr = AttributeValue::load_individual(total_name);
        attr.level = val;
        attr.save_individual(total_name);
        
        // Garante que o atual não ultrapasse o novo total
        if current.get() > val {
            set_current.set(val);
            let mut c_attr = AttributeValue::load_individual(current_name);
            c_attr.level = val;
            c_attr.save_individual(current_name);
        }
    };

    let update_current = move |new_val: i32| {
        let val = new_val.clamp(0, total.get());
        set_current.set(val);
        let mut attr = AttributeValue::load_individual(current_name);
        attr.level = val;
        attr.save_individual(current_name);
    };

    view! {
        <div class="willpower-container" style="margin-top: 1.2rem;">
            <h3 class="column-title">"Força de Vontade"</h3>
            
            // Força de Vontade Total (Dots)
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

            // Força de Vontade Restante (Squares)
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
