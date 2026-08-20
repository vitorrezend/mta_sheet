use leptos::*;
use leptos_router::*;
use crate::rooms::{get_user_rooms, create_room, join_room_by_code, RoomSummary};
use crate::auth::{get_current_user, UserInfo};
use crate::components::Navbar;

#[component]
pub fn RoomsPage() -> impl IntoView {
    let user_resource = create_resource(|| (), |_| async move { get_current_user().await });
    let rooms_resource = create_resource(|| (), |_| async move { get_user_rooms().await });

    let (user, set_user) = create_signal(Option::<UserInfo>::None);
    let (new_room_name, set_new_room_name) = create_signal(String::new());
    let (new_room_desc, set_new_room_desc) = create_signal(String::new());
    let (join_code, set_join_code) = create_signal(String::new());
    let (error_msg, set_error_msg) = create_signal(Option::<String>::None);
    let (success_msg, set_success_msg) = create_signal(Option::<String>::None);
    let (is_busy, set_is_busy) = create_signal(false);

    let navigate_create = use_navigate();
    let navigate_join = use_navigate();

    create_effect(move |_| {
        if let Some(Ok(u)) = user_resource.get() {
            set_user.set(u);
        }
    });

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

    let on_create_room = store_value(on_create_room);
    let on_join_code = store_value(on_join_code);

    view! {
        <link rel="stylesheet" href="/style.css"/>
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

            {move || match user.get() {
                None => view! {
                    <div class="auth-prompt-card">
                        <span class="prompt-icon">"🏰"</span>
                        <h3>"Faça Login para Acessar as Salas"</h3>
                        <p>"Você precisa de uma conta para criar mesas como Narrador ou ingressar nas crônicas de seus amigos."</p>
                        <A href="/login" class="prompt-login-btn">"Entrar ou Criar Conta"</A>
                    </div>
                }.into_view(),
                Some(_) => view! {
                    <div class="rooms-actions-grid">
                        // Create Room Card
                        <section class="room-action-card">
                            <h3>"👑 Criar Nova Crônica (Narrador)"</h3>
                            <form on:submit=move |ev| on_create_room.with_value(|f| f(ev)) class="room-form">
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

                        // Join Room Card
                        <section class="room-action-card">
                            <h3>"🔑 Entrar por Código"</h3>
                            <form on:submit=move |ev| on_join_code.with_value(|f| f(ev)) class="room-form">
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

                    // Rooms List
                    <section class="user-rooms-section">
                        <h2>"Suas Crônicas Ativas"</h2>
                        <Suspense fallback=move || view! { <p class="loading-msg">"Carregando salas..."</p> }>
                            {move || rooms_resource.get().map(|res| match res {
                                Ok(rooms) if rooms.is_empty() => view! {
                                    <div class="empty-rooms">
                                        <p>"Você ainda não participa de nenhuma sala. Crie uma nova crônica acima ou use um código para entrar!"</p>
                                    </div>
                                }.into_view(),
                                Ok(rooms) => view! {
                                    <div class="rooms-grid">
                                        {rooms.into_iter().map(|room: RoomSummary| {
                                            let id = room.id.clone();
                                            view! {
                                                <div class="room-card" class:gm-card=room.is_gm>
                                                    <div class="room-card-header">
                                                        <div class="room-badges">
                                                            {if room.is_gm {
                                                                view! { <span class="badge badge-gm">"👑 Narrador"</span> }
                                                            } else {
                                                                view! { <span class="badge badge-player">"Jogador"</span> }
                                                            }}
                                                            <span class="badge badge-code">{room.code}</span>
                                                        </div>
                                                        <h3 class="room-card-title">{room.name}</h3>
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
                                Err(e) => view! {
                                    <div class="alert-box alert-error">
                                        <p>"Erro ao carregar salas: " {e.to_string()}</p>
                                    </div>
                                }.into_view(),
                            })}
                        </Suspense>
                    </section>
                }.into_view(),
            }}
        </div>
    }
}
