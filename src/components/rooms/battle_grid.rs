use leptos::*;
use crate::rooms::{update_room_map, GridShape, MapStructure, MapToken, RoomMapData, RoomSheetSummary};

#[component]
pub fn BattleGrid(
    room_id: Signal<String>,
    is_gm: Signal<bool>,
    map_data: Signal<RoomMapData>,
    set_map_data: WriteSignal<RoomMapData>,
    room_sheets: Signal<Vec<RoomSheetSummary>>,
) -> impl IntoView {
    let (active_tool, set_active_tool) = create_signal("select"); // "select" | "ruler" | "build"
    let (selected_build_type, set_selected_build_type) = create_signal("wall".to_string());
    let (selected_token_id, set_selected_token_id) = create_signal(Option::<String>::None);
    let (hovered_cell, set_hovered_cell) = create_signal(Option::<(f32, f32)>::None);
    let (dragged_token_id, set_dragged_token_id) = create_signal(Option::<String>::None);

    // Zoom Signals
    let (zoom_scale, set_zoom_scale) = create_signal(1.0f32);

    // Ruler Signals
    let (ruler_start, set_ruler_start) = create_signal(Option::<(f32, f32)>::None);
    let (ruler_end, set_ruler_end) = create_signal(Option::<(f32, f32)>::None);

    // Modals
    let (show_gm_modal, set_show_gm_modal) = create_signal(false);
    let (show_npc_modal, set_show_npc_modal) = create_signal(false);

    // GM Config Form Signals
    let (cfg_shape, set_cfg_shape) = create_signal(GridShape::Square);
    let (cfg_cols, set_cfg_cols) = create_signal(20u32);
    let (cfg_rows, set_cfg_rows) = create_signal(20u32);
    let (cfg_cell_size, set_cfg_cell_size) = create_signal(50u32);
    let (cfg_grid_color, set_cfg_grid_color) = create_signal("rgba(99, 102, 241, 0.35)".to_string());
    let (cfg_bg_url, set_cfg_bg_url) = create_signal(String::new());
    let (cfg_bg_opacity, set_cfg_bg_opacity) = create_signal(0.85f32);
    let (cfg_meters_per_cell, set_cfg_meters_per_cell) = create_signal(1.5f32);

    // NPC Form Signals
    let (npc_name, set_npc_name) = create_signal(String::new());
    let (npc_color, set_npc_color) = create_signal("#ef4444".to_string());
    let (npc_size, set_npc_size) = create_signal(1.0f32);

    // Feedback & Auto-save
    let (save_msg, set_save_msg) = create_signal(Option::<String>::None);
    let (_is_saving, set_is_saving) = create_signal(false);

    // Sincroniza configurações locais quando o mapa remoto muda (apenas se o modal de config estiver fechado)
    create_effect(move |_| {
        let current = map_data.get();
        if !show_gm_modal.get() {
            set_cfg_shape.set(current.grid_shape);
            set_cfg_cols.set(current.cols);
            set_cfg_rows.set(current.rows);
            set_cfg_cell_size.set(current.cell_size);
            set_cfg_grid_color.set(current.grid_color);
            set_cfg_bg_url.set(current.bg_image_url);
            set_cfg_bg_opacity.set(current.bg_opacity);
            set_cfg_meters_per_cell.set(current.meters_per_cell);
        }
    });

    // Salvar Mapa no Servidor
    let save_map_to_server = move |data_to_save: RoomMapData| {
        let r_id = room_id.get();
        if r_id.is_empty() {
            return;
        }

        set_is_saving.set(true);
        set_save_msg.set(Some("Salvando...".to_string()));

        spawn_local(async move {
            match update_room_map(r_id, data_to_save).await {
                Ok(_) => {
                    let _ = set_is_saving.try_set(false);
                    let _ = set_save_msg.try_set(Some("✓ Salvo".to_string()));
                }
                Err(e) => {
                    let _ = set_is_saving.try_set(false);
                    let _ = set_save_msg.try_set(Some(format!("Erro: {}", e)));
                }
            }
        });
    };

    // Mover token por ID diretamente para célula
    let move_token_to_cell = move |token_id: &str, col: f32, row: f32| {
        let mut current = map_data.get();
        if let Some(token) = current.tokens.iter_mut().find(|t| t.id == token_id) {
            token.grid_col = col;
            token.grid_row = row;
            set_map_data.set(current.clone());
            save_map_to_server(current);
        }
    };

    // Mover token selecionado
    let move_selected_token = move |col: f32, row: f32| {
        if let Some(token_id) = selected_token_id.get() {
            move_token_to_cell(&token_id, col, row);
        }
    };

    // Remover Token
    let remove_token = move |token_id: String| {
        let mut current = map_data.get();
        current.tokens.retain(|t| t.id != token_id);
        if selected_token_id.get().as_deref() == Some(&token_id) {
            set_selected_token_id.set(None);
        }
        set_map_data.set(current.clone());
        save_map_to_server(current);
    };

    // Alternar visibilidade de Token (GM)
    let toggle_token_hidden = move |token_id: String| {
        let mut current = map_data.get();
        if let Some(token) = current.tokens.iter_mut().find(|t| t.id == token_id) {
            token.is_hidden = !token.is_hidden;
            set_map_data.set(current.clone());
            save_map_to_server(current);
        }
    };

    // Construir ou Demolir Estrutura na célula (col, row)
    let place_or_erase_structure = move |col: f32, row: f32| {
        let mut current = map_data.get();
        let build_type = selected_build_type.get();

        if build_type == "eraser" {
            // Remove estruturas e/ou tokens na célula
            current.structures.retain(|s| !((s.grid_col - col).abs() < 0.1 && (s.grid_row - row).abs() < 0.1));
            current.tokens.retain(|t| !((t.grid_col - col).abs() < 0.1 && (t.grid_row - row).abs() < 0.1));
        } else {
            // Remove estrutura pré-existente na mesma célula se houver
            current.structures.retain(|s| !((s.grid_col - col).abs() < 0.1 && (s.grid_row - row).abs() < 0.1));

            let (name, color, icon, blocks_m, blocks_s, opacity) = match build_type.as_str() {
                "wall" => ("Muro de Alvenaria", "#334155", "🧱", true, true, 0.94),
                "door" => ("Porta", "#854d0e", "🚪", false, true, 0.92),
                "cover" => ("Barricada / Cobertura", "#78350f", "🛡️", false, false, 0.88),
                "water" => ("Água / Terreno Difícil", "#0284c7", "🌊", false, false, 0.70),
                "fire" => ("Fogo Mágico", "#dc2626", "🔥", false, false, 0.80),
                "ward" => ("Círculo de Proteção", "#7c3aed", "🔯", false, false, 0.75),
                _ => ("Estrutura", "#475569", "📦", true, false, 0.80),
            };

            let new_struct = MapStructure {
                id: format!("struct-{}", uuid::Uuid::new_v4()),
                structure_type: build_type,
                name: name.to_string(),
                color: color.to_string(),
                icon: icon.to_string(),
                grid_col: col,
                grid_row: row,
                blocks_movement: blocks_m,
                blocks_sight: blocks_s,
                opacity,
            };

            current.structures.push(new_struct);
        }

        set_map_data.set(current.clone());
        save_map_to_server(current);
    };

    // Auto-Spawn dos Personagens da Sala (Cabala)
    let on_spawn_party = move |_| {
        let sheets = room_sheets.get();
        let mut current_map = map_data.get();
        let existing_sheet_ids: Vec<String> = current_map
            .tokens
            .iter()
            .filter_map(|t| t.sheet_id.clone())
            .collect();

        let mut next_col = 0.0f32;
        let mut next_row = 0.0f32;

        for sheet in sheets {
            if !existing_sheet_ids.contains(&sheet.id) {
                let badge_color = if sheet.health_badge_class.contains("bruised") || sheet.health_badge_class.contains("hurt") {
                    "#eab308".to_string() // Amarelo Ferido
                } else if sheet.health_badge_class.contains("incapacitated") || sheet.health_badge_class.contains("crippled") {
                    "#ef4444".to_string() // Vermelho Crítico
                } else {
                    "#22c55e".to_string() // Verde Saudável
                };

                let new_token = MapToken {
                    id: format!("token-{}", uuid::Uuid::new_v4()),
                    sheet_id: Some(sheet.id.clone()),
                    name: sheet.name.clone(),
                    avatar_url: sheet.photo_url.clone(),
                    grid_col: next_col,
                    grid_row: next_row,
                    size_cells: 1.0,
                    is_npc: false,
                    is_hidden: sheet.is_hidden,
                    color: badge_color,
                    health_status: Some(sheet.health_label.clone()),
                };

                current_map.tokens.push(new_token);

                next_col += 1.0;
                if next_col >= current_map.cols as f32 {
                    next_col = 0.0;
                    next_row += 1.0;
                }
            }
        }

        set_map_data.set(current_map.clone());
        save_map_to_server(current_map);
    };

    // Adicionar NPC Token
    let on_add_npc = move |_| {
        let mut name = npc_name.get().trim().to_string();
        if name.is_empty() {
            let current_count = map_data.with(|m| m.tokens.iter().filter(|t| t.is_npc).count() + 1);
            name = format!("Inimigo #{}", current_count);
        }

        let mut current_map = map_data.get();
        
        let mut spawn_col = 0.0f32;
        let mut spawn_row = 0.0f32;
        for token in &current_map.tokens {
            if (token.grid_col - spawn_col).abs() < 0.1 && (token.grid_row - spawn_row).abs() < 0.1 {
                spawn_col += 1.0;
                if spawn_col >= current_map.cols as f32 {
                    spawn_col = 0.0;
                    spawn_row += 1.0;
                }
            }
        }

        let new_token = MapToken {
            id: format!("npc-{}", uuid::Uuid::new_v4()),
            sheet_id: None,
            name,
            avatar_url: String::new(),
            grid_col: spawn_col,
            grid_row: spawn_row,
            size_cells: npc_size.get(),
            is_npc: true,
            is_hidden: false,
            color: npc_color.get(),
            health_status: Some("NPC".to_string()),
        };

        current_map.tokens.push(new_token);
        set_map_data.set(current_map.clone());
        save_map_to_server(current_map);

        set_npc_name.set(String::new());
        set_show_npc_modal.set(false);
    };

    // Salvar configurações do GM
    let on_save_gm_config = move |_| {
        let mut current = map_data.get();
        current.grid_shape = cfg_shape.get();
        current.cols = cfg_cols.get().clamp(5, 60);
        current.rows = cfg_rows.get().clamp(5, 60);
        current.cell_size = cfg_cell_size.get().clamp(25, 120);
        current.grid_color = cfg_grid_color.get();
        current.bg_image_url = cfg_bg_url.get().trim().to_string();
        current.bg_opacity = cfg_bg_opacity.get().clamp(0.1, 1.0);
        current.meters_per_cell = cfg_meters_per_cell.get().clamp(0.5, 10.0);

        set_map_data.set(current.clone());
        save_map_to_server(current);
        set_show_gm_modal.set(false);
    };

    // Abrir Modal de Configurações
    let on_open_gm_modal = move |_| {
        let current = map_data.get_untracked();
        set_cfg_shape.set(current.grid_shape);
        set_cfg_cols.set(current.cols);
        set_cfg_rows.set(current.rows);
        set_cfg_cell_size.set(current.cell_size);
        set_cfg_grid_color.set(current.grid_color);
        set_cfg_bg_url.set(current.bg_image_url);
        set_cfg_bg_opacity.set(current.bg_opacity);
        set_cfg_meters_per_cell.set(current.meters_per_cell);
        set_show_gm_modal.set(true);
    };

    // Funções de Geometria e Projeção
    let cell_size_val = move || map_data.with(|m| m.cell_size as f32);
    let shape_val = move || map_data.with(|m| m.grid_shape);
    let cols_val = move || map_data.with(|m| m.cols);
    let rows_val = move || map_data.with(|m| m.rows);

    // Dimensões do Canvas SVG
    let board_dimensions = move || {
        let cols = cols_val() as f32;
        let rows = rows_val() as f32;
        let cs = cell_size_val();
        let shape = shape_val();

        match shape {
            GridShape::Square => {
                let w = cols * cs;
                let h = rows * cs;
                (w, h)
            }
            GridShape::HexPointy => {
                let r = cs / 2.0;
                let w_hex = 1.7320508 * r; // sqrt(3) * R
                let total_w = cols * w_hex + (w_hex / 2.0);
                let total_h = rows * (1.5 * r) + (r * 0.5);
                (total_w, total_h)
            }
            GridShape::HexFlat => {
                let r = cs / 2.0;
                let h_hex = 1.7320508 * r;
                let total_w = cols * (1.5 * r) + (r * 0.5);
                let total_h = rows * h_hex + (h_hex / 2.0);
                (total_w, total_h)
            }
        }
    };

    // Auto-Fit: dimensiona o tabuleiro para caber na tela sem rolagem
    let on_fit_to_screen = move |_| {
        let (board_w, board_h) = board_dimensions();
        if board_w > 0.0 && board_h > 0.0 {
            let fit_w = 960.0 / board_w;
            let fit_h = 600.0 / board_h;
            let best_scale = fit_w.min(fit_h).clamp(0.4, 1.25);
            set_zoom_scale.set(best_scale);
        }
    };

    // Cálculo do centro de uma célula (col, row) -> (px_x, px_y)
    let get_cell_center = move |col: f32, row: f32| -> (f32, f32) {
        let cs = cell_size_val();
        let shape = shape_val();

        match shape {
            GridShape::Square => {
                let cx = col * cs + (cs / 2.0);
                let cy = row * cs + (cs / 2.0);
                (cx, cy)
            }
            GridShape::HexPointy => {
                let r = cs / 2.0;
                let w_hex = 1.7320508 * r;
                let offset_x = if (row as i32) % 2 != 0 { w_hex / 2.0 } else { 0.0 };
                let cx = col * w_hex + offset_x + (w_hex / 2.0);
                let cy = row * (1.5 * r) + r;
                (cx, cy)
            }
            GridShape::HexFlat => {
                let r = cs / 2.0;
                let h_hex = 1.7320508 * r;
                let offset_y = if (col as i32) % 2 != 0 { h_hex / 2.0 } else { 0.0 };
                let cx = col * (1.5 * r) + r;
                let cy = row * h_hex + offset_y + (h_hex / 2.0);
                (cx, cy)
            }
        }
    };

    // Gera string de pontos para polígono SVG de hexágono
    let get_hex_points = move |cx: f32, cy: f32, r: f32| -> String {
        let shape = shape_val();
        let mut pts = String::new();
        for i in 0..6 {
            let angle_deg = match shape {
                GridShape::HexPointy => 60.0 * (i as f32) - 30.0,
                _ => 60.0 * (i as f32),
            };
            let angle_rad = angle_deg.to_radians();
            let px = cx + r * angle_rad.cos();
            let py = cy + r * angle_rad.sin();
            if i > 0 {
                pts.push(' ');
            }
            pts.push_str(&format!("{:.1},{:.1}", px, py));
        }
        pts
    };

    // Medição da Régua
    let calculate_ruler_info = move || {
        if let (Some(start), Some(end)) = (ruler_start.get(), ruler_end.get()) {
            let (x1, y1) = get_cell_center(start.0, start.1);
            let (x2, y2) = get_cell_center(end.0, end.1);
            let dx = x2 - x1;
            let dy = y2 - y1;
            let dist_px = (dx * dx + dy * dy).sqrt();
            let cs = cell_size_val();
            let cells = dist_px / cs;
            let meters_per_cell = map_data.with(|m| m.meters_per_cell);
            let meters = cells * meters_per_cell;

            let (range_name, m20_sphere) = if meters <= 1.5 {
                ("Toque", "Toque Corporal / Corpo a Corpo")
            } else if meters <= 5.0 {
                ("Curto (1-3 Passos)", "Proximidade Imediata")
            } else if meters <= 20.0 {
                ("Médio (Arremesso)", "Linha Direta de Visão")
            } else if meters <= 50.0 {
                ("Longo (Armas de Fogo)", "Alcance Balístico")
            } else {
                ("Extremo (Além de 50m)", "Requer Correspondência 2+")
            };

            Some((cells, meters, range_name, m20_sphere))
        } else {
            None
        }
    };

    let selected_token_name = Signal::derive(move || {
        let sel_id = selected_token_id.get();
        if let Some(id) = sel_id {
            map_data.with(|m| m.tokens.iter().find(|t| t.id == id).map(|t| t.name.clone()))
        } else {
            None
        }
    });

    view! {
        <div class="battle-grid-wrapper">
            // ─── Barra de Ferramentas Superior ───────────────────────────
            <div class="battle-grid-toolbar">
                <div class="toolbar-left">
                    <button 
                        class="tool-btn" 
                        class:active=move || active_tool.get() == "select"
                        on:click=move |_| {
                            set_active_tool.set("select");
                            set_ruler_start.set(None);
                            set_ruler_end.set(None);
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
                            set_ruler_start.set(None);
                            set_ruler_end.set(None);
                            set_selected_token_id.set(None);
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
                            set_selected_token_id.set(None);
                        }
                        title="Medir Distância & Alcance de Esferas"
                    >
                        "📏 Régua"
                    </button>

                    <button 
                        class="tool-btn action-spawn-btn" 
                        on:click=on_spawn_party
                        title="Importar todos os personagens da sala para o mapa"
                    >
                        "👥 Cabala"
                    </button>

                    <button 
                        class="tool-btn action-npc-btn" 
                        on:click=move |_| set_show_npc_modal.set(true)
                        title="Criar Token de NPC ou Inimigo"
                    >
                        "👾 + NPC"
                    </button>
                </div>

                // ─── Ação de Remover Token Selecionado Direto na Barra ───────
                {move || if let Some(t_name) = selected_token_name.get() {
                    let sel_id_btn = selected_token_id.get().unwrap_or_default();
                    view! {
                        <div class="toolbar-selected-action">
                            <span class="selected-token-info">"🎯 " {t_name}</span>
                            <button
                                class="tool-btn token-toolbar-del-btn"
                                on:click=move |_| remove_token(sel_id_btn.clone())
                                title="Excluir este token do mapa"
                            >
                                "🗑️ Remover"
                            </button>
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }}

                // ─── Controles de Zoom & Auto-Ajuste (Sem Barra de Rolagem) ───
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
                        on:click=on_fit_to_screen
                        title="Ajustar Tabuleiro à Tela (Auto-Fit)"
                    >
                        "⛶ Ajustar"
                    </button>
                </div>

                <div class="toolbar-center">
                    <span class="grid-shape-indicator">
                        {move || match shape_val() {
                            GridShape::Square => "🔲 Quadrada",
                            GridShape::HexPointy => "⬡ Hexagonal",
                            GridShape::HexFlat => "⬡ Hex (Flat)",
                        }}
                    </span>
                    <span class="grid-dimensions-badge">
                        {move || format!("{}x{}", cols_val(), rows_val())}
                    </span>
                </div>

                <div class="toolbar-right">
                    {move || save_msg.get().map(|msg| view! { <span class="save-status-badge">{msg}</span> })}

                    {move || if is_gm.get() {
                        view! {
                            <button 
                                class="tool-btn gm-settings-btn"
                                on:click=on_open_gm_modal
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

            // ─── Sub-Barra da Paleta de Construção (Quando active_tool == "build") ───
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
                            title="Demolir / Borracha (Clique na célula para apagar a estrutura)"
                        >
                            <span class="palette-icon">"🧹"</span>
                            <span class="palette-label">"Demolir"</span>
                        </button>

                        <span class="palette-hint">"💡 Clique em qualquer célula para construir ou apagar."</span>
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}

            // ─── Display de Medição da Régua Tática ──────────────────────
            {move || calculate_ruler_info().map(|(cells, meters, range_name, m20_sphere)| {
                view! {
                    <div class="ruler-banner">
                        <div class="ruler-info-col">
                            <span class="ruler-val">{format!("{:.1} m", meters)}</span>
                            <span class="ruler-sub">{format!("({:.1} células / {:.0} passos)", cells, meters / 0.8)}</span>
                        </div>
                        <div class="ruler-divider"></div>
                        <div class="ruler-range-col">
                            <span class="ruler-range-tag">{range_name}</span>
                            <span class="ruler-m20-tag">"🔮 " {m20_sphere}</span>
                        </div>
                    </div>
                }
            })}

            // ─── Viewport do Mapa Interativo (SVG + DOM Sem Rolagem) ───────
            <div class="battle-grid-viewport">
                {move || {
                    let (board_w, board_h) = board_dimensions();
                    let cols = cols_val();
                    let rows = rows_val();
                    let cs = cell_size_val();
                    let shape = shape_val();
                    let grid_color = map_data.with(|m| m.grid_color.clone());
                    let bg_url = map_data.with(|m| m.bg_image_url.clone());
                    let bg_opacity = map_data.with(|m| m.bg_opacity);
                    let has_bg = !bg_url.is_empty();
                    let current_zoom = zoom_scale.get();

                    let container_style = format!(
                        "width: {}px; height: {}px; transform: scale({}); transform-origin: center center;",
                        board_w, board_h, current_zoom
                    );
                    let svg_style = format!("width: {}px; height: {}px;", board_w, board_h);

                    view! {
                        <div class="battle-board-container" style=container_style>
                            // Imagem de Fundo do Mapa
                            {if has_bg {
                                let bg_style = format!(
                                    "background-image: url('{}'); opacity: {}; width: {}px; height: {}px;",
                                    bg_url, bg_opacity, board_w, board_h
                                );
                                view! { <div class="battle-board-bg" style=bg_style></div> }.into_view()
                            } else {
                                view! { <div class="battle-board-bg-empty"></div> }.into_view()
                            }}

                            // ─── Camada DOM de Estruturas Construídas (Altamente Visível) ──
                            <div class="battle-structures-layer">
                                {move || {
                                    let structs = map_data.with(|m| m.structures.clone());
                                    let current_shape = shape_val();
                                    let cur_cs = cell_size_val();

                                    structs.into_iter().map(|s| {
                                        let (cx, cy) = get_cell_center(s.grid_col, s.grid_row);
                                        let s_col = s.grid_col;
                                        let s_row = s.grid_row;
                                        let s_color = s.color.clone();
                                        let s_icon = s.icon.clone();
                                        let s_opacity = s.opacity;

                                        if current_shape == GridShape::Square {
                                            let left = s.grid_col * cur_cs + 1.0;
                                            let top = s.grid_row * cur_cs + 1.0;
                                            let size = cur_cs - 2.0;
                                            let font_sz = cur_cs * 0.48;

                                            let struct_style = format!(
                                                "left: {}px; top: {}px; width: {}px; height: {}px; background-color: {}; opacity: {}; font-size: {}px;",
                                                left, top, size, size, s_color, s_opacity, font_sz
                                            );

                                            view! {
                                                <div
                                                    class="map-structure-block"
                                                    style=struct_style
                                                    title=s.name.clone()
                                                    on:click=move |ev: ev::MouseEvent| {
                                                        ev.stop_propagation();
                                                        let cur_tool = active_tool.get();
                                                        if cur_tool == "build" {
                                                            place_or_erase_structure(s_col, s_row);
                                                        }
                                                    }
                                                >
                                                    <span class="structure-icon">{s_icon}</span>
                                                </div>
                                            }.into_view()
                                        } else {
                                            let size = cur_cs * 0.88;
                                            let half = size / 2.0;
                                            let font_sz = cur_cs * 0.44;
                                            let struct_style = format!(
                                                "left: {}px; top: {}px; width: {}px; height: {}px; background-color: {}; opacity: {}; font-size: {}px;",
                                                cx - half, cy - half, size, size, s_color, s_opacity, font_sz
                                            );

                                            view! {
                                                <div
                                                    class="map-structure-block hex-structure"
                                                    style=struct_style
                                                    title=s.name.clone()
                                                    on:click=move |ev: ev::MouseEvent| {
                                                        ev.stop_propagation();
                                                        let cur_tool = active_tool.get();
                                                        if cur_tool == "build" {
                                                            place_or_erase_structure(s_col, s_row);
                                                        }
                                                    }
                                                >
                                                    <span class="structure-icon">{s_icon}</span>
                                                </div>
                                            }.into_view()
                                        }
                                    }).collect_view()
                                }}
                            </div>

                            // ─── Renderizador SVG da Grade ─────────────────
                            <svg class="battle-grid-svg" style=svg_style.clone()>
                                <defs>
                                    <pattern id="ruler-pattern" width="10" height="10" patternUnits="userSpaceOnUse">
                                        <line x1="0" y1="0" x2="10" y2="10" stroke="#f59e0b" stroke-width="1.5" />
                                    </pattern>
                                </defs>

                                // ─── Grade Interativa de Células (Square) ─
                                {if shape == GridShape::Square {
                                    let stroke = grid_color.clone();
                                    view! {
                                        <g class="square-grid-group">
                                            {(0..rows).flat_map(move |r| {
                                                let r_f = r as f32;
                                                let stroke_row = stroke.clone();
                                                (0..cols).map(move |c| {
                                                    let c_f = c as f32;
                                                    let x = c_f * cs;
                                                    let y = r_f * cs;
                                                    let is_hovered = hovered_cell.get() == Some((c_f, r_f));
                                                    let stroke_cell = stroke_row.clone();
                                                    
                                                    view! {
                                                        <rect
                                                            x=x.to_string()
                                                            y=y.to_string()
                                                            width=cs.to_string()
                                                            height=cs.to_string()
                                                            fill=if is_hovered { "rgba(99, 102, 241, 0.2)" } else { "rgba(0, 0, 0, 0.001)" }
                                                            stroke=stroke_cell
                                                            stroke-width="1"
                                                            class="grid-cell-rect"
                                                            pointer-events="all"
                                                            on:pointerenter=move |_| set_hovered_cell.set(Some((c_f, r_f)))
                                                            on:dragover=move |ev: ev::DragEvent| {
                                                                ev.prevent_default();
                                                                if let Some(dt) = ev.data_transfer() {
                                                                    dt.set_drop_effect("move");
                                                                }
                                                            }
                                                            on:drop=move |ev: ev::DragEvent| {
                                                                ev.prevent_default();
                                                                if let Some(token_id) = dragged_token_id.get_untracked() {
                                                                    move_token_to_cell(&token_id, c_f, r_f);
                                                                    set_dragged_token_id.set(None);
                                                                }
                                                            }
                                                            on:click=move |_| {
                                                                let cur_tool = active_tool.get();
                                                                if cur_tool == "build" {
                                                                    place_or_erase_structure(c_f, r_f);
                                                                } else if cur_tool == "ruler" {
                                                                    if ruler_start.get().is_none() {
                                                                        set_ruler_start.set(Some((c_f, r_f)));
                                                                        set_ruler_end.set(None);
                                                                    } else if ruler_end.get().is_none() {
                                                                        set_ruler_end.set(Some((c_f, r_f)));
                                                                    } else {
                                                                        set_ruler_start.set(Some((c_f, r_f)));
                                                                        set_ruler_end.set(None);
                                                                    }
                                                                } else {
                                                                    move_selected_token(c_f, r_f);
                                                                }
                                                            }
                                                        />
                                                    }
                                                })
                                            }).collect_view()}
                                        </g>
                                    }.into_view()
                                } else {
                                    // ─── Grade Interativa de Células (Hexagonal) ─
                                    let r_radius = cs / 2.0;
                                    let stroke = grid_color.clone();
                                    view! {
                                        <g class="hex-grid-group">
                                            {(0..rows).flat_map(move |r| {
                                                let r_f = r as f32;
                                                let stroke_row = stroke.clone();
                                                (0..cols).map(move |c| {
                                                    let c_f = c as f32;
                                                    let (cx, cy) = get_cell_center(c_f, r_f);
                                                    let pts = get_hex_points(cx, cy, r_radius);
                                                    let is_hovered = hovered_cell.get() == Some((c_f, r_f));
                                                    let stroke_cell = stroke_row.clone();

                                                    view! {
                                                        <polygon
                                                            points=pts
                                                            fill=if is_hovered { "rgba(99, 102, 241, 0.2)" } else { "rgba(0, 0, 0, 0.001)" }
                                                            stroke=stroke_cell
                                                            stroke-width="1"
                                                            class="grid-cell-hex"
                                                            pointer-events="all"
                                                            on:pointerenter=move |_| set_hovered_cell.set(Some((c_f, r_f)))
                                                            on:dragover=move |ev: ev::DragEvent| {
                                                                ev.prevent_default();
                                                                if let Some(dt) = ev.data_transfer() {
                                                                    dt.set_drop_effect("move");
                                                                }
                                                            }
                                                            on:drop=move |ev: ev::DragEvent| {
                                                                ev.prevent_default();
                                                                if let Some(token_id) = dragged_token_id.get_untracked() {
                                                                    move_token_to_cell(&token_id, c_f, r_f);
                                                                    set_dragged_token_id.set(None);
                                                                }
                                                            }
                                                            on:click=move |_| {
                                                                let cur_tool = active_tool.get();
                                                                if cur_tool == "build" {
                                                                    place_or_erase_structure(c_f, r_f);
                                                                } else if cur_tool == "ruler" {
                                                                    if ruler_start.get().is_none() {
                                                                        set_ruler_start.set(Some((c_f, r_f)));
                                                                        set_ruler_end.set(None);
                                                                    } else if ruler_end.get().is_none() {
                                                                        set_ruler_end.set(Some((c_f, r_f)));
                                                                    } else {
                                                                        set_ruler_start.set(Some((c_f, r_f)));
                                                                        set_ruler_end.set(None);
                                                                    }
                                                                } else {
                                                                    move_selected_token(c_f, r_f);
                                                                }
                                                            }
                                                        />
                                                    }
                                                })
                                            }).collect_view()}
                                        </g>
                                    }.into_view()
                                }}

                                // ─── Linha da Régua ──────────────────────────
                                {move || {
                                    if let (Some(start), Some(end)) = (ruler_start.get(), ruler_end.get()) {
                                        let (x1, y1) = get_cell_center(start.0, start.1);
                                        let (x2, y2) = get_cell_center(end.0, end.1);
                                        view! {
                                            <g class="ruler-line-group" pointer-events="none">
                                                <line
                                                    x1=x1.to_string()
                                                    y1=y1.to_string()
                                                    x2=x2.to_string()
                                                    y2=y2.to_string()
                                                    stroke="#f59e0b"
                                                    stroke-width="3"
                                                    stroke-dasharray="6,4"
                                                />
                                                <circle cx=x1.to_string() cy=y1.to_string() r="6" fill="#f59e0b" />
                                                <circle cx=x2.to_string() cy=y2.to_string() r="6" fill="#ef4444" />
                                            </g>
                                        }.into_view()
                                    } else {
                                        view! {}.into_view()
                                    }
                                }}
                            </svg>

                            // ─── Camada de Tokens Interativos (DOM) ───────
                            <div class="battle-tokens-layer">
                                {move || {
                                    let tokens = map_data.with(|m| m.tokens.clone());
                                    let is_user_gm = is_gm.get();
                                    let sel_id = selected_token_id.get();

                                    tokens.into_iter().map(|token| {
                                        let t_id_drag = token.id.clone();
                                        let t_id_select = token.id.clone();
                                        let t_id_del = token.id.clone();
                                        let t_id_hide = token.id.clone();
                                        let t_col = token.grid_col;
                                        let t_row = token.grid_row;
                                        let is_selected = sel_id.as_deref() == Some(&token.id);
                                        let is_hidden = token.is_hidden;

                                        if is_hidden && !is_user_gm {
                                            return view! {}.into_view();
                                        }

                                        let (cx, cy) = get_cell_center(token.grid_col, token.grid_row);
                                        let token_size_px = cs * token.size_cells * 0.88;
                                        let half_size = token_size_px / 2.0;
                                        let pos_x = cx - half_size;
                                        let pos_y = cy - half_size;

                                        let token_style = format!(
                                            "left: {}px; top: {}px; width: {}px; height: {}px; border-color: {};",
                                            pos_x, pos_y, token_size_px, token_size_px, token.color
                                        );

                                        let initial_char = if token.is_npc {
                                            "👾".to_string()
                                        } else {
                                            token.name.chars().next().unwrap_or('?').to_string()
                                        };

                                        let t_del_right_click = t_id_del.clone();

                                        view! {
                                            <div
                                                class="battle-map-token"
                                                class:token-selected=is_selected
                                                class:token-hidden=is_hidden
                                                draggable="true"
                                                style=token_style
                                                title="Clique para selecionar • Botão direito para remover"
                                                on:contextmenu=move |ev: ev::MouseEvent| {
                                                    ev.prevent_default();
                                                    ev.stop_propagation();
                                                    remove_token(t_del_right_click.clone());
                                                }
                                                on:dragstart=move |ev: ev::DragEvent| {
                                                    set_dragged_token_id.set(Some(t_id_drag.clone()));
                                                    if let Some(dt) = ev.data_transfer() {
                                                        let _ = dt.set_data("text/plain", &t_id_drag);
                                                        dt.set_drop_effect("move");
                                                    }
                                                }
                                                on:dragend=move |_| {
                                                    set_dragged_token_id.set(None);
                                                }
                                                on:click=move |ev: ev::MouseEvent| {
                                                    ev.stop_propagation();
                                                    let cur_tool = active_tool.get();
                                                    if cur_tool == "build" {
                                                        place_or_erase_structure(t_col, t_row);
                                                    } else if cur_tool == "select" {
                                                        if selected_token_id.get().as_deref() == Some(&t_id_select) {
                                                            set_selected_token_id.set(None);
                                                        } else {
                                                            set_selected_token_id.set(Some(t_id_select.clone()));
                                                        }
                                                    }
                                                }
                                            >
                                                // Imagem do Avatar ou Ícone
                                                {if !token.avatar_url.is_empty() {
                                                    view! {
                                                        <img
                                                            src=token.avatar_url.clone()
                                                            alt=token.name.clone()
                                                            class="token-img"
                                                        />
                                                    }.into_view()
                                                } else {
                                                    view! {
                                                        <div class="token-placeholder" style=format!("background: {};", token.color)>
                                                            <span>{initial_char}</span>
                                                        </div>
                                                    }.into_view()
                                                }}

                                                // Badge de Nome / Tag
                                                <div class="token-name-label">{token.name.clone()}</div>

                                                // Ações Rápidas Flutuantes (quando selecionado)
                                                {if is_selected {
                                                    view! {
                                                        <div class="token-quick-actions" draggable="false" on:mousedown=move |e: ev::MouseEvent| e.stop_propagation()>
                                                            {if is_user_gm {
                                                                view! {
                                                                    <button
                                                                        type="button"
                                                                        class="token-act-btn hide-btn"
                                                                        on:click=move |ev: ev::MouseEvent| {
                                                                            ev.stop_propagation();
                                                                            toggle_token_hidden(t_id_hide.clone());
                                                                        }
                                                                        title=if is_hidden { "Revelar aos Jogadores" } else { "Ocultar dos Jogadores" }
                                                                    >
                                                                        {if is_hidden { "👁️" } else { "🔒" }}
                                                                    </button>
                                                                }.into_view()
                                                            } else {
                                                                view! {}.into_view()
                                                            }}

                                                            <button
                                                                type="button"
                                                                class="token-act-btn del-btn"
                                                                on:click=move |ev: ev::MouseEvent| {
                                                                    ev.stop_propagation();
                                                                    remove_token(t_id_del.clone());
                                                                }
                                                                title="Remover Token do Mapa"
                                                            >
                                                                "✕"
                                                            </button>
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    view! {}.into_view()
                                                }}
                                            </div>
                                        }.into_view()
                                    }).collect_view()
                                }}
                            </div>
                        </div>
                    }
                }}
            </div>

            // ─── Modal de Configurações do Mestre (Grid & Imagem) ────────
            {move || if show_gm_modal.get() {
                view! {
                    <div class="grid-modal-backdrop" on:click=move |_| set_show_gm_modal.set(false)>
                        <div class="grid-modal-content" on:click=move |ev: ev::MouseEvent| ev.stop_propagation()>
                            <div class="modal-header">
                                <h3>"⚙️ Configurações do Tabuleiro Tático"</h3>
                                <button class="modal-close-btn" on:click=move |_| set_show_gm_modal.set(false)>"✕"</button>
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
                                <button class="modal-btn cancel-btn" on:click=move |_| set_show_gm_modal.set(false)>"Cancelar"</button>
                                <button class="modal-btn save-btn" on:click=on_save_gm_config>"Aplicar Alterações"</button>
                            </div>
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}

            // ─── Modal de Criar NPC / Monstro ────────────────────────────
            {move || if show_npc_modal.get() {
                view! {
                    <div class="grid-modal-backdrop" on:click=move |_| set_show_npc_modal.set(false)>
                        <div class="grid-modal-content" on:click=move |ev: ev::MouseEvent| ev.stop_propagation()>
                            <div class="modal-header">
                                <h3>"👾 Adicionar NPC ou Monstro ao Grid"</h3>
                                <button class="modal-close-btn" on:click=move |_| set_show_npc_modal.set(false)>"✕"</button>
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
                                <button class="modal-btn cancel-btn" on:click=move |_| set_show_npc_modal.set(false)>"Cancelar"</button>
                                <button class="modal-btn save-btn" on:click=on_add_npc>"Criar Token no Mapa"</button>
                            </div>
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}
        </div>
    }
}
