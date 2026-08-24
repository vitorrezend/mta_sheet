use leptos::*;
use crate::components::{Callback, StableTextArea};
use crate::state::CharacterData;

#[component]
pub fn Possessions() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    view! {
        <div class="group-box possessions-box">
            <span class="group-title">"POSSESSIONS"</span>

            // Linha Superior: 3 Colunas (Gear Carried, Equipment Owned, Foci)
            <div class="possessions-grid-3col">
                <div class="possessions-col">
                    <label class="possessions-label">"GEAR (Carried)"</label>
                    <span class="possessions-sublabel">"Equipamento Carregado"</span>
                    <StableTextArea 
                        class="possessions-textarea"
                        placeholder="Itens, ferramentas, bolsas e objetos no bolso..."
                        value=Signal::derive(move || data.with(|d| d.possessions.gear_carried.clone()))
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| s.possessions.gear_carried = val);
                        })
                    />
                </div>

                <div class="possessions-col">
                    <label class="possessions-label">"EQUIPMENT (Owned)"</label>
                    <span class="possessions-sublabel">"Equipamento Possuído"</span>
                    <StableTextArea 
                        class="possessions-textarea"
                        placeholder="Veículos, cofres, eletrônicos e posses no refúgio..."
                        value=Signal::derive(move || data.with(|d| d.possessions.equipment_owned.clone()))
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| s.possessions.equipment_owned = val);
                        })
                    />
                </div>

                <div class="possessions-col">
                    <label class="possessions-label">"FOCI"</label>
                    <span class="possessions-sublabel">"Focos & Instrumentos Mágicos"</span>
                    <StableTextArea 
                        class="possessions-textarea"
                        placeholder="Focos de paradigmas, varinhas, selos, instrumentos científicos..."
                        value=Signal::derive(move || data.with(|d| d.possessions.foci.clone()))
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| s.possessions.foci = val);
                        })
                    />
                </div>
            </div>

            // Linha Inferior: 2 Colunas (Familiar, Grimoire)
            <div class="possessions-grid-2col">
                <div class="possessions-col">
                    <label class="possessions-label">"FAMILIAR"</label>
                    <span class="possessions-sublabel">"Familiar & Companheiros Espirituais"</span>
                    <StableTextArea 
                        class="possessions-textarea"
                        placeholder="Forma, atributos, laço místico, poderes e reservas de paradoxo..."
                        value=Signal::derive(move || data.with(|d| d.possessions.familiar.clone()))
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| s.possessions.familiar = val);
                        })
                    />
                </div>

                <div class="possessions-col">
                    <label class="possessions-label">"GRIMOIRE"</label>
                    <span class="possessions-sublabel">"Grimório & Tomos de Pesquisa"</span>
                    <StableTextArea 
                        class="possessions-textarea"
                        placeholder="Fórmulas arcanas, rotes transcritos, linguagens mágicas..."
                        value=Signal::derive(move || data.with(|d| d.possessions.grimoire.clone()))
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| s.possessions.grimoire = val);
                        })
                    />
                </div>
            </div>
        </div>
    }
}
