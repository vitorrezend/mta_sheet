use leptos::*;
use crate::components::{Callback, ValueField};
use crate::state::{keys, CharacterData, DotOrigin};
use crate::components::character_sheet::ActiveDotOriginContext;

#[component]
pub fn MeritsFlaws() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");
    let active_origin_ctx = use_context::<ActiveDotOriginContext>();
    let compendium_ctx = use_context::<crate::components::mta_sheet::page5::PracticeCompendiumContext>();
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

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
            s.labels.insert(id, crate::i18n::tr("new_merit", lang()).to_string());
        });
    };

    let add_flaw = move |_| {
        set_data.update(|s| {
            let list = s.custom_lists.entry(keys::CAT_FLAWS.to_string()).or_default();
            let id = format!("flaw_{}", uuid::Uuid::new_v4());
            list.push(id.clone());
            s.labels.insert(id, crate::i18n::tr("new_flaw", lang()).to_string());
        });
    };

    let remove_item = move |category: &'static str, id: String| {
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

    let compendium_ctx_for_rows = compendium_ctx.clone();
    let render_item = std::rc::Rc::new(move |category: &'static str, id: String| {
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
        let id_for_slot = id.clone();

        let label = Signal::derive({
            let id = id_label.clone();
            move || {
                let l = lang();
                data.with(|d| {
                    let lbl = d.labels.get(&id).cloned().unwrap_or_default();
                    let lbl_trim = lbl.trim();
                    if lbl_trim == "Nova Qualidade" || lbl_trim == "New Merit" {
                        crate::i18n::tr("new_merit", l).to_string()
                    } else if lbl_trim == "Novo Defeito" || lbl_trim == "New Flaw" {
                        crate::i18n::tr("new_flaw", l).to_string()
                    } else {
                        lbl
                    }
                })
            }
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
        let on_row_open_compendium = move |_| {
            if let Some(ref c) = compendium_ctx_row {
                let pos = if category == keys::CAT_MERITS {
                    merits_list.with(|l| l.iter().position(|x| x == &id_for_slot).unwrap_or(0))
                } else {
                    flaws_list.with(|l| l.iter().position(|x| x == &id_for_slot).unwrap_or(0))
                };
                c.open_merit_flaw.call((Some(pos), label_row.get()));
            }
        };

        view! {
            <div class="merit-flaw-row-wrapper" style="display: flex; align-items: center; gap: 0.15rem; width: 100%;">
                <button
                    type="button"
                    class="compendium-row-btn"
                    style="background: none; border: none; cursor: pointer; font-size: 0.78rem; padding: 0.1rem 0.2rem; opacity: 0.65; transition: opacity 0.15s ease; flex-shrink: 0;"
                    on:click=on_row_open_compendium
                    title=move || match lang() {
                        crate::i18n::Language::PtBr => "Consultar ou substituir no Compêndio",
                        crate::i18n::Language::EnUs => "Look up or replace in Compendium",
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
                        max_chars=32
                    />
                </div>
            </div>
        }
    });

    let ctx_for_merits = compendium_ctx.clone();
    let on_open_merits_compendium = move |_| {
        if let Some(ref c) = ctx_for_merits {
            c.open_merit_flaw.call((None, "merit".to_string()));
        }
    };

    let ctx_for_flaws = compendium_ctx.clone();
    let on_open_flaws_compendium = move |_| {
        if let Some(ref c) = ctx_for_flaws {
            c.open_merit_flaw.call((None, "flaw".to_string()));
        }
    };

    view! {
        <div class="group-box">
            <span class="group-title">{move || crate::i18n::tr("merits_flaws", lang())}</span>
            <div class="attributes-block merits-flaws-block">
                // Coluna 1: Qualidades (Merits)
                <div class="attribute-column">
                    <div class="attribute-column-header" style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 0.35rem;">
                        <h3 class="column-title" style="margin-bottom: 0;">{move || crate::i18n::tr("merits", lang())}</h3>
                        <button
                            type="button"
                            class="field-compendium-lookup-btn"
                            on:click=on_open_merits_compendium
                            title=move || match lang() {
                                crate::i18n::Language::PtBr => "Consultar Compêndio M20 de Qualidades",
                                crate::i18n::Language::EnUs => "Browse M20 Merits Compendium",
                            }
                        >
                            "📖"
                        </button>
                    </div>
                    {
                        let render_item_merits = render_item.clone();
                        view! {
                            <For
                                each=move || merits_list.get()
                                key=|id| id.clone()
                                children={
                                    let render_item_merits = render_item_merits.clone();
                                    move |id| render_item_merits(keys::CAT_MERITS, id)
                                }
                            />
                        }
                    }
                    <button class="add-field-btn" on:click=add_merit title=move || crate::i18n::tr("add_merit", lang())>"+"</button>
                </div>

                // Coluna 2: Defeitos (Flaws)
                <div class="attribute-column">
                    <div class="attribute-column-header" style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 0.35rem;">
                        <h3 class="column-title" style="margin-bottom: 0;">{move || crate::i18n::tr("flaws", lang())}</h3>
                        <button
                            type="button"
                            class="field-compendium-lookup-btn"
                            on:click=on_open_flaws_compendium
                            title=move || match lang() {
                                crate::i18n::Language::PtBr => "Consultar Compêndio M20 de Defeitos",
                                crate::i18n::Language::EnUs => "Browse M20 Flaws Compendium",
                            }
                        >
                            "📖"
                        </button>
                    </div>
                    {
                        let render_item_flaws = render_item.clone();
                        view! {
                            <For
                                each=move || flaws_list.get()
                                key=|id| id.clone()
                                children={
                                    let render_item_flaws = render_item_flaws.clone();
                                    move |id| render_item_flaws(keys::CAT_FLAWS, id)
                                }
                            />
                        }
                    }
                    <button class="add-field-btn" on:click=add_flaw title=move || crate::i18n::tr("add_flaw", lang())>"+"</button>
                </div>
            </div>
        </div>
    }
}
