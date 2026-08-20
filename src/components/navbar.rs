use leptos::*;
use leptos_router::*;
use crate::auth::logout;
use crate::AuthContext;

#[component]
pub fn Navbar() -> impl IntoView {
    let auth = use_context::<AuthContext>();
    let user = auth.map(|a| a.user).unwrap_or_else(|| create_signal(None).0);
    let set_user = auth.map(|a| a.set_user).unwrap_or_else(|| create_signal(None).1);

    let on_logout = move |_| {
        spawn_local(async move {
            let _ = logout().await;
            set_user.set(None);
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(window) = web_sys::window() {
                    let _ = window.location().set_href("/");
                }
            }
        });
    };

    view! {
        <nav class="main-navbar">
            <div class="navbar-container">
                <div class="navbar-brand">
                    <A href="/" class="brand-link">
                        <span class="brand-icon">"🔮"</span>
                        <span class="brand-title">"MTA Sheet"</span>
                    </A>
                </div>

                <div class="navbar-links">
                    <A href="/" class="nav-link" exact=true>"📜 Fichas"</A>
                    <A href="/rooms" class="nav-link">"🏰 Salas de Jogo"</A>
                </div>

                <div class="navbar-auth">
                    {move || match user.get() {
                        Some(u) => view! {
                            <div class="user-pill">
                                <span class="user-greeting">"🧙 " {u.username}</span>
                                <button class="logout-btn" on:click=on_logout title="Sair da conta">"Sair"</button>
                            </div>
                        }.into_view(),
                        None => view! {
                            <A href="/login" class="login-link">"Entrar / Cadastrar"</A>
                        }.into_view(),
                    }}
                </div>
            </div>
        </nav>
    }
}
