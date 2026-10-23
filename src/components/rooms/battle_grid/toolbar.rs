//! Barra de ferramentas do mapa e paletas de montagem.

use leptos::*;
use crate::components::common::Callback;
use crate::rooms::{MapLayer, PropKind, TerrainKind, WallKind};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tool {
    /// Mover tokens e objetos, abrir portas, arrastar o mapa
    Move,
    Terrain,
    Walls,
    Props,
    Ruler,
}

/// Ferramenta atual e o que cada pincel aplica.
#[derive(Clone, Copy)]
pub struct ToolState {
    pub tool: RwSignal<Tool>,
    /// `None` = borracha
    pub terrain: RwSignal<Option<TerrainKind>>,
    pub brush_size: RwSignal<u8>,
    /// `None` = borracha
    pub wall: RwSignal<Option<WallKind>>,
    pub prop: RwSignal<PropKind>,
    pub zoom: RwSignal<f32>,
}

impl ToolState {
    pub fn new() -> Self {
        Self {
            tool: create_rw_signal(Tool::Move),
            terrain: create_rw_signal(Some(TerrainKind::Floor)),
            brush_size: create_rw_signal(1),
            wall: create_rw_signal(Some(WallKind::Wall)),
            prop: create_rw_signal(PropKind::Table),
            zoom: create_rw_signal(1.0),
        }
    }
}

#[component]
fn ToolButton(tools: ToolState, tool: Tool, icon: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <button
            type="button"
            class="grid-tool-btn"
            class:active=move || tools.tool.get() == tool
            title=label
            on:click=move |_| tools.tool.set(tool)
        >
            <span class="grid-tool-icon" aria-hidden="true">{icon}</span>
            <span class="grid-tool-label">{label}</span>
        </button>
    }
}

