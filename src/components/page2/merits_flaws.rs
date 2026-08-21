use leptos::*;
use crate::state::{CharacterData, MeritItem, FlawItem};

#[component]
pub fn MeritsFlaws() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let merits = Signal::derive(move || data.with(|d| d.merits.clone()));
    let flaws = Signal::derive(move || data.with(|d| d.flaws.clone()));

    let add_merit = move |_| {
        set_data.update(|s| {
            s.merits.push(MeritItem {
                name: String::new(),
                merit_type: "Geral".to_string(),
                cost: 1,
            });
        });
    };

    let add_flaw = move |_| {
        set_data.update(|s| {
            s.flaws.push(FlawItem {
                name: String::new(),
                flaw_type: "Geral".to_string(),
                bonus: 1,
            });
        });
    };

    view! {
        <div class="group-box merits-flaws-box">
            <div class="group-box-header">
                <span class="group-box-title">"QUALIDADES & DEFEITOS"</span>
            </div>

            <div class="merits-flaws-grid">
                // Coluna: Qualidades (Merits)
                <div class="merits-column">
                    <div class="sub-table-header">
                        <span class="col-name font-bold">"QUALIDADE"</span>
                        <span class="col-type font-bold text-center">"TIPO"</span>
                        <span class="col-cost font-bold text-center">"CUSTO"</span>
                        <span class="col-action"></span>
                    </div>

                    <div class="merits-list">
                        {move || {
                            let current_merits = merits.get();
                            if current_merits.is_empty() {
                                view! {
                                    <div class="empty-list-placeholder">
                                        <span>"Nenhuma qualidade adicionada."</span>
                                    </div>
                                }.into_view()
                            } else {
                                current_merits.into_iter().enumerate().map(|(idx, item)| {
                                    let item_name = item.name.clone();
                                    let item_type = item.merit_type.clone();
                                    let item_cost = item.cost;

                                    view! {
                                        <div class="merit-row">
                                            <input 
                                                type="text" 
                                                class="merit-input name-input"
                                                placeholder="Nome da qualidade..."
                                                prop:value=item_name
                                                on:input=move |ev| {
                                                    let val = event_target_value(&ev);
                                                    set_data.update(|s| {
                                                        if let Some(m) = s.merits.get_mut(idx) {
                                                            m.name = val;
                                                        }
                                                    });
                                                }
                                            />
                                            <input 
                                                type="text" 
                                                class="merit-input type-input text-center"
                                                placeholder="Fís/Soc/Mnt"
                                                prop:value=item_type
                                                on:input=move |ev| {
                                                    let val = event_target_value(&ev);
                                                    set_data.update(|s| {
                                                        if let Some(m) = s.merits.get_mut(idx) {
                                                            m.merit_type = val;
                                                        }
                                                    });
                                                }
                                            />
                                            <input 
                                                type="number" 
                                                min="1"
                                                max="10"
                                                class="merit-input cost-input text-center"
                                                prop:value=item_cost.to_string()
                                                on:input=move |ev| {
                                                    let val = event_target_value(&ev).parse::<i32>().unwrap_or(1);
                                                    set_data.update(|s| {
                                                        if let Some(m) = s.merits.get_mut(idx) {
                                                            m.cost = val;
                                                        }
                                                    });
                                                }
                                            />
                                            <button 
                                                type="button" 
                                                class="remove-row-btn" 
                                                title="Remover qualidade"
                                                on:click=move |_| {
                                                    set_data.update(|s| {
                                                        if idx < s.merits.len() {
                                                            s.merits.remove(idx);
                                                        }
                                                    });
                                                }
                                            >
                                                "✕"
                                            </button>
                                        </div>
                                    }
                                }).collect_view().into_view()
                            }
                        }}
                    </div>

                    <button type="button" class="add-item-btn" on:click=add_merit>
                        "+ Adicionar Qualidade"
                    </button>
                </div>

                // Coluna: Defeitos (Flaws)
                <div class="flaws-column">
                    <div class="sub-table-header">
                        <span class="col-name font-bold">"DEFEITO"</span>
                        <span class="col-type font-bold text-center">"TIPO"</span>
                        <span class="col-cost font-bold text-center">"BÔNUS"</span>
                        <span class="col-action"></span>
                    </div>

                    <div class="flaws-list">
                        {move || {
                            let current_flaws = flaws.get();
                            if current_flaws.is_empty() {
                                view! {
                                    <div class="empty-list-placeholder">
                                        <span>"Nenhum defeito adicionado."</span>
                                    </div>
                                }.into_view()
                            } else {
                                current_flaws.into_iter().enumerate().map(|(idx, item)| {
                                    let item_name = item.name.clone();
                                    let item_type = item.flaw_type.clone();
                                    let item_bonus = item.bonus;

                                    view! {
                                        <div class="flaw-row">
                                            <input 
                                                type="text" 
                                                class="merit-input name-input"
                                                placeholder="Nome do defeito..."
                                                prop:value=item_name
                                                on:input=move |ev| {
                                                    let val = event_target_value(&ev);
                                                    set_data.update(|s| {
                                                        if let Some(f) = s.flaws.get_mut(idx) {
                                                            f.name = val;
                                                        }
                                                    });
                                                }
                                            />
                                            <input 
                                                type="text" 
                                                class="merit-input type-input text-center"
                                                placeholder="Fís/Soc/Mnt"
                                                prop:value=item_type
                                                on:input=move |ev| {
                                                    let val = event_target_value(&ev);
                                                    set_data.update(|s| {
                                                        if let Some(f) = s.flaws.get_mut(idx) {
                                                            f.flaw_type = val;
                                                        }
                                                    });
                                                }
                                            />
                                            <input 
                                                type="number" 
                                                min="1"
                                                max="10"
                                                class="merit-input cost-input text-center"
                                                prop:value=item_bonus.to_string()
                                                on:input=move |ev| {
                                                    let val = event_target_value(&ev).parse::<i32>().unwrap_or(1);
                                                    set_data.update(|s| {
                                                        if let Some(f) = s.flaws.get_mut(idx) {
                                                            f.bonus = val;
                                                        }
                                                    });
                                                }
                                            />
                                            <button 
                                                type="button" 
                                                class="remove-row-btn" 
                                                title="Remover defeito"
                                                on:click=move |_| {
                                                    set_data.update(|s| {
                                                        if idx < s.flaws.len() {
                                                            s.flaws.remove(idx);
                                                        }
                                                    });
                                                }
                                            >
                                                "✕"
                                            </button>
                                        </div>
                                    }
                                }).collect_view().into_view()
                            }
                        }}
                    </div>

                    <button type="button" class="add-item-btn" on:click=add_flaw>
                        "+ Adicionar Defeito"
                    </button>
                </div>
            </div>
        </div>
    }
}
