use leptos::*;
use leptos_router::*;
use crate::state::{get_sheets, CreateSheet, DeleteSheet};

#[component]
pub fn Home() -> impl IntoView {
    let create_action = create_server_action::<CreateSheet>();
    let delete_action = create_server_action::<DeleteSheet>();

    // Reload the list whenever a create or delete action completes
    let sheets = create_resource(
        move || (create_action.version().get(), delete_action.version().get()),
        |_| async move { get_sheets().await }
    );

    let (name, set_name) = create_signal(String::new());

    // Navigate to new sheet when created
    create_effect(move |_| {
        if let Some(Ok(id)) = create_action.value().get() {
            let navigate = use_navigate();
            navigate(&format!("/sheet/{}", id), Default::default());
        }
    });

    let on_create = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let name_val = name.get();
        if !name_val.is_empty() {
            create_action.dispatch(CreateSheet { name: name_val });
            set_name.set(String::new());
        }
    };

    view! {
        <div class="home-container">
            <header class="home-header">
                <h1>"MTA Sheet Manager"</h1>
            </header>

            <section class="create-section">
                <h2>"Criar Nova Ficha"</h2>
                <form on:submit=on_create>
                    <input
                        type="text"
                        placeholder="Nome do Personagem"
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        prop:value=name
                    />
                    <button type="submit" disabled=move || create_action.pending().get()>
                        {move || if create_action.pending().get() { "Criando..." } else { "Criar" }}
                    </button>
                </form>
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
                                            <A href=format!("/sheet/{}", id)>
                                                <span class="sheet-name">{summary.name}</span>
                                                <span class="sheet-date">{summary.updated_at}</span>
                                            </A>
                                            <button
                                                class="delete-btn"
                                                on:click=move |_| {
                                                    delete_action.dispatch(DeleteSheet { id: id_for_delete.clone() });
                                                }
                                            >
                                                "Excluir"
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
