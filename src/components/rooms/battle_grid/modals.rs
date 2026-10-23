//! Janelas do mapa: configuração da grade, novo NPC e imagem de fundo.

use leptos::*;
use crate::components::common::Callback;
use crate::rooms::{BackgroundConfig, GridConfig, GridShape, MapOp, MAX_CELL_SIZE, MAX_GRID_CELLS, MIN_CELL_SIZE, MIN_GRID_CELLS};
use super::store::MapStore;

fn parse_num<T: std::str::FromStr>(ev: &ev::Event, fallback: T) -> T {
    event_target_value(ev).trim().replace(',', ".").parse().unwrap_or(fallback)
}

#[component]
fn ModalFrame(title: &'static str, on_close: Callback<()>, children: Children) -> impl IntoView {
    let close = on_close.clone();
    view! {
        <div class="grid-modal-backdrop" on:click=move |_| close.call(())>
            <div class="grid-modal-content" role="dialog" aria-label=title on:click=move |ev: ev::MouseEvent| ev.stop_propagation()>
                <div class="modal-header">
                    <h3>{title}</h3>
                    <button type="button" class="modal-close-btn" aria-label="Fechar" on:click=move |_| on_close.call(())>"✕"</button>
                </div>
                <div class="modal-body">{children()}</div>
            </div>
        </div>
    }
}

#[component]
pub fn GridConfigModal(store: MapStore, on_close: Callback<()>) -> impl IntoView {
    let cfg = create_rw_signal(store.view.with_untracked(|m| m.config()));
    let original_shape = cfg.with_untracked(|c| c.grid_shape);
    let save = {
        let on_close = on_close.clone();
        move |_| {
            store.send(vec![MapOp::Configure(cfg.get_untracked())]);
            on_close.call(());
        }
    };
    let shape_button = move |shape: GridShape, label: &'static str| view! {
        <button type="button" class="toggle-choice-btn" class:active=move || cfg.with(|c| c.grid_shape == shape) on:click=move |_| cfg.update(|c| c.grid_shape = shape)>
            {label}
        </button>
    };

    view! {
        <ModalFrame title="⚙️ Grade do mapa" on_close=on_close.clone()>
            <div class="form-group">
                <label class="form-label">"Formato"</label>
                <div class="radio-toggle-group">
                    {shape_button(GridShape::Square, "🔲 Quadrada")}
                    {shape_button(GridShape::HexPointy, "⬢ Hex (pontas em cima)")}
                    {shape_button(GridShape::HexFlat, "⬣ Hex (lados em cima)")}
                </div>
                <Show when=move || cfg.with(|c| c.grid_shape != original_shape)>
                    <p class="grid-form-warning">"Trocar o formato apaga as paredes e portas (os lados das células mudam)."</p>
                </Show>
            </div>
            <div class="grid-form-row">
                <label class="form-group">
                    <span class="form-label">{format!("Colunas ({}–{})", MIN_GRID_CELLS, MAX_GRID_CELLS)}</span>
                    <input type="number" inputmode="numeric" min=MIN_GRID_CELLS max=MAX_GRID_CELLS prop:value=move || cfg.with(|c| c.cols)
                        on:change=move |ev| { let v = parse_num(&ev, 20u32); cfg.update(|c| c.cols = v) } />
                </label>
                <label class="form-group">
                    <span class="form-label">{format!("Linhas ({}–{})", MIN_GRID_CELLS, MAX_GRID_CELLS)}</span>
                    <input type="number" inputmode="numeric" min=MIN_GRID_CELLS max=MAX_GRID_CELLS prop:value=move || cfg.with(|c| c.rows)
                        on:change=move |ev| { let v = parse_num(&ev, 20u32); cfg.update(|c| c.rows = v) } />
                </label>
            </div>
            <div class="grid-form-row">
                <label class="form-group">
                    <span class="form-label">"Tamanho da célula (px)"</span>
                    <input type="number" inputmode="numeric" min=MIN_CELL_SIZE max=MAX_CELL_SIZE prop:value=move || cfg.with(|c| c.cell_size)
                        on:change=move |ev| { let v = parse_num(&ev, 50u32); cfg.update(|c| c.cell_size = v) } />
                </label>
                <label class="form-group">
                    <span class="form-label">"Metros por célula"</span>
                    <input type="number" inputmode="decimal" step="0.5" min="0.1" prop:value=move || cfg.with(|c| c.meters_per_cell)
                        on:change=move |ev| { let v = parse_num(&ev, 1.5f32); cfg.update(|c| c.meters_per_cell = v) } />
                </label>
            </div>
            <label class="grid-check">
                <input type="checkbox" prop:checked=move || cfg.with(|c| c.show_grid) on:change=move |ev| { let v = event_target_checked(&ev); cfg.update(|c| c.show_grid = v) } />
                "Mostrar linhas da grade"
            </label>
            <div class="modal-footer">
                <button type="button" class="btn-primary" on:click=save>"Aplicar"</button>
            </div>
        </ModalFrame>
    }
}

