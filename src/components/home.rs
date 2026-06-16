use leptos::*;
use leptos_router::*;
use crate::state::{get_sheets, CreateSheet, delete_sheet};

#[component]
pub fn Home() -> impl IntoView {
    let sheets = create_resource(|| (), |_| async move { get_sheets().await });

    let create_action = create_server_action::<CreateSheet>();
    let navigate = use_navigate();

    // Effect to navigate after successful creation
    create_effect(move |_| {
        if let Some(Ok(id)) = create_action.value().get() {
            navigate(&format!("/sheet/{}", id), Default::default());
        }
    });

    let on_delete = move |id: String| {
        let confirm = window().confirm_with_message("Tem certeza que deseja excluir esta ficha?");
        if confirm.unwrap_or(false) {
            spawn_local(async move {
                match delete_sheet(id).await {
                    Ok(_) => {
                        sheets.refetch();
                    }
                    Err(e) => {
                        logging::log!("Error deleting sheet: {:?}", e);
                    }
                }
            });
        }
    };

    view! {
        <link rel="stylesheet" href="/style.css"/>
        <div class="home-container">
            <header class="home-header">
                <div class="logo-container">
                    <span class="logo-text">"MT"</span>
                    <span class="logo-sub">"A"</span>
                </div>
                <h1>"Mago: A Ascensão"</h1>
                <p>"Gerencie suas fichas com persistência SQLite"</p>
            </header>

            <section class="create-section">
                <h2>"Nova Ficha"</h2>
                <ActionForm action=create_action class="create-form">
                    <input
                        type="text"
                        name="name"
                        placeholder="Nome do Personagem (ex: Morpheus)"
                        class="name-input"
                        required
                    />
                    <button type="submit" class="create-btn" disabled=create_action.pending()>
                        {move || if create_action.pending().get() { "Criando..." } else { "Criar Ficha" }}
                    </button>
                </ActionForm>
                {move || create_action.value().get().map(|res| match res {
                    Err(e) => view! { <p class="error-msg">{format!("Erro: {}", e)}</p> }.into_view(),
                    _ => view! {}.into_view(),
                })}
            </section>

            <section class="list-section">
                <h2>"Minhas Fichas"</h2>
                <Suspense fallback=move || view! { <p class="loading-msg">"Carregando fichas salvas..."</p> }>
                    {move || sheets.get().map(|res| match res {
                        Ok(data) if data.is_empty() => view! {
                            <div class="empty-state">
                                <p class="empty-msg">"Nenhuma ficha encontrada."</p>
                                <p class="empty-sub">"Comece sua jornada criando uma ficha acima."</p>
                            </div>
                        }.into_view(),
                        Ok(data) => view! {
                            <ul class="sheet-list">
                                {data.into_iter().map(|summary| {
                                    let id = summary.id.clone();
                                    let delete_id = id.clone();
                                    view! {
                                        <li class="sheet-item">
                                            <div class="sheet-link-wrapper">
                                                <A href=format!("/sheet/{}", id) class="sheet-item-link">
                                                    <div class="sheet-info">
                                                        <span class="sheet-name">{summary.name}</span>
                                                        <span class="sheet-date">"Última alteração: " {summary.updated_at}</span>
                                                    </div>
                                                </A>
                                            </div>
                                            <div class="sheet-actions">
                                                <button
                                                    class="delete-btn"
                                                    on:click=move |ev| {
                                                        ev.stop_propagation();
                                                        on_delete(delete_id.clone());
                                                    }
                                                    title="Excluir ficha"
                                                >
                                                    "×"
                                                </button>
                                            </div>
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
                        }.into_view(),
                        Err(e) => view! { <p class="error">"Erro ao carregar fichas: " {e.to_string()}</p> }.into_view(),
                    })}
                </Suspense>
            </section>
        </div>
    }
}
