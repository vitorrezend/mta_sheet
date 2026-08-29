use leptos::*;
use crate::components::{Callback, StableTextArea};
use crate::state::CharacterData;

#[component]
pub fn Possessions() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    view! {
        <div class="group-box possessions-box">
            <span class="group-title">{move || crate::i18n::tr("possessions_title", lang())}</span>

            // Linha Superior: 3 Colunas (Gear Carried, Equipment Owned, Foci)
            <div class="possessions-grid-3col">
                <div class="possessions-col">
                    <label class="possessions-label">{move || crate::i18n::tr("gear_carried", lang())}</label>
                    <span class="possessions-sublabel">{move || crate::i18n::tr("gear_carried_sub", lang())}</span>
                    <StableTextArea 
                        class="possessions-textarea"
                        placeholder=Signal::derive(move || match lang() {
                            crate::i18n::Language::PtBr => "Itens, ferramentas, bolsas e objetos no bolso...".to_string(),
                            crate::i18n::Language::EnUs => "Items, tools, bags, and everyday carried objects...".to_string(),
                        })
                        value=Signal::derive(move || data.with(|d| d.possessions.gear_carried.clone()))
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| s.possessions.gear_carried = val);
                        })
                    />
                </div>

                <div class="possessions-col">
                    <label class="possessions-label">{move || crate::i18n::tr("equipment_owned", lang())}</label>
                    <span class="possessions-sublabel">{move || crate::i18n::tr("equipment_owned_sub", lang())}</span>
                    <StableTextArea 
                        class="possessions-textarea"
                        placeholder=Signal::derive(move || match lang() {
                            crate::i18n::Language::PtBr => "Veículos, cofres, eletrônicos e posses no refúgio...".to_string(),
                            crate::i18n::Language::EnUs => "Vehicles, safes, electronics, and owned property...".to_string(),
                        })
                        value=Signal::derive(move || data.with(|d| d.possessions.equipment_owned.clone()))
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| s.possessions.equipment_owned = val);
                        })
                    />
                </div>

                <div class="possessions-col">
                    <label class="possessions-label">{move || crate::i18n::tr("foci_title", lang())}</label>
                    <span class="possessions-sublabel">{move || crate::i18n::tr("foci_sub", lang())}</span>
                    <StableTextArea 
                        class="possessions-textarea"
                        placeholder=Signal::derive(move || match lang() {
                            crate::i18n::Language::PtBr => "Focos de paradigmas, varinhas, selos, instrumentos científicos...".to_string(),
                            crate::i18n::Language::EnUs => "Paradigm foci, wands, seals, scientific instruments...".to_string(),
                        })
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
                    <label class="possessions-label">{move || crate::i18n::tr("familiar_title", lang())}</label>
                    <span class="possessions-sublabel">{move || crate::i18n::tr("familiar_sub", lang())}</span>
                    <StableTextArea 
                        class="possessions-textarea"
                        placeholder=Signal::derive(move || match lang() {
                            crate::i18n::Language::PtBr => "Forma, atributos, laço místico, poderes e reservas de paradoxo...".to_string(),
                            crate::i18n::Language::EnUs => "Form, stats, mystical bond, powers, and paradox soak...".to_string(),
                        })
                        value=Signal::derive(move || data.with(|d| d.possessions.familiar.clone()))
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| s.possessions.familiar = val);
                        })
                    />
                </div>

                <div class="possessions-col">
                    <label class="possessions-label">{move || crate::i18n::tr("grimoire_title", lang())}</label>
                    <span class="possessions-sublabel">{move || crate::i18n::tr("grimoire_sub", lang())}</span>
                    <StableTextArea 
                        class="possessions-textarea"
                        placeholder=Signal::derive(move || match lang() {
                            crate::i18n::Language::PtBr => "Fórmulas arcanas, rotes transcritos, linguagens mágicas...".to_string(),
                            crate::i18n::Language::EnUs => "Arcane formulas, transcribed rotes, magical tongues...".to_string(),
                        })
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
