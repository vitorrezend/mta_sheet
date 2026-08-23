mod arete;
mod antecedentes;
mod resonance;
mod experience;
mod willpower;
mod quintessence_paradox;
mod vitality;

use leptos::*;
pub use arete::Arete;
pub use antecedentes::Antecedentes;
pub use resonance::Resonance;
pub use experience::Experience;
pub use willpower::Willpower;
pub use quintessence_paradox::QuintessenceParadox;
pub use vitality::Vitality;

#[component]
pub fn AdvantagesMta() -> impl IntoView {
    view! {
        <div class="group-box">
            <span class="group-title">"Vantagens"</span>
            <div class="attributes-block">
                // Primeira Coluna: Antecedentes, Ressonância e Experiência
                <div class="attribute-column">
                    <Antecedentes />
                    <div class="column-spacer" style="height: 0.8rem;"></div>
                    <Resonance />
                    <div class="column-spacer" style="height: 0.8rem;"></div>
                    <Experience />
                </div>

                // Segunda Coluna: Arete no topo e Força de Vontade
                <div class="attribute-column">
                    <Arete />
                    <Willpower />
                    <QuintessenceParadox />
                </div>

                // Terceira Coluna: Vitalidade
                <div class="attribute-column">
                    <Vitality />
                </div>
            </div>
        </div>
    }
}
