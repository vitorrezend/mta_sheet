//! Tela da ficha: monta o estado (store), as janelas e as páginas.

mod compendium;
mod meta;
mod store;

use leptos::*;
use leptos_router::*;
use crate::state::{clone_sheet, get_sheet, CharacterData, DotOrigin};
use super::sheet_conflict_banner::SheetConflictBanner;
use crate::components::{Callback, Sheet};
use crate::components::mta_sheet::page1::{Abilities, AdvantagesMta, Attributes, InfoHeader, Spheres};
use crate::components::mta_sheet::page2::PageMagicCombat;
use crate::components::mta_sheet::page3::PageExpandedBackgroundsPossessions;
use crate::components::mta_sheet::page4::PageHistoryDescriptionVisuals;
use crate::components::mta_sheet::page5::PageGrimoire;
use crate::components::mta_sheet::page6::PageNotes;
use crate::components::mta_sheet::sheet::{
    ActiveDotOriginContext, CostBreakdownModal, QuizModal, SheetDiceRoller, SheetPageTab, SheetShareModal, SheetTabs,
    SheetTopBar, SheetTopBarActions, SheetTopBarState, use_sheet_layout,
};
use compendium::{provide_sheet_compendium, SheetCompendiumModal};
use meta::SheetMeta;
use store::SheetStore;

pub use compendium::{CompendiumModalState, CompendiumTarget};

