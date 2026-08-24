use leptos::*;
use leptos_router::A;
use wasm_bindgen::JsCast;
use crate::state::{CharacterData, CostSummary, DotOrigin};

#[derive(Clone, PartialEq, Debug)]
pub enum SaveStatus {
    Idle,
    Pending,
    Saving,
    Saved(&'static str),
    Error(String),
}

#[component]
pub fn SheetTopBar(
    active_origin: ReadSignal<DotOrigin>,
    set_active_origin: WriteSignal<DotOrigin>,
    costs: Memo<CostSummary>,
    set_show_breakdown: WriteSignal<bool>,
    save_status: ReadSignal<SaveStatus>,
    is_public: Signal<bool>,
    on_toggle_privacy: Callback<()>,
    on_back_click: Callback<ev::MouseEvent>,
    do_manual_save: Callback<ev::MouseEvent>,
    on_export_json: Callback<()>,
    on_import_json: Callback<CharacterData>,
) -> impl IntoView {
    let import_input_ref = create_node_ref::<html::Input>();

    let on_file_change = move |ev: ev::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Some(file_list) = target.files() {
            if let Some(file) = file_list.get(0) {
                let on_import = on_import_json;
                let file_reader = web_sys::FileReader::new().ok();
                if let Some(fr) = file_reader {
                    let fr_clone = fr.clone();
                    let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::ProgressEvent| {
                        if let Ok(result) = fr_clone.result() {
                            if let Some(text) = result.as_string() {
                                match crate::components::common::parse_and_sanitize_sheet_json(&text) {
                                    Ok(parsed_data) => {
                                        on_import.call(parsed_data);
                                    }
                                    Err(err) => {
                                        if let Some(w) = web_sys::window() {
                                            let _ = w.alert_with_message(&format!("Erro ao importar arquivo JSON: {}", err));
                                        }
                                    }
                                }
                            }
                        }
                    }) as Box<dyn FnMut(_)>);

                    fr.set_onload(Some(onload.as_ref().unchecked_ref()));
                    onload.forget();
                    let _ = fr.read_as_text(&file);
                }
            }
        }
        target.set_value("");
    };

    view! {
        <header class="sheet-top-bar">
            <input 
                type="file" 
                accept=".json,application/json" 
                node_ref=import_input_ref 
                style="display: none;" 
                on:change=on_file_change 
            />

            <div class="top-bar-left">
                <a href="/" class="back-link" on:click=move |ev| on_back_click.call(ev)>"← Início"</a>
                <A href="/logs" class="back-link logs-nav-link">"📊 Logs"</A>
            </div>

            <div class="top-bar-center">
                <div class="mode-selector-container">
                    <span class="mode-label">"Modo:"</span>
                    <div class="mode-btn-group">
                        <button 
                            class="mode-btn mode-base"
                            class:active=move || active_origin.get() == DotOrigin::Base
                            on:click=move |_| set_active_origin.set(DotOrigin::Base)
                            title="Criação Base de Personagem (Preto)"
                        >
                            <span class="mode-dot-icon dot-base"></span>
                            "Criação"
                        </button>
                        <button 
                            class="mode-btn mode-bonus"
                            class:active=move || active_origin.get() == DotOrigin::Bonus
                            class=("limit-exceeded", move || costs.get().total_bonus_spent > 15)
                            on:click=move |_| set_active_origin.set(DotOrigin::Bonus)
                            title="Pontos de Bônus Iniciais (Roxo - 5 Atrib, 2 Hab, 7 Esfera, 4 Arete, 1 Antecedente/FV)"
                        >
                            <span class="mode-dot-icon dot-bonus"></span>
                            "Bônus (" {move || costs.get().total_bonus_spent} "/15 pts)"
                        </button>
                        <button 
                            class="mode-btn mode-xp"
                            class:active=move || active_origin.get() == DotOrigin::Experience
                            on:click=move |_| set_active_origin.set(DotOrigin::Experience)
                            title="Experiência / XP Acumulado (Verde)"
                        >
                            <span class="mode-dot-icon dot-xp"></span>
                            "XP (" {move || costs.get().total_xp_spent} " pts)"
                        </button>
                        <button 
                            class="mode-btn mode-temp"
                            class:active=move || active_origin.get() == DotOrigin::Temporary
                            on:click=move |_| set_active_origin.set(DotOrigin::Temporary)
                            title="Bônus Temporário / Feitiço / Wonder (Dourado)"
                        >
                            <span class="mode-dot-icon dot-temp"></span>
                            "Buff / Magia"
                        </button>
                    </div>

                    <button 
                        class="cost-breakdown-toggle-btn"
                        on:click=move |_| set_show_breakdown.set(true)
                        title="Abrir Extrato Completo de Custos e Gastos"
                    >
                        "📊 Extrato"
                    </button>
                </div>
            </div>

            <div class="top-bar-right">
                <div class="save-status-container">
                    {move || match save_status.get() {
                        SaveStatus::Idle => view! { <span class="status-badge status-idle"></span> }.into_view(),
                        SaveStatus::Pending => view! {
                            <span class="status-badge status-pending" title="Alterações pendentes...">
                                <span class="status-dot dot-pending"></span>
                                "Pendente"
                            </span>
                        }.into_view(),
                        SaveStatus::Saving => view! {
                            <span class="status-badge status-saving" title="Gravando dados no banco...">
                                <span class="status-spinner"></span>
                                "Salvando..."
                            </span>
                        }.into_view(),
                        SaveStatus::Saved(t) => view! {
                            <span class="status-badge status-saved" title="Todas as alterações foram salvas">
                                <span class="status-dot dot-saved"></span>
                                {format!("Salvo ({})", t)}
                            </span>
                        }.into_view(),
                        SaveStatus::Error(err) => {
                            let err_title = err.clone();
                            view! {
                                <span class="status-badge status-error" title=err_title>
                                    <span class="status-dot dot-error"></span>
                                    "Erro ao salvar"
                                </span>
                             }.into_view()
                        },
                    }}
                    <button class="manual-save-btn" on:click=move |ev| do_manual_save.call(ev) title="Salvar imediatamente">
                        "💾 Salvar"
                    </button>

                    <button 
                        type="button" 
                        class="json-top-btn json-import-btn"
                        on:click=move |_| {
                            if let Some(input) = import_input_ref.get() {
                                input.click();
                            }
                        }
                        title="Importar dados de um arquivo .json"
                    >
                        "📥 JSON"
                    </button>

                    <button 
                        type="button" 
                        class="json-top-btn json-export-btn"
                        on:click=move |_| on_export_json.call(())
                        title="Exportar e baixar esta ficha em arquivo .json"
                    >
                        "📤 JSON"
                    </button>

                    <button 
                        type="button" 
                        class=move || if is_public.get() { "privacy-toggle-top-btn btn-public" } else { "privacy-toggle-top-btn btn-private" }
                        on:click=move |_| on_toggle_privacy.call(())
                        title=move || if is_public.get() { "Ficha Pública na comunidade. Clique para tornar Privada." } else { "Ficha Privada. Clique para tornar Pública na comunidade." }
                    >
                        {move || if is_public.get() { "🌐 Pública" } else { "🔒 Privada" }}
                    </button>

                    <button 
                        type="button" 
                        class="export-pdf-btn" 
                        on:click=move |_| {
                            if let Some(w) = web_sys::window() {
                                let _ = w.print();
                            }
                        } 
                        title="Exportar Ficha em PDF Oficial (A4)"
                    >
                        "🖨️ PDF"
                    </button>
                </div>
            </div>
        </header>
    }
}
