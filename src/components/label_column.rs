use leptos::*;
use crate::components::label_field::LabelField;
use crate::state::CharacterState;

#[component]
pub fn LabelColumn(
    fields: Vec<(&'static str, &'static str)>,
) -> impl IntoView {
    // Carrega o estado inicial baseado nas chaves fornecidas
    let (state, set_state) = create_signal(fields.iter()
        .map(|&(_, key)| (key.to_string(), CharacterState::load_label(key)))
        .collect::<std::collections::HashMap<_, _>>());

    let update_label = move |name: String, value: String| {
        CharacterState::save_label(&name, &value);
        set_state.update(|s| {
            s.insert(name, value);
        });
    };

    view! {
        <div class="info-column">
            {fields.into_iter().map(|(label, key)| {
                view! {
                    <LabelField 
                        label=label 
                        value=Signal::derive(move || state.get().get(key).cloned().unwrap_or_default())
                        on_change=move |v| update_label(key.to_string(), v)
                    />
                }
            }).collect_view()}
        </div>
    }
}
