use leptos::*;
use leptos_router::*;
use crate::auth::{get_current_user, logout, UserInfo};

#[component]
pub fn Navbar() -> impl IntoView {
    let user_resource = create_resource(|| (), |_| async move { get_current_user().await });
    let (user, set_user) = create_signal(Option::<UserInfo>::None);

    create_effect(move |_| {
        if let Some(Ok(u)) = user_resource.get() {
            set_user.set(u);
        }
    });

    let on_logout = move |_| {
        spawn_local(async move {
            let _ = logout().await;
            set_user.set(None);
            user_resource.refetch();
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
