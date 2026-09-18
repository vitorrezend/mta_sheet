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

    let show_points_menu = create_rw_signal(false);
    let show_actions_menu = create_rw_signal(false);
    let on_export_json_clone = on_export_json.clone();
    let on_toggle_privacy_clone = on_toggle_privacy.clone();

    view! {
        <header class="sheet-top-bar">
            <input 
                type="file" 
                accept=".json,application/json" 
                node_ref=import_input_ref 
                style="display: none;" 
                on:change=on_file_change 
            />

            // ── LINHA ÚNICA ULTRA-CLEAN: Navegação, Idioma, Pontos, Status, Salvar e Menu ──
            <div class="top-bar-primary-row">
                <div class="top-bar-left">
                    <a href="/" class="back-link" on:click=move |ev| on_back_click.call(ev) title="Voltar para o Início">
                        <span class="btn-text">{move || crate::i18n::tr("home", lang())}</span>
                    </a>

                    <button
                        type="button"
                        class="lang-toggle-btn"
                        on:click=move |_| {
                            if let Some(ctx) = lang_ctx {
                                ctx.toggle();
                            }
                        }
                        title=move || match lang() {
                            crate::i18n::Language::PtBr => "Idioma: Português (Mudar p/ EN)",
                            crate::i18n::Language::EnUs => "Language: English (Switch to PT)",
                        }
                    >
                        <span>{move || match lang() { crate::i18n::Language::PtBr => "🇧🇷 PT", crate::i18n::Language::EnUs => "🇺🇸 EN" }}</span>
                    </button>
                </div>

                <div class="top-bar-right">
                    // 1. Menu Hambúrguer de Pontos / Modos
                    <button 
                        type="button" 
                        class="points-menu-trigger-btn"
                        class:active=move || show_points_menu.get()
                        class=("limit-exceeded", move || costs.get().creation_points.has_any_overflow || costs.get().total_bonus_spent > 15)
                        on:click=move |_| {
                            show_actions_menu.set(false);
                            show_points_menu.update(|v| *v = !*v);
                        }
                        title="Modos de Pontuação e Extrato"
                    >
                        <span class="mode-dot-indicator" class=move || match active_origin.get() {
                            DotOrigin::Base => "dot-base",
                            DotOrigin::Bonus => "dot-bonus",
                            DotOrigin::Experience => "dot-xp",
                            DotOrigin::Temporary => "dot-temp",
                        }></span>
                        <span class="btn-text">
                            {move || match active_origin.get() {
                                DotOrigin::Base => crate::i18n::tr("mode_base", lang()),
                                DotOrigin::Bonus => crate::i18n::tr("mode_bonus", lang()),
                                DotOrigin::Experience => crate::i18n::tr("mode_xp", lang()),
                                DotOrigin::Temporary => "Buff",
                            }}
                        </span>
                        {move || if costs.get().creation_points.has_any_overflow || costs.get().total_bonus_spent > 15 {
                            view! { <span class="badge-alert-dot">"⚠️"</span> }.into_view()
                        } else {
                            view! { <span></span> }.into_view()
                        }}
                        <span class="btn-caret">{move || if show_points_menu.get() { "▲" } else { "▼" }}</span>
                    </button>

                    // 2. Pílula de Status (Salvo)
                    <div class="save-status-container">
                        {move || match save_status.get() {
                            SaveStatus::Idle => view! { <span class="status-badge status-idle"></span> }.into_view(),
                            SaveStatus::Pending => view! {
                                <span class="status-badge status-pending" title="Alterações pendentes...">
                                    <span class="status-dot dot-pending"></span>
                                    <span class="status-text">"Pendente"</span>
                                </span>
                            }.into_view(),
                            SaveStatus::Saving => view! {
                                <span class="status-badge status-saving" title="Gravando dados no banco...">
                                    <span class="status-spinner"></span>
                                    <span class="status-text">"Salvando..."</span>
                                </span>
                            }.into_view(),
                            SaveStatus::Saved(t) => view! {
                                <span class="status-badge status-saved" title="Todas as alterações foram salvas">
                                    <span class="status-dot dot-saved"></span>
                                    <span class="status-text">{format!("Salvo ({})", t)}</span>
                                </span>
                            }.into_view(),
                            SaveStatus::Error(err) => {
                                let err_title = err.clone();
                                view! {
                                    <span class="status-badge status-error" title=err_title>
                                        <span class="status-dot dot-error"></span>
                                        <span class="status-text">"Erro"</span>
                                    </span>
                                }.into_view()
                            },
                        }}
                    </div>

                    // 3. Botão Salvar
                    <button class="manual-save-btn" on:click=move |ev| do_manual_save.call(ev) title="Salvar imediatamente">
                        <span class="btn-icon">"💾"</span>
                        <span class="btn-text">{move || crate::i18n::tr("save", lang())}</span>
                    </button>

                    // 4. Menu Hambúrguer de Detalhes / Ações
                    <button 
                        type="button" 
                        class="actions-menu-trigger-btn"
                        class:active=move || show_actions_menu.get()
                        on:click=move |_| {
                            show_points_menu.set(false);
                            show_actions_menu.update(|v| *v = !*v);
                        }
                        title="Abrir menu de detalhes, dossiê e opções da ficha"
                    >
                        <span class="btn-icon">"⚙️"</span>
                        <span class="btn-text">"Mais"</span>
                        <span class="btn-caret">{move || if show_actions_menu.get() { "▲" } else { "▼" }}</span>
                    </button>
                </div>
            </div>

            // ── Menu Modal / Dropdown 1: Modos de Pontuação & Extrato ──
            <div 
                class="actions-menu-backdrop points-modal-backdrop"
                class:open=move || show_points_menu.get()
                on:click=move |_| show_points_menu.set(false)
            >
                <div class="actions-menu-panel points-menu-panel" on:click=move |ev| ev.stop_propagation()>
                    <div class="actions-menu-panel-header">
                        <span class="panel-title">"📊 Modos de Pontuação & Custos"</span>
                        <button type="button" class="panel-close-btn" on:click=move |_| show_points_menu.set(false)>"✕"</button>
                    </div>
                    <div class="points-menu-body">
                        <div class="points-menu-modes-list">
                            <button 
                                type="button"
                                class="points-mode-option mode-base"
                                class:selected=move || active_origin.get() == DotOrigin::Base
                                on:click=move |_| {
                                    set_active_origin.set(DotOrigin::Base);
                                    show_points_menu.set(false);
                                }
                            >
                                <span class="mode-dot-icon dot-base"></span>
                                <div class="option-info">
                                    <div class="option-title-row">
                                        <span class="option-title">"Criação Base (Preto)"</span>
                                        {move || if costs.get().creation_points.has_any_overflow {
                                            view! { <span class="badge-alert-tag">"⚠️ Regra"</span> }.into_view()
                                        } else {
                                            view! { <span></span> }.into_view()
                                        }}
                                    </div>
                                    <span class="option-sub">"Pontos iniciais (7/5/3 Atrib, 13/9/5 Hab, 6 Esf)"</span>
                                </div>
                            </button>

                            <button 
                                type="button"
                                class="points-mode-option mode-bonus"
                                class:selected=move || active_origin.get() == DotOrigin::Bonus
                                on:click=move |_| {
                                    set_active_origin.set(DotOrigin::Bonus);
                                    show_points_menu.set(false);
                                }
                            >
                                <span class="mode-dot-icon dot-bonus"></span>
                                <div class="option-info">
                                    <div class="option-title-row">
                                        <span class="option-title">{move || format!("Pontos de Bônus ({}/15 pts)", costs.get().total_bonus_spent)}</span>
                                        {move || if costs.get().total_bonus_spent > 15 {
                                            view! { <span class="badge-alert-tag">"⚠️ > 15 pts"</span> }.into_view()
                                        } else {
                                            view! { <span></span> }.into_view()
                                        }}
                                    </div>
                                    <span class="option-sub">"5 Atrib, 2 Hab, 7 Esfera, 4 Arete, 1 FV/Antecedente"</span>
                                </div>
                            </button>

                            <button 
                                type="button"
                                class="points-mode-option mode-xp"
                                class:selected=move || active_origin.get() == DotOrigin::Experience
                                on:click=move |_| {
                                    set_active_origin.set(DotOrigin::Experience);
                                    show_points_menu.set(false);
                                }
                            >
                                <span class="mode-dot-icon dot-xp"></span>
                                <div class="option-info">
                                    <div class="option-title-row">
                                        <span class="option-title">{move || format!("Experiência / XP ({} pts)", costs.get().total_xp_spent)}</span>
                                    </div>
                                    <span class="option-sub">"Evolução acumulada ao longo da crônica"</span>
                                </div>
                            </button>

                            <button 
                                type="button"
                                class="points-mode-option mode-temp"
                                class:selected=move || active_origin.get() == DotOrigin::Temporary
                                on:click=move |_| {
                                    set_active_origin.set(DotOrigin::Temporary);
                                    show_points_menu.set(false);
                                }
                            >
                                <span class="mode-dot-icon dot-temp"></span>
                                <div class="option-info">
                                    <div class="option-title-row">
                                        <span class="option-title">"Buff Temporário / Magia (Dourado)"</span>
                                    </div>
                                    <span class="option-sub">"Feitiços ativos, maravilhas e bônus efêmeros"</span>
                                </div>
                            </button>
                        </div>

                        <div class="points-menu-footer">
                            <button 
                                type="button" 
                                class="cost-breakdown-btn full-statement-btn"
                                on:click=move |_| {
                                    show_points_menu.set(false);
                                    set_show_breakdown.set(true);
                                }
                            >
                                <span class="btn-icon">"📊"</span>
                                <span class="btn-text">"Abrir Extrato Completo de Gastos"</span>
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            // ── Menu Modal / Dropdown 2: Detalhes, Dossiê & Arquivo ──
            {
                let on_export_mobile = on_export_json_clone.clone();
                let on_privacy_mobile = on_toggle_privacy_clone.clone();
                let pdf_modal = set_show_pdf_modal;
                move || {
                    let on_export = on_export_mobile.clone();
                    let on_privacy = on_privacy_mobile.clone();

                    view! {
                        <div 
                            class="actions-menu-backdrop"
                            class:open=move || show_actions_menu.get()
                            on:click=move |_| show_actions_menu.set(false)
                        >
                            <div class="actions-menu-panel" on:click=move |ev| ev.stop_propagation()>
                                <div class="actions-menu-panel-header">
                                    <span class="panel-title">"⚙️ Detalhes & Arquivo"</span>
                                    <button type="button" class="panel-close-btn" on:click=move |_| show_actions_menu.set(false)>"✕"</button>
                                </div>
                                <div class="actions-menu-panel-body">
                                    <button 
                                        type="button" 
                                        class="actions-menu-item-btn dossier-menu-btn"
                                        on:click=move |_| {
                                            show_actions_menu.set(false);
                                            set_show_quiz.set(true);
                                        }
                                    >
                                        <span class="action-icon">"📂"</span>
                                        <span class="action-label">"Dossiê do Personagem (Questionário)"</span>
                                    </button>

                                    <button 
                                        type="button" 
                                        class="actions-menu-item-btn json-import-btn"
                                        on:click=move |_| {
                                            show_actions_menu.set(false);
                                            if let Some(input) = import_input_ref.get() {
                                                input.click();
                                            }
                                        }
                                    >
                                        <span class="action-icon">"📥"</span>
                                        <span class="action-label">"Importar Ficha (.json)"</span>
                                    </button>

                                    <button 
                                        type="button" 
                                        class="actions-menu-item-btn json-export-btn"
                                        on:click=move |_| {
                                            show_actions_menu.set(false);
                                            on_export.call(());
                                        }
                                    >
                                        <span class="action-icon">"📤"</span>
                                        <span class="action-label">"Exportar Ficha (.json)"</span>
                                    </button>

                                    <button 
                                        type="button" 
                                        class="actions-menu-item-btn export-pdf-btn"
                                        on:click=move |_| {
                                            show_actions_menu.set(false);
                                            if let Some(set_pdf) = pdf_modal {
                                                set_pdf.set(true);
                                            } else if let Some(w) = web_sys::window() {
                                                let _ = w.print();
                                            }
                                        }
                                    >
                                        <span class="action-icon">"🖨️"</span>
                                        <span class="action-label">"Exportar em PDF Oficial (A4)"</span>
                                    </button>

                                    <button 
                                        type="button" 
                                        class="actions-menu-item-btn privacy-toggle-top-btn"
                                        class:btn-public=move || is_public.get()
                                        class:btn-private=move || !is_public.get()
                                        on:click=move |_| on_privacy.call(())
                                    >
                                        <span class="action-icon">{move || if is_public.get() { "🌐" } else { "🔒" }}</span>
                                        <span class="action-label">
                                            {move || if is_public.get() { "Ficha Pública (Tornar Privada)" } else { "Ficha Privada (Tornar Pública)" }}
                                        </span>
                                    </button>

                                    <a 
                                        href="/logs" 
                                        class="actions-menu-item-btn logs-nav-link"
                                        on:click=move |_| show_actions_menu.set(false)
                                    >
                                        <span class="action-icon">"📊"</span>
                                        <span class="action-label">{move || crate::i18n::tr("logs", lang())}</span>
                                    </a>
                                </div>
                            </div>
                        </div>
                    }.into_view()
                }
            }
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