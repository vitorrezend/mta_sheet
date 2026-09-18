use leptos::*;
use leptos_router::*;
use crate::state::CharacterSummary;
use std::collections::HashMap;

pub fn render_character_grid<F>(
    data: Vec<CharacterSummary>,
    set_sheet_to_delete: WriteSignal<Option<CharacterSummary>>,
    set_sheet_to_move: WriteSignal<Option<CharacterSummary>>,
    toggle_privacy: F,
    folders_info: HashMap<String, (String, String)>,
    current_lang: crate::i18n::Language,
    dragged_sheet_id: ReadSignal<Option<String>>,
    set_dragged_sheet_id: WriteSignal<Option<String>>,
    set_drag_over_folder_id: WriteSignal<Option<String>>,
) -> View
where
    F: Fn(String, bool) + Copy + 'static,
{
    view! {
        <div class="character-cards-grid">
            {data.into_iter().map(|summary| {
                let summary_clone = summary.clone();
                let id = summary.id.clone();
                let id_card = id.clone();
                let id_vis = id.clone();
                let is_pub_val = summary.is_public;
                let is_owner = summary.is_owner;
                let photo = summary.photo_url.clone();
                let is_gm = summary.sheet_type == "gods_and_monsters";
                let has_photo = !photo.is_empty();
                let tradition_display = if !summary.tradition.is_empty() {
                    summary.tradition.clone()
                } else if is_gm {
                    "Familiar / Bygone".to_string()
                } else {
                    crate::i18n::tr("card_tradition_undefined", current_lang).to_string()
                };
                let essence_display = if !summary.essence.is_empty() {
                    summary.essence.clone()
                } else if is_gm {
                    "Gods & Monsters".to_string()
                } else {
                    crate::i18n::tr("card_essence_awakened", current_lang).to_string()
                };
                let arete_val = summary.arete;
                let wp_val = summary.willpower;
                let date_display = summary.updated_at.split(' ').next().unwrap_or(&summary.updated_at).to_string();
                let updated_at_full = summary.updated_at.clone();
                let id_nav = id.clone();

                let is_dragging = move || dragged_sheet_id.get().as_deref() == Some(id_card.as_str());

                view! {
                    <div
                        class="character-card"
                        class:is-dragging=is_dragging
                        draggable=if is_owner { "true" } else { "false" }
                        on:dragstart={
                            let id_drag = id.clone();
                            move |ev: ev::DragEvent| {
                                set_dragged_sheet_id.set(Some(id_drag.clone()));
                                if let Some(dt) = ev.data_transfer() {
                                    let _ = dt.set_data("text/plain", &id_drag);
                                    let _ = dt.set_effect_allowed("move");
                                }
                            }
                        }
                        on:dragend=move |_| {
                            set_dragged_sheet_id.set(None);
                            set_drag_over_folder_id.set(None);
                        }
                        on:click=move |_| {
                            if dragged_sheet_id.get_untracked().is_some() { return; }
                            use_navigate()(&format!("/sheet/{}", id_nav), Default::default())
                        }
                    >
                        <div class="card-portrait-box">
                            {if has_photo {
                                let img_style = format!("object-position: {}% {}%;", summary.photo_focus_x, summary.photo_focus_y);
                                view! {
                                    <img
                                        src=photo
                                        alt=summary.name.clone()
                                        class="card-portrait-img"
                                        style=img_style
                                    />
                                }.into_view()
                            } else {
                                view! {
                                    <div class="card-portrait-placeholder">
                                        <span class="placeholder-icon">{if is_gm { "🐉" } else { "🔮" }}</span>
                                        <span class="placeholder-tag">{if is_gm { "Gods & Monsters".to_string() } else { crate::i18n::tr("card_no_image", current_lang).to_string() }}</span>
                                    </div>
                                }.into_view()
                            }}
                            <div class="card-portrait-gradient"></div>
                            {if is_owner {
                                let summary_for_move = summary_clone.clone();
                                let summary_for_del = summary_clone.clone();
                                view! {
                                    <div class="card-floating-actions" draggable="false">
                                        <button
                                            type="button"
                                            draggable="false"
                                            class="card-float-action-btn card-move-floating-btn"
                                            on:click=move |ev: ev::MouseEvent| {
                                                ev.stop_propagation();
                                                set_sheet_to_move.set(Some(summary_for_move.clone()));
                                            }
                                            title=crate::i18n::tr("card_move_tooltip", current_lang)
                                        >
                                            "📁"
                                        </button>
                                        <button
                                            type="button"
                                            draggable="false"
                                            class="card-float-action-btn card-delete-floating-btn"
                                            on:click=move |ev: ev::MouseEvent| {
                                                ev.stop_propagation();
                                                set_sheet_to_delete.set(Some(summary_for_del.clone()));
                                            }
                                            title=crate::i18n::tr("card_delete_tooltip", current_lang)
                                        >
                                            "🗑️"
                                        </button>
                                    </div>
                                }.into_view()
                            } else {
                                ().into_view()
                            }}
                        </div>

                        <div class="card-content">
                            <div class="card-header-info">
                                <h3 class="card-name" title=summary.name.clone()>{summary.name.clone()}</h3>
                                <div class="card-meta-tags">
                                    {if is_gm {
                                        view! {
                                            <span class="meta-tag type-badge-gm">{crate::i18n::tr("card_tag_gm", current_lang)}</span>
                                        }.into_view()
                                    } else {
                                        view! {
                                            <span class="meta-tag type-badge-mage">{crate::i18n::tr("card_tag_mage", current_lang)}</span>
                                        }.into_view()
                                    }}
                                    {summary.folder_id.as_ref().and_then(|fid| folders_info.get(fid)).map(|(fname, ficon)| {
                                        view! {
                                            <span class="meta-tag folder-tag" title=format!("Pasta: {}", fname)>
                                                {ficon.clone()} " " {fname.clone()}
                                            </span>
                                        }
                                    })}
                                    <span class="meta-tag tradition-tag" title=tradition_display.clone()>{tradition_display}</span>
                                    <span class="meta-tag essence-tag" title=essence_display.clone()>{essence_display}</span>
                                    {if is_owner {
                                        view! {
                                            <button
                                                class=if is_pub_val { "meta-tag vis-tag vis-public" } else { "meta-tag vis-tag vis-private" }
                                                title=if is_pub_val { crate::i18n::tr("card_vis_public_tt", current_lang) } else { crate::i18n::tr("card_vis_private_tt", current_lang) }
                                                on:click=move |ev: ev::MouseEvent| {
                                                    ev.stop_propagation();
                                                    toggle_privacy(id_vis.clone(), is_pub_val);
                                                }
                                            >
                                                {if is_pub_val { crate::i18n::tr("card_vis_public", current_lang) } else { crate::i18n::tr("card_vis_private", current_lang) }}
                                            </button>
                                        }.into_view()
                                    } else {
                                        view! {
                                            <span class="meta-tag vis-tag vis-public">
                                                {crate::i18n::tr("card_vis_public", current_lang)}
                                            </span>
                                        }.into_view()
                                    }}
                                </div>
                            </div>

                            <div class="card-stats-preview">
                                <div class="card-stat-item">
                                    <span class="stat-label">{if is_gm { crate::i18n::tr("card_gnosis", current_lang) } else { crate::i18n::tr("card_arete", current_lang) }}</span>
                                    <div class="stat-dots arete-dots">
                                        {(1..=if is_gm { 10 } else { 5 }).map(|idx| {
                                            let filled = idx <= arete_val;
                                            view! {
                                                <span class=if filled { "stat-dot filled-arete" } else { "stat-dot empty-dot" }></span>
                                            }
                                        }).collect_view()}
                                    </div>
                                    <span class="stat-number">{arete_val}</span>
                                </div>

                                <div class="card-stat-item">
                                    <span class="stat-label">{crate::i18n::tr("card_willpower", current_lang)}</span>
                                    <div class="stat-dots wp-dots">
                                        {(1..=10).map(|idx| {
                                            let filled = idx <= wp_val;
                                            view! {
                                                <span class=if filled { "stat-dot filled-wp" } else { "stat-dot empty-dot" }></span>
                                            }
                                        }).collect_view()}
                                    </div>
                                    <span class="stat-number">{wp_val}</span>
                                </div>
                            </div>

                            {if !is_gm {
                                view! {
                                    <div class="card-spheres-preview">
                                        <div class="spheres-header-row">
                                            <span class="spheres-label">{crate::i18n::tr("card_spheres_title", current_lang)}</span>
                                        </div>
                                        <div class="spheres-9-grid">
                                            {summary.spheres.iter().map(|(sphere_name, lvl)| {
                                                let s_name = crate::i18n::tr_sphere(sphere_name, current_lang).to_string();
                                                let s_lvl = *lvl;
                                                let is_active = s_lvl > 0;
                                                let level_label = match current_lang {
                                                    crate::i18n::Language::PtBr => "nível",
                                                    crate::i18n::Language::EnUs => "level",
                                                };
                                                view! {
                                                    <div
                                                        class=if is_active { "sphere-item-active" } else { "sphere-item-inactive" }
                                                        title=format!("{}: {} {}", s_name, level_label, s_lvl)
                                                    >
                                                        <span class="sphere-mini-name">{s_name}</span>
                                                        <div class="sphere-mini-dots">
                                                            {(1..=5).map(|dot_i| {
                                                                let filled = dot_i <= s_lvl;
                                                                view! {
                                                                    <span class=if filled { "stat-dot filled-sphere" } else { "stat-dot empty-dot" }></span>
                                                                }
                                                            }).collect_view()}
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <div class="card-gm-badge-footer">
                                        <span class="gm-creature-desc">{crate::i18n::tr("card_gm_footer_desc", current_lang)}</span>
                                    </div>
                                }.into_view()
                            }}

                            <div class="card-footer">
                                <span class="card-date" title=format!("{}: {}", match current_lang { crate::i18n::Language::PtBr => "Última alteração", crate::i18n::Language::EnUs => "Last update" }, updated_at_full)>
                                    {crate::i18n::tr("card_updated", current_lang)} " " {date_display}
                                </span>
                                <span class="card-cta">{crate::i18n::tr("card_open_cta", current_lang)}</span>
                            </div>
                        </div>
                    </div>
                }
            }).collect_view()}
        </div>
    }.into_view()
}
