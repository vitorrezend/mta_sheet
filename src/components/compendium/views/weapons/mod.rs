use leptos::*;
use crate::compendium::weapons::{
    CombatManeuver, ManeuverCategory, WeaponCategory, WeaponDefinition,
    WeaponMainGroup, ALL_COMBAT_MANEUVERS, ALL_WEAPONS,
};
use crate::components::Callback;
use crate::i18n::Language;

pub mod callouts;
pub mod legend;
pub mod maneuver_card;
pub mod sidebar;
pub mod weapon_card;

pub use callouts::{render_do_rules_card, render_eight_limbs_card, render_thunder_punch_callout_box};
pub use legend::render_combat_legend_view;
pub use maneuver_card::render_maneuver_card;
pub use sidebar::render_weapons_sidebar;
pub use weapon_card::render_weapon_card;

pub use crate::compendium::weapons::CombatSubTab;

#[component]
pub fn WeaponsView(
    selected_weapon_id: RwSignal<String>,
    current_lang: Signal<Language>,
    #[prop(into, default = None)] mobile_show_detail: Option<RwSignal<bool>>,
    target_slot: Option<Signal<Option<usize>>>,
    #[prop(into, default = None)] on_select_weapon: Option<Callback<(Option<usize>, &'static WeaponDefinition)>>,
    #[prop(into, default = None)] on_select_maneuver: Option<Callback<(Option<usize>, &'static CombatManeuver)>>,
    #[prop(into, default = None)] on_close: Option<Callback<()>>,
    #[prop(into, default = None)] combat_subtab: Option<RwSignal<CombatSubTab>>,
    #[prop(into, default = None)] selected_maneuver_id: Option<RwSignal<String>>,
) -> impl IntoView {
    // Sub-aba ativa (Armas vs Manobras de Combate & Briga)
    let combat_subtab = combat_subtab.unwrap_or_else(|| create_rw_signal(CombatSubTab::Weapons));

    // ID da manobra de combate selecionada
    let selected_maneuver_id = selected_maneuver_id.unwrap_or_else(|| create_rw_signal("punch".to_string()));

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

    // Quando selected_weapon_id, selected_maneuver_id ou combat_subtab mudam, fecha leituras de banners/legenda
    create_effect(move |_| {
        let _ = selected_weapon_id.get();
        let _ = selected_maneuver_id.get();
        let _ = combat_subtab.get();
        show_general_legend.set(false);
        show_thunder_punch.set(false);
        show_eight_limbs.set(false);
        show_do_rules.set(false);
    });

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
            {render_weapons_sidebar(
                combat_subtab,
                selected_weapon_id,
                selected_maneuver_id,
                selected_group,
                selected_category,
                selected_maneuver_cat,
                search_query,
                show_general_legend,
                show_thunder_punch,
                show_eight_limbs,
                show_do_rules,
                current_lang,
                mobile_show_detail,
            )}

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
                    view! {
                        {render_combat_legend_view(current_lang)}
                    }.into_view()
                } else if show_thunder_punch.get() {
                    view! {
                        <div class="weapon-reading-view maneuver-reading-view">
                            {render_thunder_punch_callout_box(current_lang)}
                        </div>
                    }.into_view()
                } else if show_eight_limbs.get() {
                    view! {
                        <div class="weapon-reading-view maneuver-reading-view">
                            {render_eight_limbs_card(current_lang)}
                        </div>
                    }.into_view()
                } else if show_do_rules.get() {
                    view! {
                        <div class="weapon-reading-view maneuver-reading-view">
                            {render_do_rules_card(current_lang)}
                        </div>
                    }.into_view()
                } else if combat_subtab.get() == CombatSubTab::Maneuvers {
                    let maneuver = active_maneuver.get();
                    view! {
                        {render_maneuver_card(
                            maneuver,
                            current_lang,
                            target_slot,
                            on_select_m_act.clone(),
                            on_close_act.clone(),
                            show_thunder_punch,
                            show_eight_limbs,
                            show_do_rules,
                            show_general_legend,
                        )}
                    }.into_view()
                } else {
                    let weapon = active_weapon.get();
                    view! {
                        {render_weapon_card(
                            weapon,
                            current_lang,
                            target_slot,
                            on_select_act.clone(),
                            on_close_act.clone(),
                        )}
                    }.into_view()
                }}
            </div>
        </div>
    }
}
