use leptos::*;
use crate::compendium::instruments::{
    find_instrument, find_theory_article, ALL_INSTRUMENTS, ALL_THEORY_ARTICLES,
    InstrumentDefinition, InstrumentTheoryArticle,
};
use crate::compendium::practices::find_practice;
use crate::components::Callback;
use crate::i18n::Language;

#[component]
pub fn InstrumentsView(
    selected_instrument_id: RwSignal<String>,
    history_practice_id: RwSignal<Option<String>>,
    current_lang: Signal<Language>,
    #[prop(into, default = None)] mobile_show_detail: Option<RwSignal<bool>>,
    on_select_instrument: Option<Callback<String>>,
    on_back_to_practice: Option<Callback<String>>,
    on_close: Option<Callback<()>>,
) -> impl IntoView {
    let instrument_search = create_rw_signal(String::new());

    let is_instrument_theory = Signal::derive(move || {
        let cur = selected_instrument_id.get();
        ALL_THEORY_ARTICLES.iter().any(|a| a.id == cur)
    });

    let active_theory_article = Signal::derive(move || {
        let cur = selected_instrument_id.get();
        find_theory_article(&cur).unwrap_or(&ALL_THEORY_ARTICLES[0])
    });

    let active_instrument = Signal::derive(move || {
        let cur = selected_instrument_id.get();
        find_instrument(&cur).unwrap_or(&ALL_INSTRUMENTS[0])
    });

    let on_select_act = on_select_instrument;
    let on_back_act = on_back_to_practice;
    let on_close_act = on_close;

    view! {
        <div 
            class="compendium-section-split"
            class:mobile-show-detail=move || mobile_show_detail.map(|s| s.get()).unwrap_or(false)
        >
            // ================= INSTRUMENTOS: Coluna Esquerda =================
            <div class="practice-sidebar-pane instrument-sidebar">
                // Campo de Busca Rápida
                <div class="compendium-search-box">
                    <input
                        type="text"
                        class="compendium-search-input"
                        placeholder=move || match current_lang.get() {
                            Language::PtBr => "🔍 Filtrar 54 instrumentos...",
                            Language::EnUs => "🔍 Filter 54 instruments...",
                        }
                        prop:value=move || instrument_search.get()
                        on:input=move |ev| instrument_search.set(event_target_value(&ev))
                    />
                </div>

                // Artigos Teóricos e Regras Canônicas
                <div class="practice-sidebar-title">
                    {move || match current_lang.get() {
                        Language::PtBr => "TEORIA & REGRAS M20 (pp. 586-588)",
                        Language::EnUs => "THEORY & RULES M20 (pp. 586-588)",
                    }}
                </div>
                <div class="practice-tab-list theory-articles-list">
                    {ALL_THEORY_ARTICLES.iter().map(|art: &'static InstrumentTheoryArticle| {
                        let art_id = art.id;
                        let art_page = art.page_ref;
                        let is_active = Signal::derive(move || selected_instrument_id.get() == art_id);
                        view! {
                            <button
                                type="button"
                                class=move || {
                                    if is_active.get() {
                                        "practice-tab-btn special-box-tab active"
                                    } else {
                                        "practice-tab-btn special-box-tab"
                                    }
                                }
                                on:click=move |_| {
                                    selected_instrument_id.set(art_id.to_string());
                                    if let Some(msd) = mobile_show_detail {
                                        msd.set(true);
                                    }
                                }
                            >
                                <span class="tab-indicator">"⚖️"</span>
                                <div class="tab-text-wrap">
                                    <span class="tab-name">{move || art.title(current_lang.get())}</span>
                                    <span class="tab-sub">{art_page}</span>
                                </div>
                            </button>
                        }
                    }).collect_view()}
                </div>

                // Lista de 54 Instrumentos Comuns
                <div class="practice-sidebar-title" style="margin-top: 0.6rem;">
                    {move || match current_lang.get() {
                        Language::PtBr => "TODOS OS 54 INSTRUMENTOS (M20, pp. 588-608)",
                        Language::EnUs => "ALL 54 INSTRUMENTS (M20, pp. 588-608)",
                    }}
                </div>
                <div class="practice-tab-list">
                    {move || {
                        let q = instrument_search.get().to_lowercase();
                        ALL_INSTRUMENTS.iter().filter(move |i| {
                            if q.is_empty() {
                                return true;
                            }
                            i.name.to_lowercase().contains(&q)
                                || i.name_pt.to_lowercase().contains(&q)
                                || i.aliases.iter().any(|a| a.to_lowercase().contains(&q))
                        }).map(|inst: &'static InstrumentDefinition| {
                            let i_id = inst.id;
                            let i_page = inst.page_ref;
                            let is_active = Signal::derive(move || selected_instrument_id.get() == i_id);
                            view! {
                                <button
                                    type="button"
                                    class=move || {
                                        if is_active.get() {
                                            "practice-tab-btn active"
                                        } else {
                                            "practice-tab-btn"
                                        }
                                    }
                                    on:click=move |_| {
                                        selected_instrument_id.set(i_id.to_string());
                                        if let Some(msd) = mobile_show_detail {
                                            msd.set(true);
                                        }
                                    }
                                >
                                    <span class="practice-tab-bullet">"🛠️"</span>
                                    <span class="practice-tab-label">{move || inst.name(current_lang.get())}</span>
                                    <span class="practice-tab-page">{i_page}</span>
                                </button>
                            }
                        }).collect_view()
                    }}
                </div>
            </div>

            // ================= INSTRUMENTOS: Coluna Direita (Detalhe) =================
            <div class="practice-detail-pane">
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
                                Language::PtBr => "Voltar para a Lista de Instrumentos",
                                Language::EnUs => "Back to Instruments List",
                            }}</span>
                        </button>
                    }
                })}

                {move || {
                    let hist_prev = history_practice_id.get();
                    let on_b = on_back_act.clone();
                    let back_nav = if let Some(prev_p_id) = hist_prev {
                        let prev_name = find_practice(&prev_p_id)
                            .map(|p| p.name(current_lang.get()))
                            .unwrap_or("Prática");
                        let p_id_to_back = prev_p_id.clone();
                        view! {
                            <div class="compendium-back-nav" style="margin-bottom: 0.8rem;">
                                <button
                                    type="button"
                                    class="compendium-back-btn"
                                    on:click=move |_| {
                                        if let Some(b) = on_b.clone() {
                                            b.call(p_id_to_back.clone());
                                        }
                                        history_practice_id.set(None);
                                    }
                                >
                                    "← " {move || match current_lang.get() {
                                        Language::PtBr => format!("Voltar para a Prática ('{}')", prev_name),
                                        Language::EnUs => format!("Back to Practice ('{}')", prev_name),
                                    }}
                                </button>
                            </div>
                        }.into_view()
                    } else {
                        view! { <span></span> }.into_view()
                    };

                    let is_theory = is_instrument_theory.get();
                    let on_sel = on_select_act.clone();
                    let on_cls = on_close_act.clone();

                    let detail_view = if is_theory {
                        let theory_item = active_theory_article.get();
                        // Visualização de Artigo Teórico / Regra Opcional
                        view! {
                            <div class="box-reading-view">
                                <div class="practice-detail-header">
                                    <div class="practice-title-row">
                                        <div class="practice-main-name">
                                            "⚖️ " {move || theory_item.title(current_lang.get())}
                                        </div>
                                        <span class="practice-page-badge">
                                            "📖 " {theory_item.page_ref}
                                        </span>
                                    </div>
                                    <div class="practice-aliases">
                                        <span class="practice-aliases-label">
                                            {move || match current_lang.get() {
                                                Language::PtBr => "Classificação M20: ",
                                                Language::EnUs => "M20 Classification: ",
                                            }}
                                        </span>
                                        <span class="practice-aliases-val">
                                            {move || match current_lang.get() {
                                                Language::PtBr => "Regras Canônicas de Foco e Instrumentos (Capítulo 10)",
                                                Language::EnUs => "Canonical Focus and Instrument Rules (Chapter 10)",
                                            }}
                                        </span>
                                    </div>
                                </div>

                                <aside class="compendium-callout-box standalone">
                                    <div class="callout-header">
                                        <span class="callout-icon">"📖"</span>
                                        <span class="callout-title">{move || theory_item.title(current_lang.get())}</span>
                                        <span class="callout-page-ref">{theory_item.page_ref}</span>
                                    </div>
                                    <div class="callout-content">
                                        {move || {
                                            let c = theory_item.content(current_lang.get());
                                            c.split("\n\n").map(|paragraph| {
                                                view! { <p class="callout-para">{paragraph.to_string()}</p> }
                                            }).collect_view()
                                        }}
                                    </div>
                                </aside>
                            </div>
                        }.into_view()
                    } else {
                        let inst_item = active_instrument.get();
                        let on_sel_act = on_sel;
                        let on_cls_act = on_cls;

                        // Visualização do Instrumento Selecionado
                        view! {
                            <div class="practice-reading-view">
                                <div class="practice-detail-header">
                                    <div class="practice-title-row">
                                        <div class="practice-main-name">
                                            "🛠️ " {move || inst_item.name(current_lang.get())}
                                        </div>
                                        <span class="practice-page-badge">
                                            "📖 " {inst_item.page_ref}
                                        </span>
                                    </div>
                                    <div class="practice-aliases">
                                        <span class="practice-aliases-label">
                                            {move || match current_lang.get() {
                                                Language::PtBr => "Nomes & Formas Comuns: ",
                                                Language::EnUs => "Common Names & Forms: ",
                                            }}
                                        </span>
                                        <span class="practice-aliases-val">
                                            {inst_item.aliases.join(", ")}
                                        </span>
                                    </div>
                                </div>

                                <div class="practice-description-wrap">
                                    <h4 class="practice-section-title">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "TEXTO INTEGRAL M20 (ORIGINAL DO LIVRO)",
                                            Language::EnUs => "COMPLETE M20 UNABRIDGED TEXT",
                                        }}
                                    </h4>
                                    <div class="practice-description-text">
                                        {move || {
                                            let desc = inst_item.description(current_lang.get());
                                            desc.split("\n\n").map(|paragraph| {
                                                view! { <p class="practice-para">{paragraph.to_string()}</p> }
                                            }).collect_view()
                                        }}
                                    </div>
                                </div>

                                // Botão de Ação: Selecionar Instrumento para o Slot da Ficha
                                {if let Some(cb) = on_sel_act {
                                    let i_name = inst_item.name(current_lang.get()).to_string();
                                    let on_cls_action = on_cls_act;
                                    view! {
                                        <div class="practice-action-row" style="margin-top: 1.5rem;">
                                            <button
                                                type="button"
                                                class="practice-select-btn"
                                                on:click=move |_| {
                                                    cb.call(i_name.clone());
                                                    if let Some(c) = &on_cls_action {
                                                        c.call(());
                                                    }
                                                }
                                            >
                                                {move || match current_lang.get() {
                                                    Language::PtBr => format!("✦ Usar '{}' no Slot de Instrumento", inst_item.name(current_lang.get())),
                                                    Language::EnUs => format!("✦ Use '{}' for Instrument Slot", inst_item.name(current_lang.get())),
                                                }}
                                            </button>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <span></span> }.into_view()
                                }}
                            </div>
                        }.into_view()
                    };

                    view! {
                        {back_nav}
                        {detail_view}
                    }
                }}
            </div>
        </div>
    }
}
