use leptos::*;
use crate::components::{Callback, ValueField};
use crate::state::{CharacterData, DotOrigin};
use crate::components::character_sheet::ActiveDotOriginContext;

#[component]
pub fn Resonance() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");
    let active_origin_ctx = use_context::<ActiveDotOriginContext>();

    let category = "Resonance";

    let list = Signal::derive(move || {
        data.with(|d| d.custom_lists.get(category).cloned().unwrap_or_default())
    });

    let add_item = move |_| {
        set_data.update(|s| {
            let list = s.custom_lists.entry(category.to_string()).or_default();
            list.push(format!("res_{}", uuid::Uuid::new_v4()));
        });
    };

    let remove_item = move |id: String| {
        request_animation_frame(move || {
            set_data.update(|s| {
                if let Some(list) = s.custom_lists.get_mut(category) {
                    list.retain(|n| n != &id);
                }
                s.labels.remove(&id);
                s.attributes.remove(&id);
            });
        });
    };

    let update_label = move |id: String, new_label: String| {
        set_data.update(|s| {
            s.labels.insert(id, new_label);
        });
    };

    let render_item = move |id: String| {
        let id_level = id.clone();
        let id_mod = id.clone();
        let id_origins = id.clone();
        let id_label = id.clone();
        let id_change = id.clone();
        let id_remove = id.clone();
        let id_up_level = id.clone();
        let id_up_mod = id.clone();
        let id_up_dot = id.clone();

        let label = Signal::derive({
            let id = id_label.clone();
            move || data.with(|d| d.labels.get(&id).cloned().unwrap_or_default())
        });

        let level = Signal::derive({
            let id = id_level.clone();
            move || data.with(|d| d.get_attribute_level(&id, 0))
        });
        
        let modifier = Signal::derive({
            let id = id_mod.clone();
            move || data.with(|d| d.get_attribute_modifier(&id))
        });

        let origins = Signal::derive({
            let id = id_origins.clone();
            move || data.with(|d| d.attributes.get(&id).map(|a| a.get_origins(5)).unwrap_or_else(|| vec![DotOrigin::Base; 5]))
        });

        let on_dot_origin_change = {
            let id = id_up_dot.clone();
            Callback::new(move |(idx, orig)| {
                let id = id.clone();
                set_data.update(|s| {
                    s.set_attribute_dot_origin(&id, idx, orig);
                });
            })
        };

        view! {
            <ValueField 
                label=label
                level=level
                modifier=modifier
                origins=origins
                on_level_change={
                    let id = id_up_level.clone();
                    move |v| {
                        let current_origin = active_origin_ctx.map(|a| a.origin.get()).unwrap_or(DotOrigin::Base);
                        set_data.update(|s| {
                            s.set_attribute_with_origin(&id, Some(v), None, current_origin);
                        });
                    }
                }
                on_modifier_change={
                    let id = id_up_mod.clone();
                    move |m| {
                        set_data.update(|s| {
                            s.set_attribute_with_origin(&id, None, Some(m), DotOrigin::Base);
                        });
                    }
                }
                on_dot_origin_change=on_dot_origin_change
                is_editable=true
                on_label_change=Callback::new(move |new_l| update_label(id_change.clone(), new_l))
                on_remove=Callback::new(move |_| remove_item(id_remove.clone()))
                max_chars=18
            />
        }
    };

    view! {
        <div class="resonance-column">
            <h3 class="column-title">"RESSONÂNCIA"</h3>
            <For
                each=move || list.get()
                key=|id| id.clone()
                children=render_item
            />
            <button class="add-field-btn" on:click=add_item>"+"</button>
        </div>
    }
}