#[component]
pub fn GridToolbar(
    tools: ToolState,
    #[prop(into)] is_gm: Signal<bool>,
    #[prop(into)] status: Signal<Option<(bool, String)>>,
    on_fit: Callback<()>,
    on_spawn_party: Callback<()>,
    on_open_npc: Callback<()>,
    on_open_background: Callback<()>,
    on_open_config: Callback<()>,
    on_clear: Callback<MapLayer>,
) -> impl IntoView {
    let zoom_by = move |factor: f32| tools.zoom.update(|z| *z = (*z * factor).clamp(0.2, 3.0));
    let on_clear = store_value(on_clear);
    let clear = move |layer: MapLayer, what: &'static str| {
        #[cfg(target_arch = "wasm32")]
        {
            let ok = web_sys::window()
                .and_then(|w| w.confirm_with_message(&format!("Apagar {} do mapa inteiro?", what)).ok())
                .unwrap_or(false);
            if !ok {
                return;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = what;
        on_clear.with_value(|c| c.call(layer));
    };

    view! {
        <div class="grid-toolbar">
            <div class="grid-toolbar-group">
                <ToolButton tools=tools tool=Tool::Move icon="✋" label="Mover" />
                <Show when=move || is_gm.get()>
                    <div class="grid-toolbar-inline">
                        <ToolButton tools=tools tool=Tool::Terrain icon="🎨" label="Terreno" />
                        <ToolButton tools=tools tool=Tool::Walls icon="🧱" label="Paredes" />
                        <ToolButton tools=tools tool=Tool::Props icon="🪑" label="Objetos" />
                    </div>
                </Show>
                <ToolButton tools=tools tool=Tool::Ruler icon="📏" label="Régua" />
            </div>

            <Show when=move || is_gm.get()>
                <div class="grid-toolbar-group">
                    <button type="button" class="grid-action-btn" title="Colocar no mapa os personagens da sala" on:click={let c = on_spawn_party.clone(); move |_| c.call(())}>"👥 Personagens"</button>
                    <button type="button" class="grid-action-btn" on:click={let c = on_open_npc.clone(); move |_| c.call(())}>"👹 NPC"</button>
                    <button type="button" class="grid-action-btn" on:click={let c = on_open_background.clone(); move |_| c.call(())}>"🖼️ Fundo"</button>
                    <button type="button" class="grid-action-btn" on:click={let c = on_open_config.clone(); move |_| c.call(())}>"⚙️ Grade"</button>
                    <details class="grid-clear-menu">
                        <summary class="grid-action-btn">"🧹 Limpar"</summary>
                        <div class="grid-clear-options">
                            <button type="button" on:click=move |_| clear(MapLayer::Terrain, "todo o terreno")>"Terreno"</button>
                            <button type="button" on:click=move |_| clear(MapLayer::Walls, "todas as paredes e portas")>"Paredes e portas"</button>
                            <button type="button" on:click=move |_| clear(MapLayer::Props, "todos os objetos")>"Objetos"</button>
                            <button type="button" on:click=move |_| clear(MapLayer::Npcs, "todos os NPCs")>"NPCs"</button>
                        </div>
                    </details>
                </div>
            </Show>

            <div class="grid-toolbar-group grid-zoom">
                <button type="button" class="grid-icon-btn" aria-label="Diminuir zoom" on:click=move |_| zoom_by(1.0 / 1.25)>"−"</button>
                <span class="grid-zoom-value">{move || format!("{:.0}%", tools.zoom.get() * 100.0)}</span>
                <button type="button" class="grid-icon-btn" aria-label="Aumentar zoom" on:click=move |_| zoom_by(1.25)>"+"</button>
                <button type="button" class="grid-icon-btn" title="Ajustar à tela" on:click=move |_| on_fit.call(())>"⤢"</button>
                <span class="grid-status" class:is-error=move || status.get().is_some_and(|(e, _)| e)>
                    {move || status.get().map(|(_, m)| m).unwrap_or_default()}
                </span>
            </div>
        </div>

        <Show when=move || is_gm.get() && tools.tool.get() == Tool::Terrain>
            <div class="grid-palette">
                {TerrainKind::ALL.iter().map(|&kind| view! {
                    <button
                        type="button"
                        class="grid-swatch"
                        class:active=move || tools.terrain.get() == Some(kind)
                        title=kind.label()
                        on:click=move |_| tools.terrain.set(Some(kind))
                    >
                        <span class="grid-swatch-color" style=format!("background:{}", kind.color())></span>
                        <span class="grid-swatch-label">{kind.label()}</span>
                    </button>
                }).collect_view()}
                <button type="button" class="grid-swatch" class:active=move || tools.terrain.get().is_none() on:click=move |_| tools.terrain.set(None)>
                    <span class="grid-swatch-color grid-eraser">"🧽"</span>
                    <span class="grid-swatch-label">"Apagar"</span>
                </button>
                <div class="grid-brush-size" role="group" aria-label="Tamanho do pincel">
                    {[1u8, 2, 3].into_iter().map(|size| view! {
                        <button type="button" class:active=move || tools.brush_size.get() == size on:click=move |_| tools.brush_size.set(size)>
                            {format!("Pincel {}", size)}
                        </button>
                    }).collect_view()}
                </div>
            </div>
        </Show>

        <Show when=move || is_gm.get() && tools.tool.get() == Tool::Walls>
            <div class="grid-palette">
                {WallKind::ALL.iter().map(|&kind| view! {
                    <button type="button" class="grid-swatch" class:active=move || tools.wall.get() == Some(kind) on:click=move |_| tools.wall.set(Some(kind))>
                        <span class="grid-swatch-label">{kind.label()}</span>
                    </button>
                }).collect_view()}
                <button type="button" class="grid-swatch" class:active=move || tools.wall.get().is_none() on:click=move |_| tools.wall.set(None)>
                    <span class="grid-swatch-label">"🧽 Apagar"</span>
                </button>
                <p class="grid-palette-hint">"Toque ou arraste perto da borda das células. No modo Mover, toque numa porta para abrir ou fechar."</p>
            </div>
        </Show>

        <Show when=move || is_gm.get() && tools.tool.get() == Tool::Props>
            <div class="grid-palette">
                {PropKind::ALL.iter().map(|&kind| view! {
                    <button type="button" class="grid-swatch grid-prop-swatch" class:active=move || tools.prop.get() == kind title=kind.label() on:click=move |_| tools.prop.set(kind)>
                        <span class="grid-prop-icon">{kind.icon()}</span>
                        <span class="grid-swatch-label">{kind.label()}</span>
                    </button>
                }).collect_view()}
                <p class="grid-palette-hint">"Toque numa célula para colocar. Use Mover para arrastar, girar ou remover."</p>
            </div>
        </Show>
    }
}
