pub mod merits_flaws;
pub mod other_traits;
pub mod wonder_card;
pub mod image_modal;
pub mod magic_section;
pub mod combat_section;

pub use merits_flaws::MeritsFlaws;
pub use other_traits::OtherTraits;
pub use wonder_card::WonderCard;
pub use image_modal::ImageModal;
pub use magic_section::MagicSection;
pub use combat_section::CombatSection;

use leptos::*;

#[component]
pub fn PageMagicCombat() -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    view! {
        <div class="page2-content">
            // Cabeçalho da Página 2
            <div class="page2-header-banner">
                <div class="page2-header-title-box">
                    <h2 class="page2-title">{move || crate::i18n::tr("page2_title", lang())}</h2>
                    <span class="page2-subtitle">
                        {move || crate::i18n::tr("page2_subtitle", lang())}
                    </span>
                </div>
            </div>

            // 1. Qualidades & Defeitos
            <MeritsFlaws />

            // 2. Outras Características
            <OtherTraits />

            // 3. Magia: Maravilhas & Rotes
            <MagicSection />

            // 4. Combate: Armas & Armadura
            <CombatSection />
        </div>
    }
}
