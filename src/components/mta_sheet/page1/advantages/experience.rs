use leptos::*;
use crate::components::LabelField;
use crate::state::CharacterData;

#[component]
pub fn Experience() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    let name = "Experiência";
    let value = Signal::derive(move || data.with(|d| d.labels.get(name).cloned().unwrap_or_default()));

    let update_value = move |val: String| {
        set_data.update(|s| {
            s.labels.insert(name.to_string(), val);
        });
    };

    view! {
        <div class="experience-column">
            <LabelField 
                label=name
                value=value
                on_change=update_value
            />
        </div>
    }
}
