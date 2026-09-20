use leptos::*;
use crate::compendium::backgrounds::{
    find_unabridged_text, ALL_BACKGROUNDS, BACKGROUND_THEORY_RULES, BackgroundDefinition,
};
use crate::components::compendium::rich_text::RichTextView;
use crate::components::Callback;
use crate::i18n::Language;

fn get_bg_icon(id: &str) -> &'static str {
    match id {
        "allies" => "🤝",
        "alternate_identity" => "🎭",
        "arcane" => "🌫️",
        "avatar" => "🌟",
        "backup" => "🚔",
        "blessing" => "🍀",
        "certification" => "📜",
        "chantry" => "🏛️",
        "contacts" => "📱",
        "cult" => "🕯️",
        "demesne" => "🏰",
        "destiny" => "🌠",
        "dream" => "💭",
        "enhancement" => "🦾",
        "fame" => "📸",
        "familiar" => "🐈",
        "influence" => "🏛️",
        "legend" => "👑",
        "library" => "📚",
        "mentor" => "🧙",
        "node" => "🌀",
        "past_lives" => "⏳",
        "patron" => "🕵️",
        "rank" => "🎖️",
        "requisitions" => "📦",
        "resources" => "💰",
        "retainers" => "🤵",
        "sanctum" => "🔮",
        "secret_weapons" => "🔬",
        "spies" => "🕵️‍♂️",
        "status" => "🏅",
        "totem" => "🐺",
        "wonder" => "✨",
        _ => "✦",
    }
}

fn get_dots_visual(dots: i32) -> &'static str {
    match dots {
        0 => "X",
        1 => "● ○ ○ ○ ○",
        2 => "● ● ○ ○ ○",
        3 => "● ● ● ○ ○",
        4 => "● ● ● ● ○",
        5 => "● ● ● ● ●",
        6 => "●●●●● ●",
        7 => "●●●●● ●●",
        8 => "●●●●● ●●●",
        9 => "●●●●● ●●●●",
        10 => "●●●●● ●●●●●",
        _ => "●",
    }
}