#[component]
pub fn CharacterSheet() -> impl IntoView {
    let params = use_params_map();
    let query = use_query_map();
    let id = Signal::derive(move || params.with(|p| p.get("id").cloned().unwrap_or_default()));
    let token = Signal::derive(move || query.with(|q| q.get("token").cloned()));

    let sheet_resource = create_local_resource(
        move || (id.get(), token.get()),
        |(id, token)| async move {
            if id.is_empty() {
                return Err(ServerFnError::new("ID da ficha não fornecido"));
            }
            get_sheet(id, token).await
        },
    );

    let store = SheetStore::new(id, token);
    let SheetStore { data, set_data, .. } = store;
    let (active_origin, set_active_origin) = create_signal(DotOrigin::Base);
    let origin = ActiveDotOriginContext { origin: active_origin, set_origin: set_active_origin };
    let (active_tab, set_active_tab) = create_signal(SheetPageTab::Main);
    let layout = use_sheet_layout();
    let (show_breakdown, set_show_breakdown) = create_signal(false);
    let (show_share_modal, set_show_share_modal) = create_signal(false);
    let (show_clone_login_modal, set_show_clone_login_modal) = create_signal(false);
    let (show_quiz_modal, set_show_quiz_modal) = create_signal(false);
    let (show_pdf_modal, set_show_pdf_modal) = create_signal(false);
    let navigate = use_navigate();

    // As páginas da ficha leem e alteram os dados por contexto
    provide_context(set_data);
    provide_context(data);
    provide_context(origin);
    let compendium = provide_sheet_compendium();

    store.install_autosave();
    create_effect(move |_| {
        if let Some(Ok(fetched)) = sheet_resource.get() {
            store.loaded(fetched);
        }
    });

    let nav_back = navigate.clone();
    let on_back_click = Callback::new(move |ev: ev::MouseEvent| {
        ev.prevent_default();
        let nav = nav_back.clone();
        store.save_then(move || nav("/", Default::default()));
    });

    let (is_cloning, set_is_cloning) = create_signal(false);
    let nav_clone = navigate.clone();
    let on_clone_sheet = Callback::new(move |_: ()| {
        let current_id = store.id();
        if current_id.is_empty() || is_cloning.get() {
            return;
        }
        set_is_cloning.set(true);
        let nav = nav_clone.clone();
        let token = store.token();
        spawn_local(async move {
            match clone_sheet(current_id, token).await {
                Ok(new_id) => {
                    crate::logging::log_client("user_actions", "INFO", "Ficha clonada com sucesso", Some(&format!("new_id={}", new_id)));
                    nav(&format!("/sheet/{}", new_id), Default::default());
                }
                Err(e) => {
                    let err = e.to_string();
                    crate::logging::log_client("errors", "ERROR", "Erro ao clonar ficha", Some(&err));
                    set_is_cloning.set(false);
                    if err.contains("logado") || err.contains("autenticado") {
                        set_show_clone_login_modal.set(true);
                    } else if let Some(w) = web_sys::window() {
                        let _ = w.alert_with_message(&format!("Não foi possível clonar a ficha: {}", err));
                    }
                }
            }
        });
    });

    let can_edit = Signal::derive(move || data.with(|d| d.can_edit));
    let costs = create_memo(move |_| data.with(|d| d.calculate_costs()));
    let is_gods_and_monsters = create_memo(move |_| data.with(|d| d.is_gods_and_monsters()));

    let top_bar_state = SheetTopBarState {
        costs,
        save_status: store.status.into(),
        is_public: Signal::derive(move || data.with(|d| d.is_public)),
        can_edit: can_edit.into(),
        author_username: Signal::derive(move || data.with(|d| d.author_username.clone())).into(),
        layout,
    };
    let top_bar_actions = SheetTopBarActions {
        on_back_click,
        do_manual_save: Callback::new(move |_| store.save("Salvamento manual acionado pelo usuário")),
        on_toggle_privacy: Callback::new(move |_| store.toggle_privacy()),
        on_export_json: Callback::new(move |_| {
            if let Some(current) = data.try_get_untracked() {
                crate::components::common::export_character_json(&current);
            }
        }),
        on_import_json: Callback::new(move |imported: CharacterData| store.import(imported)),
        on_clone_sheet: Some(on_clone_sheet),
        set_show_breakdown,
        set_show_quiz: set_show_quiz_modal,
        set_show_pdf_modal: Some(set_show_pdf_modal),
        set_show_share_modal: Some(set_show_share_modal),
    };

    // Conflito de versão: recarregar a ficha do servidor ou gravar por cima mantendo as minhas alterações
    let reload_latest = move |_| {
        store.conflict.set(false);
        sheet_resource.refetch();
    };

    view! {
        <div
            class="sheet-page-container"
            class:sheet-readonly=move || !can_edit.get()
            class:sheet-layout-wide=move || layout.get().is_wide()
        >
            <SheetMeta data=data />
            <SheetDiceRoller sheet_id=id data=data can_edit=can_edit />
            <SheetConflictBanner conflict=store.conflict on_reload=reload_latest on_keep=move |_| store.keep_mine() />
            <SheetTopBar origin=origin state=top_bar_state actions=top_bar_actions />

            <SheetShareModal
                show_modal=show_share_modal
                set_show_modal=set_show_share_modal
                sheet_id=id
                sheet_name=Signal::derive(move || data.with(|d| d.get_display_name()))
            />
            <CloneLoginModal show=show_clone_login_modal set_show=set_show_clone_login_modal />
            <CostBreakdownModal costs=costs show_breakdown=show_breakdown set_show_breakdown=set_show_breakdown set_data=set_data />
            <QuizModal show_quiz=show_quiz_modal set_show_quiz=set_show_quiz_modal data=data set_data=set_data />
            <crate::components::mta_sheet::sheet::pdf_export_modal::PdfExportModal
                show_modal=show_pdf_modal
                set_show_modal=set_show_pdf_modal
                data=data
                current_active_tab=active_tab.into()
                is_gods_and_monsters=is_gods_and_monsters
            />

            <SheetTabs active_tab=active_tab set_active_tab=set_active_tab is_gods_and_monsters=is_gods_and_monsters.into() />

            <div class="sheet-main-content">
                {move || {
                    if store.is_loaded.get() {
                        view! { <SheetPages active_tab=active_tab is_gods_and_monsters=is_gods_and_monsters.get() /> }.into_view()
                    } else if let Some(Err(e)) = sheet_resource.get() {
                        view! {
                            <div class="error-container">
                                <p class="error-title">"Erro ao carregar a ficha"</p>
                                <p class="error-detail">{e.to_string()}</p>
                                <A href="/" class="back-home-btn">"Voltar para a lista de fichas"</A>
                            </div>
                        }.into_view()
                    } else {
                        view! { <div class="loading-state"><p>"Carregando Ficha..."</p></div> }.into_view()
                    }
                }}
            </div>

            // Compêndio montado por último: fica acima das abas e do conteúdo
            <SheetCompendiumModal compendium=compendium />
        </div>
    }
}

