use leptos::*;
use leptos_router::*;
use crate::rooms::{get_user_rooms, get_public_rooms, create_room, join_room_by_code, join_public_room, RoomSummary};
use crate::components::Navbar;

#[component]
pub fn RoomsPage() -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

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

    let public_rooms_resource = create_local_resource(
        move || (),
        |_| async move {
            get_public_rooms().await
        }
    );

    let (active_tab, set_active_tab) = create_signal("my_rooms"); // "my_rooms" | "public_rooms"
    let (new_room_name, set_new_room_name) = create_signal(String::new());
    let (new_room_desc, set_new_room_desc) = create_signal(String::new());
    let (new_room_is_public, set_new_room_is_public) = create_signal(false);
    let (new_room_password, set_new_room_password) = create_signal(String::new());

    let (join_code, set_join_code) = create_signal(String::new());
    let (join_password, set_join_password) = create_signal(String::new());

    // Modal de senha para entrar em sala protegida
    let (password_modal_target, set_password_modal_target) = create_signal(Option::<(String, String, bool)>::None); // (room_id_or_code, room_name, is_code)
    let (modal_password_input, set_modal_password_input) = create_signal(String::new());

    let (error_msg, set_error_msg) = create_signal(Option::<String>::None);
    let (success_msg, set_success_msg) = create_signal(Option::<String>::None);
    let (is_busy, set_is_busy) = create_signal(false);

    let navigate_create = use_navigate();
    let navigate_join = use_navigate();

    let on_create_room = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let name_val = new_room_name.get().trim().to_string();
        let desc_val = new_room_desc.get().trim().to_string();
        let is_pub = new_room_is_public.get();
        let pwd_val = new_room_password.get().trim().to_string();
        let pwd_opt = if pwd_val.is_empty() { None } else { Some(pwd_val) };

        if name_val.is_empty() {
            set_error_msg.set(Some(match lang() {
                crate::i18n::Language::PtBr => "Digite o nome da crônica".to_string(),
                crate::i18n::Language::EnUs => "Enter the chronicle name".to_string(),
            }));
            return;
        }

        set_is_busy.set(true);
        set_error_msg.set(None);
        let navigate = navigate_create.clone();

        spawn_local(async move {
            match create_room(name_val, desc_val, is_pub, pwd_opt).await {
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
        let pwd_val = join_password.get().trim().to_string();
        let pwd_opt = if pwd_val.is_empty() { None } else { Some(pwd_val) };

        if code_val.is_empty() {
            set_error_msg.set(Some(match lang() {
                crate::i18n::Language::PtBr => "Digite o código da sala".to_string(),
                crate::i18n::Language::EnUs => "Enter the room code".to_string(),
            }));
            return;
        }

        set_is_busy.set(true);
        set_error_msg.set(None);
        let navigate = navigate_join.clone();

        spawn_local(async move {
            match join_room_by_code(code_val.clone(), pwd_opt).await {
                Ok(room_id) => {
                    navigate(&format!("/room/{}", room_id), Default::default());
                }
                Err(e) => {
                    let err_str = e.to_string();
                    if err_str.contains("Senha incorreta") || err_str.contains("Senha necessária") {
                        set_password_modal_target.set(Some((code_val, "Sala Protegida".to_string(), true)));
                        set_modal_password_input.set(String::new());
                        set_error_msg.set(None);
                    } else {
                        set_error_msg.set(Some(err_str));
                    }
                    set_is_busy.set(false);
                }
            }
        });
    };

    let on_confirm_modal_password = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let target = match password_modal_target.get() {
            Some(t) => t,
            None => return,
        };
        let (id_or_code, _name, is_code) = target;
        let pwd = modal_password_input.get().trim().to_string();

        if pwd.is_empty() {
            set_error_msg.set(Some(match lang() {
                crate::i18n::Language::PtBr => "Digite a senha de acesso".to_string(),
                crate::i18n::Language::EnUs => "Enter the access password".to_string(),
            }));
            return;
        }

        set_is_busy.set(true);
        set_error_msg.set(None);
        let navigate = use_navigate();

        spawn_local(async move {
            let res = if is_code {
                join_room_by_code(id_or_code, Some(pwd)).await
            } else {
                join_public_room(id_or_code, Some(pwd)).await
            };

            match res {
                Ok(room_id) => {
                    set_password_modal_target.set(None);
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
    let on_confirm_modal_password = crate::components::SafeCallback::new(on_confirm_modal_password);

    let on_create_room_submit = on_create_room.clone();
    let on_join_code_submit = on_join_code.clone();
    let on_modal_pwd_submit = on_confirm_modal_password.clone();

    view! {
        <div class="rooms-page">
            <Navbar />
            <div class="rooms-container">
                <header class="rooms-header">
                    <h1>{move || match lang() {
                        crate::i18n::Language::PtBr => "Salas de Jogo & Crônicas",
                        crate::i18n::Language::EnUs => "Game Rooms & Chronicles",
                    }}</h1>
                    <p>{move || match lang() {
                        crate::i18n::Language::PtBr => "Reúna seus jogadores, compartilhe fichas e acompanhe a mesa em tempo real",
                        crate::i18n::Language::EnUs => "Gather your players, share sheets, and track the game in real time",
                    }}</p>
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
                            <h3>{move || match lang() {
                                crate::i18n::Language::PtBr => "Faça Login para Acessar as Salas",
                                crate::i18n::Language::EnUs => "Sign in to Access Rooms",
                            }}</h3>
                            <p>{move || match lang() {
                                crate::i18n::Language::PtBr => "Você precisa de uma conta para criar mesas como Narrador ou ingressar nas crônicas de seus amigos.",
                                crate::i18n::Language::EnUs => "You need an account to host tables as Storyteller or join your friends' chronicles.",
                            }}</p>
                            <A href="/login" class="prompt-login-btn">{move || match lang() {
                                crate::i18n::Language::PtBr => "Entrar ou Criar Conta",
                                crate::i18n::Language::EnUs => "Sign In or Register",
                            }}</A>
                        </div>
                    }
                >
                    {
                        let on_create_cb = on_create_room_submit.clone();
                        let on_join_cb = on_join_code_submit.clone();
                        view! {
                        <div class="rooms-actions-grid">
                            <section class="room-action-card">
                                <h3>{move || match lang() {
                                    crate::i18n::Language::PtBr => "👑 Criar Nova Crônica (Narrador)",
                                    crate::i18n::Language::EnUs => "👑 Create New Chronicle (Storyteller)",
                                }}</h3>
                                <form on:submit=move |ev| on_create_cb.call(ev) class="room-form">
                                    <input
                                        type="text"
                                        placeholder=move || match lang() {
                                            crate::i18n::Language::PtBr => "Nome da Crônica (ex: Cabala dos Eus)",
                                            crate::i18n::Language::EnUs => "Chronicle Name (e.g. Cabal of Selves)",
                                        }
                                        class="room-input"
                                        prop:value=new_room_name
                                        on:input=move |ev| set_new_room_name.set(event_target_value(&ev))
                                        disabled=is_busy
                                        required
                                    />
                                    <input
                                        type="text"
                                        placeholder=move || match lang() {
                                            crate::i18n::Language::PtBr => "Descrição (opcional, ex: Londres Vitoriana)",
                                            crate::i18n::Language::EnUs => "Description (optional, e.g. Victorian London)",
                                        }
                                        class="room-input"
                                        prop:value=new_room_desc
                                        on:input=move |ev| set_new_room_desc.set(event_target_value(&ev))
                                        disabled=is_busy
                                    />
                                    
                                    <div class="room-form-options">
                                        <label class="room-checkbox-label">
                                            <input
                                                type="checkbox"
                                                class="room-checkbox"
                                                checked=new_room_is_public
                                                on:change=move |ev| set_new_room_is_public.set(event_target_checked(&ev))
                                                disabled=is_busy
                                            />
                                            <span>{move || match lang() {
                                                crate::i18n::Language::PtBr => "🌐 Mesa Pública (Visível para todos explorarem)",
                                                crate::i18n::Language::EnUs => "🌐 Public Room (Visible for everyone to explore)",
                                            }}</span>
                                        </label>

                                        <input
                                            type="password"
                                            placeholder=move || match lang() {
                                                crate::i18n::Language::PtBr => "🔒 Senha de Acesso (Opcional)",
                                                crate::i18n::Language::EnUs => "🔒 Access Password (Optional)",
                                            }
                                            class="room-input pwd-input"
                                            prop:value=new_room_password
                                            on:input=move |ev| set_new_room_password.set(event_target_value(&ev))
                                            disabled=is_busy
                                        />
                                    </div>

                                    <button type="submit" class="room-btn btn-create" disabled=is_busy>
                                        {move || if is_busy.get() {
                                            match lang() { crate::i18n::Language::PtBr => "Criando...", crate::i18n::Language::EnUs => "Creating..." }
                                        } else {
                                            match lang() { crate::i18n::Language::PtBr => "Criar Sala", crate::i18n::Language::EnUs => "Create Room" }
                                        }}
                                    </button>
                                </form>
                            </section>

                            <section class="room-action-card">
                                <h3>{move || match lang() {
                                    crate::i18n::Language::PtBr => "🔑 Entrar por Código",
                                    crate::i18n::Language::EnUs => "🔑 Join by Code",
                                }}</h3>
                                <form on:submit=move |ev| on_join_cb.call(ev) class="room-form">
                                    <input
                                        type="text"
                                        placeholder=move || match lang() {
                                            crate::i18n::Language::PtBr => "Código da Sala (ex: MTA-74A)",
                                            crate::i18n::Language::EnUs => "Room Code (e.g. MTA-74A)",
                                        }
                                        class="room-input code-input"
                                        prop:value=join_code
                                        on:input=move |ev| set_join_code.set(event_target_value(&ev))
                                        disabled=is_busy
                                        required
                                    />
                                    <input
                                        type="password"
                                        placeholder=move || match lang() {
                                            crate::i18n::Language::PtBr => "Senha da sala (caso tenha)",
                                            crate::i18n::Language::EnUs => "Room password (if any)",
                                        }
                                        class="room-input pwd-input"
                                        prop:value=join_password
                                        on:input=move |ev| set_join_password.set(event_target_value(&ev))
                                        disabled=is_busy
                                    />
                                    <button type="submit" class="room-btn btn-join" disabled=is_busy>
                                        {move || if is_busy.get() {
                                            match lang() { crate::i18n::Language::PtBr => "Entrando...", crate::i18n::Language::EnUs => "Joining..." }
                                        } else {
                                            match lang() { crate::i18n::Language::PtBr => "Ingressar na Sala", crate::i18n::Language::EnUs => "Join Room" }
                                        }}
                                    </button>
                                </form>
                            </section>
                        </div>

                        <div class="rooms-tab-bar">
                            <button
                                class=move || if active_tab.get() == "my_rooms" { "rooms-tab-btn active" } else { "rooms-tab-btn" }
                                on:click=move |_| set_active_tab.set("my_rooms")
                            >
                                {move || match lang() {
                                    crate::i18n::Language::PtBr => "🏰 Minhas Mesas",
                                    crate::i18n::Language::EnUs => "🏰 My Tables",
                                }}
                            </button>
                            <button
                                class=move || if active_tab.get() == "public_rooms" { "rooms-tab-btn active" } else { "rooms-tab-btn" }
                                on:click=move |_| {
                                    set_active_tab.set("public_rooms");
                                    public_rooms_resource.refetch();
                                }
                            >
                                {move || match lang() {
                                    crate::i18n::Language::PtBr => "🌐 Explorar Mesas Públicas",
                                    crate::i18n::Language::EnUs => "🌐 Explore Public Tables",
                                }}
                            </button>
                        </div>

                        {move || {
                            let current_lang = lang();
                            if active_tab.get() == "my_rooms" {
                                view! {
                                    <section class="user-rooms-section">
                                        {move || match rooms_resource.get() {
                                            None => view! { <p class="loading-msg">{match current_lang {
                                                crate::i18n::Language::PtBr => "Carregando salas...",
                                                crate::i18n::Language::EnUs => "Loading rooms...",
                                            }}</p> }.into_view(),
                                            Some(Ok(data)) if data.is_empty() => view! {
                                                <div class="empty-rooms-msg">
                                                    <p>{match current_lang {
                                                        crate::i18n::Language::PtBr => "Você ainda não participa de nenhuma sala de jogo.",
                                                        crate::i18n::Language::EnUs => "You are not participating in any game rooms yet.",
                                                    }}</p>
                                                    <p>{match current_lang {
                                                        crate::i18n::Language::PtBr => "Crie sua mesa acima, explore as mesas públicas ou peça o código de acesso para seu Narrador!",
                                                        crate::i18n::Language::EnUs => "Create your table above, explore public rooms, or ask your Storyteller for the invite code!",
                                                    }}</p>
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
                                                                    <div class="room-badge-group">
                                                                        <div class="room-badge">{if room.is_gm {
                                                                            match current_lang { crate::i18n::Language::PtBr => "👑 Narrador", crate::i18n::Language::EnUs => "👑 Storyteller" }
                                                                        } else {
                                                                            match current_lang { crate::i18n::Language::PtBr => "🧙 Jogador", crate::i18n::Language::EnUs => "🧙 Player" }
                                                                        }}</div>
                                                                        {if room.is_public {
                                                                            view! { <span class="room-privacy-tag public">{match current_lang {
                                                                                crate::i18n::Language::PtBr => "🌐 Pública",
                                                                                crate::i18n::Language::EnUs => "🌐 Public",
                                                                            }}</span> }.into_view()
                                                                        } else {
                                                                            view! { <span class="room-privacy-tag private">{match current_lang {
                                                                                crate::i18n::Language::PtBr => "🔒 Privada",
                                                                                crate::i18n::Language::EnUs => "🔒 Private",
                                                                            }}</span> }.into_view()
                                                                        }}
                                                                        {if room.has_password {
                                                                            view! { <span class="room-pwd-tag" title=match current_lang {
                                                                                crate::i18n::Language::PtBr => "Protegida por Senha",
                                                                                crate::i18n::Language::EnUs => "Password Protected",
                                                                            }>{match current_lang {
                                                                                crate::i18n::Language::PtBr => "🔑 Senha",
                                                                                crate::i18n::Language::EnUs => "🔑 Password",
                                                                            }}</span> }.into_view()
                                                                        } else {
                                                                            view! {}.into_view()
                                                                        }}
                                                                    </div>
                                                                    <span class="room-code-tag" title=match current_lang {
                                                                        crate::i18n::Language::PtBr => "Código de Convite",
                                                                        crate::i18n::Language::EnUs => "Invite Code",
                                                                    }>{&room.code}</span>
                                                                </div>

                                                                <div class="room-card-body">
                                                                    <h3>{&room.name}</h3>
                                                                    <p class="room-card-desc">{if room.description.is_empty() {
                                                                        match current_lang {
                                                                            crate::i18n::Language::PtBr => "Sem descrição".to_string(),
                                                                            crate::i18n::Language::EnUs => "No description".to_string(),
                                                                        }
                                                                    } else { room.description }}</p>
                                                                </div>

                                                                <div class="room-card-stats">
                                                                    <span>"👥 " {room.member_count} {match current_lang { crate::i18n::Language::PtBr => " membros", crate::i18n::Language::EnUs => " members" }}</span>
                                                                    <span>"📜 " {room.sheet_count} {match current_lang { crate::i18n::Language::PtBr => " fichas", crate::i18n::Language::EnUs => " sheets" }}</span>
                                                                </div>

                                                                <div class="room-card-footer">
                                                                    <A href=format!("/room/{}", id) class="enter-room-btn">{match current_lang {
                                                                        crate::i18n::Language::PtBr => "Abrir Mesa →",
                                                                        crate::i18n::Language::EnUs => "Open Table →",
                                                                    }}</A>
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
                                }.into_view()
                            } else {
                                view! {
                                    <section class="user-rooms-section">
                                        {move || match public_rooms_resource.get() {
                                            None => view! { <p class="loading-msg">{match current_lang {
                                                crate::i18n::Language::PtBr => "Buscando mesas públicas...",
                                                crate::i18n::Language::EnUs => "Searching public tables...",
                                            }}</p> }.into_view(),
                                            Some(Ok(data)) if data.is_empty() => view! {
                                                <div class="empty-rooms-msg">
                                                    <p>{match current_lang {
                                                        crate::i18n::Language::PtBr => "Nenhuma mesa pública aberta no momento.",
                                                        crate::i18n::Language::EnUs => "No public tables available right now.",
                                                    }}</p>
                                                    <p>{match current_lang {
                                                        crate::i18n::Language::PtBr => "Que tal ser o pioneiro e criar uma nova crônica pública para sua cabala?",
                                                        crate::i18n::Language::EnUs => "How about being the first to create a new public chronicle for your cabal?",
                                                    }}</p>
                                                </div>
                                            }.into_view(),
                                            Some(Ok(data)) => view! {
                                                <div class="rooms-grid">
                                                    {data.into_iter().map(|room: RoomSummary| {
                                                        let id = room.id.clone();
                                                        let name = room.name.clone();
                                                        let has_pwd = room.has_password;
                                                        let is_gm = room.is_gm;
                                                        view! {
                                                            <div class="room-card clickable-room-card">
                                                                <div class="room-card-header">
                                                                    <div class="room-badge-group">
                                                                        <div class="room-badge">{match current_lang {
                                                                            crate::i18n::Language::PtBr => "👑 Narrador: ",
                                                                            crate::i18n::Language::EnUs => "👑 Storyteller: ",
                                                                        }} {&room.gm_username}</div>
                                                                        {if has_pwd {
                                                                            view! { <span class="room-pwd-tag" title=match current_lang {
                                                                                crate::i18n::Language::PtBr => "Requer Senha para ingressar",
                                                                                crate::i18n::Language::EnUs => "Password required to join",
                                                                            }>{match current_lang {
                                                                                crate::i18n::Language::PtBr => "🔒 Com Senha",
                                                                                crate::i18n::Language::EnUs => "🔒 Protected",
                                                                            }}</span> }.into_view()
                                                                        } else {
                                                                            view! { <span class="room-privacy-tag public">{match current_lang {
                                                                                crate::i18n::Language::PtBr => "🔓 Aberta",
                                                                                crate::i18n::Language::EnUs => "🔓 Open",
                                                                            }}</span> }.into_view()
                                                                        }}
                                                                    </div>
                                                                    <span class="room-code-tag">{&room.code}</span>
                                                                </div>

                                                                <div class="room-card-body">
                                                                    <h3>{&room.name}</h3>
                                                                    <p class="room-card-desc">{if room.description.is_empty() {
                                                                        match current_lang {
                                                                            crate::i18n::Language::PtBr => "Sem descrição".to_string(),
                                                                            crate::i18n::Language::EnUs => "No description".to_string(),
                                                                        }
                                                                    } else { room.description }}</p>
                                                                </div>

                                                                <div class="room-card-stats">
                                                                    <span>"👥 " {room.member_count} {match current_lang { crate::i18n::Language::PtBr => " membros", crate::i18n::Language::EnUs => " members" }}</span>
                                                                    <span>"📜 " {room.sheet_count} {match current_lang { crate::i18n::Language::PtBr => " fichas", crate::i18n::Language::EnUs => " sheets" }}</span>
                                                                </div>

                                                                <div class="room-card-footer">
                                                                    {if is_gm {
                                                                        view! {
                                                                            <A href=format!("/room/{}", id) class="enter-room-btn">{match current_lang {
                                                                                crate::i18n::Language::PtBr => "Sua Mesa →",
                                                                                crate::i18n::Language::EnUs => "Your Table →",
                                                                            }}</A>
                                                                        }.into_view()
                                                                    } else if has_pwd {
                                                                        let id_join = id.clone();
                                                                        let name_join = name.clone();
                                                                        view! {
                                                                            <button
                                                                                class="enter-room-btn btn-pwd-join"
                                                                                on:click=move |_| {
                                                                                    set_password_modal_target.set(Some((id_join.clone(), name_join.clone(), false)));
                                                                                    set_modal_password_input.set(String::new());
                                                                                }
                                                                            >
                                                                                {match current_lang {
                                                                                    crate::i18n::Language::PtBr => "🔒 Ingressar com Senha",
                                                                                    crate::i18n::Language::EnUs => "🔒 Join with Password",
                                                                                }}
                                                                            </button>
                                                                        }.into_view()
                                                                    } else {
                                                                        let id_join = id.clone();
                                                                        view! {
                                                                            <button
                                                                                class="enter-room-btn"
                                                                                on:click=move |_| {
                                                                                    let nav = use_navigate();
                                                                                    let id_target = id_join.clone();
                                                                                    spawn_local(async move {
                                                                                        match join_public_room(id_target.clone(), None).await {
                                                                                            Ok(_) => {
                                                                                                nav(&format!("/room/{}", id_target), Default::default());
                                                                                            }
                                                                                            Err(e) => {
                                                                                                set_error_msg.set(Some(e.to_string()));
                                                                                            }
                                                                                        }
                                                                                    });
                                                                                }
                                                                            >
                                                                                {match current_lang {
                                                                                    crate::i18n::Language::PtBr => "🚀 Ingressar na Mesa",
                                                                                    crate::i18n::Language::EnUs => "🚀 Join Table",
                                                                                }}
                                                                            </button>
                                                                        }.into_view()
                                                                    }}
                                                                </div>
                                                            </div>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            }.into_view(),
                                            Some(Err(e)) => view! {
                                                <div class="alert-box alert-error">
                                                    <p>"Erro ao carregar mesas públicas: " {e.to_string()}</p>
                                                </div>
                                            }.into_view(),
                                        }}
                                    </section>
                                }.into_view()
                            }
                        }}
                    }}
                </Show>

                // Modal de Inserção de Senha
                {move || password_modal_target.get().map(|(_id, name, _is_code)| {
                    let on_modal_pwd_cb = on_modal_pwd_submit.clone();
                    view! {
                        <div class="modal-backdrop" on:click=move |_| set_password_modal_target.set(None)>
                            <div class="modal-content pwd-modal-content" on:click=move |e| e.stop_propagation()>
                                <div class="modal-header">
                                    <h3>{match lang() {
                                        crate::i18n::Language::PtBr => "🔒 Mesa Protegida por Senha",
                                        crate::i18n::Language::EnUs => "🔒 Password Protected Table",
                                    }}</h3>
                                    <button class="modal-close-btn" on:click=move |_| set_password_modal_target.set(None)>"✕"</button>
                                </div>
                                <div class="modal-body">
                                    <p class="pwd-modal-desc">
                                        {match lang() {
                                            crate::i18n::Language::PtBr => "A mesa ",
                                            crate::i18n::Language::EnUs => "The table ",
                                        }}
                                        <strong>{name}</strong>
                                        {match lang() {
                                            crate::i18n::Language::PtBr => " requer uma senha de acesso para que você possa participar.",
                                            crate::i18n::Language::EnUs => " requires an access password to join.",
                                        }}
                                    </p>
                                    <form on:submit=move |ev| on_modal_pwd_cb.call(ev) class="pwd-modal-form">
                                        <input
                                            type="password"
                                            class="room-input pwd-modal-input"
                                            placeholder=match lang() {
                                                crate::i18n::Language::PtBr => "Digite a senha da sala...",
                                                crate::i18n::Language::EnUs => "Enter table password...",
                                            }
                                            prop:value=modal_password_input
                                            on:input=move |ev| set_modal_password_input.set(event_target_value(&ev))
                                            autofocus
                                            required
                                        />
                                        <div class="modal-actions">
                                            <button type="button" class="btn-secondary" on:click=move |_| set_password_modal_target.set(None)>
                                                {match lang() {
                                                    crate::i18n::Language::PtBr => "Cancelar",
                                                    crate::i18n::Language::EnUs => "Cancel",
                                                }}
                                            </button>
                                            <button type="submit" class="room-btn btn-join" disabled=is_busy>
                                                {move || if is_busy.get() {
                                                    match lang() {
                                                        crate::i18n::Language::PtBr => "Verificando...",
                                                        crate::i18n::Language::EnUs => "Verifying...",
                                                    }
                                                } else {
                                                    match lang() {
                                                        crate::i18n::Language::PtBr => "Confirmar e Entrar",
                                                        crate::i18n::Language::EnUs => "Confirm & Enter",
                                                    }
                                                }}
                                            </button>
                                        </div>
                                    </form>
                                </div>
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
