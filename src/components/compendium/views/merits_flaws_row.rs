//! Linha da lista do Compêndio de Qualidades e Defeitos.

use leptos::*;
use crate::compendium::merits_flaws::{MeritFlawDefinition, TraitType};
use crate::components::compendium::views::merits_flaws_list::{twin, MainTab};
use crate::components::compendium::views::merits_flaws_view::get_category_icon;
use crate::i18n::Language;

/// Uma linha da lista (Qualidade ou Defeito), usada nos blocos por categoria e em "Mencionado em".
pub fn merit_flaw_row(
    mf: &'static MeritFlawDefinition,
    lang: Language,
    selected_item_id: RwSignal<String>,
    mobile_show_detail: Option<RwSignal<bool>>,
    tab: MainTab,
) -> View {
    let item_id = mf.id;
    let is_active = move || {
        let cur = selected_item_id.get();
        // O par Qualidade/Defeito fica destacado nas duas metades
        cur == item_id || twin(mf).is_some_and(|t| t.id == cur)
    };
    let is_merit = mf.trait_type == TraitType::Merit;
    // Na aba Todas, o par aparece numa linha só, com o selo "Qualidade ou Defeito"
    let paired = tab == MainTab::All && twin(mf).is_some();
    let (pill_class, pill_text) = if paired {
        ("compendium-sub-pill trait-pill-pair", match lang {
            Language::PtBr => "Qualidade ou Defeito",
            Language::EnUs => "Merit or Flaw",
        })
    } else if is_merit {
        ("compendium-sub-pill trait-pill-merit", mf.trait_type.name(lang))
    } else {
        ("compendium-sub-pill trait-pill-flaw", mf.trait_type.name(lang))
    };
    view! {
        <button
            type="button"
            class=move || if is_active() { "practice-tab-btn active" } else { "practice-tab-btn" }
            on:click=move |_| {
                selected_item_id.set(item_id.to_string());
                if let Some(m) = mobile_show_detail {
                    m.set(true);
                }
            }
        >
            <span class="tab-indicator">{get_category_icon(mf.category)}</span>
            <div class="tab-text-wrap">
                <div class="tab-name-row">
                    <span class="tab-name">{mf.name(lang)}</span>
                    <span class=pill_class>{pill_text}</span>
                </div>
                <div class="tab-name-row" style="margin-top: 2px;">
                    <span class="tab-sub">{mf.secondary_name(lang)}</span>
                    <span class="mf-row-meta">
                        <span class=format!("mf-source-pill mf-source-{}", mf.source_book().to_lowercase())>{mf.source_book()}</span>
                        <span class="points-badge-pill">{mf.points_display(lang)}</span>
                    </span>
                </div>
            </div>
        </button>
    }
    .into_view()
}

