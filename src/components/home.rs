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
                        placeholder="Nome do Personagem (opcional, padrão: 'Novo Mago')"
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        prop:value=name
                        class="name-input"
                        disabled=is_creating
                    />
                    <button type="submit" class="create-btn" disabled=is_creating>
                        {move || if is_creating.get() { "Criando..." } else { "Criar Ficha" }}
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
                            <ul class="sheet-list">
                                {data.into_iter().map(|summary| {
                                    let summary_clone = summary.clone();
                                    let id = summary.id.clone();
                                    view! {
                                        <li class="sheet-item">
                                            <A href=format!("/sheet/{}", id) class="sheet-link">
                                                <div class="sheet-info">
                                                    <span class="sheet-name">{summary.name}</span>
                                                    <span class="sheet-date">{summary.updated_at}</span>
                                                </div>
                                            </A>
                                            <button
                                                class="delete-btn"
                                                on:click=move |_| set_sheet_to_delete.set(Some(summary_clone.clone()))
                                                title="Excluir ficha"
                                            >
                                                "×"
                                            </button>
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
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
