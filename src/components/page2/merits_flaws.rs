use leptos::*;
use crate::components::ValueField;
use crate::state::{keys, CharacterData, DotOrigin};
use crate::components::character_sheet::ActiveDotOriginContext;

#[component]
pub fn MeritsFlaws() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");
    let active_origin_ctx = use_context::<ActiveDotOriginContext>();

    let merits_list = Signal::derive(move || {
        data.with(|d| d.custom_lists.get(keys::CAT_MERITS).cloned().unwrap_or_default())
    });

    let flaws_list = Signal::derive(move || {
        data.with(|d| d.custom_lists.get(keys::CAT_FLAWS).cloned().unwrap_or_default())
    });

    let add_merit = move |_| {
        set_data.update(|s| {
            let list = s.custom_lists.entry(keys::CAT_MERITS.to_string()).or_default();
            let id = format!("merit_{}", uuid::Uuid::new_v4());
            list.push(id.clone());
            s.labels.insert(id, "Nova Qualidade".to_string());
        });
    };

    let add_flaw = move |_| {
        set_data.update(|s| {
            let list = s.custom_lists.entry(keys::CAT_FLAWS.to_string()).or_default();
            let id = format!("flaw_{}", uuid::Uuid::new_v4());
            list.push(id.clone());
            s.labels.insert(id, "Novo Defeito".to_string());
        });
    };

    let remove_item = move |category: &'static str, id: String| {
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

    let render_item = move |category: &'static str, id: String| {
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
            move || data.with(|d| d.attributes.get(&id).map(|a| a.get_origins(10)).unwrap_or_else(|| vec![DotOrigin::Base; 10]))
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
            <div class="merit-flaw-item-wrapper">
                <ValueField 
                    label=label
                    level=level
                    modifier=modifier
                    origins=origins
                    max_level=10
                    min_level=0
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
                    on_remove=Callback::new(move |_| remove_item(category, id_remove.clone()))
                    max_chars=20
                />
            </div>
        }
    };

    view! {
        <div class="group-box merits-flaws-box">
            <div class="merits-flaws-flex-grid">
                // Coluna: Qualidades (Merits)
                <div class="merits-flaws-column">
                    <div class="column-header-row">
                        <span class="column-header-title">"QUALIDADES (MERITS)"</span>
                        <button type="button" class="add-section-item-btn" on:click=add_merit title="Adicionar Qualidade">
                            "+ Qualidade"
                        </button>
                    </div>

                    <div class="merits-flaws-items-list">
                        {move || {
                            let items = merits_list.get();
                            if items.is_empty() {
                                view! {
                                    <div class="empty-column-hint">
                                        "Nenhuma qualidade adicionada. Clique em '+ Qualidade' para incluir."
                                    </div>
                                }.into_view()
                            } else {
                                items.into_iter().map(|id| render_item(keys::CAT_MERITS, id)).collect_view().into_view()
                            }
                        }}
                    </div>
                </div>

                // Coluna: Defeitos (Flaws)
                <div class="merits-flaws-column">
                    <div class="column-header-row">
                        <span class="column-header-title">"DEFEITOS (FLAWS)"</span>
                        <button type="button" class="add-section-item-btn" on:click=add_flaw title="Adicionar Defeito">
                            "+ Defeito"
                        </button>
                    </div>

                    <div class="merits-flaws-items-list">
                        {move || {
                            let items = flaws_list.get();
                            if items.is_empty() {
                                view! {
                                    <div class="empty-column-hint">
                                        "Nenhum defeito adicionado. Clique em '+ Defeito' para incluir."
                                    </div>
                                }.into_view()
                            } else {
                                items.into_iter().map(|id| render_item(keys::CAT_FLAWS, id)).collect_view().into_view()
                            }
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
