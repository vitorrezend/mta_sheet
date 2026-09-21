use leptos::*;
use crate::compendium::abilities::{
    find_ability, find_ability_theory_rule, ALL_ABILITIES, ABILITY_THEORY_RULES, AbilityCategory,
    AbilityDefinition, AbilityScope,
};
use crate::components::compendium::rich_text::RichTextView;
use crate::components::Callback;
use crate::i18n::Language;

fn get_ability_icon(cat: AbilityCategory) -> &'static str {
    match cat {
        AbilityCategory::Talents => "⚡",
        AbilityCategory::Skills => "🔧",
        AbilityCategory::Knowledges => "📚",
    }
}

fn get_dots_visual(dots: i32) -> &'static str {
    match dots {
        1 => "● ○ ○ ○ ○",
        2 => "● ● ○ ○ ○",
        3 => "● ● ● ○ ○",
        4 => "● ● ● ● ○",
        5 => "● ● ● ● ●",
        _ => "●",
    }
}

#[component]
pub fn AbilitiesView(
    selected_ability_id: RwSignal<String>,
    current_lang: Signal<Language>,
    #[prop(into, default = None)] mobile_show_detail: Option<RwSignal<bool>>,
    #[prop(into, default = None)] target_slot: Option<Signal<Option<usize>>>,
    #[prop(into, default = None)] on_select_ability: Option<Callback<(Option<usize>, String)>>,
    #[prop(into, default = None)] on_close: Option<Callback<()>>,
) -> impl IntoView {
    let (search_filter, set_search_filter) = create_signal(String::new());
    let (selected_category_filter, set_selected_category_filter) = create_signal(Option::<AbilityCategory>::None);
    let (selected_scope_filter, set_selected_scope_filter) = create_signal(Option::<AbilityScope>::None);
    let (custom_spec, set_custom_spec) = create_signal(String::new());

    create_effect(move |_| {
        let _ = selected_ability_id.get();
        set_custom_spec.set(String::new());
    });

    let is_rules_selected = Signal::derive(move || {
        selected_ability_id.get().starts_with("rule_")
    });

    let filtered_abilities = Signal::derive(move || {
        let q = search_filter.get().trim().to_lowercase();
        let cat_filter = selected_category_filter.get();
        let scope_filter = selected_scope_filter.get();

        ALL_ABILITIES
            .iter()
            .copied()
            .filter(|ab| {
                if let Some(cat) = cat_filter {
                    if ab.category != cat {
                        return false;
                    }
                }
                if let Some(scope) = scope_filter {
                    if ab.scope != scope {
                        return false;
                    }
                }
                if q.is_empty() {
                    return true;
                }
                ab.name.to_lowercase().contains(&q)
                    || ab.name_pt.to_lowercase().contains(&q)
                    || ab.description.to_lowercase().contains(&q)
                    || ab.description_pt.to_lowercase().contains(&q)
            })
            .collect::<Vec<_>>()
    });

    let active_ability = Signal::derive(move || {
        let cur_id = selected_ability_id.get();
        find_ability(&cur_id).copied().unwrap_or(ALL_ABILITIES[0])
    });

    let active_rule = Signal::derive(move || {
        let cur_id = selected_ability_id.get();
        find_ability_theory_rule(&cur_id).copied()
    });

    view! {
        <div
            class="compendium-section-split"
            class:mobile-show-detail=move || mobile_show_detail.map(|s| s.get()).unwrap_or(false)
        >
            // ================= COLUNA ESQUERDA: LISTA DE HABILIDADES =================
            <div class="practice-sidebar-pane ability-sidebar-pane">
                <div class="compendium-sidebar-header">
                    <div class="compendium-search-box">
                        <input
                            type="text"
                            class="compendium-search-input"
                            placeholder=move || match current_lang.get() {
                                Language::PtBr => "Buscar habilidade ou regra...",
                                Language::EnUs => "Search ability or rule...",
                            }
                            prop:value=move || search_filter.get()
                            on:input=move |ev| set_search_filter.set(event_target_value(&ev))
                        />
                        {move || if !search_filter.get().is_empty() {
                            view! {
                                <button
                                    type="button"
                                    class="search-clear-btn"
                                    on:click=move |_| set_search_filter.set(String::new())
                                    title="Limpar busca"
                                >
                                    "✕"
                                </button>
                            }.into_view()
                        } else {
                            view! { <span></span> }.into_view()
                        }}
                    </div>

                    // Filtros por Categoria
                    <div class="compendium-filter-pills-row">
                        <button
                            type="button"
                            class="compendium-filter-pill"
                            class:active=move || selected_category_filter.get().is_none() && !is_rules_selected.get()
                            on:click=move |_| {
                                set_selected_category_filter.set(None);
                                if is_rules_selected.get() {
                                    selected_ability_id.set(ALL_ABILITIES[0].id.to_string());
                                }
                            }
                        >
                            {move || match current_lang.get() {
                                Language::PtBr => "Todas",
                                Language::EnUs => "All",
                            }}
                        </button>
                        <button
                            type="button"
                            class="compendium-filter-pill"
                            class:active=move || selected_category_filter.get() == Some(AbilityCategory::Talents)
                            on:click=move |_| {
                                set_selected_category_filter.set(Some(AbilityCategory::Talents));
                                if is_rules_selected.get() {
                                    selected_ability_id.set("alertness".to_string());
                                }
                            }
                        >
                            "⚡ " {move || match current_lang.get() {
                                Language::PtBr => "Talentos",
                                Language::EnUs => "Talents",
                            }}
                        </button>
                        <button
                            type="button"
                            class="compendium-filter-pill"
                            class:active=move || selected_category_filter.get() == Some(AbilityCategory::Skills)
                            on:click=move |_| {
                                set_selected_category_filter.set(Some(AbilityCategory::Skills));
                                if is_rules_selected.get() {
                                    selected_ability_id.set("crafts".to_string());
                                }
                            }
                        >
                            "🔧 " {move || match current_lang.get() {
                                Language::PtBr => "Perícias",
                                Language::EnUs => "Skills",
                            }}
                        </button>
                        <button
                            type="button"
                            class="compendium-filter-pill"
                            class:active=move || selected_category_filter.get() == Some(AbilityCategory::Knowledges)
                            on:click=move |_| {
                                set_selected_category_filter.set(Some(AbilityCategory::Knowledges));
                                if is_rules_selected.get() {
                                    selected_ability_id.set("academics".to_string());
                                }
                            }
                        >
                            "📚 " {move || match current_lang.get() {
                                Language::PtBr => "Conhecimentos",
                                Language::EnUs => "Knowledges",
                            }}
                        </button>
                    </div>

                    // Sub-filtro por Escopo (Básica vs Secundária) e Botão de Regras
                    <div class="compendium-filter-pills-row" style="margin-top: 0.35rem;">
                        <button
                            type="button"
                            class="compendium-filter-pill scope-pill"
                            class:active=move || selected_scope_filter.get().is_none() && !is_rules_selected.get()
                            on:click=move |_| set_selected_scope_filter.set(None)
                        >
                            {move || match current_lang.get() {
                                Language::PtBr => "Básicas & Secundárias",
                                Language::EnUs => "Core & Secondary",
                            }}
                        </button>
                        <button
                            type="button"
                            class="compendium-filter-pill scope-pill"
                            class:active=move || selected_scope_filter.get() == Some(AbilityScope::Core)
                            on:click=move |_| set_selected_scope_filter.set(Some(AbilityScope::Core))
                        >
                            {move || match current_lang.get() {
                                Language::PtBr => "Básicas",
                                Language::EnUs => "Core Only",
                            }}
                        </button>
                        <button
                            type="button"
                            class="compendium-filter-pill scope-pill"
                            class:active=move || selected_scope_filter.get() == Some(AbilityScope::Secondary)
                            on:click=move |_| set_selected_scope_filter.set(Some(AbilityScope::Secondary))
                        >
                            {move || match current_lang.get() {
                                Language::PtBr => "Secundárias",
                                Language::EnUs => "Secondary",
                            }}
                        </button>
                        <button
                            type="button"
                            class="compendium-filter-pill rules-pill"
                            class:active=move || is_rules_selected.get()
                            on:click=move |_| {
                                selected_ability_id.set("rule_theory_body_control".to_string());
                                if let Some(m) = mobile_show_detail {
                                    m.set(true);
                                }
                            }
                        >
                            "⚖️ " {move || match current_lang.get() {
                                Language::PtBr => "Regras",
                                Language::EnUs => "Rules",
                            }}
                        </button>
                    </div>
                </div>

                // Lista de Itens
                <div class="practice-tab-list">
                    // Card especial para Artigos de Regras Teóricas
                    <div class="compendium-rules-category-header">
                        <span class="rules-category-title">
                            {move || match current_lang.get() {
                                Language::PtBr => "REGRAS E TEORIA DE HABILIDADES",
                                Language::EnUs => "ABILITY RULES & THEORY",
                            }}
                        </span>
                    </div>

                    {ABILITY_THEORY_RULES.iter().map(|rule| {
                        let r_id = rule.id.to_string();
                        let r_id_for_click = r_id.clone();
                        let is_active = Signal::derive({
                            let r_id = r_id.clone();
                            move || selected_ability_id.get() == r_id
                        });
                        view! {
                            <button
                                type="button"
                                class=move || if is_active.get() { "practice-tab-btn special-box-tab active" } else { "practice-tab-btn special-box-tab" }
                                on:click={
                                    let r_id = r_id_for_click.clone();
                                    move |_| {
                                        selected_ability_id.set(r_id.clone());
                                        if let Some(m) = mobile_show_detail {
                                            m.set(true);
                                        }
                                    }
                                }
                            >
                                <span class="tab-indicator">"⚖️"</span>
                                <div class="tab-text-wrap">
                                    <div class="tab-name-row">
                                        <span class="tab-name">{move || rule.title(current_lang.get())}</span>
                                        <span class="compendium-sub-pill compendium-sub-pill-gold">{rule.page_ref}</span>
                                    </div>
                                    <span class="tab-sub">"M20 • Regra Opcional"</span>
                                </div>
                            </button>
                        }
                    }).collect_view()}

                    <div class="compendium-rules-category-header" style="margin-top: 0.75rem;">
                        <span class="rules-category-title">
                            {move || match current_lang.get() {
                                Language::PtBr => "CATÁLOGO DE HABILIDADES",
                                Language::EnUs => "ABILITIES CATALOG",
                            }}
                        </span>
                    </div>

                    {move || {
                        let list = filtered_abilities.get();
                        if list.is_empty() {
                            view! {
                                <div class="practice-empty-state">
                                    <span class="empty-icon">"🔍"</span>
                                    <p class="empty-text">
                                        {match current_lang.get() {
                                             Language::PtBr => "Nenhuma habilidade encontrada para o filtro atual.",
                                             Language::EnUs => "No abilities found matching the current filter.",
                                        }}
                                    </p>
                                </div>
                            }.into_view()
                        } else {
                            list.into_iter().map(|ab| {
                                let a_id = ab.id.to_string();
                                let a_id_for_click = a_id.clone();
                                let is_active = Signal::derive({
                                    let a_id = a_id.clone();
                                    move || selected_ability_id.get() == a_id
                                });
                                let icon = get_ability_icon(ab.category);
                                let scope_tag = match ab.scope {
                                    AbilityScope::Core => "CORE",
                                    AbilityScope::Secondary => "SEC",
                                };
                                view! {
                                    <button
                                        type="button"
                                        class=move || if is_active.get() { "practice-tab-btn active" } else { "practice-tab-btn" }
                                        on:click={
                                            let a_id = a_id_for_click.clone();
                                            move |_| {
                                                selected_ability_id.set(a_id.clone());
                                                if let Some(m) = mobile_show_detail {
                                                    m.set(true);
                                                }
                                            }
                                        }
                                    >
                                        <span class="tab-indicator">{icon}</span>
                                        <div class="tab-text-wrap">
                                            <div class="tab-name-row">
                                                <span class="tab-name">{move || ab.name(current_lang.get())}</span>
                                                <span class=format!("compendium-sub-pill scope-badge-{}", scope_tag.to_lowercase())>
                                                    {scope_tag}
                                                </span>
                                            </div>
                                            <span class="tab-sub">{move || ab.secondary_name(current_lang.get())}</span>
                                        </div>
                                    </button>
                                }
                            }).collect_view().into_view()
                        }
                    }}
                </div>
            </div>

            // ================= COLUNA DIREITA: DETALHES =================
            <div class="practice-detail-pane">
                {move || {
                    let lang = current_lang.get();
                    if is_rules_selected.get() {
                        if let Some(rule) = active_rule.get() {
                            view! {
                                <div class="compendium-article-view">
                                    <div class="compendium-mobile-back-bar">
                                        <button
                                            type="button"
                                            class="compendium-mobile-back-btn"
                                            on:click=move |_| {
                                                if let Some(m) = mobile_show_detail {
                                                    m.set(false);
                                                }
                                            }
                                        >
                                            "← " {match lang {
                                                Language::PtBr => "Voltar à lista",
                                                Language::EnUs => "Back to list",
                                            }}
                                        </button>
                                    </div>

                                    <div class="compendium-article-header">
                                        <div class="compendium-article-title-wrap">
                                            <span class="compendium-article-icon">"⚖️"</span>
                                            <div>
                                                <span class="compendium-badge-sup">"M20 • REGRA TEÓRICA"</span>
                                                <h2 class="compendium-article-title">{rule.title(lang)}</h2>
                                                <span class="compendium-article-page">{rule.page_ref}</span>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="compendium-rich-content">
                                        <RichTextView text=Signal::derive(move || rule.content(lang).to_string()) />
                                    </div>
                                </div>
                            }.into_view()
                        } else {
                            view! { <div>"Regra não encontrada."</div> }.into_view()
                        }
                    } else {
                        let ab = active_ability.get();
                        let ab_icon = get_ability_icon(ab.category);
                        let ab_name_for_click = ab.name(lang).to_string();
                        let sel_cb = on_select_ability.clone();
                        let cls_cb = on_close.clone();

                        view! {
                            <div class="practice-detail-content">
                                <div class="compendium-mobile-back-bar">
                                    <button
                                        type="button"
                                        class="compendium-mobile-back-btn"
                                        on:click=move |_| {
                                            if let Some(m) = mobile_show_detail {
                                                m.set(false);
                                            }
                                        }
                                    >
                                        "← " {match lang {
                                            Language::PtBr => "Voltar à lista",
                                            Language::EnUs => "Back to list",
                                        }}
                                    </button>
                                </div>

                                <div class="practice-hero-header">
                                    <div class="practice-hero-title-row">
                                        <span class="practice-hero-icon">{ab_icon}</span>
                                        <div class="practice-hero-titles">
                                            <div class="hero-sup-badges">
                                                <span class="compendium-badge-sup">
                                                    {ab.category.name(lang).to_uppercase()}
                                                </span>
                                                <span class=format!("compendium-scope-badge scope-badge-{}", match ab.scope {
                                                    AbilityScope::Core => "core",
                                                    AbilityScope::Secondary => "secondary",
                                                })>
                                                    {ab.scope.name(lang)}
                                                </span>
                                            </div>
                                            <h2 class="practice-hero-title">{ab.name(lang)}</h2>
                                            <span class="practice-hero-sub">{ab.secondary_name(lang)}</span>
                                        </div>
                                        <div class="practice-hero-meta">
                                            <span class="practice-page-ref">{ab.page_ref}</span>
                                        </div>
                                    </div>
                                </div>

                                // Painel Interativo de Adicionar à Ficha
                                {if sel_cb.is_some() {
                                    let sel_cb_action = sel_cb.clone();
                                    let cls_cb_action = cls_cb.clone();
                                    let ab_name = ab_name_for_click.clone();
                                    view! {
                                        <div class="compendium-background-picker-bar">
                                            <div class="picker-bar-row">
                                                <div class="picker-spec-input-wrap" style="flex: 1; min-width: 240px;">
                                                    <input
                                                        type="text"
                                                        class="picker-spec-input"
                                                        style="width: 100%; padding: 0.45rem 0.75rem; font-size: 0.9rem; border-radius: 6px; border: 1px solid var(--border-color, #cbd5e1); background: var(--surface-input, #fff);"
                                                        placeholder=match lang {
                                                            Language::PtBr => "Especialização opcional (ex: Karate, Algoritmos, Forense)",
                                                            Language::EnUs => "Optional specialization (e.g. Karate, Algorithms, Forensics)",
                                                        }
                                                        prop:value=move || custom_spec.get()
                                                        on:input=move |ev| set_custom_spec.set(event_target_value(&ev))
                                                    />
                                                </div>
                                                <button
                                                    type="button"
                                                    class="compendium-action-btn compendium-btn-emerald compendium-btn-large"
                                                    on:click={
                                                        let sel_cb = sel_cb_action.clone();
                                                        let cls_cb = cls_cb_action.clone();
                                                        let ab_name = ab_name.clone();
                                                        move |_| {
                                                            if let Some(ref cb) = sel_cb {
                                                                let spec = custom_spec.get();
                                                                let final_name = if spec.trim().is_empty() {
                                                                    ab_name.clone()
                                                                } else {
                                                                    format!("{} ({})", ab_name, spec.trim())
                                                                };
                                                                let slot = target_slot.and_then(|s| s.get());
                                                                cb.call((slot, final_name));
                                                                if let Some(ref c) = cls_cb {
                                                                    c.call(());
                                                                }
                                                            }
                                                        }
                                                    }
                                                >
                                                    "➕ " {match lang {
                                                        Language::PtBr => "Adicionar à Ficha",
                                                        Language::EnUs => "Add to Sheet",
                                                    }}
                                                </button>
                                            </div>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <span></span> }.into_view()
                                }}

                                // Descrição da Habilidade
                                <div class="compendium-section-card">
                                    <h4 class="section-card-title">
                                        "📖 " {match lang {
                                            Language::PtBr => "Descrição & Aplicações",
                                            Language::EnUs => "Description & Applications",
                                        }}
                                    </h4>
                                    <p class="section-card-text">{ab.description(lang)}</p>
                                </div>

                                // Linhas Informativas: Possuído por & Especialidades Sugeridas
                                <div class="ability-meta-grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 1rem; margin-top: 1rem;">
                                    {
                                        let possessed = ab.possessed_by(lang);
                                        if !possessed.trim().is_empty() {
                                            view! {
                                                <div class="compendium-section-card ability-meta-card">
                                                    <h4 class="section-card-title" style="font-size: 0.85rem; color: var(--gold-primary, #b45309);">
                                                        "👥 " {match lang {
                                                            Language::PtBr => "Possuído Por:",
                                                            Language::EnUs => "Possessed By:",
                                                        }}
                                                    </h4>
                                                    <p class="section-card-text" style="font-size: 0.85rem;">{possessed}</p>
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! { <span></span> }.into_view()
                                        }
                                    }

                                    {
                                        let specs = ab.specialties(lang);
                                        if !specs.is_empty() {
                                            let specialties = specs.join(", ");
                                            view! {
                                                <div class="compendium-section-card ability-meta-card">
                                                    <h4 class="section-card-title" style="font-size: 0.85rem; color: var(--purple-bright, #9333ea);">
                                                        "🎯 " {match lang {
                                                            Language::PtBr => "Especializações Sugeridas:",
                                                            Language::EnUs => "Suggested Specialties:",
                                                        }}
                                                    </h4>
                                                    <p class="section-card-text" style="font-size: 0.85rem;">{specialties}</p>
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! { <span></span> }.into_view()
                                        }
                                    }
                                </div>

                                // Tabela de Níveis (Ratings 1 a 5 dots)
                                {if !ab.ratings.is_empty() {
                                    view! {
                                        <div class="compendium-section-card" style="margin-top: 1.25rem;">
                                            <h4 class="section-card-title">
                                                "📊 " {match lang {
                                                    Language::PtBr => "Graduações & Níveis de Domínio (1 a 5 Pontos)",
                                                    Language::EnUs => "Ratings & Mastery Levels (1 to 5 Dots)",
                                                }}
                                            </h4>
                                            <div class="ability-ratings-list" style="display: flex; flex-direction: column; gap: 0.6rem; margin-top: 0.75rem;">
                                                {ab.ratings.iter().map(|r| {
                                                    let dots_vis = get_dots_visual(r.dots);
                                                    view! {
                                                        <div class="ability-rating-row" style="display: flex; gap: 0.75rem; align-items: baseline; padding: 0.5rem 0.75rem; background: var(--surface-card, #f8fafc); border-radius: 6px; border-left: 3px solid var(--gold-primary, #caa75d);">
                                                            <span class="rating-dots" style="font-family: monospace; font-size: 0.85rem; color: var(--purple-bright, #9333ea); min-width: 95px;">
                                                                {dots_vis}
                                                            </span>
                                                            <strong class="rating-title" style="font-size: 0.85rem; min-width: 90px; color: var(--text-primary, #1e293b);">
                                                                {r.title(lang)}:
                                                            </strong>
                                                            <span class="rating-desc" style="font-size: 0.85rem; color: var(--text-secondary, #475569); flex: 1;">
                                                                {r.description(lang)}
                                                            </span>
                                                        </div>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <span></span> }.into_view()
                                }}
                            </div>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}
