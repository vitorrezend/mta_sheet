use leptos::*;
use crate::compendium::archetypes::{
    find_archetype, ALL_ARCHETYPES, ARCHETYPE_THEORY_RULES, ArchetypeDefinition,
};
use crate::components::compendium::rich_text::RichTextView;
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
                                                Language::PtBr => "Comportamento é como você se apresenta aos outros. Traços de Comportamento são razoavelmente fáceis de mudar — você simplesmente começa a agir de maneira diferente e a escolher novas prioridades na vida. (p. 268)",
                                                Language::EnUs => "Demeanor is how you present yourself to other folks. Demeanor Traits are easy enough to change – you just start acting differently and choosing new priorities in life. (p. 268)",
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
                                                Language::PtBr => "Natureza é como você se enxerga quando todas as suas máscaras caem. A Natureza, por outro lado, é difícil de mudar. Exige uma circunstância transformadora de vida para causar tais transições. (p. 268)",
                                                Language::EnUs => "Nature is how you see yourself when all your masks come off. Nature, on the other hand, is difficult to change. It takes a life-shifting circumstance to cause such transitions. (p. 268)",
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
                                                Language::PtBr => "Quando você realiza algo que está em sintonia com a sua Natureza, seu Narrador pode lhe conceder de um a três pontos de Força de Vontade para repor aqueles que você gastou ao longo da história. (p. 268)",
                                                Language::EnUs => "When you accomplish something that’s in tune with your Nature, your Storyteller might give you one to three Willpower points to replace the ones you’ve spent over the course of the story. (p. 268)",
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
                                    <div class="practice-description-text compendium-rich-content">
                                        <RichTextView text=Signal::derive(move || theory.content(current_lang.get()).to_string()) />
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        // Detalhe do Arquétipo Específico
                        let arch_item = active_archetype.get();
                        let on_sel_header = on_select_act.clone();
                        let on_sel_footer = on_select_act.clone();
                        let on_cls_header = on_close_act.clone();
                        let on_cls_footer = on_close_act.clone();
                        let target_opt = active_archetype_target.and_then(|t| t.get());

                        view! {
                            <div class="practice-reading-view">
                                <div class="practice-detail-header">
                                    <div class="practice-title-row">
                                        <div class="practice-title-left">
                                            <div class="practice-main-name">
                                                "🎭 " {arch_item.name(current_lang.get())}
                                            </div>
                                            <span class="practice-page-badge">
                                                "📖 " {arch_item.page_ref}
                                            </span>
                                        </div>

                                        {if let Some(cb_h) = on_sel_header {
                                            let cb_h1 = cb_h.clone();
                                            let cb_h2 = cb_h;
                                            let on_cls_h1 = on_cls_header.clone();
                                            let on_cls_h2 = on_cls_header;
                                            view! {
                                                <div class="compendium-header-actions">
                                                    {match target_opt {
                                                        Some(ArchetypeTarget::Nature) => view! {
                                                            <button
                                                                type="button"
                                                                class="compendium-action-btn compendium-btn-purple"
                                                                on:click=move |_| {
                                                                    let chosen = arch_item.name(current_lang.get()).to_string();
                                                                    cb_h1.call((ArchetypeTarget::Nature, chosen));
                                                                    if let Some(c) = on_cls_h1.clone() { c.call(()); }
                                                                }
                                                            >
                                                                <span class="compendium-btn-icon">"✦"</span>
                                                                <span class="compendium-btn-label">
                                                                    {move || match current_lang.get() {
                                                                        Language::PtBr => "Definir Natureza",
                                                                        Language::EnUs => "Set Nature",
                                                                    }}
                                                                </span>
                                                            </button>
                                                            <button
                                                                type="button"
                                                                class="compendium-action-btn compendium-btn-subtle"
                                                                on:click=move |_| {
                                                                    let chosen = arch_item.name(current_lang.get()).to_string();
                                                                    cb_h2.call((ArchetypeTarget::Demeanor, chosen));
                                                                    if let Some(c) = on_cls_h2.clone() { c.call(()); }
                                                                }
                                                            >
                                                                <span class="compendium-btn-label">
                                                                    {move || match current_lang.get() {
                                                                        Language::PtBr => "Comportamento",
                                                                        Language::EnUs => "Demeanor",
                                                                    }}
                                                                </span>
                                                            </button>
                                                        }.into_view(),
                                                        Some(ArchetypeTarget::Demeanor) => view! {
                                                            <button
                                                                type="button"
                                                                class="compendium-action-btn compendium-btn-gold"
                                                                on:click=move |_| {
                                                                    let chosen = arch_item.name(current_lang.get()).to_string();
                                                                    cb_h1.call((ArchetypeTarget::Demeanor, chosen));
                                                                    if let Some(c) = on_cls_h1.clone() { c.call(()); }
                                                                }
                                                            >
                                                                <span class="compendium-btn-icon">"✦"</span>
                                                                <span class="compendium-btn-label">
                                                                    {move || match current_lang.get() {
                                                                        Language::PtBr => "Definir Comportamento",
                                                                        Language::EnUs => "Set Demeanor",
                                                                    }}
                                                                </span>
                                                            </button>
                                                            <button
                                                                type="button"
                                                                class="compendium-action-btn compendium-btn-subtle"
                                                                on:click=move |_| {
                                                                    let chosen = arch_item.name(current_lang.get()).to_string();
                                                                    cb_h2.call((ArchetypeTarget::Nature, chosen));
                                                                    if let Some(c) = on_cls_h2.clone() { c.call(()); }
                                                                }
                                                            >
                                                                <span class="compendium-btn-label">
                                                                    {move || match current_lang.get() {
                                                                        Language::PtBr => "Natureza",
                                                                        Language::EnUs => "Nature",
                                                                    }}
                                                                </span>
                                                            </button>
                                                        }.into_view(),
                                                        None => view! {
                                                            <button
                                                                type="button"
                                                                class="compendium-action-btn compendium-btn-purple"
                                                                on:click=move |_| {
                                                                    let chosen = arch_item.name(current_lang.get()).to_string();
                                                                    cb_h1.call((ArchetypeTarget::Nature, chosen));
                                                                    if let Some(c) = on_cls_h1.clone() { c.call(()); }
                                                                }
                                                            >
                                                                <span class="compendium-btn-icon">"✦"</span>
                                                                <span class="compendium-btn-label">
                                                                    {move || match current_lang.get() {
                                                                        Language::PtBr => "Natureza",
                                                                        Language::EnUs => "Nature",
                                                                    }}
                                                                </span>
                                                            </button>
                                                            <button
                                                                type="button"
                                                                class="compendium-action-btn compendium-btn-gold"
                                                                on:click=move |_| {
                                                                    let chosen = arch_item.name(current_lang.get()).to_string();
                                                                    cb_h2.call((ArchetypeTarget::Demeanor, chosen));
                                                                    if let Some(c) = on_cls_h2.clone() { c.call(()); }
                                                                }
                                                            >
                                                                <span class="compendium-btn-icon">"✦"</span>
                                                                <span class="compendium-btn-label">
                                                                    {move || match current_lang.get() {
                                                                        Language::PtBr => "Comportamento",
                                                                        Language::EnUs => "Demeanor",
                                                                    }}
                                                                </span>
                                                            </button>
                                                        }.into_view(),
                                                    }}
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! { <span></span> }.into_view()
                                        }}
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
                                            Language::PtBr => "NATUREZA E FORÇA DE VONTADE (M20, p. 268)",
                                            Language::EnUs => "NATURE AND WILLPOWER (M20, p. 268)",
                                        }}
                                    </h4>
                                    <div class="practice-description-text">
                                        <p class="practice-para">
                                            {match current_lang.get() {
                                                Language::PtBr => "Em termos de jogo, a Natureza também oferece a você uma oportunidade de renovar seu Traço de Força de Vontade. Quando você realiza algo que está em sintonia com a sua Natureza, seu Narrador pode lhe conceder de um a três pontos de Força de Vontade para repor aqueles que você gastou ao longo da história. Este benefício aplica-se apenas à reposição de um Traço de Força de Vontade esgotado — você não ganha novos círculos permanentes de Força de Vontade por agir de acordo com a sua Natureza. No entanto, ao comprar novos círculos de Força de Vontade com pontos de experiência, você pode ajudar a justificar essa Vontade aprimorada se tiver seguido sua Natureza com fidelidade.".to_string(),
                                                Language::EnUs => "In game terms, the Nature also provides you with an opportunity to refresh your Willpower Trait. When you accomplish something that’s in tune with your Nature, your Storyteller might give you one to three Willpower points to replace the ones you’ve spent over the course of the story. This benefit applies only to refreshing a depleted Willpower Trait – you don’t get new Willpower dots for acting according to your Nature. When you purchase new Willpower dots, however, you can help justify that boosted Will if you’ve been successfully following your Nature.".to_string(),
                                            }}
                                        </p>
                                    </div>
                                </div>

                                // Botões de Seleção (Natureza / Comportamento)
                                {if let Some(cb) = on_sel_footer {
                                    let cb1 = cb.clone();
                                    let cb2 = cb;
                                    let on_cls1 = on_cls_footer.clone();
                                    let on_cls2 = on_cls_footer;

                                    view! {
                                        <div class="compendium-footer-actions">
                                            {match target_opt {
                                                Some(ArchetypeTarget::Nature) => view! {
                                                    <button
                                                        type="button"
                                                        class="compendium-action-btn archetype-select-btn-nature compendium-btn-large"
                                                        on:click=move |_| {
                                                            let chosen = arch_item.name(current_lang.get()).to_string();
                                                            cb1.call((ArchetypeTarget::Nature, chosen));
                                                            if let Some(c) = on_cls1.clone() { c.call(()); }
                                                        }
                                                    >
                                                        <span class="compendium-btn-icon">"✦"</span>
                                                        <span class="compendium-btn-label">
                                                            {move || match current_lang.get() {
                                                                Language::PtBr => format!("Definir como Natureza ('{}')", arch_item.name(current_lang.get())),
                                                                Language::EnUs => format!("Set as Nature ('{}')", arch_item.name(current_lang.get())),
                                                            }}
                                                        </span>
                                                    </button>
                                                    <button
                                                        type="button"
                                                        class="compendium-action-btn archetype-select-btn-demeanor-alt compendium-btn-large"
                                                        on:click=move |_| {
                                                            let chosen = arch_item.name(current_lang.get()).to_string();
                                                            cb2.call((ArchetypeTarget::Demeanor, chosen));
                                                            if let Some(c) = on_cls2.clone() { c.call(()); }
                                                        }
                                                    >
                                                        <span class="compendium-btn-label">
                                                            {move || match current_lang.get() {
                                                                Language::PtBr => "Usar como Comportamento",
                                                                Language::EnUs => "Set as Demeanor",
                                                            }}
                                                        </span>
                                                    </button>
                                                }.into_view(),
                                                Some(ArchetypeTarget::Demeanor) => view! {
                                                    <button
                                                        type="button"
                                                        class="compendium-action-btn archetype-select-btn-demeanor compendium-btn-large"
                                                        on:click=move |_| {
                                                            let chosen = arch_item.name(current_lang.get()).to_string();
                                                            cb1.call((ArchetypeTarget::Demeanor, chosen));
                                                            if let Some(c) = on_cls1.clone() { c.call(()); }
                                                        }
                                                    >
                                                        <span class="compendium-btn-icon">"✦"</span>
                                                        <span class="compendium-btn-label">
                                                            {move || match current_lang.get() {
                                                                Language::PtBr => format!("Definir como Comportamento ('{}')", arch_item.name(current_lang.get())),
                                                                Language::EnUs => format!("Set as Demeanor ('{}')", arch_item.name(current_lang.get())),
                                                            }}
                                                        </span>
                                                    </button>
                                                    <button
                                                        type="button"
                                                        class="compendium-action-btn archetype-select-btn-nature-alt compendium-btn-large"
                                                        on:click=move |_| {
                                                            let chosen = arch_item.name(current_lang.get()).to_string();
                                                            cb2.call((ArchetypeTarget::Nature, chosen));
                                                            if let Some(c) = on_cls2.clone() { c.call(()); }
                                                        }
                                                    >
                                                        <span class="compendium-btn-label">
                                                            {move || match current_lang.get() {
                                                                Language::PtBr => "Usar como Natureza",
                                                                Language::EnUs => "Set as Nature",
                                                            }}
                                                        </span>
                                                    </button>
                                                }.into_view(),
                                                None => view! {
                                                    <button
                                                        type="button"
                                                        class="compendium-action-btn archetype-select-btn-nature compendium-btn-large"
                                                        on:click=move |_| {
                                                            let chosen = arch_item.name(current_lang.get()).to_string();
                                                            cb1.call((ArchetypeTarget::Nature, chosen));
                                                            if let Some(c) = on_cls1.clone() { c.call(()); }
                                                        }
                                                    >
                                                        <span class="compendium-btn-icon">"✦"</span>
                                                        <span class="compendium-btn-label">
                                                            {move || match current_lang.get() {
                                                                Language::PtBr => format!("Usar como Natureza ('{}')", arch_item.name(current_lang.get())),
                                                                Language::EnUs => format!("Use as Nature ('{}')", arch_item.name(current_lang.get())),
                                                            }}
                                                        </span>
                                                    </button>
                                                    <button
                                                        type="button"
                                                        class="compendium-action-btn archetype-select-btn-demeanor compendium-btn-large"
                                                        on:click=move |_| {
                                                            let chosen = arch_item.name(current_lang.get()).to_string();
                                                            cb2.call((ArchetypeTarget::Demeanor, chosen));
                                                            if let Some(c) = on_cls2.clone() { c.call(()); }
                                                        }
                                                    >
                                                        <span class="compendium-btn-icon">"✦"</span>
                                                        <span class="compendium-btn-label">
                                                            {move || match current_lang.get() {
                                                                Language::PtBr => format!("Usar como Comportamento ('{}')", arch_item.name(current_lang.get())),
                                                                Language::EnUs => format!("Use as Demeanor ('{}')", arch_item.name(current_lang.get())),
                                                            }}
                                                        </span>
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
