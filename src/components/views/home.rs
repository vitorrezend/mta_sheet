use leptos::*;
use leptos_router::*;
use crate::state::{get_sheets, get_public_sheets, set_sheet_visibility, create_sheet, delete_sheet, CharacterSummary};
use crate::components::Navbar;
use crate::AuthContext;

#[component]
pub fn Home() -> impl IntoView {
    let auth = use_context::<AuthContext>();
    let user = auth.map(|a| a.user).unwrap_or_else(|| create_signal(None).0);

    let (home_tab, set_home_tab) = create_signal("my_sheets");
    let sheets = create_resource(|| (), |_| async move { get_sheets().await });
    let public_sheets = create_resource(
        move || home_tab.get(),
        |tab| async move {
            if tab == "public_sheets" {
                get_public_sheets().await
            } else {
                Ok(Vec::new())
            }
        }
    );

    let (name, set_name) = create_signal(String::new());
    let (error_msg, set_error_msg) = create_signal(Option::<String>::None);
    let (sheet_to_delete, set_sheet_to_delete) = create_signal(Option::<CharacterSummary>::None);
    let (is_creating, set_is_creating) = create_signal(false);
    let (selected_sheet_type, set_selected_sheet_type) = create_signal("mage".to_string());

    let on_create = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if is_creating.get() { return; }
        
        let raw_name = name.get().trim().to_string();
        let s_type = selected_sheet_type.get();
        let default_name = if s_type == "gods_and_monsters" { "Novo Familiar / Monstro" } else { "Novo Mago" };
        let final_name = if raw_name.is_empty() { default_name.to_string() } else { raw_name };
        
        set_is_creating.set(true);
        set_error_msg.set(None);
        
        let type_param = Some(s_type);
        spawn_local(async move {
            match create_sheet(final_name, type_param).await {
                Ok(id) => {
                    use_navigate()(&format!("/sheet/{}", id), Default::default());
                }
                Err(e) => {
                    log::error!("Error creating sheet: {:?}", e);
                    set_error_msg.set(Some(format!("Erro ao criar ficha: {}", e)));
                    set_is_creating.set(false);
                }
            }
        });
    };

    let toggle_privacy = move |sheet_id: String, current_public: bool| {
        spawn_local(async move {
            match set_sheet_visibility(sheet_id, !current_public).await {
                Ok(_) => {
                    sheets.refetch();
                    if home_tab.get_untracked() == "public_sheets" {
                        public_sheets.refetch();
                    }
                }
                Err(e) => {
                    set_error_msg.set(Some(format!("Erro ao alterar privacidade: {}", e)));
                }
            }
        });
    };

    let confirm_delete = move || {
        if let Some(target) = sheet_to_delete.get() {
            let id = target.id;
            set_sheet_to_delete.set(None);
            set_error_msg.set(None);
            spawn_local(async move {
                match delete_sheet(id).await {
                    Ok(_) => {
                        sheets.refetch();
                        if home_tab.get_untracked() == "public_sheets" {
                            public_sheets.refetch();
                        }
                    }
                    Err(e) => {
                        log::error!("Error deleting sheet: {:?}", e);
                        set_error_msg.set(Some(format!("Erro ao excluir ficha: {}", e)));
                    }
                }
            });
        }
    };

    let cancel_delete = move |_| {
        set_sheet_to_delete.set(None);
    };

    view! {
        <link rel="stylesheet" href="/style.css"/>
        <Navbar />
        <div class="home-container">
            <header class="home-header">
                <h1>"MTA Character Manager"</h1>
                <p>"Gerencie suas fichas de Mago: A Ascensão e Gods & Monsters com total privacidade"</p>
            </header>

            {move || error_msg.get().map(|msg| view! {
                <div class="alert-box alert-error">
                    <span>{msg}</span>
                    <button class="alert-close" on:click=move |_| set_error_msg.set(None)>"×"</button>
                </div>
            })}

            {move || if user.get().is_none() {
                view! {
                    <div class="visitor-banner">
                        <div class="visitor-banner-content">
                            <span class="visitor-banner-icon">"🔮"</span>
                            <div class="visitor-banner-text">
                                <h3>"Modo Visitante"</h3>
                                <p>"Suas fichas agora são 100% privadas. Conecte-se para criar, editar e acessar suas fichas salvas com segurança."</p>
                            </div>
                        </div>
                        <A href="/login" class="visitor-login-btn">"Entrar / Cadastrar"</A>
                    </div>
                }.into_view()
            } else {
                view! {
                    <section class="create-section">
                        <h2>"Criar Nova Ficha"</h2>
                        
                        <div class="sheet-type-selector">
                            <button
                                type="button"
                                class="type-pill-btn"
                                class:active=move || selected_sheet_type.get() == "mage"
                                on:click=move |_| set_selected_sheet_type.set("mage".to_string())
                            >
                                "🧙‍♂️ Mago: A Ascensão (4 Págs)"
                            </button>
                            <button
                                type="button"
                                class="type-pill-btn"
                                class:active=move || selected_sheet_type.get() == "gods_and_monsters"
                                on:click=move |_| set_selected_sheet_type.set("gods_and_monsters".to_string())
                            >
                                "🐉 Gods & Monsters (2 Págs)"
                            </button>
                        </div>

                        <form on:submit=move |ev| on_create(ev) class="create-form">
                            <input
                                type="text"
                                placeholder=move || if selected_sheet_type.get() == "gods_and_monsters" {
                                    "🐉 Nome do Familiar / Monstro (ex: Quimera de Hermes)"
                                } else {
                                    "🧙‍♂️ Nome do Personagem (ex: Hermes Trismegisto)"
                                }
                                on:input=move |ev| set_name.set(event_target_value(&ev))
                                prop:value=name
                                class="name-input"
                                disabled=is_creating
                            />
                            <button type="submit" class="create-btn" disabled=is_creating>
                                {move || if is_creating.get() { "✨ Criando..." } else { "+ Criar Ficha" }}
                            </button>
                        </form>
                    </section>
                }.into_view()
            }}

            <div class="home-tabs-container">
                {move || if user.get().is_some() {
                    view! {
                        <button
                            class="home-tab-btn"
                            class:active=move || home_tab.get() == "my_sheets"
                            on:click=move |_| set_home_tab.set("my_sheets")
                        >
                            "📜 Minhas Fichas"
                        </button>
                    }.into_view()
                } else {
                    ().into_view()
                }}
                <button
                    class="home-tab-btn"
                    class:active=move || home_tab.get() == "public_sheets" || user.get().is_none()
                    on:click=move |_| set_home_tab.set("public_sheets")
                >
                    "🌐 Fichas Públicas da Comunidade"
                </button>
            </div>

            <section class="list-section">
                {move || match home_tab.get() {
                    "public_sheets" => view! {
                        <Suspense fallback=move || view! { <p class="loading-msg">"Carregando fichas públicas..."</p> }>
                            {move || public_sheets.get().map(|res| match res {
                                Ok(data) if data.is_empty() => view! {
                                    <p class="empty-msg">"Nenhuma ficha pública encontrada na comunidade no momento."</p>
                                }.into_view(),
                                Ok(data) => render_character_grid(data, set_sheet_to_delete, toggle_privacy),
                                Err(e) => view! {
                                    <div class="alert-box alert-error">
                                        <p>"Erro ao carregar fichas públicas: " {e.to_string()}</p>
                                    </div>
                                }.into_view(),
                            })}
                        </Suspense>
                    }.into_view(),
                    _ => view! {
                        <Suspense fallback=move || view! { <p class="loading-msg">"Carregando suas fichas..."</p> }>
                            {move || sheets.get().map(|res| match res {
                                Ok(data) if data.is_empty() => view! { 
                                    <p class="empty-msg">"Nenhuma ficha privada encontrada. Crie uma nova ficha acima!"</p> 
                                }.into_view(),
                                Ok(data) => render_character_grid(data, set_sheet_to_delete, toggle_privacy),
                                Err(e) => view! { 
                                    <div class="alert-box alert-error">
                                        <p>"Erro ao carregar fichas: " {e.to_string()}</p>
                                    </div> 
                                }.into_view(),
                            })}
                        </Suspense>
                    }.into_view(),
                }}
            </section>

            // Delete Confirmation Modal
            {move || sheet_to_delete.get().map(|target| view! {
                <div class="modal-overlay" on:click=cancel_delete>
                    <div class="modal-card" on:click=move |ev| ev.stop_propagation()>
                        <h3 class="modal-title">"Confirmar Exclusão"</h3>
                        <p class="modal-text">
                            "Tem certeza que deseja excluir permanentemente a ficha de "
                            <strong>{target.name}</strong>"?"
                        </p>
                        <p class="modal-subtext">"Esta ação não pode ser desfeita."</p>
                        <div class="modal-actions">
                            <button class="modal-btn btn-cancel" on:click=cancel_delete>"Cancelar"</button>
                            <button class="modal-btn btn-danger" on:click=move |_| confirm_delete()>"Sim, Excluir"</button>
                        </div>
                    </div>
                </div>
            })}
        </div>
    }
}

