use leptos::*;
use crate::state::{AttributeValue};

#[component]
pub fn QuintessenceParadox() -> impl IntoView {
    let states_key = "quintessence_paradox_states";
    let q_total_key = "quintessence";
    let p_total_key = "paradox";

    // Carrega os estados individuais (string de 20 caracteres)
    let initial_states = {
        let val = AttributeValue::load_individual(states_key).modifier;
        if val.len() == 20 { val } else { "0".repeat(20) }
    };

    let (states, set_states) = create_signal(initial_states);

    // Totais derivados para compatibilidade com outras partes do sistema
    let quintessence_total = Signal::derive(move || {
        states.get().chars().filter(|&c| c == '1').count() as i32
    });
    let paradox_total = Signal::derive(move || {
        states.get().chars().filter(|&c| c == '2').count() as i32
    });

    let update_state = move |index: usize| {
        set_states.update(|s| {
            let mut chars: Vec<char> = s.chars().collect();
            if index < chars.len() {
                // Ciclo: 0 (vazio) -> 1 (quint) -> 2 (paradox) -> 0
                let next = match chars[index] {
                    '0' => '1',
                    '1' => '2',
                    _ => '0',
                };
                chars[index] = next;
                *s = chars.into_iter().collect();
                
                // Persiste os estados
                let mut attr = AttributeValue::load_individual(states_key);
                attr.modifier = s.clone();
                attr.save_individual(states_key);

                // Persiste os totais para compatibilidade
                let q_count = s.chars().filter(|&c| c == '1').count() as i32;
                let p_count = s.chars().filter(|&c| c == '2').count() as i32;
                
                let mut attr_q = AttributeValue::load_individual(q_total_key);
                attr_q.level = q_count;
                attr_q.save_individual(q_total_key);

                let mut attr_p = AttributeValue::load_individual(p_total_key);
                attr_p.level = p_count;
                attr_p.save_individual(p_total_key);
            }
        });
    };

    let render_box = move |i: usize| {
        let state = move || states.get().chars().nth(i).unwrap_or('0');
        let is_quint = move || state() == '1';
        let is_paradox = move || state() == '2';

        view! {
            <span 
                class="square qp-square"
                class:quintessence=is_quint
                class:paradox=is_paradox
                on:click=move |_| update_state(i)
            ></span>
        }
    };

    view! {
        <div class="quintessence-paradox-container">
            <h3 class="column-title">"Quintessência / Paradoxo"</h3>
            
            <div class="qp-grid" style="margin: 0.8rem 0;">
                // Primeira linha (1-10)
                <div class="dots-container" style="justify-content: center; gap: 4px; margin-bottom: 4px;">
                    {(0..10).map(|i| render_box(i)).collect_view()}
                </div>
                // Segunda linha (11-20)
                <div class="dots-container" style="justify-content: center; gap: 4px;">
                    {(10..20).map(|i| render_box(i)).collect_view()}
                </div>
            </div>

            <div class="qp-totals" style="display: flex; justify-content: center; gap: 1rem; font-size: 0.7rem; color: #7f8c8d; font-weight: bold; text-transform: uppercase;">
                <span>"Q: " {quintessence_total}</span>
                <span>"P: " {paradox_total}</span>
            </div>
        </div>
    }
}
