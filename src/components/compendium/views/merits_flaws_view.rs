use leptos::*;
use crate::compendium::merits_flaws::{
    find_derangement, find_merit_flaw, ALL_DERANGEMENTS, ALL_FLAWS, ALL_MERITS, ALL_MERITS_FLAWS,
    DerangementDefinition, MeritFlawDefinition, TraitCategory, TraitType,
};
use crate::components::Callback;
use crate::i18n::Language;

fn get_category_icon(cat: TraitCategory) -> &'static str {
    match cat {
        TraitCategory::Physical => "💪",
        TraitCategory::Mental => "🧠",
        TraitCategory::Social => "👥",
        TraitCategory::Supernatural => "🔮",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MainTab {
    All,
    Merits,
    Flaws,
    Derangements,
}

#[component]
pub fn MeritsFlawsView(
    selected_item_id: RwSignal<String>,
    current_lang: Signal<Language>,
    #[prop(into, default = None)] mobile_show_detail: Option<RwSignal<bool>>,
    #[prop(into, default = None)] target_slot: Option<Signal<Option<usize>>>,
    #[prop(into, default = None)] on_select_merit_flaw: Option<Callback<(Option<usize>, String, i32, bool)>>,
    #[prop(into, default = None)] on_close: Option<Callback<()>>,
) -> impl IntoView {
    let (search_filter, set_search_filter) = create_signal(String::new());
    let (active_tab, set_active_tab) = create_signal(MainTab::All);
    let (selected_category_filter, set_selected_category_filter) = create_signal(Option::<TraitCategory>::None);
    let (selected_cost, set_selected_cost) = create_signal(1);
    let (custom_spec, set_custom_spec) = create_signal(String::new());

    // Sincroniza custo padrão ao trocar de item
    create_effect(move |_| {
        let cur_id = selected_item_id.get();
        set_custom_spec.set(String::new());
        if let Some(item) = find_merit_flaw(&cur_id) {
            if let Some(&first_cost) = item.available_costs.first() {
                set_selected_cost.set(first_cost);
            }
        }
    });

    let is_derangement_active = Signal::derive(move || {
        let id = selected_item_id.get();
        find_derangement(&id).is_some()
    });

    let active_derangement = Signal::derive(move || {
        let cur_id = selected_item_id.get();
        find_derangement(&cur_id).copied()
    });

    let active_merit_flaw = Signal::derive(move || {
        let cur_id = selected_item_id.get();
        find_merit_flaw(&cur_id).copied().unwrap_or(ALL_MERITS[0])
    });

    let filtered_items = Signal::derive(move || {
        let q = search_filter.get().trim().to_lowercase();
        let tab = active_tab.get();
        let cat_filter = selected_category_filter.get();

        if tab == MainTab::Derangements {
            return Vec::new();
        }

        ALL_MERITS_FLAWS
            .iter()
            .copied()
            .filter(|mf| {
                match tab {
                    MainTab::Merits => if mf.trait_type != TraitType::Merit { return false; },
                    MainTab::Flaws => if mf.trait_type != TraitType::Flaw { return false; },
                    _ => {}
                }
                if let Some(cat) = cat_filter {
                    if mf.category != cat {
                        return false;
                    }
                }
                if q.is_empty() {
                    return true;
                }
                mf.name.to_lowercase().contains(&q)
                    || mf.name_pt.to_lowercase().contains(&q)
                    || mf.technocracy_name.map(|t| t.to_lowercase().contains(&q)).unwrap_or(false)
                    || mf.technocracy_name_pt.map(|t| t.to_lowercase().contains(&q)).unwrap_or(false)
                    || mf.description.to_lowercase().contains(&q)
                    || mf.description_pt.to_lowercase().contains(&q)
            })
            .collect::<Vec<_>>()
    });

    let filtered_derangements = Signal::derive(move || {
        let q = search_filter.get().trim().to_lowercase();
        let tab = active_tab.get();

        if tab == MainTab::Merits || tab == MainTab::Flaws {
            return Vec::new();
        }

        ALL_DERANGEMENTS
            .iter()
            .copied()
            .filter(|d| {
                if q.is_empty() {
                    return true;
                }
                d.name.to_lowercase().contains(&q)
                    || d.name_pt.to_lowercase().contains(&q)
                    || d.description.to_lowercase().contains(&q)
                    || d.description_pt.to_lowercase().contains(&q)
            })
            .collect::<Vec<_>>()
    });

    view! {
        <div
            class="compendium-section-split"
            class:mobile-show-detail=move || mobile_show_detail.map(|s| s.get()).unwrap_or(false)
        >
            // ================= COLUNA ESQUERDA: LISTA =================
            <div class="practice-sidebar-pane merit-flaw-sidebar-pane">
                <div class="compendium-sidebar-header">
                    <div class="compendium-search-box">
                        <input
                            type="text"
                            class="compendium-search-input"
                            placeholder=move || match current_lang.get() {
                                Language::PtBr => "Buscar qualidade, defeito ou perturbação...",
                                Language::EnUs => "Search merit, flaw, or derangement...",
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

                    // Abas Principais (Todas, Qualidades, Defeitos, Perturbações)
                    <div class="compendium-filter-pills-row">
                        <button
                            type="button"
                            class="compendium-filter-pill"
                            class:active=move || active_tab.get() == MainTab::All
                            on:click=move |_| set_active_tab.set(MainTab::All)
                        >
                            {move || match current_lang.get() {
                                Language::PtBr => "Todas",
                                Language::EnUs => "All",
                            }}
                        </button>
                        <button
                            type="button"
                            class="compendium-filter-pill merit-pill"
                            class:active=move || active_tab.get() == MainTab::Merits
                            on:click=move |_| {
                                set_active_tab.set(MainTab::Merits);
                                if is_derangement_active.get() {
                                    selected_item_id.set(ALL_MERITS[0].id.to_string());
                                }
                            }
                        >
                            "✨ " {move || match current_lang.get() {
                                Language::PtBr => "Qualidades",
                                Language::EnUs => "Merits",
                            }}
                        </button>
                        <button
                            type="button"
                            class="compendium-filter-pill flaw-pill"
                            class:active=move || active_tab.get() == MainTab::Flaws
                            on:click=move |_| {
                                set_active_tab.set(MainTab::Flaws);
                                if is_derangement_active.get() {
                                    selected_item_id.set(ALL_FLAWS[0].id.to_string());
                                }
                            }
                        >
                            "⚠️ " {move || match current_lang.get() {
                                Language::PtBr => "Defeitos",
                                Language::EnUs => "Flaws",
                            }}
                        </button>
                        <button
                            type="button"
                            class="compendium-filter-pill derangement-pill"
                            class:active=move || active_tab.get() == MainTab::Derangements
                            on:click=move |_| {
                                set_active_tab.set(MainTab::Derangements);
                                selected_item_id.set(ALL_DERANGEMENTS[0].id.to_string());
                            }
                        >
                            "🌀 " {move || match current_lang.get() {
                                Language::PtBr => "Perturbações",
                                Language::EnUs => "Derangements",
                            }}
                        </button>
                    </div>

                    // Sub-filtros por Categoria (quando em Todas, Qualidades ou Defeitos)
                    {move || if active_tab.get() != MainTab::Derangements {
                        view! {
                            <div class="compendium-filter-pills-row" style="margin-top: 0.35rem;">
                                <button
                                    type="button"
                                    class="compendium-filter-pill scope-pill"
                                    class:active=move || selected_category_filter.get().is_none()
                                    on:click=move |_| set_selected_category_filter.set(None)
                                >
                                    {match current_lang.get() {
                                        Language::PtBr => "Todas Categorias",
                                        Language::EnUs => "All Categories",
                                    }}
                                </button>
                                <button
                                    type="button"
                                    class="compendium-filter-pill scope-pill"
                                    class:active=move || selected_category_filter.get() == Some(TraitCategory::Physical)
                                    on:click=move |_| set_selected_category_filter.set(Some(TraitCategory::Physical))
                                >
                                    "💪 " {match current_lang.get() {
                                        Language::PtBr => "Físico",
                                        Language::EnUs => "Physical",
                                    }}
                                </button>
                                <button
                                    type="button"
                                    class="compendium-filter-pill scope-pill"
                                    class:active=move || selected_category_filter.get() == Some(TraitCategory::Mental)
                                    on:click=move |_| set_selected_category_filter.set(Some(TraitCategory::Mental))
                                >
                                    "🧠 " {match current_lang.get() {
                                        Language::PtBr => "Mental",
                                        Language::EnUs => "Mental",
                                    }}
                                </button>
                                <button
                                    type="button"
                                    class="compendium-filter-pill scope-pill"
                                    class:active=move || selected_category_filter.get() == Some(TraitCategory::Social)
                                    on:click=move |_| set_selected_category_filter.set(Some(TraitCategory::Social))
                                >
                                    "👥 " {match current_lang.get() {
                                        Language::PtBr => "Social",
                                        Language::EnUs => "Social",
                                    }}
                                </button>
                                <button
                                    type="button"
                                    class="compendium-filter-pill scope-pill"
                                    class:active=move || selected_category_filter.get() == Some(TraitCategory::Supernatural)
                                    on:click=move |_| set_selected_category_filter.set(Some(TraitCategory::Supernatural))
                                >
                                    "🔮 " {match current_lang.get() {
                                        Language::PtBr => "Sobrenatural",
                                        Language::EnUs => "Supernatural",
                                    }}
                                </button>
                            </div>
                        }.into_view()
                    } else {
                        view! { <span></span> }.into_view()
                    }}
                </div>

                // Lista de Itens
                <div class="practice-list">
                    // Seção de Qualidades e Defeitos
                    {move || {
                        let list = filtered_items.get();
                        let d_list = filtered_derangements.get();
                        let lang = current_lang.get();

                        view! {
                            <div>
                                {if !list.is_empty() {
                                    view! {
                                        <div class="practice-tab-list">
                                            {list.iter().copied().map(|mf| {
                                                let item_id = mf.id.to_string();
                                                let item_id_click = item_id.clone();
                                                let is_active = Signal::derive({
                                                    let id = item_id.clone();
                                                    move || selected_item_id.get() == id
                                                });
                                                let is_merit = mf.trait_type == TraitType::Merit;
                                                let cat_icon = get_category_icon(mf.category);
                                                view! {
                                                    <button
                                                        type="button"
                                                        class=move || if is_active.get() { "practice-tab-btn active" } else { "practice-tab-btn" }
                                                        on:click={
                                                            let id = item_id_click.clone();
                                                            move |_| {
                                                                selected_item_id.set(id.clone());
                                                                if let Some(m) = mobile_show_detail {
                                                                    m.set(true);
                                                                }
                                                            }
                                                        }
                                                    >
                                                        <span class="tab-indicator">{cat_icon}</span>
                                                        <div class="tab-text-wrap">
                                                            <div class="tab-name-row">
                                                                <span class="tab-name">{move || mf.name(lang)}</span>
                                                                <span class=format!("compendium-sub-pill trait-pill-{}", if is_merit { "merit" } else { "flaw" })>
                                                                    {mf.trait_type.name(lang)}
                                                                </span>
                                                            </div>
                                                            <div class="tab-name-row" style="margin-top: 2px;">
                                                                <span class="tab-sub">{move || mf.secondary_name(lang)}</span>
                                                                <span class="points-badge-pill">{mf.points_display(lang)}</span>
                                                            </div>
                                                        </div>
                                                    </button>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <span></span> }.into_view()
                                }}

                                // Seção de Perturbações (quando tab All ou Derangements)
                                {if !d_list.is_empty() {
                                    view! {
                                        <div style="margin-top: 0.75rem;">
                                            <div class="compendium-rules-category-header">
                                                <span class="rules-category-title">
                                                    {match lang {
                                                        Language::PtBr => "PERTURBAÇÕES MENTAIS (DERANGEMENTS)",
                                                        Language::EnUs => "DERANGEMENTS & MADNESS",
                                                    }}
                                                </span>
                                            </div>
                                            <div class="practice-tab-list">
                                                {d_list.iter().copied().map(|d| {
                                                    let d_id = d.id.to_string();
                                                    let d_id_click = d_id.clone();
                                                    let is_active = Signal::derive({
                                                        let id = d_id.clone();
                                                        move || selected_item_id.get() == id
                                                    });
                                                    view! {
                                                        <button
                                                            type="button"
                                                            class=move || if is_active.get() { "practice-tab-btn special-box-tab derangement-tab active" } else { "practice-tab-btn special-box-tab derangement-tab" }
                                                            on:click={
                                                                let id = d_id_click.clone();
                                                                move |_| {
                                                                    selected_item_id.set(id.clone());
                                                                    if let Some(m) = mobile_show_detail {
                                                                        m.set(true);
                                                                    }
                                                                }
                                                            }
                                                        >
                                                            <span class="tab-indicator">"🌀"</span>
                                                            <div class="tab-text-wrap">
                                                                <div class="tab-name-row">
                                                                    <span class="tab-name">{move || d.name(lang)}</span>
                                                                    <span class="compendium-sub-pill compendium-sub-pill-purple">{d.page_ref}</span>
                                                                </div>
                                                                <span class="tab-sub">"M20 • Perturbação Mental"</span>
                                                            </div>
                                                        </button>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <span></span> }.into_view()
                                }}

                                {if list.is_empty() && d_list.is_empty() {
                                    view! {
                                        <div class="practice-empty-state">
                                            <span class="empty-icon">"🔍"</span>
                                            <p class="empty-text">
                                                {match lang {
                                                    Language::PtBr => "Nenhum resultado encontrado para os filtros atuais.",
                                                    Language::EnUs => "No results found matching current filters.",
                                                }}
                                            </p>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <span></span> }.into_view()
                                }}
                            </div>
                        }
                    }}
                </div>
            </div>

            // ================= COLUNA DIREITA: DETALHES =================
            <div class="practice-detail-pane">
                {move || {
                    let lang = current_lang.get();
                    if is_derangement_active.get() {
                        if let Some(d) = active_derangement.get() {
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
                                            <span class="compendium-article-icon">"🌀"</span>
                                            <div>
                                                <span class="compendium-badge-sup">"M20 • PERTURBAÇÃO MENTAL"</span>
                                                <h2 class="compendium-article-title">{d.name(lang)}</h2>
                                                <span class="compendium-article-page">{d.page_ref}</span>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="compendium-section-card">
                                        <h4 class="section-card-title">
                                            "📖 " {match lang {
                                                Language::PtBr => "Descrição Clínica & Psicológica",
                                                Language::EnUs => "Clinical & Psychological Description",
                                            }}
                                        </h4>
                                        <p class="section-card-text">{d.description(lang)}</p>
                                    </div>

                                    <div class="compendium-section-card" style="margin-top: 1rem; border-left: 3px solid var(--purple-bright, #9333ea);">
                                        <h4 class="section-card-title" style="color: var(--purple-bright, #9333ea);">
                                            "⚖️ " {match lang {
                                                Language::PtBr => "Efeitos de Jogo & Crises",
                                                Language::EnUs => "Game Effects & Crises",
                                            }}
                                        </h4>
                                        <p class="section-card-text">{d.effects(lang)}</p>
                                    </div>
                                </div>
                            }.into_view()
                        } else {
                            view! { <div>"Perturbação não encontrada."</div> }.into_view()
                        }
                    } else {
                        let mf = active_merit_flaw.get();
                        let is_merit = mf.trait_type == TraitType::Merit;
                        let cat_icon = get_category_icon(mf.category);
                        let mf_name_for_click = mf.name(lang).to_string();
                        let sel_cb = on_select_merit_flaw.clone();
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
                                        <span class="practice-hero-icon">{cat_icon}</span>
                                        <div class="practice-hero-titles">
                                            <div class="hero-sup-badges">
                                                <span class=format!("compendium-badge-sup trait-badge-{}", if is_merit { "merit" } else { "flaw" })>
                                                    {mf.trait_type.name(lang).to_uppercase()}
                                                </span>
                                                <span class="compendium-scope-badge scope-badge-category">
                                                    {mf.category.name(lang)}
                                                </span>
                                                <span class="compendium-scope-badge scope-badge-cost">
                                                    {mf.points_display(lang)}
                                                </span>
                                            </div>
                                            <h2 class="practice-hero-title">{mf.name(lang)}</h2>
                                            <span class="practice-hero-sub">{mf.secondary_name(lang)}</span>
                                            {if let Some(tech_name) = mf.technocracy_name(lang) {
                                                view! {
                                                    <span class="technocracy-name-tag" style="display: inline-block; margin-top: 0.25rem; font-size: 0.8rem; font-weight: 600; color: #3b82f6;">
                                                        "🔬 " {match lang {
                                                            Language::PtBr => format!("Tecnocracia: {}", tech_name),
                                                            Language::EnUs => format!("Technocracy: {}", tech_name),
                                                        }}
                                                    </span>
                                                }.into_view()
                                            } else {
                                                view! { <span></span> }.into_view()
                                            }}
                                        </div>
                                        <div class="practice-hero-meta">
                                            <span class="practice-page-ref">{mf.page_ref}</span>
                                        </div>
                                    </div>
                                </div>

                                // Painel Interativo de Adicionar à Ficha
                                {if sel_cb.is_some() {
                                    let sel_cb_action = sel_cb.clone();
                                    let cls_cb_action = cls_cb.clone();
                                    let mf_name = mf_name_for_click.clone();
                                    let available_costs = mf.available_costs;
                                    let is_flaw = mf.trait_type == TraitType::Flaw;

                                    view! {
                                        <div class="compendium-background-picker-bar" style="margin-top: 1rem; padding: 0.9rem; border-radius: 8px; border: 1px solid var(--border-color, #e2e8f0); background: var(--surface-paper, rgba(99, 102, 241, 0.04));">
                                            <div class="picker-bar-row" style="display: flex; gap: 1rem; align-items: center; flex-wrap: wrap; justify-content: space-between;">
                                                <div class="picker-level-chooser" style="display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap;">
                                                    <span class="picker-label" style="font-weight: 700; font-size: 0.85rem; color: var(--text-primary, #334155);">
                                                        "🎯 " {match lang {
                                                            Language::PtBr => "Custo em Pontos:",
                                                            Language::EnUs => "Point Cost:",
                                                        }}
                                                    </span>
                                                    <div class="picker-dots-buttons" style="display: flex; gap: 0.25rem; align-items: center;">
                                                        {available_costs.iter().map(|&cost| {
                                                            let is_active = move || selected_cost.get() == cost;
                                                            view! {
                                                                <button
                                                                    type="button"
                                                                    class="picker-dot-btn"
                                                                    class:active=is_active
                                                                    on:click=move |_| set_selected_cost.set(cost)
                                                                    title=move || format!("{} {}", cost, if cost == 1 { "Ponto" } else { "Pontos" })
                                                                >
                                                                    <span class="dot-num">{cost}</span>
                                                                    <span class="dot-symbol">"●"</span>
                                                                </button>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                </div>

                                                <div class="picker-spec-input-wrap" style="flex: 1; min-width: 200px;">
                                                    <input
                                                        type="text"
                                                        class="picker-spec-input"
                                                        style="width: 100%; padding: 0.35rem 0.6rem; font-size: 0.85rem; border-radius: 6px; border: 1px solid var(--border-color, #cbd5e1); background: var(--surface-input, #fff);"
                                                        placeholder=match lang {
                                                            Language::PtBr => "Especificação opcional (ex: Francês, Álcool, Inimigo X)",
                                                            Language::EnUs => "Optional detail / spec (e.g. French, Alcohol, Rival X)",
                                                        }
                                                        prop:value=move || custom_spec.get()
                                                        on:input=move |ev| set_custom_spec.set(event_target_value(&ev))
                                                    />
                                                </div>
                                            </div>

                                            <div class="picker-actions-row" style="display: flex; justify-content: space-between; align-items: center; margin-top: 0.75rem; padding-top: 0.6rem; border-top: 1px dashed var(--border-color, #e2e8f0); gap: 0.8rem; flex-wrap: wrap;">
                                                <div class="picker-summary-preview" style="font-size: 0.85rem; color: var(--text-secondary, #64748b); display: flex; align-items: center; gap: 0.5rem;">
                                                    <span class="picker-preview-dots" style="font-weight: 700; color: #4f46e5; letter-spacing: 0.08em;">
                                                        {move || get_dots_visual(selected_cost.get())}
                                                    </span>
                                                    <span class="picker-preview-text" style="font-weight: 600; color: var(--text-primary, #1e293b);">
                                                        {
                                                            let mf_name = mf_name.clone();
                                                            move || {
                                                                let cost = selected_cost.get();
                                                                let spec = custom_spec.get();
                                                                let spec_str = if spec.trim().is_empty() { String::new() } else { format!(" ({})", spec.trim()) };
                                                                let trait_label = if is_flaw {
                                                                    match lang { Language::PtBr => "Defeito", Language::EnUs => "Flaw" }
                                                                } else {
                                                                    match lang { Language::PtBr => "Qualidade", Language::EnUs => "Merit" }
                                                                };
                                                                format!("{}{}: {} {} • {}", mf_name, spec_str, cost, if cost == 1 { "pt" } else { "pts" }, trait_label)
                                                            }
                                                        }
                                                    </span>
                                                </div>

                                                <button
                                                    type="button"
                                                    class="compendium-action-btn compendium-btn-emerald compendium-btn-large"
                                                    on:click={
                                                        let sel_cb = sel_cb_action.clone();
                                                        let cls_cb = cls_cb_action.clone();
                                                        let mf_name = mf_name.clone();
                                                        move |_| {
                                                            if let Some(ref cb) = sel_cb {
                                                                let spec = custom_spec.get();
                                                                let final_name = if spec.trim().is_empty() {
                                                                    mf_name.clone()
                                                                } else {
                                                                    format!("{} ({})", mf_name, spec.trim())
                                                                };
                                                                let slot = target_slot.and_then(|s| s.get());
                                                                cb.call((slot, final_name, selected_cost.get(), is_flaw));
                                                                if let Some(ref c) = cls_cb {
                                                                    c.call(());
                                                                }
                                                            }
                                                        }
                                                    }
                                                >
                                                    "➕ " {match lang {
                                                        Language::PtBr => "Vincular à Ficha com estes Pontos",
                                                        Language::EnUs => "Add to Sheet with these Points",
                                                    }}
                                                </button>
                                            </div>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <span></span> }.into_view()
                                }}

                                // Descrição do Trait
                                <div class="compendium-section-card">
                                    <h4 class="section-card-title">
                                        "📖 " {match lang {
                                            Language::PtBr => "Descrição & Narrativa",
                                            Language::EnUs => "Narrative & Description",
                                        }}
                                    </h4>
                                    <p class="section-card-text">{mf.description(lang)}</p>
                                </div>

                                // Sistema de Jogo
                                <div class="compendium-section-card" style="margin-top: 1rem; border-left: 3px solid var(--gold-primary, #caa75d);">
                                    <h4 class="section-card-title" style="color: var(--gold-primary, #b45309);">
                                        "⚖️ " {match lang {
                                            Language::PtBr => "Sistema de Regras & Mecânica",
                                            Language::EnUs => "Rules System & Mechanics",
                                        }}
                                    </h4>
                                    <div class="section-card-text" style="white-space: pre-line;">
                                        {mf.system(lang)}
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}
