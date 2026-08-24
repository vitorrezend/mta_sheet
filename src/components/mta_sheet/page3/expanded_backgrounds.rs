use leptos::*;
use crate::components::{Callback, StableTextArea, StableTextInput};
use crate::state::CharacterData;

#[component]
pub fn ExpandedBackgrounds() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    view! {
        <div class="group-box expanded-backgrounds-box">
            <span class="group-title">"EXPANDED BACKGROUNDS"</span>

            <div class="expanded-bg-grid-2col">
                // Coluna Esquerda
                <div class="expanded-bg-col">
                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">"ALLIES (Aliados)"</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder="Nomes, esferas de atuação, favores e detalhes dos aliados..."
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.allies.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.allies = val);
                            })
                        />
                    </div>

                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">"CONTACTS (Contatos)"</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder="Rede de informantes, círculos sociais e canais de comunicação..."
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.contacts.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.contacts = val);
                            })
                        />
                    </div>

                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">"FAME (Fama)"</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder="Reputação pública, notoriedade mundana ou mística..."
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.fame.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.fame = val);
                            })
                        />
                    </div>

                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">"INFLUENCE (Influência)"</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder="Controle político, corporativo, acadêmico ou comunitário..."
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.influence.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.influence = val);
                            })
                        />
                    </div>

                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">"LIBRARY (Biblioteca)"</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder="Acervo de tomos, pergaminhos, arquivos digitais e pesquisas..."
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
                        <label class="expanded-bg-label">"NODE (Nodo)"</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder="Localização, tipo de ressonância, taxa de quintessência e defesa..."
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.node.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.node = val);
                            })
                        />
                    </div>

                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">"RESOURCES (Recursos)"</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder="Renda mensal, patrimônio, contas bancárias e propriedades..."
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.resources.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.resources = val);
                            })
                        />
                    </div>

                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">"RETAINERS (Lacaios / Seguidores)"</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder="Assistentes, guarda-costas, servos ou acólitos fiéis..."
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.retainers.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.retainers = val);
                            })
                        />
                    </div>

                    <div class="expanded-bg-item">
                        <label class="expanded-bg-label">"SANCTUM (Sanctum)"</label>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder="Laboratório protegido, oficina oculta, defesas mágicas e sigilos..."
                            value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.sanctum.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.expanded_backgrounds.sanctum = val);
                            })
                        />
                    </div>

                    <div class="expanded-bg-item">
                        <div class="expanded-bg-other-header">
                            <span class="expanded-bg-label">"OTHER ("</span>
                            <StableTextInput 
                                class="expanded-bg-title-input"
                                placeholder="Nome do Antecedente..."
                                value=Signal::derive(move || data.with(|d| d.expanded_backgrounds.other_title.clone()))
                                on_change=Callback::new(move |val| {
                                    set_data.update(|s| s.expanded_backgrounds.other_title = val);
                                })
                            />
                            <span class="expanded-bg-label">")"</span>
                        </div>
                        <StableTextArea 
                            class="expanded-bg-textarea"
                            placeholder="Detalhes e histórico deste antecedente personalizado..."
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
