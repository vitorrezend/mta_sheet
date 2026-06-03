use leptos::*;
use crate::components::ValueField;
use crate::state::CharacterData;

#[component]
pub fn Resonance() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    let category = "Resonance";
    
    // Inicialização de itens padrão se a lista estiver vazia
    create_effect(move |_| {
        set_data.update(|s| {
            let list = s.custom_lists.entry(category.to_string()).or_default();
            if list.is_empty() {
                let defaults = [
                    ("res_entropic", "Entrópico"),
                    ("res_static", "Estático"),
                    ("res_dynamic", "Dinâmico"),
                ];
                for (id, label) in defaults {
                    list.push(id.to_string());
                    s.labels.insert(id.to_string(), label.to_string());
                }
            }
        });
    });

    let list = Signal::derive(move || {
        data.get().custom_lists.get(category).cloned().unwrap_or_default()
    });

    let add_item = move |_| {
        set_data.update(|s| {
            let list = s.custom_lists.entry(category.to_string()).or_default();
            list.push(format!("res_{}", uuid::Uuid::new_v4()));
        });
    };

    let remove_item = move |id: String| {
        set_data.update(|s| {
            if let Some(list) = s.custom_lists.get_mut(category) {
                list.retain(|n| n != &id);
            }
            s.labels.remove(&id);
            s.attributes.remove(&id);
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
        let id_label = id.clone();
        let id_change = id.clone();
        let id_remove = id.clone();
        let id_up_level = id.clone();
        let id_up_mod = id.clone();

        let label = Signal::derive({
            let id = id_label.clone();
            move || data.get().labels.get(&id).cloned().unwrap_or_default()
        });

        let level = Signal::derive({
            let id = id_level.clone();
            move || data.get().attributes.get(&id).map(|a| a.level).unwrap_or(0)
        });
        
        let modifier = Signal::derive({
            let id = id_mod.clone();
            move || data.get().attributes.get(&id).map(|a| a.modifier.clone()).unwrap_or_default()
        });

        view! {
            <ValueField 
                label=label
                level=level
                modifier=modifier
                on_level_change={
                    let id = id_up_level.clone();
                    move |v| {
                        set_data.update(|s| {
                            s.attributes.entry(id.clone()).or_default().level = v;
                        });
                    }
                }
                on_modifier_change={
                    let id = id_up_mod.clone();
                    move |m| {
                        set_data.update(|s| {
                            s.attributes.entry(id.clone()).or_default().modifier = m;
                        });
                    }
                }
                is_editable=true
                on_label_change=Some(Callback::new(move |new_l| update_label(id_change.clone(), new_l)))
                on_remove=Some(Callback::new(move |_| remove_item(id_remove.clone())))
                max_chars=12
            />
        }
    };

    view! {
        <div class="resonance-column">
            <h3 class="column-title">"RESSONÂNCIA"</h3>
            {move || list.get().into_iter().map(render_item).collect_view()}
            <button class="add-field-btn" on:click=add_item>"+"</button>
        </div>
    }
}
