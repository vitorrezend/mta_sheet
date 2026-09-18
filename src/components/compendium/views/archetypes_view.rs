use leptos::*;
use crate::compendium::archetypes::{
    find_archetype, ALL_ARCHETYPES, ARCHETYPE_THEORY_RULES, ArchetypeDefinition,
};
use crate::components::Callback;
use crate::i18n::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchetypeTarget {
    Nature,
    Demeanor,
}

#[component]
pub fn ArchetypesView(
    selected_archetype_id: RwSignal<String>,
    current_lang: Signal<Language>,
    #[prop(into, default = None)] mobile_show_detail: Option<RwSignal<bool>>,
    active_archetype_target: Option<RwSignal<Option<ArchetypeTarget>>>,
    on_select_archetype: Option<Callback<(ArchetypeTarget, String)>>,
    on_close: Option<Callback<()>>,
) -> impl IntoView {
    let archetype_search = create_rw_signal(String::new());

    let is_archetype_theory = Signal::derive(move || selected_archetype_id.get() == "theory_nature_demeanor");

    let active_archetype = Signal::derive(move || {
        let cur = selected_archetype_id.get();
        find_archetype(&cur).unwrap_or(&ALL_ARCHETYPES[0])
    });

    let on_select_act = on_select_archetype;
    let on_close_act = on_close;

    view! {
        <div 
            class="compendium-section-split"
            class:mobile-show-detail=move || mobile_show_detail.map(|s| s.get()).unwrap_or(false)
        >
            // ================= ARQUÉTIPOS: Coluna Esquerda =================
            <div class="practice-sidebar archetype-sidebar-pane">
                // Artigo Teórico: Natureza e Comportamento
                <div class="special-box-tab-wrap" style="margin-bottom: 0.6rem;">
                    <button
                        type="button"
                        class=move || {
                            if is_archetype_theory.get() {
                                "practice-tab-btn special-box-tab active"
                            } else {
                                "practice-tab-btn special-box-tab"
                            }
                        }
                        on:click=move |_| {
                            selected_archetype_id.set("theory_nature_demeanor".to_string());
                            if let Some(msd) = mobile_show_detail {
                                msd.set(true);
                            }
                        }
                    >
                        <span class="tab-indicator">"📖"</span>
                        <div class="tab-text-wrap">
                            <span class="tab-name">
                                {move || ARCHETYPE_THEORY_RULES.title(current_lang.get())}
                            </span>
                            <span class="tab-sub">{ARCHETYPE_THEORY_RULES.page_ref}</span>
                        </div>
                    </button>
                </div>

                // Campo de Busca Rápida
                <div class="compendium-search-box">
                    <input
                        type="text"
                        class="compendium-search-input"
                        placeholder=move || match current_lang.get() {
                            Language::PtBr => "🔍 Filtrar 20 arquétipos...",
                            Language::EnUs => "🔍 Filter 20 archetypes...",
                        }
                        prop:value=move || archetype_search.get()
                        on:input=move |ev| archetype_search.set(event_target_value(&ev))
                    />
                </div>

                // Título da Lista
                <div class="practice-sidebar-title" style="margin-top: 0.5rem;">
                    {move || match current_lang.get() {
                        Language::PtBr => "TODOS OS ARQUÉTIPOS (20)",
                        Language::EnUs => "ALL ARCHETYPES (20)",
                    }}
                </div>

                <div class="practice-tab-list">
                    {move || {
                        let filter = archetype_search.get().trim().to_lowercase();
                        let filtered: Vec<_> = ALL_ARCHETYPES.iter().filter(|arch| {
                            if filter.is_empty() {
                                return true;
                            }
                            arch.name.to_lowercase().contains(&filter)
                                || arch.name_pt.to_lowercase().contains(&filter)
                                || arch.aliases.iter().any(|a| a.to_lowercase().contains(&filter))
                        }).collect();

                        if filtered.is_empty() {
                            view! {
                                <div class="compendium-search-empty">
                                    {match current_lang.get() {
                                        Language::PtBr => "Nenhum arquétipo encontrado.",
                                        Language::EnUs => "No archetypes found.",
                                    }}
                                </div>
                            }.into_view()
                        } else {
                            filtered.into_iter().map(|arch_item| {
                                let item_id = arch_item.id;
                                let is_active = move || !is_archetype_theory.get() && selected_archetype_id.get() == item_id;

                                view! {
                                    <button
                                        type="button"
                                        class=move || if is_active() { "practice-tab-btn active" } else { "practice-tab-btn" }
                                        on:click=move |_| {
                                            selected_archetype_id.set(item_id.to_string());
                                            if let Some(msd) = mobile_show_detail {
                                                msd.set(true);
                                            }
                                        }
                                    >
                                        <span class="practice-tab-bullet">"🎭"</span>
                                        <span class="practice-tab-label">{move || arch_item.name(current_lang.get())}</span>
                                        <span class="practice-tab-page">{arch_item.page_ref}</span>
                                    </button>
                                }
                            }).collect_view()
                        }
                    }}
                </div>
            </div>

            // ================= ARQUÉTIPOS: Coluna Direita (Conteúdo) =================
            <div class="practice-detail-pane archetype-detail-pane">
                // Botão de Retorno no Mobile (visível apenas em telas <= 768px via CSS)
                {move || mobile_show_detail.map(|msd| {
                    view! {
                        <button
                            type="button"
                            class="compendium-mobile-back-btn"
                            on:click=move |_| msd.set(false)
                        >
                            <span class="back-arrow">"←"</span>
                            <span>{move || match current_lang.get() {
                                Language::PtBr => "Voltar para a Lista de Arquétipos",
                                Language::EnUs => "Back to Archetypes List",
                            }}</span>
                        </button>
                    }
                })}

                {move || {
                    if is_archetype_theory.get() {
                        // Artigo Teórico de Regras
                        let theory = &ARCHETYPE_THEORY_RULES;
                        view! {
                            <div class="practice-reading-view">
                                <div class="practice-detail-header special-box-header">
                                    <div class="practice-title-row">
                                        <div class="practice-main-name">
                                            "📖 " {theory.title(current_lang.get())}
                                        </div>
                                        <span class="practice-page-badge">
                                            "📖 " {theory.page_ref}
                                        </span>
                                    </div>
                                    <div class="practice-aliases">
                                        <span class="practice-aliases-label">
                                            {match current_lang.get() {
                                                Language::PtBr => "M20 Capítulo 6: ",
                                                Language::EnUs => "M20 Chapter 6: ",
                                            }}
                                        </span>
                                        <span class="practice-aliases-val">
                                            {match current_lang.get() {
                                                Language::PtBr => "Criação do Personagem (Regras Canônicas de Natureza & Comportamento)",
                                                Language::EnUs => "Creating the Character (Canonical Nature & Demeanor Rules)",
                                            }}
                                        </span>
                                    </div>
                                </div>

                                // Destaque mecânico resumido
                                <div class="archetype-mechanics-summary">
                                    <div class="archetype-mech-item">
                                        <span class="archetype-mech-title">
                                            {match current_lang.get() {
                                                Language::PtBr => "🎭 Comportamento (Demeanor):",
                                                Language::EnUs => "🎭 Demeanor:",
                                            }}
                                        </span>
                                        <span class="archetype-mech-desc">
                                            {match current_lang.get() {
                                                Language::PtBr => "A máscara exterior voltada para as pessoas. Fácil de mudar conforme suas prioridades mudam.",
                                                Language::EnUs => "The outer guise presented to other folks. Easy to change by shifting behaviors and priorities.",
                                            }}
                                        </span>
                                    </div>
                                    <div class="archetype-mech-item">
                                        <span class="archetype-mech-title">
                                            {match current_lang.get() {
                                                Language::PtBr => "💎 Natureza (Nature):",
                                                Language::EnUs => "💎 Nature:",
                                            }}
                                        </span>
                                        <span class="archetype-mech-desc">
                                            {match current_lang.get() {
                                                Language::PtBr => "Quem você é despido de todas as máscaras. Difícil de mudar; exige eventos de impacto profundo na vida.",
                                                Language::EnUs => "Who you are when all masks come off. Difficult to change; requires a life-shifting circumstance.",
                                            }}
                                        </span>
                                    </div>
                                    <div class="archetype-mech-item">
                                        <span class="archetype-mech-title">
                                            {match current_lang.get() {
                                                Language::PtBr => "⚡ Recuperação de Força de Vontade:",
                                                Language::EnUs => "⚡ Willpower Regain:",
                                            }}
                                        </span>
                                        <span class="archetype-mech-desc">
                                            {match current_lang.get() {
                                                Language::PtBr => "O Narrador concede de 1 a 3 pontos temporários gastos ao cumprir o propósito da sua Natureza.",
                                                Language::EnUs => "Storyteller awards 1 to 3 spent Willpower points when you accomplish something in tune with your Nature.",
                                            }}
                                        </span>
                                    </div>
                                </div>

                                <div class="practice-description-wrap">
                                    <h4 class="practice-section-title">
                                        {match current_lang.get() {
                                            Language::PtBr => "TEXTO INTEGRAL M20 (ORIGINAL DO LIVRO)",
                                            Language::EnUs => "COMPLETE M20 UNABRIDGED TEXT",
                                        }}
                                    </h4>
                                    <div class="practice-description-text">
                                        {
                                            let desc = theory.content(current_lang.get());
                                            desc.split("\n\n").map(|paragraph| {
                                                view! { <p class="practice-para">{paragraph.to_string()}</p> }
                                            }).collect_view()
                                        }
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        // Detalhe do Arquétipo Específico
                        let arch_item = active_archetype.get();
                        let on_sel = on_select_act.clone();
                        let on_cls = on_close_act.clone();
                        let target_opt = active_archetype_target.and_then(|t| t.get());

                        view! {
                            <div class="practice-reading-view">
                                <div class="practice-detail-header">
                                    <div class="practice-title-row">
                                        <div class="practice-main-name">
                                            "🎭 " {arch_item.name(current_lang.get())}
                                        </div>
                                        <span class="practice-page-badge">
                                            "📖 " {arch_item.page_ref}
                                        </span>
                                    </div>
                                    <div class="practice-aliases">
                                        <span class="practice-aliases-label">
                                            {match current_lang.get() {
                                                Language::PtBr => "Nome original em inglês: ",
                                                Language::EnUs => "Original English name: ",
                                            }}
                                        </span>
                                        <span class="practice-aliases-val">
                                            {arch_item.name}
                                        </span>
                                    </div>
                                </div>

                                // Introdução / Visão Geral do Arquétipo
                                <div class="archetype-intro-box">
                                    <p class="archetype-intro-text">{arch_item.description(current_lang.get())}</p>
                                </div>

                                // Cards de Destaque: Força, Fraqueza e Recuperação de Força de Vontade em Grid Otimizado
                                <div class="archetype-triad-grid">
                                    // Card de Força
                                    <div class="archetype-triad-card strength-card">
                                        <div class="triad-card-header">
                                            <span class="triad-icon">"🛡️"</span>
                                            <span class="triad-title">
                                                {match current_lang.get() {
                                                    Language::PtBr => format!("FORÇA: {}", arch_item.strength_name(current_lang.get()).to_uppercase()),
                                                    Language::EnUs => format!("STRENGTH: {}", arch_item.strength_name(current_lang.get()).to_uppercase()),
                                                }}
                                            </span>
                                        </div>
                                        <div class="triad-card-body">
                                            {arch_item.strength(current_lang.get())}
                                        </div>
                                    </div>

                                    // Card de Fraqueza
                                    <div class="archetype-triad-card weakness-card">
                                        <div class="triad-card-header">
                                            <span class="triad-icon">"⚠️"</span>
                                            <span class="triad-title">
                                                {match current_lang.get() {
                                                    Language::PtBr => format!("FRAQUEZA: {}", arch_item.weakness_name(current_lang.get()).to_uppercase()),
                                                    Language::EnUs => format!("WEAKNESS: {}", arch_item.weakness_name(current_lang.get()).to_uppercase()),
                                                }}
                                            </span>
                                        </div>
                                        <div class="triad-card-body">
                                            {arch_item.weakness(current_lang.get())}
                                        </div>
                                    </div>

                                    // Card de Recuperação de Força de Vontade (Ocupa as 2 Colunas)
                                    <div class="archetype-triad-card willpower-card">
                                        <div class="triad-card-header">
                                            <span class="triad-icon">"⚡"</span>
                                            <span class="triad-title">
                                                {match current_lang.get() {
                                                    Language::PtBr => "RECUPERAÇÃO DE FORÇA DE VONTADE (1 a 3 pts)",
                                                    Language::EnUs => "WILLPOWER REGAIN (1 to 3 pts)",
                                                }}
                                            </span>
                                        </div>
                                        <div class="triad-card-body">
                                            {arch_item.willpower_regain(current_lang.get())}
                                        </div>
                                    </div>
                                </div>

                                // Diretrizes Canônicas de Interpretação (M20)
                                <div class="practice-description-wrap">
                                    <h4 class="practice-section-title">
                                        {match current_lang.get() {
                                            Language::PtBr => "DINÂMICA DE INTERPRETAÇÃO: NATUREZA VS COMPORTAMENTO",
                                            Language::EnUs => "ROLEPLAYING DYNAMICS: NATURE VS DEMEANOR",
                                        }}
                                    </h4>
                                    <div class="practice-description-text">
                                        <p class="practice-para">
                                            {match current_lang.get() {
                                                Language::PtBr => format!("Se este arquétipo for a sua Natureza, sua verdadeira essência é regida por '{}'. Você recupera de 1 a 3 pontos de Força de Vontade toda vez que seus atos cumprirem o critério de '{}'. Se for adotado apenas como Comportamento, representa como você se projeta socialmente perante outros magos e adormecidos.", arch_item.name_pt, arch_item.willpower_regain_pt),
                                                Language::EnUs => format!("If chosen as your Nature, your true self is driven by '{}'. You regain 1 to 3 points of Willpower whenever your actions satisfy '{}'. If used only as a Demeanor, this represents the mask and facade you wear when interacting with society and the Sleepers.", arch_item.name, arch_item.willpower_regain),
                                            }}
                                        </p>
                                    </div>
                                </div>

                                // Botões de Seleção (Natureza / Comportamento)
                                {if let Some(cb) = on_sel {
                                    let cb1 = cb.clone();
                                    let cb2 = cb;
                                    let on_cls1 = on_cls.clone();
                                    let on_cls2 = on_cls;

                                    view! {
                                        <div class="practice-action-row" style="margin-top: 1.5rem; display: flex; gap: 0.8rem; flex-wrap: wrap;">
                                            {match target_opt {
                                                Some(ArchetypeTarget::Nature) => view! {
                                                    <button
                                                        type="button"
                                                        class="practice-select-btn archetype-select-btn-nature"
                                                        on:click=move |_| {
                                                            let chosen = arch_item.name(current_lang.get()).to_string();
                                                            cb1.call((ArchetypeTarget::Nature, chosen));
                                                            if let Some(c) = on_cls1.clone() { c.call(()); }
                                                        }
                                                    >
                                                        {move || match current_lang.get() {
                                                            Language::PtBr => format!("✦ Definir como Natureza ('{}')", arch_item.name(current_lang.get())),
                                                            Language::EnUs => format!("✦ Set as Nature ('{}')", arch_item.name(current_lang.get())),
                                                        }}
                                                    </button>
                                                    <button
                                                        type="button"
                                                        class="practice-select-btn archetype-select-btn-demeanor-alt"
                                                        on:click=move |_| {
                                                            let chosen = arch_item.name(current_lang.get()).to_string();
                                                            cb2.call((ArchetypeTarget::Demeanor, chosen));
                                                            if let Some(c) = on_cls2.clone() { c.call(()); }
                                                        }
                                                    >
                                                        {move || match current_lang.get() {
                                                            Language::PtBr => "Usar como Comportamento",
                                                            Language::EnUs => "Set as Demeanor",
                                                        }}
                                                    </button>
                                                }.into_view(),
                                                Some(ArchetypeTarget::Demeanor) => view! {
                                                    <button
                                                        type="button"
                                                        class="practice-select-btn archetype-select-btn-demeanor"
                                                        on:click=move |_| {
                                                            let chosen = arch_item.name(current_lang.get()).to_string();
                                                            cb1.call((ArchetypeTarget::Demeanor, chosen));
                                                            if let Some(c) = on_cls1.clone() { c.call(()); }
                                                        }
                                                    >
                                                        {move || match current_lang.get() {
                                                            Language::PtBr => format!("✦ Definir como Comportamento ('{}')", arch_item.name(current_lang.get())),
                                                            Language::EnUs => format!("✦ Set as Demeanor ('{}')", arch_item.name(current_lang.get())),
                                                        }}
                                                    </button>
                                                    <button
                                                        type="button"
                                                        class="practice-select-btn archetype-select-btn-nature-alt"
                                                        on:click=move |_| {
                                                            let chosen = arch_item.name(current_lang.get()).to_string();
                                                            cb2.call((ArchetypeTarget::Nature, chosen));
                                                            if let Some(c) = on_cls2.clone() { c.call(()); }
                                                        }
                                                    >
                                                        {move || match current_lang.get() {
                                                            Language::PtBr => "Usar como Natureza",
                                                            Language::EnUs => "Set as Nature",
                                                        }}
                                                    </button>
                                                }.into_view(),
                                                None => view! {
                                                    <button
                                                        type="button"
                                                        class="practice-select-btn archetype-select-btn-nature"
                                                        on:click=move |_| {
                                                            let chosen = arch_item.name(current_lang.get()).to_string();
                                                            cb1.call((ArchetypeTarget::Nature, chosen));
                                                            if let Some(c) = on_cls1.clone() { c.call(()); }
                                                        }
                                                    >
                                                        {move || match current_lang.get() {
                                                            Language::PtBr => format!("✦ Usar como Natureza ('{}')", arch_item.name(current_lang.get())),
                                                            Language::EnUs => format!("✦ Use as Nature ('{}')", arch_item.name(current_lang.get())),
                                                        }}
                                                    </button>
                                                    <button
                                                        type="button"
                                                        class="practice-select-btn archetype-select-btn-demeanor"
                                                        on:click=move |_| {
                                                            let chosen = arch_item.name(current_lang.get()).to_string();
                                                            cb2.call((ArchetypeTarget::Demeanor, chosen));
                                                            if let Some(c) = on_cls2.clone() { c.call(()); }
                                                        }
                                                    >
                                                        {move || match current_lang.get() {
                                                            Language::PtBr => format!("✦ Usar como Comportamento ('{}')", arch_item.name(current_lang.get())),
                                                            Language::EnUs => format!("✦ Use as Demeanor ('{}')", arch_item.name(current_lang.get())),
                                                        }}
                                                    </button>
                                                }.into_view(),
                                            }}
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
