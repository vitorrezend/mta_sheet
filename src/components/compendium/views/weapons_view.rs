use leptos::*;
use crate::compendium::weapons::{
    explain_weapon_note, get_categories_by_main_group, get_weapons_by_category,
    get_weapons_by_class, get_weapons_by_main_group, ALL_RULE_NOTES, ALL_WEAPONS,
    ALL_WEAPON_CATEGORIES, ALL_WEAPON_CLASSES, ALL_WEAPON_MAIN_GROUPS,
    MELEE_CATEGORIES, RANGED_CATEGORIES, RANGED_NOTE_1, RANGED_NOTE_2, RANGED_NOTE_3,
    RANGED_NOTE_4, RANGED_NOTE_5, RANGED_NOTE_6, RANGED_NOTE_7, RANGED_NOTE_8,
    RANGED_NOTE_9, RANGED_NOTE_10, RANGED_NOTE_11, RANGED_NOTE_12, WeaponCategory,
    WeaponClass, WeaponDefinition, WeaponMainGroup,
    CombatManeuver, ManeuverCategory, ALL_COMBAT_MANEUVERS, ALL_MANEUVER_CATEGORIES,
    get_maneuvers_by_category, THUNDER_PUNCH_TRICK, EIGHT_LIMBS_ARTICLE, DO_RULES_ARTICLE,
};
use crate::components::Callback;
use crate::i18n::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatSubTab {
    Weapons,
    Maneuvers,
}

