use leptos::*;
use crate::components::LabelField;
use crate::state::CharacterData;
use crate::components::mta_sheet::sheet::XpLedgerModal;

#[component]
pub fn Experience() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    let name = "Experiência";
    let value = Signal::derive(move || data.with(|d| d.labels.get(name).cloned().unwrap_or_default()));

    let update_value = move |val: String| {
        set_data.update(|s| {
            s.labels.insert(name.to_string(), val);
        });
    };

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let (show_ledger, set_show_ledger) = create_signal(false);
    let costs = create_memo(move |_| data.with(|d| d.calculate_costs()));
    let can_edit = Signal::derive(move || data.with(|d| d.can_edit));

    let total_awarded = Signal::derive(move || data.with(|d| d.total_xp_awarded()));
    let total_spent = Signal::derive(move || costs.get().total_xp_spent);
    let balance = Signal::derive(move || total_awarded.get() - total_spent.get());
    let has_ledger_entries = Signal::derive(move || data.with(|d| !d.xp_ledger.is_empty()));

    view! {
        <div class="experience-column">
            <div class="experience-header-row">
                <LabelField 
                    label=Signal::derive(move || crate::i18n::tr("experience", lang()).to_string())
                    value=value
                    on_change=update_value
                />
                <button 
                    type="button" 
                    class="xp-ledger-open-btn" 
                    on:click=move |_| set_show_ledger.set(true)
                    title="Abrir Extrato & Histórico de Evolução de XP"
                >
                    "📜 Extrato"
                </button>
            </div>

            {move || if has_ledger_entries.get() || total_spent.get() > 0 {
                let bal = balance.get();
                let is_neg = bal < 0;
                view! {
                    <div class="xp-quick-badge" on:click=move |_| set_show_ledger.set(true)>
                        <span class="xp-badge-item" class:negative=is_neg>
                            "🪙 Saldo: " <strong>{bal} " XP"</strong>
                        </span>
                        <span class="xp-badge-sep">"•"</span>
                        <span class="xp-badge-sub">
                            "Ganho: " {move || total_awarded.get()} " | Gasto: " {move || total_spent.get()}
                        </span>
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}

            <XpLedgerModal 
                is_open=show_ledger
                set_is_open=set_show_ledger
                data=data
                set_data=set_data
                costs=costs
                can_edit=can_edit
            />
        </div>
    }
}
