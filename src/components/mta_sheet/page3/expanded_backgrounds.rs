use leptos::*;
use crate::components::{Callback, StableTextArea, StableTextInput};
use crate::state::CharacterData;

#[component]
pub fn ExpandedBackgrounds() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    view! {
        <div class="group-box expanded-backgrounds-box">
            <span class="group-title">{move || crate::i18n::tr("expanded_bg_title", lang())}</span>

            <div class="expanded-bg-grid-2col">
                // Coluna Esquerda
                <div class="expanded-bg-col">
                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">{move || crate::i18n::tr("allies", lang())}</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Nomes, esferas de atuação, favores e detalhes dos aliados...".to_string(),
                                crate::i18n::Language::EnUs => "Names, influence areas, favors, and ally details...".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.allies.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.allies = val);
                            })
                        />
                    </div>

                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">{move || crate::i18n::tr("contacts", lang())}</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Rede de informantes, círculos sociais e canais de comunicação...".to_string(),
                                crate::i18n::Language::EnUs => "Informant networks, social circles, and contact channels...".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.contacts.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.contacts = val);
                            })
                        />
                    </div>

                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">{move || crate::i18n::tr("fame", lang())}</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Reputação pública, notoriedade mundana ou mística...".to_string(),
                                crate::i18n::Language::EnUs => "Public reputation, mundane or mystical renown...".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.fame.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.fame = val);
                            })
                        />
                    </div>

                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">{move || crate::i18n::tr("influence", lang())}</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Controle político, corporativo, acadêmico ou comunitário...".to_string(),
                                crate::i18n::Language::EnUs => "Political, corporate, academic, or community sway...".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.influence.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.influence = val);
                            })
                        />
                    </div>

                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">{move || crate::i18n::tr("library", lang())}</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Acervo de tomos, pergaminhos, arquivos digitais e pesquisas...".to_string(),
                                crate::i18n::Language::EnUs => "Collection of tomes, scrolls, digital archives, and research...".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.library.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.library = val);
                            })
                        />
                    </div>
                </div>

                // Coluna Direita
                <div class="expanded-bg-col">
                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">{move || crate::i18n::tr("node", lang())}</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Localização, tipo de ressonância, taxa de quintessência e defesa...".to_string(),
                                crate::i18n::Language::EnUs => "Location, resonance type, quintessence yield, and defenses...".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.node.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.node = val);
                            })
                        />
                    </div>

                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">{move || crate::i18n::tr("resources", lang())}</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Renda mensal, patrimônio, contas bancárias e propriedades...".to_string(),
                                crate::i18n::Language::EnUs => "Monthly income, assets, bank accounts, and real estate...".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.resources.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.resources = val);
                            })
                        />
                    </div>

                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">{move || crate::i18n::tr("retainers", lang())}</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Assistentes, guarda-costas, servos ou acólitos fiéis...".to_string(),
                                crate::i18n::Language::EnUs => "Assistants, bodyguards, servants, or devoted acolytes...".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.retainers.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.retainers = val);
                            })
                        />
                    </div>

                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">{move || crate::i18n::tr("sanctum", lang())}</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Laboratório protegido, oficina oculta, defesas mágicas e sigilos...".to_string(),
                                crate::i18n::Language::EnUs => "Warded lab, occult workshop, magical wards, and sigils...".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.sanctum.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.sanctum = val);
                            })
                        />
                    </div>

                    <div class="expanded-bg-item">
                        <div class="expanded-bg-other-header">
                            <span class="expanded-bg-label">{move || format!("{}: ", crate::i18n::tr("other_bg", lang()))}</span>
                            <StableTextInput 
                                class="expanded-bg-title-input"
                                placeholder=Signal::derive(move || match lang() {
                                    crate::i18n::Language::PtBr => "Nome do Antecedente...".to_string(),
                                    crate::i18n::Language::EnUs => "Background Name...".to_string(),
                                })
                                value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.other_title.clone()))
                                on_change=Callback::new(move |val| {
                                    set_data.update(|s| s.expanded_backgrounds.other_title = val);
                                })
                            />
                        </div>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Detalhes e histórico deste antecedente personalizado...".to_string(),
                                crate::i18n::Language::EnUs => "Details and history of this custom background...".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.other_text.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.other_text = val);
                            })
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}