/// Renderiza a Box Oficial do Livro M20 (p. 449): "Mage Trick: The Thunder Punch"
fn render_thunder_punch_callout_box(lang: Signal<Language>) -> impl IntoView {
    let trick = &THUNDER_PUNCH_TRICK;
    view! {
        <div class="thunder-punch-callout-box">
            // Cabeçalho da Box Oficial
            <div class="thunder-punch-header">
                <div class="thunder-punch-hero-icon" style="font-size: 2rem; margin-bottom: 0.25rem;">"⚡"</div>
                <h2 class="thunder-punch-hero-title">
                    {move || trick.title(lang.get())}
                </h2>
                <span class="thunder-punch-hero-subtitle">
                    {move || match lang.get() {
                        Language::PtBr => "M20 • Livro de Regras Básico • Capítulo 9: Combate & Narrativa (p. 449)",
                        Language::EnUs => "M20 Core Rulebook • Chapter 9: Combat & Storytelling (p. 449)",
                    }}
                </span>
            </div>

            // Esferas Envolvidas
            <div class="thunder-punch-spheres-wrap">
                <span class="thunder-punch-spheres-label">
                    "🔮 " {move || match lang.get() {
                        Language::PtBr => "Esferas Aplicáveis & Efeitos de Iluminação:",
                        Language::EnUs => "Applicable Spheres & Enlightened Effects:",
                    }}
                </span>
                <div class="thunder-punch-spheres-list">
                    {move || trick.sphere_tags(lang.get()).iter().map(|&tag| {
                        view! {
                            <span class="thunder-sphere-pill">
                                "✨ " {tag}
                            </span>
                        }
                    }).collect_view()}
                </div>
            </div>

            // Grid dos 4 Pilares Táticos de Regras
            <div class="thunder-tactical-grid">
                <div class="thunder-tactical-card">
                    <span class="thunder-tactical-card-title">
                        "🎯 " {move || match lang.get() {
                            Language::PtBr => "Modificador de Dificuldade",
                            Language::EnUs => "Difficulty Modifier",
                        }}
                    </span>
                    <p class="thunder-tactical-card-desc">
                        {move || trick.difficulty_rule(lang.get())}
                    </p>
                </div>

                <div class="thunder-tactical-card">
                    <span class="thunder-tactical-card-title">
                        "💥 " {move || match lang.get() {
                            Language::PtBr => "Dano & Metamágica",
                            Language::EnUs => "Damage & Metamagick",
                        }}
                    </span>
                    <p class="thunder-tactical-card-desc">
                        {move || trick.damage_rule(lang.get())}
                    </p>
                </div>

                <div class="thunder-tactical-card">
                    <span class="thunder-tactical-card-title">
                        "⚡ " {move || match lang.get() {
                            Language::PtBr => "Coincidente vs Vulgar",
                            Language::EnUs => "Coincidental vs Vulgar",
                        }}
                    </span>
                    <p class="thunder-tactical-card-desc">
                        {move || match lang.get() {
                            Language::PtBr => "Coincidente se parecer golpe perfeito. Vulgar se desproporcional à compleição física.",
                            Language::EnUs => "Coincidental if looking like a perfect strike. Vulgar if visibly defying physical build.",
                        }}
                    </p>
                </div>

                <div class="thunder-tactical-card">
                    <span class="thunder-tactical-card-title">
                        "⚠️ " {move || match lang.get() {
                            Language::PtBr => "Efeito Reverso / Absorção",
                            Language::EnUs => "Backlash / Full Soak",
                        }}
                    </span>
                    <p class="thunder-tactical-card-desc">
                        {move || trick.backlash_rule(lang.get())}
                    </p>
                </div>
            </div>

            // Texto Original / Traduzido do Livro M20 (3 Parágrafos)
            <div class="thunder-punch-body-paragraphs">
                {move || trick.paragraphs(lang.get()).iter().map(|&p| {
                    view! {
                        <p class="thunder-punch-paragraph">
                            {p}
                        </p>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Renderiza o Card Dedicado dos Oito Membros da Maestria (Dô / Akashayana)
#[allow(dead_code)]
fn render_eight_limbs_card(lang: Signal<Language>) -> impl IntoView {
    let article = &EIGHT_LIMBS_ARTICLE;
    view! {
        <div class="eight-limbs-callout-box">
            // Cabeçalho da Box Oficial dos Oito Membros
            <div class="eight-limbs-header">
                <div class="eight-limbs-hero-icon" style="font-size: 2.2rem; margin-bottom: 0.25rem;">"🪷"</div>
                <h2 class="eight-limbs-hero-title">
                    {move || article.title(lang.get())}
                </h2>
                <span class="eight-limbs-hero-subtitle">
                    {move || article.subtitle(lang.get())}
                </span>
                <p class="eight-limbs-intro">
                    {move || article.intro(lang.get())}
                </p>
            </div>

            // Grid dos Oito Membros da Maestria
            <div class="eight-limbs-grid">
                {article.limbs.iter().map(|limb| {
                    view! {
                        <div class="limb-card">
                            <div class="limb-card-header">
                                <span class="limb-bullet">"🪷"</span>
                                <h3 class="limb-title">
                                    {move || limb.title(lang.get())}
                                </h3>
                            </div>
                            <p class="limb-desc">
                                {move || limb.description(lang.get())}
                            </p>
                            <div class="limb-abilities-wrap">
                                <span class="limb-abilities-label">
                                    {move || match lang.get() {
                                        Language::PtBr => "Habilidades Associadas:",
                                        Language::EnUs => "Associated Abilities:",
                                    }}
                                </span>
                                <div class="limb-abilities-tags">
                                    {limb.abilities(Language::EnUs).iter().zip(limb.abilities(Language::PtBr).iter()).map(|(&en, &pt)| {
                                        view! {
                                            <span class="limb-ability-badge">
                                                {move || match lang.get() {
                                                    Language::PtBr => pt,
                                                    Language::EnUs => en,
                                                }}
                                            </span>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>

            // Card da Regra de Progressão dos Membros
            <div class="eight-limbs-progression-card">
                <div class="progression-header">
                    <span class="progression-icon">"⚖️"</span>
                    <strong class="progression-title">
                        {move || match lang.get() {
                            Language::PtBr => "REGRA DE PROGRESSÃO & ESTUDO DOS MEMBROS",
                            Language::EnUs => "PROGRESSION RULE & STUDY OF THE LIMBS",
                        }}
                    </strong>
                </div>
                <p class="progression-desc">
                    {move || article.progression_rule(lang.get())}
                </p>
            </div>

            // Box Oficial da Regra Opcional: O Caminho Pacífico (The Peaceful Way)
            <div class="peaceful-way-callout-box">
                <div class="peaceful-way-header">
                    <span class="peaceful-way-icon">"🕊️"</span>
                    <h3 class="peaceful-way-title">
                        {move || article.peaceful_way_title(lang.get())}
                    </h3>
                </div>
                <div class="peaceful-way-desc-paragraphs">
                    {move || {
                        let text = article.peaceful_way_rule(lang.get());
                        text.split("\n\n").map(|p| {
                            view! {
                                <p class="peaceful-way-p">{p}</p>
                            }
                        }).collect_view()
                    }}
                </div>
            </div>
        </div>
    }
}

/// Renderiza o Card de Regras Canônicas & Treinamento de Dô (M20)
#[allow(dead_code)]
fn render_do_rules_card(lang: Signal<Language>) -> impl IntoView {
    let article = &DO_RULES_ARTICLE;
    view! {
        <div class="do-rules-callout-box">
            // Cabeçalho Oficial
            <div class="do-rules-header">
                <div class="do-rules-hero-icon" style="font-size: 2.2rem; margin-bottom: 0.25rem;">"🥋"</div>
                <h2 class="do-rules-hero-title">
                    {move || article.title(lang.get())}
                </h2>
                <span class="do-rules-hero-subtitle">
                    {move || article.subtitle(lang.get())}
                </span>
                <p class="do-rules-overview">
                    {move || article.overview(lang.get())}
                </p>
            </div>

            // Card de Treino Diário e Compromisso
            <div class="do-commitment-card">
                <div class="commitment-header">
                    <span class="commitment-icon">"⏳"</span>
                    <strong class="commitment-title">
                        {move || match lang.get() {
                            Language::PtBr => "COMPROMISSO, TREINAMENTO DIÁRIO & LIMITES",
                            Language::EnUs => "COMMITMENT, DAILY TRAINING & LIMITS",
                        }}
                    </strong>
                </div>
                <p class="commitment-desc">
                    {move || article.commitment(lang.get())}
                </p>
            </div>

            // Grid das Vantagens e Regras do Sistema
            <div class="do-advantages-grid">
                {article.advantages.iter().map(|adv| {
                    view! {
                        <div class="do-advantage-card">
                            <div class="advantage-card-header">
                                <span class="advantage-icon">{adv.icon}</span>
                                <h4 class="advantage-title">{move || adv.title(lang.get())}</h4>
                            </div>
                            <p class="advantage-rule">{move || adv.rule(lang.get())}</p>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

#[component]
pub fn WeaponsView(
    selected_weapon_id: RwSignal<String>,
    current_lang: Signal<Language>,
    #[prop(into, default = None)] mobile_show_detail: Option<RwSignal<bool>>,
    target_slot: Option<Signal<Option<usize>>>,
    #[prop(into, default = None)] on_select_weapon: Option<Callback<(Option<usize>, &'static WeaponDefinition)>>,
    #[prop(into, default = None)] on_select_maneuver: Option<Callback<(Option<usize>, &'static CombatManeuver)>>,
    #[prop(into, default = None)] on_close: Option<Callback<()>>,
) -> impl IntoView {
    // Sub-aba ativa (Armas vs Manobras de Combate & Briga)
    let combat_subtab = create_rw_signal(CombatSubTab::Weapons);

    // ID da manobra de combate selecionada
    let selected_maneuver_id = create_rw_signal("punch".to_string());

    // Filtro por Categoria de Manobra (None = Todas)
    let selected_maneuver_cat = create_rw_signal(Option::<ManeuverCategory>::None);

    // Filtro por Grupo Principal de Armas (Corpo a Corpo ou À Distância) - None = Todas
    let selected_group = create_rw_signal(Option::<WeaponMainGroup>::None);

    // Filtro por Categoria Específica dentro do Grupo - None = Todas do Grupo
    let selected_category = create_rw_signal(Option::<WeaponCategory>::None);

    // Texto de busca
    let search_query = create_rw_signal(String::new());

    // Visualizar legenda geral
    let show_general_legend = create_rw_signal(false);

    // Visualizar Box Oficial M20: Mage Trick: The Thunder Punch (p. 449)
    let show_thunder_punch = create_rw_signal(false);

    // Visualizar Card Dedicado dos Oito Membros da Maestria (Dô / Akashayana)
    let show_eight_limbs = create_rw_signal(false);

    // Visualizar Card de Regras Canônicas & Treinamento de Dô
    let show_do_rules = create_rw_signal(false);

    let active_weapon = Signal::derive(move || {
        let cur_id = selected_weapon_id.get();
        ALL_WEAPONS
            .iter()
            .find(|w| w.id == cur_id)
            .unwrap_or(&ALL_WEAPONS[0])
    });

    let active_maneuver = Signal::derive(move || {
        let cur_id = selected_maneuver_id.get();
        ALL_COMBAT_MANEUVERS
            .iter()
            .find(|m| m.id == cur_id)
            .unwrap_or(&ALL_COMBAT_MANEUVERS[0])
    });

    let on_select_act = on_select_weapon;
    let on_select_m_act = on_select_maneuver;
    let on_close_act = on_close;

    view! {
        <div 
            class="compendium-section-split weapon-compendium-split"
            class:mobile-show-detail=move || mobile_show_detail.map(|s| s.get()).unwrap_or(false)
        >
            // ================= COLUNA 1: Lista e Filtros de Armas / Manobras =================
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

            // ================= COLUNA 2: Detalhes da Arma ou Regras Gerais =================
            <div class="practice-detail-pane weapon-detail-pane">
                // Botão de Retorno no Mobile (visível apenas em telas <= 768px via CSS)
                {move || mobile_show_detail.map(|msd| {
                    view! {
                        <button
                            type="button"
                            class="compendium-mobile-back-btn"
                            on:click=move |_| msd.set(false)
                        >
                            <span class="back-arrow">"←"</span>
                            <span>{move || match current_lang.get() {
                                Language::PtBr => match combat_subtab.get() {
                                    CombatSubTab::Weapons => "Voltar para a Lista de Armas",
                                    CombatSubTab::Maneuvers => "Voltar para a Lista de Manobras",
                                },
                                Language::EnUs => match combat_subtab.get() {
                                    CombatSubTab::Weapons => "Back to Weapons List",
                                    CombatSubTab::Maneuvers => "Back to Maneuvers List",
                                },
                            }}</span>
                        </button>
                    }
                })}

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

                            // Regras Canônicas de Artes Marciais (M20 pp. 423-426, 580-581)
                            <div class="legend-notes-title" style="margin-top: 1.5rem;">
                                {move || match current_lang.get() {
                                    Language::PtBr => "ARTES MARCIAIS: ESTILOS, MANOBRAS & FOCO MÁGICO (M20 pp. 423-426, 580-581)",
                                    Language::EnUs => "MARTIAL ARTS: STYLES, MANEUVERS & MAGICKAL FOCUS (M20 pp. 423-426, 580-581)",
                                }}
                            </div>
                            <div class="legend-params-grid">
                                <div class="legend-param-card">
                                    <strong>{move || match current_lang.get() {
                                        Language::PtBr => "🥊 Estilos Duros (Hard Styles)",
                                        Language::EnUs => "🥊 Hard Styles",
                                    }}</strong>
                                    <p>{move || match current_lang.get() {
                                        Language::PtBr => "Focam em impacto direto, golpes lineares devastadores, socos e chutes contundentes (Karatê, Boxe, Muay Thai, Krav Maga, Tae Kwon Do). Priorizam força, velocidade e neutralização ofensiva imediata.",
                                        Language::EnUs => "Focus on direct kinetic impact, linear strikes, and punishing punches and kicks (Karate, Boxing, Muay Thai, Krav Maga, Tae Kwon Do). Emphasize force, speed, and immediate offensive neutralization.",
                                    }}</p>
                                </div>
                                <div class="legend-param-card">
                                    <strong>{move || match current_lang.get() {
                                        Language::PtBr => "🌊 Estilos Suaves (Soft Styles)",
                                        Language::EnUs => "🌊 Soft Styles",
                                    }}</strong>
                                    <p>{move || match current_lang.get() {
                                        Language::PtBr => "Focam em movimentos circulares, alavancagem, esquivas fluidas, chaves de articulação e redirecionamento do ímpeto adversário (Aikidô, Judô, Tai Chi Chuan, Jujutsu, Hapkido). Usam a própria força do atacante contra ele.",
                                        Language::EnUs => "Focus on circular motion, leverage, fluid evasion, joint locks, and redirecting the opponent's momentum (Aikido, Judo, Tai Chi, Jujutsu, Hapkido). Turn the attacker's own strength against them.",
                                    }}</p>
                                </div>
                                <div class="legend-param-card">
                                    <strong>{move || match current_lang.get() {
                                        Language::PtBr => "🥋 Aquisição de Manobras (2 por ponto)",
                                        Language::EnUs => "🥋 Maneuver Selection (2 per dot)",
                                    }}</strong>
                                    <p>{move || match current_lang.get() {
                                        Language::PtBr => "Um personagem adquire 2 manobras marciais para cada ponto em Artes Marciais (como especialidade de Briga/Luta, Armas Brancas ou Habilidade Secundária). O lutador deve atender aos pré-requisitos de estilo e pontuação mínima.",
                                        Language::EnUs => "A character selects 2 martial maneuvers per dot in Martial Arts (as a Brawl/Melee specialty or Secondary Ability). The combatant must meet style requirements and minimum ability ratings.",
                                    }}</p>
                                </div>
                                <div class="legend-param-card">
                                    <strong>{move || match current_lang.get() {
                                        Language::PtBr => "🔮 Artes Marciais como Foco Mágico (Dô)",
                                        Language::EnUs => "🔮 Martial Arts as Magickal Focus (Do)",
                                    }}</strong>
                                    <p>{move || match current_lang.get() {
                                        Language::PtBr => "M20 pp. 580-581: Magos marciais canalizam o Chi através de seus corpos como instrumento focal. Permite conjurar Mágika Iluminada em combate (Forças cinética, Vida estrutural, Mente disciplinada, Correspondência espacial e Tempo acelerado).",
                                        Language::EnUs => "M20 pp. 580-581: Martial mages channel Chi through disciplined physical katas as a magickal focus instrument. Enhances combat with Enlightened Magick (kinetic Forces, biological Life, disciplined Mind, spatial Correspondence, and accelerated Time).",
                                    }}</p>
                                </div>
                            </div>
                        </div>
                    }.into_view()
                } else if show_thunder_punch.get() {
                    // Visualização da Box Oficial M20: "Mage Trick: The Thunder Punch" (p. 449)
                    view! {
                        <div class="weapon-reading-view maneuver-reading-view">
                            {render_thunder_punch_callout_box(current_lang)}
                        </div>
                    }.into_view()
                } else if show_eight_limbs.get() {
                    // Card Dedicado dos Oito Membros da Maestria (Dô / Akashayana)
                    view! {
                        <div class="weapon-reading-view maneuver-reading-view">
                            {render_eight_limbs_card(current_lang)}
                        </div>
                    }.into_view()
                } else if show_do_rules.get() {
                    // Card de Regras Canônicas & Treinamento de Dô (M20)
                    view! {
                        <div class="weapon-reading-view maneuver-reading-view">
                            {render_do_rules_card(current_lang)}
                        </div>
                    }.into_view()
                } else if combat_subtab.get() == CombatSubTab::Maneuvers {
                    // Detalhe da Manobra de Combate Ativa
                    let maneuver = active_maneuver.get();
                    let m_cat = maneuver.category;

                    let slot_idx = target_slot.and_then(|s| s.get());
                    let on_sel_m = on_select_m_act.clone();
                    let on_cls = on_close_act.clone();

                    view! {
                        <div class="weapon-reading-view maneuver-reading-view">
                            // Cabeçalho da Manobra
                            <div class="weapon-detail-header">
                                <div class="weapon-title-box">
                                    <div class="weapon-main-name">
                                        <span class="weapon-detail-icon">{m_cat.icon()}</span>
                                        {move || maneuver.name(current_lang.get())}
                                    </div>
                                    <span class="weapon-alt-name">
                                        {move || match current_lang.get() {
                                            Language::PtBr => format!("Original em Inglês: {}", maneuver.name),
                                            Language::EnUs => format!("Nome em Português: {}", maneuver.name_pt),
                                        }}
                                    </span>
                                </div>
                                <div class="weapon-header-badges">
                                    <span class="badge-category">{move || m_cat.name(current_lang.get())}</span>
                                    {if m_cat == ManeuverCategory::MartialArts {
                                        let req = maneuver.requirement;
                                        let is_hard = req.contains("hard") || req.contains("Hard");
                                        let is_soft = req.contains("soft") || req.contains("Soft");
                                        let style_badge = if is_hard && !is_soft {
                                            view! { <span class="badge-hard-style">{move || match current_lang.get() { Language::PtBr => "🥊 Estilo Duro", Language::EnUs => "🥊 Hard Style" }}</span> }.into_view()
                                        } else if is_soft && !is_hard {
                                            view! { <span class="badge-soft-style">{move || match current_lang.get() { Language::PtBr => "🌊 Estilo Suave", Language::EnUs => "🌊 Soft Style" }}</span> }.into_view()
                                        } else {
                                            view! { <span class="badge-any-style">{move || match current_lang.get() { Language::PtBr => "⚖️ Qualquer Estilo", Language::EnUs => "⚖️ Any Style" }}</span> }.into_view()
                                        };
                                        view! {
                                            {style_badge}
                                            <span class="badge-page">"M20 pp. 423-426"</span>
                                        }.into_view()
                                    } else if m_cat == ManeuverCategory::Do {
                                        view! {
                                            <span class="badge-any-style" style="background: rgba(217, 119, 6, 0.15); border-color: rgba(217, 119, 6, 0.4); color: #b45309;">
                                                {move || match current_lang.get() {
                                                    Language::PtBr => "🪷 Dô Akashiano",
                                                    Language::EnUs => "🪷 Akashic Do",
                                                }}
                                            </span>
                                            <span class="badge-page">"M20 pp. 423-426, 580-581"</span>
                                        }.into_view()
                                    } else {
                                        view! {
                                            <span class="badge-page">"M20 pp. 448-450"</span>
                                        }.into_view()
                                    }}
                                </div>
                            </div>

                            // Banner de Estatísticas da Manobra (4 colunas)
                            <div class="weapon-stats-banner maneuver-stats-banner">
                                <div class="stat-box">
                                    <span class="stat-label">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "PARADA DE TESTE",
                                            Language::EnUs => "DICE POOL",
                                        }}
                                    </span>
                                    <span class=move || {
                                        let len = maneuver.roll(current_lang.get()).chars().count();
                                        if len > 22 {
                                            "stat-value stat-pool stat-value-long"
                                        } else if len > 12 {
                                            "stat-value stat-pool stat-value-dense"
                                        } else {
                                            "stat-value stat-pool"
                                        }
                                    }>
                                        {move || maneuver.roll(current_lang.get())}
                                    </span>
                                    <span class="stat-sub">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "Atributo + Habilidade",
                                            Language::EnUs => "Trait + Ability",
                                        }}
                                    </span>
                                </div>

                                <div class="stat-box">
                                    <span class="stat-label">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "DIFICULDADE",
                                            Language::EnUs => "DIFFICULTY",
                                        }}
                                    </span>
                                    <span class=move || {
                                        let len = maneuver.difficulty(current_lang.get()).chars().count();
                                        if len > 22 {
                                            "stat-value stat-diff stat-value-long"
                                        } else if len > 12 {
                                            "stat-value stat-diff stat-value-dense"
                                        } else {
                                            "stat-value stat-diff"
                                        }
                                    }>
                                        {move || maneuver.difficulty(current_lang.get())}
                                    </span>
                                    <span class="stat-sub">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "Dificuldade Base",
                                            Language::EnUs => "Base Difficulty",
                                        }}
                                    </span>
                                </div>

                                <div class="stat-box stat-box-damage">
                                    <span class="stat-label">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "DANO / EFEITO",
                                            Language::EnUs => "DAMAGE / EFFECT",
                                        }}
                                    </span>
                                    <span class=move || {
                                        let len = maneuver.damage(current_lang.get()).chars().count();
                                        if len > 22 {
                                            "stat-value stat-dmg stat-value-long"
                                        } else if len > 12 {
                                            "stat-value stat-dmg stat-value-dense"
                                        } else {
                                            "stat-value stat-dmg"
                                        }
                                    }>
                                        {move || maneuver.damage(current_lang.get())}
                                    </span>
                                    <span class="stat-sub">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "Dano Causado",
                                            Language::EnUs => "Damage Inflicted",
                                        }}
                                    </span>
                                </div>

                                <div class="stat-box">
                                    <span class="stat-label">
                                        {move || match current_lang.get() {
                                            Language::PtBr => "AÇÕES",
                                            Language::EnUs => "ACTIONS",
                                        }}
                                    </span>
                                    <span class="stat-value stat-actions" style="color: #6b21a8;">
                                        {maneuver.actions}
                                    </span>
                                    <span class="stat-sub">
                                        {move || match (maneuver.actions, current_lang.get()) {
                                            (1, Language::PtBr) => "1 Turno / Ação",
                                            (1, Language::EnUs) => "1 Turn / Action",
                                            (_, Language::PtBr) => "Múltiplas Ações",
                                            (_, Language::EnUs) => "Multiple Actions",
                                        }}
                                    </span>
                                </div>
                            </div>

                            // Requisito ou Condição Especial (se houver)
                            {if !maneuver.requirement.is_empty() {
                                view! {
                                    <div class="maneuver-requirement-card">
                                        <div class="requirement-header">
                                            <span class="requirement-icon">"⚠️"</span>
                                            <strong class="requirement-title">
                                                {move || match current_lang.get() {
                                                    Language::PtBr => "REQUISITO / CONDIÇÃO TÁTICA",
                                                    Language::EnUs => "REQUIREMENT / TACTICAL CONDITION",
                                                }}
                                            </strong>
                                        </div>
                                        <p class="requirement-desc">
                                            {move || maneuver.requirement(current_lang.get())}
                                        </p>
                                    </div>
                                }.into_view()
                            } else {
                                view! { <span></span> }.into_view()
                            }}

                            // Descrição Canônica e Emprego Tático
                            <div class="weapon-desc-section">
                                <h4 class="weapon-section-title">
                                    {move || match current_lang.get() {
                                        Language::PtBr => "DESCRIÇÃO & REGRAS CANÔNICAS (M20)",
                                        Language::EnUs => "DESCRIPTION & CANONICAL RULES (M20)",
                                    }}
                                </h4>
                                <div class="maneuver-desc-paragraphs">
                                    {move || {
                                        let desc = maneuver.description(current_lang.get());
                                        desc.split("\n\n").map(|p| {
                                            view! {
                                                <p class="weapon-desc-text" style="margin-bottom: 0.65rem;">
                                                    {p}
                                                </p>
                                            }
                                        }).collect_view()
                                    }}
                                </div>
                            </div>

                            // Box Integrada de Truque de Mago para ataques desarmados (Soco, Chute)
                            {if maneuver.id == "punch" || maneuver.id == "kick" {
                                view! {
                                    <div class="maneuver-embedded-thunder-callout">
                                        <div class="weapon-desc-section">
                                            <h4 class="maneuver-embedded-thunder-title">
                                                <span>"⚡"</span>
                                                {move || match current_lang.get() {
                                                    Language::PtBr => "TRUQUE DE MAGO APLICÁVEL (BOX M20, p. 449)",
                                                    Language::EnUs => "APPLICABLE MAGE TRICK (M20 BOX, p. 449)",
                                                }}
                                            </h4>
                                            {render_thunder_punch_callout_box(current_lang)}
                                        </div>
                                    </div>
                                }.into_view()
                            } else if maneuver.category == ManeuverCategory::Do {
                                view! {
                                    <div class="maneuver-embedded-do-callout">
                                        <div class="maneuver-embedded-do-card">
                                            <div class="maneuver-embedded-do-header">
                                                <span class="maneuver-embedded-do-icon">"🪷"</span>
                                                <h4 class="maneuver-embedded-do-title">
                                                    {move || match current_lang.get() {
                                                        Language::PtBr => "DISCIPLINA DE DÔ (IRMANDADE DE AKASHAYANA)",
                                                        Language::EnUs => "DO DISCIPLINE (AKASHIC BROTHERHOOD)",
                                                    }}
                                                </h4>
                                            </div>
                                            <p class="maneuver-embedded-do-desc">
                                                {move || match current_lang.get() {
                                                    Language::PtBr => "Esta técnica canônica requer treinamento na arte marcial Dô (Habilidade Secundária). O praticante deve dominar os Oito Membros da Maestria e manter pelo menos uma hora diária de meditação/katas para preservar sua proficiência.",
                                                    Language::EnUs => "This canonical technique requires training in the martial art Do (Secondary Ability). The practitioner must master the Eight Limbs of Expertise and commit to at least one hour daily of practice/katas to maintain proficiency.",
                                                }}
                                            </p>
                                            <div class="maneuver-embedded-do-actions">
                                                <button
                                                    type="button"
                                                    class="embedded-do-action-btn"
                                                    on:click=move |_| {
                                                        show_eight_limbs.set(true);
                                                        show_do_rules.set(false);
                                                        show_thunder_punch.set(false);
                                                        show_general_legend.set(false);
                                                    }
                                                >
                                                    <span class="embedded-do-btn-icon">"📜"</span>
                                                    <span class="embedded-do-btn-label">
                                                        {move || match current_lang.get() {
                                                            Language::PtBr => "Ver os 8 Membros da Maestria",
                                                            Language::EnUs => "View Eight Limbs of Expertise",
                                                        }}
                                                    </span>
                                                </button>
                                                <button
                                                    type="button"
                                                    class="embedded-do-action-btn"
                                                    on:click=move |_| {
                                                        show_do_rules.set(true);
                                                        show_eight_limbs.set(false);
                                                        show_thunder_punch.set(false);
                                                        show_general_legend.set(false);
                                                    }
                                                >
                                                    <span class="embedded-do-btn-icon">"🥋"</span>
                                                    <span class="embedded-do-btn-label">
                                                        {move || match current_lang.get() {
                                                            Language::PtBr => "Ver Regras & Treino de Dô",
                                                            Language::EnUs => "View Do Rules & Training",
                                                        }}
                                                    </span>
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                }.into_view()
                            } else {
                                view! { <span></span> }.into_view()
                            }}

                            // Botão de Ação: Equipar Manobra no Slot de Combate da Ficha
                            {if let Some(cb) = on_sel_m {
                                let m_to_equip = maneuver;
                                let on_close_action = on_cls;
                                view! {
                                    <div class="weapon-equip-action-row" style="margin-top: 1.5rem;">
                                        <button
                                            type="button"
                                            class="practice-select-btn weapon-equip-btn"
                                            on:click=move |_| {
                                                cb.call((slot_idx, m_to_equip));
                                                if let Some(c) = &on_close_action {
                                                    c.call(());
                                                }
                                            }
                                        >
                                            {move || match (slot_idx, current_lang.get()) {
                                                (Some(idx), Language::PtBr) => format!("✦ Equipar Manobra '{}' na Linha #{}", m_to_equip.name(current_lang.get()), idx + 1),
                                                (Some(idx), Language::EnUs) => format!("✦ Equip Maneuver '{}' in Row #{}", m_to_equip.name(current_lang.get()), idx + 1),
                                                (None, Language::PtBr) => format!("✦ Equipar Manobra '{}' na Tabela de Combate", m_to_equip.name(current_lang.get())),
                                                (None, Language::EnUs) => format!("✦ Equip Maneuver '{}' to Combat Table", m_to_equip.name(current_lang.get())),
                                            }}
                                        </button>
                                    </div>
                                }.into_view()
                            } else {
                                view! { <span></span> }.into_view()
                            }}
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
