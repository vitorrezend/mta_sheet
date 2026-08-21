use leptos::*;
use crate::components::ValueField;
use crate::state::{CharacterData, DotOrigin, WonderItem};
use crate::components::character_sheet::ActiveDotOriginContext;

#[component]
pub fn MagicSection() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");
    let active_origin_ctx = use_context::<ActiveDotOriginContext>();

    let add_wonder = move |_| {
        set_data.update(|s| {
            s.wonders.push(WonderItem::default());
        });
    };

    let remove_wonder = move |idx: usize| {
        set_data.update(|s| {
            if s.wonders.len() > 1 {
                s.wonders.remove(idx);
            } else {
                s.wonders[0] = WonderItem::default();
            }
        });
    };

    let update_wonder_points = move |idx: usize, new_lvl: i32, origin: DotOrigin| {
        set_data.update(|s| {
            while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
            let cur = s.wonders[idx].points.level;
            let final_lvl = if new_lvl == cur { (new_lvl - 1).max(0) } else { new_lvl };
            s.wonders[idx].points.set_level_with_origin(final_lvl, origin);
        });
    };

    let update_wonder_points_mod = move |idx: usize, m: String| {
        set_data.update(|s| {
            while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
            s.wonders[idx].points.modifier = m;
        });
    };

    let update_wonder_points_dot_origin = move |idx: usize, dot_idx: usize, origin: DotOrigin| {
        set_data.update(|s| {
            while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
            s.wonders[idx].points.set_dot_origin(dot_idx, origin);
        });
    };

    let update_wonder_arete = move |idx: usize, new_lvl: i32, origin: DotOrigin| {
        set_data.update(|s| {
            while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
            let cur = s.wonders[idx].arete.level;
            let final_lvl = if new_lvl == cur { (new_lvl - 1).max(0) } else { new_lvl };
            s.wonders[idx].arete.set_level_with_origin(final_lvl, origin);
        });
    };

    let update_wonder_arete_mod = move |idx: usize, m: String| {
        set_data.update(|s| {
            while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
            s.wonders[idx].arete.modifier = m;
        });
    };

    let update_wonder_arete_dot_origin = move |idx: usize, dot_idx: usize, origin: DotOrigin| {
        set_data.update(|s| {
            while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
            s.wonders[idx].arete.set_dot_origin(dot_idx, origin);
        });
    };

    let update_wonder_quint_current = move |idx: usize, sq_idx: i32| {
        set_data.update(|s| {
            while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
            let cur = s.wonders[idx].quintessence_current;
            s.wonders[idx].quintessence_current = if sq_idx == cur { sq_idx - 1 } else { sq_idx };
        });
    };

    let change_wonder_quint_max = move |idx: usize, delta: i32| {
        set_data.update(|s| {
            while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
            let new_max = (s.wonders[idx].quintessence_max + delta).clamp(5, 20);
            s.wonders[idx].quintessence_max = new_max;
            s.wonders[idx].quintessence_current = s.wonders[idx].quintessence_current.clamp(0, new_max);
        });
    };

    let render_wonder_card = move |idx: usize| {
        let points_level = Signal::derive(move || {
            data.with(|d| d.wonders.get(idx).map(|w| w.points.level).unwrap_or(0))
        });
        let points_mod = Signal::derive(move || {
            data.with(|d| d.wonders.get(idx).map(|w| w.points.modifier.clone()).unwrap_or_default())
        });
        let points_origins = Signal::derive(move || {
            data.with(|d| d.wonders.get(idx).map(|w| w.points.get_origins(5)).unwrap_or_else(|| vec![DotOrigin::Base; 5]))
        });

        let arete_level = Signal::derive(move || {
            data.with(|d| d.wonders.get(idx).map(|w| w.arete.level).unwrap_or(0))
        });
        let arete_mod = Signal::derive(move || {
            data.with(|d| d.wonders.get(idx).map(|w| w.arete.modifier.clone()).unwrap_or_default())
        });
        let arete_origins = Signal::derive(move || {
            data.with(|d| d.wonders.get(idx).map(|w| w.arete.get_origins(5)).unwrap_or_else(|| vec![DotOrigin::Base; 5]))
        });

        view! {
            <div class="wonder-card">
                <div class="wonder-card-top-row">
                    <div class="wonder-field name-field">
                        <input 
                            type="text" 
                            class="wonder-input font-bold"
                            placeholder="Nome da Maravilha / Artefato..."
                            prop:value=move || data.with(|d| d.wonders.get(idx).map(|w| w.name.clone()).unwrap_or_default())
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_data.update(|s| {
                                    while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
                                    s.wonders[idx].name = val;
                                });
                            }
                            on:blur=move |ev| {
                                let val = event_target_value(&ev);
                                set_data.update(|s| {
                                    while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
                                    s.wonders[idx].name = val;
                                });
                            }
                        />
                    </div>
                    <button 
                        type="button" 
                        class="wonder-remove-btn" 
                        on:click=move |_| remove_wonder(idx)
                        title="Remover Maravilha"
                    >
                        "🗑️"
                    </button>
                </div>

                // Linha de Pontos e Arete utilizando ValueField padronizado
                <div class="wonder-fields-block">
                    <ValueField 
                        label=Signal::derive(move || "Pontos".to_string())
                        level=points_level
                        modifier=points_mod
                        origins=points_origins
                        max_level=5
                        min_level=0
                        on_level_change=move |v| {
                            let current_origin = active_origin_ctx.map(|a| a.origin.get()).unwrap_or(DotOrigin::Base);
                            update_wonder_points(idx, v, current_origin);
                        }
                        on_modifier_change=move |m| update_wonder_points_mod(idx, m)
                        on_dot_origin_change=Callback::new(move |(dot_i, orig)| update_wonder_points_dot_origin(idx, dot_i, orig))
                        is_editable=false
                    />

                    <ValueField 
                        label=Signal::derive(move || "Arete".to_string())
                        level=arete_level
                        modifier=arete_mod
                        origins=arete_origins
                        max_level=5
                        min_level=0
                        on_level_change=move |v| {
                            let current_origin = active_origin_ctx.map(|a| a.origin.get()).unwrap_or(DotOrigin::Base);
                            update_wonder_arete(idx, v, current_origin);
                        }
                        on_modifier_change=move |m| update_wonder_arete_mod(idx, m)
                        on_dot_origin_change=Callback::new(move |(dot_i, orig)| update_wonder_arete_dot_origin(idx, dot_i, orig))
                        is_editable=false
                    />
                </div>

                // Trilha de Quintessência com Quadradinhos (Flexível de 5 a 20)
                <div class="wonder-quintessence-row">
                    <div class="wonder-quint-header">
                        <span class="wonder-stat-label">"Quintessência:"</span>
                        <div class="wonder-quint-controls">
                            <button 
                                type="button" 
                                class="quint-step-btn" 
                                on:click=move |_| change_wonder_quint_max(idx, -5)
                                title="Reduzir capacidade de Quintessência (-5)"
                                disabled=move || data.with(|d| d.wonders.get(idx).map(|w| w.quintessence_max <= 5).unwrap_or(true))
                            >
                                "-5"
                            </button>
                            <span class="quint-max-badge">
                                {move || data.with(|d| d.wonders.get(idx).map(|w| format!("{}/{}", w.quintessence_current, w.quintessence_max)).unwrap_or_else(|| "0/5".to_string()))}
                            </span>
                            <button 
                                type="button" 
                                class="quint-step-btn" 
                                on:click=move |_| change_wonder_quint_max(idx, 5)
                                title="Aumentar capacidade de Quintessência (+5)"
                                disabled=move || data.with(|d| d.wonders.get(idx).map(|w| w.quintessence_max >= 20).unwrap_or(false))
                            >
                                "+5"
                            </button>
                        </div>
                    </div>

                    <div class="wonder-squares-container">
                        {move || {
                            let max_q = data.with(|d| d.wonders.get(idx).map(|w| w.quintessence_max).unwrap_or(5));
                            (1..=max_q).map(|sq_i| {
                                let is_filled = move || data.with(|d| d.wonders.get(idx).map(|w| w.quintessence_current >= sq_i).unwrap_or(false));
                                view! {
                                    <span 
                                        class="square wonder-square"
                                        class:filled=is_filled
                                        on:click=move |_| update_wonder_quint_current(idx, sq_i)
                                        title=format!("Quintessência {}", sq_i)
                                    ></span>
                                }
                            }).collect_view()
                        }}
                    </div>
                </div>

                // Descrição da Maravilha
                <div class="wonder-desc-row">
                    <label class="wonder-label">"Descrição / Poderes:"</label>
                    <textarea 
                        class="wonder-desc-textarea"
                        placeholder="Poderes místicos, esferas exigidas, gatilhos, histórico e efeitos..."
                        prop:value=move || data.with(|d| d.wonders.get(idx).map(|w| w.description.clone()).unwrap_or_default())
                        on:change=move |ev| {
                            let val = event_target_value(&ev);
                            set_data.update(|s| {
                                while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
                                s.wonders[idx].description = val;
                            });
                        }
                        on:blur=move |ev| {
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
            <span class="group-title">"MAGIA (MAGIC)"</span>

            <div class="magic-columns-grid">
                // Coluna: Wonders (Maravilhas)
                <div class="wonders-column">
                    <div class="section-sub-title-row">
                        <span class="section-sub-title">"MARAVILHAS (WONDERS)"</span>
                    </div>

                    <div class="wonders-list">
                        {move || {
                            let count = data.with(|d| d.wonders.len());
                            (0..count).map(render_wonder_card).collect_view()
                        }}
                        <button class="add-field-btn" on:click=add_wonder title="Adicionar Maravilha">"+"</button>
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
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_data.update(|s| s.rotes = val);
                            }
                            on:blur=move |ev| {
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
