use leptos::*;
use leptos_router::*;
use crate::state::{clone_sheet, get_user_profile, CharacterSummary};

fn handle_clone_sheet(
    sheet_id: String,
    cloning_id: ReadSignal<Option<String>>,
    set_cloning_id: WriteSignal<Option<String>>,
    log_source: &'static str,
) {
    if cloning_id.get().is_some() {
        return;
    }
    let navigate = use_navigate();
    set_cloning_id.set(Some(sheet_id.clone()));
    spawn_local(async move {
        match clone_sheet(sheet_id, None).await {
            Ok(new_id) => {
                crate::logging::log_client(
                    "user_actions",
                    "INFO",
                    &format!("Ficha clonada via {}", log_source),
                    Some(&format!("new_id={}", new_id)),
                );
                navigate(&format!("/sheet/{}", new_id), Default::default());
            }
            Err(e) => {
                crate::logging::log_client(
                    "errors",
                    "ERROR",
                    &format!("Erro ao clonar ficha ({})", log_source),
                    Some(&e.to_string()),
                );
                set_cloning_id.set(None);
            }
        }
    });
}

#[component]
pub fn ProfilePage() -> impl IntoView {
    let params = use_params_map();
    let target_username = move || params.with(|p| p.get("username").cloned());

    let profile_resource = create_local_resource(target_username, |user_opt| async move {
        get_user_profile(user_opt).await
    });

    let (active_tab, set_active_tab) = create_signal("sheets");
    let (search_filter, set_search_filter) = create_signal(String::new());
    let (cloning_id, set_cloning_id) = create_signal(Option::<String>::None);

    // Sinais da Central de Segurança & Troca de Senha
    let (current_pwd, set_current_pwd) = create_signal(String::new());
    let (new_pwd, set_new_pwd) = create_signal(String::new());
    let (confirm_pwd, set_confirm_pwd) = create_signal(String::new());
    let (show_current_pwd, set_show_current_pwd) = create_signal(false);
    let (show_new_pwd, set_show_new_pwd) = create_signal(false);
    let (show_confirm_pwd, set_show_confirm_pwd) = create_signal(false);
    let (is_changing_pwd, set_is_changing_pwd) = create_signal(false);
    let (pwd_feedback, set_pwd_feedback) = create_signal(Option::<(bool, String)>::None);
    let (is_revoking_sessions, set_is_revoking_sessions) = create_signal(false);
    let (sessions_feedback, set_sessions_feedback) = create_signal(Option::<String>::None);
    let (copied_id, set_copied_id) = create_signal(false);

    // Medidor de Força de Senha em Tempo Real
    let pwd_strength = move || {
        let p = new_pwd.get();
        if p.is_empty() {
            return (0, "", "");
        }
        let mut score = 0;
        if p.len() >= 8 { score += 1; }
        if p.len() >= 12 { score += 1; }
        if p.chars().any(|c| c.is_ascii_lowercase()) && p.chars().any(|c| c.is_ascii_uppercase()) { score += 1; }
        if p.chars().any(|c| c.is_numeric()) { score += 1; }
        if p.chars().any(|c| !c.is_alphanumeric()) { score += 1; }

        match score {
            0..=2 => (1, "Fraca", "strength-weak"),
            3..=4 => (2, "Média", "strength-medium"),
            _ => (3, "Forte", "strength-strong"),
        }
    };

    view! {
        <div class="profile-container">
            <leptos_meta::Title text=move || {
                let target = target_username();
                match target {
                    Some(u) => format!("Perfil de @{} | MTA Sheet", u),
                    None => "Meu Perfil | MTA Sheet".to_string(),
                }
            } />

            <Transition fallback=move || view! { <div class="feed-empty-state"><p>"Carregando perfil..."</p></div> }>
                {move || {
                    let profile_data = match profile_resource.get() {
                        Some(Ok(data)) => data,
                        Some(Err(e)) => return view! {
                            <div class="feed-empty-state">
                                <span class="feed-empty-icon">"⚠️"</span>
                                <h3>"Erro ao carregar perfil"</h3>
                                <p>{e.to_string()}</p>
                                <A href="/" class="profile-cta-btn profile-cta-primary">
                                    "Voltar para o Início"
                                </A>
                            </div>
                        }.into_view(),
                        None => return view! {
                            <div class="feed-empty-state"><p>"Carregando dados do usuário..."</p></div>
                        }.into_view(),
                    };

                    let profile_data = std::rc::Rc::new(profile_data);
                    let is_self = profile_data.is_self;
                    let is_admin = profile_data.is_admin;
                    let user_id = profile_data.id.clone();
                    let user_id_copy = user_id.clone();
                    let _ = &user_id_copy;
                    let username_display = profile_data.username.clone();
                    let member_since = if profile_data.created_at.len() >= 10 {
                        profile_data.created_at[..10].to_string()
                    } else {
                        profile_data.created_at.clone()
                    };

                    let on_copy_id = move |_| {
                        #[cfg(target_arch = "wasm32")]
                        {
                            if let Some(w) = web_sys::window() {
                                let _ = w.navigator().clipboard().write_text(&user_id_copy);
                            }
                        }
                        set_copied_id.set(true);
                    };

                    let on_change_password_submit = move |ev: ev::SubmitEvent| {
                        ev.prevent_default();
                        let cur = current_pwd.get();
                        let new_p = new_pwd.get();
                        let conf = confirm_pwd.get();

                        if cur.is_empty() || new_p.is_empty() || conf.is_empty() {
                            set_pwd_feedback.set(Some((false, "Todos os campos de senha são obrigatórios.".to_string())));
                            return;
                        }
                        if new_p != conf {
                            set_pwd_feedback.set(Some((false, "A confirmação não coincide com a nova senha.".to_string())));
                            return;
                        }
                        if let Err(err) = crate::auth::validate_password_strength(&new_p) {
                            set_pwd_feedback.set(Some((false, err.to_string())));
                            return;
                        }
                        if cur == new_p {
                            set_pwd_feedback.set(Some((false, "A nova senha deve ser diferente da senha atual.".to_string())));
                            return;
                        }

                        set_is_changing_pwd.set(true);
                        set_pwd_feedback.set(None);

                        spawn_local(async move {
                            match crate::auth::change_password(cur, new_p, conf).await {
                                Ok(_) => {
                                    set_is_changing_pwd.set(false);
                                    set_current_pwd.set(String::new());
                                    set_new_pwd.set(String::new());
                                    set_confirm_pwd.set(String::new());
                                    set_pwd_feedback.set(Some((true, "Senha alterada com sucesso! Todas as outras sessões ativas foram desconectadas por segurança.".to_string())));
                                    profile_resource.refetch();
                                }
                                Err(e) => {
                                    set_is_changing_pwd.set(false);
                                    set_pwd_feedback.set(Some((false, e.to_string())));
                                }
                            }
                        });
                    };

                    let on_revoke_other_sessions = move |_| {
                        set_is_revoking_sessions.set(true);
                        set_sessions_feedback.set(None);
                        spawn_local(async move {
                            match crate::auth::revoke_other_sessions().await {
                                Ok(count) => {
                                    set_is_revoking_sessions.set(false);
                                    set_sessions_feedback.set(Some(format!("{} outro(s) dispositivo(s) desconectado(s) com sucesso!", count)));
                                    profile_resource.refetch();
                                }
                                Err(e) => {
                                    set_is_revoking_sessions.set(false);
                                    set_sessions_feedback.set(Some(format!("Erro ao desconectar sessões: {}", e)));
                                }
                            }
                        });
                    };

                    view! {
                        <div>
                            <div class="page-breadcrumb-nav">
                                <A href="/" class="breadcrumb-back-link">
                                    "← Voltar para Minhas Fichas"
                                </A>
                                <span class="breadcrumb-separator">"•"</span>
                                <span class="breadcrumb-current">{format!("Perfil de @{}", username_display)}</span>
                            </div>

                            // Header do Perfil
                            <header class="profile-header-card">
                                <div class="profile-user-info-group">
                                    <div class="profile-avatar-box">
                                        "🧙"
                                    </div>
                                    <div class="profile-user-details">
                                        <div class="profile-username-row">
                                            <h1 class="profile-username">{format!("@{}", username_display)}</h1>
                                            {if is_admin {
                                                view! { <span class="profile-badge-admin">"👑 Administrador"</span> }.into_view()
                                            } else if is_self {
                                                view! { <span class="profile-badge-self">"Você"</span> }.into_view()
                                            } else {
                                                view! { <span class="profile-badge-member">"Membro da Comunidade"</span> }.into_view()
                                            }}
                                        </div>
                                        <div class="profile-user-id-row">
                                            <span>"ID: "</span>
                                            <span class="profile-user-id-badge">
                                                {if user_id.len() > 13 { format!("{}...", &user_id[..10]) } else { user_id.clone() }}
                                            </span>
                                            <button
                                                type="button"
                                                class="profile-copy-id-btn"
                                                on:click=on_copy_id
                                                title="Copiar ID de usuário"
                                            >
                                                {move || if copied_id.get() { "✓ Copiado" } else { "📋 Copiar" }}
                                            </button>
                                            <span>" • "</span>
                                            <span class="profile-member-since">
                                                {format!("Membro desde: {}", member_since)}
                                            </span>
                                        </div>
                                    </div>
                                </div>

                                <div class="profile-header-actions">
                                    {if is_self {
                                        view! {
                                            <div class="profile-actions-inner">
                                                <A href="/" class="profile-cta-btn profile-cta-primary">
                                                    "➕ Minhas Fichas"
                                                </A>
                                                <A href="/feed" class="profile-cta-btn profile-cta-secondary">
                                                    "🌐 Explorar Feed"
                                                </A>
                                                <button
                                                    type="button"
                                                    class="profile-logout-btn"
                                                    on:click=move |_| {
                                                        let nav = use_navigate();
                                                        spawn_local(async move {
                                                            let _ = crate::auth::logout().await;
                                                            nav("/login", Default::default());
                                                        });
                                                    }
                                                    title="Sair da conta"
                                                >
                                                    "🚪 Sair"
                                                </button>
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! {
                                            <div class="profile-actions-inner">
                                                <A href="/feed" class="profile-cta-btn profile-cta-secondary">
                                                    "🌐 Ver Mais no Feed"
                                                </A>
                                            </div>
                                        }.into_view()
                                    }}
                                </div>
                            </header>

                            // Navegação por Abas (Exclusivo para o próprio usuário)
                            {if is_self {
                                view! {
                                    <nav class="profile-tabs-nav">
                                        <button
                                            type="button"
                                            class="profile-tab-btn"
                                            class:active=move || active_tab.get() == "sheets"
                                            on:click=move |_| set_active_tab.set("sheets")
                                        >
                                            "📜 Grimório & Fichas (" {profile_data.total_sheets} ")"
                                        </button>
                                        <button
                                            type="button"
                                            class="profile-tab-btn"
                                            class:active=move || active_tab.get() == "stats"
                                            on:click=move |_| set_active_tab.set("stats")
                                        >
                                            "📊 Estatísticas & Crônicas"
                                        </button>
                                        <button
                                            type="button"
                                            class="profile-tab-btn"
                                            class:active=move || active_tab.get() == "security"
                                            on:click=move |_| set_active_tab.set("security")
                                        >
                                            "🛡️ Central de Segurança"
                                        </button>
                                    </nav>
                                }.into_view()
                            } else {
                                view! { <div></div> }.into_view()
                            }}

                            // Conteúdo das Abas
                            {
                                let pd = std::rc::Rc::clone(&profile_data);
                                move || match active_tab.get() {
                                    "stats" if is_self => view! {
                                        <div class="profile-analytics-grid">
                                            <div class="analytics-card">
                                                <div class="analytics-card-header">
                                                    <span class="analytics-card-icon">"🔮"</span>
                                                    <h3 class="analytics-card-title">"Sistemas & Splats"</h3>
                                                </div>
                                                <div class="analytics-breakdown-list">
                                                    <div class="breakdown-row">
                                                        <span>"Mago: A Ascensão (M20):"</span>
                                                        <strong>{pd.mage_sheets_count}</strong>
                                                    </div>
                                                    <div class="breakdown-row">
                                                        <span>"Deuses & Monstros:"</span>
                                                        <strong>{pd.gods_monsters_sheets_count}</strong>
                                                    </div>
                                                    <div class="breakdown-row">
                                                        <span>"Total de Personagens:"</span>
                                                        <strong>{pd.total_sheets}</strong>
                                                    </div>
                                                </div>
                                            </div>

                                            <div class="analytics-card">
                                                <div class="analytics-card-header">
                                                    <span class="analytics-card-icon">"🌐"</span>
                                                    <h3 class="analytics-card-title">"Privacidade & Arquivo"</h3>
                                                </div>
                                                <div class="analytics-breakdown-list">
                                                    <div class="breakdown-row">
                                                        <span>"Fichas Públicas (Comunidade):"</span>
                                                        <strong>{pd.public_sheets_count}</strong>
                                                    </div>
                                                    <div class="breakdown-row">
                                                        <span>"Fichas Privadas (Pessoais):"</span>
                                                        <strong>{pd.private_sheets_count}</strong>
                                                    </div>
                                                    <div class="breakdown-row">
                                                        <span>"Pastas no Grimório:"</span>
                                                        <strong>{pd.folders_count}</strong>
                                                    </div>
                                                </div>
                                            </div>

                                            <div class="analytics-card">
                                                <div class="analytics-card-header">
                                                    <span class="analytics-card-icon">"🏰"</span>
                                                    <h3 class="analytics-card-title">"Crônicas & Mesas"</h3>
                                                </div>
                                                <div class="analytics-breakdown-list">
                                                    <div class="breakdown-row">
                                                        <span>"Como Narrador / Mestre:"</span>
                                                        <strong>{pd.gm_rooms_count}</strong>
                                                    </div>
                                                    <div class="breakdown-row">
                                                        <span>"Como Jogador da Cabala:"</span>
                                                        <strong>{pd.player_rooms_count}</strong>
                                                    </div>
                                                    <div class="breakdown-row">
                                                        <span>"Total de Salas Participantes:"</span>
                                                        <strong>{pd.rooms_count}</strong>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    }.into_view(),

                                    "security" if is_self => view! {
                                        <div class="security-panel">
                                            // Card de Status da Conta
                                            <div class="security-card">
                                                <div class="security-card-header">
                                                    <h3 class="security-card-title">"🛡️ Status & Credenciais da Conta"</h3>
                                                    <p class="security-card-desc">"Gerenciamento de conexões, criptografia e proteção da sua conta."</p>
                                                </div>

                                                <div class="security-status-grid">
                                                    <div class="security-status-box">
                                                        <span class="security-status-label">"Criptografia de Senha"</span>
                                                        <span class="security-status-val">"Bcrypt (Custo 12 • Alta Segurança)"</span>
                                                    </div>
                                                    <div class="security-status-box">
                                                        <span class="security-status-label">"Proteção de Sessão"</span>
                                                        <span class="security-status-val">"Cookies Criptografados (HttpOnly • SameSite)"</span>
                                                    </div>
                                                    <div class="security-status-box">
                                                        <span class="security-status-label">"Dispositivos Conectados"</span>
                                                        <span class="security-status-val">{format!("{} sessão(ões) ativa(s)", pd.active_sessions_count)}</span>
                                                    </div>
                                                </div>

                                            {move || sessions_feedback.get().map(|msg| view! {
                                                <div class="security-alert success" style="margin-bottom: 1rem;">
                                                    <span>"✓ " {msg}</span>
                                                </div>
                                            })}

                                            <button
                                                type="button"
                                                class="revoke-sessions-btn"
                                                disabled=move || is_revoking_sessions.get()
                                                on:click=on_revoke_other_sessions
                                                title="Derrubar logins em outros computadores ou navegadores"
                                            >
                                                {move || if is_revoking_sessions.get() { "⏳ Desconectando..." } else { "🔌 Desconectar Outros Dispositivos" }}
                                            </button>
                                        </div>

                                        // Card de Alteração de Senha
                                        <div class="security-card">
                                            <div class="security-card-header">
                                                <h3 class="security-card-title">"🔑 Alteração de Senha Segura"</h3>
                                                <p class="security-card-desc">"Atualize sua senha seguindo os protocolos de complexidade. Suas outras sessões serão automaticamente encerradas por segurança."</p>
                                            </div>

                                            {move || pwd_feedback.get().map(|(ok, msg)| view! {
                                                <div class=format!("security-alert {}", if ok { "success" } else { "error" }) style="margin-bottom: 1rem;">
                                                    <span>{if ok { "✓ " } else { "⚠️ " }} {msg}</span>
                                                </div>
                                            })}

                                            <form class="security-password-form" on:submit=on_change_password_submit>
                                                <div class="security-form-group">
                                                    <label class="security-form-label">"Senha Atual:"</label>
                                                    <div class="password-input-wrapper">
                                                        <input
                                                            type=move || if show_current_pwd.get() { "text" } else { "password" }
                                                            placeholder="Digite sua senha atual"
                                                            prop:value=move || current_pwd.get()
                                                            on:input=move |ev| set_current_pwd.set(event_target_value(&ev))
                                                            required
                                                        />
                                                        <button
                                                            type="button"
                                                            class="pwd-toggle-btn"
                                                            on:click=move |_| set_show_current_pwd.update(|v| *v = !*v)
                                                            title="Mostrar/Ocultar"
                                                        >
                                                            {move || if show_current_pwd.get() { "🙈" } else { "👁️" }}
                                                        </button>
                                                    </div>
                                                </div>

                                                <div class="security-form-group">
                                                    <label class="security-form-label">"Nova Senha:"</label>
                                                    <div class="password-input-wrapper">
                                                        <input
                                                            type=move || if show_new_pwd.get() { "text" } else { "password" }
                                                            placeholder="Mínimo 8 caracteres (letras e números)"
                                                            prop:value=move || new_pwd.get()
                                                            on:input=move |ev| set_new_pwd.set(event_target_value(&ev))
                                                            required
                                                        />
                                                        <button
                                                            type="button"
                                                            class="pwd-toggle-btn"
                                                            on:click=move |_| set_show_new_pwd.update(|v| *v = !*v)
                                                            title="Mostrar/Ocultar"
                                                        >
                                                            {move || if show_new_pwd.get() { "🙈" } else { "👁️" }}
                                                        </button>
                                                    </div>

                                                    // Medidor de Força
                                                    {move || {
                                                        let (_, label, class_name) = pwd_strength();
                                                        if !new_pwd.get().is_empty() {
                                                            view! {
                                                                <div class="password-strength-container">
                                                                    <div class="strength-bar-track">
                                                                        <div class=format!("strength-bar-fill {}", class_name)></div>
                                                                    </div>
                                                                    <div class="strength-meta-row">
                                                                        <span>"Força da Senha: " <strong class=format!("strength-label {}", class_name)>{label}</strong></span>
                                                                    </div>
                                                                </div>
                                                            }.into_view()
                                                        } else {
                                                            view! { <div></div> }.into_view()
                                                        }
                                                    }}

                                                    // Checklist de Requisitos
                                                    <div class="pwd-checklist">
                                                        <span class="pwd-check-item" class:valid=move || (new_pwd.get().len() >= 8)>
                                                            {move || if new_pwd.get().len() >= 8 { "✓ 8+ caracteres" } else { "○ 8+ caracteres" }}
                                                        </span>
                                                        <span class="pwd-check-item" class:valid=move || new_pwd.get().chars().any(|c| c.is_alphabetic())>
                                                            {move || if new_pwd.get().chars().any(|c| c.is_alphabetic()) { "✓ Letra" } else { "○ Letra" }}
                                                        </span>
                                                        <span class="pwd-check-item" class:valid=move || new_pwd.get().chars().any(|c| c.is_numeric())>
                                                            {move || if new_pwd.get().chars().any(|c| c.is_numeric()) { "✓ Número" } else { "○ Número" }}
                                                        </span>
                                                    </div>
                                                </div>

                                                <div class="security-form-group">
                                                    <label class="security-form-label">"Confirmar Nova Senha:"</label>
                                                    <div class="password-input-wrapper">
                                                        <input
                                                            type=move || if show_confirm_pwd.get() { "text" } else { "password" }
                                                            placeholder="Repita a nova senha"
                                                            prop:value=move || confirm_pwd.get()
                                                            on:input=move |ev| set_confirm_pwd.set(event_target_value(&ev))
                                                            required
                                                        />
                                                        <button
                                                            type="button"
                                                            class="pwd-toggle-btn"
                                                            on:click=move |_| set_show_confirm_pwd.update(|v| *v = !*v)
                                                            title="Mostrar/Ocultar"
                                                        >
                                                            {move || if show_confirm_pwd.get() { "🙈" } else { "👁️" }}
                                                        </button>
                                                    </div>
                                                </div>

                                                <button
                                                    type="submit"
                                                    class="security-submit-btn"
                                                    disabled=move || is_changing_pwd.get() || new_pwd.get().is_empty() || new_pwd.get() != confirm_pwd.get()
                                                >
                                                    {move || if is_changing_pwd.get() { "⏳ Atualizando..." } else { "🔐 Atualizar Senha" }}
                                                </button>
                                            </form>
                                        </div>
                                    </div>
                                }.into_view(),

                                _ => {
                                    let pd_inner = std::rc::Rc::clone(&pd);
                                    view! {
                                        <section class="profile-sheets-section">
                                            <div class="profile-section-header">
                                                <h2 class="profile-section-title">
                                                    <span>"📂"</span>
                                                    <span>
                                                        {if is_self {
                                                            "Seu Grimório de Personagens"
                                                        } else {
                                                            "Personagens Públicos do Usuário"
                                                        }}
                                                    </span>
                                                </h2>

                                                <div class="profile-search-filter-row">
                                                    <div class="profile-search-box">
                                                        <span class="profile-search-icon">"🔍"</span>
                                                        <input
                                                            type="text"
                                                            class="profile-filter-input"
                                                            placeholder="Filtrar por nome, essência ou tradição..."
                                                            prop:value=move || search_filter.get()
                                                            on:input=move |ev| set_search_filter.set(event_target_value(&ev))
                                                        />
                                                    </div>
                                                </div>
                                            </div>

                                            {let pd_filter = std::rc::Rc::clone(&pd_inner);
                                            move || {
                                                let query = search_filter.get().trim().to_lowercase();
                                                let filtered: Vec<CharacterSummary> = if query.is_empty() {
                                                    pd_filter.sheets.clone()
                                                } else {
                                                    pd_filter.sheets.iter().filter(|s| {
                                                        s.name.to_lowercase().contains(&query)
                                                            || s.tradition.to_lowercase().contains(&query)
                                                            || s.essence.to_lowercase().contains(&query)
                                                    }).cloned().collect()
                                                };

                                                if filtered.is_empty() {
                                                    view! {
                                                        <div class="feed-empty-state">
                                                            <span class="feed-empty-icon">"📜"</span>
                                                            <p>
                                                                {if is_self {
                                                                    "Você ainda não possui personagens cadastrados."
                                                                } else {
                                                                    "Este usuário ainda não publicou nenhuma ficha pública."
                                                                }}
                                                            </p>
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    view! {
                                                        <div class="feed-cards-grid">
                                                            {filtered.into_iter().map(|character| {
                                                                let is_gm = character.sheet_type == "gods_and_monsters" || character.sheet_type == "gm";
                                                                let splat_badge = if is_gm { "🐉 Deuses & Monstros" } else { "🔮 Mago M20" };
                                                                let badge_class = if is_gm { "badge-gm" } else { "badge-mage" };
                                                                let id_for_view = character.id.clone();
                                                                let id_for_clone = character.id.clone();
                                                                let id_being_cloned = cloning_id.get();
                                                                let is_being_cloned = id_being_cloned.as_ref() == Some(&character.id);
                                                                let is_sheet_public = character.is_public;
                                                                let has_portrait = !character.photo_url.trim().is_empty();

                                                                view! {
                                                                    <article class="feed-card">
                                                                        // Retrato do Personagem
                                                                        <div class="feed-card-portrait-box">
                                                                            {if has_portrait {
                                                                                let focus_style = format!("object-position: {}% {}%;", character.photo_focus_x, character.photo_focus_y);
                                                                                view! {
                                                                                    <img
                                                                                        src=character.photo_url.clone()
                                                                                        alt=character.name.clone()
                                                                                        class="feed-card-portrait-img"
                                                                                        style=focus_style
                                                                                        loading="lazy"
                                                                                    />
                                                                                    <div class="feed-card-portrait-gradient"></div>
                                                                                }.into_view()
                                                                            } else {
                                                                                view! {
                                                                                    <div class="feed-card-portrait-placeholder">
                                                                                        <span style="font-size: 2.2rem;">{if is_gm { "🐉" } else { "🧙" }}</span>
                                                                                        <span style="font-size: 0.78rem; opacity: 0.75; text-transform: uppercase; letter-spacing: 0.05em;">"Sem Retrato"</span>
                                                                                    </div>
                                                                                    <div class="feed-card-portrait-gradient"></div>
                                                                                }.into_view()
                                                                            }}

                                                                            <span class=format!("feed-card-splat-badge {}", badge_class)>
                                                                                {splat_badge}
                                                                            </span>

                                                                            {if is_self {
                                                                                view! {
                                                                                    <span class=format!("profile-sheet-visibility-badge {}", if is_sheet_public { "is-public" } else { "is-private" })>
                                                                                        {if is_sheet_public { "🌐 Pública" } else { "🔒 Privada" }}
                                                                                    </span>
                                                                                }.into_view()
                                                                            } else {
                                                                                view! { <span></span> }.into_view()
                                                                            }}
                                                                        </div>

                                                                        // Conteúdo do Card
                                                                        <div class="feed-card-body">
                                                                            <div class="feed-card-name-row">
                                                                                <h3 class="feed-card-name">
                                                                                    {if character.name.trim().is_empty() { "Sem Nome".to_string() } else { character.name.clone() }}
                                                                                </h3>
                                                                            </div>

                                                                            <div class="feed-card-tags-row">
                                                                                {if !character.tradition.trim().is_empty() {
                                                                                    view! { <span class="feed-tag">{character.tradition.clone()}</span> }.into_view()
                                                                                } else {
                                                                                    view! { <span></span> }.into_view()
                                                                                }}
                                                                                {if !character.essence.trim().is_empty() {
                                                                                    view! { <span class="feed-tag">{character.essence.clone()}</span> }.into_view()
                                                                                } else {
                                                                                    view! { <span></span> }.into_view()
                                                                                }}
                                                                            </div>

                                                                            <div class="feed-card-stats-row">
                                                                                <div class="feed-stat-pill">
                                                                                    <span class="feed-stat-label">"Arete"</span>
                                                                                    <span class="feed-stat-val">{character.arete}</span>
                                                                                </div>
                                                                                <div class="feed-stat-pill">
                                                                                    <span class="feed-stat-label">"Vontade"</span>
                                                                                    <span class="feed-stat-val">{character.willpower}</span>
                                                                                </div>
                                                                            </div>

                                                                            <div class="feed-card-spheres-row">
                                                                                {character.spheres.into_iter().filter(|(_, val)| *val > 0).map(|(name, val)| {
                                                                                    view! {
                                                                                        <span class="feed-sphere-chip">
                                                                                            <span>{name}</span>
                                                                                            <strong>{val}</strong>
                                                                                        </span>
                                                                                    }
                                                                                }).collect_view()}
                                                                            </div>

                                                                            <div class="feed-card-footer">
                                                                                <span class="feed-card-date">
                                                                                    {if character.updated_at.len() >= 10 {
                                                                                        character.updated_at[..10].to_string()
                                                                                    } else {
                                                                                        character.updated_at.clone()
                                                                                    }}
                                                                                </span>
                                                                                <div class="feed-card-actions">
                                                                                    <A href=format!("/sheet/{}", id_for_view) class="feed-action-btn feed-view-btn">
                                                                                        {if is_self { "✏️ Abrir" } else { "👁️ Ver" }}
                                                                                    </A>
                                                                                    <button
                                                                                        type="button"
                                                                                        class="feed-action-btn feed-clone-btn"
                                                                                        disabled=is_being_cloned
                                                                                        on:click=move |_| handle_clone_sheet(id_for_clone.clone(), cloning_id, set_cloning_id, "Perfil")
                                                                                    >
                                                                                        {if is_being_cloned { "⏳ Clonando..." } else { "📋 Clonar" }}
                                                                                    </button>
                                                                                </div>
                                                                            </div>
                                                                        </div>
                                                                    </article>
                                                                }
                                                            }).collect_view()}
                                                        </div>
                                                    }.into_view()
                                                }
                                            }}
                                        </section>
                                    }.into_view()
                                }
                            }}
                        </div>
                    }.into_view()
                }}
            </Transition>
        </div>
    }
}
