use leptos::*;
use crate::components::LabelField;
use crate::state::CharacterData;

#[component]
pub fn GodsAndMonstersHeader() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    let update_name = move |new_val: String| {
        set_data.update(|d| d.name = new_val);
    };

    let update_label = move |key: &'static str, val: String| {
        set_data.update(|d| {
            d.set_label(key, val);
        });
    };

    view! {
        <div class="gods-header-container">
            <div class="gods-title-banner">
                <h1 class="gods-main-title">"GODS & MONSTERS"</h1>
            </div>

            <div class="header-grid gods-header-grid">
                // Column 1
                <div class="header-column">
                    <LabelField 
                        label="Name:" 
                        value=Signal::derive(move || data.with(|d| d.name.clone()))
                        on_change=update_name
                    />
                    <LabelField 
                        label="Chronicle:" 
                        value=Signal::derive(move || data.with(|d| d.get_label("Chronicle")))
                        on_change=move |v| update_label("Chronicle", v)
                    />
                </div>

                // Column 2
                <div class="header-column">
                    <LabelField 
                        label="Nature:" 
                        value=Signal::derive(move || data.with(|d| d.get_label("Nature")))
                        on_change=move |v| update_label("Nature", v)
                    />
                    <LabelField 
                        label="Demeanor:" 
                        value=Signal::derive(move || data.with(|d| d.get_label("Demeanor")))
                        on_change=move |v| update_label("Demeanor", v)
                    />
                </div>

                // Column 3
                <div class="header-column">
                    <LabelField 
                        label="Type:" 
                        value=Signal::derive(move || data.with(|d| d.get_label("Type")))
                        on_change=move |v| update_label("Type", v)
                    />
                    <LabelField 
                        label="Concept:" 
                        value=Signal::derive(move || data.with(|d| d.get_label("Concept")))
                        on_change=move |v| update_label("Concept", v)
                    />
                </div>
            </div>
        </div>
    }
}
