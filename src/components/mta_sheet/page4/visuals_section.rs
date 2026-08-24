use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{FileReader, HtmlInputElement, ProgressEvent};
use crate::components::Callback;
use crate::components::page2::ImageModal;
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

        if let Ok(reader) = FileReader::new() {
            let reader_clone = reader.clone();
            let set_data_clone = set_data.clone();
            let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: ProgressEvent| {
                if let Ok(result) = reader_clone.result() {
                    if let Some(data_url) = result.as_string() {
                        let s_id = sheet_id.clone();
                        let f_name = file_name.clone();
                        let set_d = set_data_clone.clone();

                        spawn_local(async move {
                            match crate::state::save_uploaded_media(s_id, "cabal".to_string(), f_name, data_url).await {
                                Ok(uploaded_url) => {
                                    set_d.update(|s| s.visuals.cabal_chart_url = uploaded_url);
                                }
                                Err(e) => {
                                    crate::logging::log_client(
                                        "errors",
                                        "ERROR",
                                        "Falha ao enviar organograma do Cabal para o servidor",
                                        Some(&e.to_string()),
                                    );
                                }
                            }
                        });
                    }
                }
            }) as Box<dyn FnMut(ProgressEvent)>);

            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
            onload.forget();
            let _ = reader.read_as_data_url(&file);
        }
    };

    let handle_sketch_file = move |file: web_sys::File| {
        if file.size() > MAX_FILE_SIZE_BYTES {
            set_sketch_error.set(Some("O arquivo excede o limite de 10MB.".to_string()));
            return;
        }
        set_sketch_error.set(None);
        let file_name = file.name();
        let sheet_id = data.with_untracked(|d| d.id.clone());

        if let Ok(reader) = FileReader::new() {
            let reader_clone = reader.clone();
            let set_data_clone = set_data.clone();
            let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: ProgressEvent| {
                if let Ok(result) = reader_clone.result() {
                    if let Some(data_url) = result.as_string() {
                        let s_id = sheet_id.clone();
                        let f_name = file_name.clone();
                        let set_d = set_data_clone.clone();

                        spawn_local(async move {
                            match crate::state::save_uploaded_media(s_id, "profile".to_string(), f_name, data_url).await {
                                Ok(uploaded_url) => {
                                    set_d.update(|s| {
                                        s.visuals.character_sketch_url = uploaded_url.clone();
                                        s.set_profile_photo(uploaded_url);
                                    });
                                }
                                Err(e) => {
                                    crate::logging::log_client(
                                        "errors",
                                        "ERROR",
                                        "Falha ao enviar retrato do personagem para o servidor",
                                        Some(&e.to_string()),
                                    );
                                }
                            }
                        });
                    }
                }
            }) as Box<dyn FnMut(ProgressEvent)>);

            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
            onload.forget();
            let _ = reader.read_as_data_url(&file);
        }
    };

    view! {
        <div class="group-box visuals-box">
            <span class="group-title">"VISUALS"</span>

            <div class="visuals-grid-2col">
                // 1. Cabal Chart
                <div class="visual-card">
                    <div class="visual-card-header">
                        <label class="visual-label">"CABAL CHART (Organograma / Diagrama do Cabal)"</label>
                        {move || {
                            let url = cabal_chart_url.get();
                            if !url.is_empty() {
                                view! {
                                    <button 
                                        type="button"
                                        class="btn-remove-visual"
                                        on:click=move |_| {
                                            set_data.update(|s| s.visuals.cabal_chart_url.clear());
                                        }
                                        title="Remover Imagem"
                                    >
                                        "Remover"
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
                            if !url.is_empty() {
                                let u_modal = url.clone();
                                view! {
                                    <div class="visual-image-wrapper">
                                        <img 
                                            src=url
                                            alt="Cabal Chart"
                                            class="visual-img wonder-image-preview"
                                            title="Clique para ampliar em tela cheia"
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
                                        <span class="visual-dropzone-text">"Clique ou arraste o diagrama / símbolo do Cabal"</span>
                                        <span class="visual-dropzone-hint">"JPG, PNG, WEBP até 10MB"</span>
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
                        <label class="visual-label">"CHARACTER SKETCH (Retrato do Personagem)"</label>
                        {move || {
                            let url = character_sketch_url.get();
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
                                        title="Remover Imagem"
                                    >
                                        "Remover"
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
                            if !url.is_empty() {
                                let u_modal = url.clone();
                                view! {
                                    <div class="visual-image-wrapper">
                                        <img 
                                            src=url
                                            alt="Character Sketch"
                                            class="visual-img wonder-image-preview"
                                            title="Clique para ampliar em tela cheia"
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
                                                                handle_sketch_file(file);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        />
                                        <span class="visual-dropzone-icon">"🎨"</span>
                                        <span class="visual-dropzone-text">"Clique ou arraste o retrato / esboço do personagem"</span>
                                        <span class="visual-dropzone-hint">"JPG, PNG, WEBP até 10MB"</span>
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
        </div>
    }
}
