//! Painel lateral das mini fichas: lista do encontro, busca no Apêndice I e o botão
//! flutuante que reabre o painel quando há fichas.

use leptos::*;
use crate::compendium::bestiary::{BestiaryGroup, ALL_BESTIARY};
use crate::i18n::Language;
use super::{has_mini_sheet, use_npc_tray, MiniSheet};

#[component]
pub fn NpcTrayDock() -> impl IntoView {
    let Some(tray) = use_npc_tray() else { return ().into_view() };
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = Signal::derive(move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default());
    let t = move |pt: &'static str, en: &'static str| if lang.get() == Language::PtBr { pt } else { en };

    let picker_open = create_rw_signal(false);
    let search = create_rw_signal(String::new());
    let confirm_clear = create_rw_signal(false);
    let count = Signal::derive(move || tray.cards.with(Vec::len));

    // Abre a busca sozinha quando o painel abre vazio
    create_effect(move |_| {
        if tray.open.get() && tray.cards.with_untracked(Vec::is_empty) {
            picker_open.set(true);
        }
    });

    let picker = move || {
        let q = search.get().trim().to_lowercase();
        let l = lang.get();
        BestiaryGroup::ALL
            .iter()
            .filter_map(|g| {
                let items: Vec<_> = ALL_BESTIARY
                    .iter()
                    .filter(|e| e.group == *g && has_mini_sheet(e))
                    .filter(|e| q.is_empty() || e.name.to_lowercase().contains(&q) || e.name_pt.to_lowercase().contains(&q))
                    .collect();
                (!items.is_empty()).then(|| view! {
                    <div class="npc-picker-group">{g.label(l)}</div>
                    {items.into_iter().map(|e| {
                        let s = e.quick_stats();
                        let mut meta = Vec::new();
                        if let Some(w) = s.willpower {
                            meta.push(format!("{} {}", if l == Language::PtBr { "FV" } else { "WP" }, w));
                        }
                        if !s.health_levels.is_empty() {
                            meta.push(format!("{} {}", s.health_levels.len(), if l == Language::PtBr { "vit." } else { "HL" }));
                        }
                        if let Some(es) = s.essence {
                            meta.push(format!("Ess. {}", es));
                        }
                        view! {
                            <button type="button" class="npc-picker-item" on:click=move |_| tray.add_from_bestiary(e, lang.get_untracked())>
                                <span class="npc-picker-name">{e.name(l)}</span>
                                <span class="npc-picker-meta">{meta.join(" · ")}</span>
                                <span class="npc-picker-add">"+"</span>
                            </button>
                        }
                    }).collect_view()}
                })
            })
            .collect_view()
    };

    view! {
        <Show when=move || { !tray.open.get() && count.get() > 0 }>
            <button
                type="button"
                class="npc-tray-fab"
                title=move || t("Abrir as mini fichas de NPC", "Open NPC mini sheets")
                on:click=move |_| tray.open.set(true)
            >
                <span class="npc-tray-fab-icon">"💀"</span>
                <span class="npc-tray-fab-label">"NPCs"</span>
                <span class="npc-tray-fab-count">{count}</span>
            </button>
        </Show>

        <Show when=move || tray.open.get()>
            <aside class="npc-tray-panel" role="dialog" aria-label="NPCs">
                <header class="npc-tray-head">
                    <div class="npc-tray-title">
                        <span>"💀 " {move || t("Mini fichas", "Mini sheets")}</span>
                        <span class="npc-tray-count">{count}</span>
                    </div>
                    <button type="button" class="mini-icon-btn" title=move || t("Fechar (as fichas continuam aqui)", "Close (sheets are kept)") on:click=move |_| tray.open.set(false)>"✕"</button>
                </header>
                <p class="npc-tray-hint">{move || t(
                    "Fichas descartáveis para a cena. Ficam só neste navegador.",
                    "Throwaway sheets for the scene. Kept only in this browser.",
                )}</p>

                <div class="npc-tray-toolbar">
                    <button type="button" class="npc-tool-btn" class:active=picker_open on:click=move |_| picker_open.update(|v| *v = !*v)>
                        {move || t("📖 Do bestiário", "📖 From bestiary")}
                    </button>
                    <button type="button" class="npc-tool-btn" on:click=move |_| tray.add_blank()>
                        {move || t("+ NPC em branco", "+ Blank NPC")}
                    </button>
                    <Show when=move || { count.get() > 0 }>
                        <button
                            type="button"
                            class="npc-tool-btn danger"
                            class:confirm=confirm_clear
                            on:click=move |_| {
                                if confirm_clear.get_untracked() {
                                    tray.cards.set(Vec::new());
                                    confirm_clear.set(false);
                                } else {
                                    confirm_clear.set(true);
                                    set_timeout(move || { let _ = confirm_clear.try_set(false); }, std::time::Duration::from_secs(3));
                                }
                            }
                        >
                            {move || if confirm_clear.get() { t("Apagar todas?", "Delete all?") } else { t("🧹 Limpar", "🧹 Clear") }}
                        </button>
                    </Show>
                </div>

                <Show when=move || picker_open.get()>
                    <div class="npc-picker">
                        <input
                            class="npc-picker-search"
                            type="search"
                            placeholder=move || t("Buscar: lobo, policial, elemental...", "Search: wolf, cop, elemental...")
                            prop:value=search
                            on:input=move |ev| search.set(event_target_value(&ev))
                        />
                        <div class="npc-picker-list">{picker}</div>
                    </div>
                </Show>

                <div class="npc-tray-list">
                    <Show when=move || count.get() == 0>
                        <div class="npc-tray-empty">{move || t(
                            "Nenhuma mini ficha ainda. Escolha uma ficha do bestiário ou crie um NPC em branco.",
                            "No mini sheets yet. Pick one from the bestiary or create a blank NPC.",
                        )}</div>
                    </Show>
                    <For
                        each=move || tray.cards.with(|c| c.iter().map(|x| x.uid.clone()).collect::<Vec<_>>())
                        key=|u| u.clone()
                        children=move |uid| view! { <MiniSheet uid=uid /> }
                    />
                </div>
            </aside>
        </Show>
    }
    .into_view()
}