#[component]
pub fn BackgroundsView(
    selected_background_id: RwSignal<String>,
    current_lang: Signal<Language>,
    #[prop(into, default = None)] mobile_show_detail: Option<RwSignal<bool>>,
    #[prop(into, default = None)] target_slot: Option<Signal<Option<usize>>>,
    #[prop(into, default = None)] on_select_background: Option<Callback<(Option<usize>, String, i32)>>,
    #[prop(into, default = None)] on_close: Option<Callback<()>>,
) -> impl IntoView {
    let (search_filter, set_search_filter) = create_signal(String::new());
    let (selected_level, set_selected_level) = create_signal(1);
    let (custom_spec, set_custom_spec) = create_signal(String::new());
    let (show_unabridged, set_show_unabridged) = create_signal(true);

    // Reset de nível selecionado e especificação ao trocar de antecedente
    create_effect(move |_| {
        let _ = selected_background_id.get();
        set_selected_level.set(1);
        set_custom_spec.set(String::new());
    });

    let is_rules_selected = Signal::derive(move || selected_background_id.get() == "theory_background_rules");

    let filtered_backgrounds = Signal::derive(move || {
        let q = search_filter.get().trim().to_lowercase();
        if q.is_empty() {
            return ALL_BACKGROUNDS.to_vec();
        }
        ALL_BACKGROUNDS
            .iter()
            .copied()
            .filter(|bg| {
                bg.name.to_lowercase().contains(&q)
                    || bg.name_pt.to_lowercase().contains(&q)
                    || bg.technocracy_name.map(|t| t.to_lowercase().contains(&q)).unwrap_or(false)
                    || bg.technocracy_name_pt.map(|t| t.to_lowercase().contains(&q)).unwrap_or(false)
            })
            .collect::<Vec<_>>()
    });

    let active_background = Signal::derive(move || {
        let cur_id = selected_background_id.get();
        ALL_BACKGROUNDS
            .iter()
            .find(|b| b.id == cur_id)
            .copied()
            .unwrap_or(ALL_BACKGROUNDS[0])
    });

    view! {
        <div
            class="compendium-section-split"
            class:mobile-show-detail=move || mobile_show_detail.map(|s| s.get()).unwrap_or(false)
        >
            // ================= COLUNA ESQUERDA: LISTA DE ANTECEDENTES =================
            <div class="practice-sidebar-pane">
                // Campo de Busca
                <div class="practice-search-box" style="margin-bottom: 0.6rem;">
                    <input
                        type="text"
                        class="practice-search-input"
                        placeholder=move || match current_lang.get() {
                            Language::PtBr => "Buscar Antecedente (ex: Aliados, Arcano, Avatar...)",
                            Language::EnUs => "Search Background (e.g. Allies, Arcane, Avatar...)",
                        }
                        prop:value=search_filter
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
                                "×"
                            </button>
                        }.into_view()
                    } else {
                        view! { <span></span> }.into_view()
                    }}
                </div>

                // Botão especial: Regras de Antecedentes (M20, pp. 301-303)
                <div class="special-box-tab-wrap" style="margin-bottom: 0.6rem;">
                    <button
                        type="button"
                        class=move || {
                            if is_rules_selected.get() {
                                "practice-tab-btn special-box-tab active"
                            } else {
                                "practice-tab-btn special-box-tab"
                            }
                        }
                        on:click=move |_| {
                            selected_background_id.set("theory_background_rules".to_string());
                            if let Some(msd) = mobile_show_detail {
                                msd.set(true);
                            }
                        }
                    >
                        <span class="tab-indicator">"📜"</span>
                        <div class="tab-text-wrap">
                            <span class="tab-name">
                                {move || match current_lang.get() {
                                    Language::PtBr => "Regras: Acima de 5 & Cabala",
                                    Language::EnUs => "Rules: Over 5 & Pooling",
                                }}
                            </span>
                            <span class="tab-sub">"M20, pp. 301-303"</span>
                        </div>
                    </button>
                </div>

                <div class="practice-sidebar-title">
                    {move || match current_lang.get() {
                        Language::PtBr => format!("ANTECEDENTES CANÔNICOS ({})", filtered_backgrounds.get().len()),
                        Language::EnUs => format!("CANONICAL BACKGROUNDS ({})", filtered_backgrounds.get().len()),
                    }}
                </div>

                <div class="practice-tab-list">
                    {move || {
                        let list = filtered_backgrounds.get();
                        let cur_id = selected_background_id.get();
                        let is_rule = is_rules_selected.get();
                        let lang = current_lang.get();

                        list.into_iter().map(|bg: BackgroundDefinition| {
                            let b_id = bg.id;
                            let is_active = !is_rule && cur_id == b_id;
                            let icon = get_bg_icon(b_id);
                            let tec_badge = bg.technocracy_name(lang);

                            view! {
                                <button
                                    type="button"
                                    class=if is_active { "practice-tab-btn active" } else { "practice-tab-btn" }
                                    on:click=move |_| {
                                        selected_background_id.set(b_id.to_string());
                                        if let Some(msd) = mobile_show_detail {
                                            msd.set(true);
                                        }
                                    }
                                >
                                    <span class="tab-indicator">{icon}</span>
                                    <div class="tab-text-wrap">
                                        <div class="tab-name-row" style="display: flex; align-items: center; justify-content: space-between; gap: 0.3rem;">
                                            <span class="tab-name">{bg.name(lang)}</span>
                                            {if let Some(tec) = tec_badge {
                                                view! {
                                                    <span class="compendium-sub-pill" style="font-size: 0.65rem; padding: 0.1rem 0.35rem; border-radius: 4px; background: rgba(59, 130, 246, 0.12); color: #2563eb;">
                                                        {tec}
                                                    </span>
                                                }.into_view()
                                            } else {
                                                view! { <span></span> }.into_view()
                                            }}
                                        </div>
                                        <span class="tab-sub">{format!("{} • {}", bg.secondary_name(lang), bg.page_ref)}</span>
                                    </div>
                                </button>
                            }
                        }).collect_view()
                    }}
                </div>
            </div>

            // ================= COLUNA DIREITA: DETALHES DO ANTECEDENTE =================
            <div class="practice-detail-pane">
                // Botão de Retorno no Mobile
                {move || mobile_show_detail.map(|msd| {
                    view! {
                        <button
                            type="button"
                            class="compendium-mobile-back-btn"
                            on:click=move |_| msd.set(false)
                        >
                            <span class="back-arrow">"←"</span>
                            <span>{match current_lang.get() {
                                Language::PtBr => "Voltar para Lista de Antecedentes",
                                Language::EnUs => "Back to Backgrounds List",
                            }}</span>
                        </button>
                    }
                })}

                {move || {
                    let lang = current_lang.get();

                    if is_rules_selected.get() {
                        let rule = &BACKGROUND_THEORY_RULES;
                        view! {
                            <div class="box-reading-view">
                                <div class="practice-detail-header special-box-header">
                                    <div class="practice-title-group">
                                        <div class="practice-main-name">
                                            "📜 " {rule.title(lang)}
                                        </div>
                                        <span class="practice-page-badge">
                                            "📖 " {rule.page_ref}
                                        </span>
                                    </div>
                                    <div class="practice-aliases">
                                        <span class="practice-aliases-label">
                                            {match lang {
                                                Language::PtBr => "Fonte Canônica: ",
                                                Language::EnUs => "Canonical Source: ",
                                            }}
                                        </span>
                                        <span class="practice-aliases-val">
                                            {match lang {
                                                Language::PtBr => "M20, pp. 301-303 • Capítulo 6: Criação do Personagem (Regras de Antecedentes)",
                                                Language::EnUs => "M20, pp. 301-303 • Chapter 6: Creating the Character (Background Rules)",
                                            }}
                                        </span>
                                    </div>
                                </div>

                                <div class="archetype-mechanics-summary" style="margin-top: 1rem;">
                                    <div class="archetype-mech-item">
                                        <span class="archetype-mech-title">
                                            "★ " {match lang {
                                                Language::PtBr => "Pontuações Acima de 5 Bolinhas:",
                                                Language::EnUs => "Traits Over Five Dots:",
                                            }}
                                        </span>
                                        <span class="archetype-mech-desc">
                                            {match lang {
                                                Language::PtBr => "Mortais e iniciantes limitam-se a 5. Em mesas de alto poder mítico ou ativos comunitários da cabala, Antecedentes como Aliados, Reforço, Capela, Contatos e Culto podem ascender até 10 pontos.",
                                                Language::EnUs => "Mortals and starting characters cap at 5. In mythic elder games or pooled cabal assets, Traits like Allies, Backup, Chantry, Contacts, and Cult may scale up to 10 dots.",
                                            }}
                                        </span>
                                    </div>
                                    <div class="archetype-mech-item" style="margin-top: 0.5rem;">
                                        <span class="archetype-mech-title">
                                            "★ " {match lang {
                                                Language::PtBr => "Agrupamento de Recursos na Cabala:",
                                                Language::EnUs => "Pooling Cabal Resources:",
                                            }}
                                        </span>
                                        <span class="archetype-mech-desc">
                                            {match lang {
                                                Language::PtBr => "Membros de uma mesma Cabala ou Amálgama podem somar seus pontos para fundar Capelas, Nós de Quintessência compartilhados ou redes conjuntas de contatos e cultistas.",
                                                Language::EnUs => "Members of a Cabal or Amalgam may pool background points to build shared Chantries, communal Nodes, or joint information webs.",
                                            }}
                                        </span>
                                    </div>
                                </div>

                                <div class="practice-description-block" style="margin-top: 1.2rem;">
                                    {
                                        let content = rule.content(lang);
                                        content.split("\n\n").map(|paragraph| {
                                            view! {
                                                <p class="practice-desc-para">{paragraph.to_string()}</p>
                                            }
                                        }).collect_view()
                                    }
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        let bg = active_background.get();
                        let bg_icon = get_bg_icon(bg.id);
                        let max_dots_label = if bg.max_dots > 5 { "1-10 Pontos" } else { "1-5 Pontos" };
                        let bg_name_for_header = bg.name(lang).to_string();
                        let bg_name_for_footer = bg.name(lang).to_string();
                        let sel_cb = on_select_background.clone();
                        let sel_cb_header = on_select_background.clone();
                        let sel_cb_footer = on_select_background.clone();
                        let cls_cb = on_close.clone();
                        let cls_cb_header = on_close.clone();
                        let cls_cb_footer = on_close.clone();

                        view! {
                            <div class="practice-reading-view">
                                <div class="practice-detail-header">
                                    <div class="practice-title-row">
                                        <div class="practice-title-left">
                                            <div class="practice-main-name">
                                                {bg_icon} " " {bg.name(lang)}
                                                <span class="practice-title-secondary">
                                                    " (" {bg.secondary_name(lang)} ")"
                                                </span>
                                            </div>
                                            <div class="header-badges-row" style="display: flex; gap: 0.5rem; align-items: center; flex-wrap: wrap;">
                                                {if let Some(tec) = bg.technocracy_name(lang) {
                                                    view! {
                                                        <span class="attribute-cat-badge badge-mental" title="Designação na União Tecnocrática">
                                                            "🔬 " {tec}
                                                        </span>
                                                    }.into_view()
                                                } else {
                                                    view! { <span></span> }.into_view()
                                                }}
                                                <span class="attribute-cat-badge badge-social">
                                                    {max_dots_label}
                                                </span>
                                                <span class="practice-page-badge">
                                                    "📖 " {bg.page_ref}
                                                </span>
                                            </div>
                                        </div>

                                        // Botão Canônico de Ação no Topo Direito (Desktop)
                                        {if let Some(ref cb) = sel_cb_header {
                                            let name_cloned = bg_name_for_header.clone();
                                            let cb_cloned = cb.clone();
                                            let cls_cloned = cls_cb_header.clone();
                                            view! {
                                                <div class="compendium-header-actions">
                                                    <button
                                                        type="button"
                                                        class="compendium-action-btn compendium-btn-emerald"
                                                        on:click=move |_| {
                                                            let spec = custom_spec.get();
                                                            let final_name = if spec.trim().is_empty() {
                                                                name_cloned.clone()
                                                            } else {
                                                                format!("{} ({})", name_cloned, spec.trim())
                                                            };
                                                            let slot = target_slot.and_then(|s| s.get());
                                                            cb_cloned.call((slot, final_name, selected_level.get()));
                                                            if let Some(ref c) = cls_cloned {
                                                                c.call(());
                                                            }
                                                        }
                                                    >
                                                        "➕ " {match lang {
                                                            Language::PtBr => "Vincular à Ficha",
                                                            Language::EnUs => "Add to Sheet",
                                                        }}
                                                    </button>
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! { <span></span> }.into_view()
                                        }}
                                    </div>
                                </div>

                                // Painel Interativo de Seleção de Nível e Especificação (quando aberto para vincular à ficha)
                                {if sel_cb.is_some() {
                                    let bg_name_for_preview = bg.name(lang).to_string();
                                    let bg_name_for_click = bg.name(lang).to_string();
                                    let sel_cb_action = sel_cb.clone();
                                    let cls_cb_action = cls_cb.clone();
                                    let max_dots = bg.max_dots;

                                    view! {
                                        <div class="compendium-background-picker-bar" style="margin-top: 1rem; padding: 0.9rem; border-radius: 8px; border: 1px solid var(--border-color, #e2e8f0); background: var(--surface-paper, rgba(99, 102, 241, 0.04));">
                                            <div class="picker-bar-row" style="display: flex; gap: 1rem; align-items: center; flex-wrap: wrap; justify-content: space-between;">
                                                <div class="picker-level-chooser" style="display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap;">
                                                    <span class="picker-label" style="font-weight: 700; font-size: 0.85rem; color: var(--text-primary, #334155);">
                                                        "🎯 " {match lang {
                                                            Language::PtBr => "Nível a Adicionar:",
                                                            Language::EnUs => "Rating to Add:",
                                                        }}
                                                    </span>
                                                    <div class="picker-dots-buttons" style="display: flex; gap: 0.25rem; align-items: center;">
                                                        {(1..=max_dots).map(|lvl| {
                                                            let is_active = move || selected_level.get() == lvl;
                                                            let is_epic = lvl > 5;
                                                            view! {
                                                                <button
                                                                    type="button"
                                                                    class="picker-dot-btn"
                                                                    class:active=is_active
                                                                    class:epic=is_epic
                                                                    on:click=move |_| set_selected_level.set(lvl)
                                                                    title=move || format!("{} {}", lvl, if lvl == 1 { "Ponto" } else { "Pontos" })
                                                                >
                                                                    <span class="dot-num">{lvl}</span>
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
                                                            Language::PtBr => "Especificação opcional (ex: Detetive Miller, Santuário)",
                                                            Language::EnUs => "Optional detail / spec (e.g. Detective Miller)",
                                                        }
                                                        prop:value=move || custom_spec.get()
                                                        on:input=move |ev| set_custom_spec.set(event_target_value(&ev))
                                                    />
                                                </div>
                                            </div>

                                            <div class="picker-actions-row" style="display: flex; justify-content: space-between; align-items: center; margin-top: 0.75rem; padding-top: 0.6rem; border-top: 1px dashed var(--border-color, #e2e8f0); gap: 0.8rem; flex-wrap: wrap;">
                                                <div class="picker-summary-preview" style="font-size: 0.85rem; color: var(--text-secondary, #64748b); display: flex; align-items: center; gap: 0.5rem;">
                                                    <span class="picker-preview-dots" style="font-weight: 700; color: #4f46e5; letter-spacing: 0.08em;">
                                                        {move || get_dots_visual(selected_level.get())}
                                                    </span>
                                                    <span class="picker-preview-text" style="font-weight: 600; color: var(--text-primary, #1e293b);">
                                                        {
                                                            let bg_name = bg_name_for_preview.clone();
                                                            move || {
                                                                let lvl = selected_level.get();
                                                                let spec = custom_spec.get();
                                                                let spec_str = if spec.trim().is_empty() { String::new() } else { format!(" ({})", spec.trim()) };
                                                                format!("{}{}: {} {}", bg_name, spec_str, lvl, if lvl == 1 { "ponto" } else { "pontos" })
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
                                                        let bg_name = bg_name_for_click.clone();
                                                        move |_| {
                                                            if let Some(ref cb) = sel_cb {
                                                                let spec = custom_spec.get();
                                                                let final_name = if spec.trim().is_empty() {
                                                                    bg_name.clone()
                                                                } else {
                                                                    format!("{} ({})", bg_name, spec.trim())
                                                                };
                                                                let slot = target_slot.and_then(|s| s.get());
                                                                cb.call((slot, final_name, selected_level.get()));
                                                                if let Some(ref c) = cls_cb {
                                                                    c.call(());
                                                                }
                                                            }
                                                        }
                                                    }
                                                >
                                                    "➕ " {match lang {
                                                        Language::PtBr => "Vincular à Ficha com este Nível",
                                                        Language::EnUs => "Add to Sheet with this Rating",
                                                    }}
                                                </button>
                                            </div>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <span></span> }.into_view()
                                }}

                                // Alternância entre Modo Texto Integral M20 e Resumo Rápido
                                <div class="compendium-view-mode-toggle">
                                    <button
                                        type="button"
                                        class=move || if show_unabridged.get() { "view-mode-btn active" } else { "view-mode-btn" }
                                        on:click=move |_| set_show_unabridged.set(true)
                                    >
                                        "📖 " {match lang {
                                            Language::PtBr => "Texto Integral M20",
                                            Language::EnUs => "Unabridged M20",
                                        }}
                                    </button>
                                    <button
                                        type="button"
                                        class=move || if !show_unabridged.get() { "view-mode-btn active" } else { "view-mode-btn" }
                                        on:click=move |_| set_show_unabridged.set(false)
                                    >
                                        "⚡ " {match lang {
                                            Language::PtBr => "Resumo Rápido",
                                            Language::EnUs => "Quick Summary",
                                        }}
                                    </button>
                                </div>

                                {move || {
                                    if show_unabridged.get() {
                                        if let Some(unabridged_text) = find_unabridged_text(bg.id, lang) {
                                            let text_sig = Signal::derive(move || unabridged_text.to_string());
                                            return view! {
                                                <div class="practice-description-block" style="margin-top: 0.8rem;">
                                                    <RichTextView text=text_sig />
                                                </div>
                                            }.into_view();
                                        }
                                    }

                                    view! {
                                        <div>
                                            // Bloco de Descrição Narrativa
                                            <div class="practice-description-block" style="margin-top: 0.8rem;">
                                                <p class="practice-desc-para">{bg.description(lang)}</p>
                                            </div>

                                            // Bloco em Destaque do Sistema de Regras
                                            <div class="archetype-mechanics-summary" style="margin-top: 1.2rem;">
                                                <div class="archetype-mech-item">
                                                    <span class="archetype-mech-title">
                                                        "🎲 " {match lang {
                                                            Language::PtBr => "Mecânica & Sistema de Jogo:",
                                                            Language::EnUs => "Mechanics & Game System:",
                                                        }}
                                                    </span>
                                                    <span class="archetype-mech-desc" style="display: block; margin-top: 0.35rem; line-height: 1.5;">
                                                        {bg.system(lang)}
                                                    </span>
                                                </div>
                                            </div>
                                        </div>
                                    }.into_view()
                                }}

                                // Grade de Níveis de Pontuação (1 a 5 ou 1 a 10)
                                <div class="attribute-ratings-section" style="margin-top: 1.5rem;">
                                    <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 0.5rem; margin-bottom: 0.4rem;">
                                        <h4 class="compendium-section-title" style="margin-bottom: 0;">
                                            "📊 " {match lang {
                                                Language::PtBr => format!("Escala de Pontuação & Diferenças ({})", max_dots_label),
                                                Language::EnUs => format!("Rating Scale & Differences ({})", max_dots_label),
                                            }}
                                        </h4>
                                        {if sel_cb.is_some() {
                                            view! {
                                                <span style="font-size: 0.78rem; color: #6366f1; font-weight: 600;">
                                                    "💡 " {match lang {
                                                        Language::PtBr => "Clique em qualquer nível para selecioná-lo",
                                                        Language::EnUs => "Click any level to choose it",
                                                    }}
                                                </span>
                                            }.into_view()
                                        } else {
                                            view! { <span></span> }.into_view()
                                        }}
                                    </div>
                                    <div class="attribute-ratings-list">
                                        {bg.ratings.iter().map(|rating| {
                                            let dots = rating.dots;
                                            let dots_visual = get_dots_visual(dots);
                                            let is_epic = dots > 5;
                                            let is_selectable = dots > 0 && sel_cb.is_some();
                                            let is_selected = Signal::derive(move || selected_level.get() == dots);

                                            view! {
                                                <div
                                                    class="attribute-rating-card"
                                                    class:rating-unlocked=is_epic
                                                    class:rating-selectable=is_selectable
                                                    class:rating-selected=move || is_selectable && is_selected.get()
                                                    on:click=move |_| {
                                                        if dots > 0 {
                                                            set_selected_level.set(dots);
                                                        }
                                                    }
                                                >
                                                    <div class="rating-card-header">
                                                        <span class="rating-dots-indicator" style="font-weight: 700; letter-spacing: 0.1em;">
                                                            {dots_visual}
                                                        </span>
                                                        <span class="rating-label-title" style="font-size: 0.8rem; color: #64748b;">
                                                            {format!("{} {}", dots, if dots == 1 { "Ponto" } else { "Pontos" })}
                                                        </span>
                                                        {if is_epic {
                                                            view! {
                                                                <span class="rating-unlocked-tag">
                                                                    "★ " {match lang {
                                                                        Language::PtBr => "Escala Acima de 5",
                                                                        Language::EnUs => "Traits Over 5",
                                                                    }}
                                                                </span>
                                                            }.into_view()
                                                        } else {
                                                            view! { <span></span> }.into_view()
                                                        }}

                                                        {if is_selectable {
                                                            view! {
                                                                <div class="rating-card-actions" style="margin-left: auto;">
                                                                    {move || if is_selected.get() {
                                                                        view! {
                                                                            <span class="rating-chosen-badge" style="font-size: 0.72rem; padding: 0.15rem 0.45rem; border-radius: 4px; background: #4f46e5; color: white; font-weight: 700;">
                                                                                "✔ " {match lang {
                                                                                    Language::PtBr => "Nível Escolhido",
                                                                                    Language::EnUs => "Selected Level",
                                                                                }}
                                                                            </span>
                                                                        }.into_view()
                                                                    } else {
                                                                        view! {
                                                                            <button
                                                                                type="button"
                                                                                class="rating-pick-btn"
                                                                                style="font-size: 0.72rem; padding: 0.15rem 0.45rem; border-radius: 4px; background: rgba(99, 102, 241, 0.1); color: #4f46e5; border: 1px solid rgba(99, 102, 241, 0.25); cursor: pointer;"
                                                                                on:click=move |ev| {
                                                                                    ev.stop_propagation();
                                                                                    set_selected_level.set(dots);
                                                                                }
                                                                            >
                                                                                {match lang {
                                                                                    Language::PtBr => "Escolher Nível",
                                                                                    Language::EnUs => "Choose Level",
                                                                                }}
                                                                            </button>
                                                                        }.into_view()
                                                                    }}
                                                                </div>
                                                            }.into_view()
                                                        } else {
                                                            view! { <span></span> }.into_view()
                                                        }}
                                                    </div>
                                                    <div class="rating-desc-text">
                                                        {rating.description(lang)}
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>

                                // Tabela Especial: Amostras de Equipes de Reforço (Backup)
                                {if let Some(teams) = bg.backup_teams {
                                    view! {
                                        <div class="compendium-extra-table-section" style="margin-top: 1.5rem;">
                                            <h4 class="compendium-section-title">
                                                "👥 " {match lang {
                                                    Language::PtBr => "Exemplos de Equipes de Reforço por Arquétipo (M20, pp. 305-306)",
                                                    Language::EnUs => "Sample Backup Teams by Archetype (M20, pp. 305-306)",
                                                }}
                                            </h4>
                                            <div class="compendium-table-wrapper" style="overflow-x: auto; margin-top: 0.5rem; border: 1px solid #e2e8f0; border-radius: 8px;">
                                                <table class="compendium-table" style="width: 100%; border-collapse: collapse; font-size: 0.85rem;">
                                                    <thead>
                                                        <tr style="background: #f8fafc; border-bottom: 2px solid #e2e8f0; text-align: left;">
                                                            <th style="padding: 0.6rem 0.8rem; font-weight: 700; color: #334155; width: 30%;">
                                                                {match lang {
                                                                    Language::PtBr => "Arquétipo do Mago",
                                                                    Language::EnUs => "Mage Archetype",
                                                                }}
                                                            </th>
                                                            <th style="padding: 0.6rem 0.8rem; font-weight: 700; color: #334155;">
                                                                {match lang {
                                                                    Language::PtBr => "Tipos de Agentes de Apoio Requisitados",
                                                                    Language::EnUs => "Typical Support Agents Dispatched",
                                                                }}
                                                            </th>
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        {teams.iter().map(|team| {
                                                            view! {
                                                                <tr style="border-bottom: 1px solid #f1f5f9;">
                                                                    <td style="padding: 0.55rem 0.8rem; font-weight: 600; color: #1e293b; vertical-align: top;">
                                                                        {team.archetype(lang)}
                                                                    </td>
                                                                    <td style="padding: 0.55rem 0.8rem; color: #475569; line-height: 1.4;">
                                                                        {team.members(lang)}
                                                                    </td>
                                                                </tr>
                                                            }
                                                        }).collect_view()}
                                                    </tbody>
                                                </table>
                                            </div>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <span></span> }.into_view()
                                }}

                                // Tabela Especial: Piscinas de Construção de Capelas (Chantry Pools)
                                {if let Some(pools) = bg.chantry_pools {
                                    view! {
                                        <div class="compendium-extra-table-section" style="margin-top: 1.5rem;">
                                            <h4 class="compendium-section-title">
                                                "🏛️ " {match lang {
                                                    Language::PtBr => "Níveis de Instalação & Piscinas de Construção (M20, p. 308)",
                                                    Language::EnUs => "Facility Levels & Construction Point Pools (M20, p. 308)",
                                                }}
                                            </h4>
                                            <div class="compendium-table-wrapper" style="overflow-x: auto; margin-top: 0.5rem; border: 1px solid #e2e8f0; border-radius: 8px;">
                                                <table class="compendium-table" style="width: 100%; border-collapse: collapse; font-size: 0.85rem;">
                                                    <thead>
                                                        <tr style="background: #f8fafc; border-bottom: 2px solid #e2e8f0; text-align: left;">
                                                            <th style="padding: 0.6rem 0.8rem; font-weight: 700; color: #334155; width: 22%;">
                                                                {match lang {
                                                                    Language::PtBr => "Piscina de Pontos",
                                                                    Language::EnUs => "Point Pool",
                                                                }}
                                                            </th>
                                                            <th style="padding: 0.6rem 0.8rem; font-weight: 700; color: #334155; width: 30%;">
                                                                {match lang {
                                                                    Language::PtBr => "Categoria da Sede",
                                                                    Language::EnUs => "Stronghold Type",
                                                                }}
                                                            </th>
                                                            <th style="padding: 0.6rem 0.8rem; font-weight: 700; color: #334155;">
                                                                {match lang {
                                                                    Language::PtBr => "Recursos & Amenidades Paranormais",
                                                                    Language::EnUs => "Resources & Paranormal Amenities",
                                                                }}
                                                            </th>
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        {pools.iter().map(|pl| {
                                                            view! {
                                                                <tr style="border-bottom: 1px solid #f1f5f9;">
                                                                    <td style="padding: 0.55rem 0.8rem; font-weight: 700; color: #6366f1; vertical-align: top;">
                                                                        {pl.pool_range}
                                                                    </td>
                                                                    <td style="padding: 0.55rem 0.8rem; font-weight: 600; color: #1e293b; vertical-align: top;">
                                                                        {pl.name(lang)}
                                                                    </td>
                                                                    <td style="padding: 0.55rem 0.8rem; color: #475569; line-height: 1.4;">
                                                                        {pl.description(lang)}
                                                                    </td>
                                                                </tr>
                                                            }
                                                        }).collect_view()}
                                                    </tbody>
                                                </table>
                                            </div>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <span></span> }.into_view()
                                }}

                                // Tabela Especial: Dificuldade de Requisições por Lealdade (Requisitions Chart - M20 p. 316)
                                {if let Some(req_chart) = bg.requisitions_chart {
                                    view! {
                                        <div class="compendium-extra-table-section" style="margin-top: 1.5rem;">
                                            <h4 class="compendium-section-title">
                                                "📦 " {match lang {
                                                    Language::PtBr => "Tabela de Requisições por Grau de Lealdade (M20, p. 316)",
                                                    Language::EnUs => "Requisitions Chart by Loyalty Standing (M20, p. 316)",
                                                }}
                                            </h4>
                                            <div class="compendium-table-wrapper" style="overflow-x: auto; margin-top: 0.5rem; border: 1px solid #e2e8f0; border-radius: 8px;">
                                                <table class="compendium-table" style="width: 100%; border-collapse: collapse; font-size: 0.85rem;">
                                                    <thead>
                                                        <tr style="background: #f8fafc; border-bottom: 2px solid #e2e8f0; text-align: left;">
                                                            <th style="padding: 0.6rem 0.8rem; font-weight: 700; color: #334155; width: 60%;">
                                                                {match lang {
                                                                    Language::PtBr => "Relação / Grau de Lealdade com os Superiores",
                                                                    Language::EnUs => "Relationship / Loyalty Standing with Superiors",
                                                                }}
                                                            </th>
                                                            <th style="padding: 0.6rem 0.8rem; font-weight: 700; color: #334155;">
                                                                {match lang {
                                                                    Language::PtBr => "Dificuldade do Teste",
                                                                    Language::EnUs => "Difficulty",
                                                                }}
                                                            </th>
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        {req_chart.iter().map(|entry| {
                                                            view! {
                                                                <tr style="border-bottom: 1px solid #f1f5f9;">
                                                                    <td style="padding: 0.55rem 0.8rem; font-weight: 600; color: #1e293b; vertical-align: top;">
                                                                        {entry.relationship(lang)}
                                                                    </td>
                                                                    <td style="padding: 0.55rem 0.8rem; font-weight: 700; color: #6366f1;">
                                                                        {format!("Dif. {}", entry.difficulty)}
                                                                    </td>
                                                                </tr>
                                                            }
                                                        }).collect_view()}
                                                    </tbody>
                                                </table>
                                            </div>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <span></span> }.into_view()
                                }}

                                // Botão Canônico de Ação no Rodapé (Sticky no Mobile)
                                {if let Some(ref cb) = sel_cb_footer {
                                    let name_cloned = bg_name_for_footer.clone();
                                    let cb_cloned = cb.clone();
                                    let cls_cloned = cls_cb_footer.clone();
                                    view! {
                                        <div class="compendium-footer-actions">
                                            <button
                                                type="button"
                                                class="compendium-action-btn compendium-btn-emerald compendium-btn-large"
                                                on:click=move |_| {
                                                    let spec = custom_spec.get();
                                                    let final_name = if spec.trim().is_empty() {
                                                        name_cloned.clone()
                                                    } else {
                                                        format!("{} ({})", name_cloned, spec.trim())
                                                    };
                                                    let slot = target_slot.and_then(|s| s.get());
                                                    cb_cloned.call((slot, final_name, selected_level.get()));
                                                    if let Some(ref c) = cls_cloned {
                                                        c.call(());
                                                    }
                                                }
                                            >
                                                "➕ " {match lang {
                                                    Language::PtBr => "Vincular à Ficha",
                                                    Language::EnUs => "Add to Sheet",
                                                }}
                                            </button>
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