fn render_character_grid<F>(
    data: Vec<CharacterSummary>,
    set_sheet_to_delete: WriteSignal<Option<CharacterSummary>>,
    toggle_privacy: F,
) -> View
where
    F: Fn(String, bool) + Copy + 'static,
{
    view! {
        <div class="character-cards-grid">
            {data.into_iter().map(|summary| {
                let summary_clone = summary.clone();
                let id = summary.id.clone();
                let id_vis = id.clone();
                let is_pub_val = summary.is_public;
                let is_owner = summary.is_owner;
                let photo = summary.photo_url.clone();
                let is_gm = summary.sheet_type == "gods_and_monsters";
                let has_photo = !photo.is_empty();
                let tradition_display = if !summary.tradition.is_empty() {
                    summary.tradition.clone()
                } else if is_gm {
                    "Familiar / Bygone".to_string()
                } else {
                    "Tradição não definida".to_string()
                };
                let essence_display = if !summary.essence.is_empty() {
                    summary.essence.clone()
                } else if is_gm {
                    "Gods & Monsters".to_string()
                } else {
                    "Mago Desperto".to_string()
                };
                let arete_val = summary.arete;
                let wp_val = summary.willpower;
                let date_display = summary.updated_at.split(' ').next().unwrap_or(&summary.updated_at).to_string();
                let updated_at_full = summary.updated_at.clone();
                let id_nav = id.clone();

                view! {
                    <div
                        class="character-card"
                        on:click=move |_| use_navigate()(&format!("/sheet/{}", id_nav), Default::default())
                    >
                        <div class="card-portrait-box">
                            {if has_photo {
                                view! {
                                    <img
                                        src=photo
                                        alt=summary.name.clone()
                                        class="card-portrait-img"
                                    />
                                }.into_view()
                            } else {
                                view! {
                                    <div class="card-portrait-placeholder">
                                        <span class="placeholder-icon">{if is_gm { "🐉" } else { "🔮" }}</span>
                                        <span class="placeholder-tag">{if is_gm { "Gods & Monsters" } else { "Sem Imagem" }}</span>
                                    </div>
                                }.into_view()
                            }}
                            <div class="card-portrait-gradient"></div>
                            {if is_owner {
                                view! {
                                    <button
                                        class="card-delete-floating-btn"
                                        on:click=move |ev: ev::MouseEvent| {
                                            ev.stop_propagation();
                                            set_sheet_to_delete.set(Some(summary_clone.clone()));
                                        }
                                        title="Excluir ficha"
                                    >
                                        "🗑️"
                                    </button>
                                }.into_view()
                            } else {
                                ().into_view()
                            }}
                        </div>

                        <div class="card-content">
                            <div class="card-header-info">
                                <h3 class="card-name" title=summary.name.clone()>{summary.name.clone()}</h3>
                                <div class="card-meta-tags">
                                    {if is_gm {
                                        view! {
                                            <span class="meta-tag type-badge-gm">"🐉 Gods & Monsters"</span>
                                        }.into_view()
                                    } else {
                                        view! {
                                            <span class="meta-tag type-badge-mage">"🧙‍♂️ Mago"</span>
                                        }.into_view()
                                    }}
                                    <span class="meta-tag tradition-tag">{tradition_display}</span>
                                    <span class="meta-tag essence-tag">{essence_display}</span>
                                    {if is_owner {
                                        view! {
                                            <button
                                                class=if is_pub_val { "meta-tag vis-tag vis-public" } else { "meta-tag vis-tag vis-private" }
                                                title=if is_pub_val { "Visível para a comunidade. Clique para tornar privada." } else { "Privada para você. Clique para tornar pública." }
                                                on:click=move |ev: ev::MouseEvent| {
                                                    ev.stop_propagation();
                                                    toggle_privacy(id_vis.clone(), is_pub_val);
                                                }
                                            >
                                                {if is_pub_val { "🌐 Pública" } else { "🔒 Privada" }}
                                            </button>
                                        }.into_view()
                                    } else {
                                        view! {
                                            <span class="meta-tag vis-tag vis-public">
                                                "🌐 Pública"
                                            </span>
                                        }.into_view()
                                    }}
                                </div>
                            </div>

                            <div class="card-stats-preview">
                                <div class="card-stat-item">
                                    <span class="stat-label">{if is_gm { "Gnose" } else { "Arete" }}</span>
                                    <div class="stat-dots arete-dots">
                                        {(1..=if is_gm { 10 } else { 5 }).map(|idx| {
                                            let filled = idx <= arete_val;
                                            view! {
                                                <span class=if filled { "stat-dot filled-arete" } else { "stat-dot empty-dot" }></span>
                                            }
                                        }).collect_view()}
                                    </div>
                                    <span class="stat-number">{arete_val}</span>
                                </div>

                                <div class="card-stat-item">
                                    <span class="stat-label">"Vontade"</span>
                                    <div class="stat-dots wp-dots">
                                        {(1..=10).map(|idx| {
                                            let filled = idx <= wp_val;
                                            view! {
                                                <span class=if filled { "stat-dot filled-wp" } else { "stat-dot empty-dot" }></span>
                                            }
                                        }).collect_view()}
                                    </div>
                                    <span class="stat-number">{wp_val}</span>
                                </div>
                            </div>

                            {if !is_gm {
                                view! {
                                    <div class="card-spheres-preview">
                                        <div class="spheres-header-row">
                                            <span class="spheres-label">"9 Esferas"</span>
                                        </div>
                                        <div class="spheres-9-grid">
                                            {summary.spheres.iter().map(|(sphere_name, lvl)| {
                                                let s_name = sphere_name.clone();
                                                let s_lvl = *lvl;
                                                let is_active = s_lvl > 0;
                                                view! {
                                                    <div
                                                        class=if is_active { "sphere-item-active" } else { "sphere-item-inactive" }
                                                        title=format!("{}: nível {}", s_name, s_lvl)
                                                    >
                                                        <span class="sphere-mini-name">{s_name}</span>
                                                        <div class="sphere-mini-dots">
                                                            {(1..=5).map(|dot_i| {
                                                                let filled = dot_i <= s_lvl;
                                                                view! {
                                                                    <span class=if filled { "stat-dot filled-sphere" } else { "stat-dot empty-dot" }></span>
                                                                }
                                                            }).collect_view()}
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <div class="card-gm-badge-footer">
                                        <span class="gm-creature-desc">"🐉 Entidade Sobrenatural (Familiar / Bygone / Espírito)"</span>
                                    </div>
                                }.into_view()
                            }}

                            <div class="card-footer">
                                <span class="card-date" title=format!("Última alteração: {}", updated_at_full)>
                                    "Atualizado: " {date_display}
                                </span>
                                <span class="card-cta">"Abrir Ficha →"</span>
                            </div>
                        </div>
                    </div>
                }
            }).collect_view()}
        </div>
    }.into_view()
}
