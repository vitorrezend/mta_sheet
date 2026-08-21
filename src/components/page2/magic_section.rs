use leptos::*;
use crate::state::{CharacterData, WonderItem};

#[component]
pub fn MagicSection() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let render_wonder_card = move |idx: usize| {
        view! {
            <div class="wonder-card">
                <div class="wonder-card-top-row">
                    <div class="wonder-field name-field">
                        <label class="wonder-label">"Nome:"</label>
                        <input 
                            type="text" 
                            class="wonder-input"
                            placeholder="Nome do artefato / maravilha..."
                            prop:value=move || data.with(|d| d.wonders.get(idx).map(|w| w.name.clone()).unwrap_or_default())
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                set_data.update(|s| {
                                    while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
                                    s.wonders[idx].name = val;
                                });
                            }
                        />
                    </div>
                </div>

                <div class="wonder-stats-row">
                    <div class="wonder-field stat-field">
                        <label class="wonder-label">"Pontos:"</label>
                        <input 
                            type="text" 
                            class="wonder-input text-center"
                            placeholder="Pts"
                            prop:value=move || data.with(|d| d.wonders.get(idx).map(|w| w.points.clone()).unwrap_or_default())
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                set_data.update(|s| {
                                    while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
                                    s.wonders[idx].points = val;
                                });
                            }
                        />
                    </div>

                    <div class="wonder-field stat-field">
                        <label class="wonder-label">"Arete:"</label>
                        <input 
                            type="text" 
                            class="wonder-input text-center"
                            placeholder="Arete"
                            prop:value=move || data.with(|d| d.wonders.get(idx).map(|w| w.arete.clone()).unwrap_or_default())
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                set_data.update(|s| {
                                    while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
                                    s.wonders[idx].arete = val;
                                });
                            }
                        />
                    </div>

                    <div class="wonder-field stat-field">
                        <label class="wonder-label">"Quintessência:"</label>
                        <input 
                            type="text" 
                            class="wonder-input text-center"
                            placeholder="Quint."
                            prop:value=move || data.with(|d| d.wonders.get(idx).map(|w| w.quintessence.clone()).unwrap_or_default())
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                set_data.update(|s| {
                                    while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
                                    s.wonders[idx].quintessence = val;
                                });
                            }
                        />
                    </div>
                </div>

                <div class="wonder-desc-row">
                    <label class="wonder-label">"Descrição:"</label>
                    <textarea 
                        class="wonder-desc-textarea"
                        placeholder="Poderes, gatilhos, histórico e efeitos..."
                        prop:value=move || data.with(|d| d.wonders.get(idx).map(|w| w.description.clone()).unwrap_or_default())
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            set_data.update(|s| {
                                while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
                                s.wonders[idx].description = val;
                            });
                        }
                    ></textarea>
                </div>
            </div>
        }
    };

    view! {
        <div class="group-box magic-section-box">
            <div class="group-box-header">
                <span class="group-box-title">"MAGIA (MAGIC)"</span>
            </div>

            <div class="magic-columns-grid">
                // Coluna: Wonders (3 Cards Estáticos)
                <div class="wonders-column">
                    <div class="section-sub-title-row">
                        <span class="section-sub-title">"MARAVILHAS (WONDERS)"</span>
                    </div>

                    <div class="wonders-list">
                        {(0..3).map(render_wonder_card).collect_view()}
                    </div>
                </div>

                // Coluna: Rotes (Fórmulas & Feitiços Catalogados)
                <div class="rotes-column">
                    <div class="section-sub-title-row">
                        <span class="section-sub-title">"FÓRMULAS & FEITIÇOS (ROTES)"</span>
                    </div>

                    <div class="rotes-textarea-wrapper">
                        <textarea 
                            class="rotes-textarea"
                            placeholder="Liste aqui suas Fórmulas (Rotes) consagradas: Nome da Fórmula, Esferas necessárias, Instrumentos/Focos, Dificuldade e Efeitos..."
                            prop:value=move || data.with(|d| d.rotes.clone())
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                set_data.update(|s| s.rotes = val);
                            }
                        ></textarea>
                    </div>
                </div>
            </div>
        </div>
    }
}
