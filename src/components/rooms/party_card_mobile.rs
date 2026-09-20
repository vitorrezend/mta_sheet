use leptos::*;
use leptos_router::*;
use crate::rooms::{RoomSheetSummary, QuickStatAction};
use crate::i18n::Language;

#[component]
pub fn PartyCardMobile<FQuick, FToggle, FRemove, FDragStart, FDragOver, FDrop, FDragEnd, FTouchMove>(
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
    on_touch_move_coord: FTouchMove,
) -> impl IntoView
where
    FQuick: Fn(String, QuickStatAction) + Copy + 'static,
    FToggle: Fn(String, bool) + Copy + 'static,
    FRemove: Fn(String) + Copy + 'static,
    FDragStart: Fn(String) + Copy + 'static,
    FDragOver: Fn(String) + Copy + 'static,
    FDrop: Fn(String) + Copy + 'static,
    FDragEnd: Fn() + Copy + 'static,
    FTouchMove: Fn(i32, i32) + Copy + 'static,
{
    let s_id_nav = sheet.id.clone();
    let s_id_data = sheet.id.clone();
    let s_id_dstart = sheet.id.clone();
    let s_id_tstart = sheet.id.clone();
    let s_id_dover = sheet.id.clone();
    let s_id_ddrop = sheet.id.clone();
    let s_id_nav_avatar = sheet.id.clone();
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

    let wp_tot = sheet.willpower_total.clamp(1, 10);
    let wp_cur = sheet.willpower_current.clamp(0, wp_tot);
    let badge_cls = format!("party-health-badge {}", sheet.health_badge_class);

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

    let s_id_card = s_id_nav.clone();
    let char_name = sheet.name.clone();
    let char_name_alt = sheet.name.clone();
    let player_name = sheet.player_name.clone();
    let photo_focus_x = sheet.photo_focus_x;
    let photo_focus_y = sheet.photo_focus_y;
    let arete_val = sheet.arete;

    view! {
        <div 
            class="character-card party-card-mobile" 
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
            // 1. Top Bar: Alça de Arraste + Avatar + Informações Básicas + Botões de Ação
            <div class="mobile-card-top">
                // Alça de Arraste Touch & Mouse
                <div
                    class="party-drag-handle mobile-drag-handle"
                    draggable="true"
                    title="Segure e arraste para reordenar"
                    on:click=move |ev: ev::MouseEvent| ev.stop_propagation()
                    on:dragstart=move |ev: ev::DragEvent| {
                        if let Some(dt) = ev.data_transfer() {
                            let _ = dt.set_data("text/plain", &s_id_dstart);
                            dt.set_effect_allowed("move");
                        }
                        on_drag_start(s_id_dstart.clone());
                    }
                    on:dragend=move |_| on_drag_end()
                    on:touchstart=move |ev: ev::TouchEvent| {
                        ev.stop_propagation();
                        on_drag_start(s_id_tstart.clone());
                    }
                    on:touchmove=move |ev: ev::TouchEvent| {
                        ev.prevent_default();
                        ev.stop_propagation();
                        if let Some(t) = ev.touches().item(0) {
                            on_touch_move_coord(t.client_x(), t.client_y());
                        }
                    }
                    on:touchend=move |ev: ev::TouchEvent| {
                        ev.stop_propagation();
                        on_drag_end();
                    }
                >
                    "⋮⋮"
                </div>

                // Avatar clicável para ficha
                <A href=format!("/sheet/{}", s_id_nav_avatar) class="mobile-avatar-link" on:click=move |ev: ev::MouseEvent| ev.stop_propagation()>
                    {if has_photo {
                        let img_style = format!("object-position: {}% {}%;", photo_focus_x, photo_focus_y);
                        view! {
                            <img
                                src=photo
                                alt=char_name_alt
                                class="mobile-card-avatar"
                                style=img_style
                            />
                        }.into_view()
                    } else {
                        view! {
                            <div class="mobile-card-avatar-placeholder">
                                <span>{if is_gm_char { "🐉" } else { "🔮" }}</span>
                            </div>
                        }.into_view()
                    }}
                </A>

                // Nome, Arete e Tradição
                <div class="mobile-card-title-group">
                    <div class="mobile-card-name-row">
                        <A href=format!("/sheet/{}", s_id_nav) class="mobile-card-name" on:click=move |ev: ev::MouseEvent| ev.stop_propagation()>
                            {char_name}
                        </A>
                        <span class="mobile-card-arete-badge">"Arete " {arete_val}</span>
                    </div>
                    <div class="mobile-card-subtext">
                        <span>{tradition_display}</span>
                        {if !player_name.is_empty() {
                            view! { <span class="mobile-player-name">"• " {player_name.clone()}</span> }.into_view()
                        } else {
                            ().into_view()
                        }}
                    </div>
                </div>

                // Botões de Ação do Topo (Ocultar, Desvincular, Abrir)
                <div class="mobile-card-actions" on:click=move |ev: ev::MouseEvent| ev.stop_propagation()>
                    {if can_toggle {
                        let t_id = toggle_id.clone();
                        view! {
                            <button
                                type="button"
                                class="mobile-btn-action"
                                class:active-hidden=is_hidden
                                on:click=move |ev: ev::MouseEvent| {
                                    ev.stop_propagation();
                                    on_toggle_visibility(t_id.clone(), is_hidden);
                                }
                                title=if is_hidden { "Ficha Oculta. Clique para Revelar." } else { "Ficha Visível. Clique para Ocultar." }
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
                                class="mobile-btn-action mobile-btn-unlink"
                                on:click=move |ev: ev::MouseEvent| {
                                    ev.stop_propagation();
                                    on_remove_sheet(r_id.clone());
                                }
                                title="Desvincular ficha da mesa"
                            >
                                "✕"
                            </button>
                        }.into_view()
                    } else {
                        ().into_view()
                    }}

                    <a href=format!("/sheet/{}", s_id_nav) class="mobile-btn-action" title="Abrir Ficha Completa">
                        "📖"
                    </a>
                </div>
            </div>

            // 2. Linha de Pílulas de Esferas Ativas
            <div class="mobile-card-spheres">
                {active_spheres_view}
            </div>

            // 3. Painel Tático Interativo para o Narrador (GM) ou Resumo para Jogador
            {if is_gm {
                let s_id_gm = sheet.id.clone();
                let s_id_heal = sheet.id.clone();
                let s_id_q_sub = sheet.id.clone();
                let s_id_q_add = sheet.id.clone();
                let s_id_p_sub = sheet.id.clone();
                let s_id_p_add = sheet.id.clone();
                let s_id_wp_sub = sheet.id.clone();
                let s_id_wp_add = sheet.id.clone();

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
                            class=format!("mobile-health-box dmg-{}", dmg_key)
                            title=format!("Nível #{}: {} (Toque: alternar dano | Segure: curar)", idx + 1, dmg_key)
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

                view! {
                    <div class="mobile-tactical-row" on:click=move |ev: ev::MouseEvent| ev.stop_propagation()>
                        // Linha de Vitalidade: Rótulo + Trilha Compacta + Botão Curar
                        <div class="mobile-health-row">
                            <div class="mobile-health-info">
                                <span class="mobile-section-label">"HP:"</span>
                                <span class=badge_cls>{sheet.health_label.clone()}" ("{sheet.health_penalty.clone()}")"</span>
                            </div>
                            <div class="mobile-health-track">
                                {health_boxes_view}
                            </div>
                            <button
                                type="button"
                                class="mobile-heal-btn"
                                title="Curar todo o dano"
                                on:click=move |ev: ev::MouseEvent| {
                                    ev.stop_propagation();
                                    on_quick_stat(s_id_heal.clone(), QuickStatAction::ClearHealth);
                                }
                            >
                                "↺"
                            </button>
                        </div>

                        // Grade de Controles Rápidos: Força de Vontade, Quintessência e Paradoxo
                        <div class="mobile-resources-grid">
                            // Força de Vontade
                            <div class="mobile-resource-box">
                                <span class="mobile-resource-tag">"Vontade"</span>
                                <div class="mobile-resource-controls">
                                    <button
                                        type="button"
                                        class="mobile-resource-btn"
                                        title="Gastar 1 Vontade"
                                        on:click=move |ev: ev::MouseEvent| {
                                            ev.stop_propagation();
                                            let new_val = (wp_cur - 1).max(0);
                                            on_quick_stat(s_id_wp_sub.clone(), QuickStatAction::SetWillpowerCurrent { value: new_val });
                                        }
                                    >"−"</button>
                                    <span class="mobile-resource-num">{wp_cur} "/" {wp_tot}</span>
                                    <button
                                        type="button"
                                        class="mobile-resource-btn"
                                        title="Recuperar 1 Vontade"
                                        on:click=move |ev: ev::MouseEvent| {
                                            ev.stop_propagation();
                                            let new_val = (wp_cur + 1).min(wp_tot);
                                            on_quick_stat(s_id_wp_add.clone(), QuickStatAction::SetWillpowerCurrent { value: new_val });
                                        }
                                    >"+"</button>
                                </div>
                            </div>

                            // Quintessência
                            <div class="mobile-resource-box">
                                <span class="mobile-resource-tag">"✨ Quint"</span>
                                <div class="mobile-resource-controls">
                                    <button
                                        type="button"
                                        class="mobile-resource-btn"
                                        title="Gastar 1 Quintessência"
                                        on:click=move |ev: ev::MouseEvent| {
                                            ev.stop_propagation();
                                            on_quick_stat(s_id_q_sub.clone(), QuickStatAction::RemoveQuintessence);
                                        }
                                    >"−"</button>
                                    <span class="mobile-resource-num" style="color: #0284c7;">{sheet.quintessence}</span>
                                    <button
                                        type="button"
                                        class="mobile-resource-btn"
                                        title="Adicionar 1 Quintessência"
                                        on:click=move |ev: ev::MouseEvent| {
                                            ev.stop_propagation();
                                            on_quick_stat(s_id_q_add.clone(), QuickStatAction::AddQuintessence);
                                        }
                                    >"+"</button>
                                </div>
                            </div>

                            // Paradoxo
                            <div class="mobile-resource-box">
                                <span class="mobile-resource-tag">"⚡ Paradox"</span>
                                <div class="mobile-resource-controls">
                                    <button
                                        type="button"
                                        class="mobile-resource-btn"
                                        title="Reduzir 1 Paradoxo"
                                        on:click=move |ev: ev::MouseEvent| {
                                            ev.stop_propagation();
                                            on_quick_stat(s_id_p_sub.clone(), QuickStatAction::RemoveParadox);
                                        }
                                    >"−"</button>
                                    <span class="mobile-resource-num" style="color: #dc2626;">{sheet.paradox}</span>
                                    <button
                                        type="button"
                                        class="mobile-resource-btn"
                                        title="Adicionar 1 Paradoxo"
                                        on:click=move |ev: ev::MouseEvent| {
                                            ev.stop_propagation();
                                            on_quick_stat(s_id_p_add.clone(), QuickStatAction::AddParadox);
                                        }
                                    >"+"</button>
                                </div>
                            </div>
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {
                    <div class="mobile-tactical-row mobile-player-row">
                        <div class="mobile-health-row">
                            <div class="mobile-health-info">
                                <span class="mobile-section-label">"Vitalidade:"</span>
                                <span class=badge_cls>{sheet.health_label.clone()}" ("{sheet.health_penalty.clone()}")"</span>
                            </div>
                            <div class="mobile-simple-stat">
                                <span class="stat-box-label">"Vontade: "</span>
                                <span class="stat-box-num">{wp_cur} "/" {wp_tot}</span>
                            </div>
                        </div>
                    </div>
                }.into_view()
            }}
        </div>
    }
}