/// Uma aba da ficha; as outras continuam montadas, só escondidas.
#[component]
fn TabPane(tab: SheetPageTab, active_tab: ReadSignal<SheetPageTab>, class: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class=format!("sheet-page-tab-pane {}", class) class:tab-hidden=move || active_tab.get() != tab>
            {children()}
        </div>
    }
}

#[component]
fn SheetPages(active_tab: ReadSignal<SheetPageTab>, is_gods_and_monsters: bool) -> impl IntoView {
    if is_gods_and_monsters {
        return view! {
            <Sheet>
                <TabPane tab=SheetPageTab::Main active_tab=active_tab class="page-gods-1">
                    <crate::components::gods_and_monsters::GodsAndMonstersPage1 />
                </TabPane>
                <TabPane tab=SheetPageTab::MagicCombat active_tab=active_tab class="page-gods-2">
                    <crate::components::gods_and_monsters::GodsAndMonstersPage2 />
                </TabPane>
            </Sheet>
        }
        .into_view();
    }
    view! {
        <Sheet>
            <TabPane tab=SheetPageTab::Main active_tab=active_tab class="page-main">
                <InfoHeader />
                <Attributes />
                <Abilities />
                <Spheres />
                <AdvantagesMta />
            </TabPane>
            <TabPane tab=SheetPageTab::MagicCombat active_tab=active_tab class="page-magic-combat">
                <PageMagicCombat />
            </TabPane>
            <TabPane tab=SheetPageTab::Expanded active_tab=active_tab class="page-expanded">
                <PageExpandedBackgroundsPossessions />
            </TabPane>
            <TabPane tab=SheetPageTab::HistoryVisuals active_tab=active_tab class="page-history-visuals">
                <PageHistoryDescriptionVisuals />
            </TabPane>
            <TabPane tab=SheetPageTab::Grimoire active_tab=active_tab class="page-grimoire">
                <PageGrimoire />
            </TabPane>
            <TabPane tab=SheetPageTab::Notes active_tab=active_tab class="page-notes">
                <PageNotes />
            </TabPane>
        </Sheet>
    }
    .into_view()
}

/// Aviso para quem tenta clonar a ficha sem estar logado.
#[component]
fn CloneLoginModal(show: ReadSignal<bool>, set_show: WriteSignal<bool>) -> impl IntoView {
    view! {
        <crate::components::common::Modal
            is_open=show
            on_close=crate::components::common::SafeCallback::new(move |_| set_show.set(false))
            title="Salvar Cópia da Ficha"
            icon=Some("📋")
            size=crate::components::common::ModalSize::Sm
        >
            <div class="clone-prompt-content">
                <div class="clone-prompt-icon">"✨"</div>
                <h4 class="clone-prompt-title">"Crie sua cópia pessoal!"</h4>
                <p class="clone-prompt-desc">
                    "Para clonar esta ficha e editá-la livremente em sua própria biblioteca, faça login na sua conta ou crie um cadastro gratuito."
                </p>
                <div class="clone-prompt-actions">
                    <a href="/login" class="clone-prompt-login-btn">
                        <span>"🔑"</span>
                        <span>"Entrar ou Cadastrar"</span>
                    </a>
                    <button type="button" class="clone-prompt-close-btn" on:click=move |_| set_show.set(false)>
                        "Fechar"
                    </button>
                </div>
            </div>
        </crate::components::common::Modal>
    }
}
