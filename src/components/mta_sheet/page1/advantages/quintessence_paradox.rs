use leptos::*;
use crate::state::CharacterData;

#[component]
pub fn QuintessenceParadox() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    let states_key = "quintessence_paradox_states";

    let states = Signal::derive(move || {
        let val = data.with(|d| d.labels.get(states_key).cloned().unwrap_or_default());
        if val.len() == 20 { val } else { "0".repeat(20) }
    });

    let quintessence_total = Signal::derive(move || {
        states.get().chars().filter(|&c| c == '1').count() as i32
    });
    let paradox_total = Signal::derive(move || {
        states.get().chars().filter(|&c| c == '2').count() as i32
    });

    let update_state = move |index: usize| {
        set_data.update(|s| {
            let current = s.labels.entry(states_key.to_string()).or_insert_with(|| "0".repeat(20));
            if current.len() != 20 { *current = "0".repeat(20); }

            let mut chars: Vec<char> = current.chars().collect();
            if index < chars.len() {
                let next = match chars[index] {
                    '0' => '1',
                    '1' => '2',
                    _ => '0',
                };
                chars[index] = next;
                *current = chars.into_iter().collect();
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
                <div class="dots-container" style="justify-content: center; gap: 4px; margin-bottom: 4px;">
                    {(0..10).map(|i| render_box(i)).collect_view()}
                </div>
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
