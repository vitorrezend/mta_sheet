use leptos::*;
use leptos_router::*;
use crate::state::{get_sheets, CreateSheet, delete_sheet};

#[component]
pub fn Home() -> impl IntoView {
    let sheets = create_resource(|| (), |_| async move { get_sheets().await });

    let create_sheet_action = create_server_action::<CreateSheet>();
    let navigate = use_navigate();

    create_effect(move |_| {
        if let Some(Ok(id)) = create_sheet_action.value().get() {
            navigate(&format!("/sheet/{}", id), Default::default());
        }
    });

    let (name, set_name) = create_signal(String::new());

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
                    <span class="logo-icon">"✨"</span>
                    <h1>"Gerenciador de Fichas MTA"</h1>
                </div>
                <p class="subtitle">"Mago: A Ascensão — Crônicas de Magia e Mistério"</p>
                <div class="header-description">
                    <p>"Bem-vindo ao seu santuário digital. Aqui você pode forjar novos despertos ou retomar jornadas já iniciadas."</p>
                    <p class="db-info">"Os dados são persistidos localmente em um banco SQLite."</p>
                </div>
            </header>

            <section class="create-section">
                <h2>"Criar Nova Ficha"</h2>
                <ActionForm action=create_sheet_action class="create-form">
                    <input
                        type="text"
                        name="name"
                        placeholder="Nome do Personagem"
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        prop:value=name
                        class="name-input"
                        required
                    />
                    <button type="submit" class="create-btn">
                        {move || if create_sheet_action.pending().get() { "Criando..." } else { "Criar" }}
                    </button>
                </ActionForm>
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
