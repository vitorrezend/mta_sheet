use leptos::*;
use leptos_router::*;
use crate::state::{get_sheets, create_sheet, delete_sheet};

#[component]
pub fn Home() -> impl IntoView {
    let sheets = create_resource(|| (), |_| async move { get_sheets().await });
    let (name, set_name) = create_signal(String::new());
    let navigate = use_navigate();

    let on_create = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let name_val = name.get();
        if !name_val.is_empty() {
            let navigate = navigate.clone();
            spawn_local(async move {
                match create_sheet(name_val).await {
                    Ok(id) => {
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
                    sheets.refetch();
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
                <h1>"Mago: A Ascensão"</h1>
                <p>"Gerenciador de Fichas de Personagem"</p>
            </header>

            <section class="create-section">
                <h2>"Criar Nova Ficha"</h2>
                <form on:submit=on_create class="create-form">
                    <input
                        type="text"
                        placeholder="Nome do Personagem"
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        prop:value=name
                        class="name-input"
                    />
                    <button type="submit" class="create-btn">"Criar"</button>
                </form>
            </section>

            <section class="list-section">
                <h2>"Fichas Existentes"</h2>
                <Suspense fallback=move || view! { <p>"Carregando..."</p> }>
                    {move || sheets.get().map(|res| match res {
                        Ok(data) if data.is_empty() => view! { <p class="empty-msg">"Nenhuma ficha encontrada."</p> }.into_view(),
                        Ok(data) => view! {
                            <ul class="sheet-list">
                                {data.into_iter().map(|summary| {
                                    let id = summary.id.clone();
                                    let id_for_delete = id.clone();
                                    view! {
                                        <li class="sheet-item">
                                            <A href=format!("/sheet/{}", id) class="sheet-link">
                                                <div class="sheet-info">
                                                    <span class="sheet-name">{summary.name}</span>
                                                    <span class="sheet-date">"Atualizado em: " {summary.updated_at}</span>
                                                </div>
                                            </A>
                                            <button
                                                class="delete-btn"
                                                on:click=move |ev| {
                                                    ev.stop_propagation();
                                                    on_delete(id_for_delete.clone());
                                                }
                                                title="Excluir Ficha"
                                            >
                                                "✕"
                                            </button>
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
