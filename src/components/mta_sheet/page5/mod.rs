use leptos::*;
use crate::components::{StableTextArea, StableTextInput};
use crate::state::{CharacterData, GrimoireRoteItem};

#[component]
pub fn PageGrimoire() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    // Paradigma Mágico
    let paradigm = Signal::derive(move || data.with(|d| d.grimoire.paradigm.clone()));
    let on_paradigm_change = Callback::new(move |val| {
        set_data.update(|s| s.grimoire.paradigm = val);
    });

    // Práticas Mágicas
    let practices_list = Signal::derive(move || {
        let list = data.with(|d| d.grimoire.practices.clone());
        if list.is_empty() {
            vec!["".to_string(), "".to_string(), "".to_string()]
        } else {
            list
        }
    });

    let add_practice = move |_| {
        set_data.update(|s| {
            s.grimoire.practices.push(String::new());
        });
    };

    let remove_practice = move |idx: usize| {
        set_data.update(|s| {
            if idx < s.grimoire.practices.len() {
                s.grimoire.practices.remove(idx);
            }
        });
    };

    let update_practice = move |idx: usize, val: String| {
        set_data.update(|s| {
            while s.grimoire.practices.len() <= idx {
                s.grimoire.practices.push(String::new());
            }
            s.grimoire.practices[idx] = val;
        });
    };

    // Instrumentos Mágicos
    let instruments_list = Signal::derive(move || {
        let list = data.with(|d| d.grimoire.instruments.clone());
        if list.is_empty() {
            vec!["".to_string(), "".to_string(), "".to_string()]
        } else {
            list
        }
    });

    let add_instrument = move |_| {
        set_data.update(|s| {
            s.grimoire.instruments.push(String::new());
        });
    };

    let remove_instrument = move |idx: usize| {
        set_data.update(|s| {
            if idx < s.grimoire.instruments.len() {
                s.grimoire.instruments.remove(idx);
            }
        });
    };

    let update_instrument = move |idx: usize, val: String| {
        set_data.update(|s| {
            while s.grimoire.instruments.len() <= idx {
                s.grimoire.instruments.push(String::new());
            }
            s.grimoire.instruments[idx] = val;
        });
    };

    // Rotinas (Rotes)
    let rotes_list = Signal::derive(move || data.with(|d| d.grimoire.rotes.clone()));

    let add_rote = move |_| {
        set_data.update(|s| {
            s.grimoire.rotes.push(GrimoireRoteItem {
                id: format!("rote_{}", uuid::Uuid::new_v4()),
                name: String::new(),
                spheres: String::new(),
                focus: String::new(),
                practice: String::new(),
                instrument: String::new(),
                description: String::new(),
            });
        });
    };

    let remove_rote = move |idx: usize| {
        set_data.update(|s| {
            if idx < s.grimoire.rotes.len() {
                s.grimoire.rotes.remove(idx);
            }
        });
    };

    let update_rote = move |idx: usize, rote: GrimoireRoteItem| {
        set_data.update(|s| {
            if idx < s.grimoire.rotes.len() {
                s.grimoire.rotes[idx] = rote;
            }
        });
    };

    // Anotações Gerais do Grimório
    let general_notes = Signal::derive(move || data.with(|d| d.grimoire.general_notes.clone()));
    let on_general_notes_change = Callback::new(move |val| {
        set_data.update(|s| s.grimoire.general_notes = val);
    });

    view! {
        <div class="sheet-page-content page-grimoire-content">
            // Box 1: Paradigma, Práticas e Instrumentos
            <div class="group-box grimoire-foundations-box">
                <span class="group-title">"MAGICKAL FOUNDATION (Paradigma, Práticas & Instrumentos)"</span>

                // Paradigma
                <div class="grimoire-paradigm-row">
                    <label class="grimoire-section-label">"PARADIGMA / FILOSOFIA MÁGICA:"</label>
                    <StableTextInput 
                        class="grimoire-paradigm-input"
                        placeholder="Ex: Tudo é Mente e Informação, A Criação é Alquimia Divina, Magia Mecânica..."
                        value=paradigm
                        on_change=on_paradigm_change
                    />
                </div>

                // Duas Colunas: Práticas & Instrumentos
                <div class="grimoire-2col-grid">
                    // Coluna de Práticas
                    <div class="grimoire-list-col">
                        <div class="grimoire-col-header">
                            <h3 class="column-title">"PRÁTICAS MÁGICAS (PRACTICES)"</h3>
                            <button 
                                type="button"
                                class="add-grimoire-item-btn" 
                                on:click=add_practice
                                title="Adicionar Prática Mágica"
                            >
                                "+ Prática"
                            </button>
                        </div>

                        <div class="grimoire-items-list">
                            {move || {
                                let list = practices_list.get();
                                list.into_iter().enumerate().map(|(idx, val)| {
                                    let val_sig = Signal::derive({
                                        let val = val.clone();
                                        move || val.clone()
                                    });
                                    view! {
                                        <div class="grimoire-list-item-row">
                                            <span class="grimoire-item-bullet">"✦"</span>
                                            <StableTextInput 
                                                class="grimoire-item-input"
                                                placeholder="Ex: Alquimia, Alta Magia Ritual, Bruxaria, Artes Marciais..."
                                                value=val_sig
                                                on_change=Callback::new(move |new_val| update_practice(idx, new_val))
                                            />
                                            <button 
                                                type="button"
                                                class="remove-grimoire-btn"
                                                on:click=move |_| remove_practice(idx)
                                                title="Remover Prática"
                                            >
                                                "×"
                                            </button>
                                        </div>
                                    }
                                }).collect_view()
                            }}
                        </div>
                    </div>

                    // Coluna de Instrumentos
                    <div class="grimoire-list-col">
                        <div class="grimoire-col-header">
                            <h3 class="column-title">"INSTRUMENTOS MÁGICOS (INSTRUMENTS)"</h3>
                            <button 
                                type="button"
                                class="add-grimoire-item-btn" 
                                on:click=add_instrument
                                title="Adicionar Instrumento"
                            >
                                "+ Instrumento"
                            </button>
                        </div>

                        <div class="grimoire-items-list">
                            {move || {
                                let list = instruments_list.get();
                                list.into_iter().enumerate().map(|(idx, val)| {
                                    let val_sig = Signal::derive({
                                        let val = val.clone();
                                        move || val.clone()
                                    });
                                    view! {
                                        <div class="grimoire-list-item-row">
                                            <span class="grimoire-item-bullet">"✧"</span>
                                            <StableTextInput 
                                                class="grimoire-item-input"
                                                placeholder="Ex: Cálice de Prata, Varinha, Sigilos, Computador, Sangue..."
                                                value=val_sig
                                                on_change=Callback::new(move |new_val| update_instrument(idx, new_val))
                                            />
                                            <button 
                                                type="button"
                                                class="remove-grimoire-btn"
                                                on:click=move |_| remove_instrument(idx)
                                                title="Remover Instrumento"
                                            >
                                                "×"
                                            </button>
                                        </div>
                                    }
                                }).collect_view()
                            }}
                        </div>
                    </div>
                </div>
            </div>

            // Box 2: Cards de Rotinas Mágicas (Rotes)
            <div class="group-box grimoire-rotes-box">
                <span class="group-title">"MAGICKAL ROTES (Rotinas Mágicas & Feitiços)"</span>

                <div class="rotes-header-actions">
                    <span class="rotes-hint">"Rotinas consagradas, efeitos característicos e fórmulas místicas do Mago"</span>
                    <button 
                        type="button"
                        class="add-rote-card-btn" 
                        on:click=add_rote
                        title="Adicionar nova Rotina Mágica ao Grimório"
                    >
                        "✨ + Adicionar Nova Rotina"
                    </button>
                </div>

                <div class="rotes-cards-grid">
                    {move || {
                        let rotes = rotes_list.get();
                        if rotes.is_empty() {
                            view! {
                                <div class="empty-rotes-banner">
                                    <span class="empty-rotes-icon">"📖"</span>
                                    <p class="empty-rotes-title">"Nenhuma rotina mágica registrada ainda."</p>
                                    <p class="empty-rotes-desc">"Clique no botão acima para adicionar suas rotinas, fórmulas e rituais ao Grimório."</p>
                                </div>
                            }.into_view()
                        } else {
                            rotes.into_iter().enumerate().map(|(idx, rote)| {
                                let rote_name = rote.name.clone();
                                let rote_spheres = rote.spheres.clone();
                                let rote_practice = rote.practice.clone();
                                let rote_focus = rote.focus.clone();
                                let rote_desc = rote.description.clone();

                                let r_name_sig = Signal::derive({
                                    let v = rote_name.clone();
                                    move || v.clone()
                                });
                                let r_spheres_sig = Signal::derive({
                                    let v = rote_spheres.clone();
                                    move || v.clone()
                                });
                                let r_practice_sig = Signal::derive({
                                    let v = rote_practice.clone();
                                    move || v.clone()
                                });
                                let r_focus_sig = Signal::derive({
                                    let v = rote_focus.clone();
                                    move || v.clone()
                                });
                                let r_desc_sig = Signal::derive({
                                    let v = rote_desc.clone();
                                    move || v.clone()
                                });

                                let rote_for_name = rote.clone();
                                let rote_for_spheres = rote.clone();
                                let rote_for_practice = rote.clone();
                                let rote_for_focus = rote.clone();
                                let rote_for_desc = rote.clone();

                                view! {
                                    <div class="rote-card">
                                        <div class="rote-card-header">
                                            <div class="rote-title-wrap">
                                                <span class="rote-number-tag">{format!("#{}", idx + 1)}</span>
                                                <StableTextInput 
                                                    class="rote-name-input"
                                                    placeholder="Nome da Rotina / Feitiço..."
                                                    value=r_name_sig
                                                    on_change=Callback::new(move |val| {
                                                        let mut r = rote_for_name.clone();
                                                        r.name = val;
                                                        update_rote(idx, r);
                                                    })
                                                />
                                            </div>
                                            <button 
                                                type="button"
                                                class="remove-rote-card-btn"
                                                on:click=move |_| remove_rote(idx)
                                                title="Remover esta Rotina"
                                            >
                                                "×"
                                            </button>
                                        </div>

                                        <div class="rote-meta-grid">
                                            <div class="rote-meta-field">
                                                <label class="rote-meta-label">"ESFERAS & NÍVEIS:"</label>
                                                <StableTextInput 
                                                    class="rote-meta-input rote-spheres-input"
                                                    placeholder="Ex: Forças 3, Primórdio 2..."
                                                    value=r_spheres_sig
                                                    on_change=Callback::new(move |val| {
                                                        let mut r = rote_for_spheres.clone();
                                                        r.spheres = val;
                                                        update_rote(idx, r);
                                                    })
                                                />
                                            </div>

                                            <div class="rote-meta-field">
                                                <label class="rote-meta-label">"PRÁTICA UTILIZADA:"</label>
                                                <StableTextInput 
                                                    class="rote-meta-input"
                                                    placeholder="Ex: Alquimia, Bruxaria, Hipertecnologia..."
                                                    value=r_practice_sig
                                                    on_change=Callback::new(move |val| {
                                                        let mut r = rote_for_practice.clone();
                                                        r.practice = val;
                                                        update_rote(idx, r);
                                                    })
                                                />
                                            </div>

                                            <div class="rote-meta-field">
                                                <label class="rote-meta-label">"FOCO & INSTRUMENTO:"</label>
                                                <StableTextInput 
                                                    class="rote-meta-input"
                                                    placeholder="Ex: Adaga e Canto, Dispositivo Eletrônico..."
                                                    value=r_focus_sig
                                                    on_change=Callback::new(move |val| {
                                                        let mut r = rote_for_focus.clone();
                                                        r.focus = val;
                                                        update_rote(idx, r);
                                                    })
                                                />
                                            </div>
                                        </div>

                                        <div class="rote-desc-wrap">
                                            <label class="rote-desc-label">"DESCRIÇÃO NARRATIVA & EFEITOS MECÂNICOS:"</label>
                                            <StableTextArea 
                                                class="rote-desc-textarea"
                                                placeholder="Descreva o procedimento mágico, narrativa visual do feitiço, paradas de dados, dificuldade, gastos de quintessência, regras de paradoxo e efeitos..."
                                                value=r_desc_sig
                                                on_change=Callback::new(move |val| {
                                                    let mut r = rote_for_desc.clone();
                                                    r.description = val;
                                                    update_rote(idx, r);
                                                })
                                            />
                                        </div>
                                    </div>
                                }
                            }).collect_view().into_view()
                        }
                    }}
                </div>
            </div>

            // Box 3: Anotações Gerais do Grimório
            <div class="group-box grimoire-notes-box">
                <span class="group-title">"GRIMOIRE NOTES & SECRETS (Anotações do Tomo Mágico)"</span>
                <StableTextArea 
                    class="grimoire-notes-textarea"
                    placeholder="Histórico do grimório, linhagem de mestres, linguagens herméticas ou enochianas, códigos arcanos e anotações adicionais..."
                    value=general_notes
                    on_change=on_general_notes_change
                />
            </div>
        </div>
    }
}
