use leptos::*;
use leptos_router::*;
use crate::state::{get_sheets, CreateSheet, DeleteSheet};

#[component]
pub fn Home() -> impl IntoView {
    let create_action = create_server_action::<CreateSheet>();
    let delete_action = create_server_action::<DeleteSheet>();

    let sheets = create_resource(
        move || (create_action.version().get(), delete_action.version().get()),
        |_| async move { get_sheets().await }
    );

    let (name, set_name) = create_signal(String::new());

    // Redirect after creation
    create_effect(move |_| {
        if let Some(Ok(id)) = create_action.value().get() {
            let navigate = use_navigate();
            navigate(&format!("/sheet/{}", id), Default::default());
        }
    });

    view! {
        <div class="home-container">
            <header class="home-header">
                <h1>"MTA Sheet Manager"</h1>
                <p>"Gerencie suas fichas de Mago: A Ascensão"</p>
            </header>

            <main class="home-content">
                <section class="create-section">
                    <h2>"Nova Ficha"</h2>
                    <ActionForm action=create_action class="create-form">
                        <input
                            type="text"
                            name="name"
                            placeholder="Nome do Personagem"
                            on:input=move |ev| set_name.set(event_target_value(&ev))
                            prop:value=name
                            required
                        />
                        <button type="submit">
                            {move || if create_action.pending().get() { "Criando..." } else { "Criar Ficha" }}
                        </button>
                    </ActionForm>
                </section>

                <section class="list-section">
                    <h2>"Suas Fichas"</h2>
                    <Suspense fallback=move || view! { <div class="loading">"Carregando fichas..."</div> }>
                        {move || sheets.get().map(|res| match res {
                            Ok(data) if data.is_empty() => view! {
                                <div class="empty-state">
                                    <p>"Você ainda não possui nenhuma ficha criada."</p>
                                </div>
                            }.into_view(),
                            Ok(data) => view! {
                                <div class="sheet-grid">
                                    {data.into_iter().map(|summary| {
                                        let id = summary.id.clone();
                                        let id_for_delete = id.clone();
                                        let name = summary.name.clone();
                                        view! {
                                            <div class="sheet-card">
                                                <A href=format!("/sheet/{}", id) class="sheet-link">
                                                    <span class="sheet-name">{name}</span>
                                                    <span class="sheet-date">"Atualizado em: " {summary.updated_at}</span>
                                                </A>
                                                <ActionForm action=delete_action>
                                                    <input type="hidden" name="id" value=id_for_delete />
                                                    <button
                                                        type="submit"
                                                        class="delete-btn"
                                                        title="Excluir ficha"
                                                        on:click=move |ev| {
                                                            if !window().confirm_with_message(&format!("Deseja realmente excluir a ficha de {}?", summary.name)).unwrap_or(false) {
                                                                ev.prevent_default();
                                                            }
                                                        }
                                                    >
                                                        "×"
                                                    </button>
                                                </ActionForm>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_view(),
                            Err(e) => view! { <div class="error">"Erro ao carregar fichas: " {e.to_string()}</div> }.into_view(),
                        })}
                    </Suspense>
                </section>
            </main>
        </div>
    }
}
