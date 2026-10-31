use leptos::*;
use crate::compendium::bestiary::{find_bestiary_entry, BestiaryEntry, BestiaryGroup, ALL_BESTIARY};
use crate::components::compendium::rich_text::RichTextView;
use crate::components::npc_tray::{attribute_label, has_mini_sheet, is_drawn_stat_label, is_flavor_label, stat_lines_for, use_npc_tray};
use crate::i18n::Language;
use crate::state::{final_label_pt, NPC_ATTRIBUTES};

fn pt_en(lang: Language, pt: &'static str, en: &'static str) -> &'static str {
    match lang {
        Language::PtBr => pt,
        Language::EnUs => en,
    }
}

/// Texto corrido da ficha: sem o título, a linha de fonte e as linhas `- **Rótulo:** valor`.
fn prose_of(content: &str) -> String {
    content
        .lines()
        .filter(|l| {
            let t = l.trim();
            !(t.starts_with("### ")
                || t.starts_with("> **Source:**")
                || t.starts_with("> **Fonte:**")
                || (t.starts_with("- **") && t.contains(":** ")))
        })
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

/// Resumo curto da lista: "FV 3 · 7 vit." ou "Ess. 40".
fn list_meta(e: &BestiaryEntry, lang: Language) -> String {
    let s = e.quick_stats();
    let mut parts = Vec::new();
    if let Some(w) = s.willpower {
        parts.push(format!("{} {}", pt_en(lang, "FV", "WP"), w));
    }
    if !s.health_levels.is_empty() {
        parts.push(format!("{} {}", s.health_levels.len(), pt_en(lang, "vit.", "HL")));
    }
    if let Some(es) = s.essence {
        parts.push(format!("Ess. {}", es));
    }
    parts.join(" · ")
}

#[component]
pub fn BestiaryView(
    selected_bestiary_id: RwSignal<String>,
    current_lang: Signal<Language>,
    #[prop(into, default = None)] mobile_show_detail: Option<RwSignal<bool>>,
) -> impl IntoView {
    let active_entry = Signal::derive(move || {
        find_bestiary_entry(&selected_bestiary_id.get()).unwrap_or(&ALL_BESTIARY[0])
    });
    let filter = create_rw_signal(String::new());
    let open_groups = create_rw_signal(Vec::<BestiaryGroup>::new());
    let tray = use_npc_tray();

    // O grupo da ficha aberta fica sempre expandido (inclusive quando ela chega por link)
    create_effect(move |_| {
        let g = active_entry.get().group;
        if !open_groups.with_untracked(|o| o.contains(&g)) {
            open_groups.update(|o| o.push(g));
        }
    });

    let select = move |id: &'static str| {
        selected_bestiary_id.set(id.to_string());
        if let Some(msd) = mobile_show_detail {
            msd.set(true);
        }
    };

    view! {
        <div
            class="compendium-section-split"
            class:mobile-show-detail=move || mobile_show_detail.map(|s| s.get()).unwrap_or(false)
        >
            // ================= APÊNDICE I: Coluna Esquerda (grupos do livro) =================
            <div class="practice-sidebar-pane bestiary-sidebar">
                <input
                    class="bestiary-filter"
                    type="search"
                    placeholder=move || pt_en(current_lang.get(), "Filtrar fichas...", "Filter entries...")
                    prop:value=filter
                    on:input=move |ev| filter.set(event_target_value(&ev))
                />
                {BestiaryGroup::ALL.iter().map(|group| {
                    let group = *group;
                    let entries: Vec<&'static BestiaryEntry> = ALL_BESTIARY.iter().filter(|e| e.group == group).collect();
                    let sheets = entries.iter().filter(|e| !e.is_article).count();
                    let is_open = Signal::derive(move || !filter.get().trim().is_empty() || open_groups.with(|o| o.contains(&group)));
                    let entries_for_list = entries.clone();
                    let has_match = Signal::derive(move || {
                        let q = filter.get().trim().to_lowercase();
                        q.is_empty() || entries.iter().any(|e| e.name.to_lowercase().contains(&q) || e.name_pt.to_lowercase().contains(&q))
                    });
                    view! {
                        <div class="bestiary-group" class:is-open=is_open class:is-hidden=move || !has_match.get()>
                            <button
                                type="button"
                                class="bestiary-group-head"
                                on:click=move |_| open_groups.update(|o| {
                                    if let Some(i) = o.iter().position(|g| *g == group) { o.remove(i); } else { o.push(group); }
                                })
                            >
                                <span class="bestiary-group-caret">{move || if is_open.get() { "▾" } else { "▸" }}</span>
                                <span class="bestiary-group-name">{move || group.label(current_lang.get())}</span>
                                {(sheets > 0).then(|| view! { <span class="bestiary-group-count">{sheets}</span> })}
                            </button>
                            <Show when=move || is_open.get()>
                                <div class="bestiary-group-items">
                                    {entries_for_list.iter().map(|e| {
                                        let e: &'static BestiaryEntry = e;
                                        let e_id = e.id;
                                        let is_active = Signal::derive(move || selected_bestiary_id.get() == e_id);
                                        let visible = Signal::derive(move || {
                                            let q = filter.get().trim().to_lowercase();
                                            q.is_empty() || e.name.to_lowercase().contains(&q) || e.name_pt.to_lowercase().contains(&q)
                                        });
                                        view! {
                                            <button
                                                type="button"
                                                class="bestiary-item"
                                                class:is-article=e.is_article
                                                class:active=is_active
                                                class:is-hidden=move || !visible.get()
                                                on:click=move |_| select(e_id)
                                            >
                                                <span class="bestiary-item-name">
                                                    {if e.is_article { "📜 " } else { "" }}
                                                    {move || e.name(current_lang.get())}
                                                </span>
                                                <span class="bestiary-item-meta">
                                                    {move || if e.is_article { e.page_ref.replace("M20, ", "") } else { list_meta(e, current_lang.get()) }}
                                                </span>
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>
                            </Show>
                        </div>
                    }
                }).collect_view()}
            </div>

            // ================= APÊNDICE I: Coluna Direita (Detalhe) =================
            <div class="practice-detail-pane">
                {move || mobile_show_detail.map(|msd| {
                    view! {
                        <button
                            type="button"
                            class="compendium-mobile-back-btn"
                            on:click=move |_| msd.set(false)
                        >
                            <span class="back-arrow">"←"</span>
                            <span>{move || pt_en(current_lang.get(), "Voltar para a Lista", "Back to List")}</span>
                        </button>
                    }
                })}

                {move || {
                    let entry = active_entry.get();
                    let lang = current_lang.get();
                    let structured = has_mini_sheet(entry);
                    view! {
                        <div class="practice-reading-view bestiary-reading">
                            <div class=if entry.is_article { "practice-detail-header special-box-header" } else { "practice-detail-header" }>
                                <div class="practice-title-row">
                                    <div class="practice-title-left">
                                        <div class="practice-main-name">
                                            {if entry.is_article { "📜 " } else { "" }} {entry.name(lang)}
                                        </div>
                                        <span class="practice-page-badge">"📖 " {entry.page_ref}</span>
                                    </div>
                                    {(structured && tray.is_some()).then(|| view! {
                                        <button
                                            type="button"
                                            class="bestiary-mini-btn"
                                            title=pt_en(lang, "Cria uma mini ficha descartável com vitalidade e Força de Vontade para marcar na cena", "Creates a throwaway mini sheet with health and Willpower to track in the scene")
                                            on:click=move |_| if let Some(t) = tray { t.add_from_bestiary(entry, current_lang.get_untracked()) }
                                        >
                                            {pt_en(lang, "💀 Criar mini ficha", "💀 Create mini sheet")}
                                        </button>
                                    })}
                                </div>
                                <div class="bestiary-subtitle">
                                    <span class="bestiary-group-tag">{entry.group.label(lang)}</span>
                                </div>
                            </div>

                            {if structured {
                                stat_block(entry, lang).into_view()
                            } else {
                                view! {
                                    <div class="practice-description-text compendium-rich-content">
                                        <RichTextView text=Signal::derive(move || entry.content(current_lang.get()).to_string()) />
                                    </div>
                                }.into_view()
                            }}
                        </div>
                    }.into_view()
                }}
            </div>
        </div>
    }
}

/// Ficha estruturada: números no topo (bolinhas e caixas), depois as outras linhas,
/// a descrição e, por último, Imagem e Notas de Interpretação.
fn stat_block(entry: &'static BestiaryEntry, lang: Language) -> impl IntoView {
    let stats = entry.quick_stats();
    let ranges = entry.attribute_ranges();
    let lines = stat_lines_for(entry, lang);
    let table: Vec<_> = lines.iter().filter(|(l, _)| !is_drawn_stat_label(l) && !is_flavor_label(l)).cloned().collect();
    let flavor: Vec<_> = lines.iter().filter(|(l, _)| is_flavor_label(l)).cloned().collect();
    // Linhas desenhadas que têm algo além dos números (ex.: "use Gnosis for Social...")
    let drawn_notes: Vec<_> = lines
        .iter()
        .filter(|(l, v)| is_drawn_stat_label(l) && (v.contains("use ") || v.contains("usa") || v.contains("(")))
        .cloned()
        .collect();
    let prose = prose_of(entry.content(lang));

    let mut chips: Vec<String> = Vec::new();
    if let Some(i) = stats.initiative_base() {
        chips.push(format!("{} {} + 1d10", pt_en(lang, "Iniciativa", "Initiative"), i));
    }
    if let Some(a) = stats.arete {
        chips.push(format!("Arete {}", a));
    }
    for (pt, en, v) in [("Fúria", "Rage", stats.rage), ("Gnose", "Gnosis", stats.gnosis), ("Essência", "Essence", stats.essence)] {
        if let Some(v) = v {
            chips.push(format!("{} {}", pt_en(lang, pt, en), v));
        }
    }

    let groups: [(&str, &str, &[&str]); 3] = [
        ("Físicos", "Physical", &NPC_ATTRIBUTES[0..3]),
        ("Sociais", "Social", &NPC_ATTRIBUTES[3..6]),
        ("Mentais", "Mental", &NPC_ATTRIBUTES[6..9]),
    ];
    let last_hl = stats.health_levels.len().saturating_sub(1);

    view! {
        <section class="bestiary-statblock">
            {(!ranges.is_empty()).then(|| view! {
                <div class="mini-attr-grid bestiary-attr-grid">
                    {groups.into_iter().map(|(pt, en, names)| view! {
                        <div class="mini-attr-col">
                            <div class="mini-attr-col-title">{pt_en(lang, pt, en)}</div>
                            {names.iter().map(|name| {
                                let r = ranges.iter().find(|(n, _, _)| n.as_str() == *name).cloned();
                                view! {
                                    <div class="mini-attr-row" class:is-missing=r.is_none()>
                                        <span class="mini-attr-name">{attribute_label(name, lang)}</span>
                                        {match r {
                                            Some((_, lo, hi)) => view! {
                                                <span class="mini-attr-dots">
                                                    {(1..=hi.max(5)).map(|i| view! {
                                                        <span class="dot mini-dot is-static" class:filled={i <= lo} class:is-range={i > lo && i <= hi}></span>
                                                    }).collect_view()}
                                                    {(hi > lo).then(|| view! { <span class="mini-attr-range">{format!("{}-{}", lo, hi)}</span> })}
                                                </span>
                                            }.into_view(),
                                            None => view! { <span class="mini-attr-none">"—"</span> }.into_view(),
                                        }}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }).collect_view()}
                </div>
            })}

            <div class="bestiary-vitals">
                {stats.willpower.map(|w| view! {
                    <div class="bestiary-vital">
                        <span class="mini-label">{pt_en(lang, "Força de Vontade", "Willpower")}</span>
                        <span class="mini-squares">
                            {(1..=w).map(|_| view! { <span class="square mini-square filled is-static"></span> }).collect_view()}
                            <span class="mini-count">{w}</span>
                        </span>
                    </div>
                })}
                {(!stats.health_levels.is_empty()).then(|| view! {
                    <div class="bestiary-vital">
                        <span class="mini-label">{pt_en(lang, "Vitalidade", "Health")}</span>
                        <span class="mini-health">
                            {stats.health_levels.iter().enumerate().map(|(i, label)| {
                                let shown = if i == last_hl {
                                    if lang == Language::PtBr { final_label_pt(label).to_string() } else { label.clone() }
                                } else if label == "OK" { "·".to_string() } else { label.clone() };
                                view! {
                                    <span class="mini-health-cell" title=label.clone()>
                                        <span class="health-box mini-health-box is-static"></span>
                                        <span class="mini-health-pen" class:is-final={i == last_hl}>{shown}</span>
                                    </span>
                                }
                            }).collect_view()}
                        </span>
                    </div>
                })}
                {(!chips.is_empty()).then(|| view! {
                    <div class="mini-chips">
                        {chips.into_iter().map(|c| view! { <span class="mini-chip">{c}</span> }).collect_view()}
                    </div>
                })}
            </div>

            {(!table.is_empty() || !drawn_notes.is_empty()).then(|| view! {
                <dl class="mini-stat-lines bestiary-stat-lines">
                    {drawn_notes.into_iter().chain(table).map(|(label, value)| view! {
                        <div class="mini-stat-line">
                            <dt>{label}</dt>
                            <dd>{value.replace('*', "")}</dd>
                        </div>
                    }).collect_view()}
                </dl>
            })}
        </section>

        {(!prose.is_empty()).then(|| view! {
            <div class="practice-description-text compendium-rich-content">
                <RichTextView text=Signal::derive(move || prose.clone()) />
            </div>
        })}

        {flavor.into_iter().map(|(label, value)| view! {
            <div class="practice-quote bestiary-flavor">
                <strong class="bestiary-flavor-label">{label}</strong>
                <span>{value.replace('*', "")}</span>
            </div>
        }).collect_view()}
    }
}
