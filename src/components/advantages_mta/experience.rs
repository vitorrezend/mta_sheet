use leptos::*;
use crate::components::LabelField;
use crate::state::CharacterState;

#[component]
pub fn Experience() -> impl IntoView {
    let name = "Experiência";
    let (value, set_value) = create_signal(CharacterState::load_label(name));

    let update_value = move |val: String| {
        set_value.set(val.clone());
        CharacterState::save_label(name, &val);
    };

    view! {
        <div class="experience-column">
            <LabelField 
                label=name
                value=Signal::derive(move || value.get())
                on_change=update_value
            />
        </div>
    }
}
