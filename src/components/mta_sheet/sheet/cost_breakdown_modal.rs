use leptos::*;
use crate::state::{CharacterData, CostSummary};

#[component]
pub fn CostBreakdownModal(
    costs: Memo<CostSummary>,
    show_breakdown: ReadSignal<bool>,
    set_show_breakdown: WriteSignal<bool>,
    set_data: WriteSignal<CharacterData>,
) -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();
    let (active_tab, set_active_tab) = create_signal(0); // 0 = Criação Base, 1 = Bônus & XP

    view! {
        {move || if show_breakdown.get() {
            let current_costs = costs.get();
            let creation = current_costs.creation_points.clone();
            let remaining_bonus = 15 - current_costs.total_bonus_spent;
            let aff_sphere = current_costs.affinity_sphere.clone().unwrap_or_default();
            let tab = active_tab.get();
            let current_lang = lang();

            view! {
                <div class="modal-overlay" on:click=move |_| set_show_breakdown.set(false)>
                    <div class="modal-card cost-modal" on:click=move |ev| ev.stop_propagation()>
                        <div class="modal-header">
                            <div class="modal-title-group">
                                <h2 class="modal-title">{match current_lang {
                                    crate::i18n::Language::PtBr => "Contabilidade & Extrato da Ficha",
                                    crate::i18n::Language::EnUs => "Sheet Audit & Cost Breakdown",
                                }}</h2>
                                <span class="modal-subtitle">{match current_lang {
                                    crate::i18n::Language::PtBr => "Auditoria de Pontos de Criação Base (M20), Pontos de Bônus (15) e XP",
                                    crate::i18n::Language::EnUs => "Audit of Base Creation Points (M20), Freebie Points (15), and XP",
                                }}</span>
                            </div>
                            <button class="modal-close-btn" on:click=move |_| set_show_breakdown.set(false)>"✕"</button>
                        </div>

                        <div class="cost-tab-bar">
                            <button 
                                type="button"
                                class="cost-tab-btn"
                                class:active=move || tab == 0
                                on:click=move |_| set_active_tab.set(0)
                            >
                                {match current_lang {
                                    crate::i18n::Language::PtBr => "🎯 Pontos de Criação Base",
                                    crate::i18n::Language::EnUs => "🎯 Base Creation Points",
                                }}
                                {if creation.has_any_overflow {
                                    view! { <span class="tab-badge-alert">{match current_lang {
                                        crate::i18n::Language::PtBr => "⚠️ Limite",
                                        crate::i18n::Language::EnUs => "⚠️ Overflow",
                                    }}</span> }.into_view()
                                } else {
                                    view! { <span class="tab-badge-ok">"✓ OK"</span> }.into_view()
                                }}
                            </button>
                            <button 
                                type="button"
                                class="cost-tab-btn"
                                class:active=move || tab == 1
                                on:click=move |_| set_active_tab.set(1)
                            >
                                {match current_lang {
                                    crate::i18n::Language::PtBr => "🟣 Pontos de Bônus & XP",
                                    crate::i18n::Language::EnUs => "🟣 Freebies & Experience",
                                }}
                                {if current_costs.total_bonus_spent > 15 {
                                    view! { <span class="tab-badge-alert">"⚠️ " {current_costs.total_bonus_spent} "/15"</span> }.into_view()
                                } else {
                                    view! { <span class="tab-badge-neutral">{current_costs.total_bonus_spent} "/15 pts"</span> }.into_view()
                                }}
                            </button>
                        </div>

                        {if tab == 0 {
                            // ==========================================
                            // ABA 0: PONTOS DE CRIAÇÃO BASE
                            // ==========================================
                            view! {
                                <div class="creation-tab-content">
                                    {if creation.has_any_overflow {
                                        view! {
                                            <div class="cost-alert-banner">
                                                <span class="alert-icon">"⚠️"</span>
                                                <div class="alert-content">
                                                    <strong>{match current_lang {
                                                        crate::i18n::Language::PtBr => "Avisos de Criação de Personagem (Regras M20):",
                                                        crate::i18n::Language::EnUs => "Character Creation Rule Warnings (M20):",
                                                    }}</strong>
                                                    <ul class="creation-warnings-list">
                                                        {creation.warnings.iter().map(|w| {
                                                            view! { <li>{w}</li> }
                                                        }).collect_view()}
                                                    </ul>
                                                </div>
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! {
                                            <div class="cost-success-banner">
                                                <span class="alert-icon">"✅"</span>
                                                <div>
                                                    <strong>{match current_lang {
                                                        crate::i18n::Language::PtBr => "Distribuição Conforme as Regras Oficiais:",
                                                        crate::i18n::Language::EnUs => "Standard Rules Compliance:",
                                                    }}</strong>
                                                    {match current_lang {
                                                        crate::i18n::Language::PtBr => " Todos os pontos de criação base estão perfeitamente distribuídos dentro dos orçamentos do M20.",
                                                        crate::i18n::Language::EnUs => " All base creation points are accurately distributed within M20 allowances.",
                                                    }}
                                                </div>
                                            </div>
                                        }.into_view()
                                    }}

                                    <div class="creation-cards-grid">
                                        // 1. Atributos (7 / 5 / 3)
                                        <div class="cost-metric-card" class:limit-alert=creation.attr_exceeded>
                                            <div class="metric-header">
                                                <span class="metric-icon">"🛡️"</span>
                                                <span class="metric-title">{match current_lang {
                                                    crate::i18n::Language::PtBr => "Atributos (7 / 5 / 3)",
                                                    crate::i18n::Language::EnUs => "Attributes (7 / 5 / 3)",
                                                }}</span>
                                            </div>
                                            <div class="metric-value">
                                                {creation.attr_total_spent} <span class="metric-total">{match current_lang {
                                                    crate::i18n::Language::PtBr => "/ 15 pts gastos",
                                                    crate::i18n::Language::EnUs => "/ 15 pts spent",
                                                }}</span>
                                            </div>
                                            <div class="metric-subdetail">
                                                {match current_lang {
                                                    crate::i18n::Language::PtBr => format!("Físicos: {} | Sociais: {} | Mentais: {}", creation.attr_physical, creation.attr_social, creation.attr_mental),
                                                    crate::i18n::Language::EnUs => format!("Physical: {} | Social: {} | Mental: {}", creation.attr_physical, creation.attr_social, creation.attr_mental),
                                                }}
                                            </div>
                                            <div class="metric-footer" class:text-alert=creation.attr_exceeded>
                                                {if creation.attr_spread_valid && creation.attr_total_spent <= 15 {
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => "✓ Alocação válida (1º ponto grátis descontado)".to_string(),
                                                        crate::i18n::Language::EnUs => "✓ Valid allocation (1st free dot accounted)".to_string(),
                                                    }
                                                } else {
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => "⚠️ Distribuição fora das faixas 7 / 5 / 3".to_string(),
                                                        crate::i18n::Language::EnUs => "⚠️ Distribution outside 7 / 5 / 3 brackets".to_string(),
                                                    }
                                                }}
                                            </div>
                                        </div>

                                        // 2. Habilidades (13 / 9 / 5, max 3)
                                        <div class="cost-metric-card" class:limit-alert=creation.ab_exceeded>
                                            <div class="metric-header">
                                                <span class="metric-icon">"⚔️"</span>
                                                <span class="metric-title">{match current_lang {
                                                    crate::i18n::Language::PtBr => "Habilidades (13 / 9 / 5)",
                                                    crate::i18n::Language::EnUs => "Abilities (13 / 9 / 5)",
                                                }}</span>
                                            </div>
                                            <div class="metric-value">
                                                {creation.ab_total_spent} <span class="metric-total">{match current_lang {
                                                    crate::i18n::Language::PtBr => "/ 27 pts gastos",
                                                    crate::i18n::Language::EnUs => "/ 27 pts spent",
                                                }}</span>
                                            </div>
                                            <div class="metric-subdetail">
                                                {match current_lang {
                                                    crate::i18n::Language::PtBr => format!("Talentos: {} | Perícias: {} | Conhecimentos: {}", creation.ab_talents, creation.ab_skills, creation.ab_knowledges),
                                                    crate::i18n::Language::EnUs => format!("Talents: {} | Skills: {} | Knowledges: {}", creation.ab_talents, creation.ab_skills, creation.ab_knowledges),
                                                }}
                                            </div>
                                            <div class="metric-footer" class:text-alert=creation.ab_exceeded>
                                                {if !creation.ab_cap_violations.is_empty() {
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => format!("⚠️ {} habilidade(s) com > 3 pts base", creation.ab_cap_violations.len()),
                                                        crate::i18n::Language::EnUs => format!("⚠️ {} ability(ies) with > 3 base dots", creation.ab_cap_violations.len()),
                                                    }
                                                } else if creation.ab_spread_valid && creation.ab_total_spent <= 27 {
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => "✓ Alocação válida (Máximo 3 pts por habilidade)".to_string(),
                                                        crate::i18n::Language::EnUs => "✓ Valid allocation (Max 3 dots per ability)".to_string(),
                                                    }
                                                } else {
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => "⚠️ Distribuição fora das faixas 13 / 9 / 5".to_string(),
                                                        crate::i18n::Language::EnUs => "⚠️ Distribution outside 13 / 9 / 5 brackets".to_string(),
                                                    }
                                                }}
                                            </div>
                                        </div>

                                        // 3. Esferas (6 pts)
                                        <div class="cost-metric-card" class:limit-alert=creation.spheres_exceeded>
                                            <div class="metric-header">
                                                <span class="metric-icon">"🔮"</span>
                                                <span class="metric-title">{match current_lang {
                                                    crate::i18n::Language::PtBr => "Esferas de Magia",
                                                    crate::i18n::Language::EnUs => "Magick Spheres",
                                                }}</span>
                                            </div>
                                            <div class="metric-value">
                                                {creation.spheres_spent} <span class="metric-total">{match current_lang {
                                                    crate::i18n::Language::PtBr => "/ 6 pts gastos",
                                                    crate::i18n::Language::EnUs => "/ 6 pts spent",
                                                }}</span>
                                            </div>
                                            <div class="metric-subdetail">
                                                {if aff_sphere.is_empty() {
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => "Nenhuma Esfera de Afinidade selecionada".to_string(),
                                                        crate::i18n::Language::EnUs => "No Affinity Sphere selected".to_string(),
                                                    }
                                                } else {
                                                    let sph_name = crate::i18n::tr_sphere(&aff_sphere, current_lang);
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => format!("1 pt grátis em {} (Afinidade)", sph_name),
                                                        crate::i18n::Language::EnUs => format!("1 free dot in {} (Affinity)", sph_name),
                                                    }
                                                }}
                                            </div>
                                            <div class="metric-footer" class:text-alert=creation.spheres_exceeded>
                                                {if creation.spheres_spent <= 6 {
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => "✓ Dentro do orçamento de 6 pontos".to_string(),
                                                        crate::i18n::Language::EnUs => "✓ Within 6 points budget".to_string(),
                                                    }
                                                } else {
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => format!("⚠️ Excedente: {} pts de criação", creation.spheres_spent - 6),
                                                        crate::i18n::Language::EnUs => format!("⚠️ Overflow: {} creation pts", creation.spheres_spent - 6),
                                                    }
                                                }}
                                            </div>
                                        </div>

                                        // 4. Antecedentes (7 pts)
                                        <div class="cost-metric-card" class:limit-alert=creation.backgrounds_exceeded>
                                            <div class="metric-header">
                                                <span class="metric-icon">"📜"</span>
                                                <span class="metric-title">{crate::i18n::tr("backgrounds", current_lang)}</span>
                                            </div>
                                            <div class="metric-value">
                                                {creation.backgrounds_spent} <span class="metric-total">{match current_lang {
                                                    crate::i18n::Language::PtBr => "/ 7 pts gastos",
                                                    crate::i18n::Language::EnUs => "/ 7 pts spent",
                                                }}</span>
                                            </div>
                                            <div class="metric-subdetail">
                                                {match current_lang {
                                                    crate::i18n::Language::PtBr => "Aliados, Mentor, Recursos, Avatar, Nodo, etc.",
                                                    crate::i18n::Language::EnUs => "Allies, Mentor, Resources, Avatar, Node, etc.",
                                                }}
                                            </div>
                                            <div class="metric-footer" class:text-alert=creation.backgrounds_exceeded>
                                                {if creation.backgrounds_spent <= 7 {
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => "✓ Dentro do orçamento de 7 pontos".to_string(),
                                                        crate::i18n::Language::EnUs => "✓ Within 7 points budget".to_string(),
                                                    }
                                                } else {
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => format!("⚠️ Excedente: {} pts de criação", creation.backgrounds_spent - 7),
                                                        crate::i18n::Language::EnUs => format!("⚠️ Overflow: {} creation pts", creation.backgrounds_spent - 7),
                                                    }
                                                }}
                                            </div>
                                        </div>

                                        // 5. Ressonância (1 pt)
                                        <div class="cost-metric-card" class:limit-alert=creation.resonance_exceeded>
                                            <div class="metric-header">
                                                <span class="metric-icon">"🌀"</span>
                                                <span class="metric-title">{crate::i18n::tr("resonance", current_lang)}</span>
                                            </div>
                                            <div class="metric-value">
                                                {creation.resonance_spent} <span class="metric-total">{match current_lang {
                                                    crate::i18n::Language::PtBr => "/ 1 pt gasto",
                                                    crate::i18n::Language::EnUs => "/ 1 pt spent",
                                                }}</span>
                                            </div>
                                            <div class="metric-subdetail">
                                                {match current_lang {
                                                    crate::i18n::Language::PtBr => "Dinâmica, Entrópica ou Estática",
                                                    crate::i18n::Language::EnUs => "Dynamic, Entropic, or Static",
                                                }}
                                            </div>
                                            <div class="metric-footer" class:text-alert=creation.resonance_exceeded>
                                                {if creation.resonance_spent <= 1 {
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => "✓ 1 ponto inicial alocado".to_string(),
                                                        crate::i18n::Language::EnUs => "✓ 1 initial point allocated".to_string(),
                                                    }
                                                } else {
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => "⚠️ Pontos adicionais exigem Pontos de Bônus".to_string(),
                                                        crate::i18n::Language::EnUs => "⚠️ Additional dots require Freebie points".to_string(),
                                                    }
                                                }}
                                            </div>
                                        </div>

                                        // 6. Arete & Força de Vontade
                                        <div class="cost-metric-card" class:limit-alert=creation.arete_exceeded || creation.willpower_exceeded>
                                            <div class="metric-header">
                                                <span class="metric-icon">"🧠"</span>
                                                <span class="metric-title">{match current_lang {
                                                    crate::i18n::Language::PtBr => "Arete & Vontade",
                                                    crate::i18n::Language::EnUs => "Arete & Willpower",
                                                }}</span>
                                            </div>
                                            <div class="metric-value">
                                                "1 & 5" <span class="metric-total">{match current_lang {
                                                    crate::i18n::Language::PtBr => "pts grátis",
                                                    crate::i18n::Language::EnUs => "free dots",
                                                }}</span>
                                            </div>
                                            <div class="metric-subdetail">
                                                {match current_lang {
                                                    crate::i18n::Language::PtBr => format!("Arete base: {} | Vontade base: {}", creation.arete_base, creation.willpower_base),
                                                    crate::i18n::Language::EnUs => format!("Base Arete: {} | Base Willpower: {}", creation.arete_base, creation.willpower_base),
                                                }}
                                            </div>
                                            <div class="metric-footer" class:text-alert=creation.arete_exceeded || creation.willpower_exceeded>
                                                {if !creation.arete_exceeded && !creation.willpower_exceeded {
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => "✓ Começam em Arete 1 e Vontade 5".to_string(),
                                                        crate::i18n::Language::EnUs => "✓ Starts at Arete 1 and Willpower 5".to_string(),
                                                    }
                                                } else {
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => "⚠️ Pontos acima do base exigem Bônus (Arete: 4 pts, FV: 1 pt)".to_string(),
                                                        crate::i18n::Language::EnUs => "⚠️ Dots above base require Freebies (Arete: 4 pts, Willpower: 1 pt)".to_string(),
                                                    }
                                                }}
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }.into_view()
                        } else {
                            // ==========================================
                            // ABA 1: PONTOS DE BÔNUS (FREEBIES) & XP
                            // ==========================================
                            view! {
                                <div class="bonus-xp-tab-content">
                                    {if current_costs.arete_warning {
                                        view! {
                                            <div class="cost-alert-banner">
                                                <span class="alert-icon">"⚠️"</span>
                                                <div>
                                                    <strong>{match current_lang {
                                                        crate::i18n::Language::PtBr => "Aviso de Regras: ",
                                                        crate::i18n::Language::EnUs => "Rules Warning: ",
                                                    }}</strong>
                                                    {match current_lang {
                                                        crate::i18n::Language::PtBr => format!("Na criação de personagem, a Arete inicial não deve ultrapassar 3 pontos (Atualmente: {}).", current_costs.arete_total),
                                                        crate::i18n::Language::EnUs => format!("During character creation, initial Arete should not exceed 3 dots (Currently: {}).", current_costs.arete_total),
                                                    }}
                                                </div>
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! { <div></div> }.into_view()
                                    }}

                                    <div class="cost-summary-grid">
                                        <div class="cost-metric-card bonus-card" class=("limit-alert", move || remaining_bonus < 0)>
                                            <div class="metric-header">
                                                <span class="metric-icon">"🟣"</span>
                                                <span class="metric-title">{match current_lang {
                                                    crate::i18n::Language::PtBr => "Pontos de Bônus (Freebies)",
                                                    crate::i18n::Language::EnUs => "Freebie Points",
                                                }}</span>
                                            </div>
                                            <div class="metric-value">
                                                {current_costs.total_bonus_spent} <span class="metric-total">"/ 15 pts"</span>
                                            </div>
                                            <div class="metric-footer">
                                                {if remaining_bonus >= 0 {
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => format!("Restante para gastar: {} pts", remaining_bonus),
                                                        crate::i18n::Language::EnUs => format!("Remaining to spend: {} pts", remaining_bonus),
                                                    }
                                                } else {
                                                    match current_lang {
                                                        crate::i18n::Language::PtBr => format!("Excedente não permitido: {} pts!", remaining_bonus.abs()),
                                                        crate::i18n::Language::EnUs => format!("Disallowed excess: {} pts!", remaining_bonus.abs()),
                                                    }
                                                }}
                                            </div>
                                        </div>

                                        <div class="cost-metric-card xp-card">
                                            <div class="metric-header">
                                                <span class="metric-icon">"🟢"</span>
                                                <span class="metric-title">{crate::i18n::tr("experience", current_lang)}</span>
                                            </div>
                                            <div class="metric-value">
                                                {current_costs.total_xp_spent} <span class="metric-total">{match current_lang {
                                                    crate::i18n::Language::PtBr => "XP gastos",
                                                    crate::i18n::Language::EnUs => "XP spent",
                                                }}</span>
                                            </div>
                                            <div class="metric-footer">
                                                {match current_lang {
                                                    crate::i18n::Language::PtBr => "Cálculo conforme tabelas oficiais do M20",
                                                    crate::i18n::Language::EnUs => "Calculated per official M20 tables",
                                                }}
                                            </div>
                                        </div>
                                    </div>

                                    <div class="affinity-sphere-selector-box">
                                        <label class="affinity-label">
                                            <span class="affinity-star">"⭐"</span>
                                            {match current_lang {
                                                crate::i18n::Language::PtBr => "Esfera de Afinidade da Tradição (1º ponto grátis e Custo XP: Atual × 7):",
                                                crate::i18n::Language::EnUs => "Tradition Affinity Sphere (1st dot free and XP Cost: Current × 7):",
                                            }}
                                        </label>
                                        <select 
                                            class="affinity-select"
                                            on:change=move |ev| {
                                                let val = event_target_value(&ev);
                                                set_data.update(|s| {
                                                    if val.is_empty() {
                                                        s.set_affinity_sphere(None);
                                                    } else {
                                                        s.set_affinity_sphere(Some(val));
                                                    }
                                                });
                                            }
                                        >
                                            <option value="" selected=aff_sphere.is_empty()>
                                                {match current_lang {
                                                    crate::i18n::Language::PtBr => "-- Nenhuma selecionada (Todas Atual × 8) --",
                                                    crate::i18n::Language::EnUs => "-- None selected (All Current × 8) --",
                                                }}
                                            </option>
                                            {crate::state::STANDARD_SPHERES.iter().map(|&sph| {
                                                let is_sel = aff_sphere.eq_ignore_ascii_case(sph);
                                                let sph_display = crate::i18n::tr_sphere(sph, current_lang);
                                                view! {
                                                    <option value=sph selected=is_sel>{sph_display}</option>
                                                }
                                            }).collect_view()}
                                        </select>
                                    </div>

                                    <div class="breakdown-table-wrapper">
                                        <table class="breakdown-table">
                                            <thead>
                                                <tr>
                                                    <th>{match current_lang { crate::i18n::Language::PtBr => "Característica", crate::i18n::Language::EnUs => "Trait" }}</th>
                                                    <th>{match current_lang { crate::i18n::Language::PtBr => "Categoria", crate::i18n::Language::EnUs => "Category" }}</th>
                                                    <th class="text-center">{match current_lang { crate::i18n::Language::PtBr => "Nível", crate::i18n::Language::EnUs => "Rating" }}</th>
                                                    <th class="text-center">{match current_lang { crate::i18n::Language::PtBr => "Bônus (Dots)", crate::i18n::Language::EnUs => "Freebie (Dots)" }}</th>
                                                    <th class="text-right">{match current_lang { crate::i18n::Language::PtBr => "Custo Bônus", crate::i18n::Language::EnUs => "Freebie Cost" }}</th>
                                                    <th class="text-center">{match current_lang { crate::i18n::Language::PtBr => "XP (Dots)", crate::i18n::Language::EnUs => "XP (Dots)" }}</th>
                                                    <th class="text-right">{match current_lang { crate::i18n::Language::PtBr => "Custo XP", crate::i18n::Language::EnUs => "XP Cost" }}</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {if current_costs.items.is_empty() {
                                                    view! {
                                                        <tr>
                                                            <td colspan="7" class="empty-table-msg">
                                                                {match current_lang {
                                                                    crate::i18n::Language::PtBr => "Nenhum ponto de Bônus ou XP comprado ainda. Use os modos na barra superior para marcar bolinhas!",
                                                                    crate::i18n::Language::EnUs => "No Freebie or XP points spent yet. Switch modes on the top bar to mark dots!",
                                                                }}
                                                            </td>
                                                        </tr>
                                                    }.into_view()
                                                } else {
                                                    current_costs.items.clone().into_iter().map(|item| {
                                                        let item_name = crate::i18n::tr_sphere(&item.name, current_lang).to_string();
                                                        view! {
                                                            <tr>
                                                                <td class="font-bold">{item_name}</td>
                                                                <td><span class="badge-cat">{item.category}</span></td>
                                                                <td class="text-center">{item.level}</td>
                                                                <td class="text-center">{if item.bonus_dots > 0 { format!("{}", item.bonus_dots) } else { "-".to_string() }}</td>
                                                                <td class="text-right font-bold text-purple">{if item.bonus_cost > 0 { format!("{} pts", item.bonus_cost) } else { "-".to_string() }}</td>
                                                                <td class="text-center">{if item.xp_dots > 0 { format!("{}", item.xp_dots) } else { "-".to_string() }}</td>
                                                                <td class="text-right font-bold text-green">{if item.xp_cost > 0 { format!("{} XP", item.xp_cost) } else { "-".to_string() }}</td>
                                                            </tr>
                                                        }
                                                    }).collect_view().into_view()
                                                }}
                                            </tbody>
                                            <tfoot>
                                                <tr class="table-totals-row">
                                                    <td colspan="4" class="font-bold">
                                                        {match current_lang {
                                                            crate::i18n::Language::PtBr => "TOTAIS ACUMULADOS",
                                                            crate::i18n::Language::EnUs => "CUMULATIVE TOTALS",
                                                        }}
                                                    </td>
                                                    <td class="text-right font-bold text-purple">{format!("{} pts", current_costs.total_bonus_spent)}</td>
                                                    <td></td>
                                                    <td class="text-right font-bold text-green">{format!("{} XP", current_costs.total_xp_spent)}</td>
                                                </tr>
                                            </tfoot>
                                        </table>
                                    </div>
                                </div>
                            }.into_view()
                        }}
                    </div>
                </div>
            }.into_view()
        } else {
            view! { <div></div> }.into_view()
        }}
    }
}
