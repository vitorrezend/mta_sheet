use leptos::*;
use crate::components::ValueField;
use crate::state::{CharacterData, AttributeValue};

#[component]
pub fn Attributes() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    let update_attr = move |name: String, level: Option<i32>, modifier: Option<String>| {
        set_data.update(|s| {
            let attr = s.attributes.entry(name).or_insert(AttributeValue::default());
            if let Some(l) = level { attr.level = l; }
            if let Some(m) = modifier { attr.modifier = m; }
        });
    };

    let attr_field = move |name: &'static str| {
        let name_str = name.to_string();
        let name_str2 = name.to_string();
        let name_str3 = name.to_string();
        let level = Signal::derive({
            let name = name_str.clone();
            move || data.get().attributes.get(&name).map(|a| a.level).unwrap_or(1).max(1)
        });
        let modifier = Signal::derive({
            let name = name_str2.clone();
            move || data.get().attributes.get(&name).map(|a| a.modifier.clone()).unwrap_or_default()
        });
        
        let on_level_change = {
            let name = name_str3.clone();
            move |v| update_attr(name.clone(), Some(v), None)
        };
        let on_modifier_change = {
            let name = name_str3.clone();
            move |m| update_attr(name.clone(), None, Some(m))
        };

        view! {
            <ValueField 
                label=Signal::derive(move || name.to_string()) 
                level=level
                modifier=modifier
                on_level_change=on_level_change
                on_modifier_change=on_modifier_change
                min_level=1
                max_chars=18
            />
        }
    };

    view! {
        <div class="group-box">
            <span class="group-title">"Atributos"</span>
            <div class="attributes-block">
                <AttributeColumn title="Físicos">
                    {attr_field("Força")}
                    {attr_field("Destreza")}
                    {attr_field("Vigor")}
                </AttributeColumn>
                
                <AttributeColumn title="Sociais">
                    {attr_field("Carisma")}
                    {attr_field("Manipulação")}
                    {attr_field("Aparência")}
                </AttributeColumn>

                <AttributeColumn title="Mentais">
                    {attr_field("Percepção")}
                    {attr_field("Inteligência")}
                    {attr_field("Raciocínio")}
                </AttributeColumn>
            </div>
        </div>
    }
}

#[component]
fn AttributeColumn(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="attribute-column">
            <h3 class="column-title">{title}</h3>
            {children()}
        </div>
    }
}
