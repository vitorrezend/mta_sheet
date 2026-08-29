mod arete;
mod antecedentes;
mod resonance;
mod experience;
mod willpower;
mod quintessence_paradox;
mod vitality;
mod quick_combat;

use leptos::*;
pub use arete::Arete;
pub use antecedentes::Antecedentes;
pub use resonance::Resonance;
pub use experience::Experience;
pub use willpower::Willpower;
pub use quintessence_paradox::QuintessenceParadox;
pub use vitality::Vitality;
pub use quick_combat::QuickCombat;

#[component]
pub fn AdvantagesMta() -> impl IntoView {
    view! {
        <div class="group-box">
            <span class="group-title">"Vantagens"</span>
            <div class="attributes-block">
                // Primeira Coluna: Antecedentes, Ressonância e Experiência
                <div class="attribute-column">
                    <Antecedentes />
                    <div class="column-spacer" style="height: 0.5rem;"></div>
                    <Resonance />
                    <div class="column-spacer" style="height: 0.5rem;"></div>
                    <Experience />
                </div>

                // Segunda Coluna: Arete no topo, Força de Vontade e Roda de Quintessência/Paradoxo
                <div class="attribute-column">
                    <Arete />
                    <Willpower />
                    <QuintessenceParadox />
                </div>

                // Terceira Coluna: Vitalidade e Combate Rápido
                <div class="attribute-column">
                    <Vitality />
                    <div class="column-spacer" style="height: 0.4rem;"></div>
                    <QuickCombat />
                </div>
            </div>
        </div>
    }
}