#[component]
pub fn NpcModal(on_create: Callback<(String, String, f32)>, on_close: Callback<()>) -> impl IntoView {
    let name = create_rw_signal(String::new());
    let color = create_rw_signal("#ef4444".to_string());
    let size = create_rw_signal(1.0f32);
    let create = {
        let on_close = on_close.clone();
        move |_| {
            on_create.call((name.get_untracked(), color.get_untracked(), size.get_untracked()));
            on_close.call(());
        }
    };
    view! {
        <ModalFrame title="👹 Novo NPC" on_close=on_close.clone()>
            <label class="form-group">
                <span class="form-label">"Nome"</span>
                <input type="text" maxlength="40" placeholder="Ex.: Agente da Tecnocracia" prop:value=move || name.get() on:change=move |ev| name.set(event_target_value(&ev)) />
            </label>
            <div class="grid-form-row">
                <label class="form-group">
                    <span class="form-label">"Cor"</span>
                    <input type="color" prop:value=move || color.get() on:change=move |ev| color.set(event_target_value(&ev)) />
                </label>
                <label class="form-group">
                    <span class="form-label">"Tamanho (células)"</span>
                    <select on:change=move |ev| size.set(parse_num(&ev, 1.0))>
                        <option value="1" selected=true>"1 (humano)"</option>
                        <option value="2">"2 (grande)"</option>
                        <option value="3">"3 (enorme)"</option>
                    </select>
                </label>
            </div>
            <div class="modal-footer">
                <button type="button" class="btn-primary" on:click=create>"Colocar no mapa"</button>
            </div>
        </ModalFrame>
    }
}

/// Envia a imagem (comprimida no navegador) e devolve o endereço público.
#[cfg(target_arch = "wasm32")]
fn upload_background(file: web_sys::File, on_done: impl Fn(Result<String, String>) + 'static) {
    use crate::components::common::image_compressor::{compress_image_file_to_webp, ImageCompressionOptions};
    let on_done = std::rc::Rc::new(on_done);
    let options = ImageCompressionOptions { max_width: 3000, max_height: 3000, quality: 0.85 };
    compress_image_file_to_webp(&file, options, Callback::new(move |result: Result<String, String>| {
        let on_done = on_done.clone();
        spawn_local(async move {
            let uploaded = async {
                let data_url = result?;
                let (meta, b64) = data_url.split_once(',').ok_or("Imagem inválida")?;
                let ext = if meta.contains("webp") { "webp" } else if meta.contains("png") { "png" } else { "jpg" };
                use base64::Engine;
                let bytes = base64::engine::general_purpose::STANDARD.decode(b64).map_err(|e| e.to_string())?;
                let parts = js_sys::Array::of1(&js_sys::Uint8Array::from(bytes.as_slice()));
                let blob = web_sys::Blob::new_with_u8_array_sequence(&parts).map_err(|_| "Falha ao preparar a imagem")?;
                let form = web_sys::FormData::new().map_err(|_| "Falha ao preparar o envio")?;
                form.append_with_blob_and_filename("file", &blob, &format!("mapa.{}", ext)).map_err(|_| "Falha ao preparar o envio")?;
                let resp = gloo_net::http::Request::post("/api/upload_image").body(form).map_err(|e| e.to_string())?.send().await.map_err(|e| e.to_string())?;
                let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
                json.get("url").and_then(|u| u.as_str()).map(str::to_string)
                    .ok_or_else(|| json.get("error").and_then(|e| e.as_str()).unwrap_or("Falha no envio").to_string())
            }
            .await;
            on_done(uploaded);
        });
    }));
}

