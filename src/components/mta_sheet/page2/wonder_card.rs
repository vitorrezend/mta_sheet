use leptos::*;
use crate::components::{Callback, ValueField, StableTextArea, StableTextInput};
use crate::state::{CharacterData, DotOrigin, WonderItem};
#[allow(unused_imports)]
use crate::state::save_uploaded_media;
use crate::components::character_sheet::ActiveDotOriginContext;

#[component]
pub fn WonderCard(
    idx: usize,
    on_image_click: Callback<String>,
) -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");
    let active_origin_ctx = use_context::<ActiveDotOriginContext>();

    let (show_image_field, set_show_image_field) = create_signal(false);

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

    let clear_wonder_image = move |idx: usize| {
        set_data.update(|s| {
            if let Some(w) = s.wonders.get_mut(idx) {
                w.image_url = String::new();
            }
        });
    };

    let on_image_file_change = move |idx: usize, ev: ev::Event| {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::HtmlInputElement;

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
                    let set_d = set_data.clone();

                    crate::components::common::compress_image_file_to_webp(
                        &file,
                        crate::components::common::ImageCompressionOptions::portrait(),
                        Callback::new(move |res: Result<String, String>| {
                            match res {
                                Ok(compressed_data_url) => {
                                    let s_id = sheet_id.clone();
                                    let f_name = file_name.clone();
                                    let set_d_inner = set_d.clone();
                                    let compressed_backup = compressed_data_url.clone();

                                    spawn_local(async move {
                                        match save_uploaded_media(s_id, "wonders".to_string(), f_name, compressed_data_url).await {
                                            Ok(uploaded_url) => {
                                                let _ = set_d_inner.try_update(|s| {
                                                    while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
                                                    s.wonders[idx].image_url = uploaded_url;
                                                });
                                            }
                                            Err(e) => {
                                                let _ = set_d_inner.try_update(|s| {
                                                    while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
                                                    s.wonders[idx].image_url = compressed_backup;
                                                });
                                                crate::logging::log_client(
                                                    "errors",
                                                    "WARN",
                                                    "Falha no upload em disco da maravilha, salvo como WebP comprimido inline",
                                                    Some(&e.to_string()),
                                                );
                                            }
                                        }
                                    });
                                }
                                Err(err_msg) => {
                                    if let Some(w) = web_sys::window() {
                                        let _ = w.alert_with_message(&format!("Erro ao processar imagem: {}", err_msg));
                                    }
                                }
                            }
                        }),
                    );
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (idx, ev);
        }
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
            let new_max = (s.wonders[idx].quintessence_max + delta).clamp(0, 20);
            s.wonders[idx].quintessence_max = new_max;
            s.wonders[idx].quintessence_current = s.wonders[idx].quintessence_current.clamp(0, new_max);
        });
    };

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

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    view! {
        <div class="wonder-card">
            // 1. Linha do Topo: Nome, Botão de Imagem e Botão de Remover
            <div class="wonder-card-top-row">
                <div class="wonder-field name-field">
                    <StableTextInput 
                        class="wonder-input font-bold"
                        placeholder=Signal::derive(move || crate::i18n::tr("wonder_name_placeholder", lang()).to_string())
                        value=Signal::derive(move || data.with(|d| d.wonders.get(idx).map(|w| w.name.clone()).unwrap_or_default()))
                        on_change=Callback::new(move |val| update_wonder_name(idx, val))
                    />
                </div>
                <button 
                    type="button" 
                    class="wonder-img-toggle-btn"
                    class:active=move || !current_image_url.get().is_empty() || show_image_field.get()
                    on:click=move |_| set_show_image_field.update(|cur| *cur = !*cur)
                    title=move || match lang() {
                        crate::i18n::Language::PtBr => "Vincular/Alterar Imagem da Maravilha",
                        crate::i18n::Language::EnUs => "Attach/Change Wonder Image",
                    }
                >
                    "🖼️"
                </button>
                <button 
                    type="button" 
                    class="wonder-remove-btn" 
                    on:click=move |_| remove_wonder(idx)
                    title=move || crate::i18n::tr("remove_wonder", lang())
                >
                    "🗑️"
                </button>
            </div>

            // Bloco de Imagem do Item (Upload de até 10MB ou Link URL)
            {
                let on_image_click = on_image_click.clone();
                move || {
                    let on_image_click = on_image_click.clone();
                    let url = current_image_url.get();
                    let is_visible = show_image_field.get() || !url.is_empty();
                    let current_lang = lang();
                    if is_visible {
                        view! {
                            <div class="wonder-image-section">
                                <div class="wonder-image-controls-grid">
                                    <label class="wonder-file-upload-label" title="Upload image (max 10MB)">
                                        {match current_lang {
                                            crate::i18n::Language::PtBr => "📁 Escolher Imagem (até 10MB)",
                                            crate::i18n::Language::EnUs => "📁 Choose Image (up to 10MB)",
                                        }}
                                        <input 
                                            type="file" 
                                            accept="image/*" 
                                            class="wonder-hidden-file-input"
                                            on:change=move |ev| on_image_file_change(idx, ev)
                                        />
                                    </label>
                                    <StableTextInput 
                                        class="wonder-image-url-input"
                                        placeholder=Signal::derive(move || match lang() {
                                            crate::i18n::Language::PtBr => "Ou cole a URL da imagem (https://...)".to_string(),
                                            crate::i18n::Language::EnUs => "Or paste image URL (https://...)".to_string(),
                                        })
                                        value=current_image_url
                                        on_change=Callback::new(move |val| update_wonder_image(idx, val))
                                    />
                                </div>
                                {if !url.trim().is_empty() {
                                    let img_url = url.clone();
                                    let img_modal_url = url.clone();
                                    let on_image_click = on_image_click.clone();
                                    view! {
                                        <div class="wonder-image-preview-wrapper">
                                            <img 
                                                src=img_url 
                                                alt="Wonder Image"
                                                class="wonder-image-preview"
                                                loading="lazy"
                                                title="Click to zoom"
                                                on:click=move |_| on_image_click.call(img_modal_url.clone())
                                            />
                                        <button 
                                            type="button" 
                                            class="wonder-remove-image-btn" 
                                            on:click=move |_| clear_wonder_image(idx)
                                            title="Remove Image"
                                        >
                                            {match current_lang {
                                                crate::i18n::Language::PtBr => "✕ Remover Imagem",
                                                crate::i18n::Language::EnUs => "✕ Remove Image",
                                            }}
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
                    label=Signal::derive(move || match lang() {
                        crate::i18n::Language::PtBr => "Pontos (1-20)".to_string(),
                        crate::i18n::Language::EnUs => "Rating (1-20)".to_string(),
                    })
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

            // 4. Bloco de Quintessência (0 a 20 pontos organizados em 2 linhas de 10)
            <div class="wonder-quintessence-row">
                <div class="wonder-quint-header">
                    <span class="wonder-stat-label">{move || format!("{}:", crate::i18n::tr("quintessence", lang()))}</span>
                    <div class="wonder-quint-controls">
                        <button 
                            type="button" 
                            class="quint-step-btn" 
                            on:click=move |_| change_wonder_quint_max(idx, -5)
                            title="Reduzir capacidade (-5)"
                            disabled=move || data.with(|d| d.wonders.get(idx).map(|w| w.quintessence_max <= 0).unwrap_or(true))
                        >
                            "-5"
                        </button>
                        <span class="quint-max-badge">
                            {move || data.with(|d| d.wonders.get(idx).map(|w| format!("{}/{}", w.quintessence_current, w.quintessence_max)).unwrap_or_else(|| "0/0".to_string()))}
                        </span>
                        <button 
                            type="button" 
                            class="quint-step-btn" 
                            on:click=move |_| change_wonder_quint_max(idx, 5)
                            title="Aumentar capacidade (+5)"
                            disabled=move || data.with(|d| d.wonders.get(idx).map(|w| w.quintessence_max >= 20).unwrap_or(false))
                        >
                            "+5"
                        </button>
                    </div>
                </div>

                <div class="wonder-squares-grid-10">
                    {move || {
                        let max_q = data.with(|d| d.wonders.get(idx).map(|w| w.quintessence_max).unwrap_or(0));
                        (1..=max_q).map(|sq_i| {
                            let is_filled = move || data.with(|d| d.wonders.get(idx).map(|w| w.quintessence_current >= sq_i).unwrap_or(false));
                            view! {
                                <span 
                                    class="square wonder-square"
                                    class:filled=is_filled
                                    on:click=move |_| update_wonder_quint_current(idx, sq_i)
                                    title=format!("Quintessence {}", sq_i)
                                ></span>
                            }
                        }).collect_view()
                    }}
                </div>
            </div>

            // 5. Bloco de Descrição e Poderes da Maravilha
            <div class="wonder-desc-row">
                <label class="wonder-label">{move || format!("{}:", crate::i18n::tr("wonder_powers", lang()))}</label>
                <StableTextArea 
                    class="wonder-desc-textarea"
                    placeholder=Signal::derive(move || crate::i18n::tr("wonder_powers_placeholder", lang()).to_string())
                    value=Signal::derive(move || data.with(|d| d.wonders.get(idx).map(|w| w.description.clone()).unwrap_or_default()))
                    on_change=Callback::new(move |val| {
                        set_data.update(|s| {
                            while s.wonders.len() <= idx { s.wonders.push(WonderItem::default()); }
                            s.wonders[idx].description = val;
                        });
                    })
                />
            </div>
        </div>
    }
}
