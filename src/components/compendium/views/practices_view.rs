use leptos::*;
use crate::compendium::instruments::find_instrument;
use crate::compendium::practices::{
    find_practice, ALL_PRACTICES, BOX_LEFT_AND_RIGHT_HAND_PATHS, PracticeDefinition,
};
use crate::components::Callback;
use crate::i18n::Language;

#[component]
pub fn PracticesView(
    selected_practice_id: RwSignal<String>,
    current_lang: Signal<Language>,
    on_select_practice: Option<Callback<String>>,
    on_navigate_to_instrument: Option<Callback<(String, String)>>,
    on_close: Option<Callback<()>>,
) -> impl IntoView {
    let is_practice_box = Signal::derive(move || selected_practice_id.get() == "box_left_right");
    let active_practice = Signal::derive(move || {
        let cur_id = selected_practice_id.get();
        find_practice(&cur_id).unwrap_or(&ALL_PRACTICES[0])
    });

    let on_select_act = on_select_practice;
    let on_nav_inst = on_navigate_to_instrument;
    let on_close_act = on_close;

    view! {
        <div class="compendium-section-split">
            // ================= PRÁTICAS: Coluna Esquerda =================
            <div class="practice-sidebar-pane">
                // Box Especial
                <div class="special-box-tab-wrap" style="margin-bottom: 0.6rem;">
                    <button
                        type="button"
                        class=move || {
                            if is_practice_box.get() {
                                "practice-tab-btn special-box-tab active"
                            } else {
                                "practice-tab-btn special-box-tab"
                            }
                        }
                        on:click=move |_| selected_practice_id.set("box_left_right".to_string())
                    >
                        <span class="tab-indicator">"📜"</span>
                        <div class="tab-text-wrap">
                            <span class="tab-name">
                                {move || match current_lang.get() {
                                    Language::PtBr => "Mão Esquerda & Direita",
                                    Language::EnUs => "Left- & Right-Hand",
                                }}
                            </span>
                            <span class="tab-sub">{BOX_LEFT_AND_RIGHT_HAND_PATHS.page_ref}</span>
                        </div>
                    </button>
                </div>

                <div class="practice-sidebar-title">
                    {move || match current_lang.get() {
                        Language::PtBr => "TODAS AS 20 PRÁTICAS (M20, pp. 573-586)",
                        Language::EnUs => "ALL 20 PRACTICES (M20, pp. 573-586)",
                    }}
                </div>
                <div class="practice-tab-list">
                    {ALL_PRACTICES.iter().map(|p: &'static PracticeDefinition| {
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
                                on:click=move |_| selected_practice_id.set(p_id.to_string())
                            >
                                <span class="practice-tab-bullet">"✦"</span>
                                <span class="practice-tab-label">{move || p.name(current_lang.get())}</span>
                                <span class="practice-tab-page">{p_page}</span>
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>

            // ================= PRÁTICAS: Coluna Direita (Detalhe) =================
            <div class="practice-detail-pane">
                {move || {
                    if is_practice_box.get() {
                        // Visualização Dedicada do Box "Left- and Right-Hand Paths"
                        view! {
                            <div class="box-reading-view">
                                <div class="practice-detail-header">
                                    <div class="practice-title-row">
                                        <div class="practice-main-name">
                                            "📜 " {move || BOX_LEFT_AND_RIGHT_HAND_PATHS.title(current_lang.get())}
                                        </div>
                                        <span class="practice-page-badge">
                                            "📖 " {BOX_LEFT_AND_RIGHT_HAND_PATHS.page_ref}
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
                                            {move || match current_lang.get() {
                                                Language::PtBr => "Sidebar / Box de Regras M20 (Caminhos da Mão Esquerda e da Mão Direita)",
                                                Language::EnUs => "M20 Rules Sidebar / Feature Box (Left- and Right-Hand Paths)",
                                            }}
                                        </span>
                                    </div>
                                </div>

                                <aside class="compendium-callout-box standalone">
                                    <div class="callout-header">
                                        <span class="callout-icon">"⚖️"</span>
                                        <span class="callout-title">{move || BOX_LEFT_AND_RIGHT_HAND_PATHS.title(current_lang.get())}</span>
                                        <span class="callout-page-ref">{BOX_LEFT_AND_RIGHT_HAND_PATHS.page_ref}</span>
                                    </div>
                                    <div class="callout-content">
                                        {move || {
                                            let c = BOX_LEFT_AND_RIGHT_HAND_PATHS.content(current_lang.get());
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
                        let on_sel = on_select_act.clone();
                        let on_cls = on_close_act.clone();
                        let on_nav = on_nav_inst.clone();

                        view! {
                            <div class="practice-reading-view">
                                // Cabeçalho do Detalhe com Nome e Página
                                <div class="practice-detail-header">
                                    <div class="practice-title-row">
                                        <div class="practice-main-name">{move || practice.name(current_lang.get())}</div>
                                        <span class="practice-page-badge">
                                            "📖 " {practice.page_ref}
                                        </span>
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
                                <blockquote class="practice-quote">
                                    <span class="quote-mark">"“"</span>
                                    {move || practice.quote(current_lang.get())}
                                    <span class="quote-mark">"”"</span>
                                </blockquote>

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
                                            practice.paradigm_affinity(current_lang.get()).iter().map(|para| {
                                                view! { <span class="practice-pill pill-paradigm">{*para}</span> }
                                            }).collect_view()
                                        }}
                                    </div>
                                </div>

                                // Texto Descritivo Completo e Integral do M20
                                <div class="practice-description-wrap">
                                    <h4 class="practice-section-title">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "TEXTO INTEGRAL M20 (ORIGINAL DO LIVRO)",
                                            Language::EnUs => "COMPLETE M20 UNABRIDGED TEXT",
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
                                                            if let Some(found) = find_instrument(&inst_click) {
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
                                {if let Some(cb) = on_sel {
                                    let p_name = practice.name(current_lang.get()).to_string();
                                    let on_cls_action = on_cls;
                                    view! {
                                        <div class="practice-action-row" style="margin-top: 1.5rem;">
                                            <button
                                                type="button"
                                                class="practice-select-btn"
                                                on:click=move |_| {
                                                    cb.call(p_name.clone());
                                                    if let Some(c) = &on_cls_action {
                                                        c.call(());
                                                    }
                                                }
                                            >
                                                {move || match current_lang.get() {
                                                    Language::PtBr => format!("✦ Usar '{}' no Slot de Prática", practice.name(current_lang.get())),
                                                    Language::EnUs => format!("✦ Use '{}' for Practice Slot", practice.name(current_lang.get())),
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
