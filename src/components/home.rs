use leptos::*;
use leptos_router::*;
use crate::state::get_sheets;

#[component]
pub fn Home() -> impl IntoView {
    let create_action = create_server_action::<crate::state::CreateSheet>();
    let delete_action = create_server_action::<crate::state::DeleteSheet>();

    let sheets = create_resource(
        move || (create_action.version().get(), delete_action.version().get()),
        |_| async move { get_sheets().await }
    );

    let (name, set_name) = create_signal(String::new());

    let on_create = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let name_val = name.get();
        if !name_val.is_empty() {
            create_action.dispatch(crate::state::CreateSheet { name: name_val });
            set_name.set(String::new());
        }
    };

    create_effect(move |_| {
        if let Some(Ok(id)) = create_action.value().get() {
            let navigate = use_navigate();
            navigate(&format!("/sheet/{}", id), Default::default());
        }
    });

    view! {
        <div class="home-container">
            <header class="home-hero">
                <div class="hero-content">
                    <h1>"MTA Sheet Manager"</h1>
                    <p>"Gerencie suas fichas de Mago: A Ascensão de forma simples e organizada."</p>

                    <form on:submit=on_create class="create-form">
                        <input
                            type="text"
                            placeholder="Nome do Novo Personagem"
                            on:input=move |ev| set_name.set(event_target_value(&ev))
                            prop:value=name
                            required
                        />
                        <button type="submit" class="btn-create">
                            "Criar Nova Ficha"
                        </button>
                    </form>
                </div>
            </header>

            <main class="home-main">
                <section class="list-section">
                    <h2>"Suas Fichas"</h2>
                    <Suspense fallback=move || view! { <div class="loading">"Carregando fichas..."</div> }>
                        {move || sheets.get().map(|res| match res {
                            Ok(data) if data.is_empty() => view! {
                                <div class="empty-state">
                                    <p>"Você ainda não possui fichas criadas."</p>
                                </div>
                            }.into_view(),
                            Ok(data) => view! {
                                <div class="sheet-grid">
                                    {data.into_iter().map(|summary| {
                                        let id = summary.id.clone();
                                        let delete_id = id.clone();
                                        view! {
                                            <div class="sheet-card">
                                                <A href=format!("/sheet/{}", id) class="card-link">
                                                    <div class="card-icon">"📜"</div>
                                                    <div class="card-info">
                                                        <span class="sheet-name">{summary.name}</span>
                                                        <span class="sheet-date">"Atualizada em: " {summary.updated_at}</span>
                                                    </div>
                                                </A>
                                                <button
                                                    class="btn-delete"
                                                    on:click=move |_| {
                                                        delete_action.dispatch(crate::state::DeleteSheet { id: delete_id.clone() });
                                                    }
                                                    title="Excluir ficha"
                                                >
                                                    "×"
                                                </button>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_view(),
                            Err(e) => view! { <p class="error">"Erro ao carregar fichas: " {e.to_string()}</p> }.into_view(),
                        })}
                    </Suspense>
                </section>
            </main>
        </div>
    }
}
