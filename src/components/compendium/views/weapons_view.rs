use leptos::*;
use crate::compendium::weapons::{
    explain_weapon_note, get_categories_by_main_group, get_weapons_by_category,
    get_weapons_by_class, get_weapons_by_main_group, ALL_RULE_NOTES, ALL_WEAPONS,
    ALL_WEAPON_CATEGORIES, ALL_WEAPON_CLASSES, ALL_WEAPON_MAIN_GROUPS,
    MELEE_CATEGORIES, RANGED_CATEGORIES, RANGED_NOTE_1, RANGED_NOTE_2, RANGED_NOTE_3,
    RANGED_NOTE_4, RANGED_NOTE_5, RANGED_NOTE_6, RANGED_NOTE_7, RANGED_NOTE_8,
    RANGED_NOTE_9, RANGED_NOTE_10, RANGED_NOTE_11, RANGED_NOTE_12, WeaponCategory,
    WeaponClass, WeaponDefinition, WeaponMainGroup,
};
use crate::components::Callback;
use crate::i18n::Language;

#[component]
pub fn WeaponsView(
    selected_weapon_id: RwSignal<String>,
    current_lang: Signal<Language>,
    target_slot: Option<Signal<Option<usize>>>,
    on_select_weapon: Option<Callback<(Option<usize>, &'static WeaponDefinition)>>,
    on_close: Option<Callback<()>>,
) -> impl IntoView {
    // Filtro por Grupo Principal (Corpo a Corpo ou À Distância) - None = Todas
    let selected_group = create_rw_signal(Option::<WeaponMainGroup>::None);

    // Filtro por Categoria Específica dentro do Grupo - None = Todas do Grupo
    let selected_category = create_rw_signal(Option::<WeaponCategory>::None);

    // Texto de busca
    let search_query = create_rw_signal(String::new());

    // Visualizar legenda geral
    let show_general_legend = create_rw_signal(false);

    let active_weapon = Signal::derive(move || {
        let cur_id = selected_weapon_id.get();
        ALL_WEAPONS
            .iter()
            .find(|w| w.id == cur_id)
            .unwrap_or(&ALL_WEAPONS[0])
    });

    let on_select_act = on_select_weapon;
    let on_close_act = on_close;

    view! {
        <div class="compendium-section-split weapon-compendium-split">
            // ================= COLUNA 1: Lista e Filtros de Armas =================
            <div class="practice-sidebar-pane weapon-sidebar-pane">
                // Filtro de Busca Rápida
                <div class="compendium-search-box weapon-search-box">
                    <input
                        type="text"
                        class="compendium-search-input weapon-search-input"
                        placeholder=move || match current_lang.get() {
                            Language::PtBr => "🔍 Buscar 82 armas (ex: Katana, Beretta, Fuzil, Arco)...",
                            Language::EnUs => "🔍 Search 82 weapons (e.g. Katana, Beretta, Rifle, Bow)...",
                        }
                        prop:value=move || search_query.get()
                        on:input=move |ev| search_query.set(event_target_value(&ev))
                    />
                    {move || if !search_query.get().is_empty() {
                        view! {
                            <button
                                type="button"
                                class="weapon-search-clear"
                                on:click=move |_| search_query.set(String::new())
                            >
                                "✕"
                            </button>
                        }.into_view()
                    } else {
                        view! { <span></span> }.into_view()
                    }}
                </div>

                // Botão de alternar legenda de notas / regras gerais
                <div class="special-box-tab-wrap" style="margin-bottom: 0.6rem;">
                    <button
                        type="button"
                        class=move || if show_general_legend.get() { "practice-tab-btn special-box-tab active" } else { "practice-tab-btn special-box-tab" }
                        on:click=move |_| show_general_legend.update(|v| *v = !*v)
                        title=move || match current_lang.get() {
                            Language::PtBr => "Exibir/Ocultar Regras Gerais & Legenda de Notas",
                            Language::EnUs => "Show/Hide General Rules & Notes Legend",
                        }
                    >
                        <span class="tab-indicator">"📜"</span>
                        <div class="tab-text-wrap">
                            <span class="tab-name">
                                {move || match current_lang.get() {
                                    Language::PtBr => "Regras de Combate & Legenda",
                                    Language::EnUs => "Combat Rules & Legend",
                                }}
                            </span>
                            <span class="tab-sub">"M20, pp. 450-453"</span>
                        </div>
                    </button>
                </div>

                // Pílulas Principais: Todas (82) | Arma Branca e Corpo a Corpo (42) | Armas de Fogo e à Distância (40)
                <div class="weapon-class-pills">
                    <button
                        type="button"
                        class=move || if selected_group.get().is_none() {
                            "class-pill-btn active"
                        } else {
                            "class-pill-btn"
                        }
                        on:click=move |_| {
                            selected_group.set(None);
                            selected_category.set(None);
                        }
                    >
                        "🌐 " {move || match current_lang.get() {
                            Language::PtBr => "Todas (82)",
                            Language::EnUs => "All (82)",
                        }}
                    </button>
                    {ALL_WEAPON_MAIN_GROUPS.iter().map(|&grp| {
                        let is_active = Signal::derive(move || selected_group.get() == Some(grp));
                        let count = get_weapons_by_main_group(grp).len();
                        view! {
                            <button
                                type="button"
                                class=move || if is_active.get() { "class-pill-btn active" } else { "class-pill-btn" }
                                on:click=move |_| {
                                    selected_group.set(Some(grp));
                                    selected_category.set(None);
                                }
                            >
                                {grp.icon()} " " {move || grp.name(current_lang.get())}
                                <span class="pill-count">" (" {count} ")"</span>
                            </button>
                        }
                    }).collect_view()}
                </div>

                // Sub-Pílulas Dinâmicas: Abrem ao selecionar uma das pílulas principais
                {move || {
                    if let Some(grp) = selected_group.get() {
                        let total_in_grp = get_weapons_by_main_group(grp).len();
                        let sub_cats = get_categories_by_main_group(grp);
                        view! {
                            <div class="weapon-subfilter-container">
                                <div class="weapon-subfilter-pills">
                                    <button
                                        type="button"
                                        class=move || if selected_category.get().is_none() { "sub-pill-btn active" } else { "sub-pill-btn" }
                                        on:click=move |_| selected_category.set(None)
                                    >
                                        "• " {move || match current_lang.get() {
                                            Language::PtBr => format!("Todas do Grupo ({})", total_in_grp),
                                            Language::EnUs => format!("All in Group ({})", total_in_grp),
                                        }}
                                    </button>
                                    {sub_cats.iter().map(|&cat| {
                                        let is_cat_active = Signal::derive(move || selected_category.get() == Some(cat));
                                        let count = get_weapons_by_category(cat).len();
                                        view! {
                                            <button
                                                type="button"
                                                class=move || if is_cat_active.get() { "sub-pill-btn active" } else { "sub-pill-btn" }
                                                on:click=move |_| selected_category.set(Some(cat))
                                            >
                                                {cat.icon()} " " {move || cat.name(current_lang.get())}
                                                <span class="sub-pill-count">" (" {count} ")"</span>
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="weapon-subfilter-hint">
                                <span class="hint-icon">"💡"</span>
                                <span class="hint-text">
                                    {move || match current_lang.get() {
                                        Language::PtBr => "Clique em 'Arma Branca' ou 'Armas de Fogo' para abrir as categorias específicas (Facas, Machados, Pistolas, Arcos, etc.)",
                                        Language::EnUs => "Click on 'Melee' or 'Firearms' to view subcategories (Knives, Axes, Pistols, Bows, etc.)",
                                    }}
                                </span>
                            </div>
                        }.into_view()
                    }
                }}

                // Lista de Armas Filtradas
                <div class="practice-tab-list weapon-tab-list">
                    {move || {
                        let q = search_query.get().to_lowercase();
                        let cur_grp = selected_group.get();
                        let cur_cat = selected_category.get();

                        let filtered: Vec<&'static WeaponDefinition> = ALL_WEAPONS.iter().filter(|w| {
                            if let Some(c) = cur_cat {
                                if w.category != c { return false; }
                            } else if let Some(grp) = cur_grp {
                                if w.category.main_group() != grp { return false; }
                            }
                            if !q.is_empty() {
                                let name_match = w.name.to_lowercase().contains(&q)
                                    || w.name_pt.to_lowercase().contains(&q)
                                    || w.aliases.iter().any(|a| a.to_lowercase().contains(&q))
                                    || w.category.name(current_lang.get()).to_lowercase().contains(&q);
                                if !name_match { return false; }
                            }
                            true
                        }).collect();

                        if filtered.is_empty() {
                            view! {
                                <div class="weapon-no-results">
                                    <span>"🔍"</span>
                                    <p>{move || match current_lang.get() {
                                        Language::PtBr => "Nenhuma arma encontrada.",
                                        Language::EnUs => "No weapons found.",
                                    }}</p>
                                </div>
                            }.into_view()
                        } else {
                            filtered.into_iter().map(|w| {
                                let w_id = w.id;
                                let is_selected = Signal::derive(move || selected_weapon_id.get() == w_id);
                                let has_notes = !w.notes.is_empty();
                                view! {
                                    <button
                                        type="button"
                                        class=move || if is_selected.get() { "practice-tab-btn active weapon-card-tab" } else { "practice-tab-btn weapon-card-tab" }
                                        on:click=move |_| {
                                            selected_weapon_id.set(w_id.to_string());
                                            show_general_legend.set(false);
                                        }
                                    >
                                        <span class="practice-tab-bullet">{w.category.icon()}</span>
                                        <div class="tab-text-wrap">
                                            <span class="tab-name">{move || w.name(current_lang.get())}</span>
                                            <span class="tab-sub">
                                                "Dif " {w.difficulty} " • " {move || w.damage(current_lang.get())}
                                                {if w.range != "—" {
                                                    format!(" • Alc {}", w.range)
                                                } else {
                                                    String::new()
                                                }}
                                            </span>
                                        </div>
                                        {if has_notes {
                                            view! {
                                                <span class="weapon-card-notes-indicator" title=w.notes.join(", ")>
                                                    {w.notes[0]}
                                                </span>
                                            }.into_view()
                                        } else {
                                            view! { <span></span> }.into_view()
                                        }}
                                    </button>
                                }
                            }).collect_view().into_view()
                        }
                    }}
                </div>
            </div>

            // ================= COLUNA 2: Detalhes da Arma ou Regras Gerais =================
            <div class="practice-detail-pane weapon-detail-pane">
                {move || if show_general_legend.get() {
                    // Visualização da Legenda de Regras e Notas Canônicas M20
                    view! {
                        <div class="weapon-legend-view">
                            <div class="legend-header">
                                <h4 class="legend-title">
                                    "📜 " {move || match current_lang.get() {
                                        Language::PtBr => "Regras de Combate & Notas Canônicas",
                                        Language::EnUs => "Combat Rules & Canonical Notes",
                                    }}
                                </h4>
                                <span class="legend-ref">"M20, pp. 450-453"</span>
                            </div>

                            // Parâmetros Básicos
                            <div class="legend-params-grid">
                                <div class="legend-param-card">
                                    <strong>{move || match current_lang.get() {
                                        Language::PtBr => "Dificuldade (Difficulty)",
                                        Language::EnUs => "Difficulty (Dif)",
                                    }}</strong>
                                    <p>{move || match current_lang.get() {
                                        Language::PtBr => "Armas brancas: dificuldade do teste de Destreza + Luta/Armas Brancas. Armas de fogo: Dif 6 no alcance listado, Dif 8 no dobro do alcance, Dif 4 a queima-roupa (2 jardas).",
                                        Language::EnUs => "Melee: Dexterity + Brawl/Melee difficulty. Firearms: Diff 6 at listed range, Diff 8 at twice range, Diff 4 at point-blank (within 2 yards).",
                                    }}</p>
                                </div>
                                <div class="legend-param-card">
                                    <strong>{move || match current_lang.get() {
                                        Language::PtBr => "Dano & Tipos (Damage)",
                                        Language::EnUs => "Damage & Types",
                                    }}</strong>
                                    <p>{move || match current_lang.get() {
                                        Language::PtBr => "B = Contundente (Bashing) • L = Letal (Lethal) • A = Agravado (Aggravated). Todo dano de armas de fogo e arcos é letal.",
                                        Language::EnUs => "B = Bashing • L = Lethal • A = Aggravated. All damage from firearms and bows is lethal.",
                                    }}</p>
                                </div>
                                <div class="legend-param-card">
                                    <strong>{move || match current_lang.get() {
                                        Language::PtBr => "Cadência & Pente (Rate / Clip)",
                                        Language::EnUs => "Rate & Clip",
                                    }}</strong>
                                    <p>{move || match current_lang.get() {
                                        Language::PtBr => "Cadência: tiros ou rajadas por turno. Pente: capacidade de munição. '+1' indica bala extra na câmara pronta para disparo.",
                                        Language::EnUs => "Rate: bullets/bursts fired per turn. Clip: ammo capacity. '+1' indicates a round loaded in chamber.",
                                    }}</p>
                                </div>
                                <div class="legend-param-card">
                                    <strong>{move || match current_lang.get() {
                                        Language::PtBr => "Ocultabilidade (Conceal)",
                                        Language::EnUs => "Concealment (Conceal)",
                                    }}</strong>
                                    <p>{move || match current_lang.get() {
                                        Language::PtBr => "P = Bolso (Pocket) • J = Jaqueta (Jacket) • T = Sobretudo (Trenchcoat) • N = N/A (Não Ocultável).",
                                        Language::EnUs => "P = Pocket • J = Jacket • T = Trenchcoat • N = N/A (Cannot be concealed).",
                                    }}</p>
                                </div>
                            </div>

                            // Notas de Armas Brancas (#1 a #10)
                            <div class="legend-notes-title">
                                {move || match current_lang.get() {
                                    Language::PtBr => "NOTAS DE ARMAS BRANCAS & COMBATE CORPO A CORPO (#1 a #10)",
                                    Language::EnUs => "MELEE WEAPONS SPECIAL NOTES (#1 to #10)",
                                }}
                            </div>
                            <div class="legend-notes-list">
                                {ALL_RULE_NOTES.iter().map(|n| {
                                    view! {
                                        <div class="legend-note-row">
                                            <span class="note-code-badge melee-badge">{n.code}</span>
                                            <div class="note-content-wrap">
                                                <strong class="note-title">{move || n.title(current_lang.get())}</strong>
                                                <p class="note-desc">{move || n.description(current_lang.get())}</p>
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>

                            // Notas de Armas de Fogo e À Distância (#1 a #12)
                            <div class="legend-notes-title" style="margin-top: 1.5rem;">
                                {move || match current_lang.get() {
                                    Language::PtBr => "NOTAS DE ARMAS DE FOGO & À DISTÂNCIA (#1 a #12 - M20 pp. 452-453)",
                                    Language::EnUs => "RANGED WEAPONS SPECIAL NOTES (#1 to #12 - M20 pp. 452-453)",
                                }}
                            </div>
                            <div class="legend-notes-list">
                                {[
                                    &RANGED_NOTE_1, &RANGED_NOTE_2, &RANGED_NOTE_3, &RANGED_NOTE_4,
                                    &RANGED_NOTE_5, &RANGED_NOTE_6, &RANGED_NOTE_7, &RANGED_NOTE_8,
                                    &RANGED_NOTE_9, &RANGED_NOTE_10, &RANGED_NOTE_11, &RANGED_NOTE_12,
                                ].iter().map(|n| {
                                    view! {
                                        <div class="legend-note-row">
                                            <span class="note-code-badge ranged-badge">{n.code}</span>
                                            <div class="note-content-wrap">
                                                <strong class="note-title">{move || n.title(current_lang.get())}</strong>
                                                <p class="note-desc">{move || n.description(current_lang.get())}</p>
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    }.into_view()
                } else {
                    // Detalhe da Arma Ativa
                    let weapon = active_weapon.get();
                    let w_cls = weapon.category.class();
                    let slot_idx = target_slot.and_then(|s| s.get());
                    let on_sel = on_select_act.clone();
                    let on_cls = on_close_act.clone();

                    view! {
                        <div class="weapon-reading-view">
                            // Cabeçalho da Arma
                            <div class="weapon-detail-header">
                                <div class="weapon-title-box">
                                    <div class="weapon-main-name">
                                        <span class="weapon-detail-icon">{weapon.category.icon()}</span>
                                        {move || weapon.name(current_lang.get())}
                                    </div>
                                    <span class="weapon-alt-name">
                                        {move || match current_lang.get() {
                                            Language::PtBr => format!("Original em Inglês: {}", weapon.name),
                                            Language::EnUs => format!("Nome em Português: {}", weapon.name_pt),
                                        }}
                                    </span>
                                </div>
                                <div class="weapon-header-badges">
                                    <span class="badge-class">{move || w_cls.name(current_lang.get())}</span>
                                    <span class="badge-category">{move || weapon.category.name(current_lang.get())}</span>
                                    <span class="badge-page">{weapon.page_ref}</span>
                                </div>
                            </div>

                            // Banner de Estatísticas da Arma (adaptativo para corpo a corpo ou à distância)
                            {
                                let is_melee = w_cls == WeaponClass::Melee || (weapon.range == "—" && weapon.rate == "—" && weapon.clip == "—");
                                if is_melee {
                                    view! {
                                        <div class="weapon-stats-banner is-melee">
                                            <div class="stat-box">
                                                <span class="stat-label">
                                                    {move || crate::i18n::tr("diff_header", current_lang.get()).to_uppercase()}
                                                </span>
                                                <span class="stat-value stat-diff">{weapon.difficulty}</span>
                                                <span class="stat-sub">
                                                    {move || match current_lang.get() {
                                                        Language::PtBr => "Dificuldade",
                                                        Language::EnUs => "Difficulty",
                                                    }}
                                                </span>
                                            </div>

                                            <div class="stat-box stat-box-damage">
                                                <span class="stat-label">
                                                    {move || crate::i18n::tr("dmg_header", current_lang.get()).to_uppercase()}
                                                </span>
                                                <span class="stat-value stat-dmg">
                                                    {move || weapon.damage(current_lang.get())}
                                                </span>
                                                <span class="stat-sub">
                                                    {move || match current_lang.get() {
                                                        Language::PtBr => "Dano / Efeito",
                                                        Language::EnUs => "Damage / Effect",
                                                    }}
                                                </span>
                                            </div>

                                            <div class="stat-box">
                                                <span class="stat-label">
                                                    {move || crate::i18n::tr("conceal_header", current_lang.get()).to_uppercase()}
                                                </span>
                                                <span class="stat-value stat-conceal">{weapon.conceal}</span>
                                                <span class="stat-sub">
                                                    {move || match (weapon.conceal, current_lang.get()) {
                                                        ("P", Language::PtBr) => "P (Bolso)",
                                                        ("P", Language::EnUs) => "P (Pocket)",
                                                        ("J", Language::PtBr) => "J (Jaqueta)",
                                                        ("J", Language::EnUs) => "J (Jacket)",
                                                        ("T", Language::PtBr) => "T (Sobretudo)",
                                                        ("T", Language::EnUs) => "T (Trenchcoat)",
                                                        _ => "N (Não Ocultável)",
                                                    }}
                                                </span>
                                            </div>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! {
                                        <div class="weapon-stats-banner">
                                            <div class="stat-box">
                                                <span class="stat-label">
                                                    {move || crate::i18n::tr("diff_header", current_lang.get()).to_uppercase()}
                                                </span>
                                                <span class="stat-value stat-diff">{weapon.difficulty}</span>
                                                <span class="stat-sub">
                                                    {move || match current_lang.get() {
                                                        Language::PtBr => "Dificuldade",
                                                        Language::EnUs => "Difficulty",
                                                    }}
                                                </span>
                                            </div>

                                            <div class="stat-box">
                                                <span class="stat-label">
                                                    {move || crate::i18n::tr("dmg_header", current_lang.get()).to_uppercase()}
                                                </span>
                                                <span class="stat-value stat-dmg">
                                                    {move || weapon.damage(current_lang.get())}
                                                </span>
                                                <span class="stat-sub">
                                                    {move || match current_lang.get() {
                                                        Language::PtBr => "Dano",
                                                        Language::EnUs => "Damage",
                                                    }}
                                                </span>
                                            </div>

                                            <div class="stat-box">
                                                <span class="stat-label">
                                                    {move || crate::i18n::tr("range_header", current_lang.get()).to_uppercase()}
                                                </span>
                                                <span class="stat-value stat-range">{weapon.range}</span>
                                                <span class="stat-sub">
                                                    {move || match current_lang.get() {
                                                        Language::PtBr => "Alcance (jardas)",
                                                        Language::EnUs => "Range (yards)",
                                                    }}
                                                </span>
                                            </div>

                                            <div class="stat-box">
                                                <span class="stat-label">
                                                    {move || crate::i18n::tr("rate_header", current_lang.get()).to_uppercase()}
                                                </span>
                                                <span class="stat-value stat-rate">{weapon.rate}</span>
                                                <span class="stat-sub">
                                                    {move || match current_lang.get() {
                                                        Language::PtBr => "Cadência / Turno",
                                                        Language::EnUs => "Rate of Fire",
                                                    }}
                                                </span>
                                            </div>

                                            <div class="stat-box">
                                                <span class="stat-label">
                                                    {move || crate::i18n::tr("clip_header", current_lang.get()).to_uppercase()}
                                                </span>
                                                <span class="stat-value stat-clip">{weapon.clip}</span>
                                                <span class="stat-sub">
                                                    {move || match current_lang.get() {
                                                        Language::PtBr => "Pente / Munição",
                                                        Language::EnUs => "Clip Capacity",
                                                    }}
                                                </span>
                                            </div>

                                            <div class="stat-box">
                                                <span class="stat-label">
                                                    {move || crate::i18n::tr("conceal_header", current_lang.get()).to_uppercase()}
                                                </span>
                                                <span class="stat-value stat-conceal">{weapon.conceal}</span>
                                                <span class="stat-sub">
                                                    {move || match (weapon.conceal, current_lang.get()) {
                                                        ("P", Language::PtBr) => "Bolso",
                                                        ("P", Language::EnUs) => "Pocket",
                                                        ("J", Language::PtBr) => "Jaqueta",
                                                        ("J", Language::EnUs) => "Jacket",
                                                        ("T", Language::PtBr) => "Sobretudo",
                                                        ("T", Language::EnUs) => "Trenchcoat",
                                                        _ => "Não Ocultável",
                                                    }}
                                                </span>
                                            </div>
                                        </div>
                                    }.into_view()
                                }
                            }

                            // Seção de Notas de Regras Especiais da Arma
                            {if !weapon.notes.is_empty() {
                                view! {
                                    <div class="weapon-rules-section">
                                        <h4 class="weapon-section-title">
                                            {move || crate::i18n::tr("weapon_notes_header", current_lang.get())}
                                        </h4>
                                        <div class="weapon-notes-cards">
                                            {weapon.notes.iter().map(|&note_code| {
                                                if note_code == "Used as pair" {
                                                    view! {
                                                        <div class="weapon-rule-card">
                                                            <div class="rule-card-header">
                                                                <span class="rule-badge">"⚔️ Pair"</span>
                                                                <strong class="rule-card-title">
                                                                    {move || match current_lang.get() {
                                                                        Language::PtBr => "Empunhadura em Pares",
                                                                        Language::EnUs => "Used as Pair",
                                                                    }}
                                                                </strong>
                                                            </div>
                                                            <p class="rule-card-desc">
                                                                {move || match current_lang.get() {
                                                                    Language::PtBr => "Geralmente empunhadas simultaneamente em ambas as mãos como um par coordenado de lâminas.",
                                                                    Language::EnUs => "Commonly employed simultaneously as a coordinated pair of weapons.",
                                                                }}
                                                            </p>
                                                        </div>
                                                    }.into_view()
                                                } else if let Some((title, desc)) = explain_weapon_note(note_code, current_lang.get(), w_cls) {
                                                    view! {
                                                        <div class="weapon-rule-card">
                                                            <div class="rule-card-header">
                                                                <span class="rule-badge">{note_code}</span>
                                                                <strong class="rule-card-title">{title}</strong>
                                                            </div>
                                                            <p class="rule-card-desc">{desc}</p>
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    view! {
                                                        <div class="weapon-rule-card">
                                                            <span class="rule-badge">{note_code}</span>
                                                        </div>
                                                    }.into_view()
                                                }
                                            }).collect_view()}
                                        </div>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <div class="weapon-no-special-rules">
                                        <span>"✓"</span>
                                        <p>{move || match current_lang.get() {
                                            Language::PtBr => "Arma padrão sem penalidades ou requisitos especiais de disparo ou empunhadura.",
                                            Language::EnUs => "Standard weapon with no special penalties or wielding requirements.",
                                        }}</p>
                                    </div>
                                }.into_view()
                            }}

                            // Descrição Canônica e Emprego Tático
                            <div class="weapon-desc-section">
                                <h4 class="weapon-section-title">
                                    {move || match current_lang.get() {
                                        Language::PtBr => "DESCRIÇÃO & EMPREGO TÁTICO",
                                        Language::EnUs => "DESCRIPTION & TACTICAL USE",
                                    }}
                                </h4>
                                <p class="weapon-desc-text">
                                    {move || weapon.description(current_lang.get())}
                                </p>
                            </div>

                            // Aliases / Outros Nomes Conhecidos
                            {if weapon.aliases.len() > 1 {
                                view! {
                                    <div class="weapon-aliases-row">
                                        <span class="aliases-label">
                                            {move || match current_lang.get() {
                                                Language::PtBr => "Outros nomes comuns: ",
                                                Language::EnUs => "Common aliases: ",
                                            }}
                                        </span>
                                        <span class="aliases-text">{weapon.aliases.join(" • ")}</span>
                                    </div>
                                }.into_view()
                            } else {
                                view! { <span></span> }.into_view()
                            }}

                            // Botão de Ação: Equipar no Slot de Combate da Ficha
                            {if let Some(cb) = on_sel {
                                let w_to_equip = weapon;
                                let on_close_action = on_cls;
                                view! {
                                    <div class="weapon-equip-action-row" style="margin-top: 1.5rem;">
                                        <button
                                            type="button"
                                            class="practice-select-btn weapon-equip-btn"
                                            on:click=move |_| {
                                                cb.call((slot_idx, w_to_equip));
                                                if let Some(c) = &on_close_action {
                                                    c.call(());
                                                }
                                            }
                                        >
                                            {move || match (slot_idx, current_lang.get()) {
                                                (Some(idx), Language::PtBr) => format!("✦ Equipar '{}' na Linha #{}", w_to_equip.name(current_lang.get()), idx + 1),
                                                (Some(idx), Language::EnUs) => format!("✦ Equip '{}' in Row #{}", w_to_equip.name(current_lang.get()), idx + 1),
                                                (None, Language::PtBr) => format!("✦ Equipar '{}' na Tabela de Combate", w_to_equip.name(current_lang.get())),
                                                (None, Language::EnUs) => format!("✦ Equip '{}' to Combat Table", w_to_equip.name(current_lang.get())),
                                            }}
                                        </button>
                                    </div>
                                }.into_view()
                            } else {
                                view! { <span></span> }.into_view()
                            }}
                        </div>
                    }.into_view()
                }}
            </div>
        </div>
    }
}
