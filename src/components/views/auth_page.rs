use leptos::*;
use leptos_router::*;
use crate::auth::{login, register};

#[component]
pub fn AuthPage() -> impl IntoView {
    let query = use_query_map();
    let initial_tab_register = query.with(|q| q.get("tab").map(|t| t == "register").unwrap_or(false));
    let initial_error = query.with(|q| q.get("error").cloned());
    let initial_user = query.with(|q| q.get("user").cloned().unwrap_or_default());

    let (is_register, set_is_register) = create_signal(initial_tab_register);
    let (username, set_username) = create_signal(initial_user);
    let (password, set_password) = create_signal(String::new());
    let (confirm_password, set_confirm_password) = create_signal(String::new());
    let (error_msg, set_error_msg) = create_signal(initial_error);
    let (is_submitting, set_is_submitting) = create_signal(false);
    #[cfg(not(target_arch = "wasm32"))]
    let navigate = use_navigate();

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let user_val = username.get().trim().to_string();
        let pass_val = password.get();
        let confirm_val = confirm_password.get();

        if user_val.is_empty() || pass_val.is_empty() {
            set_error_msg.set(Some("Preencha todos os campos obrigatórios".to_string()));
            return;
        }

        if is_register.get() && pass_val != confirm_val {
            set_error_msg.set(Some("As senhas não conferem".to_string()));
            return;
        }

        let is_reg = is_register.get_untracked();
        set_is_submitting.set(true);
        set_error_msg.set(None);
        #[cfg(not(target_arch = "wasm32"))]
        let navigate = navigate.clone();

        spawn_local(async move {
            let res = if is_reg {
                register(user_val, pass_val).await
            } else {
                login(user_val, pass_val).await
            };

            match res {
                Ok(_) => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        if let Some(window) = web_sys::window() {
                            let _ = window.location().set_href("/");
                            return;
                        }
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    navigate("/", Default::default());
                }
                Err(e) => {
                    let mut msg = e.to_string();
                    if msg.contains("error trying to connect") || msg.contains("database") {
                        msg = "Falha temporária ao conectar ao banco de dados".to_string();
                    } else if msg.starts_with("error: ") {
                        msg = msg.trim_start_matches("error: ").to_string();
                    }
                    set_error_msg.set(Some(msg));
                    set_is_submitting.set(false);
                }
            }
        });
    };

    view! {
        <div class="auth-page-container">
            <div class="auth-card">
                <div class="auth-header">
                    <span class="auth-icon">"🔮"</span>
                    <h2>{move || if is_register.get() { "Criar Conta de Mago" } else { "Entrar no MTA Sheet" }}</h2>
                    <p>{move || if is_register.get() { "Crie sua conta para gerenciar suas crônicas e salas" } else { "Acesse suas fichas e salas de jogo" }}</p>
                </div>

                <div class="auth-tabs">
                    <button 
                        class="auth-tab" 
                        class:active=move || !is_register.get()
                        on:click=move |_| { set_is_register.set(false); set_error_msg.set(None); }
                    >
                        "Entrar"
                    </button>
                    <button 
                        class="auth-tab" 
                        class:active=move || is_register.get()
                        on:click=move |_| { set_is_register.set(true); set_error_msg.set(None); }
                    >
                        "Cadastrar"
                    </button>
                </div>

                {move || error_msg.get().map(|msg| view! {
                    <div class="alert-box alert-error">
                        <span>{msg}</span>
                    </div>
                })}

                <form on:submit=on_submit class="auth-form">
                    <div class="form-group">
                        <label class="form-label">"Usuário"</label>
                        <input
                            type="text"
                            name="username"
                            placeholder="Seu nome de usuário"
                            class="form-input"
                            prop:value=username
                            on:input=move |ev| set_username.set(event_target_value(&ev))
                            disabled=move || is_submitting.get()
                            required
                        />
                    </div>

                    <div class="form-group">
                        <label class="form-label">"Senha"</label>
                        <input
                            type="password"
                            name="password"
                            placeholder="Sua senha"
                            class="form-input"
                            prop:value=password
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                            disabled=move || is_submitting.get()
                            required
                        />
                    </div>

                    {move || is_register.get().then(|| view! {
                        <div class="form-group">
                            <label class="form-label">"Confirmar Senha"</label>
                            <input
                                type="password"
                                name="confirm_password"
                                placeholder="Confirme sua senha"
                                class="form-input"
                                prop:value=confirm_password
                                on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                                disabled=move || is_submitting.get()
                                required
                            />
                        </div>
                    })}

                    <button type="submit" class="auth-submit-btn" disabled=move || is_submitting.get()>
                        {move || if is_submitting.get() {
                            "Processando..."
                        } else if is_register.get() {
                            "Criar Minha Conta"
                        } else {
                            "Entrar"
                        }}
                    </button>
                </form>

                <div class="auth-footer">
                    <A href="/" class="auth-back-link">"← Continuar como Convidado"</A>
                </div>
            </div>
        </div>
    }
}
