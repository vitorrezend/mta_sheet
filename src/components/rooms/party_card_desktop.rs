use leptos::*;
use leptos_router::*;
use crate::rooms::{RoomSheetSummary, QuickStatAction};
use crate::i18n::Language;

fn get_qp_box_coords(index: usize) -> (f64, f64) {
    let cx = 85.0;
    let cy = 85.0;
    let r = 67.0;
    let angle_deg: f64 = if index < 10 {
        180.0 - (index as f64 + 0.5) * 18.0
    } else {
        let step = (19 - index) as f64;
        180.0 + (step + 0.5) * 18.0
    };
    let rad = angle_deg.to_radians();
    let x = cx + r * rad.cos() - 6.5;
    let y = cy - r * rad.sin() - 6.5;
    (x, y)
}

#[component]
pub fn PartyCardDesktop<FQuick, FToggle, FRemove, FDragStart, FDragOver, FDrop, FDragEnd>(
    sheet: RoomSheetSummary,
    is_gm: bool,
    current_lang: Language,
    is_dragging: bool,
    is_drag_over: bool,
    on_quick_stat: FQuick,
    on_toggle_visibility: FToggle,
    on_remove_sheet: FRemove,
    on_drag_start: FDragStart,
    on_drag_over: FDragOver,
    on_drop: FDrop,
    on_drag_end: FDragEnd,
) -> impl IntoView
where
    FQuick: Fn(String, QuickStatAction) + Copy + 'static,
    FToggle: Fn(String, bool) + Copy + 'static,
    FRemove: Fn(String) + Copy + 'static,
    FDragStart: Fn(String) + Copy + 'static,
    FDragOver: Fn(String) + Copy + 'static,
    FDrop: Fn(String) + Copy + 'static,
    FDragEnd: Fn() + Copy + 'static,
{
    let s_id_nav = sheet.id.clone();
    let s_id_data = sheet.id.clone();
    let s_id_dstart = sheet.id.clone();
    let s_id_dover = sheet.id.clone();
    let s_id_ddrop = sheet.id.clone();
    let remove_id = sheet.id.clone();
    let toggle_id = sheet.id.clone();
    let is_hidden = sheet.is_hidden;
    let can_toggle = is_gm || sheet.is_owner;
    let is_gm_char = sheet.sheet_type == "gods_and_monsters";
    let photo = sheet.photo_url.clone();
    let has_photo = !photo.is_empty();

    let tradition_display = if !sheet.tradition.is_empty() {
        sheet.tradition.clone()
    } else if is_gm_char {
        "Familiar / Bygone".to_string()
    } else {
        crate::i18n::tr("card_tradition_undefined", current_lang).to_string()
    };

    let title_tradition = crate::i18n::tr_header_label("Tradicao", current_lang).to_string();
    let title_essence = crate::i18n::tr_header_label("Essencia", current_lang).to_string();
    let title_concept = crate::i18n::tr_header_label("Conceito", current_lang).to_string();
    let title_demeanor = crate::i18n::tr_header_label("Comportamento", current_lang).to_string();
    let tradition_unspec = crate::i18n::tr("card_tradition_undefined", current_lang).to_string();

    let tradition_pill = if !sheet.tradition.is_empty() || is_gm_char {
        let icon = if is_gm_char { "🐉" } else { "🏛️" };
        view! {
            <span class="party-pill pill-tradition" title=title_tradition>
                <span class="pill-icon">{icon}</span>
                <span class="pill-text">{tradition_display.clone()}</span>
            </span>
        }.into_view()
    } else {
        view! {
            <span class="party-pill pill-tradition pill-empty" title=tradition_unspec.clone()>
                <span class="pill-icon">"🏛️"</span>
                <span class="pill-text">{tradition_unspec}</span>
            </span>
        }.into_view()
    };

    let concept_pill = if !sheet.concept.is_empty() {
        let conc = sheet.concept.clone();
        view! {
            <span class="party-pill pill-concept" title=title_concept>
                <span class="pill-icon">"💡"</span>
                <span class="pill-text">{conc}</span>
            </span>
        }.into_view()
    } else {
        ().into_view()
    };

    let demeanor_pill = if !sheet.demeanor.is_empty() {
        let dem = sheet.demeanor.clone();
        view! {
            <span class="party-pill pill-demeanor" title=title_demeanor>
                <span class="pill-icon">"🎭"</span>
                <span class="pill-text">{dem}</span>
            </span>
        }.into_view()
    } else {
        ().into_view()
    };

    let essence_pill = if !sheet.essence.is_empty() {
        let ess = sheet.essence.clone();
        view! {
            <span class="party-pill pill-essence" title=title_essence>
                <span class="pill-icon">"✨"</span>
                <span class="pill-text">{ess}</span>
            </span>
        }.into_view()
    } else {
        ().into_view()
    };

    // Pílulas de Esferas Ativas (nível > 0)
    let active_spheres_view = sheet.spheres.iter()
        .filter(|(_, lvl)| *lvl > 0)
        .map(|(sph_name, lvl)| {
            let tr_name = crate::i18n::tr_sphere(sph_name, current_lang);
            let sph_title = format!("{}: nível {}", tr_name, lvl);
            let sph_text = format!("{} {}", tr_name, lvl);
            view! {
                <span class="party-pill pill-sphere" title=sph_title>
                    <span class="pill-icon">"🔮"</span>
                    <span class="pill-text">{sph_text}</span>
                </span>
            }
        })
        .collect_view();

    let arete_val = sheet.arete;
    let wp_tot = sheet.willpower_total.clamp(1, 10);
    let wp_cur = sheet.willpower_current.clamp(0, wp_tot);
    let badge_cls = format!("party-health-badge {}", sheet.health_badge_class);

    let s_id_card = s_id_nav.clone();

    view! {
        <div 
            class="character-card party-card party-card-desktop" 
            class:party-card-hidden=is_hidden
            class:is-dragging=is_dragging
            class:is-drag-over=is_drag_over
            attr:data-sheet-id=s_id_data
            on:dragover=move |ev: ev::DragEvent| {
                ev.prevent_default();
                on_drag_over(s_id_dover.clone());
            }
            on:drop=move |ev: ev::DragEvent| {
                ev.prevent_default();
                on_drop(s_id_ddrop.clone());
            }
            on:click=move |_| use_navigate()(&format!("/sheet/{}", s_id_card), Default::default())
        >
            // 1. Portrait Header Box
            <div class="card-portrait-box">
                // Drag Handle
                <div
                    class="party-drag-handle desktop-drag-handle"
                    draggable="true"
                    title="Clique e arraste para reordenar esta ficha"
                    on:click=move |ev: ev::MouseEvent| ev.stop_propagation()
                    on:dragstart=move |ev: ev::DragEvent| {
                        if let Some(dt) = ev.data_transfer() {
                            let _ = dt.set_data("text/plain", &s_id_dstart);
                            dt.set_effect_allowed("move");
                        }
                        on_drag_start(s_id_dstart.clone());
                    }
                    on:dragend=move |_| on_drag_end()
                >
                    "⋮⋮"
                </div>
                {if has_photo {
                    let img_style = format!("object-position: {}% {}%;", sheet.photo_focus_x, sheet.photo_focus_y);
                    view! {
                        <img
                            src=photo
                            alt=sheet.name.clone()
                            class="card-portrait-img"
                            style=img_style
                        />
                    }.into_view()
                } else {
                    view! {
                        <div class="card-portrait-placeholder">
                            <span class="placeholder-icon">{if is_gm_char { "🐉" } else { "🔮" }}</span>
                            <span class="placeholder-tag">{if is_gm_char { "Gods & Monsters".to_string() } else { crate::i18n::tr("card_no_image", current_lang).to_string() }}</span>
                        </div>
                    }.into_view()
                }}
                <div class="card-portrait-gradient"></div>

                // Floating Top Actions
                <div class="party-floating-actions">
                    {if can_toggle {
                        let t_id = toggle_id.clone();
                        view! {
                            <button
                                type="button"
                                class="party-float-btn party-vis-btn"
                                class:active-hidden=is_hidden
                                on:click=move |ev: ev::MouseEvent| {
                                    ev.stop_propagation();
                                    on_toggle_visibility(t_id.clone(), is_hidden);
                                }
                                title=if is_hidden { "Ficha Oculta dos Jogadores. Clique para Revelar." } else { "Ficha Visível. Clique para Ocultar dos Jogadores." }
                            >
                                {if is_hidden { "🔒" } else { "👁️" }}
                            </button>
                        }.into_view()
                    } else {
                        ().into_view()
                    }}

                    {if is_gm || sheet.is_owner {
                        let r_id = remove_id.clone();
                        view! {
                            <button
                                type="button"
                                class="party-float-btn party-unlink-btn"
                                on:click=move |ev: ev::MouseEvent| {
                                    ev.stop_propagation();
                                    on_remove_sheet(r_id.clone());
                                }
                                title="Desvincular esta ficha da mesa"
                            >
                                "✕"
                            </button>
                        }.into_view()
                    } else {
                        ().into_view()
                    }}
                </div>
            </div>

            // 2. Card Content
            <div class="card-content">
                <div class="card-header-info">
                    <h3 class="card-name">{sheet.name.clone()}</h3>
                    <div class="party-card-pills">
                        {tradition_pill}
                        {concept_pill}
                        {demeanor_pill}
                        {essence_pill}
                        {active_spheres_view}
                    </div>
                </div>

                <div class="party-card-details">
                    // 3. Stats & GM Tactical HUD (Only visible to Storyteller/GM)
                    {if is_gm {
                        let s_id_gm = sheet.id.clone();
                        let s_id_heal_all = sheet.id.clone();
                        let s_id_q_sub = sheet.id.clone();
                        let s_id_q_add = sheet.id.clone();
                        let s_id_p_sub = sheet.id.clone();
                        let s_id_p_add = sheet.id.clone();
                        let qp_track = sheet.quintessence_paradox_track.clone();

                        let health_boxes_view = sheet.health_boxes.iter().enumerate().map(|(idx, dmg_key)| {
                            let s_id_box = s_id_gm.clone();
                            let s_id_box_heal = s_id_gm.clone();
                            let dmg_symbol = match dmg_key.as_str() {
                                "bashing" => "/",
                                "lethal" => "✕",
                                "aggravated" => "✳",
                                _ => "",
                            };
                            view! {
                                <button
                                    type="button"
                                    class=format!("health-mini-box dmg-{}", dmg_key)
                                    title=format!("Nível #{}: {} (Esq: alternar dano | Dir: curar)", idx + 1, dmg_key)
                                    on:click=move |ev: ev::MouseEvent| {
                                        ev.stop_propagation();
                                        on_quick_stat(s_id_box.clone(), QuickStatAction::CycleHealthBox { index: idx });
                                    }
                                    on:contextmenu=move |ev: ev::MouseEvent| {
                                        ev.prevent_default();
                                        ev.stop_propagation();
                                        on_quick_stat(s_id_box_heal.clone(), QuickStatAction::HealHealthBox { index: idx });
                                    }
                                >
                                    {dmg_symbol}
                                </button>
                            }
                        }).collect_view();

                        let qp_wheel_boxes = (0..20).map(|i| {
                            let (x, y) = get_qp_box_coords(i);
                            let state_char = qp_track.chars().nth(i).unwrap_or('0');
                            let is_quint = state_char == '1';
                            let is_paradox = state_char == '2';
                            let s_id_wheel_click = s_id_gm.clone();
                            let s_id_wheel_right = s_id_gm.clone();
                            let box_title = match state_char {
                                '1' => format!("✦ Quintessência (Slot #{}) | Clique: alternar | Dir: limpar", i + 1),
                                '2' => format!("⚡ Paradoxo (Slot #{}) | Clique: alternar | Dir: limpar", 20 - i),
                                _ => format!("○ Slot Livre #{} | Clique: alternar | Dir: limpar", i + 1),
                            };
                            view! {
                                <rect
                                    x=format!("{:.1}", x)
                                    y=format!("{:.1}", y)
                                    width="13"
                                    height="13"
                                    rx="2"
                                    ry="2"
                                    class="qp-wheel-box"
                                    class:qp-box-quintessence=is_quint
                                    class:qp-box-paradox=is_paradox
                                    on:click=move |ev: ev::MouseEvent| {
                                        ev.stop_propagation();
                                        on_quick_stat(s_id_wheel_click.clone(), QuickStatAction::CycleQuintessenceParadoxBox { index: i });
                                    }
                                    on:contextmenu=move |ev: ev::MouseEvent| {
                                        ev.prevent_default();
                                        ev.stop_propagation();
                                        on_quick_stat(s_id_wheel_right.clone(), QuickStatAction::ClearQuintessenceParadoxBox { index: i });
                                    }
                                >
                                    <title>{box_title}</title>
                                </rect>
                            }
                        }).collect_view();

                        view! {
                            <div class="gm-tactical-hud" on:click=move |ev: ev::MouseEvent| ev.stop_propagation()>
                                // Top bar with Arete & Jogador
                                <div class="party-stats-grid party-stats-grid-gm">
                                    <div class="party-stat-box">
                                        <span class="stat-box-label">"Arete"</span>
                                        <span class="stat-box-num">{arete_val}</span>
                                    </div>
                                    <div class="party-stat-box party-stat-box-wide">
                                        <span class="stat-box-label">"Jogador"</span>
                                        <span class="stat-box-val-text">{if !sheet.player_name.is_empty() { sheet.player_name.clone() } else { "—".to_string() }}</span>
                                    </div>
                                </div>

                                // 1. Força de Vontade: Linha de Bolinhas (Total) + Linha de Quadradinhos (Atual)
                                <div class="gm-hud-section gm-hud-willpower">
                                    <div class="gm-hud-header-row">
                                        <span class="gm-hud-label">"Força de Vontade"</span>
                                        <span class="gm-stat-badge">{wp_cur} " / " {wp_tot}</span>
                                    </div>

                                    // Linha Total: 10 Bolinhas
                                    <div class="gm-wp-line">
                                        <span class="gm-wp-line-tag">"Total"</span>
                                        <div class="dots-container gm-dots-row">
                                            {(1..=10).map(|i| {
                                                let is_filled = wp_tot >= i;
                                                let s_id_dot = s_id_gm.clone();
                                                view! {
                                                    <span
                                                        class="dot"
                                                        class:filled=is_filled
                                                        title=format!("Força de Vontade Total: {} (Clique para definir)", i)
                                                        on:click=move |ev: ev::MouseEvent| {
                                                            ev.stop_propagation();
                                                            let new_val = if wp_tot == i { (i - 1).max(1) } else { i };
                                                            on_quick_stat(s_id_dot.clone(), QuickStatAction::SetWillpowerTotal { value: new_val });
                                                        }
                                                    ></span>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </div>

                                    // Linha Atual: 10 Quadradinhos
                                    <div class="gm-wp-line">
                                        <span class="gm-wp-line-tag">"Atual"</span>
                                        <div class="dots-container gm-squares-row">
                                            {(1..=10).map(|i| {
                                                let is_filled = wp_cur >= i;
                                                let is_over = i > wp_tot;
                                                let s_id_sq = s_id_gm.clone();
                                                view! {
                                                    <span
                                                        class="square"
                                                        class:filled=is_filled
                                                        class:disabled=is_over
                                                        title=format!("Força de Vontade Atual: {}/{} (Clique para alterar)", i, wp_tot)
                                                        on:click=move |ev: ev::MouseEvent| {
                                                            ev.stop_propagation();
                                                            let new_val = if wp_cur == i { i - 1 } else { i };
                                                            on_quick_stat(s_id_sq.clone(), QuickStatAction::SetWillpowerCurrent { value: new_val });
                                                        }
                                                    ></span>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </div>
                                </div>

                                // 2. Quintessência & Paradoxo (Roda Circular M20)
                                <div class="gm-hud-section gm-hud-qp-wheel">
                                    <div class="gm-hud-header-row" style="margin-bottom: 2px;">
                                        <span class="gm-hud-label">"Quintessência / Paradoxo"</span>
                                        <div class="gm-stat-badge">
                                            <span style="color: #0284c7; font-weight: 800;">"✨ " {sheet.quintessence}</span>
                                            <span style="color: #94a3b8; margin: 0 4px;">"/"</span>
                                            <span style="color: #dc2626; font-weight: 800;">"⚡ " {sheet.paradox}</span>
                                        </div>
                                    </div>

                                    <div class="qp-wheel-wrapper gm-card-wheel">
                                        <svg
                                            class="qp-wheel-svg"
                                            viewBox="0 0 170 170"
                                            width="160"
                                            height="160"
                                        >
                                            <circle cx="85" cy="85" r="67" class="qp-track-ring" />
                                            <g class="qp-origin-marker" title="Ponto de Origem (9h)">
                                                <line x1="9" y1="85" x2="20" y2="85" class="qp-origin-line" />
                                                <circle cx="8" cy="85" r="2" class="qp-origin-dot" />
                                                <text x="1" y="87.5" class="qp-origin-text">"9h"</text>
                                            </g>
                                            {qp_wheel_boxes}
                                        </svg>

                                        // Núcleo Central Interativo com Botões de Ação Rápida
                                        <div class="qp-center-hub" on:click=move |ev: ev::MouseEvent| ev.stop_propagation()>
                                            <div class="qp-hub-row qp-hub-quint">
                                                <span class="qp-hub-title">"Quintessência"</span>
                                                <div class="qp-hub-controls">
                                                    <button
                                                        type="button"
                                                        class="qp-btn qp-btn-minus qp-btn-quint"
                                                        title="Remover 1 ponto de Quintessência"
                                                        on:click=move |ev: ev::MouseEvent| {
                                                            ev.stop_propagation();
                                                            on_quick_stat(s_id_q_sub.clone(), QuickStatAction::RemoveQuintessence);
                                                        }
                                                    >"−"</button>
                                                    <span class="qp-count-badge qp-badge-quint">{sheet.quintessence}</span>
                                                    <button
                                                        type="button"
                                                        class="qp-btn qp-btn-plus qp-btn-quint"
                                                        title="Adicionar 1 ponto de Quintessência (sentido horário)"
                                                        on:click=move |ev: ev::MouseEvent| {
                                                            ev.stop_propagation();
                                                            on_quick_stat(s_id_q_add.clone(), QuickStatAction::AddQuintessence);
                                                        }
                                                    >"+"</button>
                                                </div>
                                            </div>

                                            <div class="qp-hub-divider"></div>

                                            <div class="qp-hub-row qp-hub-paradox">
                                                <span class="qp-hub-title">"Paradoxo"</span>
                                                <div class="qp-hub-controls">
                                                    <button
                                                        type="button"
                                                        class="qp-btn qp-btn-minus qp-btn-paradox"
                                                        title="Remover 1 ponto de Paradoxo"
                                                        on:click=move |ev: ev::MouseEvent| {
                                                            ev.stop_propagation();
                                                            on_quick_stat(s_id_p_sub.clone(), QuickStatAction::RemoveParadox);
                                                        }
                                                    >"−"</button>
                                                    <span class="qp-count-badge qp-badge-paradox">{sheet.paradox}</span>
                                                    <button
                                                        type="button"
                                                        class="qp-btn qp-btn-plus qp-btn-paradox"
                                                        title="Adicionar 1 ponto de Paradoxo (sentido anti-horário)"
                                                        on:click=move |ev: ev::MouseEvent| {
                                                            ev.stop_propagation();
                                                            on_quick_stat(s_id_p_add.clone(), QuickStatAction::AddParadox);
                                                        }
                                                    >"+"</button>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                // 3. Vitalidade Header & Mini Track
                                <div class="gm-hud-section gm-hud-vitality">
                                    <div class="gm-hud-header-row">
                                        <div class="gm-hud-title-group">
                                            <span class="gm-hud-label">"Vitalidade:"</span>
                                            <span class=badge_cls>{sheet.health_label.clone()}" ("{sheet.health_penalty.clone()}")"</span>
                                        </div>
                                        <button
                                            type="button"
                                            class="gm-hud-heal-btn"
                                            title="Curar todos os ferimentos"
                                            on:click=move |ev: ev::MouseEvent| {
                                                ev.stop_propagation();
                                                on_quick_stat(s_id_heal_all.clone(), QuickStatAction::ClearHealth);
                                            }
                                        >
                                            "↺ Curar"
                                        </button>
                                    </div>
                                    <div class="health-mini-track">
                                        {health_boxes_view}
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="party-stats-grid party-stats-grid-player">
                                <div class="party-stat-box">
                                    <span class="stat-box-label">"Arete"</span>
                                    <span class="stat-box-num">{arete_val}</span>
                                </div>
                                <div class="party-stat-box party-stat-box-wide">
                                    <span class="stat-box-label">"Jogador"</span>
                                    <span class="stat-box-val-text">{if !sheet.player_name.is_empty() { sheet.player_name.clone() } else { "—".to_string() }}</span>
                                </div>
                            </div>
                        }.into_view()
                    }}
                </div>

                // 5. Card Footer
                <div class="party-card-footer-action">
                    <A href=format!("/sheet/{}", s_id_nav) class="btn-open-party-sheet">
                        "📖 "
                        {match current_lang {
                            crate::i18n::Language::PtBr => "Abrir Ficha Completa",
                            crate::i18n::Language::EnUs => "Open Full Sheet",
                        }}
                    </A>
                </div>
            </div>
        </div>
    }
}
