use leptos::*;
use leptos_router::*;
use crate::state::{get_sheets, create_sheet, delete_sheet};

#[component]
pub fn Home() -> impl IntoView {
    let (sheets_version, set_sheets_version) = create_signal(0);
    let sheets = create_resource(move || sheets_version.get(), |_| async move { get_sheets().await });
    let (name, set_name) = create_signal(String::new());

    let on_create = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let name_val = name.get();
        if !name_val.is_empty() {
            spawn_local(async move {
                match create_sheet(name_val).await {
                    Ok(id) => {
                        let navigate = use_navigate();
                        navigate(&format!("/sheet/{}", id), Default::default());
                    }
                    Err(e) => {
                        logging::log!("Error creating sheet: {:?}", e);
                    }
                }
            });
        }
    };

    let on_delete = move |id: String| {
        spawn_local(async move {
            match delete_sheet(id).await {
                Ok(_) => {
                    set_sheets_version.update(|v| *v += 1);
                }
                Err(e) => {
                    logging::log!("Error deleting sheet: {:?}", e);
                }
            }
        });
    };

    view! {
        <div class="home-container">
            <header class="home-header">
                <h1>"MTA Sheet Manager"</h1>
            </header>

            <div class="home-content">
                <section class="create-section">
                    <h2>"Nova Ficha"</h2>
                    <form on:submit=on_create class="create-form">
                        <input
                            type="text"
                            placeholder="Nome do Personagem"
                            on:input=move |ev| set_name.set(event_target_value(&ev))
                            prop:value=name
                        />
                        <button type="submit" class="btn-primary">"Criar"</button>
                    </form>
                </section>

                <section class="list-section">
                    <h2>"Minhas Fichas"</h2>
                    <Suspense fallback=move || view! { <p class="loading">"Carregando..."</p> }>
                        {move || sheets.get().map(|res| match res {
                            Ok(data) if data.is_empty() => view! { <p class="empty-msg">"Nenhuma ficha encontrada."</p> }.into_view(),
                            Ok(data) => view! {
                                <div class="sheet-grid">
                                    {data.into_iter().map(|summary| {
                                        let id = summary.id.clone();
                                        let delete_id = id.clone();
                                        view! {
                                            <div class="sheet-card">
                                                <div class="sheet-card-info">
                                                    <span class="sheet-card-name">{summary.name}</span>
                                                    <span class="sheet-card-date">{summary.updated_at}</span>
                                                </div>
                                                <div class="sheet-card-actions">
                                                    <A href=format!("/sheet/{}", id) class="btn-open">"Abrir"</A>
                                                    <button
                                                        on:click=move |_| on_delete(delete_id.clone())
                                                        class="btn-delete"
                                                    >
                                                        "Excluir"
                                                    </button>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_view(),
                            Err(e) => view! { <p class="error">"Erro: " {e.to_string()}</p> }.into_view(),
                        })}
                    </Suspense>
                </section>
            </div>
        </div>
    }
}
