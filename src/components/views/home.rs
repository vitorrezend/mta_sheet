use leptos::*;
use leptos_router::*;
use crate::state::{get_sheets, create_sheet, delete_sheet, CharacterSummary};
use crate::components::Navbar;

#[component]
pub fn Home() -> impl IntoView {
    let sheets = create_resource(|| (), |_| async move { get_sheets().await });
    let (name, set_name) = create_signal(String::new());
    let (error_msg, set_error_msg) = create_signal(Option::<String>::None);
    let (sheet_to_delete, set_sheet_to_delete) = create_signal(Option::<CharacterSummary>::None);
    let (is_creating, set_is_creating) = create_signal(false);

    let navigate = use_navigate();

    let on_create = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let raw_name = name.get().trim().to_string();
        let final_name = if raw_name.is_empty() {
            "Novo Mago".to_string()
        } else {
            raw_name
        };
        let navigate = navigate.clone();
        set_is_creating.set(true);
        set_error_msg.set(None);
        spawn_local(async move {
            match create_sheet(final_name).await {
                Ok(id) => {
                    navigate(&format!("/sheet/{}", id), Default::default());
                }
                Err(e) => {
                    log::error!("Error creating sheet: {:?}", e);
                    set_error_msg.set(Some(format!("Erro ao criar ficha: {}", e)));
                    set_is_creating.set(false);
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
                <p>"Gerencie suas fichas de Mago: A Ascensão"</p>
            </header>

            {move || error_msg.get().map(|msg| view! {
                <div class="alert-box alert-error">
                    <span>{msg}</span>
                    <button class="alert-close" on:click=move |_| set_error_msg.set(None)>"×"</button>
                </div>
            })}

            <section class="create-section">
                <h2>"Criar Nova Ficha"</h2>
                <form on:submit=on_create class="create-form">
                    <input
                        type="text"
                        placeholder="🧙‍♂️ Nome do Personagem (ex: Hermes Trismegisto)"
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

            <section class="list-section">
                <h2>"Fichas Salvas"</h2>
                <Suspense fallback=move || view! { <p class="loading-msg">"Carregando fichas..."</p> }>
                    {move || sheets.get().map(|res| match res {
                        Ok(data) if data.is_empty() => view! { 
                            <p class="empty-msg">"Nenhuma ficha encontrada. Comece criando uma nova acima!"</p> 
                        }.into_view(),
                        Ok(data) => view! {
                            <div class="character-cards-grid">
                                {data.into_iter().map(|summary| {
                                    let summary_clone = summary.clone();
                                    let id = summary.id.clone();
                                    let photo = summary.photo_url.clone();
                                    let has_photo = !photo.is_empty();
                                    let tradition_display = if !summary.tradition.is_empty() {
                                        summary.tradition.clone()
                                    } else {
                                        "Tradição não definida".to_string()
                                    };
                                    let essence_display = if !summary.essence.is_empty() {
                                        summary.essence.clone()
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
                                                            <span class="placeholder-icon">"🔮"</span>
                                                            <span class="placeholder-tag">"Sem Imagem"</span>
                                                        </div>
                                                    }.into_view()
                                                }}
                                                <div class="card-portrait-gradient"></div>
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
                                            </div>

                                            <div class="card-content">
                                                <div class="card-header-info">
                                                    <h3 class="card-name" title=summary.name.clone()>{summary.name.clone()}</h3>
                                                    <div class="card-meta-tags">
                                                        <span class="meta-tag tradition-tag">{tradition_display}</span>
                                                        <span class="meta-tag essence-tag">{essence_display}</span>
                                                    </div>
                                                </div>

                                                <div class="card-stats-preview">
                                                    <div class="card-stat-item">
                                                        <span class="stat-label">"Arete"</span>
                                                        <div class="stat-dots arete-dots">
                                                            {(1..=5).map(|idx| {
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

                                                <div class="card-footer">
                                                    <span class="card-date" title=format!("Atualizado em {}", updated_at_full)>
                                                        "🕒 " {date_display}
                                                    </span>
                                                    <span class="card-access-cta">
                                                        "Acessar Ficha →"
                                                    </span>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_view(),
                        Err(e) => view! { 
                            <div class="alert-box alert-error">
                                <p>"Erro ao carregar fichas: " {e.to_string()}</p>
                            </div> 
                        }.into_view(),
                    })}
                </Suspense>
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
