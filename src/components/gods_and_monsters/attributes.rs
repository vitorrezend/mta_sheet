use leptos::*;
use crate::components::ValueField;
use crate::state::{CharacterData, DotOrigin};
use crate::components::character_sheet::ActiveDotOriginContext;

#[component]
pub fn GodsAndMonstersAttributes() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");
    let active_origin_ctx = use_context::<ActiveDotOriginContext>();

    let update_attr = move |name: String, level: Option<i32>, modifier: Option<String>| {
        let current_origin = active_origin_ctx.map(|a| a.origin.get()).unwrap_or(DotOrigin::Base);
        set_data.update(|s| {
            s.set_attribute_with_origin(&name, level, modifier, current_origin);
        });
    };

    let update_attr_dot = move |name: String, dot_idx: usize, origin: DotOrigin| {
        set_data.update(|s| {
            s.set_attribute_dot_origin(&name, dot_idx, origin);
        });
    };

    let attr_field = move |name: &'static str| {
        let name_str = name.to_string();
        let name_str2 = name.to_string();
        let name_str3 = name.to_string();
        let name_str4 = name.to_string();
        let name_str5 = name.to_string();
        let name_str6 = name.to_string();

        let level = Signal::derive({
            let name = name_str.clone();
            move || data.with(|d| d.get_attribute_level(&name, 1))
        });
        let modifier = Signal::derive({
            let name = name_str2.clone();
            move || data.with(|d| d.get_attribute_modifier(&name))
        });
        let origins = Signal::derive({
            let name = name_str3.clone();
            move || data.with(|d| d.attributes.get(&name).map(|a| a.get_origins(5)).unwrap_or_else(|| vec![DotOrigin::Base; 5]))
        });
        
        let on_level_change = {
            let name = name_str4.clone();
            move |v| update_attr(name.clone(), Some(v), None)
        };
        let on_modifier_change = {
            let name = name_str5.clone();
            move |m| update_attr(name.clone(), None, Some(m))
        };
        let on_dot_origin_change = {
            let name = name_str6.clone();
            Callback::new(move |(idx, orig)| update_attr_dot(name.clone(), idx, orig))
        };

        view! {
            <ValueField 
                label=Signal::derive(move || name.to_string()) 
                level=level
                modifier=modifier
                origins=origins
                on_level_change=on_level_change
                on_modifier_change=on_modifier_change
                on_dot_origin_change=on_dot_origin_change
                min_level=1
                max_chars=18
            />
        }
    };

    view! {
        <div class="group-box gods-box">
            <span class="group-title">"Attributes"</span>
            <div class="attributes-block">
                <div class="attribute-column">
                    <h3 class="column-title">"Physical"</h3>
                    {attr_field("Strength")}
                    {attr_field("Dexterity")}
                    {attr_field("Stamina")}
                </div>
                
                <div class="attribute-column">
                    <h3 class="column-title">"Social"</h3>
                    {attr_field("Charisma")}
                    {attr_field("Manipulation")}
                    {attr_field("Appearance")}
                </div>

                <div class="attribute-column">
                    <h3 class="column-title">"Mental"</h3>
                    {attr_field("Perception")}
                    {attr_field("Intelligence")}
                    {attr_field("Wits")}
                </div>
            </div>
        </div>
    }
}
