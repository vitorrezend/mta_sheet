//! Mini ficha de NPC: vitalidade clicável, Força de Vontade, iniciativa e, aberta,
//! os Atributos em bolinhas, as outras linhas da ficha e notas.

use leptos::*;
use crate::compendium::bestiary::find_bestiary_entry;
use crate::components::mta_sheet::page1::advantages::HealthDamageIcon;
use crate::i18n::Language;
use crate::state::{DamageType, NpcCard, NPC_ATTRIBUTES};
use super::{attribute_label, is_drawn_stat_label, is_flavor_label, stat_lines_for, use_npc_tray, NpcInitiativeRequest, NpcTokenRequest};

fn pt_en(lang: Language, pt: &'static str, en: &'static str) -> &'static str {
    match lang {
        Language::PtBr => pt,
        Language::EnUs => en,
    }
}

/// Classe de cor do estado: ok, bruised, hurt, bad, crit, down.
fn severity(card: &NpcCard) -> &'static str {
    if card.is_down() {
        "down"
    } else if card.total_damage() == 0 {
        "ok"
    } else {
        match card.wound_penalty() {
            0 => "bruised",
            1 => "hurt",
            2 | 3 => "bad",
            _ => "crit",
        }
    }
}

#[component]
pub fn MiniSheet(uid: String) -> impl IntoView {
    let Some(tray) = use_npc_tray() else { return ().into_view() };
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = Signal::derive(move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default());

    let uid = store_value(uid);
    let card = create_memo(move |_| uid.with_value(|u| tray.cards.with(|c| c.iter().find(|x| &x.uid == u).cloned())));
    let edit = move |f: Box<dyn FnOnce(&mut NpcCard)>| uid.with_value(|u| tray.update_card(u, f));
    let is_expanded = Signal::derive(move || uid.with_value(|u| tray.expanded.get().as_deref() == Some(u.as_str())));
    let confirm_remove = create_rw_signal(false);

    view! {
        {move || card.get().map(|c| {
            let l = lang.get();
            let sev = severity(&c);
            // Lidos aqui (e não em blocos dinâmicos aninhados): o cartão inteiro redesenha
            let expanded = is_expanded.get();
            let initiative_ready = tray.initiative_ready.get();
            let map_ready = tray.map_ready.get();
            let has_room_actions = initiative_ready || map_ready;
            view! {
                <article class=format!("mini-sheet sev-{}", sev) class:is-expanded=expanded>
                    <header class="mini-sheet-head">
                        <input
                            class="mini-sheet-name"
                            prop:value=c.name.clone()
                            title=pt_en(l, "Nome do NPC (clique para renomear)", "NPC name (click to rename)")
                            on:change=move |ev| {
                                let v = event_target_value(&ev);
                                if !v.trim().is_empty() {
                                    edit(Box::new(move |c| c.name = v.trim().to_string()));
                                }
                            }
                        />
                        <span class=format!("mini-sheet-status status-{}", sev)>{c.health_status(l)}</span>
                        <div class="mini-sheet-tools">
                            <button
                                type="button"
                                class="mini-icon-btn"
                                title=pt_en(l, "Mostrar Atributos e detalhes", "Show Attributes and details")
                                on:click=move |_| uid.with_value(|u| tray.expanded.update(|e| {
                                    *e = if e.as_deref() == Some(u.as_str()) { None } else { Some(u.clone()) };
                                }))
                            >
                                {if expanded { "▾" } else { "▸" }}
                            </button>
                            <button
                                type="button"
                                class="mini-icon-btn"
                                title=pt_en(l, "Duplicar (outro NPC igual, sem dano)", "Duplicate (same NPC, no damage)")
                                on:click=move |_| uid.with_value(|u| tray.duplicate(u))
                            >"⧉"</button>
                            <button
                                type="button"
                                class="mini-icon-btn danger"
                                class:confirm=confirm_remove
                                title=pt_en(l, "Remover mini ficha", "Remove mini sheet")
                                on:click=move |_| {
                                    if confirm_remove.get_untracked() {
                                        uid.with_value(|u| tray.remove(u));
                                    } else {
                                        confirm_remove.set(true);
                                        set_timeout(move || { let _ = confirm_remove.try_set(false); }, std::time::Duration::from_secs(3));
                                    }
                                }
                            >{move || if confirm_remove.get() { "✓?" } else { "✕" }}</button>
                        </div>
                    </header>

                    {health_view(&c, l, edit)}
                    {willpower_view(&c, l, edit)}
                    {chips_view(&c, l)}

                    {has_room_actions.then(|| view! {
                    <div class="mini-room-actions">
                        {initiative_ready.then(|| {
                            view! {
                                <button
                                    type="button"
                                    class="mini-action-btn"
                                    title=pt_en(l, "Colocar este NPC no rastreador de iniciativa da sala", "Add this NPC to the room's initiative tracker")
                                    on:click=move |_| if let Some(c) = card.get_untracked() {
                                        tray.initiative_request.set(Some(NpcInitiativeRequest {
                                            name: c.name.clone(),
                                            base: c.initiative_base.unwrap_or(1),
                                            penalty: c.wound_penalty(),
                                        }));
                                    }
                                >{pt_en(l, "⚔️ Na iniciativa", "⚔️ To initiative")}</button>
                            }
                        })}
                        {map_ready.then(|| {
                            view! {
                                <button
                                    type="button"
                                    class="mini-action-btn"
                                    title=pt_en(l, "Criar um token de inimigo no mapa", "Create an enemy token on the map")
                                    on:click=move |_| if let Some(c) = card.get_untracked() {
                                        tray.token_request.set(Some(NpcTokenRequest { name: c.name.clone(), color: "#b91c1c".into() }));
                                    }
                                >{pt_en(l, "🗺️ Token no mapa", "🗺️ Map token")}</button>
                            }
                        })}
                    </div>
                    })}

                    {expanded.then(|| details_view(&c, l, edit))}
                </article>
            }
        })}
    }
    .into_view()
}

