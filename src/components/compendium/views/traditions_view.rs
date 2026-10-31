use leptos::*;
use crate::compendium::spheres::find_sphere;
use crate::compendium::traditions::{
    find_faction, TraditionDefinition, ALL_CONVENTIONS, ALL_DISPARATES, ALL_TRADITIONS, TRADITION_DESCRIPTIONS,
};
use crate::components::compendium::rich_text::RichTextView;
use crate::i18n::Language;

#[component]
pub fn TraditionsView(
    selected_tradition_id: RwSignal<String>,
    current_lang: Signal<Language>,
    #[prop(into, default = None)] mobile_show_detail: Option<RwSignal<bool>>,
) -> impl IntoView {
    let is_article = Signal::derive(move || selected_tradition_id.get() == TRADITION_DESCRIPTIONS.id);
    let active_tradition = Signal::derive(move || {
        find_faction(&selected_tradition_id.get()).unwrap_or(&ALL_TRADITIONS[0])
    });

    view! {
        <div
            class="compendium-section-split"
            class:mobile-show-detail=move || mobile_show_detail.map(|s| s.get()).unwrap_or(false)
        >
            // ================= TRADIÇÕES: Coluna Esquerda =================
            <div class="practice-sidebar-pane">
                <div class="special-box-tab-wrap" style="margin-bottom: 0.6rem;">
                    <button
                        type="button"
                        class=move || if is_article.get() { "practice-tab-btn special-box-tab active" } else { "practice-tab-btn special-box-tab" }
                        on:click=move |_| {
                            selected_tradition_id.set(TRADITION_DESCRIPTIONS.id.to_string());
                            if let Some(msd) = mobile_show_detail {
                                msd.set(true);
                            }
                        }
                    >
                        <span class="tab-indicator">"📜"</span>
                        <div class="tab-text-wrap">
                            <span class="tab-name">{move || TRADITION_DESCRIPTIONS.title(current_lang.get())}</span>
                            <span class="tab-sub">{TRADITION_DESCRIPTIONS.page_ref}</span>
                        </div>
                    </button>
                </div>

                <div class="practice-sidebar-title">
                    {move || match current_lang.get() {
                        Language::PtBr => "AS NOVE TRADIÇÕES (M20, pp. 148-165)",
                        Language::EnUs => "THE NINE TRADITIONS (M20, pp. 148-165)",
                    }}
                </div>
                <div class="practice-tab-list">
                    {ALL_TRADITIONS.iter().map(|t: &'static TraditionDefinition| {
                        let t_id = t.id;
                        let is_active = Signal::derive(move || selected_tradition_id.get() == t_id);
                        view! {
                            <button
                                type="button"
                                class=move || if is_active.get() { "practice-tab-btn active" } else { "practice-tab-btn" }
                                on:click=move |_| {
                                    selected_tradition_id.set(t_id.to_string());
                                    if let Some(msd) = mobile_show_detail {
                                        msd.set(true);
                                    }
                                }
                            >
                                <span class="practice-tab-bullet">"✦"</span>
                                <span class="practice-tab-label">{move || t.name(current_lang.get())}</span>
                                <span class="practice-tab-page">{t.page_ref}</span>
                            </button>
                        }
                    }).collect_view()}
                </div>

                <div class="practice-sidebar-title" style="margin-top: 0.9rem;">
                    {move || match current_lang.get() {
                        Language::PtBr => "A UNIÃO TECNOCRÁTICA (M20, pp. 186-195)",
                        Language::EnUs => "THE TECHNOCRATIC UNION (M20, pp. 186-195)",
                    }}
                </div>
                <div class="practice-tab-list">
                    {ALL_CONVENTIONS.iter().map(|t: &'static TraditionDefinition| {
                        let t_id = t.id;
                        let is_active = Signal::derive(move || selected_tradition_id.get() == t_id);
                        view! {
                            <button
                                type="button"
                                class=move || if is_active.get() { "practice-tab-btn active" } else { "practice-tab-btn" }
                                on:click=move |_| {
                                    selected_tradition_id.set(t_id.to_string());
                                    if let Some(msd) = mobile_show_detail {
                                        msd.set(true);
                                    }
                                }
                            >
                                <span class="practice-tab-bullet">"⚙"</span>
                                <span class="practice-tab-label">{move || t.name(current_lang.get())}</span>
                                <span class="practice-tab-page">{t.page_ref}</span>
                            </button>
                        }
                    }).collect_view()}
                </div>

                <div class="practice-sidebar-title" style="margin-top: 0.9rem;">
                    {move || match current_lang.get() {
                        Language::PtBr => "OS DISPARATES (M20, pp. 202-223)",
                        Language::EnUs => "THE DISPARATES (M20, pp. 202-223)",
                    }}
                </div>
                <div class="practice-tab-list">
                    {ALL_DISPARATES.iter().map(|t: &'static TraditionDefinition| {
                        let t_id = t.id;
                        let is_active = Signal::derive(move || selected_tradition_id.get() == t_id);
                        view! {
                            <button
                                type="button"
                                class=move || if is_active.get() { "practice-tab-btn active" } else { "practice-tab-btn" }
                                on:click=move |_| {
                                    selected_tradition_id.set(t_id.to_string());
                                    if let Some(msd) = mobile_show_detail {
                                        msd.set(true);
                                    }
                                }
                            >
                                <span class="practice-tab-bullet">"✧"</span>
                                <span class="practice-tab-label">{move || t.name(current_lang.get())}</span>
                                <span class="practice-tab-page">{t.page_ref}</span>
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>

            // ================= TRADIÇÕES: Coluna Direita (Detalhe) =================
            <div class="practice-detail-pane">
                {move || mobile_show_detail.map(|msd| {
                    view! {
                        <button
                            type="button"
                            class="compendium-mobile-back-btn"
                            on:click=move |_| msd.set(false)
                        >
                            <span class="back-arrow">"←"</span>
                            <span>{move || match current_lang.get() {
                                Language::PtBr => "Voltar para a Lista de Facções",
                                Language::EnUs => "Back to Factions List",
                            }}</span>
                        </button>
                    }
                })}

                {move || {
                    if is_article.get() {
                        view! {
                            <div class="practice-reading-view">
                                <div class="practice-detail-header special-box-header">
                                    <div class="practice-title-row">
                                        <div class="practice-main-name">
                                            "📜 " {move || TRADITION_DESCRIPTIONS.title(current_lang.get())}
                                        </div>
                                        <span class="practice-page-badge">"📖 " {TRADITION_DESCRIPTIONS.page_ref}</span>
                                    </div>
                                </div>
                                <div class="practice-description-text compendium-rich-content">
                                    <RichTextView text=Signal::derive(move || TRADITION_DESCRIPTIONS.content(current_lang.get()).to_string()) />
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        let tradition = active_tradition.get();
                        view! {
                            <div class="practice-reading-view">
                                <div class="practice-detail-header">
                                    <div class="practice-title-row">
                                        <div class="practice-title-left">
                                            <div class="practice-main-name">"🏛️ " {move || tradition.name(current_lang.get())}</div>
                                            <span class="practice-page-badge">"📖 " {tradition.page_ref}</span>
                                        </div>
                                    </div>
                                    <div class="practice-aliases">
                                        <span class="practice-aliases-label">
                                            {move || match current_lang.get() {
                                                Language::PtBr => "Nome original em inglês: ",
                                                Language::EnUs => "Book name: ",
                                            }}
                                        </span>
                                        <span class="practice-aliases-val">{tradition.name}</span>
                                    </div>
                                </div>

                                // Esferas de afinidade com link para o verbete de cada Esfera
                                <div class="practice-meta-section">
                                    <h4 class="practice-section-title">
                                        {move || match current_lang.get() {
                                            Language::PtBr => format!("ESFERAS DE AFINIDADE ({})", tradition.affinity_label(current_lang.get())),
                                            Language::EnUs => format!("AFFINITY SPHERES ({})", tradition.affinity_label(current_lang.get())),
                                        }}
                                    </h4>
                                    <div class="practice-pills-wrap">
                                        {move || {
                                            tradition.affinity_spheres.iter().map(|sid| {
                                                let label = find_sphere(sid)
                                                    .map(|s| s.name(current_lang.get()).to_string())
                                                    .unwrap_or_else(|| sid.to_string());
                                                view! {
                                                    <a class="practice-pill pill-clickable" href=format!("mta://spheres/{}", sid)>
                                                        {label}
                                                        <span class="pill-nav-icon">" ↗"</span>
                                                    </a>
                                                }
                                            }).collect_view()
                                        }}
                                    </div>
                                </div>

                                {move || if current_lang.get() == Language::PtBr && !tradition.has_pt_text() {
                                    view! {
                                        <p class="section-card-text" style="font-style: italic; margin-bottom: 0.75rem;">
                                            "Texto disponível por enquanto apenas em inglês."
                                        </p>
                                    }.into_view()
                                } else {
                                    view! { <span></span> }.into_view()
                                }}

                                <div class="practice-description-wrap">
                                    <h4 class="practice-section-title">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "TEXTO INTEGRAL M20 (ORIGINAL DO LIVRO)",
                                            Language::EnUs => "COMPLETE M20 UNABRIDGED TEXT",
                                        }}
                                    </h4>
                                    <div class="practice-description-text compendium-rich-content">
                                        <RichTextView text=Signal::derive(move || tradition.content(current_lang.get()).unwrap_or(tradition.content).to_string()) />
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
