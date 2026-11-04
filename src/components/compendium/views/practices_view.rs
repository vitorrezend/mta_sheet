use leptos::*;
use crate::compendium::instruments::find_instrument;
use crate::compendium::paradigms::find_paradigm;
use crate::compendium::practices::{
    find_practice, find_practice_box, ALL_PRACTICES, BOX_EXPANDED_PRACTICES,
    BOX_LEFT_AND_RIGHT_HAND_PATHS, PracticeDefinition,
};
use crate::components::Callback;
use crate::i18n::Language;

/// Botão de quadro (sidebar) na coluna da lista de práticas.
fn box_tab(
    box_id: &'static str,
    label_pt: &'static str,
    label_en: &'static str,
    page_ref: &'static str,
    selected_practice_id: RwSignal<String>,
    current_lang: Signal<Language>,
    mobile_show_detail: Option<RwSignal<bool>>,
) -> impl IntoView {
    view! {
        <div class="special-box-tab-wrap" style="margin-bottom: 0.6rem;">
            <button
                type="button"
                class=move || {
                    if selected_practice_id.get() == box_id {
                        "practice-tab-btn special-box-tab active"
                    } else {
                        "practice-tab-btn special-box-tab"
                    }
                }
                on:click=move |_| {
                    selected_practice_id.set(box_id.to_string());
                    if let Some(msd) = mobile_show_detail {
                        msd.set(true);
                    }
                }
            >
                <span class="tab-indicator">"📜"</span>
                <div class="tab-text-wrap">
                    <span class="tab-name">
                        {move || match current_lang.get() {
                            Language::PtBr => label_pt,
                            Language::EnUs => label_en,
                        }}
                    </span>
                    <span class="tab-sub">{page_ref}</span>
                </div>
            </button>
        </div>
    }
}

/// Lista de botões de práticas filtrada pela fonte (M20 ou Book of Secrets).
fn practice_tabs(
    from_bos: bool,
    selected_practice_id: RwSignal<String>,
    current_lang: Signal<Language>,
    mobile_show_detail: Option<RwSignal<bool>>,
) -> impl IntoView {
    view! {
        <div class="practice-tab-list">
            {ALL_PRACTICES.iter().filter(move |p| p.page_ref.starts_with("BoS") == from_bos).map(|p: &'static PracticeDefinition| {
                let p_id = p.id;
                let p_page = p.page_ref;
                let is_active = Signal::derive(move || selected_practice_id.get() == p_id);
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
                            selected_practice_id.set(p_id.to_string());
                            if let Some(msd) = mobile_show_detail {
                                msd.set(true);
                            }
                        }
                    >
                        <span class="practice-tab-bullet">"✦"</span>
                        <span class="practice-tab-label">{move || p.name(current_lang.get())}</span>
                        <span class="practice-tab-page">{p_page}</span>
                    </button>
                }
            }).collect_view()}
        </div>
    }
}