fn health_view(c: &NpcCard, l: Language, edit: impl Fn(Box<dyn FnOnce(&mut NpcCard)>) + Copy + 'static) -> View {
    if !c.has_health_track() {
        let Some(max) = c.essence_max else { return ().into_view() };
        let pct = if max > 0 { c.essence * 100 / max } else { 0 };
        return view! {
            <div class="mini-row">
                <span class="mini-label">{pt_en(l, "Essência", "Essence")}</span>
                <div class="mini-essence">
                    <button type="button" class="mini-step-btn" on:click=move |_| edit(Box::new(|c| c.adjust_essence(-5)))>"−5"</button>
                    <button type="button" class="mini-step-btn" on:click=move |_| edit(Box::new(|c| c.adjust_essence(-1)))>"−1"</button>
                    <div class="mini-essence-bar" title=pt_en(l, "Espíritos perdem Essência no lugar de vitalidade", "Spirits lose Essence instead of health")>
                        <div class="mini-essence-fill" style=format!("width: {}%", pct)></div>
                        <span class="mini-essence-text">{format!("{} / {}", c.essence, max)}</span>
                    </div>
                    <button type="button" class="mini-step-btn" on:click=move |_| edit(Box::new(|c| c.adjust_essence(1)))>"+1"</button>
                </div>
            </div>
        }
        .into_view();
    }
    let last = c.health_boxes() - 1;
    view! {
        <div class="mini-row mini-row-health">
            <span class="mini-label">{pt_en(l, "Vitalidade", "Health")}</span>
            <div class="mini-health">
                {c.health_labels.iter().enumerate().map(|(i, label)| {
                    let dmg = c.damage_at(i);
                    let shown = if i == last { "✝".to_string() } else if label == "OK" { "·".to_string() } else { label.clone() };
                    let tip = if i == last && l == Language::PtBr { crate::state::final_label_pt(label).to_string() } else { label.clone() };
                    view! {
                        <div class="mini-health-cell" title=format!("{} · {}", tip, pt_en(l, "clique: / → ✕ → ✳ · botão direito: curar", "click: / → ✕ → ✳ · right click: heal"))>
                            <div
                                class="health-box mini-health-box"
                                class:damage-bashing=dmg == DamageType::Bashing
                                class:damage-lethal=dmg == DamageType::Lethal
                                class:damage-aggravated=dmg == DamageType::Aggravated
                                on:click=move |_| edit(Box::new(move |c| c.click_box(i)))
                                on:contextmenu=move |ev: ev::MouseEvent| {
                                    ev.prevent_default();
                                    edit(Box::new(|c| c.heal(1)));
                                }
                            >
                                <HealthDamageIcon dmg=dmg />
                            </div>
                            <span class="mini-health-pen">{shown}</span>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
        <div class="mini-dmg-actions">
            <button type="button" class="mini-dmg-btn dmg-b" title=pt_en(l, "+1 dano contundente", "+1 bashing damage") on:click=move |_| edit(Box::new(|c| c.apply_damage(DamageType::Bashing, 1)))>"+ /"</button>
            <button type="button" class="mini-dmg-btn dmg-l" title=pt_en(l, "+1 dano letal", "+1 lethal damage") on:click=move |_| edit(Box::new(|c| c.apply_damage(DamageType::Lethal, 1)))>"+ ✕"</button>
            <button type="button" class="mini-dmg-btn dmg-a" title=pt_en(l, "+1 dano agravado", "+1 aggravated damage") on:click=move |_| edit(Box::new(|c| c.apply_damage(DamageType::Aggravated, 1)))>"+ ✳"</button>
            <button type="button" class="mini-dmg-btn heal" title=pt_en(l, "Cura 1 nível (o dano mais leve primeiro)", "Heal 1 level (lightest damage first)") on:click=move |_| edit(Box::new(|c| c.heal(1)))>{pt_en(l, "− Curar", "− Heal")}</button>
            <button type="button" class="mini-dmg-btn reset" title=pt_en(l, "Limpar todo o dano", "Clear all damage") on:click=move |_| edit(Box::new(|c| c.clear_damage()))>"🧹"</button>
        </div>
    }
    .into_view()
}

fn willpower_view(c: &NpcCard, l: Language, edit: impl Fn(Box<dyn FnOnce(&mut NpcCard)>) + Copy + 'static) -> View {
    if c.willpower_max <= 0 {
        return ().into_view();
    }
    let current = c.willpower;
    view! {
        <div class="mini-row">
            <span class="mini-label">{pt_en(l, "Força de Vontade", "Willpower")}</span>
            <div class="mini-squares">
                {(1..=c.willpower_max).map(|i| view! {
                    <span
                        class="square mini-square"
                        class:filled={i <= current}
                        title=pt_en(l, "Clique para gastar ou recuperar", "Click to spend or regain")
                        on:click=move |_| edit(Box::new(move |c| {
                            let v = if i == c.willpower { i - 1 } else { i };
                            c.set_willpower(v);
                        }))
                    ></span>
                }).collect_view()}
                <span class="mini-count">{format!("{}/{}", current, c.willpower_max)}</span>
            </div>
        </div>
    }
    .into_view()
}

fn chips_view(c: &NpcCard, l: Language) -> View {
    let mut chips: Vec<(String, String)> = Vec::new();
    if let Some(now) = c.initiative_now() {
        let base = c.initiative_base.unwrap_or(now);
        let tip = if now < base {
            format!("{} {} − {} {}", pt_en(l, "Base", "Base"), base, c.wound_penalty(), pt_en(l, "de ferimento", "wound penalty"))
        } else {
            pt_en(l, "Destreza + Raciocínio", "Dexterity + Wits").to_string()
        };
        chips.push((format!("{} {} + 1d10", pt_en(l, "Iniciativa", "Initiative"), now), tip));
    }
    if c.wound_penalty() > 0 && !c.is_down() {
        chips.push((format!("−{} {}", c.wound_penalty(), pt_en(l, "dados", "dice")), pt_en(l, "Penalidade de ferimento nas paradas", "Wound penalty to dice pools").to_string()));
    }
    if let Some(a) = c.arete {
        chips.push((format!("Arete {}", a), String::new()));
    }
    for (pt, en, v) in [("Fúria", "Rage", c.rage), ("Gnose", "Gnosis", c.gnosis)] {
        if let Some(v) = v {
            chips.push((format!("{} {}", pt_en(l, pt, en), v), String::new()));
        }
    }
    if chips.is_empty() {
        return ().into_view();
    }
    view! {
        <div class="mini-chips">
            {chips.into_iter().map(|(t, tip)| view! { <span class="mini-chip" title=tip>{t}</span> }).collect_view()}
        </div>
    }
    .into_view()
}

fn details_view(c: &NpcCard, l: Language, edit: impl Fn(Box<dyn FnOnce(&mut NpcCard)>) + Copy + 'static) -> View {
    let groups: [(&str, &str, &[&str]); 3] = [
        ("Físicos", "Physical", &NPC_ATTRIBUTES[0..3]),
        ("Sociais", "Social", &NPC_ATTRIBUTES[3..6]),
        ("Mentais", "Mental", &NPC_ATTRIBUTES[6..9]),
    ];
    let has_any_attr = !c.attributes.is_empty();
    let lines: Vec<(&'static str, &'static str)> = c
        .source_id
        .as_deref()
        .and_then(find_bestiary_entry)
        .map(|e| stat_lines_for(e, l).into_iter().filter(|(lab, _)| !is_drawn_stat_label(lab) && !is_flavor_label(lab)).collect())
        .unwrap_or_default();
    let notes = c.notes.clone();

    view! {
        <div class="mini-sheet-details">
            {has_any_attr.then(|| view! {
                <div class="mini-attr-grid">
                    {groups.into_iter().map(|(pt, en, names)| view! {
                        <div class="mini-attr-col">
                            <div class="mini-attr-col-title">{pt_en(l, pt, en)}</div>
                            {names.iter().map(|name| {
                                let name: &'static str = *name;
                                let attr = c.attributes.iter().find(|a| a.name == name).cloned();
                                let value = attr.as_ref().map(|a| a.value);
                                let range = attr.and_then(|a| a.range);
                                let max = value.unwrap_or(0).max(5);
                                view! {
                                    <div class="mini-attr-row" class:is-missing=value.is_none()>
                                        <span class="mini-attr-name">{attribute_label(name, l)}</span>
                                        {match value {
                                            Some(v) => view! {
                                                <span class="mini-attr-dots" title=range.clone().map(|r| format!("{} {}", pt_en(l, "O livro sugere", "Book suggests"), r)).unwrap_or_default()>
                                                    {(1..=max).map(|i| view! {
                                                        <span
                                                            class="dot mini-dot"
                                                            class:filled={i <= v}
                                                            on:click=move |_| edit(Box::new(move |c| {
                                                                let cur = c.attribute(name).unwrap_or(0);
                                                                c.set_attribute(name, if i == cur { i - 1 } else { i });
                                                            }))
                                                        ></span>
                                                    }).collect_view()}
                                                    {range.map(|r| view! { <span class="mini-attr-range">{r}</span> })}
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

            {(!lines.is_empty()).then(|| view! {
                <dl class="mini-stat-lines">
                    {lines.into_iter().map(|(label, value)| view! {
                        <div class="mini-stat-line">
                            <dt>{label}</dt>
                            <dd>{value.replace('*', "")}</dd>
                        </div>
                    }).collect_view()}
                </dl>
            })}

            <textarea
                class="mini-notes"
                rows="2"
                placeholder=pt_en(l, "Notas da cena (só neste navegador)...", "Scene notes (this browser only)...")
                prop:value=notes
                on:change=move |ev| {
                    let v = event_target_value(&ev);
                    edit(Box::new(move |c| c.notes = v));
                }
            ></textarea>
        </div>
    }
    .into_view()
}
