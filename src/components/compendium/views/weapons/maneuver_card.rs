use leptos::*;
use crate::compendium::weapons::{CombatManeuver, ManeuverCategory};
use crate::components::{Callback, StatBox};
use crate::i18n::Language;
use super::callouts::render_thunder_punch_callout_box;

/// Renderiza o painel de leitura detalhada de uma Manobra de Combate Selecionada
pub fn render_maneuver_card(
    maneuver: &'static CombatManeuver,
    current_lang: Signal<Language>,
    target_slot: Option<Signal<Option<usize>>>,
    on_select_m_act: Option<Callback<(Option<usize>, &'static CombatManeuver)>>,
    on_close_act: Option<Callback<()>>,
    show_thunder_punch: RwSignal<bool>,
    show_eight_limbs: RwSignal<bool>,
    show_do_rules: RwSignal<bool>,
    show_general_legend: RwSignal<bool>,
) -> impl IntoView {
    let m_cat = maneuver.category;
    let slot_idx = target_slot.and_then(|s| s.get());
    let on_sel_header = on_select_m_act.clone();
    let on_sel_footer = on_select_m_act;
    let on_cls_header = on_close_act.clone();
    let on_cls_footer = on_close_act;

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
                <div class="weapon-header-actions-wrap" style="display: flex; align-items: center; gap: 0.65rem; margin-left: auto; flex-wrap: wrap;">
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
                                <span class="badge-any-style" style="background: var(--token-do-surface-tint, rgba(217, 119, 6, 0.15)); border-color: var(--token-do-border, rgba(217, 119, 6, 0.4)); color: var(--token-do-gold, #b45309);">
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
                    {if let Some(cb) = on_sel_header {
                        let m_to_equip = maneuver;
                        let on_close_action = on_cls_header;
                        view! {
                            <div class="compendium-header-actions">
                                <button
                                    type="button"
                                    class="compendium-action-btn compendium-btn-emerald"
                                    on:click=move |_| {
                                        cb.call((slot_idx, m_to_equip));
                                        if let Some(c) = &on_close_action {
                                            c.call(());
                                        }
                                    }
                                >
                                    <span class="compendium-btn-icon">"🥋"</span>
                                    <span class="compendium-btn-label">
                                        {move || match (slot_idx, current_lang.get()) {
                                            (Some(idx), Language::PtBr) => format!("Equipar (#{})", idx + 1),
                                            (Some(idx), Language::EnUs) => format!("Equip (#{})", idx + 1),
                                            (None, Language::PtBr) => "Equipar na Ficha".to_string(),
                                            (None, Language::EnUs) => "Equip on Sheet".to_string(),
                                        }}
                                    </span>
                                </button>
                            </div>
                        }.into_view()
                    } else {
                        view! { <span></span> }.into_view()
                    }}
                </div>
            </div>

            // Banner de Estatísticas da Manobra (4 colunas)
            <div class="weapon-stats-banner maneuver-stats-banner">
                <StatBox
                    label=Signal::derive(move || match current_lang.get() {
                        Language::PtBr => "PARADA DE TESTE".to_string(),
                        Language::EnUs => "DICE POOL".to_string(),
                    })
                    value=Signal::derive(move || maneuver.roll(current_lang.get()).to_string())
                    sub=Signal::derive(move || match current_lang.get() {
                        Language::PtBr => "Atributo + Habilidade".to_string(),
                        Language::EnUs => "Trait + Ability".to_string(),
                    })
                    color_class="stat-pool"
                />

                <StatBox
                    label=Signal::derive(move || match current_lang.get() {
                        Language::PtBr => "DIFICULDADE".to_string(),
                        Language::EnUs => "DIFFICULTY".to_string(),
                    })
                    value=Signal::derive(move || maneuver.difficulty(current_lang.get()).to_string())
                    sub=Signal::derive(move || match current_lang.get() {
                        Language::PtBr => "Dificuldade Base".to_string(),
                        Language::EnUs => "Base Difficulty".to_string(),
                    })
                    color_class="stat-diff"
                />

                <StatBox
                    label=Signal::derive(move || match current_lang.get() {
                        Language::PtBr => "DANO / EFEITO".to_string(),
                        Language::EnUs => "DAMAGE / EFFECT".to_string(),
                    })
                    value=Signal::derive(move || maneuver.damage(current_lang.get()).to_string())
                    sub=Signal::derive(move || match current_lang.get() {
                        Language::PtBr => "Dano Causado".to_string(),
                        Language::EnUs => "Damage Inflicted".to_string(),
                    })
                    color_class="stat-dmg"
                    is_damage=true
                />

                <StatBox
                    label=Signal::derive(move || match current_lang.get() {
                        Language::PtBr => "AÇÕES".to_string(),
                        Language::EnUs => "ACTIONS".to_string(),
                    })
                    value=maneuver.actions.to_string()
                    sub=Signal::derive(move || match (maneuver.actions, current_lang.get()) {
                        (1, Language::PtBr) => "1 Turno / Ação".to_string(),
                        (1, Language::EnUs) => "1 Turn / Action".to_string(),
                        (_, Language::PtBr) => "Múltiplas Ações".to_string(),
                        (_, Language::EnUs) => "Multiple Actions".to_string(),
                    })
                    color_class="stat-actions"
                />
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
            {if let Some(cb) = on_sel_footer {
                let m_to_equip = maneuver;
                let on_close_action = on_cls_footer;
                view! {
                    <div class="compendium-footer-actions">
                        <button
                            type="button"
                            class="compendium-action-btn compendium-btn-emerald compendium-btn-large"
                            on:click=move |_| {
                                cb.call((slot_idx, m_to_equip));
                                if let Some(c) = &on_close_action {
                                    c.call(());
                                }
                            }
                        >
                            <span class="compendium-btn-icon">"🥋"</span>
                            <span class="compendium-btn-label">
                                {move || match (slot_idx, current_lang.get()) {
                                    (Some(idx), Language::PtBr) => format!("Equipar Manobra '{}' na Linha #{}", m_to_equip.name(current_lang.get()), idx + 1),
                                    (Some(idx), Language::EnUs) => format!("Equip Maneuver '{}' in Row #{}", m_to_equip.name(current_lang.get()), idx + 1),
                                    (None, Language::PtBr) => format!("Equipar Manobra '{}' na Tabela de Combate", m_to_equip.name(current_lang.get())),
                                    (None, Language::EnUs) => format!("Equip Maneuver '{}' to Combat Table", m_to_equip.name(current_lang.get())),
                                }}
                            </span>
                        </button>
                    </div>
                }.into_view()
            } else {
                view! { <span></span> }.into_view()
            }}
        </div>
    }
}
