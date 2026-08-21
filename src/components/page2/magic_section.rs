use leptos::*;
use crate::components::ValueField;
use crate::state::{CharacterData, DotOrigin, WonderItem};
#[allow(unused_imports)]
use crate::state::save_uploaded_media;
use crate::components::character_sheet::ActiveDotOriginContext;

#[component]
pub fn MagicSection() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");
    let active_origin_ctx = use_context::<ActiveDotOriginContext>();

    // Signal para abrir modal de visualização/zoom de imagem (Lightbox)
    let (modal_image_url, set_modal_image_url) = create_signal(Option::<String>::None);

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

    let update_wonder_name = move |idx: usize, val: String| {
        set_data.update(|s| {
            while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
            s.wonders[idx].name = val;
        });
    };

    let update_wonder_image = move |idx: usize, val: String| {
        set_data.update(|s| {
            while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
            s.wonders[idx].image_url = val;
        });
    };

    let on_image_file_change = move |idx: usize, ev: ev::Event| {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::{FileReader, HtmlInputElement};
            use wasm_bindgen::JsCast;
            use wasm_bindgen::closure::Closure;

            let input: HtmlInputElement = event_target(&ev);
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let max_size = 10 * 1024 * 1024; // 10MB
                    if file.size() as usize > max_size {
                        if let Some(w) = web_sys::window() {
                            let _ = w.alert_with_message("A imagem deve ter no máximo 10MB.");
                        }
                        return;
                    }

                    let file_name = file.name();
                    let sheet_id = data.with_untracked(|d| d.id.clone());
                    let reader = FileReader::new().unwrap();
                    let reader_clone = reader.clone();
                    let set_data_clone = set_data.clone();

                    let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
                        if let Ok(result) = reader_clone.result() {
                            if let Some(data_url) = result.as_string() {
                                let s_id = sheet_id.clone();
                                let f_name = file_name.clone();
                                let set_d = set_data_clone.clone();

                                spawn_local(async move {
                                    match save_uploaded_media(s_id, "wonders".to_string(), f_name, data_url).await {
                                        Ok(uploaded_url) => {
                                            set_d.update(|s| {
                                                while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
                                                s.wonders[idx].image_url = uploaded_url;
                                            });
                                        }
                                        Err(e) => {
                                            crate::logging::log_client(
                                                "errors",
                                                "ERROR",
                                                "Falha ao enviar imagem para o servidor",
                                                Some(&e.to_string()),
                                            );
                                        }
                                    }
                                });
                            }
                        }
                    }) as Box<dyn FnMut(_)>);

                    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                    onload.forget();
                    let _ = reader.read_as_data_url(&file);
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (idx, ev);
        }
    };

    let clear_wonder_image = move |idx: usize| {
        set_data.update(|s| {
            if let Some(w) = s.wonders.get_mut(idx) {
                w.image_url = String::new();
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
        let (show_image_field, set_show_image_field) = create_signal(false);

        let points_level = Signal::derive(move || {
            data.with(|d| d.wonders.get(idx).map(|w| w.points.level).unwrap_or(0))
        });
        let points_mod = Signal::derive(move || {
            data.with(|d| d.wonders.get(idx).map(|w| w.points.modifier.clone()).unwrap_or_default())
        });
        let points_origins = Signal::derive(move || {
            data.with(|d| d.wonders.get(idx).map(|w| w.points.get_origins(20)).unwrap_or_else(|| vec![DotOrigin::Base; 20]))
        });

        let arete_level = Signal::derive(move || {
            data.with(|d| d.wonders.get(idx).map(|w| w.arete.level).unwrap_or(0))
        });
        let arete_mod = Signal::derive(move || {
            data.with(|d| d.wonders.get(idx).map(|w| w.arete.modifier.clone()).unwrap_or_default())
        });
        let arete_origins = Signal::derive(move || {
            data.with(|d| d.wonders.get(idx).map(|w| w.arete.get_origins(9)).unwrap_or_else(|| vec![DotOrigin::Base; 9]))
        });

        let current_image_url = Signal::derive(move || {
            data.with(|d| d.wonders.get(idx).map(|w| w.image_url.clone()).unwrap_or_default())
        });

        view! {
            <div class="wonder-card">
                // 1. Linha do Topo: Nome, Botão de Imagem e Botão de Remover
                <div class="wonder-card-top-row">
                    <div class="wonder-field name-field">
                        <input 
                            type="text" 
                            class="wonder-input font-bold"
                            placeholder="Nome da Maravilha / Artefato..."
                            prop:value=move || data.with(|d| d.wonders.get(idx).map(|w| w.name.clone()).unwrap_or_default())
                            on:change=move |ev| update_wonder_name(idx, event_target_value(&ev))
                            on:blur=move |ev| update_wonder_name(idx, event_target_value(&ev))
                        />
                    </div>
                    <button 
                        type="button" 
                        class="wonder-img-toggle-btn"
                        class:active=move || !current_image_url.get().is_empty() || show_image_field.get()
                        on:click=move |_| set_show_image_field.update(|cur| *cur = !*cur)
                        title="Vincular/Alterar Imagem da Maravilha"
                    >
                        "🖼️"
                    </button>
                    <button 
                        type="button" 
                        class="wonder-remove-btn" 
                        on:click=move |_| remove_wonder(idx)
                        title="Remover Maravilha"
                    >
                        "🗑️"
                    </button>
                </div>

                // Bloco de Imagem do Item (Upload de até 10MB ou Link URL)
                {move || {
                    let url = current_image_url.get();
                    let is_visible = show_image_field.get() || !url.is_empty();
                    if is_visible {
                        view! {
                            <div class="wonder-image-section">
                                <div class="wonder-image-controls-grid">
                                    <label class="wonder-file-upload-label" title="Upload de imagem local (até 10MB)">
                                        "📁 Escolher Imagem (até 10MB)"
                                        <input 
                                            type="file" 
                                            accept="image/*" 
                                            class="wonder-hidden-file-input"
                                            on:change=move |ev| on_image_file_change(idx, ev)
                                        />
                                    </label>
                                    <input 
                                        type="text" 
                                        class="wonder-image-url-input"
                                        placeholder="Ou cole a URL da imagem (https://...)"
                                        prop:value=move || current_image_url.get()
                                        on:change=move |ev| update_wonder_image(idx, event_target_value(&ev))
                                        on:blur=move |ev| update_wonder_image(idx, event_target_value(&ev))
                                    />
                                </div>
                                {if !url.trim().is_empty() {
                                    let img_url = url.clone();
                                    let img_modal_url = url.clone();
                                    view! {
                                        <div class="wonder-image-preview-wrapper">
                                            <img 
                                                src=img_url 
                                                alt="Imagem da Maravilha (Clique para ampliar)"
                                                class="wonder-image-preview"
                                                loading="lazy"
                                                title="Clique para abrir e dar zoom na imagem"
                                                on:click=move |_| set_modal_image_url.set(Some(img_modal_url.clone()))
                                            />
                                            <button 
                                                type="button" 
                                                class="wonder-remove-image-btn" 
                                                on:click=move |_| clear_wonder_image(idx)
                                                title="Remover Imagem"
                                            >
                                                "✕ Remover Imagem"
                                            </button>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! {}.into_view()
                                }}
                            </div>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }
                }}

                // 2. Bloco de Pontos da Maravilha (1 a 20 pontos com custo e origens)
                <div class="wonder-stat-row-block">
                    <ValueField 
                        label=Signal::derive(move || "Pontos (1-20)".to_string())
                        level=points_level
                        modifier=points_mod
                        origins=points_origins
                        max_level=20
                        min_level=0
                        on_level_change=move |v| {
                            let current_origin = active_origin_ctx.map(|a| a.origin.get()).unwrap_or(DotOrigin::Base);
                            update_wonder_points(idx, v, current_origin);
                        }
                        on_modifier_change=move |m| update_wonder_points_mod(idx, m)
                        on_dot_origin_change=Callback::new(move |(dot_i, orig)| update_wonder_points_dot_origin(idx, dot_i, orig))
                        is_editable=false
                    />
                </div>

                // 3. Bloco de Arete da Maravilha (1 a 9 pontos - Estatística Própria)
                <div class="wonder-stat-row-block">
                    <ValueField 
                        label=Signal::derive(move || "Arete (1-9)".to_string())
                        level=arete_level
                        modifier=arete_mod
                        origins=arete_origins
                        max_level=9
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

                // 4. Bloco de Quintessência (5 a 20 pontos organizados em 2 linhas de 10)
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

                    <div class="wonder-squares-grid-10">
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

                // 5. Bloco de Descrição e Poderes da Maravilha
                <div class="wonder-desc-row">
                    <label class="wonder-label">"Descrição / Poderes & Esferas:"</label>
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
            <span class="group-title">"MARAVILHAS & ARTEFATOS"</span>

            // Grade 2x2 preenchendo toda a largura da folha
            <div class="wonders-grid-2x2">
                {move || {
                    let count = data.with(|d| d.wonders.len());
                    (0..count).map(render_wonder_card).collect_view()
                }}
            </div>

            <div class="wonders-footer-actions">
                <button class="add-field-btn" on:click=add_wonder title="Adicionar Nova Maravilha">"+"</button>
            </div>

            // Modal Lightbox para visualização e zoom da imagem
            {move || modal_image_url.get().map(|url| {
                view! {
                    <div class="image-modal-backdrop" on:click=move |_| set_modal_image_url.set(None)>
                        <div class="image-modal-container" on:click=move |ev| ev.stop_propagation()>
                            <button 
                                type="button" 
                                class="image-modal-close-btn"
                                on:click=move |_| set_modal_image_url.set(None)
                                title="Fechar"
                            >
                                "✕"
                            </button>
                            <img 
                                src=url 
                                alt="Imagem da Maravilha em Alta Resolução"
                                class="image-modal-img"
                            />
                        </div>
                    </div>
                }
            })}
        </div>
    }
}

// =========================================================================
// O código da seção de Rotinas (Rotes) está preservado para a próxima etapa:
// =========================================================================
#[allow(dead_code)]
fn _render_rotes_preserved_section(
    data: ReadSignal<CharacterData>,
    set_data: WriteSignal<CharacterData>,
) -> impl IntoView {
    view! {
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
    }
}
