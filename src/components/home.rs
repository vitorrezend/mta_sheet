use leptos::*;
use leptos_router::*;
use crate::state::{get_sheets, create_sheet, delete_sheet};

#[component]
pub fn Home() -> impl IntoView {
    let sheets = create_resource(|| (), |_| async move { get_sheets().await });
    let (name, set_name) = create_signal(String::new());
    let (creating, set_creating) = create_signal(false);

    // Obter navigate no escopo do componente
    let navigate = use_navigate();

    let on_create = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let name_val = name.get();
        let navigate = navigate.clone();
        if !name_val.is_empty() && !creating.get() {
            set_creating.set(true);
            spawn_local(async move {
                match create_sheet(name_val).await {
                    Ok(id) => {
                        navigate(&format!("/sheet/{}", id), Default::default());
                    }
                    Err(e) => {
                        set_creating.set(false);
                        logging::log!("Error creating sheet: {:?}", e);
                    }
                }
            });
        }
    };

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
                <div class="logo-placeholder">"MTA"</div>
                <h1>"MTA Character Manager"</h1>
                <p>"Gerencie suas fichas de Mago: A Ascensão de forma simples e segura."</p>
                <div class="project-info">
                    <p>"Esta aplicação permite criar, visualizar e editar fichas de personagem. Todos os dados são persistidos localmente em um banco de dados SQLite em formato JSON, garantindo que suas alterações sejam salvas automaticamente."</p>
                </div>
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
                    <button type="submit" class="create-btn" disabled=creating>
                        {move || if creating.get() { "Criando..." } else { "Criar" }}
                    </button>
                </form>
            </section>

            <section class="list-section">
                <h2>"Fichas Salvas"</h2>
                <Suspense fallback=move || view! { <p>"Carregando..."</p> }>
                    {move || sheets.get().map(|res| match res {
                        Ok(data) if data.is_empty() => view! { <p class="empty-msg">"Nenhuma ficha encontrada. Comece criando uma nova!"</p> }.into_view(),
                        Ok(data) => view! {
                            <ul class="sheet-list">
                                {data.into_iter().map(|summary| {
                                    let id = summary.id.clone();
                                    let open_id = id.clone();
                                    let delete_id = id.clone();
                                    view! {
                                        <li class="sheet-item">
                                            <div class="sheet-info">
                                                <span class="sheet-name">{summary.name}</span>
                                                <span class="sheet-date">{summary.updated_at}</span>
                                            </div>
                                            <div class="sheet-actions">
                                                <A href=format!("/sheet/{}", open_id) class="open-btn">
                                                    "Abrir"
                                                </A>
                                                <button
                                                    class="delete-btn"
                                                    on:click=move |_| on_delete(delete_id.clone())
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
