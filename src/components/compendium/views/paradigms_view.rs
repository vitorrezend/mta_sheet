//! Seção "Paradigmas" do Compêndio: Paradigmas Expandidos do Book of Secrets (BoS, pp. 188-196).

use leptos::*;
use crate::compendium::paradigms::{
    find_paradigm, ParadigmDefinition, ALL_PARADIGMS, ARTICLE_EXPANDED_PARADIGMS,
};
use crate::compendium::practices::find_practice;
use crate::i18n::Language;

/// Id da prática do compêndio citada num item de "Associated Practices", se houver.
/// Ignora comentários entre parênteses: "Chaos magick (which has...)" -> "chaos_magick".
pub fn associated_practice_id(item: &str) -> Option<&'static str> {
    let base = item.split('(').next().unwrap_or(item).trim();
    let base = base.strip_prefix("The ").unwrap_or(base);
    let base = base.split('/').next().unwrap_or(base).trim();
    if base.is_empty() {
        return None;
    }
    find_practice(base)
        .or_else(|| find_practice(&base.replace('-', " ")))
        .map(|p| p.id)
}

#[component]
pub fn ParadigmsView(
    selected_paradigm_id: RwSignal<String>,
    current_lang: Signal<Language>,
    #[prop(into, default = None)] mobile_show_detail: Option<RwSignal<bool>>,
) -> impl IntoView {
    let is_article = Signal::derive(move || selected_paradigm_id.get() == ARTICLE_EXPANDED_PARADIGMS.id);
    let active = Signal::derive(move || {
        find_paradigm(&selected_paradigm_id.get()).unwrap_or(&ALL_PARADIGMS[0])
    });
    let select = move |id: &'static str| {
        selected_paradigm_id.set(id.to_string());
        if let Some(msd) = mobile_show_detail {
            msd.set(true);
        }
    };

    view! {
        <div
            class="compendium-section-split"
            class:mobile-show-detail=move || mobile_show_detail.map(|s| s.get()).unwrap_or(false)
        >
            <div class="practice-sidebar-pane">
                <div class="special-box-tab-wrap" style="margin-bottom: 0.6rem;">
                    <button
                        type="button"
                        class=move || if is_article.get() { "practice-tab-btn special-box-tab active" } else { "practice-tab-btn special-box-tab" }
                        on:click=move |_| select(ARTICLE_EXPANDED_PARADIGMS.id)
                    >
                        <span class="tab-indicator">"📜"</span>
                        <div class="tab-text-wrap">
                            <span class="tab-name">{move || ARTICLE_EXPANDED_PARADIGMS.title(current_lang.get())}</span>
                            <span class="tab-sub">{ARTICLE_EXPANDED_PARADIGMS.page_ref}</span>
                        </div>
                    </button>
                </div>

                <div class="practice-sidebar-title">
                    {move || match current_lang.get() {
                        Language::PtBr => format!("{} PARADIGMAS (BoS, pp. 188-196)", ALL_PARADIGMS.len()),
                        Language::EnUs => format!("{} PARADIGMS (BoS, pp. 188-196)", ALL_PARADIGMS.len()),
                    }}
                </div>
                <div class="practice-tab-list">
                    {ALL_PARADIGMS.iter().map(|p: &'static ParadigmDefinition| {
                        let p_id = p.id;
                        let is_active = Signal::derive(move || selected_paradigm_id.get() == p_id);
                        view! {
                            <button
                                type="button"
                                class=move || if is_active.get() { "practice-tab-btn active" } else { "practice-tab-btn" }
                                on:click=move |_| select(p_id)
                            >
                                <span class="practice-tab-bullet">"☯"</span>
                                <span class="practice-tab-label">{move || p.name(current_lang.get())}</span>
                                <span class="practice-tab-page">{p.page_ref}</span>
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>

            <div class="practice-detail-pane">
                {move || mobile_show_detail.map(|msd| {
                    view! {
                        <button type="button" class="compendium-mobile-back-btn" on:click=move |_| msd.set(false)>
                            <span class="back-arrow">"←"</span>
                            <span>{move || match current_lang.get() {
                                Language::PtBr => "Voltar para a Lista de Paradigmas",
                                Language::EnUs => "Back to Paradigms List",
                            }}</span>
                        </button>
                    }
                })}

                {move || {
                    if is_article.get() {
                        let art = &ARTICLE_EXPANDED_PARADIGMS;
                        view! {
                            <div class="practice-reading-view">
                                <div class="practice-detail-header">
                                    <div class="practice-title-row">
                                        <div class="practice-title-left">
                                            <div class="practice-main-name">{move || art.title(current_lang.get())}</div>
                                            <span class="practice-page-badge">"📖 " {art.page_ref}</span>
                                        </div>
                                    </div>
                                </div>
                                <div class="practice-description-wrap">
                                    <div class="practice-description-text">
                                        {move || art.content(current_lang.get()).split("\n\n").map(|para| {
                                            view! { <p class="practice-para">{para.to_string()}</p> }
                                        }).collect_view()}
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        let paradigm = active.get();
                        view! {
                            <div class="practice-reading-view">
                                <div class="practice-detail-header">
                                    <div class="practice-title-row">
                                        <div class="practice-title-left">
                                            <div class="practice-main-name">{move || paradigm.name(current_lang.get())}</div>
                                            <span class="practice-page-badge">"📖 " {paradigm.page_ref}</span>
                                        </div>
                                    </div>
                                    <div class="practice-aliases">
                                        <span class="practice-aliases-label">
                                            {move || match current_lang.get() {
                                                Language::PtBr => "Nome original: ",
                                                Language::EnUs => "Portuguese name: ",
                                            }}
                                        </span>
                                        <span class="practice-aliases-val">
                                            {move || match current_lang.get() {
                                                Language::PtBr => paradigm.name,
                                                Language::EnUs => paradigm.name_pt,
                                            }}
                                        </span>
                                    </div>
                                </div>

                                <div class="practice-description-wrap">
                                    <h4 class="practice-section-title">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "TEXTO INTEGRAL (LIVRO DOS SEGREDOS)",
                                            Language::EnUs => "COMPLETE UNABRIDGED TEXT (BOOK OF SECRETS)",
                                        }}
                                    </h4>
                                    <div class="practice-description-text">
                                        {move || paradigm.description(current_lang.get()).split("\n\n").map(|para| {
                                            view! { <p class="practice-para">{para.to_string()}</p> }
                                        }).collect_view()}
                                    </div>
                                </div>

                                <div class="practice-meta-section">
                                    <h4 class="practice-section-title">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "PRÁTICAS ASSOCIADAS",
                                            Language::EnUs => "ASSOCIATED PRACTICES",
                                        }}
                                    </h4>
                                    <div class="practice-pills-wrap">
                                        {move || {
                                            let lang = current_lang.get();
                                            paradigm.associated_practices(lang).iter().enumerate().map(|(i, label)| {
                                                let en = paradigm.associated_practices.get(i).copied().unwrap_or(label);
                                                match associated_practice_id(en) {
                                                    Some(id) => view! {
                                                        <a class="practice-pill pill-instrument pill-clickable" href=format!("mta://practices/{}", id)>
                                                            {*label}
                                                            <span class="pill-nav-icon">" ↗"</span>
                                                        </a>
                                                    }.into_view(),
                                                    None => view! { <span class="practice-pill pill-paradigm">{*label}</span> }.into_view(),
                                                }
                                            }).collect_view()
                                        }}
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

#[cfg(test)]
mod tests {
    use super::associated_practice_id;

    #[test]
    fn associated_practices_resolve_to_compendium_entries() {
        assert_eq!(associated_practice_id("Chaos magick (which has plenty of weird ties)"), Some("chaos_magick"));
        assert_eq!(associated_practice_id("god-bonding (likewise)"), Some("god_bonding"));
        assert_eq!(associated_practice_id("The Art of Desire/hypereconomics"), Some("art_of_desire"));
        assert_eq!(associated_practice_id("medicine-work"), Some("medicine_work"));
        assert_eq!(associated_practice_id("postmodern variations on yoga"), None);
    }
}
