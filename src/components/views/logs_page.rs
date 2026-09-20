use leptos::*;
use leptos_router::*;
use crate::logging::get_system_logs;

#[component]
pub fn LogsPage() -> impl IntoView {
    let (admin_tab, set_admin_tab) = create_signal("logs".to_string());
    let (selected_category, set_selected_category) = create_signal(Option::<String>::None);
    let (search_query, set_search_query) = create_signal(String::new());
    let (refresh_trigger, set_refresh_trigger) = create_signal(0u32);
    let (feature_msg, set_feature_msg) = create_signal(Option::<String>::None);
    let (is_saving_flag, set_is_saving_flag) = create_signal(false);

    let logs_resource = create_local_resource(
        move || (selected_category.get(), search_query.get(), refresh_trigger.get()),
        |(cat, search, _)| async move {
            let s = if search.trim().is_empty() { None } else { Some(search) };
            get_system_logs(cat, s, Some(150)).await
        },
    );

    let flags_resource = create_local_resource(
        move || refresh_trigger.get(),
        |_| async move { crate::settings::get_system_feature_flags().await },
    );

    let do_refresh = move |_| set_refresh_trigger.update(|t| *t += 1);

    let on_toggle_flag = move |key: String, val: String| {
        set_is_saving_flag.set(true);
        set_feature_msg.set(None);
        spawn_local(async move {
            match crate::settings::update_system_feature_flag(key, val).await {
                Ok(_) => {
                    set_is_saving_flag.set(false);
                    set_feature_msg.set(Some("✓ Configuração de recurso atualizada com sucesso!".to_string()));
                    set_refresh_trigger.update(|t| *t += 1);
                }
                Err(e) => {
                    set_is_saving_flag.set(false);
                    set_feature_msg.set(Some(format!("❌ Erro ao atualizar recurso: {}", e)));
                }
            }
        });
    };

    view! {
        <div class="logs-page-container">
            <header class="logs-header">
                <div class="logs-header-left">
                    <A href="/" class="back-link">"← Início"</A>
                    <h1 class="logs-title">"Painel de Administração do Sistema"</h1>
                </div>
                <div class="logs-header-right">
                    <button type="button" class="logs-refresh-btn" on:click=do_refresh>
                        "🔄 Atualizar"
                    </button>
                </div>
            </header>

            // Alternador de Seções de Administração
            <div class="admin-nav-tabs-bar">
                <button
                    type="button"
                    class="admin-nav-tab-btn"
                    class:active=move || admin_tab.get() == "logs"
                    on:click=move |_| set_admin_tab.set("logs".to_string())
                >
                    "🛡️ Auditoria & Logs"
                </button>
                <button
                    type="button"
                    class="admin-nav-tab-btn"
                    class:active=move || admin_tab.get() == "features"
                    on:click=move |_| set_admin_tab.set("features".to_string())
                >
                    "🎛️ Recursos & Feature Flags"
                </button>
            </div>

            {move || if admin_tab.get() == "features" {
                view! {
                    <div class="admin-features-panel">
                        <div class="features-intro-card">
                            <h2>"Gerenciamento de Módulos & Recursos (Feature Flags)"</h2>
                            <p>"Controle em tempo real a visibilidade e o acesso a funcionalidades da aplicação em todas as salas, sem necessidade de reinicialização."</p>
                        </div>

                        {move || feature_msg.get().map(|msg| {
                            let is_err = msg.starts_with('❌');
                            view! {
                                <div class="alert-box" class:alert-error=is_err class:alert-success=!is_err>
                                    <span>{msg}</span>
                                    <button class="alert-close" on:click=move |_| set_feature_msg.set(None)>"×"</button>
                                </div>
                            }
                        })}

                        <div class="features-list-wrapper">
                            <Suspense fallback=move || view! { <p class="loading-msg">"Carregando módulos do sistema..."</p> }>
                                {move || flags_resource.get().map(|res| match res {
                                    Ok(flags) => view! {
                                        <div class="features-grid">
                                            {flags.into_iter().map(|flag| {
                                                let f_key = flag.key.clone();
                                                let f_val = flag.value.clone();
                                                let f_desc = flag.description.clone();
                                                
                                                let k1 = f_key.clone();
                                                let k2 = f_key.clone();
                                                let k3 = f_key.clone();

                                                let val_curr = f_val.clone();
                                                let (badge_class, badge_text) = match f_val.as_str() {
                                                    "disabled" => ("badge-disabled", "🔴 Desativado (Ninguém Vê)"),
                                                    "admin_only" => ("badge-admin", "🟡 Apenas Administradores (Modo Dev / Testes)"),
                                                    "enabled" => ("badge-enabled", "🟢 Liberado para Todos (Público)"),
                                                    _ => ("badge-unknown", "Desconhecido"),
                                                };

                                                view! {
                                                    <div class="feature-card">
                                                        <div class="feature-card-header">
                                                            <div class="feature-card-title-group">
                                                                <h3 class="feature-title">{f_desc}</h3>
                                                                <code class="feature-code-key">{f_key}</code>
                                                            </div>
                                                            <span class=format!("feature-status-badge {}", badge_class)>{badge_text}</span>
                                                        </div>
                                                        <div class="feature-card-body">
                                                            <p class="feature-explanation">
                                                                {match val_curr.as_str() {
                                                                    "disabled" => "O recurso está oculto em todas as salas e suas APIs de salvamento estão estritamente bloqueadas.",
                                                                    "admin_only" => "Apenas contas de Administrador conseguem visualizar e interagir com o recurso nas salas. Jogadores e narradores comuns não têm acesso.",
                                                                    "enabled" => "O recurso está ativo e acessível para todos os jogadores e narradores em todas as salas.",
                                                                    _ => "",
                                                                }}
                                                            </p>
                                                            <div class="feature-actions-row">
                                                                <span class="feature-actions-label">"Alterar Estado:"</span>
                                                                <div class="feature-btn-group">
                                                                    <button
                                                                        type="button"
                                                                        class="feature-toggle-btn btn-state-disabled"
                                                                        class:active=val_curr == "disabled"
                                                                        disabled=move || is_saving_flag.get()
                                                                        on:click=move |_| on_toggle_flag(k1.clone(), "disabled".to_string())
                                                                    >
                                                                        "🔴 Desativar"
                                                                    </button>
                                                                    <button
                                                                        type="button"
                                                                        class="feature-toggle-btn btn-state-admin"
                                                                        class:active=val_curr == "admin_only"
                                                                        disabled=move || is_saving_flag.get()
                                                                        on:click=move |_| on_toggle_flag(k2.clone(), "admin_only".to_string())
                                                                    >
                                                                        "🟡 Modo Admin (Dev)"
                                                                    </button>
                                                                    <button
                                                                        type="button"
                                                                        class="feature-toggle-btn btn-state-enabled"
                                                                        class:active=val_curr == "enabled"
                                                                        disabled=move || is_saving_flag.get()
                                                                        on:click=move |_| on_toggle_flag(k3.clone(), "enabled".to_string())
                                                                    >
                                                                        "🟢 Liberar Geral"
                                                                    </button>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_view(),
                                    Err(e) => view! {
                                        <div class="alert-box alert-error">
                                            <p>"Erro ao carregar configurações: " {e.to_string()}</p>
                                        </div>
                                    }.into_view(),
                                })}
                            </Suspense>
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {
                    <div>
                        // Barra de Filtros e Busca
                        <div class="logs-controls-bar">
                            <div class="logs-category-tabs">
                                <button 
                                    type="button" 
                                    class="log-tab-btn" 
                                    class:active=move || selected_category.get().is_none()
                                    on:click=move |_| set_selected_category.set(None)
                                >
                                    "📋 Todos"
                                </button>
                                <button 
                                    type="button" 
                                    class="log-tab-btn" 
                                    class:active=move || selected_category.get().as_deref() == Some("access")
                                    on:click=move |_| set_selected_category.set(Some("access".to_string()))
                                >
                                    "🛡️ Acessos (Bot vs Humano)"
                                </button>
                                <button 
                                    type="button" 
                                    class="log-tab-btn" 
                                    class:active=move || selected_category.get().as_deref() == Some("requests")
                                    on:click=move |_| set_selected_category.set(Some("requests".to_string()))
                                >
                                    "🌐 Requisições"
                                </button>
                                <button 
                                    type="button" 
                                    class="log-tab-btn" 
                                    class:active=move || selected_category.get().as_deref() == Some("database")
                                    on:click=move |_| set_selected_category.set(Some("database".to_string()))
                                >
                                    "💾 Banco de Dados"
                                </button>
                                <button 
                                    type="button" 
                                    class="log-tab-btn" 
                                    class:active=move || selected_category.get().as_deref() == Some("user_actions")
                                    on:click=move |_| set_selected_category.set(Some("user_actions".to_string()))
                                >
                                    "👤 Interações"
                                </button>
                                <button 
                                    type="button" 
                                    class="log-tab-btn tab-error" 
                                    class:active=move || selected_category.get().as_deref() == Some("errors")
                                    on:click=move |_| set_selected_category.set(Some("errors".to_string()))
                                >
                                    "⚠️ Erros"
                                </button>
                            </div>

                            <div class="logs-search-box">
                                <input 
                                    type="text" 
                                    class="logs-search-input" 
                                    placeholder="Filtrar logs por texto, IP ou tipo..." 
                                    prop:value=search_query
                                    on:input=move |ev| set_search_query.set(event_target_value(&ev))
                                />
                            </div>
                        </div>

                        // Tabela de Logs
                        <div class="logs-content-wrapper">
                            <Suspense fallback=move || view! { <div class="logs-loading"><p>"Carregando auditoria do sistema..."</p></div> }>
                                {move || logs_resource.get().map(|res| match res {
                                    Ok(logs) => {
                                        if logs.is_empty() {
                                            view! {
                                                <div class="empty-logs">
                                                    <p>"Nenhum log encontrado para o filtro selecionado."</p>
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! {
                                                <div class="table-responsive">
                                                    <table class="logs-table">
                                                        <thead>
                                                            <tr>
                                                                <th>"Data/Hora"</th>
                                                                <th>"Nível"</th>
                                                                <th>"Categoria"</th>
                                                                <th>"Mensagem"</th>
                                                                <th>"Detalhes Técnicos"</th>
                                                            </tr>
                                                        </thead>
                                                        <tbody>
                                                            {logs.into_iter().map(|entry| {
                                                                let lvl_class = format!("level-badge level-{}", entry.level.to_lowercase());
                                                                let cat_class = format!("cat-badge cat-{}", entry.category.to_lowercase());
                                                                view! {
                                                                    <tr>
                                                                        <td class="col-timestamp">{entry.timestamp}</td>
                                                                        <td class="col-level"><span class=lvl_class>{entry.level}</span></td>
                                                                        <td class="col-cat"><span class=cat_class>{entry.category}</span></td>
                                                                        <td class="col-msg">{entry.message}</td>
                                                                        <td class="col-details">
                                                                            {entry.details.unwrap_or_else(|| "-".to_string())}
                                                                        </td>
                                                                    </tr>
                                                                }
                                                            }).collect_view()}
                                                        </tbody>
                                                    </table>
                                                </div>
                                            }.into_view()
                                        }
                                    }
                                    Err(e) => {
                                        let err_msg = e.to_string();
                                        let is_unauthorized = err_msg.contains("Acesso negado") || err_msg.contains("administrador");
                                        view! {
                                            <div class="error-container">
                                                {if is_unauthorized {
                                                    view! {
                                                        <div class="unauthorized-box" style="text-align: center; padding: 2.5rem 1.5rem; background: white; border-radius: 8px; border: 1px solid #fee2e2;">
                                                            <div style="font-size: 2.5rem; margin-bottom: 0.75rem;">"🔒"</div>
                                                            <h2 style="color: #991b1b; margin-bottom: 0.5rem; font-size: 1.25rem;">"Acesso Restrito a Administradores"</h2>
                                                            <p style="color: #64748b; margin-bottom: 1.5rem; font-size: 0.9rem;">"Você precisa estar autenticado como Administrador para visualizar as opções de auditoria e módulos do sistema."</p>
                                                            <div style="display: flex; gap: 0.75rem; justify-content: center;">
                                                                <a href="/login" style="padding: 0.5rem 1.2rem; background: #2563eb; color: white; border-radius: 6px; text-decoration: none; font-weight: 600; font-size: 0.85rem;">"🔑 Fazer Login"</a>
                                                                <a href="/" style="padding: 0.5rem 1.2rem; background: #f1f5f9; color: #475569; border-radius: 6px; text-decoration: none; font-weight: 600; font-size: 0.85rem;">"← Voltar ao Início"</a>
                                                            </div>
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    view! {
                                                        <div>
                                                            <p class="error-title">"Erro ao ler arquivos de log"</p>
                                                            <p class="error-detail">{err_msg}</p>
                                                        </div>
                                                    }.into_view()
                                                }}
                                            </div>
                                        }.into_view()
                                    }
                                })}
                            </Suspense>
                        </div>
                    </div>
                }.into_view()
            }}
        </div>
    }
}
