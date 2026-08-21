use leptos::*;
use crate::state::{CharacterData, WonderItem};

#[component]
pub fn MagicSection() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let wonders = Signal::derive(move || data.with(|d| d.wonders.clone()));
    let rotes = Signal::derive(move || data.with(|d| d.rotes.clone()));

    let add_wonder = move |_| {
        set_data.update(|s| {
            s.wonders.push(WonderItem::default());
        });
    };

    view! {
        <div class="group-box magic-section-box">
            <div class="group-box-header">
                <span class="group-box-title">"MAGIA (MAGIC)"</span>
            </div>

            <div class="magic-columns-grid">
                // Coluna: Wonders (Maravilhas / Talismãs)
                <div class="wonders-column">
                    <div class="section-sub-title-row">
                        <span class="section-sub-title">"MARAVILHAS (WONDERS)"</span>
                        <button type="button" class="add-mini-btn" on:click=add_wonder title="Adicionar maravilha">
                            "+ Maravilha"
                        </button>
                    </div>

                    <div class="wonders-list">
                        {move || {
                            wonders.get().into_iter().enumerate().map(|(idx, wonder)| {
                                let w_name = wonder.name.clone();
                                let w_pts = wonder.points.clone();
                                let w_arete = wonder.arete.clone();
                                let w_quint = wonder.quintessence.clone();
                                let w_desc = wonder.description.clone();

                                view! {
                                    <div class="wonder-card">
                                        <div class="wonder-card-top-row">
                                            <div class="wonder-field name-field">
                                                <label class="wonder-label">"Nome:"</label>
                                                <input 
                                                    type="text" 
                                                    class="wonder-input"
                                                    placeholder="Nome do artefato..."
                                                    prop:value=w_name
                                                    on:input=move |ev| {
                                                        let val = event_target_value(&ev);
                                                        set_data.update(|s| {
                                                            if let Some(w) = s.wonders.get_mut(idx) {
                                                                w.name = val;
                                                            }
                                                        });
                                                    }
                                                />
                                            </div>
                                            {if idx >= 3 {
                                                view! {
                                                    <button 
                                                        type="button" 
                                                        class="remove-row-btn"
                                                        title="Remover maravilha"
                                                        on:click=move |_| {
                                                            set_data.update(|s| {
                                                                if idx < s.wonders.len() {
                                                                    s.wonders.remove(idx);
                                                                }
                                                            });
                                                        }
                                                    >
                                                        "✕"
                                                    </button>
                                                }.into_view()
                                            } else {
                                                view! { <span></span> }.into_view()
                                            }}
                                        </div>

                                        <div class="wonder-stats-row">
                                            <div class="wonder-field stat-field">
                                                <label class="wonder-label">"Pontos:"</label>
                                                <input 
                                                    type="text" 
                                                    class="wonder-input text-center"
                                                    placeholder="Pts"
                                                    prop:value=w_pts
                                                    on:input=move |ev| {
                                                        let val = event_target_value(&ev);
                                                        set_data.update(|s| {
                                                            if let Some(w) = s.wonders.get_mut(idx) {
                                                                w.points = val;
                                                            }
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
                                                    prop:value=w_arete
                                                    on:input=move |ev| {
                                                        let val = event_target_value(&ev);
                                                        set_data.update(|s| {
                                                            if let Some(w) = s.wonders.get_mut(idx) {
                                                                w.arete = val;
                                                            }
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
                                                    prop:value=w_quint
                                                    on:input=move |ev| {
                                                        let val = event_target_value(&ev);
                                                        set_data.update(|s| {
                                                            if let Some(w) = s.wonders.get_mut(idx) {
                                                                w.quintessence = val;
                                                            }
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
                                                prop:value=w_desc
                                                on:input=move |ev| {
                                                    let val = event_target_value(&ev);
                                                    set_data.update(|s| {
                                                        if let Some(w) = s.wonders.get_mut(idx) {
                                                            w.description = val;
                                                        }
                                                    });
                                                }
                                            ></textarea>
                                        </div>
                                    </div>
                                }
                            }).collect_view()
                        }}
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
                            prop:value=move || rotes.get()
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
