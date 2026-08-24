use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{FileReader, HtmlInputElement, ProgressEvent};
use crate::components::common::{Callback, StableTextArea};
use crate::components::page2::ImageModal;
use crate::state::CharacterData;

const MAX_FILE_SIZE_BYTES: f64 = 10.0 * 1024.0 * 1024.0; // 10MB

#[component]
pub fn PageNotes() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    let (active_modal_image, set_active_modal_image) = create_signal::<Option<String>>(None);
    let (image_error, set_image_error) = create_signal::<Option<String>>(None);

    let session_notes = Signal::derive(move || data.with(|d| d.notes_data.session_notes.clone()));
    let on_session_notes_change = Callback::new(move |val| {
        set_data.update(|s| s.notes_data.session_notes = val);
    });

    let campaign_journal = Signal::derive(move || data.with(|d| d.notes_data.campaign_journal.clone()));
    let on_campaign_journal_change = Callback::new(move |val| {
        set_data.update(|s| s.notes_data.campaign_journal = val);
    });

    let attachment_image_url = Signal::derive(move || data.with(|d| d.notes_data.attachment_image_url.clone()));

    let handle_image_file = move |file: web_sys::File| {
        if file.size() > MAX_FILE_SIZE_BYTES {
            set_image_error.set(Some("O arquivo excede o limite máximo permitido de 10MB.".to_string()));
            return;
        }
        set_image_error.set(None);
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
                            match crate::state::save_uploaded_media(s_id, "notes".to_string(), f_name, data_url).await {
                                Ok(uploaded_url) => {
                                    set_d.update(|s| s.notes_data.attachment_image_url = uploaded_url);
                                }
                                Err(e) => {
                                    crate::logging::log_client(
                                        "errors",
                                        "ERROR",
                                        "Falha ao enviar imagem de anotações para o servidor",
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
        <div class="sheet-page-content page-notes-content">
            // Box 1: Anotações da Sessão & Contatos
            <div class="group-box notes-session-box">
                <span class="group-title">"CHRONICLE & SESSION NOTES"</span>
                <div class="notes-box-header-info">
                    <span class="notes-box-sub">"Registros da mesa, contatos, pistas investigativas, acordos e acontecimentos da crônica"</span>
                </div>
                <StableTextArea 
                    class="notes-fullpage-textarea"
                    placeholder="Escreva livremente sobre os acontecimentos da crônica, reuniões com a Cabala, encontros com NPCs, favores, débitos de Quintessência e pistas da investigação..."
                    value=session_notes
                    on_change=on_session_notes_change
                />
            </div>

            // Box 2: Diário de Campanha & Mistérios Arcanos
            <div class="group-box notes-journal-box">
                <span class="group-title">"CAMPAIGN JOURNAL & ARCANUM"</span>
                <div class="notes-box-header-info">
                    <span class="notes-box-sub">"Diário pessoal do Mago, visões de Avatar, reflexões filosóficas, teorias de Paradigma e memórias arcanas"</span>
                </div>
                <StableTextArea 
                    class="notes-fullpage-textarea"
                    placeholder="Diário íntimo do Mago, epifanias sobre o Paradigma, mensagens do Avatar, planos para o Despertar e estudos herméticos..."
                    value=campaign_journal
                    on_change=on_campaign_journal_change
                />
            </div>

            // Box 3: Arquivo Visual, Mapas & Evidências (Imagens até 10MB)
            <div class="group-box notes-visual-box">
                <span class="group-title">"VISUAL ARCHIVES & ATTACHMENTS (MAPAS & EVIDÊNCIAS)"</span>
                <div class="notes-visual-container">
                    <div class="notes-visual-header">
                        <div class="notes-visual-info">
                            <span class="notes-visual-badge">"📁 Anexo Visual"</span>
                            <span class="notes-visual-desc">"Mapas de nós místicos, diagramas, fotos de pistas, documentos arcanos ou evidências (até 10MB)"</span>
                        </div>
                        {move || {
                            let url = attachment_image_url.get();
                            if !url.is_empty() {
                                view! {
                                    <button 
                                        type="button"
                                        class="btn-remove-note-image"
                                        on:click=move |_| {
                                            set_data.update(|s| s.notes_data.attachment_image_url.clear());
                                        }
                                        title="Remover Imagem Anexada"
                                    >
                                        "× Remover Imagem"
                                    </button>
                                }.into_view()
                            } else {
                                view! { <span /> }.into_view()
                            }
                        }}
                    </div>

                    <div class="notes-image-preview-area">
                        {move || {
                            let url = attachment_image_url.get();
                            if !url.is_empty() {
                                let u_modal = url.clone();
                                view! {
                                    <div class="notes-attached-image-wrapper">
                                        <img 
                                            src=url
                                            alt="Documento / Mapa Anexado"
                                            class="notes-attached-img"
                                            title="Clique para ampliar em tela cheia"
                                            on:click=move |_| set_active_modal_image.set(Some(u_modal.clone()))
                                        />
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <label class="notes-dropzone">
                                        <input 
                                            type="file" 
                                            accept="image/*" 
                                            class="hidden-file-input"
                                            on:change=move |ev| {
                                                if let Some(target) = ev.target() {
                                                    if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                                                        if let Some(files) = input.files() {
                                                            if let Some(file) = files.get(0) {
                                                                handle_image_file(file);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        />
                                        <span class="notes-dropzone-icon">"🗺️"</span>
                                        <span class="notes-dropzone-text">"Clique ou arraste um mapa, documento ou imagem de apoio"</span>
                                        <span class="notes-dropzone-hint">"JPG, PNG, WEBP, GIF até 10MB • Clique para zoom após o upload"</span>
                                    </label>
                                }.into_view()
                            }
                        }}
                    </div>

                    {move || {
                        image_error.get().map(|err| {
                            view! { <p class="notes-image-error-msg">{err}</p> }
                        })
                    }}
                </div>
            </div>

            // Lightbox Modal para visualização em alta resolução
            <ImageModal 
                image_url=active_modal_image.into()
                on_close=Callback::new(move |_| set_active_modal_image.set(None)) 
            />
        </div>
    }
}
