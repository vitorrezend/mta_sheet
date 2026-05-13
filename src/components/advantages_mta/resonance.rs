use leptos::*;
use crate::components::ValueField;
use crate::state::{CharacterState, AttributeValue};

#[component]
pub fn Resonance() -> impl IntoView {
    let category = "Resonance";
    
    let (list, set_list) = create_signal({
        let mut l = CharacterState::load_custom_list(category);
        
        let defaults = [
            ("res_id_entropic", "Entrópico"),
            ("res_id_static", "Estático"),
            ("res_id_dynamic", "Dinâmico"),
        ];

        let mut changed = false;
        // Percorre do último ao primeiro para que o insert(0) mantenha a ordem correta no topo
        for (id, label) in defaults.iter().rev() {
            // Garante que o label exista se estiver vazio
            if CharacterState::load_label(id).is_empty() {
                CharacterState::save_label(id, label);
            }
            
            // Garante que o ID esteja na lista
            if !l.contains(&id.to_string()) {
                l.insert(0, id.to_string());
                changed = true;
            }
        }

        if changed {
            CharacterState::save_custom_list(category, &l);
        }
        l
    });

    let (trigger, set_trigger) = create_signal(0);

    let add_item = move |_| {
        set_list.update(|l| {
            l.push(format!("res_id_{}_{}", l.len() + 1, js_sys::Math::random()));
            CharacterState::save_custom_list(category, l);
        });
    };

    let remove_item = move |id: String| {
        set_list.update(|l| {
            l.retain(|n| n != &id);
            CharacterState::save_custom_list(category, l);
        });
    };

    let update_label = move |id: String, new_label: String| {
        CharacterState::save_label(&id, &new_label);
        set_trigger.update(|t| *t += 1);
    };

    let render_item = move |id: String| {
        let id_level = id.clone();
        let id_mod = id.clone();
        let id_label = id.clone();
        let id_change = id.clone();
        let id_remove = id.clone();
        let id_up_level = id.clone();
        let id_up_mod = id.clone();

        let label = Signal::derive(move || {
            trigger.track();
            let l = CharacterState::load_label(&id_label);
            if l.is_empty() { "".to_string() } else { l }
        });

        let level = Signal::derive(move || {
            trigger.track();
            AttributeValue::load_individual(&id_level).level
        });
        
        let modifier = Signal::derive(move || {
            trigger.track();
            AttributeValue::load_individual(&id_mod).modifier
        });

        view! {
            <ValueField 
                label=label
                level=level
                modifier=modifier
                on_level_change=move |v| {
                    let mut attr = AttributeValue::load_individual(&id_up_level);
                    attr.level = v;
                    attr.save_individual(&id_up_level);
                    set_trigger.update(|t| *t += 1);
                }
                on_modifier_change=move |m| {
                    let mut attr = AttributeValue::load_individual(&id_up_mod);
                    attr.modifier = m;
                    attr.save_individual(&id_up_mod);
                    set_trigger.update(|t| *t += 1);
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
