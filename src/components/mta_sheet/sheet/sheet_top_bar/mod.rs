//! Cabeçalho da ficha: ações (voltar, salvar, compartilhar, menu) em cima e, para quem
//! edita, a faixa de modos de pontuação embaixo.

mod menu;
mod points;

use leptos::*;
use leptos_router::A;
use wasm_bindgen::JsCast;
use crate::components::Callback;
use crate::i18n::Language;
use crate::state::{CharacterData, CostSummary, DotOrigin};
use super::{ActiveDotOriginContext, SheetLayout};
use menu::MoreMenu;
use points::PointsModeBar;

/// Texto no idioma atual.
fn pick(lang: Language, pt: &'static str, en: &'static str) -> &'static str {
    match lang {
        Language::PtBr => pt,
        Language::EnUs => en,
    }
}

/// Texto curto e dica do indicador de salvamento.
fn save_label(status: &SaveStatus, lang: Language) -> (&'static str, String) {
    match status {
        SaveStatus::Idle | SaveStatus::Saved(_) => (
            pick(lang, "Salvo", "Saved"),
            pick(
                lang,
                "Tudo salvo. A ficha salva sozinha enquanto você edita; clique para salvar agora.",
                "All saved. The sheet saves itself while you edit; click to save now.",
            )
            .to_string(),
        ),
        SaveStatus::Pending => (
            pick(lang, "Alterado", "Edited"),
            pick(lang, "Alterações serão salvas em instantes; clique para salvar agora.", "Changes will be saved shortly; click to save now.").to_string(),
        ),
        SaveStatus::Saving => (pick(lang, "Salvando…", "Saving…"), pick(lang, "Gravando no servidor…", "Saving to the server…").to_string()),
        SaveStatus::Error(err) => (
            pick(lang, "Erro ao salvar", "Save failed"),
            format!("{} {}", pick(lang, "Clique para tentar de novo.", "Click to retry."), err),
        ),
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SaveStatus {
    Idle,
    Pending,
    Saving,
    Saved(&'static str),
    Error(String),
}

/// O que a barra mostra sobre a ficha.
#[derive(Clone)]
pub struct SheetTopBarState {
    pub costs: Memo<CostSummary>,
    pub save_status: Signal<SaveStatus>,
    pub is_public: Signal<bool>,
    pub can_edit: MaybeSignal<bool>,
    pub author_username: MaybeSignal<Option<String>>,
    /// Largura da ficha na tela (A4 ou Larga)
    pub layout: RwSignal<SheetLayout>,
}

/// O que os botões da barra fazem: ações e janelas que eles abrem.
#[derive(Clone)]
pub struct SheetTopBarActions {
    pub on_back_click: Callback<ev::MouseEvent>,
    pub do_manual_save: Callback<ev::MouseEvent>,
    pub on_toggle_privacy: Callback<()>,
    pub on_export_json: Callback<()>,
    pub on_import_json: Callback<CharacterData>,
    pub on_clone_sheet: Option<Callback<()>>,
    pub set_show_breakdown: WriteSignal<bool>,
    pub set_show_quiz: WriteSignal<bool>,
    pub set_show_pdf_modal: Option<WriteSignal<bool>>,
    pub set_show_share_modal: Option<WriteSignal<bool>>,
}

#[component]
pub fn SheetTopBar(
    /// Modo de pontos ativo (Base, Bônus, XP, Buff)
    origin: ActiveDotOriginContext,
    state: SheetTopBarState,
    actions: SheetTopBarActions,
) -> impl IntoView {
    let ActiveDotOriginContext { origin: active_origin, set_origin: set_active_origin } = origin;
    let SheetTopBarState { costs, save_status, is_public, can_edit, author_username, layout } = state;
    let author_username = Signal::derive(move || author_username.get());
    let menu_actions = actions.clone();
    let SheetTopBarActions {
        on_back_click,
        do_manual_save,
        on_import_json,
        on_clone_sheet,
        set_show_breakdown,
        set_show_share_modal,
        ..
    } = actions;

    let import_input_ref = create_node_ref::<html::Input>();
    let on_file_change = move |ev: ev::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Some(file) = target.files().and_then(|files| files.get(0)) {
            let on_import = on_import_json.clone();
            if let Ok(reader) = web_sys::FileReader::new() {
                let reader_clone = reader.clone();
                let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::ProgressEvent| {
                    let Some(text) = reader_clone.result().ok().and_then(|r| r.as_string()) else { return };
                    match crate::components::common::parse_and_sanitize_sheet_json(&text) {
                        Ok(parsed) => on_import.call(parsed),
                        Err(err) => {
                            if let Some(w) = web_sys::window() {
                                let _ = w.alert_with_message(&format!("Erro ao importar arquivo JSON: {}", err));
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);
                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                onload.forget();
                let _ = reader.read_as_text(&file);
            }
        }
        target.set_value("");
    };

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = Signal::derive(move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default());
    let show_menu = create_rw_signal(false);
    let status_class = move || match save_status.get() {
        SaveStatus::Idle | SaveStatus::Saved(_) => "is-saved",
        SaveStatus::Pending => "is-pending",
        SaveStatus::Saving => "is-saving",
        SaveStatus::Error(_) => "is-error",
    };

    view! {
        <header class="sheet-top-bar">
            <input type="file" accept=".json,application/json" node_ref=import_input_ref style="display: none;" on:change=on_file_change />

            <div class="top-bar-primary-row">
                <div class="top-bar-left">
                    // Sem delegação: o roteador também escuta cliques em links no window e navegaria
                    // antes de a edição pendente ser salva
                    <a
                        href="/"
                        class="back-link"
                        on:click:undelegated=move |ev| on_back_click.call(ev)
                        title=move || pick(lang.get(), "Voltar para o Início (salva antes de sair)", "Back to Home (saves first)")
                    >
                        <span aria-hidden="true">"←"</span>
                        <span class="btn-text">{move || pick(lang.get(), "Início", "Home")}</span>
                    </a>
                    <Show when=move || !can_edit.get()>
                        <div class="readonly-view-pill">
                            <span>{move || pick(lang.get(), "👁️ Somente leitura", "👁️ Read only")}</span>
                            {move || author_username.get().map(|author| view! {
                                <span class="readonly-author">
                                    {pick(lang.get_untracked(), "por ", "by ")}
                                    <A href=format!("/user/{}", author) class="readonly-author-link">{format!("@{}", author)}</A>
                                </span>
                            })}
                        </div>
                    </Show>
                </div>

                <div class="top-bar-right">
                    {
                        let do_save = do_manual_save.clone();
                        let on_clone = on_clone_sheet.clone();
                        move || if can_edit.get() {
                            let do_save = do_save.clone();
                            view! {
                                <div class="top-bar-actions">
                                    <button
                                        type="button"
                                        class=move || format!("manual-save-btn save-indicator {}", status_class())
                                        title=move || save_status.with(|s| save_label(s, lang.get()).1)
                                        on:click=move |ev| do_save.call(ev)
                                    >
                                        <span class="save-indicator-dot" aria-hidden="true"></span>
                                        <span class="save-indicator-text" aria-live="polite">{move || save_status.with(|s| save_label(s, lang.get()).0)}</span>
                                        <span class="save-indicator-icon" aria-hidden="true">"💾"</span>
                                    </button>
                                    <button
                                        type="button"
                                        class="share-top-bar-btn"
                                        title=move || pick(lang.get(), "Compartilhar com pessoas, salas e link público", "Share with people, rooms and a public link")
                                        on:click=move |_| {
                                            if let Some(set_share) = set_show_share_modal {
                                                set_share.set(true);
                                            }
                                        }
                                    >
                                        <span class="btn-icon">"🔗"</span>
                                        <span class="btn-text">{move || pick(lang.get(), "Compartilhar", "Share")}</span>
                                    </button>
                                </div>
                            }.into_view()
                        } else {
                            let on_clone = on_clone.clone();
                            view! {
                                <div class="top-bar-actions">
                                    <button
                                        type="button"
                                        class="clone-sheet-top-btn"
                                        title=move || pick(lang.get(), "Copiar esta ficha para a sua biblioteca", "Copy this sheet to your library")
                                        on:click=move |_| {
                                            if let Some(ref cb) = on_clone {
                                                cb.call(());
                                            }
                                        }
                                    >
                                        <span class="btn-icon">"📋"</span>
                                        <span class="btn-text">{move || pick(lang.get(), "Clonar ficha", "Clone sheet")}</span>
                                    </button>
                                </div>
                            }.into_view()
                        }
                    }
                    <button
                        type="button"
                        class="actions-menu-trigger-btn"
                        class:active=move || show_menu.get()
                        aria-haspopup="menu"
                        aria-expanded=move || if show_menu.get() { "true" } else { "false" }
                        title=move || pick(lang.get(), "Mais opções: dossiê, arquivo, compartilhamento e exibição", "More: dossier, file, sharing and display")
                        on:click=move |_| show_menu.update(|v| *v = !*v)
                    >
                        <span aria-hidden="true">"⋯"</span>
                        <span class="btn-text">{move || pick(lang.get(), "Mais", "More")}</span>
                    </button>
                </div>
            </div>

            <Show when=move || can_edit.get()>
                <PointsModeBar
                    active_origin=active_origin
                    set_active_origin=set_active_origin
                    costs=costs
                    lang=lang
                    on_open_breakdown=Callback::new(move |_| set_show_breakdown.set(true))
                />
            </Show>

            <MoreMenu
                open=show_menu
                can_edit=can_edit
                is_public=is_public
                layout=layout
                lang=lang
                actions=menu_actions.clone()
                on_pick_import=Callback::new(move |_| {
                    if let Some(input) = import_input_ref.get() {
                        input.click();
                    }
                })
            />
        </header>
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheet_top_bar_rendering_and_stable_root_structure() {
        let runtime = create_runtime();

        let (origin, set_origin) = create_signal(DotOrigin::Base);
        let (_show_breakdown, set_show_breakdown) = create_signal(false);
        let (_show_quiz, set_show_quiz) = create_signal(false);
        let state = SheetTopBarState {
            costs: create_memo(|_| CostSummary::default()),
            save_status: Signal::derive(|| SaveStatus::Idle),
            is_public: Signal::derive(|| false),
            can_edit: true.into(),
            author_username: None.into(),
            layout: create_rw_signal(SheetLayout::A4),
        };
        let actions = SheetTopBarActions {
            on_back_click: Callback::new(|_| ()),
            do_manual_save: Callback::new(|_| ()),
            on_toggle_privacy: Callback::new(|_| ()),
            on_export_json: Callback::new(|_| ()),
            on_import_json: Callback::new(|_| ()),
            on_clone_sheet: None,
            set_show_breakdown,
            set_show_quiz,
            set_show_pdf_modal: None,
            set_show_share_modal: None,
        };

        let view = view! {
            <SheetTopBar origin=ActiveDotOriginContext { origin, set_origin } state=state actions=actions />
        };

        let html = view.into_view().render_to_string();

        assert!(
            html.contains("header") && html.contains("sheet-top-bar"),
            "TopBar deve renderizar header com classe sheet-top-bar"
        );

        assert!(html.contains("json-export-btn"), "Botao exportar JSON deve estar presente");
        assert!(html.contains("json-import-btn"), "Botao importar JSON deve estar presente");
        assert!(html.contains("share-top-bar-btn"), "Botao compartilhar deve estar presente");

        runtime.dispose();
    }
}