#[component]
pub fn PracticesView(
    selected_practice_id: RwSignal<String>,
    current_lang: Signal<Language>,
    #[prop(into, default = None)] mobile_show_detail: Option<RwSignal<bool>>,
    on_select_practice: Option<Callback<String>>,
    on_navigate_to_instrument: Option<Callback<(String, String)>>,
    on_close: Option<Callback<()>>,
) -> impl IntoView {
    let active_box = Signal::derive(move || find_practice_box(&selected_practice_id.get()));
    let n_m20 = ALL_PRACTICES.iter().filter(|p| !p.page_ref.starts_with("BoS")).count();
    let n_bos = ALL_PRACTICES.len() - n_m20;
    let active_practice = Signal::derive(move || {
        let cur_id = selected_practice_id.get();
        find_practice(&cur_id).unwrap_or(&ALL_PRACTICES[0])
    });

    let on_select_act = on_select_practice;
    let on_nav_inst = on_navigate_to_instrument;
    let on_close_act = on_close;

    view! {
        <div 
            class="compendium-section-split"
            class:mobile-show-detail=move || mobile_show_detail.map(|s| s.get()).unwrap_or(false)
        >
            // ================= PRÁTICAS: Coluna Esquerda =================
            <div class="practice-sidebar-pane">
                // Quadro especial do M20
                {box_tab("box_left_right", "Mão Esquerda & Direita", "Left- & Right-Hand", BOX_LEFT_AND_RIGHT_HAND_PATHS.page_ref, selected_practice_id, current_lang, mobile_show_detail)}

                <div class="practice-sidebar-title">
                    {move || match current_lang.get() {
                        Language::PtBr => format!("{} PRÁTICAS DO M20 (pp. 573-586)", n_m20),
                        Language::EnUs => format!("{} M20 PRACTICES (pp. 573-586)", n_m20),
                    }}
                </div>
                {practice_tabs(false, selected_practice_id, current_lang, mobile_show_detail)}

                <div class="practice-sidebar-title" style="margin-top: 0.6rem;">
                    {move || match current_lang.get() {
                        Language::PtBr => format!("{} PRÁTICAS EXPANDIDAS (BoS, pp. 196-205)", n_bos),
                        Language::EnUs => format!("{} EXPANDED PRACTICES (BoS, pp. 196-205)", n_bos),
                    }}
                </div>
                {box_tab("box_expanded_practices", "Práticas Expandidas", "Expanded Practices", BOX_EXPANDED_PRACTICES.page_ref, selected_practice_id, current_lang, mobile_show_detail)}
                {practice_tabs(true, selected_practice_id, current_lang, mobile_show_detail)}
            </div>

            // ================= PRÁTICAS: Coluna Direita (Detalhe) =================
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
                                Language::PtBr => "Voltar para a Lista de Práticas",
                                Language::EnUs => "Back to Practices List",
                            }}</span>
                        </button>
                    }
                })}

                {move || {
                    if let Some(bx) = active_box.get() {
                        // Visualização dedicada de um quadro (sidebar) da seção
                        view! {
                            <div class="box-reading-view">
                                <div class="practice-detail-header">
                                    <div class="practice-title-row">
                                        <div class="practice-main-name">
                                            "📜 " {move || bx.title(current_lang.get())}
                                        </div>
                                        <span class="practice-page-badge">
                                            "📖 " {bx.page_ref}
                                        </span>
                                    </div>
                                    <div class="practice-aliases">
                                        <span class="practice-aliases-label">
                                            {move || match current_lang.get() {
                                                Language::PtBr => "Tipo de Conteúdo: ",
                                                Language::EnUs => "Content Type: ",
                                            }}
                                        </span>
                                        <span class="practice-aliases-val">
                                            {move || match (current_lang.get(), bx.page_ref.starts_with("BoS")) {
                                                (Language::PtBr, true) => "Texto do Livro dos Segredos",
                                                (Language::EnUs, true) => "Book of Secrets text",
                                                (Language::PtBr, false) => "Quadro de Regras do M20",
                                                (Language::EnUs, false) => "M20 Rules Sidebar",
                                            }}
                                        </span>
                                    </div>
                                </div>

                                <aside class="compendium-callout-box standalone">
                                    <div class="callout-header">
                                        <span class="callout-icon">"⚖️"</span>
                                        <span class="callout-title">{move || bx.title(current_lang.get())}</span>
                                        <span class="callout-page-ref">{bx.page_ref}</span>
                                    </div>
                                    <div class="callout-content">
                                        {move || {
                                            let c = bx.content(current_lang.get());
                                            c.split("\n\n").map(|paragraph| {
                                                view! { <p class="callout-para">{paragraph.to_string()}</p> }
                                            }).collect_view()
                                        }}
                                    </div>
                                </aside>
                            </div>
                        }.into_view()
                    } else {
                        let practice = active_practice.get();
                        let on_sel_header = on_select_act.clone();
                        let on_sel_footer = on_select_act.clone();
                        let on_cls_header = on_close_act.clone();
                        let on_cls_footer = on_close_act.clone();
                        let on_nav = on_nav_inst.clone();

                        view! {
                            <div class="practice-reading-view">
                                // Cabeçalho do Detalhe com Nome e Página
                                <div class="practice-detail-header">
                                    <div class="practice-title-row">
                                        <div class="practice-title-left">
                                            <div class="practice-main-name">{move || practice.name(current_lang.get())}</div>
                                            <span class="practice-page-badge">
                                                "📖 " {practice.page_ref}
                                            </span>
                                        </div>

                                        {if let Some(cb) = on_sel_header {
                                            let p_name = practice.name(current_lang.get()).to_string();
                                            let on_cls_h = on_cls_header;
                                            view! {
                                                <div class="compendium-header-actions">
                                                    <button
                                                        type="button"
                                                        class="compendium-action-btn compendium-btn-emerald"
                                                        on:click=move |_| {
                                                            cb.call(p_name.clone());
                                                            if let Some(c) = &on_cls_h {
                                                                c.call(());
                                                            }
                                                        }
                                                    >
                                                        <span class="compendium-btn-icon">"✦"</span>
                                                        <span class="compendium-btn-label">
                                                            {move || match current_lang.get() {
                                                                Language::PtBr => "Usar na Ficha",
                                                                Language::EnUs => "Use on Sheet",
                                                            }}
                                                        </span>
                                                    </button>
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! { <span></span> }.into_view()
                                        }}
                                    </div>
                                    <div class="practice-aliases">
                                        <span class="practice-aliases-label">
                                            {move || match current_lang.get() {
                                                Language::PtBr => "Também conhecida como / Nomes: ",
                                                Language::EnUs => "Also known as: ",
                                            }}
                                        </span>
                                        <span class="practice-aliases-val">
                                            {practice.aliases.join(", ")}
                                        </span>
                                    </div>
                                </div>

                                // Citação / Filosofia da Prática
                                {(!practice.quote.is_empty()).then(|| view! {
                                    <blockquote class="practice-quote">
                                        <span class="quote-mark">"“"</span>
                                        {move || practice.quote(current_lang.get())}
                                        <span class="quote-mark">"”"</span>
                                    </blockquote>
                                })}

                                // Callout Boxes Associados à Prática (se houver)
                                {practice.callouts.iter().map(|box_item| {
                                    view! {
                                        <aside class="compendium-callout-box">
                                            <div class="callout-header">
                                                <span class="callout-icon">"📜"</span>
                                                <span class="callout-title">{move || box_item.title(current_lang.get())}</span>
                                                <span class="callout-page-ref">{box_item.page_ref}</span>
                                            </div>
                                            <div class="callout-content">
                                                {move || {
                                                    let c = box_item.content(current_lang.get());
                                                    c.split("\n\n").map(|p| {
                                                        view! { <p class="callout-para">{p.to_string()}</p> }
                                                    }).collect_view()
                                                }}
                                            </div>
                                        </aside>
                                    }
                                }).collect_view()}

                                // Paradigmas Afins
                                <div class="practice-meta-section">
                                    <h4 class="practice-section-title">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "PARADIGMAS ASSOCIADOS",
                                            Language::EnUs => "ASSOCIATED PARADIGMS",
                                        }}
                                    </h4>
                                    <div class="practice-pills-wrap">
                                        {move || {
                                            let lang = current_lang.get();
                                            practice.paradigm_affinity(lang).iter().enumerate().map(|(i, para)| {
                                                // Paradigmas do Livro dos Segredos viram links para a seção Paradigmas
                                                let en = practice.paradigm_affinity.get(i).copied().unwrap_or(para);
                                                let base = en.split('(').next().unwrap_or(en).trim();
                                                match find_paradigm(base).filter(|p| p.name.eq_ignore_ascii_case(base)) {
                                                    Some(p) => view! {
                                                        <a class="practice-pill pill-paradigm pill-clickable" href=format!("mta://paradigms/{}", p.id)>
                                                            {*para}
                                                            <span class="pill-nav-icon">" ↗"</span>
                                                        </a>
                                                    }.into_view(),
                                                    None => view! { <span class="practice-pill pill-paradigm">{*para}</span> }.into_view(),
                                                }
                                            }).collect_view()
                                        }}
                                    </div>
                                </div>

                                // Texto Descritivo Completo e Integral do M20
                                <div class="practice-description-wrap">
                                    <h4 class="practice-section-title">
                                        {move || match (current_lang.get(), practice.page_ref.starts_with("BoS")) {
                                            (Language::PtBr, false) => "TEXTO INTEGRAL M20 (ORIGINAL DO LIVRO)",
                                            (Language::EnUs, false) => "COMPLETE M20 UNABRIDGED TEXT",
                                            (Language::PtBr, true) => "TEXTO INTEGRAL (LIVRO DOS SEGREDOS)",
                                            (Language::EnUs, true) => "COMPLETE UNABRIDGED TEXT (BOOK OF SECRETS)",
                                        }}
                                    </h4>
                                    <div class="practice-description-text">
                                        {move || {
                                            let desc = practice.description(current_lang.get());
                                            desc.split("\n\n").map(|paragraph| {
                                                view! { <p class="practice-para">{paragraph.to_string()}</p> }
                                            }).collect_view()
                                        }}
                                    </div>
                                </div>

                                // Instrumentos Comuns Sugeridos - COM PÍLULAS INTERATIVAS / LINKS
                                <div class="practice-meta-section">
                                    <h4 class="practice-section-title">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "INSTRUMENTOS COMUNS (CLIQUE PARA VER REGRAS M20)",
                                            Language::EnUs => "COMMON INSTRUMENTS (CLICK TO VIEW M20 RULES)",
                                        }}
                                    </h4>
                                    <div class="practice-pills-wrap">
                                        {move || {
                                            let p_id = practice.id.to_string();
                                            let on_n = on_nav.clone();
                                            practice.common_instruments(current_lang.get()).iter().map(|inst| {
                                                let inst_title = inst.to_string();
                                                let inst_click = inst.to_string();
                                                let p_id_hist = p_id.clone();
                                                let on_n_item = on_n.clone();
                                                view! {
                                                    <button
                                                        type="button"
                                                        class="practice-pill pill-instrument pill-clickable"
                                                        title=move || match current_lang.get() {
                                                            Language::PtBr => format!("Ver verbete do instrumento '{}' no Compêndio", inst_title),
                                                            Language::EnUs => format!("View entry for '{}' in Compendium", inst_title),
                                                        }
                                                        on:click=move |_| {
                                                            let base = inst_click.split('(').next().unwrap_or("").trim().to_string();
                                                            if let Some(found) = find_instrument(&inst_click).or_else(|| find_instrument(&base)) {
                                                                if let Some(n) = on_n_item.clone() {
                                                                    n.call((p_id_hist.clone(), found.id.to_string()));
                                                                }
                                                            }
                                                        }
                                                    >
                                                        {*inst}
                                                        <span class="pill-nav-icon">" ↗"</span>
                                                    </button>
                                                }
                                            }).collect_view()
                                        }}
                                    </div>
                                </div>

                                // Habilidades Associadas
                                <div class="practice-meta-section">
                                    <h4 class="practice-section-title">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "HABILIDADES ASSOCIADAS (ASSOCIATED ABILITIES)",
                                            Language::EnUs => "ASSOCIATED ABILITIES",
                                        }}
                                    </h4>
                                    <div class="practice-pills-wrap">
                                        {move || {
                                            practice.associated_abilities(current_lang.get()).iter().map(|ab| {
                                                view! { <span class="practice-pill pill-ability">{*ab}</span> }
                                            }).collect_view()
                                        }}
                                    </div>
                                </div>

                                // Botão de Ação: Selecionar Prática para o Slot da Ficha
                                {if let Some(cb) = on_sel_footer {
                                    let p_name = practice.name(current_lang.get()).to_string();
                                    let on_cls_action = on_cls_footer;
                                    view! {
                                        <div class="compendium-footer-actions">
                                            <button
                                                type="button"
                                                class="compendium-action-btn compendium-btn-emerald compendium-btn-large"
                                                on:click=move |_| {
                                                    cb.call(p_name.clone());
                                                    if let Some(c) = &on_cls_action {
                                                        c.call(());
                                                    }
                                                }
                                            >
                                                <span class="compendium-btn-icon">"✦"</span>
                                                <span class="compendium-btn-label">
                                                    {move || match current_lang.get() {
                                                        Language::PtBr => format!("Usar '{}' no Slot de Prática", practice.name(current_lang.get())),
                                                        Language::EnUs => format!("Use '{}' for Practice Slot", practice.name(current_lang.get())),
                                                    }}
                                                </span>
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
