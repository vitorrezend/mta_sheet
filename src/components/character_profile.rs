use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, FileReader, HtmlInputElement, ProgressEvent};
use crate::state::CharacterData;

const MAX_FILE_SIZE_BYTES: f64 = 10.0 * 1024.0 * 1024.0; // 10MB

#[component]
pub fn CharacterProfile() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let (upload_error, set_upload_error) = create_signal::<Option<String>>(None);
    let (is_dragging, set_is_dragging) = create_signal(false);

    let profile_photo = Signal::derive(move || data.with(|d| d.get_profile_photo()));
    let history_text = Signal::derive(move || data.with(|d| d.get_history()));
    let notes_text = Signal::derive(move || data.with(|d| d.get_notes()));

    let handle_file = move |file: web_sys::File| {
        if file.size() > MAX_FILE_SIZE_BYTES {
            set_upload_error.set(Some("O arquivo selecionado excede o limite máximo de 10MB.".to_string()));
            return;
        }

        let file_type = file.type_();
        if !file_type.is_empty() && !file_type.starts_with("image/") {
            set_upload_error.set(Some("Formato não suportado. Por favor escolha uma imagem JPG, PNG ou WEBP.".to_string()));
            return;
        }

        set_upload_error.set(None);

        if let Ok(reader) = FileReader::new() {
            let reader_clone = reader.clone();
            let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: ProgressEvent| {
                if let Ok(result) = reader_clone.result() {
                    if let Some(data_url) = result.as_string() {
                        set_data.update(|s| s.set_profile_photo(data_url));
                    }
                }
            }) as Box<dyn FnMut(ProgressEvent)>);

            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
            onload.forget();
            let _ = reader.read_as_data_url(&file);
        }
    };

    let on_file_input_change = move |ev: Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                if let Some(files) = input.files() {
                    if let Some(file) = files.get(0) {
                        handle_file(file);
                    }
                }
            }
        }
    };

    let remove_photo = move |_| {
        set_data.update(|s| s.set_profile_photo(String::new()));
        set_upload_error.set(None);
    };

    let on_drag_over = move |ev: web_sys::DragEvent| {
        ev.prevent_default();
        set_is_dragging.set(true);
    };

    let on_drag_leave = move |ev: web_sys::DragEvent| {
        ev.prevent_default();
        set_is_dragging.set(false);
    };

    let on_drop = move |ev: web_sys::DragEvent| {
        ev.prevent_default();
        set_is_dragging.set(false);
        if let Some(data_transfer) = ev.data_transfer() {
            if let Some(files) = data_transfer.files() {
                if let Some(file) = files.get(0) {
                    handle_file(file);
                }
            }
        }
    };

    view! {
        <div class="profile-page-content">
            // Cabeçalho da Página de Perfil
            <div class="profile-header-banner">
                <div class="profile-header-title-box">
                    <h2 class="profile-title">"PERFIL DO PERSONAGEM"</h2>
                    <span class="profile-subtitle">
                        "Registros de Identidade, Origens, Grimório e Memórias"
                    </span>
                </div>
            </div>

            // Layout Principal em Grid A4
            <div class="profile-grid-container">
                // Coluna Esquerda: Foto & Identidade Visual
                <div class="profile-portrait-column">
                    <div class="group-box profile-portrait-box">
                        <div class="group-box-header">
                            <span class="group-box-title">"RETRATO / FOTO"</span>
                            <span class="group-box-badge">"JPG / PNG até 10MB"</span>
                        </div>

                        <div 
                            class="portrait-dropzone"
                            class:has-photo=move || !profile_photo.get().is_empty()
                            class:is-dragging=move || is_dragging.get()
                            on:dragover=on_drag_over
                            on:dragleave=on_drag_leave
                            on:drop=on_drop
                        >
                            {move || {
                                let photo = profile_photo.get();
                                if !photo.is_empty() {
                                    view! {
                                        <div class="portrait-preview-wrapper">
                                            <div class="portrait-checkerboard-bg">
                                                <img 
                                                    src=photo 
                                                    alt="Retrato do Personagem" 
                                                    class="portrait-img"
                                                />
                                            </div>
                                            <div class="portrait-actions-overlay">
                                                <label class="portrait-btn portrait-btn-change" title="Substituir foto">
                                                    "📷 Alterar"
                                                    <input 
                                                        type="file" 
                                                        accept="image/png, image/jpeg, image/jpg, image/webp, image/gif" 
                                                        class="hidden-file-input"
                                                        on:change=on_file_input_change
                                                    />
                                                </label>
                                                <button 
                                                    type="button"
                                                    class="portrait-btn portrait-btn-remove" 
                                                    on:click=remove_photo
                                                    title="Remover foto"
                                                >
                                                    "🗑️ Remover"
                                                </button>
                                            </div>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! {
                                        <label class="portrait-upload-placeholder">
                                            <div class="upload-icon-circle">
                                                <span class="upload-icon">"👤"</span>
                                            </div>
                                            <div class="upload-text-group">
                                                <span class="upload-primary-text">"Clique para enviar ou arraste a foto"</span>
                                                <span class="upload-secondary-text">"Formatos: PNG (com transparência), JPG, WEBP"</span>
                                                <span class="upload-limit-badge">"Tamanho Máximo: 10MB"</span>
                                            </div>
                                            <input 
                                                type="file" 
                                                accept="image/png, image/jpeg, image/jpg, image/webp, image/gif" 
                                                class="hidden-file-input"
                                                on:change=on_file_input_change
                                            />
                                        </label>
                                    }.into_view()
                                }
                            }}
                        </div>

                        {move || upload_error.get().map(|err| view! {
                            <div class="portrait-error-banner">
                                <span class="error-icon">"⚠️"</span>
                                <span>{err}</span>
                            </div>
                        })}

                        <div class="portrait-hint-text">
                            "Imagens com fundo transparente (PNG) são renderizadas com total fidelidade."
                        </div>
                    </div>
                </div>

                // Coluna Direita / Bloco 1: História
                <div class="profile-text-column">
                    <div class="group-box profile-text-box">
                        <div class="group-box-header">
                            <span class="group-box-title">"HISTÓRIA & ORIGENS"</span>
                            <span class="text-length-badge">
                                {move || format!("{} caracteres", history_text.get().chars().count())}
                            </span>
                        </div>
                        <div class="profile-textarea-container">
                            <textarea
                                class="profile-textarea"
                                placeholder="Escreva aqui a trajetória do personagem: sua vida antes do Despertar, o momento da Epifania, a natureza de seu Avatar, mentores, tradição, alianças passadas, motivações e objetivos arcanos..."
                                prop:value=move || history_text.get()
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_data.update(|s| s.set_history(val));
                                }
                                on:blur=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_data.update(|s| s.set_history(val));
                                }
                            ></textarea>
                        </div>
                    </div>
                </div>
            </div>

            // Bloco Inferior: Anotações & Grimório
            <div class="profile-full-row">
                <div class="group-box profile-text-box notes-box">
                    <div class="group-box-header">
                        <span class="group-box-title">"ANOTAÇÕES, GRIMÓRIO & REGISTROS"</span>
                        <span class="text-length-badge">
                            {move || format!("{} caracteres", notes_text.get().chars().count())}
                        </span>
                    </div>
                    <div class="profile-textarea-container">
                        <textarea
                            class="profile-textarea notes-textarea"
                            placeholder="Registre feitiços conhecidos (Rotes), instrumentos de foco mágico, aliados da Cabala, pertences místicos, sanctuários, diário de sessões e observações do narrador..."
                            prop:value=move || notes_text.get()
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_data.update(|s| s.set_notes(val));
                            }
                            on:blur=move |ev| {
                                let val = event_target_value(&ev);
                                set_data.update(|s| s.set_notes(val));
                            }
                        ></textarea>
                    </div>
                </div>
            </div>
        </div>
    }
}
