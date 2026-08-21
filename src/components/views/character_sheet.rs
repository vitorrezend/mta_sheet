use leptos::*;
use leptos_router::*;
use crate::state::{get_sheet, update_sheet, CharacterData, DotOrigin};
use crate::components::common::Sheet;
use crate::components::page1::{Abilities, AdvantagesMta, Attributes, InfoHeader, Spheres};
use crate::components::page2::PageMagicCombat;
use crate::components::page3::PageExpandedBackgroundsPossessions;
use crate::components::page4::PageHistoryDescriptionVisuals;
use crate::components::sheet::{
    ActiveDotOriginContext, CostBreakdownModal, SaveStatus, SheetPageTab, SheetTabs, SheetTopBar,
};

fn get_current_time_str() -> &'static str {
    "agora"
}

#[component]
pub fn CharacterSheet() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());
    let get_id_untracked = move || params.with_untracked(|p| p.get("id").cloned().unwrap_or_default());

    let sheet_resource = create_local_resource(id, |id| async move {
        if id.is_empty() {
            return Err(ServerFnError::new("ID da ficha não fornecido"));
        }
        get_sheet(id).await
    });

    let (data, set_data) = create_signal(CharacterData::default());
    let (save_status, set_save_status) = create_signal(SaveStatus::Idle);
    let (is_dirty, set_is_dirty) = create_signal(false);
    let (is_loaded, set_is_loaded) = create_signal(false);
    let (active_origin, set_active_origin) = create_signal(DotOrigin::Base);
    let (active_tab, set_active_tab) = create_signal(SheetPageTab::Main);
    let (show_breakdown, set_show_breakdown) = create_signal(false);
    let navigate = use_navigate();

    // Provide the sheet data and active dot origin as context for all child components
    provide_context(set_data);
    provide_context(data);
    provide_context(ActiveDotOriginContext {
        origin: active_origin,
        set_origin: set_active_origin,
    });

    let is_mounted = std::rc::Rc::new(std::cell::Cell::new(true));
    let is_mounted_cleanup = is_mounted.clone();
    let change_seq = std::rc::Rc::new(std::cell::Cell::new(0u64));

    // Salva automaticamente no unmount (ao navegar para qualquer outro lugar da aplicação)
    on_cleanup(move || {
        is_mounted_cleanup.set(false);
        if is_dirty.try_get_untracked().unwrap_or(false) {
            let current_id = get_id_untracked();
            if let Some(current_data) = data.try_get_untracked() {
                if !current_id.is_empty() {
                    spawn_local(async move {
                        let _ = update_sheet(current_id.clone(), current_data).await;
                        crate::logging::log_client(
                            "user_actions",
                            "INFO",
                            "Ficha salva automaticamente ao sair da página (on_cleanup)",
                            Some(&format!("id={}", current_id)),
                        );
                    });
                }
            }
        }
    });

    create_effect(move |_| {
        if let Some(Ok(fetched_data)) = sheet_resource.get() {
            let _ = set_data.try_set(fetched_data);
            let _ = set_is_loaded.try_set(true);
            let _ = set_is_dirty.try_set(false);
            let _ = set_save_status.try_set(SaveStatus::Saved(get_current_time_str()));
            crate::logging::log_client(
                "user_actions",
                "INFO",
                &format!("Ficha carregada no navegador: id='{}'", get_id_untracked()),
                None,
            );
        }
    });

    // Marca o formulário como alterado (dirty) e dispara auto-save inteligente (Debounce de 1.2s)
    let change_seq_dirty = change_seq.clone();
    let is_mounted_dirty = is_mounted.clone();
    create_effect(move |_| {
        data.track();
        if is_loaded.try_get_untracked().unwrap_or(false) {
            let _ = set_is_dirty.try_set(true);
            let _ = set_save_status.try_set(SaveStatus::Pending);

            let next_seq = change_seq_dirty.get() + 1;
            change_seq_dirty.set(next_seq);

            let seq_check = change_seq_dirty.clone();
            let is_mounted_task = is_mounted_dirty.clone();

            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(1_200).await;
                if seq_check.get() == next_seq && is_dirty.try_get_untracked().unwrap_or(false) {
                    let current_id = get_id_untracked();
                    if let Some(current_data) = data.try_get_untracked() {
                        if !current_id.is_empty() {
                            if is_mounted_task.get() {
                                let _ = set_save_status.try_set(SaveStatus::Saving);
                            }
                            match update_sheet(current_id.clone(), current_data).await {
                                Ok(_) => {
                                    if seq_check.get() == next_seq {
                                        let _ = set_is_dirty.try_set(false);
                                        if is_mounted_task.get() {
                                            let _ = set_save_status.try_set(SaveStatus::Saved(get_current_time_str()));
                                        }
                                        crate::logging::log_client(
                                            "database",
                                            "INFO",
                                            "Auto-save inteligente (debounce 1.2s) executado com sucesso",
                                            Some(&format!("id={}", current_id)),
                                        );
                                    }
                                }
                                Err(e) => {
                                    if is_mounted_task.get() {
                                        let _ = set_save_status.try_set(SaveStatus::Error(e.to_string()));
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    });

    // Salvamento Periódico como redundância de segurança a cada 30 segundos
    let is_mounted_loop = is_mounted.clone();
    create_effect(move |_| {
        if is_loaded.get() {
            let is_mounted_task = is_mounted_loop.clone();
            spawn_local(async move {
                loop {
                    gloo_timers::future::TimeoutFuture::new(30_000).await;
                    if !is_mounted_task.get() {
                        break;
                    }

                    let current_data = match data.try_get_untracked() {
                        Some(d) => d,
                        None => break,
                    };

                    if is_dirty.try_get_untracked().unwrap_or(false) {
                        let current_id = get_id_untracked();
                        if !current_id.is_empty() && is_mounted_task.get() {
                            let _ = set_save_status.try_set(SaveStatus::Saving);
                            match update_sheet(current_id.clone(), current_data).await {
                                Ok(_) => {
                                    if !is_mounted_task.get() { break; }
                                    let _ = set_is_dirty.try_set(false);
                                    let _ = set_save_status.try_set(SaveStatus::Saved(get_current_time_str()));
                                }
                                Err(e) => {
                                    if !is_mounted_task.get() { break; }
                                    let _ = set_save_status.try_set(SaveStatus::Error(e.to_string()));
                                }
                            }
                        }
                    }
                }
            });
        }
    });

    // Salvamento manual
    let do_manual_save = Callback::new(move |_: ev::MouseEvent| {
        let current_id = get_id_untracked();
        if let Some(current_data) = data.try_get_untracked() {
            if !current_id.is_empty() {
                let _ = set_save_status.try_set(SaveStatus::Saving);
                spawn_local(async move {
                    match update_sheet(current_id.clone(), current_data).await {
                        Ok(_) => {
                            let _ = set_is_dirty.try_set(false);
                            crate::logging::log_client(
                                "user_actions",
                                "INFO",
                                "Salvamento manual acionado pelo usuário",
                                Some(&format!("id={}", current_id)),
                            );
                            let _ = set_save_status.try_set(SaveStatus::Saved(get_current_time_str()));
                        }
                        Err(e) => {
                            crate::logging::log_client(
                                "errors",
                                "ERROR",
                                "Falha no salvamento manual",
                                Some(&e.to_string()),
                            );
                            let _ = set_save_status.try_set(SaveStatus::Error(e.to_string()));
                        }
                    }
                });
            }
        }
    });

    // Navegação ao clicar em "← Início" garantindo salvamento antes de sair
    let on_back_click = Callback::new(move |ev: ev::MouseEvent| {
        ev.prevent_default();
        if is_dirty.try_get_untracked().unwrap_or(false) {
            let current_id = get_id_untracked();
            if let Some(current_data) = data.try_get_untracked() {
                let nav = navigate.clone();
                let _ = set_save_status.try_set(SaveStatus::Saving);
                spawn_local(async move {
                    if !current_id.is_empty() {
                        let _ = update_sheet(current_id.clone(), current_data).await;
                        crate::logging::log_client(
                            "user_actions",
                            "INFO",
                            "Ficha salva automaticamente ao navegar para a tela inicial",
                            Some(&format!("id={}", current_id)),
                        );
                    }
                    nav("/", Default::default());
                });
            } else {
                navigate.clone()("/", Default::default());
            }
        } else {
            navigate.clone()("/", Default::default());
        }
    });

    let costs = create_memo(move |_| data.with(|d| d.calculate_costs()));
    let is_public = Signal::derive(move || data.with(|d| d.is_public));

    let on_toggle_privacy = Callback::new(move |_| {
        let current_id = get_id_untracked();
        let current_pub = data.with_untracked(|d| d.is_public);
        let new_pub = !current_pub;
        set_data.update(|d| d.is_public = new_pub);
        if !current_id.is_empty() {
            spawn_local(async move {
                let _ = crate::state::set_sheet_visibility(current_id, new_pub).await;
            });
        }
    });

    view! {
        <link rel="stylesheet" href="/style.css"/>
        <div class="sheet-page-container">
            // Barra Superior e Seletor de Modos
            <SheetTopBar 
                active_origin=active_origin
                set_active_origin=set_active_origin
                costs=costs
                set_show_breakdown=set_show_breakdown
                save_status=save_status
                is_public=is_public
                on_toggle_privacy=on_toggle_privacy
                on_back_click=on_back_click
                do_manual_save=do_manual_save
            />

            // Modal de Extrato de Custos
            <CostBreakdownModal 
                costs=costs
                show_breakdown=show_breakdown
                set_show_breakdown=set_show_breakdown
                set_data=set_data
            />

            // Barra de Navegação de Abas (Página 1, Página 2, Perfil)
            <SheetTabs 
                active_tab=active_tab
                set_active_tab=set_active_tab
            />

            {move || match sheet_resource.get() {
                Some(Ok(_)) => view! {
                    <Sheet>
                        <div 
                            class="sheet-page-tab-pane page-main"
                            class:tab-hidden=move || active_tab.get() != SheetPageTab::Main
                        >
                            <InfoHeader />
                            <Attributes />
                            <Abilities />
                            <Spheres />
                            <AdvantagesMta />
                        </div>

                        <div 
                            class="sheet-page-tab-pane page-magic-combat"
                            class:tab-hidden=move || active_tab.get() != SheetPageTab::MagicCombat
                        >
                            <PageMagicCombat />
                        </div>

                        <div 
                            class="sheet-page-tab-pane page-expanded"
                            class:tab-hidden=move || active_tab.get() != SheetPageTab::Expanded
                        >
                            <PageExpandedBackgroundsPossessions />
                        </div>

                        <div 
                            class="sheet-page-tab-pane page-history-visuals"
                            class:tab-hidden=move || active_tab.get() != SheetPageTab::HistoryVisuals
                        >
                            <PageHistoryDescriptionVisuals />
                        </div>
                    </Sheet>
                }.into_view(),
                Some(Err(e)) => view! { 
                    <div class="error-container">
                        <p class="error-title">"Erro ao carregar a ficha"</p>
                        <p class="error-detail">{e.to_string()}</p>
                        <A href="/" class="back-home-btn">"Voltar para a lista de fichas"</A>
                    </div>
                }.into_view(),
                None => view! {
                    <div class="loading-state"><p>"Carregando Ficha..."</p></div>
                }.into_view(),
            }}
        </div>
    }
}