#[component]
pub fn BackgroundModal(store: MapStore, on_close: Callback<()>) -> impl IntoView {
    let bg = create_rw_signal(store.view.with_untracked(|m| m.background()));
    let uploading = create_rw_signal(false);
    let message = create_rw_signal(Option::<String>::None);

    // Pré-visualiza ao arrastar; grava ao soltar
    let preview = move || store.preview(&[MapOp::SetBackground(bg.get_untracked())]);
    let commit = move || store.send(vec![MapOp::SetBackground(bg.get_untracked())]);

    let on_file = move |ev: ev::Event| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            let file = ev.target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                .and_then(|i| i.files())
                .and_then(|f| f.get(0));
            if let Some(file) = file {
                uploading.set(true);
                message.set(Some("Enviando imagem…".into()));
                upload_background(file, move |result| {
                    uploading.set(false);
                    match result {
                        Ok(url) => {
                            bg.update(|b| { b.url = url; b.offset_x = 0.0; b.offset_y = 0.0; b.scale = 1.0; b.opacity = 1.0; });
                            commit();
                            message.set(Some("Imagem aplicada. Ajuste a posição e a escala para alinhar com a grade.".into()));
                        }
                        Err(e) => message.set(Some(format!("Não foi possível enviar: {}", e))),
                    }
                });
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = ev;
    };

    let slider = move |label: &'static str, min: f32, max: f32, step: f32, get: fn(&BackgroundConfig) -> f32, set: fn(&mut BackgroundConfig, f32)| view! {
        <label class="form-group grid-slider">
            <span class="form-label">{label} ": " <strong>{move || format!("{:.2}", bg.with(get))}</strong></span>
            <input type="range" min=min max=max step=step prop:value=move || bg.with(get)
                on:input=move |ev| { let v = parse_num(&ev, 0.0f32); bg.update(|b| set(b, v)); preview(); }
                on:change=move |_| commit() />
        </label>
    };

    view! {
        <ModalFrame title="🖼️ Imagem de fundo" on_close=on_close.clone()>
            <label class="form-group">
                <span class="form-label">"Enviar imagem do mapa (até 5 MB depois de comprimida)"</span>
                <input type="file" accept="image/png,image/jpeg,image/webp,image/gif" disabled=move || uploading.get() on:change=on_file />
            </label>
            <Show when=move || message.get().is_some()>
                <p class="grid-form-hint">{move || message.get().unwrap_or_default()}</p>
            </Show>
            <Show when=move || bg.with(|b| !b.url.is_empty())>
                <div class="grid-bg-controls">
                    {slider("Opacidade", 0.05, 1.0, 0.05, |b| b.opacity, |b, v| b.opacity = v)}
                    {slider("Escala", 0.1, 4.0, 0.01, |b| b.scale, |b, v| b.scale = v)}
                    {slider("Mover na horizontal (px)", -1000.0, 1000.0, 1.0, |b| b.offset_x, |b, v| b.offset_x = v)}
                    {slider("Mover na vertical (px)", -1000.0, 1000.0, 1.0, |b| b.offset_y, |b, v| b.offset_y = v)}
                    <button type="button" class="btn-secondary" on:click=move |_| { bg.update(|b| b.url.clear()); commit(); }>"Remover imagem"</button>
                </div>
            </Show>
        </ModalFrame>
    }
}
