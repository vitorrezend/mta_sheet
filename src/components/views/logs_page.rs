use leptos::*;
use leptos_router::*;
use crate::logging::get_system_logs;

#[component]
pub fn LogsPage() -> impl IntoView {
    let (selected_category, set_selected_category) = create_signal(Option::<String>::None);
    let (search_query, set_search_query) = create_signal(String::new());
    let (refresh_trigger, set_refresh_trigger) = create_signal(0u32);

    let logs_resource = create_local_resource(
        move || (selected_category.get(), search_query.get(), refresh_trigger.get()),
        |(cat, search, _)| async move {
            let s = if search.trim().is_empty() { None } else { Some(search) };
            get_system_logs(cat, s, Some(150)).await
        },
    );

    let do_refresh = move |_| set_refresh_trigger.update(|t| *t += 1);

    view! {
        <div class="logs-page-container">
            <header class="logs-header">
                <div class="logs-header-left">
                    <A href="/" class="back-link">"← Início"</A>
                    <h1 class="logs-title">"Painel de Logs do Sistema"</h1>
                </div>
                <div class="logs-header-right">
                    <button type="button" class="logs-refresh-btn" on:click=do_refresh>
                        "🔄 Atualizar Logs"
                    </button>
                </div>
            </header>

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
                <Suspense fallback=move || view! { <div class="loading-state"><p>"Carregando arquivos de logs..."</p></div> }>
                    {move || logs_resource.get().map(|res| match res {
                        Ok(entries) => {
                            if entries.is_empty() {
                                view! {
                                    <div class="empty-logs-msg">
                                        <p>"Nenhum registro de log encontrado para os filtros selecionados."</p>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <div class="logs-table-container">
                                        <table class="logs-table">
                                            <thead>
                                                <tr>
                                                    <th class="th-time">"Timestamp"</th>
                                                    <th class="th-cat">"Categoria / Tipo"</th>
                                                    <th class="th-lvl">"Nível"</th>
                                                    <th class="th-msg">"Mensagem / Rota"</th>
                                                    <th class="th-details">"Detalhes do Cliente & Auditoria"</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {entries.into_iter().map(|entry| {
                                                    let lvl_class = match entry.level.to_uppercase().as_str() {
                                                        "ERROR" => "lvl-error",
                                                        "WARN" => "lvl-warn",
                                                        _ => "lvl-info",
                                                    };
                                                    let is_access = entry.category == "access";
                                                    let details_text = entry.details.clone().unwrap_or_default();
                                                    
                                                    let client_badge = if is_access {
                                                        if details_text.contains("type:human") {
                                                            Some(("badge-human", "🟢 HUMANO"))
                                                        } else if details_text.contains("type:crawler") {
                                                            Some(("badge-crawler", "🤖 CRAWLER"))
                                                        } else if details_text.contains("type:bot") {
                                                            Some(("badge-bot", "🔴 MÁQUINA / BOT"))
                                                        } else if details_text.contains("type:suspicious") {
                                                            Some(("badge-suspicious", "⚠️ SUSPEITO"))
                                                        } else {
                                                            None
                                                        }
                                                    } else {
                                                        None
                                                    };

                                                    let cat_badge_class = match entry.category.as_str() {
                                                        "access" => "cat-access",
                                                        "database" => "cat-database",
                                                        "user_actions" => "cat-user-actions",
                                                        "requests" => "cat-requests",
                                                        "errors" => "cat-errors",
                                                        _ => "cat-default",
                                                    };
                                                    view! {
                                                        <tr class="log-row">
                                                            <td class="log-cell-time font-mono">{entry.timestamp}</td>
                                                            <td>
                                                                {if let Some((b_class, b_label)) = client_badge {
                                                                    view! {
                                                                        <span class=b_class>{b_label}</span>
                                                                    }.into_view()
                                                                } else {
                                                                    view! {
                                                                        <span class=format!("log-cat-badge {}", cat_badge_class)>
                                                                            {entry.category}
                                                                        </span>
                                                                    }.into_view()
                                                                }}
                                                            </td>
                                                            <td>
                                                                <span class=format!("log-lvl-badge {}", lvl_class)>
                                                                    {entry.level}
                                                                </span>
                                                            </td>
                                                            <td class="log-cell-msg">{entry.message}</td>
                                                            <td class="log-cell-details font-mono">
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
                                                <p style="color: #64748b; margin-bottom: 1.5rem; font-size: 0.9rem;">"Você precisa estar autenticado como Administrador para visualizar os logs de auditoria do sistema."</p>
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
    }
}
