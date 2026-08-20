use leptos::*;
use crate::components::ValueField;
use crate::state::{CharacterData, AttributeValue};

#[component]
pub fn Spheres() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    let update_sphere = move |name: String, level: Option<i32>, modifier: Option<String>| {
        set_data.update(|s| {
            let attr = s.attributes.entry(name).or_insert(AttributeValue::default());
            if let Some(l) = level { attr.level = l; }
            if let Some(m) = modifier { attr.modifier = m; }
        });
    };

    let sphere_field = move |name: &'static str| {
        let name_str = name.to_string();
        let name_str2 = name.to_string();
        let name_str3 = name.to_string();
        let level = Signal::derive({
            let name = name_str.clone();
            move || data.get().attributes.get(&name).map(|a| a.level).unwrap_or(0)
        });
        let modifier = Signal::derive({
            let name = name_str2.clone();
            move || data.get().attributes.get(&name).map(|a| a.modifier.clone()).unwrap_or_default()
        });
        
        let on_level_change = {
            let name = name_str3.clone();
            move |v| update_sphere(name.clone(), Some(v), None)
        };
        let on_modifier_change = {
            let name = name_str3.clone();
            move |m| update_sphere(name.clone(), None, Some(m))
        };

        view! {
            <ValueField 
                label=Signal::derive(move || name.to_string()) 
                level=level
                modifier=modifier
                on_level_change=on_level_change
                on_modifier_change=on_modifier_change
                min_level=0
                max_chars=15
            />
        }
    };

    view! {
        <div class="group-box">
            <span class="group-title">"Esferas"</span>
            <div class="attributes-block">
                <div class="attribute-column">
                    {sphere_field("Correspondência")}
                    {sphere_field("Entropia")}
                    {sphere_field("Forças")}
                </div>
                <div class="attribute-column">
                    {sphere_field("Vida")}
                    {sphere_field("Matéria")}
                    {sphere_field("Mente")}
                </div>
                <div class="attribute-column">
                    {sphere_field("Primórdio")}
                    {sphere_field("Espírito")}
                    {sphere_field("Tempo")}
                </div>
            </div>
        </div>
    }
}
