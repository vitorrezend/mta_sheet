use leptos::*;
use crate::state::{CharacterData, WeaponItem};

#[component]
pub fn CombatSection() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let weapons = Signal::derive(move || data.with(|d| d.weapons.clone()));
    let armor = Signal::derive(move || data.with(|d| d.armor.clone()));

    let add_weapon = move |_| {
        set_data.update(|s| {
            s.weapons.push(WeaponItem::default());
        });
    };

    view! {
        <div class="group-box combat-section-box">
            <div class="group-box-header">
                <span class="group-box-title">"COMBATE (COMBAT)"</span>
            </div>

            <div class="combat-grid">
                // Tabela de Armas e Ataques
                <div class="weapons-table-column">
                    <div class="weapons-table-header-row">
                        <span class="weapons-table-title">"ARMAS & ATAQUES"</span>
                        <button type="button" class="add-mini-btn" on:click=add_weapon title="Adicionar arma">
                            "+ Arma"
                        </button>
                    </div>

                    <div class="weapons-table-container">
                        <table class="weapons-table">
                            <thead>
                                <tr>
                                    <th class="th-weapon">"ARMA / ATAQUE"</th>
                                    <th class="th-stat">"DIF."</th>
                                    <th class="th-stat">"DANO"</th>
                                    <th class="th-stat">"ALCANCE"</th>
                                    <th class="th-stat">"CADÊNCIA"</th>
                                    <th class="th-stat">"PENTE"</th>
                                    <th class="th-stat">"OCULT."</th>
                                    <th class="th-act"></th>
                                </tr>
                            </thead>
                            <tbody>
                                {move || {
                                    weapons.get().into_iter().enumerate().map(|(idx, wp)| {
                                        let w_name = wp.name.clone();
                                        let w_diff = wp.diff.clone();
                                        let w_damage = wp.damage.clone();
                                        let w_range = wp.range.clone();
                                        let w_rate = wp.rate.clone();
                                        let w_clip = wp.clip.clone();
                                        let w_conceal = wp.conceal.clone();

                                        view! {
                                            <tr>
                                                <td>
                                                    <input 
                                                        type="text" 
                                                        class="table-cell-input text-left font-bold"
                                                        placeholder="Arma / Golpe..."
                                                        prop:value=w_name
                                                        on:input=move |ev| {
                                                            let val = event_target_value(&ev);
                                                            set_data.update(|s| {
                                                                if let Some(w) = s.weapons.get_mut(idx) {
                                                                    w.name = val;
                                                                }
                                                            });
                                                        }
                                                    />
                                                </td>
                                                <td>
                                                    <input 
                                                        type="text" 
                                                        class="table-cell-input text-center"
                                                        placeholder="6"
                                                        prop:value=w_diff
                                                        on:input=move |ev| {
                                                            let val = event_target_value(&ev);
                                                            set_data.update(|s| {
                                                                if let Some(w) = s.weapons.get_mut(idx) {
                                                                    w.diff = val;
                                                                }
                                                            });
                                                        }
                                                    />
                                                </td>
                                                <td>
                                                    <input 
                                                        type="text" 
                                                        class="table-cell-input text-center"
                                                        placeholder="For+2L"
                                                        prop:value=w_damage
                                                        on:input=move |ev| {
                                                            let val = event_target_value(&ev);
                                                            set_data.update(|s| {
                                                                if let Some(w) = s.weapons.get_mut(idx) {
                                                                    w.damage = val;
                                                                }
                                                            });
                                                        }
                                                    />
                                                </td>
                                                <td>
                                                    <input 
                                                        type="text" 
                                                        class="table-cell-input text-center"
                                                        placeholder="-"
                                                        prop:value=w_range
                                                        on:input=move |ev| {
                                                            let val = event_target_value(&ev);
                                                            set_data.update(|s| {
                                                                if let Some(w) = s.weapons.get_mut(idx) {
                                                                    w.range = val;
                                                                }
                                                            });
                                                        }
                                                    />
                                                </td>
                                                <td>
                                                    <input 
                                                        type="text" 
                                                        class="table-cell-input text-center"
                                                        placeholder="1"
                                                        prop:value=w_rate
                                                        on:input=move |ev| {
                                                            let val = event_target_value(&ev);
                                                            set_data.update(|s| {
                                                                if let Some(w) = s.weapons.get_mut(idx) {
                                                                    w.rate = val;
                                                                }
                                                            });
                                                        }
                                                    />
                                                </td>
                                                <td>
                                                    <input 
                                                        type="text" 
                                                        class="table-cell-input text-center"
                                                        placeholder="-"
                                                        prop:value=w_clip
                                                        on:input=move |ev| {
                                                            let val = event_target_value(&ev);
                                                            set_data.update(|s| {
                                                                if let Some(w) = s.weapons.get_mut(idx) {
                                                                    w.clip = val;
                                                                }
                                                            });
                                                        }
                                                    />
                                                </td>
                                                <td>
                                                    <input 
                                                        type="text" 
                                                        class="table-cell-input text-center"
                                                        placeholder="P"
                                                        prop:value=w_conceal
                                                        on:input=move |ev| {
                                                            let val = event_target_value(&ev);
                                                            set_data.update(|s| {
                                                                if let Some(w) = s.weapons.get_mut(idx) {
                                                                    w.conceal = val;
                                                                }
                                                            });
                                                        }
                                                    />
                                                </td>
                                                <td class="text-center">
                                                    {if idx >= 4 {
                                                        view! {
                                                            <button 
                                                                type="button" 
                                                                class="remove-row-btn"
                                                                title="Remover arma"
                                                                on:click=move |_| {
                                                                    set_data.update(|s| {
                                                                        if idx < s.weapons.len() {
                                                                            s.weapons.remove(idx);
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
                                                </td>
                                            </tr>
                                        }
                                    }).collect_view()
                                }}
                            </tbody>
                        </table>
                    </div>
                </div>

                // Bloco de Armadura
                <div class="armor-column">
                    <div class="armor-header">
                        <span class="weapons-table-title">"ARMADURA (ARMOR)"</span>
                    </div>

                    <div class="armor-box-content">
                        <div class="armor-field-row">
                            <label class="armor-label">"Classe:"</label>
                            <input 
                                type="text" 
                                class="armor-input"
                                placeholder="Ex: Jaqueta de Couro"
                                prop:value=move || armor.get().class_name
                                on:input=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_data.update(|s| s.armor.class_name = val);
                                }
                            />
                        </div>

                        <div class="armor-field-row">
                            <label class="armor-label">"Classificação (Rating):"</label>
                            <input 
                                type="text" 
                                class="armor-input text-center"
                                placeholder="1"
                                prop:value=move || armor.get().rating
                                on:input=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_data.update(|s| s.armor.rating = val);
                                }
                            />
                        </div>

                        <div class="armor-field-row">
                            <label class="armor-label">"Penalidade:"</label>
                            <input 
                                type="text" 
                                class="armor-input text-center"
                                placeholder="0"
                                prop:value=move || armor.get().penalty
                                on:input=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_data.update(|s| s.armor.penalty = val);
                                }
                            />
                        </div>

                        <div class="armor-desc-row">
                            <label class="armor-label">"Descrição:"</label>
                            <textarea 
                                class="armor-desc-textarea"
                                placeholder="Detalhes da proteção..."
                                prop:value=move || armor.get().description
                                on:input=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_data.update(|s| s.armor.description = val);
                                }
                            ></textarea>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
