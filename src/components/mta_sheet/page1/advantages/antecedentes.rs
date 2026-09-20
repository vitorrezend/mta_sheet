use leptos::*;
use crate::components::{Callback, ValueField};
use crate::state::{CharacterData, DotOrigin};
use crate::components::character_sheet::ActiveDotOriginContext;

#[component]
pub fn Antecedentes() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");
    let active_origin_ctx = use_context::<ActiveDotOriginContext>();

    let category = "Antecedentes";
    
    let list = Signal::derive(move || {
        data.with(|d| d.custom_lists.get(category).cloned().unwrap_or_default())
    });

    let add_item = move |_| {
        set_data.update(|s| {
            let list = s.custom_lists.entry(category.to_string()).or_default();
            list.push(format!("bg_{}", uuid::Uuid::new_v4()));
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

    let compendium_ctx = use_context::<crate::components::mta_sheet::page5::PracticeCompendiumContext>();
    let compendium_ctx_for_rows = compendium_ctx.clone();
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

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
        let id_sup = id.clone();
        let id_up_sup = id.clone();
        let id_for_idx = id.clone();

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

        let is_supernatural = Signal::derive({
            let id = id_sup.clone();
            move || data.with(|d| d.is_attribute_supernatural(&id))
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

        let on_toggle_supernatural = {
            let id = id_up_sup.clone();
            Callback::new(move |_| {
                let id = id.clone();
                set_data.update(|s| {
                    s.toggle_attribute_supernatural(&id);
                });
            })
        };

        let compendium_ctx_row = compendium_ctx_for_rows.clone();
        let label_row = label.clone();
        let id_row_pos = id_for_idx.clone();
        let on_row_open_compendium = move |_| {
            if let Some(ref c) = compendium_ctx_row {
                let pos = list.with(|l| l.iter().position(|x| x == &id_row_pos).unwrap_or(0));
                c.open_background.call((Some(pos), label_row.get()));
            }
        };

        view! {
            <div class="antecedente-row-wrapper" style="display: flex; align-items: center; gap: 0.15rem; width: 100%;">
                <button
                    type="button"
                    class="compendium-row-btn"
                    style="background: none; border: none; cursor: pointer; font-size: 0.78rem; padding: 0.1rem 0.2rem; opacity: 0.65; transition: opacity 0.15s ease; flex-shrink: 0;"
                    on:click=on_row_open_compendium
                    title=move || match lang_ctx.map(|c| c.lang.get()).unwrap_or_default() {
                        crate::i18n::Language::PtBr => "Consultar ou substituir este Antecedente no Compêndio",
                        crate::i18n::Language::EnUs => "Look up or replace this Background in Compendium",
                    }
                >
                    "📖"
                </button>
                <div style="flex: 1; min-width: 0;">
                    <ValueField 
                        label=label
                        level=level
                        modifier=modifier
                        origins=origins
                        is_supernatural=is_supernatural
                        on_toggle_supernatural=on_toggle_supernatural
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
                </div>
            </div>
        }
    };

    let ctx_for_header = compendium_ctx.clone();
    let on_header_open_compendium = move |_| {
        if let Some(ref c) = ctx_for_header {
            c.open_background.call((None, String::new()));
        }
    };

    view! {
        <div class="antecedentes-column">
            <div class="antecedentes-header-row" style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 0.35rem;">
                <h3 class="column-title" style="margin-bottom: 0;">{move || crate::i18n::tr("backgrounds", lang())}</h3>
                <button
                    type="button"
                    class="field-compendium-lookup-btn"
                    on:click=on_header_open_compendium
                    title=move || match lang() {
                        crate::i18n::Language::PtBr => "Consultar Compêndio M20 de Antecedentes",
                        crate::i18n::Language::EnUs => "Browse M20 Backgrounds Compendium",
                    }
                >
                    "📖"
                </button>
            </div>
            <For
                each=move || list.get()
                key=|id| id.clone()
                children=render_item
            />
            <button
                type="button"
                class="add-field-btn"
                on:click=add_item
                title=move || match lang() {
                    crate::i18n::Language::PtBr => "Adicionar Antecedente",
                    crate::i18n::Language::EnUs => "Add Background",
                }
            >
                "+"
            </button>
        </div>
    }
}
