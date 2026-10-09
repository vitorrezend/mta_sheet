use leptos::*;
use crate::rooms::GridShape;

#[component]
pub fn BattleGridToolbar(
    active_tool: ReadSignal<&'static str>,
    set_active_tool: WriteSignal<&'static str>,
    selected_build_type: ReadSignal<String>,
    set_selected_build_type: WriteSignal<String>,
    selected_token_id: ReadSignal<Option<String>>,
    selected_token_name: Signal<Option<String>>,
    zoom_scale: ReadSignal<f32>,
    set_zoom_scale: WriteSignal<f32>,
    save_msg: ReadSignal<Option<String>>,
    is_gm: Signal<bool>,
    grid_shape: Signal<GridShape>,
    cols: Signal<u32>,
    rows: Signal<u32>,
    on_spawn_party: Callback<()>,
    on_open_npc_modal: Callback<()>,
    on_open_gm_modal: Callback<()>,
    on_fit_to_screen: Callback<()>,
    on_remove_token: Callback<String>,
    on_reset_ruler: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="battle-grid-toolbar">
            <div class="toolbar-left">
                <button 
                    class="tool-btn" 
                    class:active=move || active_tool.get() == "select"
                    on:click=move |_| {
                        set_active_tool.set("select");
                        on_reset_ruler.call(());
                    }
                    title="Modo Seleção & Movimento (Arraste ou clique)"
                >
                    "👆 Mover"
                </button>

                <button 
                    class="tool-btn" 
                    class:active=move || active_tool.get() == "build"
                    on:click=move |_| {
                        set_active_tool.set("build");
                        on_reset_ruler.call(());
                    }
                    title="Construir Muros, Portas, Terreno e Selos Arcanos"
                >
                    "🧱 Construir"
                </button>

                <button 
                    class="tool-btn" 
                    class:active=move || active_tool.get() == "ruler"
                    on:click=move |_| {
                        set_active_tool.set("ruler");
                        on_reset_ruler.call(());
                    }
                    title="Medir Distância & Alcance de Esferas"
                >
                    "📏 Régua"
                </button>

                <button 
                    class="tool-btn action-spawn-btn" 
                    on:click=move |_| on_spawn_party.call(())
                    title="Importar todos os personagens da sala para o mapa"
                >
                    "👥 Cabala"
                </button>

                <button 
                    class="tool-btn action-npc-btn" 
                    on:click=move |_| on_open_npc_modal.call(())
                    title="Criar Token de NPC ou Inimigo"
                >
                    "👾 + NPC"
                </button>
            </div>

            // Ação de Remover Token Selecionado Direto na Barra
            {move || if let Some(t_name) = selected_token_name.get() {
                let sel_id_btn = selected_token_id.get().unwrap_or_default();
                view! {
                    <div class="toolbar-selected-action">
                        <span class="selected-token-info">"🎯 " {t_name}</span>
                        <button
                            class="tool-btn token-toolbar-del-btn"
                            on:click=move |_| on_remove_token.call(sel_id_btn.clone())
                            title="Excluir este token do mapa"
                        >
                            "🗑️ Remover"
                        </button>
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}

            // Controles de Zoom & Auto-Ajuste
            <div class="toolbar-zoom-group">
                <button 
                    class="zoom-btn" 
                    on:click=move |_| set_zoom_scale.update(|z| *z = (*z - 0.15).max(0.35))
                    title="Afastar Zoom (-)"
                >
                    "−"
                </button>
                <button 
                    class="zoom-btn zoom-val-btn" 
                    on:click=move |_| set_zoom_scale.set(1.0)
                    title="Resetar Zoom para 100%"
                >
                    {move || format!("{:.0}%", zoom_scale.get() * 100.0)}
                </button>
                <button 
                    class="zoom-btn" 
                    on:click=move |_| set_zoom_scale.update(|z| *z = (*z + 0.15).min(2.5))
                    title="Aproximar Zoom (+)"
                >
                    "+"
                </button>
                <button 
                    class="zoom-btn fit-btn" 
                    on:click=move |_| on_fit_to_screen.call(())
                    title="Ajustar Tabuleiro à Tela (Auto-Fit)"
                >
                    "⛶ Ajustar"
                </button>
            </div>

            <div class="toolbar-center">
                <span class="grid-shape-indicator">
                    {move || match grid_shape.get() {
                        GridShape::Square => "🔲 Quadrada",
                        GridShape::HexPointy => "⬡ Hexagonal",
                        GridShape::HexFlat => "⬡ Hex (Flat)",
                    }}
                </span>
                <span class="grid-dimensions-badge">
                    {move || format!("{}x{}", cols.get(), rows.get())}
                </span>
            </div>

            <div class="toolbar-right">
                {move || save_msg.get().map(|msg| view! { <span class="save-status-badge">{msg}</span> })}

                {move || if is_gm.get() {
                    view! {
                        <button 
                            class="tool-btn gm-settings-btn" 
                            on:click=move |_| on_open_gm_modal.call(())
                            title="Configurar Tabuleiro, Grade e Fundo"
                        >
                            "⚙️ Tabuleiro"
                        </button>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }}
            </div>
        </div>

        // Sub-Barra da Paleta de Construção (Quando active_tool == "build")
        {move || if active_tool.get() == "build" {
            view! {
                <div class="build-palette-bar">
                    <span class="palette-title">"Construção:"</span>
                    
                    <button
                        class="palette-item-btn"
                        class:active=move || selected_build_type.get() == "wall"
                        on:click=move |_| set_selected_build_type.set("wall".to_string())
                        title="Muro / Parede de Alvenaria (Bloqueia movimento e visão)"
                    >
                        <span class="palette-icon">"🧱"</span>
                        <span class="palette-label">"Muro"</span>
                    </button>

                    <button
                        class="palette-item-btn"
                        class:active=move || selected_build_type.get() == "door"
                        on:click=move |_| set_selected_build_type.set("door".to_string())
                        title="Porta / Passagem (Bloqueia visão, permite passagem)"
                    >
                        <span class="palette-icon">"🚪"</span>
                        <span class="palette-label">"Porta"</span>
                    </button>

                    <button
                        class="palette-item-btn"
                        class:active=move || selected_build_type.get() == "cover"
                        on:click=move |_| set_selected_build_type.set("cover".to_string())
                        title="Barricada / Cobertura Tática (+2 Dificuldade para tiros)"
                    >
                        <span class="palette-icon">"🛡️"</span>
                        <span class="palette-label">"Cobertura"</span>
                    </button>

                    <button
                        class="palette-item-btn"
                        class:active=move || selected_build_type.get() == "water"
                        on:click=move |_| set_selected_build_type.set("water".to_string())
                        title="Água Profunda / Terreno Difícil (Custo dobrado de movimento)"
                    >
                        <span class="palette-icon">"🌊"</span>
                        <span class="palette-label">"Água"</span>
                    </button>

                    <button
                        class="palette-item-btn"
                        class:active=move || selected_build_type.get() == "fire"
                        on:click=move |_| set_selected_build_type.set("fire".to_string())
                        title="Fogo Mágico / Campo de Chamas (Dano por turno)"
                    >
                        <span class="palette-icon">"🔥"</span>
                        <span class="palette-label">"Fogo"</span>
                    </button>

                    <button
                        class="palette-item-btn"
                        class:active=move || selected_build_type.get() == "ward"
                        on:click=move |_| set_selected_build_type.set("ward".to_string())
                        title="Selo Arcano / Círculo de Proteção (Barreira mágica)"
                    >
                        <span class="palette-icon">"🔯"</span>
                        <span class="palette-label">"Selo Arcano"</span>
                    </button>

                    <button
                        class="palette-item-btn palette-eraser"
                        class:active=move || selected_build_type.get() == "eraser"
                        on:click=move |_| set_selected_build_type.set("eraser".to_string())
                        title="Demolir / Borracha (Clique ou arraste na célula para apagar)"
                    >
                        <span class="palette-icon">"🧹"</span>
                        <span class="palette-label">"Demolir"</span>
                    </button>

                    <span class="palette-hint">"💡 Clique para construir/toggle • Botão direito para demolir • Segure e arraste para desenhar"</span>
                </div>
            }.into_view()
        } else {
            view! {}.into_view()
        }}
    }
}
