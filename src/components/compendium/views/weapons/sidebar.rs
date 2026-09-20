use leptos::*;
use crate::compendium::weapons::{
    get_categories_by_main_group, get_maneuvers_by_category, get_weapons_by_category,
    get_weapons_by_main_group, CombatManeuver, ManeuverCategory, WeaponCategory,
    WeaponDefinition, WeaponMainGroup, ALL_COMBAT_MANEUVERS, ALL_MANEUVER_CATEGORIES,
    ALL_WEAPONS, ALL_WEAPON_CATEGORIES, ALL_WEAPON_MAIN_GROUPS, MELEE_CATEGORIES,
    RANGED_CATEGORIES,
};
use crate::i18n::Language;
use super::CombatSubTab;

/// Renderiza o painel lateral de navegação e filtragem (Armas, Manobras, Categorias e Banners)
pub fn render_weapons_sidebar(
    combat_subtab: RwSignal<CombatSubTab>,
    selected_weapon_id: RwSignal<String>,
    selected_maneuver_id: RwSignal<String>,
    selected_group: RwSignal<Option<WeaponMainGroup>>,
    selected_category: RwSignal<Option<WeaponCategory>>,
    selected_maneuver_cat: RwSignal<Option<ManeuverCategory>>,
    search_query: RwSignal<String>,
    show_general_legend: RwSignal<bool>,
    show_thunder_punch: RwSignal<bool>,
    show_eight_limbs: RwSignal<bool>,
    show_do_rules: RwSignal<bool>,
    current_lang: Signal<Language>,
    mobile_show_detail: Option<RwSignal<bool>>,
) -> impl IntoView {
    view! {
        <div class="practice-sidebar-pane weapon-sidebar-pane">
            // Sub-Navegação de Combate: Armas (82) | Manobras & Artes Marciais | Regras & Legenda
            <div class="weapon-mode-pills">
                <button
                    type="button"
                    class=move || if combat_subtab.get() == CombatSubTab::Weapons && !show_general_legend.get() && !show_thunder_punch.get() && !show_eight_limbs.get() && !show_do_rules.get() { "weapon-mode-pill-btn active" } else { "weapon-mode-pill-btn" }
                    on:click=move |_| {
                        combat_subtab.set(CombatSubTab::Weapons);
                        show_general_legend.set(false);
                        show_thunder_punch.set(false);
                        show_eight_limbs.set(false);
                        show_do_rules.set(false);
                    }
                >
                    "⚔️ " <span class="pill-label">{move || match current_lang.get() {
                        Language::PtBr => "Armas (82)",
                        Language::EnUs => "Weapons (82)",
                    }}</span>
                </button>
                <button
                    type="button"
                    class=move || if combat_subtab.get() == CombatSubTab::Maneuvers && !show_general_legend.get() && !show_thunder_punch.get() && !show_eight_limbs.get() && !show_do_rules.get() { "weapon-mode-pill-btn active" } else { "weapon-mode-pill-btn" }
                    on:click=move |_| {
                        combat_subtab.set(CombatSubTab::Maneuvers);
                        show_general_legend.set(false);
                        show_thunder_punch.set(false);
                        show_eight_limbs.set(false);
                        show_do_rules.set(false);
                        if let Some(msd) = mobile_show_detail {
                            msd.set(false);
                        }
                    }
                >
                    "🥋 " <span class="pill-label">{move || match current_lang.get() {
                        Language::PtBr => format!("Manobras ({})", ALL_COMBAT_MANEUVERS.len()),
                        Language::EnUs => format!("Maneuvers ({})", ALL_COMBAT_MANEUVERS.len()),
                    }}</span>
                </button>
                <button
                    type="button"
                    class=move || if show_general_legend.get() { "weapon-mode-pill-btn legend-btn active" } else { "weapon-mode-pill-btn legend-btn" }
                    on:click=move |_| {
                        show_general_legend.update(|v| *v = !*v);
                        show_thunder_punch.set(false);
                        show_eight_limbs.set(false);
                        show_do_rules.set(false);
                        if let Some(msd) = mobile_show_detail {
                            msd.set(true);
                        }
                    }
                    title=move || match current_lang.get() {
                        Language::PtBr => "Regras de Combate & Legenda (M20, pp. 450-453)",
                        Language::EnUs => "Combat Rules & Legend (M20, pp. 450-453)",
                    }
                >
                    "📜"
                </button>
            </div>

            // Filtro de Busca Rápida Dinâmico
            <div class="compendium-search-box weapon-search-box">
                <input
                    type="text"
                    class="compendium-search-input weapon-search-input"
                    placeholder=move || match (combat_subtab.get(), current_lang.get()) {
                        (CombatSubTab::Weapons, Language::PtBr) => "🔍 Buscar 82 armas (ex: Katana, Beretta, Fuzil)...",
                        (CombatSubTab::Weapons, Language::EnUs) => "🔍 Search 82 weapons (e.g. Katana, Beretta, Rifle)...",
                        (CombatSubTab::Maneuvers, Language::PtBr) => "🔍 Buscar 35 manobras & artes marciais (ex: Chute Voador, Golpe Mortal, Soco)...",
                        (CombatSubTab::Maneuvers, Language::EnUs) => "🔍 Search 35 maneuvers & martial arts (e.g. Flying Kick, Death Strike, Punch)...",
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

            {move || match combat_subtab.get() {
                CombatSubTab::Weapons => view! {
                    <div class="weapons-list-container">
                        // Pílulas Principais: Todas (82) | Arma Branca (42) | À Distância (40)
                        <div class="weapon-class-pills weapon-groups-pills">
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
                                "🌐 " <span class="pill-label">{move || match current_lang.get() {
                                    Language::PtBr => "Todas",
                                    Language::EnUs => "All",
                                }}</span>
                                <span class="pill-count">" (82)"</span>
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
                                        {grp.icon()} " " <span class="pill-label">{move || grp.short_name(current_lang.get())}</span>
                                        <span class="pill-count">" (" {count} ")"</span>
                                    </button>
                                }
                            }).collect_view()}
                        </div>

                        // Menu Dropdown de Seleção Rápida de Categorias
                        <div class="weapon-category-select-wrap">
                            <select
                                class="weapon-category-select"
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    if val.is_empty() {
                                        selected_category.set(None);
                                    } else {
                                        let cat = WeaponCategory::from_id_str(&val);
                                        selected_category.set(cat);
                                        if let Some(c) = cat {
                                            selected_group.set(Some(c.main_group()));
                                        }
                                    }
                                }
                                prop:value=move || selected_category.get().map(|c| c.id_str()).unwrap_or_default()
                            >
                                <option value="">
                                    {move || match current_lang.get() {
                                        Language::PtBr => match selected_group.get() {
                                            None => "📂 Todas as Categorias (15)".to_string(),
                                            Some(WeaponMainGroup::Melee) => "📂 Todas de Arma Branca (7 categorias)".to_string(),
                                            Some(WeaponMainGroup::Ranged) => "📂 Todas de Fogo & Distância (8 categorias)".to_string(),
                                        },
                                        Language::EnUs => match selected_group.get() {
                                            None => "📂 All Categories (15)".to_string(),
                                            Some(WeaponMainGroup::Melee) => "📂 All Melee (7 categories)".to_string(),
                                            Some(WeaponMainGroup::Ranged) => "📂 All Firearms & Ranged (8 categories)".to_string(),
                                        },
                                    }}
                                </option>
                                {move || {
                                    let cats = match selected_group.get() {
                                        None => ALL_WEAPON_CATEGORIES,
                                        Some(WeaponMainGroup::Melee) => MELEE_CATEGORIES,
                                        Some(WeaponMainGroup::Ranged) => RANGED_CATEGORIES,
                                    };
                                    cats.iter().map(|&cat| {
                                        let count = get_weapons_by_category(cat).len();
                                        view! {
                                            <option value=cat.id_str()>
                                                {cat.icon()} " " {cat.name(current_lang.get())} " (" {count} ")"
                                            </option>
                                        }
                                    }).collect_view()
                                }}
                            </select>
                        </div>

                        // Sub-Pílulas Dinâmicas
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
                                view! { <span></span> }.into_view()
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
                                        let is_ranged = w.category.main_group() == WeaponMainGroup::Ranged;
                                        view! {
                                            <button
                                                type="button"
                                                class=move || if is_selected.get() { "practice-tab-btn active weapon-card-tab" } else { "practice-tab-btn weapon-card-tab" }
                                                on:click=move |_| {
                                                    selected_weapon_id.set(w_id.to_string());
                                                    show_general_legend.set(false);
                                                    if let Some(msd) = mobile_show_detail {
                                                        msd.set(true);
                                                    }
                                                }
                                            >
                                                <span class="practice-tab-bullet">{w.category.icon()}</span>
                                                <div class="tab-text-wrap">
                                                    <div style="display: flex; align-items: center; justify-content: space-between; width: 100%; gap: 0.3rem;">
                                                        <span class="tab-name">{move || w.name(current_lang.get())}</span>
                                                        <span 
                                                            class="weapon-group-pill"
                                                            class:is-ranged=is_ranged
                                                        >
                                                            {if is_ranged { "🔫 Dist." } else { "⚔️ Branca" }}
                                                        </span>
                                                    </div>
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
                }.into_view(),

                CombatSubTab::Maneuvers => view! {
                    <div class="maneuvers-list-container">
                        // Pílulas de Categoria de Manobras: Todas (44) | Gerais (10) | Luta Suja (8) | Artes Marciais (16) | Dô (9) | Especiais (1)
                        <div class="weapon-class-pills maneuver-categories-pills">
                            <button
                                type="button"
                                class=move || if selected_maneuver_cat.get().is_none() { "class-pill-btn active" } else { "class-pill-btn" }
                                on:click=move |_| selected_maneuver_cat.set(None)
                            >
                                "🌐 " <span class="pill-label">{move || match current_lang.get() {
                                    Language::PtBr => "Todas",
                                    Language::EnUs => "All",
                                }}</span>
                                <span class="pill-count">" (" {ALL_COMBAT_MANEUVERS.len()} ")"</span>
                            </button>
                            {ALL_MANEUVER_CATEGORIES.iter().map(|&cat| {
                                let is_active = Signal::derive(move || selected_maneuver_cat.get() == Some(cat));
                                let count = get_maneuvers_by_category(Some(cat)).len();
                                view! {
                                    <button
                                        type="button"
                                        class=move || if is_active.get() { "class-pill-btn active" } else { "class-pill-btn" }
                                        on:click=move |_| selected_maneuver_cat.set(Some(cat))
                                    >
                                        {cat.icon()} " " <span class="pill-label">{move || cat.name(current_lang.get())}</span>
                                        <span class="pill-count">" (" {count} ")"</span>
                                    </button>
                                }
                            }).collect_view()}
                        </div>

                        // Box de Destaque M20: Truque de Mago: O Golpe Trovão (M20 p. 449)
                        <div class="thunder-punch-banner-wrap" style="margin-bottom: 0.45rem;">
                            <button
                                type="button"
                                class=move || if show_thunder_punch.get() { "thunder-punch-banner-btn active" } else { "thunder-punch-banner-btn" }
                                on:click=move |_| {
                                    show_thunder_punch.set(true);
                                    show_general_legend.set(false);
                                    show_eight_limbs.set(false);
                                    show_do_rules.set(false);
                                    if let Some(msd) = mobile_show_detail {
                                        msd.set(true);
                                    }
                                }
                            >
                                <div class="thunder-punch-banner-left">
                                    <span class="thunder-punch-icon">"⚡"</span>
                                    <div class="thunder-punch-text">
                                        <strong class="thunder-punch-title">
                                            {move || match current_lang.get() {
                                                Language::PtBr => "Truque de Mago: Golpe Trovão",
                                                Language::EnUs => "Mage Trick: The Thunder Punch",
                                            }}
                                        </strong>
                                        <span class="thunder-punch-ref">
                                            {move || match current_lang.get() {
                                                Language::PtBr => "M20 p. 449 • Box de Regra Especial",
                                                Language::EnUs => "M20 p. 449 • Special Rule Box",
                                            }}
                                        </span>
                                    </div>
                                </div>
                                <span class="thunder-punch-badge">
                                    {move || match current_lang.get() {
                                        Language::PtBr => "BOX M20",
                                        Language::EnUs => "M20 BOX",
                                    }}
                                </span>
                            </button>
                        </div>

                        // Banner Card Dedicado: Oito Membros da Maestria (Dô / Akashayana)
                        <div class="eight-limbs-banner-wrap" style="margin-bottom: 0.45rem;">
                            <button
                                type="button"
                                class=move || if show_eight_limbs.get() { "eight-limbs-banner-btn active" } else { "eight-limbs-banner-btn" }
                                on:click=move |_| {
                                    show_eight_limbs.set(true);
                                    show_do_rules.set(false);
                                    show_thunder_punch.set(false);
                                    show_general_legend.set(false);
                                    if let Some(msd) = mobile_show_detail {
                                        msd.set(true);
                                    }
                                }
                            >
                                <div class="eight-limbs-banner-left">
                                    <span class="eight-limbs-icon">"🪷"</span>
                                    <div class="eight-limbs-text">
                                        <strong class="eight-limbs-title">
                                            {move || match current_lang.get() {
                                                Language::PtBr => "Oito Membros da Maestria",
                                                Language::EnUs => "Eight Limbs of Expertise",
                                            }}
                                        </strong>
                                        <span class="eight-limbs-ref">
                                            {move || match current_lang.get() {
                                                Language::PtBr => "Akashic Dharma Sutra • M20",
                                                Language::EnUs => "Akashic Dharma Sutra • M20",
                                            }}
                                        </span>
                                    </div>
                                </div>
                                <span class="eight-limbs-badge">
                                    {move || match current_lang.get() {
                                        Language::PtBr => "CARD DÔ",
                                        Language::EnUs => "DO CARD",
                                    }}
                                </span>
                            </button>
                        </div>

                        // Banner: Regras Canônicas & Treino de Dô
                        <div class="do-rules-banner-wrap" style="margin-bottom: 0.75rem;">
                            <button
                                type="button"
                                class=move || if show_do_rules.get() { "do-rules-banner-btn active" } else { "do-rules-banner-btn" }
                                on:click=move |_| {
                                    show_do_rules.set(true);
                                    show_eight_limbs.set(false);
                                    show_thunder_punch.set(false);
                                    show_general_legend.set(false);
                                    if let Some(msd) = mobile_show_detail {
                                        msd.set(true);
                                    }
                                }
                            >
                                <div class="do-rules-banner-left">
                                    <span class="do-rules-icon">"🥋"</span>
                                    <div class="do-rules-text">
                                        <strong class="do-rules-title">
                                            {move || match current_lang.get() {
                                                Language::PtBr => "Regras & Treino de Dô",
                                                Language::EnUs => "Do Rules & Training",
                                            }}
                                        </strong>
                                        <span class="do-rules-ref">
                                            {move || match current_lang.get() {
                                                Language::PtBr => "M20 pp. 580-581 • Vantagens",
                                                Language::EnUs => "M20 pp. 580-581 • Advantages",
                                            }}
                                        </span>
                                    </div>
                                </div>
                                <span class="do-rules-badge">
                                    {move || match current_lang.get() {
                                        Language::PtBr => "M20",
                                        Language::EnUs => "M20",
                                    }}
                                </span>
                            </button>
                        </div>

                        // Lista de Manobras Filtradas
                        <div class="practice-tab-list weapon-tab-list">
                            {move || {
                                let q = search_query.get().to_lowercase();
                                let cur_cat = selected_maneuver_cat.get();

                                let filtered: Vec<&'static CombatManeuver> = ALL_COMBAT_MANEUVERS.iter().filter(|m| {
                                    if let Some(cat) = cur_cat {
                                        if m.category != cat { return false; }
                                    }
                                    if !q.is_empty() {
                                        let matches = m.name.to_lowercase().contains(&q)
                                            || m.name_pt.to_lowercase().contains(&q)
                                            || m.roll.to_lowercase().contains(&q)
                                            || m.roll_pt.to_lowercase().contains(&q)
                                            || m.requirement.to_lowercase().contains(&q)
                                            || m.requirement_pt.to_lowercase().contains(&q)
                                            || m.description.to_lowercase().contains(&q)
                                            || m.description_pt.to_lowercase().contains(&q);
                                        if !matches { return false; }
                                    }
                                    true
                                }).collect();

                                if filtered.is_empty() {
                                    view! {
                                        <div class="weapon-no-results">
                                            <span>"🔍"</span>
                                            <p>{move || match current_lang.get() {
                                                Language::PtBr => "Nenhuma manobra encontrada.",
                                                Language::EnUs => "No maneuvers found.",
                                            }}</p>
                                        </div>
                                    }.into_view()
                                } else {
                                    filtered.into_iter().map(|m| {
                                        let m_id = m.id;
                                        let is_selected = Signal::derive(move || selected_maneuver_id.get() == m_id && !show_thunder_punch.get() && !show_eight_limbs.get() && !show_do_rules.get());
                                        view! {
                                            <button
                                                type="button"
                                                class=move || if is_selected.get() { "practice-tab-btn active weapon-card-tab" } else { "practice-tab-btn weapon-card-tab" }
                                                on:click=move |_| {
                                                    selected_maneuver_id.set(m_id.to_string());
                                                    show_general_legend.set(false);
                                                    show_thunder_punch.set(false);
                                                    show_eight_limbs.set(false);
                                                    show_do_rules.set(false);
                                                    if let Some(msd) = mobile_show_detail {
                                                        msd.set(true);
                                                    }
                                                }
                                            >
                                                <span class="practice-tab-bullet">{m.category.icon()}</span>
                                                <div class="tab-text-wrap">
                                                    <div style="display: flex; align-items: center; justify-content: space-between; width: 100%; gap: 0.3rem;">
                                                        <span class="tab-name">{move || m.name(current_lang.get())}</span>
                                                        <span class="weapon-group-pill" style="font-size: 0.62rem; padding: 2px 5px;">
                                                            {move || m.category.name(current_lang.get())}
                                                        </span>
                                                    </div>
                                                    <span class="tab-sub">
                                                        "Dif " {move || m.difficulty(current_lang.get())} " • " {move || m.damage(current_lang.get())}
                                                    </span>
                                                </div>
                                            </button>
                                        }
                                    }).collect_view().into_view()
                                }
                            }}
                        </div>
                    </div>
                }.into_view(),
            }}
        </div>
    }
}
