use leptos::*;
use leptos_router::*;
use crate::rooms::{get_user_rooms, create_room, join_room_by_code, RoomSummary};
use crate::components::Navbar;

#[component]
pub fn RoomsPage() -> impl IntoView {
    let auth = use_context::<crate::AuthContext>();
    let user = auth.map(|a| a.user).unwrap_or_else(|| Signal::derive(|| None));

    let rooms_resource = create_local_resource(
        move || user.get(),
        |u| async move {
            if u.is_some() {
                get_user_rooms().await
            } else {
                Ok(Vec::new())
            }
        }
    );

    let (new_room_name, set_new_room_name) = create_signal(String::new());
    let (new_room_desc, set_new_room_desc) = create_signal(String::new());
    let (join_code, set_join_code) = create_signal(String::new());
    let (error_msg, set_error_msg) = create_signal(Option::<String>::None);
    let (success_msg, set_success_msg) = create_signal(Option::<String>::None);
    let (is_busy, set_is_busy) = create_signal(false);

    let navigate_create = use_navigate();
    let navigate_join = use_navigate();

    let on_create_room = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let name_val = new_room_name.get().trim().to_string();
        let desc_val = new_room_desc.get().trim().to_string();

        if name_val.is_empty() {
            set_error_msg.set(Some("Digite o nome da crônica".to_string()));
            return;
        }

        set_is_busy.set(true);
        set_error_msg.set(None);
        let navigate = navigate_create.clone();

        spawn_local(async move {
            match create_room(name_val, desc_val).await {
                Ok(room_id) => {
                    navigate(&format!("/room/{}", room_id), Default::default());
                }
                Err(e) => {
                    set_error_msg.set(Some(e.to_string()));
                    set_is_busy.set(false);
                }
            }
        });
    };

    let on_join_code = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let code_val = join_code.get().trim().to_string();

        if code_val.is_empty() {
            set_error_msg.set(Some("Digite o código da sala".to_string()));
            return;
        }

        set_is_busy.set(true);
        set_error_msg.set(None);
        let navigate = navigate_join.clone();

        spawn_local(async move {
            match join_room_by_code(code_val).await {
                Ok(room_id) => {
                    navigate(&format!("/room/{}", room_id), Default::default());
                }
                Err(e) => {
                    set_error_msg.set(Some(e.to_string()));
                    set_is_busy.set(false);
                }
            }
        });
    };

    let on_create_room = crate::components::SafeCallback::new(on_create_room);
    let on_join_code = crate::components::SafeCallback::new(on_join_code);

    let on_create_room_submit = on_create_room.clone();
    let on_join_code_submit = on_join_code.clone();

    view! {
        <div class="rooms-page">
            <Navbar />
            <div class="rooms-container">
                <header class="rooms-header">
                    <h1>"Salas de Jogo & Crônicas"</h1>
                    <p>"Reúna seus jogadores, compartilhe fichas e acompanhe a mesa em tempo real"</p>
                </header>

                {move || error_msg.get().map(|msg| view! {
                    <div class="alert-box alert-error">
                        <span>{msg}</span>
                        <button class="alert-close" on:click=move |_| set_error_msg.set(None)>"×"</button>
                    </div>
                })}

                {move || success_msg.get().map(|msg| view! {
                    <div class="alert-box alert-success">
                        <span>{msg}</span>
                        <button class="alert-close" on:click=move |_| set_success_msg.set(None)>"×"</button>
                    </div>
                })}

                <Show
                    when=move || user.get().is_some()
                    fallback=move || view! {
                        <div class="auth-prompt-card">
                            <span class="prompt-icon">"🏰"</span>
                            <h3>"Faça Login para Acessar as Salas"</h3>
                            <p>"Você precisa de uma conta para criar mesas como Narrador ou ingressar nas crônicas de seus amigos."</p>
                            <A href="/login" class="prompt-login-btn">"Entrar ou Criar Conta"</A>
                        </div>
                    }
                >
                    {
                        let on_create_cb = on_create_room_submit.clone();
                        let on_join_cb = on_join_code_submit.clone();
                        view! {
                        <div class="rooms-actions-grid">
                            <section class="room-action-card">
                                <h3>"👑 Criar Nova Crônica (Narrador)"</h3>
                                <form on:submit=move |ev| on_create_cb.call(ev) class="room-form">
                                    <input
                                        type="text"
                                        placeholder="Nome da Crônica (ex: Cabala dos Eus)"
                                        class="room-input"
                                        prop:value=new_room_name
                                        on:input=move |ev| set_new_room_name.set(event_target_value(&ev))
                                        disabled=is_busy
                                        required
                                    />
                                    <input
                                        type="text"
                                        placeholder="Descrição (opcional, ex: Londres Vitoriana)"
                                        class="room-input"
                                        prop:value=new_room_desc
                                        on:input=move |ev| set_new_room_desc.set(event_target_value(&ev))
                                        disabled=is_busy
                                    />
                                    <button type="submit" class="room-btn btn-create" disabled=is_busy>
                                        {move || if is_busy.get() { "Criando..." } else { "Criar Sala" }}
                                    </button>
                                </form>
                            </section>

                            <section class="room-action-card">
                                <h3>"🔑 Entrar por Código"</h3>
                                <form on:submit=move |ev| on_join_cb.call(ev) class="room-form">
                                    <input
                                        type="text"
                                        placeholder="Código da Sala (ex: MTA-74A)"
                                        class="room-input code-input"
                                        prop:value=join_code
                                        on:input=move |ev| set_join_code.set(event_target_value(&ev))
                                        disabled=is_busy
                                        required
                                    />
                                    <button type="submit" class="room-btn btn-join" disabled=is_busy>
                                        {move || if is_busy.get() { "Entrando..." } else { "Ingressar na Sala" }}
                                    </button>
                                </form>
                            </section>
                        </div>

                        <section class="user-rooms-section">
                            <h2>"Minhas Mesas"</h2>
                            {move || match rooms_resource.get() {
                                None => view! { <p class="loading-msg">"Carregando salas..."</p> }.into_view(),
                                Some(Ok(data)) if data.is_empty() => view! {
                                    <div class="empty-rooms-msg">
                                        <p>"Você ainda não participa de nenhuma sala de jogo."</p>
                                        <p>"Crie sua mesa acima ou peça o código de acesso para seu Narrador!"</p>
                                    </div>
                                }.into_view(),
                                Some(Ok(data)) => view! {
                                    <div class="rooms-grid">
                                        {data.into_iter().map(|room: RoomSummary| {
                                            let id = room.id.clone();
                                            let id_card = id.clone();
                                            view! {
                                                <div 
                                                    class="room-card clickable-room-card"
                                                    on:click=move |_| {
                                                        let nav = use_navigate();
                                                        let _ = nav(&format!("/room/{}", id_card), Default::default());
                                                    }
                                                >
                                                    <div class="room-card-header">
                                                        <div class="room-badge">{if room.is_gm { "👑 Narrador" } else { "🧙 Jogador" }}</div>
                                                        <span class="room-code-tag" title="Código de Convite">{&room.code}</span>
                                                    </div>

                                                    <div class="room-card-body">
                                                        <h3>{&room.name}</h3>
                                                        <p class="room-card-desc">{if room.description.is_empty() { "Sem descrição".to_string() } else { room.description }}</p>
                                                    </div>

                                                    <div class="room-card-stats">
                                                        <span>"👥 " {room.member_count} " membros"</span>
                                                        <span>"📜 " {room.sheet_count} " fichas"</span>
                                                    </div>

                                                    <div class="room-card-footer">
                                                        <A href=format!("/room/{}", id) class="enter-room-btn">"Abrir Mesa →"</A>
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_view(),
                                Some(Err(e)) => view! {
                                    <div class="alert-box alert-error">
                                        <p>"Erro ao carregar salas: " {e.to_string()}</p>
                                    </div>
                                }.into_view(),
                            }}
                        </section>
                    }}
                </Show>
            </div>
        </div>
    }
}
