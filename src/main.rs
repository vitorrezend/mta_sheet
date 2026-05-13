mod components;
mod state;

use leptos::*;
use components::{Sheet, Attributes, InfoHeader, Abilities, Spheres, AdvantagesMta};

#[component]
fn App() -> impl IntoView {
    // Simulação de carregamento assíncrono
    view! {
        <Sheet>
            <InfoHeader />

            // Bloco de Atributos (Físicos, Sociais, Mentais)
            <Attributes />
            
            // Bloco de Habilidades (Talentos, Perícias, Conhecimentos)
            <Abilities />

            // Bloco de Esferas
            <Spheres />

            // Bloco de Vantagens (MTA)
            <AdvantagesMta />
            
        </Sheet>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
