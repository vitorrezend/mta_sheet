use leptos::*;
use crate::components::{Callback, ValueField};
use crate::state::{CharacterData, DotOrigin};
use crate::components::character_sheet::ActiveDotOriginContext;

const TALENTOS: &[&'static str] = &[
    "Prontidão", "Esportes", "Briga", "Esquiva", "Consciência", 
    "Expressão", "Intimidação", "Liderança", "Manha", "Lábia"
];

const PERICIAS: &[&'static str] = &[
    "Ofícios", "Condução", "Etiqueta", "Armas de Fogo", "Meditação", 
    "Armas Brancas", "Performance", "Furtividade", "Sobrevivência", "Tecnologia"
];

const CONHECIMENTOS: &[&'static str] = &[
    "Acadêmicos", "Computador", "Cosmologia", "Enigmas", "Investigação", 
    "Direito", "Medicina", "Ocultismo", "Esotérica", "Ciência"
];

#[component]
pub fn Abilities() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");
    let active_origin_ctx = use_context::<ActiveDotOriginContext>();
    let compendium_ctx = use_context::<crate::components::mta_sheet::page5::PracticeCompendiumContext>();

    // Função para atualizar uma habilidade
    let update_ability = move |name: String, level: Option<i32>, modifier: Option<String>| {
        let current_origin = active_origin_ctx.map(|a| a.origin.get()).unwrap_or(DotOrigin::Base);
        set_data.update(|s| {
            s.set_attribute_with_origin(&name, level, modifier, current_origin);
        });
    };

    let update_ability_dot = move |name: String, dot_idx: usize, origin: DotOrigin| {
        set_data.update(|s| {
            s.set_attribute_dot_origin(&name, dot_idx, origin);
        });
    };

    let update_ability_supernatural = move |name: String| {
        set_data.update(|s| {
            s.toggle_attribute_supernatural(&name);
        });
    };

    // Função para adicionar novo campo
    let add_custom = move |category: &'static str| {
        set_data.update(|s| {
            let list = s.custom_lists.entry(category.to_string()).or_default();
            let category_prefix = match category {
                "Talentos" => "tal",
                "Perícias" => "per",
                "Conhecimentos" => "con",
                _ => "ab",
            };
            list.push(format!("ab_{}_{}", category_prefix, uuid::Uuid::new_v4()));
        });
    };

    // Função para remover campo
    let remove_custom = move |category: &'static str, id: String| {
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

    // Função para atualizar o nome de um campo personalizado
    let update_custom_name = move |_category: &'static str, id: String, new_name: String| {
        set_data.update(|s| {
            s.labels.insert(id, new_name);
        });
    };

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let compendium_ctx_for_rows = compendium_ctx.clone();

    // Helper para criar o campo de habilidade (estático ou dinâmico)
    let render_field = std::rc::Rc::new(move |name: String, is_custom: bool, category: &'static str| {
        let n_level = name.clone();
        let n_mod = name.clone();
        let n_origins = name.clone();
        let n_label = name.clone();
        let n_change_label = name.clone();
        let n_update_level = name.clone();
        let n_update_mod = name.clone();
        let n_update_dot = name.clone();
        let n_remove = name.clone();
        let n_sup = name.clone();
        let n_toggle_sup = name.clone();
        let n_for_open = name.clone();
        let is_custom_row = is_custom;
        
        let label = Signal::derive({
            let id = n_label.clone();
            move || data.with(|d| {
                if is_custom {
                    d.labels.get(&id).cloned().unwrap_or_else(|| {
                        if id.starts_with("ab_") {
                            String::new()
                        } else {
                            id.clone()
                        }
                    })
                } else {
                    crate::i18n::tr_ability(&id, lang()).to_string()
                }
            })
        });
        let level = Signal::derive({
            let name = n_level.clone();
            move || data.with(|d| d.get_attribute_level(&name, 0))
        });
        let modifier = Signal::derive({
            let name = n_mod.clone();
            move || data.with(|d| d.get_attribute_modifier(&name))
        });
        let origins = Signal::derive({
            let name = n_origins.clone();
            move || data.with(|d| d.attributes.get(&name).map(|a| a.get_origins(10)).unwrap_or_else(|| vec![DotOrigin::Base; 10]))
        });
        let is_supernatural = Signal::derive({
            let name = n_sup.clone();
            move || data.with(|d| d.is_attribute_supernatural(&name))
        });

        let on_dot_origin_change = {
            let name = n_update_dot.clone();
            Callback::new(move |(idx, orig)| update_ability_dot(name.clone(), idx, orig))
        };
        let on_toggle_supernatural = {
            let name = n_toggle_sup.clone();
            Callback::new(move |_| update_ability_supernatural(name.clone()))
        };

        let compendium_ctx_row = compendium_ctx_for_rows.clone();
        let on_row_open_compendium = move |_| {
            if let Some(ref c) = compendium_ctx_row {
                let query_str = if is_custom_row {
                    data.with(|d| d.labels.get(&n_for_open).cloned().unwrap_or_default())
                } else {
                    n_for_open.clone()
                };
                c.open_ability.call((None, query_str));
            }
        };

        let inner_field = if is_custom {
            let old = n_change_label.clone();
            let n = n_remove.clone();
            view! {
                <ValueField 
                    label=label
                    level=level
                    modifier=modifier
                    origins=origins
                    is_supernatural=is_supernatural
                    on_toggle_supernatural=on_toggle_supernatural
                    on_level_change=move |v| update_ability(n_update_level.clone(), Some(v), None)
                    on_modifier_change=move |m| update_ability(n_update_mod.clone(), None, Some(m))
                    on_dot_origin_change=on_dot_origin_change
                    min_level=0
                    max_chars=18
                    is_editable=true
                    on_label_change=Callback::new(move |new_n| update_custom_name(category, old.clone(), new_n))
                    on_remove=Callback::new(move |_| remove_custom(category, n.clone()))
                />
            }.into_view()
        } else {
            view! {
                <ValueField 
                    label=label
                    level=level
                    modifier=modifier
                    origins=origins
                    is_supernatural=is_supernatural
                    on_toggle_supernatural=on_toggle_supernatural
                    on_level_change=move |v| update_ability(n_update_level.clone(), Some(v), None)
                    on_modifier_change=move |m| update_ability(n_update_mod.clone(), None, Some(m))
                    on_dot_origin_change=on_dot_origin_change
                    min_level=0
                    max_chars=18
                    is_editable=false
                />
            }.into_view()
        };

        view! {
            <div class="ability-row-wrapper" style="display: flex; align-items: center; gap: 0.15rem; width: 100%;">
                <button
                    type="button"
                    class="compendium-row-btn"
                    style="background: none; border: none; cursor: pointer; font-size: 0.78rem; padding: 0.1rem 0.2rem; opacity: 0.65; transition: opacity 0.15s ease; flex-shrink: 0;"
                    on:click=on_row_open_compendium
                    title=move || match lang() {
                        crate::i18n::Language::PtBr => "Consultar regras desta Habilidade no Compêndio",
                        crate::i18n::Language::EnUs => "Browse rules for this Ability in Compendium",
                    }
                >
                    "📖"
                </button>
                <div style="flex: 1; min-width: 0;">
                    {inner_field}
                </div>
            </div>
        }
    });

    let ctx_for_talents = compendium_ctx.clone();
    let on_open_talents = Callback::new(move |_| {
        if let Some(ref c) = ctx_for_talents {
            c.open_ability.call((None, "Talentos".to_string()));
        }
    });

    let ctx_for_skills = compendium_ctx.clone();
    let on_open_skills = Callback::new(move |_| {
        if let Some(ref c) = ctx_for_skills {
            c.open_ability.call((None, "Perícias".to_string()));
        }
    });

    let ctx_for_knowledges = compendium_ctx.clone();
    let on_open_knowledges = Callback::new(move |_| {
        if let Some(ref c) = ctx_for_knowledges {
            c.open_ability.call((None, "Conhecimentos".to_string()));
        }
    });

    view! {
        <div class="group-box">
            <span class="group-title">{move || crate::i18n::tr("abilities", lang())}</span>
            <div class="attributes-block">
                {
                    let rf_static = render_field.clone();
                    let rf_custom = render_field.clone();
                    view! {
                        <AbilityColumn 
                            title=Signal::derive(move || crate::i18n::tr("talents", lang()).to_string()) 
                            on_add=Callback::new(move |_| add_custom("Talentos"))
                            on_compendium=Some(on_open_talents)
                        >
                            {TALENTOS.iter().map(move |&n| rf_static(n.to_string(), false, "Talentos")).collect_view()}
                            <For
                                each=move || data.with(|d| d.custom_lists.get("Talentos").cloned().unwrap_or_default())
                                key=|n| n.clone()
                                children=move |n| rf_custom(n, true, "Talentos")
                            />
                        </AbilityColumn>
                    }
                }
                
                {
                    let rf_static = render_field.clone();
                    let rf_custom = render_field.clone();
                    view! {
                        <AbilityColumn 
                            title=Signal::derive(move || crate::i18n::tr("skills", lang()).to_string()) 
                            on_add=Callback::new(move |_| add_custom("Perícias"))
                            on_compendium=Some(on_open_skills)
                        >
                            {PERICIAS.iter().map(move |&n| rf_static(n.to_string(), false, "Perícias")).collect_view()}
                            <For
                                each=move || data.with(|d| d.custom_lists.get("Perícias").cloned().unwrap_or_default())
                                key=|n| n.clone()
                                children=move |n| rf_custom(n, true, "Perícias")
                            />
                        </AbilityColumn>
                    }
                }

                {
                    let rf_static = render_field.clone();
                    let rf_custom = render_field.clone();
                    view! {
                        <AbilityColumn 
                            title=Signal::derive(move || crate::i18n::tr("knowledges", lang()).to_string()) 
                            on_add=Callback::new(move |_| add_custom("Conhecimentos"))
                            on_compendium=Some(on_open_knowledges)
                        >
                            {CONHECIMENTOS.iter().map(move |&n| rf_static(n.to_string(), false, "Conhecimentos")).collect_view()}
                            <For
                                each=move || data.with(|d| d.custom_lists.get("Conhecimentos").cloned().unwrap_or_default())
                                key=|n| n.clone()
                                children=move |n| rf_custom(n, true, "Conhecimentos")
                            />
                        </AbilityColumn>
                    }
                }
            </div>
        </div>
    }
}

#[component]
fn AbilityColumn(
    title: Signal<String>, 
    children: Children, 
    on_add: Callback<()>,
    #[prop(into, default = None)] on_compendium: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <div class="attribute-column">
            <div class="attribute-column-header" style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 0.35rem;">
                <h3 class="column-title" style="margin-bottom: 0;">{move || title.get()}</h3>
                {if let Some(on_browse) = on_compendium {
                    view! {
                        <button
                            type="button"
                            class="field-compendium-lookup-btn"
                            on:click=move |_| on_browse.call(())
                            title="Consultar Compêndio M20 de Habilidades"
                        >
                            "📖"
                        </button>
                    }.into_view()
                } else {
                    view! { <span></span> }.into_view()
                }}
            </div>
            {children()}
            <button class="add-field-btn" on:click=move |_| on_add.call(())>"+"</button>
        </div>
    }
}
