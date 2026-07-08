use leptos::*;
use crate::components::label_field::LabelField;
use crate::state::CharacterData;

#[component]
pub fn LabelColumn(
    fields: Vec<(&'static str, &'static str)>,
) -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    let update_label = move |key: String, value: String| {
        set_data.update(|s| {
            if key == "Nome" {
                s.name = value.clone();
            }
            s.labels.insert(key, value);
        });
    };

    view! {
        <div class="info-column">
            {fields.into_iter().map(|(label, key)| {
                let key_str = key.to_string();
                let key_str2 = key.to_string();
                let value = Signal::derive({
                    let key = key_str.clone();
                    move || data.get().labels.get(&key).cloned().unwrap_or_default()
                });
                view! {
                    <LabelField 
                        label=label 
                        value=value
                        on_change=move |v| update_label(key_str2.clone(), v)
                    />
                }
            }).collect_view()}
        </div>
    }
}
