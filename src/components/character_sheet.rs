use leptos::*;
use leptos_router::*;
use crate::state::{get_sheet, update_sheet, CharacterData, DotOrigin};
use crate::components::{Attributes, InfoHeader, Abilities, Spheres, AdvantagesMta, Sheet, CharacterProfile, PageMagicCombat};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SheetPageTab {
    #[default]
    Main,
    MagicCombat,
    Profile,
}

#[derive(Clone, Copy)]
pub struct ActiveDotOriginContext {
    pub origin: ReadSignal<DotOrigin>,
    #[allow(dead_code)]
    pub set_origin: WriteSignal<DotOrigin>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SaveStatus {
    Idle,
    Saved(String),
    Saving,
    Pending,
    Error(String),
}

fn get_current_time_str() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::new_0()
            .to_locale_time_string("pt-BR")
            .as_string()
            .unwrap_or_else(|| "agora".to_string())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "agora".to_string()
    }
}

#[component]
pub fn CharacterSheet() -> impl IntoView {
    let params = use_params_map();
    let id = create_memo(move |_| params.with(|p| p.get("id").cloned().unwrap_or_default()));

    let sheet_resource = create_local_resource(move || id.get(), |id| async move {
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
    let navigate = use_navigate();

    // Provide the sheet data and active dot origin as context for all child components
    provide_context(set_data);
    provide_context(data);
    provide_context(ActiveDotOriginContext {
        origin: active_origin,
        set_origin: set_active_origin,
    });

    create_effect(move |_| {
        if let Some(Ok(fetched_data)) = sheet_resource.get() {
            set_data.set(fetched_data);
            set_is_loaded.set(true);
            set_is_dirty.set(false);
            set_save_status.set(SaveStatus::Saved(get_current_time_str()));
            let current_id = id.get_untracked();
            crate::logging::log_client(
                "user_actions",
                "INFO",
                &format!("Ficha carregada no navegador: id='{}'", current_id),
                None,
            );
        }
    });

    // Marca o formulário como alterado (dirty) quando qualquer dado muda
    create_effect(move |_| {
        data.track();
        if is_loaded.try_get_untracked().unwrap_or(false) {
            set_is_dirty.set(true);
            let _ = set_save_status.try_set(SaveStatus::Pending);
        }
    });

    // Salvamento Automático em Background a cada 30 segundos
    create_effect(move |_| {
        if is_loaded.get() {
            spawn_local(async move {
                loop {
                    gloo_timers::future::TimeoutFuture::new(30_000).await;
                    if is_dirty.try_get_untracked().unwrap_or(false) {
                        let current_id = id.get_untracked();
                        let current_data = data.get_untracked();
                        if !current_id.is_empty() {
                            let _ = set_save_status.try_set(SaveStatus::Saving);
                            match update_sheet(current_id.clone(), current_data).await {
                                Ok(_) => {
                                    set_is_dirty.set(false);
                                    crate::logging::log_client(
                                        "database",
                                        "INFO",
                                        "Auto-save periódico (30s) executado com sucesso",
                                        Some(&format!("id={}", current_id)),
                                    );
                                    let _ = set_save_status.try_set(SaveStatus::Saved(get_current_time_str()));
                                }
                                Err(e) => {
                                    crate::logging::log_client(
                                        "errors",
                                        "ERROR",
                                        "Falha no auto-save periódico (30s)",
                                        Some(&e.to_string()),
                                    );
                                    let _ = set_save_status.try_set(SaveStatus::Error(e.to_string()));
                                }
                            }
                        }
                    }
                }
            });
        }
    });

    // Salvamento manual ou ao sair
    let do_manual_save = move |_| {
        let current_id = id.get_untracked();
        let current_data = data.get_untracked();
        if !current_id.is_empty() {
            let _ = set_save_status.try_set(SaveStatus::Saving);
            spawn_local(async move {
                match update_sheet(current_id.clone(), current_data).await {
                    Ok(_) => {
                        set_is_dirty.set(false);
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
    };

    // Navegação ao clicar em "← Início" garantindo salvamento antes de sair
    let on_back_click = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        let current_id = id.get_untracked();
        if is_dirty.get_untracked() {
            let current_data = data.get_untracked();
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
    };

    let costs = create_memo(move |_| data.with(|d| d.calculate_costs()));
    let (show_breakdown, set_show_breakdown) = create_signal(false);

    view! {
        <link rel="stylesheet" href="/style.css"/>
        <div class="sheet-page-container">
            <header class="sheet-top-bar">
                <div class="top-bar-left">
                    <a href="/" class="back-link" on:click=on_back_click>"← Início"</a>
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
                        <button class="manual-save-btn" on:click=do_manual_save title="Salvar imediatamente">
                            "Salvar"
                        </button>
                    </div>
                </div>
            </header>

            // Modal de Extrato de Custos
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

            // Barra de Navegação de Páginas da Ficha (Abas A4)
            <nav class="sheet-tabs-container" aria-label="Páginas da Ficha">
                <div class="sheet-tabs-nav">
                    <button 
                        class="sheet-tab-btn"
                        class:active=move || active_tab.get() == SheetPageTab::Main
                        on:click=move |_| set_active_tab.set(SheetPageTab::Main)
                        title="Página 1: Atributos, Habilidades, Esferas e Vantagens"
                    >
                        <span class="sheet-tab-icon">"📜"</span>
                        <span class="sheet-tab-title">"Ficha Principal"</span>
                        <span class="sheet-tab-page-tag">"Pág. 1"</span>
                    </button>

                    <button 
                        class="sheet-tab-btn"
                        class:active=move || active_tab.get() == SheetPageTab::MagicCombat
                        on:click=move |_| set_active_tab.set(SheetPageTab::MagicCombat)
                        title="Página 2: Qualidades & Defeitos, Outros Traços, Maravilhas, Rotes e Combate"
                    >
                        <span class="sheet-tab-icon">"⚔️"</span>
                        <span class="sheet-tab-title">"Magia & Combate"</span>
                        <span class="sheet-tab-page-tag">"Pág. 2"</span>
                    </button>

                    <button 
                        class="sheet-tab-btn"
                        class:active=move || active_tab.get() == SheetPageTab::Profile
                        on:click=move |_| set_active_tab.set(SheetPageTab::Profile)
                        title="Página de Perfil: Retrato, História e Anotações Gerais"
                    >
                        <span class="sheet-tab-icon">"👤"</span>
                        <span class="sheet-tab-title">"Perfil do Personagem"</span>
                        <span class="sheet-tab-page-tag">"Perfil"</span>
                    </button>
                </div>
            </nav>

            <Suspense fallback=move || view! { <div class="loading-state"><p>"Carregando Ficha..."</p></div> }>
                {move || sheet_resource.get().map(|res| match res {
                    Ok(_) => view! {
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
                                class="sheet-page-tab-pane page-profile"
                                class:tab-hidden=move || active_tab.get() != SheetPageTab::Profile
                            >
                                <CharacterProfile />
                            </div>
                        </Sheet>
                    }.into_view(),
                    Err(e) => view! { 
                        <div class="error-container">
                            <p class="error-title">"Erro ao carregar a ficha"</p>
                            <p class="error-detail">{e.to_string()}</p>
                            <A href="/" class="back-home-btn">"Voltar para a lista de fichas"</A>
                        </div>
                    }.into_view(),
                })}
            </Suspense>
        </div>
    }
}
