use leptos::*;
use std::collections::HashSet;
use crate::components::{StableTextArea, StableTextInput};
use crate::state::{CharacterData, GrimoireRoteItem, RoteSphereRequirement};

#[component]
pub fn PageGrimoire() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    // Controle de cards de rotinas recolhidos/expandidos
    let (collapsed_rotes, set_collapsed_rotes) = create_signal::<HashSet<String>>(HashSet::new());

    let toggle_collapse = move |id: String| {
        set_collapsed_rotes.update(|set| {
            if set.contains(&id) {
                set.remove(&id);
            } else {
                set.insert(id);
            }
        });
    };

    // Paradigma Mágico
    let paradigm = Signal::derive(move || data.with(|d| d.grimoire.paradigm.clone()));
    let on_paradigm_change = Callback::new(move |val| {
        set_data.update(|s| s.grimoire.paradigm = val);
    });

    // Práticas Mágicas
    let practices_list = Signal::derive(move || {
        let list = data.with(|d| d.grimoire.practices.clone());
        if list.is_empty() {
            vec![String::new(), String::new(), String::new()]
        } else {
            list
        }
    });

    let add_practice = move |_| {
        set_data.update(|s| {
            if s.grimoire.practices.is_empty() {
                s.grimoire.practices = vec![String::new(), String::new(), String::new(), String::new()];
            } else {
                s.grimoire.practices.push(String::new());
            }
        });
    };

    let remove_practice = move |idx: usize| {
        set_data.update(|s| {
            if s.grimoire.practices.is_empty() {
                s.grimoire.practices = vec![String::new(), String::new(), String::new()];
            }
            if idx < s.grimoire.practices.len() {
                s.grimoire.practices.remove(idx);
            }
        });
    };

    let update_practice = move |idx: usize, val: String| {
        set_data.update(|s| {
            if s.grimoire.practices.is_empty() {
                s.grimoire.practices = vec![String::new(), String::new(), String::new()];
            }
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
            vec![String::new(), String::new(), String::new()]
        } else {
            list
        }
    });

    let add_instrument = move |_| {
        set_data.update(|s| {
            if s.grimoire.instruments.is_empty() {
                s.grimoire.instruments = vec![String::new(), String::new(), String::new(), String::new()];
            } else {
                s.grimoire.instruments.push(String::new());
            }
        });
    };

    let remove_instrument = move |idx: usize| {
        set_data.update(|s| {
            if s.grimoire.instruments.is_empty() {
                s.grimoire.instruments = vec![String::new(), String::new(), String::new()];
            }
            if idx < s.grimoire.instruments.len() {
                s.grimoire.instruments.remove(idx);
            }
        });
    };

    let update_instrument = move |idx: usize, val: String| {
        set_data.update(|s| {
            if s.grimoire.instruments.is_empty() {
                s.grimoire.instruments = vec![String::new(), String::new(), String::new()];
            }
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
                sphere_list: Vec::new(),
                highest_sphere: 0,
                enhancing_ability: String::new(),
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
                <span class="group-title">"MAGICKAL FOUNDATION"</span>

                // Banner Hero do Paradigma Central
                <div class="grimoire-paradigm-hero">
                    <div class="paradigm-header-wrap">
                        <span class="paradigm-icon">"🔮"</span>
                        <div class="paradigm-titles">
                            <label class="paradigm-main-title">"PARADIGMA CENTRAL (CRENÇA MÁGICA)"</label>
                            <span class="paradigm-sub-title">"A Filosofia e Verdade que moldam a Realidade do Mago"</span>
                        </div>
                    </div>
                    <StableTextInput 
                        class="grimoire-paradigm-input"
                        placeholder="Ex: Tudo é Mente e Informação • A Criação é Alquimia Divina • Caos e Vontade Pura..."
                        value=paradigm
                        on_change=on_paradigm_change
                    />
                </div>

                // Duas Colunas: Práticas & Instrumentos
                <div class="grimoire-2col-grid">
                    // Coluna de Práticas
                    <div class="grimoire-list-col">
                        <div class="grimoire-col-header">
                            <div class="col-title-wrap">
                                <span class="col-header-icon">"✦"</span>
                                <h3 class="column-title">"PRÁTICAS MÁGICAS"</h3>
                            </div>
                            <button 
                                type="button"
                                class="add-grimoire-pill-btn" 
                                on:click=add_practice
                                title="Adicionar nova Prática Mágica"
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
                                            <span class="grimoire-item-tag">{format!("{:02}", idx + 1)}</span>
                                            <StableTextInput 
                                                class="grimoire-item-input"
                                                placeholder="Ex: Alquimia, Alta Magia Ritual, Bruxaria, Cybernética..."
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
                            <div class="col-title-wrap">
                                <span class="col-header-icon">"✧"</span>
                                <h3 class="column-title">"INSTRUMENTOS MÁGICOS"</h3>
                            </div>
                            <button 
                                type="button"
                                class="add-grimoire-pill-btn" 
                                on:click=add_instrument
                                title="Adicionar novo Instrumento"
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
                                            <span class="grimoire-item-tag">{format!("{:02}", idx + 1)}</span>
                                            <StableTextInput 
                                                class="grimoire-item-input"
                                                placeholder="Ex: Cálice de Prata, Varinha, Sigilos, Sangue, Computador..."
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
                <span class="group-title">"MAGICKAL ROTES (ROTINAS MÁGICAS)"</span>

                <div class="rotes-header-actions">
                    <div class="rotes-header-info">
                        <span class="rotes-main-badge">"📜 Grimório de Feitiços"</span>
                        <span class="rotes-hint">"Rotinas consagradas, fórmulas arcanas e efeitos característicos do Mago"</span>
                    </div>
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
                                    <p class="empty-rotes-title">"Nenhuma rotina mágica registrada no Grimório."</p>
                                    <p class="empty-rotes-desc">"Clique no botão acima para adicionar feitiços, fórmulas e rituais."</p>
                                </div>
                            }.into_view()
                        } else {
                            rotes.into_iter().enumerate().map(|(idx, rote)| {
                                view! {
                                    <RoteCardComponent 
                                        idx=idx
                                        rote=rote
                                        collapsed_rotes=collapsed_rotes
                                        on_toggle_collapse=Callback::new(toggle_collapse)
                                        on_update_rote=Callback::new(move |updated| update_rote(idx, updated))
                                        on_remove_rote=Callback::new(move |_| remove_rote(idx))
                                    />
                                }
                            }).collect_view().into_view()
                        }
                    }}
                </div>
            </div>

            // Box 3: Anotações Gerais do Grimório
            <div class="group-box grimoire-notes-box">
                <span class="group-title">"GRIMOIRE SECRETS (ANOTAÇÕES DO TOMO)"</span>
                <StableTextArea 
                    class="grimoire-notes-textarea"
                    placeholder="Histórico do tomo, linhagem de mestres, linguagens herméticas ou enochianas, cifras secretas, senhas arcanas e anotações adicionais..."
                    value=general_notes
                    on_change=on_general_notes_change
                />
            </div>
        </div>
    }
}

