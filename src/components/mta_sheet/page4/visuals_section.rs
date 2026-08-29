use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{FileReader, HtmlInputElement, ProgressEvent};
use crate::components::Callback;
use crate::components::page2::ImageModal;
use super::CardFramingModal;
use crate::state::CharacterData;

const MAX_FILE_SIZE_BYTES: f64 = 10.0 * 1024.0 * 1024.0; // 10MB

#[component]
pub fn VisualsSection() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let (active_modal_image, set_active_modal_image) = create_signal::<Option<String>>(None);
    let (cabal_error, set_cabal_error) = create_signal::<Option<String>>(None);
    let (sketch_error, set_sketch_error) = create_signal::<Option<String>>(None);
    let is_framing_modal_open = create_rw_signal(false);

    let cabal_chart_url = Signal::derive(move || data.with(|d| d.visuals.cabal_chart_url.clone()));
    let character_sketch_url = Signal::derive(move || {
        data.with(|d| {
            if !d.visuals.character_sketch_url.is_empty() {
                d.visuals.character_sketch_url.clone()
            } else {
                d.get_profile_photo()
            }
        })
    });

    let handle_cabal_file = move |file: web_sys::File| {
        if file.size() > MAX_FILE_SIZE_BYTES {
            set_cabal_error.set(Some("O arquivo excede o limite de 10MB.".to_string()));
            return;
        }
        set_cabal_error.set(None);
        let file_name = file.name();
        let sheet_id = data.with_untracked(|d| d.id.clone());
        let set_d = set_data.clone();
        let set_err = set_cabal_error.clone();

        crate::components::common::compress_image_file_to_webp(
            &file,
            crate::components::common::ImageCompressionOptions::chart_or_map(),
            Callback::new(move |res: Result<String, String>| {
                match res {
                    Ok(compressed_data_url) => {
                        let s_id = sheet_id.clone();
                        let f_name = file_name.clone();
                        let set_d_inner = set_d.clone();
                        let compressed_backup = compressed_data_url.clone();

                        spawn_local(async move {
                            match crate::state::save_uploaded_media(s_id, "cabal".to_string(), f_name, compressed_data_url).await {
                                Ok(uploaded_url) => {
                                    set_d_inner.update(|s| s.visuals.cabal_chart_url = uploaded_url);
                                }
                                Err(e) => {
                                    // Fallback: salva a string WebP comprimida de tamanho reduzido diretamente
                                    set_d_inner.update(|s| s.visuals.cabal_chart_url = compressed_backup);
                                    crate::logging::log_client(
                                        "errors",
                                        "WARN",
                                        "Upload em disco indisponível, imagem comprimida WebP salva inline",
                                        Some(&e.to_string()),
                                    );
                                }
                            }
                        });
                    }
                    Err(err_msg) => {
                        set_err.set(Some(format!("Erro ao processar imagem: {}", err_msg)));
                    }
                }
            }),
        );
    };

    let handle_sketch_file = move |file: web_sys::File| {
        if file.size() > MAX_FILE_SIZE_BYTES {
            set_sketch_error.set(Some("O arquivo excede o limite de 10MB.".to_string()));
            return;
        }
        set_sketch_error.set(None);
        let file_name = file.name();
        let sheet_id = data.with_untracked(|d| d.id.clone());
        let set_d = set_data.clone();
        let set_err = set_sketch_error.clone();

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
                            match crate::state::save_uploaded_media(s_id, "sketch".to_string(), f_name, compressed_data_url).await {
                                Ok(uploaded_url) => {
                                    set_d_inner.update(|s| {
                                        s.visuals.character_sketch_url = uploaded_url.clone();
                                        s.set_profile_photo(uploaded_url);
                                    });
                                }
                                Err(e) => {
                                    // Fallback: salva a string WebP comprimida de tamanho reduzido diretamente
                                    set_d_inner.update(|s| {
                                        s.visuals.character_sketch_url = compressed_backup.clone();
                                        s.set_profile_photo(compressed_backup);
                                    });
                                    crate::logging::log_client(
                                        "errors",
                                        "WARN",
                                        "Upload em disco indisponível, imagem comprimida WebP salva inline",
                                        Some(&e.to_string()),
                                    );
                                }
                            }
                        });
                    }
                    Err(err_msg) => {
                        set_err.set(Some(format!("Erro ao processar retrato: {}", err_msg)));
                    }
                }
            }),
        );
    };

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    view! {
        <div class="group-box visuals-box">
            <span class="group-title">{move || crate::i18n::tr("visuals_title", lang())}</span>

            <div class="visuals-grid-2col">
                // 1. Cabal Chart
                <div class="visual-card">
                    <div class="visual-card-header">
                        <label class="visual-label">{move || crate::i18n::tr("cabal_chart_title", lang())}</label>
                        {move || {
                            let url = cabal_chart_url.get();
                            let current_lang = lang();
                            if !url.is_empty() {
                                view! {
                                    <button 
                                        type="button"
                                        class="btn-remove-visual"
                                        on:click=move |_| {
                                            set_data.update(|s| s.visuals.cabal_chart_url.clear());
                                        }
                                        title=move || match current_lang {
                                            crate::i18n::Language::PtBr => "Remover Imagem",
                                            crate::i18n::Language::EnUs => "Remove Image",
                                        }
                                    >
                                        {match current_lang {
                                            crate::i18n::Language::PtBr => "Remover",
                                            crate::i18n::Language::EnUs => "Remove",
                                        }}
                                    </button>
                                }.into_view()
                            } else {
                                view! { <span /> }.into_view()
                            }
                        }}
                    </div>

                    <div class="visual-preview-container">
                        {move || {
                            let url = cabal_chart_url.get();
                            let current_lang = lang();
                            if !url.is_empty() {
                                let u_modal = url.clone();
                                view! {
                                    <div class="visual-image-wrapper">
                                        <img 
                                            src=url
                                            alt="Cabal Chart"
                                            class="visual-img wonder-image-preview"
                                            title="Click to zoom"
                                            on:click=move |_| set_active_modal_image.set(Some(u_modal.clone()))
                                        />
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <label class="visual-dropzone">
                                        <input 
                                            type="file" 
                                            accept="image/*" 
                                            class="hidden-file-input"
                                            on:change=move |ev| {
                                                if let Some(target) = ev.target() {
                                                    if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                                                        if let Some(files) = input.files() {
                                                            if let Some(file) = files.get(0) {
                                                                handle_cabal_file(file);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        />
                                        <span class="visual-dropzone-icon">"📐"</span>
                                        <span class="visual-dropzone-text">
                                            {match current_lang {
                                                crate::i18n::Language::PtBr => "Clique ou arraste o diagrama / símbolo da Cabala",
                                                crate::i18n::Language::EnUs => "Click or drag Cabal diagram / crest",
                                            }}
                                        </span>
                                        <span class="visual-dropzone-hint">"JPG, PNG, WEBP max 10MB"</span>
                                    </label>
                                }.into_view()
                            }
                        }}
                    </div>

                    {move || {
                        cabal_error.get().map(|err| {
                            view! { <p class="visual-error-msg">{err}</p> }
                        })
                    }}
                </div>

                // 2. Character Sketch
                <div class="visual-card">
                    <div class="visual-card-header">
                        <label class="visual-label">{move || crate::i18n::tr("portrait_title", lang())}</label>
                        {move || {
                            let url = character_sketch_url.get();
                            let current_lang = lang();
                            if !url.is_empty() {
                                view! {
                                    <button 
                                        type="button"
                                        class="btn-remove-visual"
                                        on:click=move |_| {
                                            set_data.update(|s| {
                                                s.visuals.character_sketch_url.clear();
                                                s.set_profile_photo(String::new());
                                            });
                                        }
                                        title=move || match current_lang {
                                            crate::i18n::Language::PtBr => "Remover Imagem",
                                            crate::i18n::Language::EnUs => "Remove Image",
                                        }
                                    >
                                        {match current_lang {
                                            crate::i18n::Language::PtBr => "Remover",
                                            crate::i18n::Language::EnUs => "Remove",
                                        }}
                                    </button>
                                }.into_view()
                            } else {
                                view! { <span /> }.into_view()
                            }
                        }}
                    </div>

                    <div class="visual-preview-container">
                        {move || {
                            let url = character_sketch_url.get();
                            let current_lang = lang();
                            if !url.is_empty() {
                                let u_modal = url.clone();
                                let u_card_preview = url.clone();
                                let focus_y = Signal::derive(move || data.with(|d| d.get_photo_focus().1));
                                let focus_x = Signal::derive(move || data.with(|d| d.get_photo_focus().0));

                                view! {
                                    <div class="visual-sketch-with-framing">
                                        <div class="visual-image-wrapper">
                                            <img 
                                                src=url
                                                alt="Character Sketch"
                                                class="visual-img wonder-image-preview"
                                                title="Click to zoom"
                                                on:click=move |_| set_active_modal_image.set(Some(u_modal.clone()))
                                            />
                                        </div>

                                        // Card Framing Action Panel
                                        <div class="visual-framing-action-box">
                                            <div class="framing-action-info">
                                                <span class="framing-action-title">
                                                    "📐 "
                                                    {match current_lang {
                                                        crate::i18n::Language::PtBr => "Enquadramento nos Cards",
                                                        crate::i18n::Language::EnUs => "Card Framing & Position",
                                                    }}
                                                </span>
                                                <span class="framing-action-hint">
                                                    {match current_lang {
                                                        crate::i18n::Language::PtBr => "Arraste a foto no modal para escolher qual parte aparece nos cards.",
                                                        crate::i18n::Language::EnUs => "Drag the photo in the modal to choose which part appears on the cards.",
                                                    }}
                                                </span>
                                            </div>

                                            <div class="framing-action-controls">
                                                <button
                                                    type="button"
                                                    class="btn-open-framing-modal"
                                                    on:click=move |_| is_framing_modal_open.set(true)
                                                >
                                                    "📐 "
                                                    {match current_lang {
                                                        crate::i18n::Language::PtBr => "Ajustar Enquadramento",
                                                        crate::i18n::Language::EnUs => "Adjust Framing",
                                                    }}
                                                </button>

                                                <div 
                                                    class="card-portrait-box mini-card-framing-preview clickable-preview" 
                                                    title=match current_lang {
                                                        crate::i18n::Language::PtBr => "Clique para ajustar enquadramento",
                                                        crate::i18n::Language::EnUs => "Click to adjust framing",
                                                    }
                                                    on:click=move |_| is_framing_modal_open.set(true)
                                                >
                                                    <img
                                                        src=u_card_preview
                                                        alt="Card Preview"
                                                        class="card-portrait-img"
                                                        style=move || format!("object-position: {}% {}%;", focus_x.get(), focus_y.get())
                                                    />
                                                    <div class="card-portrait-gradient"></div>
                                                    <div class="preview-hover-tag">"✏️ Editar"</div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <label class="visual-dropzone">
                                        <input 
                                            type="file" 
                                            accept="image/*" 
                                            class="hidden-file-input"
                                            on:change=move |ev| {
                                                if let Some(target) = ev.target() {
                                                    if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                                                        if let Some(files) = input.files() {
                                                            if let Some(file) = files.get(0) {
                                                                handle_sketch_file(file);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        />
                                        <span class="visual-dropzone-icon">"🎨"</span>
                                        <span class="visual-dropzone-text">
                                            {match current_lang {
                                                crate::i18n::Language::PtBr => "Clique ou arraste o retrato / foto do personagem",
                                                crate::i18n::Language::EnUs => "Click or drag character portrait / photo",
                                            }}
                                        </span>
                                        <span class="visual-dropzone-hint">"JPG, PNG, WEBP max 10MB"</span>
                                    </label>
                                }.into_view()
                            }
                        }}
                    </div>

                    {move || {
                        sketch_error.get().map(|err| {
                            view! { <p class="visual-error-msg">{err}</p> }
                        })
                    }}
                </div>
            </div>

            // Lightbox Modal
            <ImageModal 
                image_url=active_modal_image.into()
                on_close=Callback::new(move |_| set_active_modal_image.set(None)) 
            />

            // Card Framing Drag Modal
            <Show
                when=move || is_framing_modal_open.get() && !character_sketch_url.get().is_empty()
                fallback=|| ()
            >
                {
                    let url = character_sketch_url.get();
                    let (fx, fy) = data.with(|d| d.get_photo_focus());
                    let set_d = set_data.clone();
                    view! {
                        <CardFramingModal
                            image_url=url
                            initial_focus_x=fx
                            initial_focus_y=fy
                            on_save=Callback::new(move |(new_x, new_y)| {
                                set_d.update(|s| s.set_photo_focus(new_x, new_y));
                                is_framing_modal_open.set(false);
                            })
                            on_close=Callback::new(move |_| {
                                is_framing_modal_open.set(false);
                            })
                        />
                    }
                }
            </Show>
        </div>
    }
}
