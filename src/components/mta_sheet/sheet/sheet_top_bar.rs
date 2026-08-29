use leptos::*;
use leptos_router::A;
use wasm_bindgen::JsCast;
use crate::components::Callback;
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
    set_show_quiz: WriteSignal<bool>,
    save_status: ReadSignal<SaveStatus>,
    is_public: Signal<bool>,
    on_toggle_privacy: Callback<()>,
    on_back_click: Callback<ev::MouseEvent>,
    do_manual_save: Callback<ev::MouseEvent>,
    on_export_json: Callback<()>,
    on_import_json: Callback<CharacterData>,
    #[prop(optional)] set_show_pdf_modal: Option<WriteSignal<bool>>,
) -> impl IntoView {
    let import_input_ref = create_node_ref::<html::Input>();

    let on_file_change = move |ev: ev::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Some(file_list) = target.files() {
            if let Some(file) = file_list.get(0) {
                let on_import = on_import_json.clone();
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

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    view! {
        <header class="sheet-top-bar">
            <div class="top-bar-left">
                <a href="/" class="back-link" on:click=move |ev| on_back_click.call(ev)>{move || crate::i18n::tr("home", lang())}</a>
                <a href="/logs" class="back-link logs-nav-link">{move || crate::i18n::tr("logs", lang())}</a>
            </div>

            <div class="top-bar-center">
                <div class="mode-selector-container">
                    <span class="mode-label">{move || crate::i18n::tr("mode", lang())}</span>
                    <div class="mode-btn-group">
                        <button 
                            class="mode-btn mode-base"
                            class:active=move || active_origin.get() == DotOrigin::Base
                            class=("limit-exceeded", move || costs.get().creation_points.has_any_overflow)
                            on:click=move |_| set_active_origin.set(DotOrigin::Base)
                            title=move || if costs.get().creation_points.has_any_overflow {
                                "Criação Base: ⚠️ Há orçamentos ou regras extrapolados! Clique em Extrato para ver."
                            } else {
                                "Criação Base de Personagem (Preto) - Orçamentos Oficiais M20"
                            }
                        >
                            <span class="mode-dot-icon dot-base"></span>
                            {move || crate::i18n::tr("mode_base", lang())}
                            {move || if costs.get().creation_points.has_any_overflow {
                                view! { <span class="badge-alert-dot">"⚠️"</span> }.into_view()
                            } else {
                                view! { <span></span> }.into_view()
                            }}
                        </button>
                        <button 
                            class="mode-btn mode-bonus"
                            class:active=move || active_origin.get() == DotOrigin::Bonus
                            class=("limit-exceeded", move || costs.get().total_bonus_spent > 15)
                            on:click=move |_| set_active_origin.set(DotOrigin::Bonus)
                            title="Pontos de Bônus Iniciais (Roxo - 5 Atrib, 2 Hab, 7 Esfera, 4 Arete, 1 Antecedente/FV)"
                        >
                            <span class="mode-dot-icon dot-bonus"></span>
                            {move || format!("{} ({}/15 pts)", crate::i18n::tr("mode_bonus", lang()), costs.get().total_bonus_spent)}
                        </button>
                        <button 
                            class="mode-btn mode-xp"
                            class:active=move || active_origin.get() == DotOrigin::Experience
                            on:click=move |_| set_active_origin.set(DotOrigin::Experience)
                            title="Experiência / XP Acumulado (Verde)"
                        >
                            <span class="mode-dot-icon dot-xp"></span>
                            {move || format!("{} ({} pts)", crate::i18n::tr("mode_xp", lang()), costs.get().total_xp_spent)}
                        </button>
                        <button 
                            class="mode-btn mode-temp"
                            class:active=move || active_origin.get() == DotOrigin::Temporary
                            on:click=move |_| set_active_origin.set(DotOrigin::Temporary)
                            title="Bônus Temporário / Feitiço / Wonder (Dourado)"
                        >
                            <span class="mode-dot-icon dot-temp"></span>
                            {move || crate::i18n::tr("mode_temp", lang())}
                        </button>
                    </div>

                    <button 
                        type="button" 
                        class="cost-breakdown-btn"
                        on:click=move |_| set_show_breakdown.set(true)
                        title="Ver extrato detalhado de gastos de Pontos de Bônus e XP"
                    >
                        {move || crate::i18n::tr("statement", lang())}
                    </button>

                    <button 
                        type="button" 
                        class="cost-breakdown-btn dossier-btn"
                        on:click=move |_| set_show_quiz.set(true)
                        title="Abrir Dossiê do Personagem (Questionário de Criação)"
                    >
                        {move || crate::i18n::tr("dossier", lang())}
                    </button>
                </div>
            </div>

            <div class="top-bar-right">
                <input 
                    type="file" 
                    accept=".json,application/json" 
                    node_ref=import_input_ref 
                    style="display: none;" 
                    on:change=on_file_change 
                />
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
                    <button
                        type="button"
                        class="lang-toggle-btn"
                        on:click=move |_| {
                            if let Some(ctx) = lang_ctx {
                                ctx.toggle();
                            }
                        }
                        title=move || match lang() {
                            crate::i18n::Language::PtBr => "Idioma: Português (Clique para mudar para English)",
                            crate::i18n::Language::EnUs => "Language: English (Click to switch to Português)",
                        }
                    >
                        {move || match lang() {
                            crate::i18n::Language::PtBr => "🇧🇷 PT",
                            crate::i18n::Language::EnUs => "🇺🇸 EN",
                        }}
                    </button>

                    <button class="manual-save-btn" on:click=move |ev| do_manual_save.call(ev) title="Salvar imediatamente">
                        <span class="btn-icon">"💾"</span>
                        <span>{move || crate::i18n::tr("save", lang())}</span>
                    </button>

                    <div class="top-bar-action-group">
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
                            {move || format!("📥 {}", crate::i18n::tr("import", lang()))}
                        </button>

                        <button 
                            type="button" 
                            class="json-top-btn json-export-btn"
                            on:click=move |_| on_export_json.call(())
                            title="Exportar e baixar esta ficha em arquivo .json"
                        >
                            {move || format!("📤 {}", crate::i18n::tr("export", lang()))}
                        </button>

                        <button 
                            type="button" 
                            class="export-pdf-btn" 
                            on:click=move |_| {
                                if let Some(set_pdf) = set_show_pdf_modal {
                                    set_pdf.set(true);
                                } else if let Some(w) = web_sys::window() {
                                    let _ = w.print();
                                }
                            } 
                            title="Exportar Ficha em PDF Oficial (A4)"
                        >
                            "🖨️ PDF"
                        </button>
                    </div>

                    <button 
                        type="button" 
                        class=move || if is_public.get() { "privacy-toggle-top-btn btn-public" } else { "privacy-toggle-top-btn btn-private" }
                        on:click=move |_| on_toggle_privacy.call(())
                        title=move || if is_public.get() { "Ficha Pública na comunidade. Clique para tornar Privada." } else { "Ficha Privada. Clique para tornar Pública na comunidade." }
                    >
                        {move || match (is_public.get(), lang()) {
                            (true, crate::i18n::Language::PtBr) => "🌐 Pública",
                            (true, crate::i18n::Language::EnUs) => "🌐 Public",
                            (false, crate::i18n::Language::PtBr) => "🔒 Privada",
                            (false, crate::i18n::Language::EnUs) => "🔒 Private",
                        }}
                    </button>
                </div>
            </div>
        </header>
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheet_top_bar_rendering_and_stable_root_structure() {
        let runtime = create_runtime();

        let (active_origin, set_active_origin) = create_signal(DotOrigin::Base);
        let costs = create_memo(|_| CostSummary::default());
        let (_show_breakdown, set_show_breakdown) = create_signal(false);
        let (_show_quiz, set_show_quiz) = create_signal(false);
        let (save_status, _set_save_status) = create_signal(SaveStatus::Idle);
        let (is_public, _set_is_public) = create_signal(false);

        let on_toggle_privacy = Callback::new(|_| ());
        let on_back_click = Callback::new(|_| ());
        let do_manual_save = Callback::new(|_| ());
        let on_export_json = Callback::new(|_| ());
        let on_import_json = Callback::new(|_| ());

        let view = view! {
            <SheetTopBar
                active_origin=active_origin
                set_active_origin=set_active_origin
                costs=costs
                set_show_breakdown=set_show_breakdown
                set_show_quiz=set_show_quiz
                save_status=save_status
                is_public=is_public.into()
                on_toggle_privacy=on_toggle_privacy
                on_back_click=on_back_click
                do_manual_save=do_manual_save
                on_export_json=on_export_json
                on_import_json=on_import_json
            />
        };

        let html = view.into_view().render_to_string();

        assert!(
            html.contains("header") && html.contains("sheet-top-bar"),
            "TopBar deve renderizar header com classe sheet-top-bar"
        );

        assert!(html.contains("json-export-btn"), "Botao exportar JSON deve estar presente");
        assert!(html.contains("json-import-btn"), "Botao importar JSON deve estar presente");

        runtime.dispose();
    }
}