#[component]
fn RoteCardComponent(
    idx: usize,
    rote: GrimoireRoteItem,
    collapsed_rotes: ReadSignal<HashSet<String>>,
    on_toggle_collapse: Callback<String>,
    on_update_rote: Callback<GrimoireRoteItem>,
    on_remove_rote: Callback<()>,
) -> impl IntoView {
    let rote_id = if rote.id.is_empty() { format!("rote_{}", idx) } else { rote.id.clone() };
    let r_id_for_collapse = rote_id.clone();
    
    let is_collapsed = Signal::derive({
        let r_id = rote_id.clone();
        move || collapsed_rotes.get().contains(&r_id)
    });

    // Estados locais para o seletor de adicionar esfera
    let (new_sphere, set_new_sphere) = create_signal("Correspondência".to_string());
    let (new_level, set_new_level) = create_signal(1);

    let rote_name = rote.name.clone();
    let rote_sphere_list = rote.sphere_list.clone();
    let rote_ability = rote.enhancing_ability.clone();
    let rote_practice = rote.practice.clone();
    let rote_focus = rote.focus.clone();
    let rote_desc = rote.description.clone();

    let r_name_sig = Signal::derive({ let v = rote_name.clone(); move || v.clone() });
    let r_sphere_list_sig = Signal::derive({ let v = rote_sphere_list.clone(); move || v.clone() });
    let r_ability_sig = Signal::derive({ let v = rote_ability.clone(); move || v.clone() });
    let r_practice_sig = Signal::derive({ let v = rote_practice.clone(); move || v.clone() });
    let r_focus_sig = Signal::derive({ let v = rote_focus.clone(); move || v.clone() });
    let r_desc_sig = Signal::derive({ let v = rote_desc.clone(); move || v.clone() });

    let r_diff_sig = Signal::derive({ let r = rote.clone(); move || r.calculate_difficulties() });
    let r_max_sphere_sig = Signal::derive({ let r = rote.clone(); move || r.get_highest_sphere_level() });

    let rote_for_name = rote.clone();
    let rote_for_spheres_add = rote.clone();
    let rote_for_spheres_rem = rote.clone();
    let rote_for_ability = rote.clone();
    let rote_for_practice = rote.clone();
    let rote_for_focus = rote.clone();
    let rote_for_desc = rote.clone();

    let add_sphere_tag = {
        let on_update = on_update_rote.clone();
        move |_| {
            let mut r = rote_for_spheres_add.clone();
            let s_name = new_sphere.get();
            let s_lvl = new_level.get();
            if let Some(existing) = r.sphere_list.iter_mut().find(|s| s.sphere == s_name) {
                existing.level = s_lvl;
            } else {
                r.sphere_list.push(RoteSphereRequirement {
                    sphere: s_name,
                    level: s_lvl,
                });
            }
            r.sync_spheres_string();
            on_update.call(r);
        }
    };

    let remove_sphere_tag = {
        let on_update = on_update_rote.clone();
        move |s_idx: usize| {
            let mut r = rote_for_spheres_rem.clone();
            if s_idx < r.sphere_list.len() {
                r.sphere_list.remove(s_idx);
                r.sync_spheres_string();
                on_update.call(r);
            }
        }
    };

    view! {
        <div class="rote-card" class:collapsed=is_collapsed>
            <div class="rote-card-header">
                <div class="rote-title-wrap">
                    <button 
                        type="button" 
                        class="rote-collapse-btn"
                        on:click=move |_| on_toggle_collapse.call(r_id_for_collapse.clone())
                        title=move || if is_collapsed.get() { "Expandir Rotina" } else { "Recolher Rotina" }
                    >
                        {move || if is_collapsed.get() { "▶" } else { "▼" }}
                    </button>
                    <span class="rote-number-tag">{format!("Rote #{:02}", idx + 1)}</span>
                    <StableTextInput 
                        class="rote-name-input"
                        placeholder="Nome da Rotina / Feitiço..."
                        value=r_name_sig
                        on_change=Callback::new({
                            let on_update = on_update_rote.clone();
                            move |val| {
                                let mut r = rote_for_name.clone();
                                r.name = val;
                                on_update.call(r);
                            }
                        })
                    />
                </div>

                <div class="rote-header-actions-right">
                    <button 
                        type="button"
                        class="remove-rote-card-btn"
                        on:click=move |_| on_remove_rote.call(())
                        title="Remover esta Rotina"
                    >
                        "×"
                    </button>
                </div>
            </div>

            // Preview resumido quando recolhido
            <div class="rote-collapsed-preview" class:tab-hidden=move || !is_collapsed.get()>
                {move || {
                    let sphere_list = r_sphere_list_sig.get();
                    let a_txt = r_ability_sig.get();
                    let p_txt = r_practice_sig.get();
                    let f_txt = r_focus_sig.get();
                    let (c_diff, v_diff, vw_diff) = r_diff_sig.get();
                    view! {
                        <div class="preview-pills-row">
                            {sphere_list.into_iter().map(|s| {
                                view! {
                                    <span class="preview-pill preview-spheres">
                                        {format!("🔮 {} {}", s.sphere, s.level)}
                                    </span>
                                }
                            }).collect_view()}
                            {if !a_txt.is_empty() {
                                view! { <span class="preview-pill preview-ability">{format!("⭐ {}", a_txt)}</span> }.into_view()
                            } else {
                                view! {}.into_view()
                            }}
                            {if !p_txt.is_empty() {
                                view! { <span class="preview-pill preview-practice">{format!("⚡ {}", p_txt)}</span> }.into_view()
                            } else {
                                view! {}.into_view()
                            }}
                            {if !f_txt.is_empty() {
                                view! { <span class="preview-pill preview-focus">{format!("🎯 {}", f_txt)}</span> }.into_view()
                            } else {
                                view! {}.into_view()
                            }}
                            <span class="preview-pill preview-diff-tag" title="Dificuldades M20: Coincidente / Vulgar / Vulgar com Testemunha">
                                {format!("🎯 Dif: {} / {} / {}", c_diff, v_diff, vw_diff)}
                            </span>
                        </div>
                    }
                }}
            </div>

            // Corpo Expandido do Card
            <div class="rote-card-body" class:tab-hidden=is_collapsed>
                // Matriz de Cálculo de Dificuldade M20
                <div class="rote-diff-matrix">
                    <span class="diff-matrix-title">"DIFICULDADE MÁGICA (M20):"</span>
                    <div class="diff-pills-wrap">
                        <span class="diff-pill diff-coincidental" title="Magia Coincidente: Maior Esfera + 2">
                            <span class="diff-icon">"🟢"</span>
                            <span class="diff-type">"Coincidente:"</span>
                            <strong class="diff-value">{move || format!("Dif {}", r_diff_sig.get().0)}</strong>
                            <small class="diff-calc">{move || format!("({}+2)", r_max_sphere_sig.get())}</small>
                        </span>

                        <span class="diff-pill diff-vulgar" title="Magia Vulgar (Sem Testemunhas): Maior Esfera + 3">
                            <span class="diff-icon">"🟡"</span>
                            <span class="diff-type">"Vulgar:"</span>
                            <strong class="diff-value">{move || format!("Dif {}", r_diff_sig.get().1)}</strong>
                            <small class="diff-calc">{move || format!("({}+3)", r_max_sphere_sig.get())}</small>
                        </span>

                        <span class="diff-pill diff-witness" title="Magia Vulgar com Testemunha Adormecida: Maior Esfera + 4">
                            <span class="diff-icon">"🔴"</span>
                            <span class="diff-type">"Vulgar c/ Testemunha:"</span>
                            <strong class="diff-value">{move || format!("Dif {}", r_diff_sig.get().2)}</strong>
                            <small class="diff-calc">{move || format!("({}+4)", r_max_sphere_sig.get())}</small>
                        </span>
                    </div>
                </div>

                // Layout Organizado em 2 Colunas Limpas (2x2)
                <div class="rote-meta-2col-grid">
                    // Coluna 1: Esferas & Prática
                    <div class="rote-meta-col">
                        // Bloco de Esferas com Seletor e Tags
                        <div class="rote-spheres-block">
                            <label class="rote-meta-label">
                                <span class="meta-icon">"🔮"</span> "ESFERAS UTILIZADAS:"
                            </label>

                            // Container das Tags de Esferas
                            <div class="rote-sphere-tags-container">
                                {move || {
                                    let list = r_sphere_list_sig.get();
                                    if list.is_empty() {
                                        view! {
                                            <span class="no-spheres-hint">"Nenhuma esfera selecionada (escolha e adicione abaixo)"</span>
                                        }.into_view()
                                    } else {
                                        list.into_iter().enumerate().map(|(s_idx, s_req)| {
                                            let s_req_name = s_req.sphere.clone();
                                            let remove_action = remove_sphere_tag.clone();
                                            view! {
                                                <span class="rote-sphere-pill-tag">
                                                    <span class="sphere-pill-icon">"🔮"</span>
                                                    <span class="sphere-pill-name">{s_req.sphere.clone()}</span>
                                                    <strong class="sphere-pill-lvl">{format!("{}", s_req.level)}</strong>
                                                    <button 
                                                        type="button" 
                                                        class="sphere-pill-remove-btn"
                                                        on:click=move |_| remove_action(s_idx)
                                                        title=format!("Remover esfera {}", s_req_name)
                                                    >
                                                        "×"
                                                    </button>
                                                </span>
                                            }
                                        }).collect_view().into_view()
                                    }
                                }}
                            </div>

                            // Controles para Adicionar Esfera e Nível
                            <div class="add-sphere-inline-control">
                                <select 
                                    class="sphere-select-dropdown"
                                    on:change=move |ev| set_new_sphere.set(event_target_value(&ev))
                                    prop:value=new_sphere
                                >
                                    <option value="Correspondência">"Correspondência"</option>
                                    <option value="Entropia">"Entropia"</option>
                                    <option value="Espírito">"Espírito"</option>
                                    <option value="Forças">"Forças"</option>
                                    <option value="Matéria">"Matéria"</option>
                                    <option value="Mente">"Mente"</option>
                                    <option value="Primórdio">"Primórdio"</option>
                                    <option value="Tempo">"Tempo"</option>
                                    <option value="Vida">"Vida"</option>
                                </select>

                                <select 
                                    class="sphere-level-dropdown"
                                    on:change=move |ev| {
                                        if let Ok(lvl) = event_target_value(&ev).parse::<i32>() {
                                            set_new_level.set(lvl);
                                        }
                                    }
                                    prop:value=move || new_level.get().to_string()
                                >
                                    <option value="1">"Nível 1"</option>
                                    <option value="2">"Nível 2"</option>
                                    <option value="3">"Nível 3"</option>
                                    <option value="4">"Nível 4"</option>
                                    <option value="5">"Nível 5"</option>
                                </select>

                                <button 
                                    type="button" 
                                    class="btn-add-sphere-tag"
                                    on:click=add_sphere_tag
                                    title="Adicionar Esfera à Rotina"
                                >
                                    "+ Adicionar"
                                </button>
                            </div>
                        </div>

                        // Prática Utilizada
                        <div class="rote-meta-field">
                            <label class="rote-meta-label">
                                <span class="meta-icon">"⚡"</span> "PRÁTICA UTILIZADA:"
                            </label>
                            <StableTextInput 
                                class="rote-meta-input"
                                placeholder="Ex: Alta Magia Ritualística, Alquimia, Bruxaria..."
                                value=r_practice_sig
                                on_change=Callback::new({
                                    let on_update = on_update_rote.clone();
                                    move |val| {
                                        let mut r = rote_for_practice.clone();
                                        r.practice = val;
                                        on_update.call(r);
                                    }
                                })
                            />
                        </div>
                    </div>

                    // Coluna 2: Habilidade & Foco
                    <div class="rote-meta-col">
                        // Habilidade Realçando Mágica
                        <div class="rote-meta-field">
                            <label class="rote-meta-label">
                                <span class="meta-icon">"⭐"</span> "HABILIDADE REALÇANDO MÁGICA:"
                            </label>
                            <StableTextInput 
                                class="rote-meta-input"
                                placeholder="Ex: Esotérica (Geometria Sagrada), Ocultismo, Ciência..."
                                value=r_ability_sig
                                on_change=Callback::new({
                                    let on_update = on_update_rote.clone();
                                    move |val| {
                                        let mut r = rote_for_ability.clone();
                                        r.enhancing_ability = val;
                                        on_update.call(r);
                                    }
                                })
                            />
                        </div>

                        // Foco & Instrumento
                        <div class="rote-meta-field">
                            <label class="rote-meta-label">
                                <span class="meta-icon">"🎯"</span> "FOCO & INSTRUMENTO UTILIZADO:"
                            </label>
                            <StableTextInput 
                                class="rote-meta-input"
                                placeholder="Ex: Círculos mágicos, Adaga de prata, Canto..."
                                value=r_focus_sig
                                on_change=Callback::new({
                                    let on_update = on_update_rote.clone();
                                    move |val| {
                                        let mut r = rote_for_focus.clone();
                                        r.focus = val;
                                        on_update.call(r);
                                    }
                                })
                            />
                        </div>
                    </div>
                </div>

                // Descrição Narrativa & Mecânica
                <div class="rote-desc-wrap">
                    <label class="rote-desc-label">
                        <span class="desc-icon">"📜"</span> "DESCRIÇÃO NARRATIVA & EFEITOS MECÂNICOS:"
                    </label>
                    <StableTextArea 
                        class="rote-desc-textarea"
                        placeholder="Descreva o procedimento mágico, narrativa visual do feitiço, paradas de dados, dificuldade, gastos de quintessência, regras de paradoxo e efeitos..."
                        value=r_desc_sig
                        on_change=Callback::new({
                            let on_update = on_update_rote.clone();
                            move |val| {
                                let mut r = rote_for_desc.clone();
                                r.description = val;
                                on_update.call(r);
                            }
                        })
                    />
                </div>
            </div>
        </div>
    }
}
