use leptos::*;
use leptos_router::*;
use crate::state::{get_sheets, create_sheet, delete_sheet};

#[component]
pub fn Home() -> impl IntoView {
    let create_sheet_action = create_server_action::<CreateSheetAction>();
    let delete_sheet_action = create_server_action::<DeleteSheetAction>();

    let sheets = create_resource(
        move || (create_sheet_action.version().get(), delete_sheet_action.version().get()),
        |_| async move { get_sheets().await }
    );

    create_effect(move |_| {
        if let Some(Ok(id)) = create_sheet_action.value().get() {
            let navigate = use_navigate();
            navigate(&format!("/sheet/{}", id), Default::default());
        }
    });

    view! {
        <div class="home-container">
            <header class="home-header">
                <h1>"MTA Sheet Manager"</h1>
            </header>

            <section class="create-section">
                <h2>"Criar Nova Ficha"</h2>
                <ActionForm action=create_sheet_action>
                    <input
                        type="text"
                        name="name"
                        placeholder="Nome do Personagem"
                        required
                    />
                    <button type="submit">"Criar"</button>
                </ActionForm>
            </section>

            <section class="list-section">
                <h2>"Fichas Existentes"</h2>
                <Suspense fallback=move || view! { <p>"Carregando..."</p> }>
                    {move || sheets.get().map(|res| match res {
                        Ok(data) if data.is_empty() => view! { <p>"Nenhuma ficha encontrada."</p> }.into_view(),
                        Ok(data) => view! {
                            <ul class="sheet-list">
                                {data.into_iter().map(|summary| {
                                    let id = summary.id.clone();
                                    let id_for_delete = id.clone();
                                    view! {
                                        <li>
                                            <div class="sheet-item">
                                                <A href=format!("/sheet/{}", id)>
                                                    <span class="sheet-name">{summary.name}</span>
                                                    <span class="sheet-date">{summary.updated_at}</span>
                                                </A>
                                                <ActionForm action=delete_sheet_action>
                                                    <input type="hidden" name="id" value=id_for_delete />
                                                    <button type="submit" class="delete-btn">"Deletar"</button>
                                                </ActionForm>
                                            </div>
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
                        }.into_view(),
                        Err(e) => view! { <p class="error">"Erro: " {e.to_string()}</p> }.into_view(),
                    })}
                </Suspense>
            </section>
        </div>
    }
}

#[server(CreateSheetAction, "/api")]
pub async fn create_sheet_action(name: String) -> Result<String, ServerFnError> {
    create_sheet(name).await
}

#[server(DeleteSheetAction, "/api")]
pub async fn delete_sheet_action(id: String) -> Result<(), ServerFnError> {
    delete_sheet(id).await
}
