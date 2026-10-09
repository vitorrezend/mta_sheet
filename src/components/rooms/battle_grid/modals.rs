use leptos::*;
use crate::rooms::GridShape;

#[component]
pub fn GmConfigModal(
    show: Signal<bool>,
    cfg_shape: ReadSignal<GridShape>,
    set_cfg_shape: WriteSignal<GridShape>,
    cfg_cols: ReadSignal<u32>,
    set_cfg_cols: WriteSignal<u32>,
    cfg_rows: ReadSignal<u32>,
    set_cfg_rows: WriteSignal<u32>,
    cfg_cell_size: ReadSignal<u32>,
    set_cfg_cell_size: WriteSignal<u32>,
    cfg_meters_per_cell: ReadSignal<f32>,
    set_cfg_meters_per_cell: WriteSignal<f32>,
    cfg_bg_url: ReadSignal<String>,
    set_cfg_bg_url: WriteSignal<String>,
    cfg_bg_opacity: ReadSignal<f32>,
    set_cfg_bg_opacity: WriteSignal<f32>,
    on_close: Callback<()>,
    on_save: Callback<()>,
) -> impl IntoView {
    view! {
        {move || if show.get() {
            view! {
                <div class="grid-modal-backdrop" on:click=move |_| on_close.call(())>
                    <div class="grid-modal-content" on:click=move |ev: ev::MouseEvent| ev.stop_propagation()>
                        <div class="modal-header">
                            <h3>"⚙️ Configurações do Tabuleiro Tático"</h3>
                            <button class="modal-close-btn" on:click=move |_| on_close.call(())>"✕"</button>
                        </div>

                        <div class="modal-body">
                            <div class="form-group">
                                <label class="form-label">"Formato da Grade:"</label>
                                <div class="radio-toggle-group">
                                    <button
                                        type="button"
                                        class="toggle-choice-btn"
                                        class:active=move || cfg_shape.get() == GridShape::Square
                                        on:click=move |_| set_cfg_shape.set(GridShape::Square)
                                    >
                                        "🔲 Quadrada"
                                    </button>
                                    <button
                                        type="button"
                                        class="toggle-choice-btn"
                                        class:active=move || cfg_shape.get() == GridShape::HexPointy
                                        on:click=move |_| set_cfg_shape.set(GridShape::HexPointy)
                                    >
                                        "⬡ Hexagonal (Vértice)"
                                    </button>
                                    <button
                                        type="button"
                                        class="toggle-choice-btn"
                                        class:active=move || cfg_shape.get() == GridShape::HexFlat
                                        on:click=move |_| set_cfg_shape.set(GridShape::HexFlat)
                                    >
                                        "⬡ Hexagonal (Lado)"
                                    </button>
                                </div>
                            </div>

                            <div class="form-row-2">
                                <div class="form-group">
                                    <label class="form-label">"Colunas (X): " {move || cfg_cols.get()}</label>
                                    <input
                                        type="range"
                                        min="5"
                                        max="50"
                                        prop:value=move || cfg_cols.get()
                                        on:input=move |ev| {
                                            if let Ok(v) = event_target_value(&ev).parse::<u32>() {
                                                set_cfg_cols.set(v);
                                            }
                                        }
                                    />
                                </div>

                                <div class="form-group">
                                    <label class="form-label">"Linhas (Y): " {move || cfg_rows.get()}</label>
                                    <input
                                        type="range"
                                        min="5"
                                        max="50"
                                        prop:value=move || cfg_rows.get()
                                        on:input=move |ev| {
                                            if let Ok(v) = event_target_value(&ev).parse::<u32>() {
                                                set_cfg_rows.set(v);
                                            }
                                        }
                                    />
                                </div>
                            </div>

                            <div class="form-row-2">
                                <div class="form-group">
                                    <label class="form-label">"Tamanho da Célula: " {move || format!("{}px", cfg_cell_size.get())}</label>
                                    <input
                                        type="range"
                                        min="30"
                                        max="100"
                                        step="5"
                                        prop:value=move || cfg_cell_size.get()
                                        on:input=move |ev| {
                                            if let Ok(v) = event_target_value(&ev).parse::<u32>() {
                                                set_cfg_cell_size.set(v);
                                            }
                                        }
                                    />
                                </div>

                                <div class="form-group">
                                    <label class="form-label">"Metros por Célula: " {move || format!("{:.1}m", cfg_meters_per_cell.get())}</label>
                                    <input
                                        type="range"
                                        min="0.5"
                                        max="5.0"
                                        step="0.5"
                                        prop:value=move || cfg_meters_per_cell.get()
                                        on:input=move |ev| {
                                            if let Ok(v) = event_target_value(&ev).parse::<f32>() {
                                                set_cfg_meters_per_cell.set(v);
                                            }
                                        }
                                    />
                                </div>
                            </div>

                            <div class="form-group">
                                <label class="form-label">"URL da Imagem de Fundo (Mapa/Planta):"</label>
                                <input
                                    type="text"
                                    class="modal-input"
                                    placeholder="https://exemplo.com/mapa.png ou /uploads/mapa.webp"
                                    prop:value=move || cfg_bg_url.get()
                                    on:input=move |ev| set_cfg_bg_url.set(event_target_value(&ev))
                                />
                            </div>

                            <div class="form-group">
                                <label class="form-label">"Opacidade do Fundo: " {move || format!("{:.0}%", cfg_bg_opacity.get() * 100.0)}</label>
                                <input
                                    type="range"
                                    min="0.1"
                                    max="1.0"
                                    step="0.05"
                                    prop:value=move || cfg_bg_opacity.get()
                                    on:input=move |ev| {
                                        if let Ok(v) = event_target_value(&ev).parse::<f32>() {
                                            set_cfg_bg_opacity.set(v);
                                        }
                                    }
                                />
                            </div>
                        </div>

                        <div class="modal-footer">
                            <button class="modal-btn cancel-btn" on:click=move |_| on_close.call(())>"Cancelar"</button>
                            <button class="modal-btn save-btn" on:click=move |_| on_save.call(())>"Aplicar Alterações"</button>
                        </div>
                    </div>
                </div>
            }.into_view()
        } else {
            view! {}.into_view()
        }}
    }
}

