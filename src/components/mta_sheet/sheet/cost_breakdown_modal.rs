use leptos::*;
use crate::state::{CharacterData, CostSummary};

#[component]
pub fn CostBreakdownModal(
    costs: Memo<CostSummary>,
    show_breakdown: ReadSignal<bool>,
    set_show_breakdown: WriteSignal<bool>,
    set_data: WriteSignal<CharacterData>,
) -> impl IntoView {
    view! {
        {move || if show_breakdown.get() {
            let current_costs = costs.get();
            let remaining_bonus = 15 - current_costs.total_bonus_spent;
            let aff_sphere = current_costs.affinity_sphere.clone().unwrap_or_default();

            view! {
                <div class="modal-overlay" on:click=move |_| set_show_breakdown.set(false)>
                    <div class="modal-card cost-modal" on:click=move |ev| ev.stop_propagation()>
                        <div class="modal-header">
                            <div class="modal-title-group">
                                <h2 class="modal-title">"Extrato de Gastos da Ficha"</h2>
                                <span class="modal-subtitle">"Cálculo de Pontos de Bônus (15 Iniciais) e Experiência (XP)"</span>
                            </div>
                            <button class="modal-close-btn" on:click=move |_| set_show_breakdown.set(false)>"✕"</button>
                        </div>

                        {if current_costs.arete_warning {
                            view! {
                                <div class="cost-alert-banner">
                                    <span class="alert-icon">"⚠️"</span>
                                    <div>
                                        <strong>"Aviso de Regras: "</strong>
                                        "Na criação de personagem, a Arete inicial não deve ultrapassar 3 pontos (Atualmente: " {current_costs.arete_total} ")."
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
                                    <span class="metric-title">"Pontos de Bônus (Freebies)"</span>
                                </div>
                                <div class="metric-value">
                                    {current_costs.total_bonus_spent} <span class="metric-total">"/ 15 pts"</span>
                                </div>
                                <div class="metric-footer">
                                    {if remaining_bonus >= 0 {
                                        format!("Restante para gastar: {} pts", remaining_bonus)
                                    } else {
                                        format!("Excedente não permitido: {} pts!", remaining_bonus.abs())
                                    }}
                                </div>
                            </div>

                            <div class="cost-metric-card xp-card">
                                <div class="metric-header">
                                    <span class="metric-icon">"🟢"</span>
                                    <span class="metric-title">"Experiência (XP)"</span>
                                </div>
                                <div class="metric-value">
                                    {current_costs.total_xp_spent} <span class="metric-total">"XP gastos"</span>
                                </div>
                                <div class="metric-footer">
                                    "Cálculo conforme tabelas oficiais"
                                </div>
                            </div>
                        </div>

                        <div class="affinity-sphere-selector-box">
                            <label class="affinity-label">
                                <span class="affinity-star">"⭐"</span>
                                "Esfera de Afinidade da Tradição (Custo XP: Atual × 7):"
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
                                <option value="" selected=aff_sphere.is_empty()>"-- Nenhuma selecionada (Todas Atual × 8) --"</option>
                                {crate::state::STANDARD_SPHERES.iter().map(|&sph| {
                                    let is_sel = aff_sphere.eq_ignore_ascii_case(sph);
                                    view! {
                                        <option value=sph selected=is_sel>{sph}</option>
                                    }
                                }).collect_view()}
                            </select>
                        </div>

                        <div class="breakdown-table-wrapper">
                            <table class="breakdown-table">
                                <thead>
                                    <tr>
                                        <th>"Característica"</th>
                                        <th>"Categoria"</th>
                                        <th class="text-center">"Nível"</th>
                                        <th class="text-center">"Bônus (Dots)"</th>
                                        <th class="text-right">"Custo Bônus"</th>
                                        <th class="text-center">"XP (Dots)"</th>
                                        <th class="text-right">"Custo XP"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {if current_costs.items.is_empty() {
                                        view! {
                                            <tr>
                                                <td colspan="7" class="empty-table-msg">
                                                    "Nenhum ponto de Bônus ou XP comprado ainda. Use os modos na barra superior para marcar bolinhas!"
                                                </td>
                                            </tr>
                                        }.into_view()
                                    } else {
                                        current_costs.items.iter().map(|item| {
                                            view! {
                                                <tr>
                                                    <td class="font-bold">{&item.name}</td>
                                                    <td><span class="badge-cat">{&item.category}</span></td>
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
                                        <td colspan="4" class="font-bold">"TOTAIS ACUMULADOS"</td>
                                        <td class="text-right font-bold text-purple">{format!("{} pts", current_costs.total_bonus_spent)}</td>
                                        <td></td>
                                        <td class="text-right font-bold text-green">{format!("{} XP", current_costs.total_xp_spent)}</td>
                                    </tr>
                                </tfoot>
                            </table>
                        </div>
                    </div>
                </div>
            }.into_view()
        } else {
            view! { <div></div> }.into_view()
        }}
    }
}
