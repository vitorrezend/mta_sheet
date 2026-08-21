pub mod merits_flaws;
pub mod other_traits;
pub mod magic_section;
pub mod combat_section;

pub use merits_flaws::MeritsFlaws;
pub use other_traits::OtherTraits;
pub use magic_section::MagicSection;
pub use combat_section::CombatSection;

use leptos::*;

#[component]
pub fn PageMagicCombat() -> impl IntoView {
    view! {
        <div class="page2-content">
            // Cabeçalho da Página 2
            <div class="page2-header-banner">
                <div class="page2-header-title-box">
                    <h2 class="page2-title">"MAGIA & COMBATE"</h2>
                    <span class="page2-subtitle">
                        "Qualidades & Defeitos • Outras Características • Maravilhas • Rotes • Armamento"
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