#[component]
pub fn NpcCreateModal(
    show: Signal<bool>,
    npc_name: ReadSignal<String>,
    set_npc_name: WriteSignal<String>,
    npc_color: ReadSignal<String>,
    set_npc_color: WriteSignal<String>,
    npc_size: ReadSignal<f32>,
    set_npc_size: WriteSignal<f32>,
    on_close: Callback<()>,
    on_create: Callback<()>,
) -> impl IntoView {
    view! {
        {move || if show.get() {
            view! {
                <div class="grid-modal-backdrop" on:click=move |_| on_close.call(())>
                    <div class="grid-modal-content" on:click=move |ev: ev::MouseEvent| ev.stop_propagation()>
                        <div class="modal-header">
                            <h3>"👾 Adicionar NPC ou Monstro ao Grid"</h3>
                            <button class="modal-close-btn" on:click=move |_| on_close.call(())>"✕"</button>
                        </div>

                        <div class="modal-body">
                            <div class="form-group">
                                <label class="form-label">"Nome do Inimigo / Construto / NPC:"</label>
                                <input
                                    type="text"
                                    class="modal-input"
                                    placeholder="Ex: HIT Mark V, Agente NWO, Espírito de Tempestade"
                                    prop:value=move || npc_name.get()
                                    on:input=move |ev| set_npc_name.set(event_target_value(&ev))
                                />
                            </div>

                            <div class="form-row-2">
                                <div class="form-group">
                                    <label class="form-label">"Cor da Borda:"</label>
                                    <div class="color-picker-row">
                                        <input
                                            type="color"
                                            class="color-picker-input"
                                            prop:value=move || npc_color.get()
                                            on:input=move |ev| set_npc_color.set(event_target_value(&ev))
                                        />
                                        <span class="color-code">{move || npc_color.get()}</span>
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label class="form-label">"Tamanho do Token:"</label>
                                    <select
                                        class="modal-select"
                                        on:change=move |ev| {
                                            if let Ok(v) = event_target_value(&ev).parse::<f32>() {
                                                set_npc_size.set(v);
                                            }
                                        }
                                    >
                                        <option value="1.0" selected=move || npc_size.get() == 1.0>"Médio (1x1 Célula)"</option>
                                        <option value="2.0" selected=move || npc_size.get() == 2.0>"Grande (2x2 Células)"</option>
                                        <option value="3.0" selected=move || npc_size.get() == 3.0>"Enorme (3x3 Células)"</option>
                                    </select>
                                </div>
                            </div>
                        </div>

                        <div class="modal-footer">
                            <button class="modal-btn cancel-btn" on:click=move |_| on_close.call(())>"Cancelar"</button>
                            <button class="modal-btn save-btn" on:click=move |_| on_create.call(())>"Criar Token no Mapa"</button>
                        </div>
                    </div>
                </div>
            }.into_view()
        } else {
            view! {}.into_view()
        }}
    }
}
