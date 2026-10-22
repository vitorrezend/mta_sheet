use leptos::*;
use crate::state::{CharacterData, CostSummary, XpLedgerEntry};

#[component]
pub fn XpLedgerModal(
    is_open: ReadSignal<bool>,
    set_is_open: WriteSignal<bool>,
    data: ReadSignal<CharacterData>,
    set_data: WriteSignal<CharacterData>,
    costs: Memo<CostSummary>,
    #[prop(into, default = true.into())] can_edit: MaybeSignal<bool>,
) -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let (new_desc, set_new_desc) = create_signal(String::new());
    let (new_amount, set_new_amount) = create_signal(3i32);
    let (new_date, set_new_date) = create_signal(String::new());

    let entries_sig = Signal::derive(move || data.with(|d| d.xp_ledger.clone()));

    let total_awarded_sig = Signal::derive(move || data.with(|d| d.total_xp_awarded()));
    let total_spent_sig = Signal::derive(move || costs.get().total_xp_spent);
    let balance_sig = Signal::derive(move || total_awarded_sig.get() - total_spent_sig.get());

    let on_add_entry = move |_| {
        let desc = new_desc.get().trim().to_string();
        if desc.is_empty() {
            return;
        }
        let amount = new_amount.get().max(1);
        let date = if new_date.get().trim().is_empty() {
            None
        } else {
            Some(new_date.get().trim().to_string())
        };

        set_data.update(|d| {
            d.add_xp_entry(desc, amount, date);
        });

        set_new_desc.set(String::new());
        set_new_amount.set(3);
        set_new_date.set(String::new());
    };

    view! {
        {move || if is_open.get() {
            let current_lang = lang();
            let is_pt = current_lang == crate::i18n::Language::PtBr;

            view! {
                <div class="modal-overlay" on:click=move |_| set_is_open.set(false)>
                    <div class="modal-card xp-ledger-modal" on:click=move |ev| ev.stop_propagation()>
                        <div class="modal-header">
                            <div class="modal-title-group">
                                <h2 class="modal-title">
                                    {if is_pt { "📜 Extrato & Histórico de Evolução de XP" } else { "📜 XP Ledger & Progression History" }}
                                </h2>
                                <span class="modal-subtitle">
                                    {if is_pt {
                                        "Contabilidade de pontos de experiência concedidos pelo Narrador e auditoria de gastos"
                                    } else {
                                        "Accounting of experience points awarded by the Storyteller and expenditure audit"
                                    }}
                                </span>
                            </div>
                            <button class="modal-close-btn" on:click=move |_| set_is_open.set(false)>"✕"</button>
                        </div>

                        // ==========================================
                        // CARDS DE RESUMO / MÉTRICAS
                        // ==========================================
                        <div class="xp-metrics-grid">
                            <div class="xp-metric-card awarded-card">
                                <span class="metric-icon">"🟢"</span>
                                <div class="metric-info">
                                    <span class="metric-label">
                                        {if is_pt { "Total Concedido" } else { "Total Awarded" }}
                                    </span>
                                    <span class="metric-value">{move || total_awarded_sig.get()} <span class="metric-unit">"XP"</span></span>
                                </div>
                            </div>

                            <div class="xp-metric-card spent-card">
                                <span class="metric-icon">"🟣"</span>
                                <div class="metric-info">
                                    <span class="metric-label">
                                        {if is_pt { "Investido na Ficha" } else { "Spent on Sheet" }}
                                    </span>
                                    <span class="metric-value">{move || total_spent_sig.get()} <span class="metric-unit">"XP"</span></span>
                                </div>
                            </div>

                            <div 
                                class="xp-metric-card balance-card"
                                class:negative=move || balance_sig.get() < 0
                            >
                                <span class="metric-icon">"🪙"</span>
                                <div class="metric-info">
                                    <span class="metric-label">
                                        {if is_pt { "Saldo Disponível" } else { "Available Balance" }}
                                    </span>
                                    <span class="metric-value">{move || balance_sig.get()} <span class="metric-unit">"XP"</span></span>
                                </div>
                            </div>
                        </div>

                        {move || if balance_sig.get() < 0 {
                            view! {
                                <div class="cost-alert-banner">
                                    <span class="alert-icon">"⚠️"</span>
                                    <div class="alert-content">
                                        <strong>
                                            {if is_pt { "Aviso de Saldo Negativo de XP:" } else { "Negative XP Balance Warning:" }}
                                        </strong>
                                        <p>
                                            {if is_pt {
                                                "O personagem possui mais pontos investidos na ficha do que o total registrado de XP concedido."
                                            } else {
                                                "The character has more points invested on the sheet than the recorded total awarded XP."
                                            }}
                                        </p>
                                    </div>
                                </div>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }}

                        // ==========================================
                        // FORMULÁRIO DE CONCESSÃO (NOVO REGISTRO)
                        // ==========================================
                        {move || if can_edit.get() {
                            view! {
                                <div class="xp-award-form">
                                    <h4 class="form-title">
                                        {if is_pt { "➕ Registrar Novo XP Concedido" } else { "➕ Record New Awarded XP" }}
                                    </h4>
                                    <div class="form-fields-row">
                                        <div class="form-field date-field">
                                            <label>{if is_pt { "Data / Sessão" } else { "Date / Session" }}</label>
                                            <input 
                                                type="text" 
                                                placeholder={if is_pt { "Ex: Sessão 3 ou 2026-10-22" } else { "Ex: Session 3" }}
                                                prop:value=move || new_date.get()
                                                on:input=move |ev| set_new_date.set(event_target_value(&ev))
                                            />
                                        </div>
                                        <div class="form-field desc-field">
                                            <label>{if is_pt { "Motivo / Descrição" } else { "Reason / Description" }}</label>
                                            <input 
                                                type="text" 
                                                placeholder={if is_pt { "Ex: Vitória contra o Culto ou Boa Interpretação" } else { "Ex: Good Roleplay" }}
                                                prop:value=move || new_desc.get()
                                                on:input=move |ev| set_new_desc.set(event_target_value(&ev))
                                            />
                                        </div>
                                        <div class="form-field amount-field">
                                            <label>{if is_pt { "Quantidade" } else { "Amount" }}</label>
                                            <input 
                                                type="number" 
                                                min="1"
                                                max="100"
                                                prop:value=move || new_amount.get().to_string()
                                                on:input=move |ev| {
                                                    if let Ok(v) = event_target_value(&ev).parse::<i32>() {
                                                        set_new_amount.set(v);
                                                    }
                                                }
                                            />
                                        </div>
                                        <button 
                                            type="button" 
                                            class="add-xp-btn"
                                            on:click=on_add_entry
                                            disabled=move || new_desc.get().trim().is_empty()
                                        >
                                            {if is_pt { "+ Conceder" } else { "+ Award" }}
                                        </button>
                                    </div>
                                </div>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }}

                        // ==========================================
                        // TABELA / FEED DO HISTÓRICO DE XP
                        // ==========================================
                        <div class="xp-ledger-table-container">
                            <h4 class="table-section-title">
                                {if is_pt { "📋 Linha do Tempo de Concessões" } else { "📋 Awards Timeline" }}
                            </h4>
                            <div class="xp-table-wrapper">
                                <table class="breakdown-table xp-ledger-table">
                                    <thead>
                                        <tr>
                                            <th>{if is_pt { "Data / Sessão" } else { "Date / Session" }}</th>
                                            <th>{if is_pt { "Descrição / Motivo" } else { "Description / Reason" }}</th>
                                            <th class="text-center">{if is_pt { "Pontos" } else { "Points" }}</th>
                                            {move || if can_edit.get() {
                                                view! { <th class="text-right">{if is_pt { "Ações" } else { "Actions" }}</th> }.into_view()
                                            } else {
                                                view! {}.into_view()
                                            }}
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {move || {
                                            let entries = entries_sig.get();
                                            if entries.is_empty() {
                                                view! {
                                                    <tr>
                                                        <td colspan="4" class="empty-table-msg">
                                                            {if is_pt {
                                                                "Nenhuma concessão de XP registrada ainda. Use o formulário acima para adicionar!"
                                                            } else {
                                                                "No awarded XP recorded yet. Use the form above to add an entry!"
                                                            }}
                                                        </td>
                                                    </tr>
                                                }.into_view()
                                            } else {
                                                view! {
                                                    <For
                                                        each=move || entries_sig.get()
                                                        key=|entry| entry.id.clone()
                                                        children=move |entry: XpLedgerEntry| {
                                                            let entry_id = entry.id.clone();
                                                            let can_edit_val = can_edit.get();
                                                            view! {
                                                                <tr>
                                                                    <td class="xp-date-col">{entry.date}</td>
                                                                    <td class="xp-desc-col">{entry.description}</td>
                                                                    <td class="text-center">
                                                                        <span class="xp-amount-pill">
                                                                            "+" {entry.amount} " XP"
                                                                        </span>
                                                                    </td>
                                                                    {if can_edit_val {
                                                                        let id_for_del = entry_id.clone();
                                                                        view! {
                                                                            <td class="text-right">
                                                                                <button 
                                                                                    type="button" 
                                                                                    class="delete-xp-entry-btn" 
                                                                                    title={if is_pt { "Excluir lançamento" } else { "Delete entry" }}
                                                                                    on:click=move |_| {
                                                                                        let id_to_remove = id_for_del.clone();
                                                                                        set_data.update(|d| {
                                                                                            d.remove_xp_entry(&id_to_remove);
                                                                                        });
                                                                                    }
                                                                                >
                                                                                    "🗑️"
                                                                                </button>
                                                                            </td>
                                                                        }.into_view()
                                                                    } else {
                                                                        view! {}.into_view()
                                                                    }}
                                                                </tr>
                                                            }
                                                        }
                                                    />
                                                }.into_view()
                                            }
                                        }}
                                    </tbody>
                                </table>
                            </div>
                        </div>

                    </div>
                </div>
            }.into_view()
        } else {
            view! {}.into_view()
        }}
    }
}
