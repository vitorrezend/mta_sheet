use leptos::*;
use leptos_router::*;
use crate::state::get_sheets;

#[component]
pub fn Home() -> impl IntoView {
    let create_sheet_action = create_server_action::<crate::state::CreateSheet>();
    let delete_sheet_action = create_server_action::<crate::state::DeleteSheet>();

    let sheets = create_resource(
        move || (create_sheet_action.version().get(), delete_sheet_action.version().get()),
        |_| async move { get_sheets().await }
    );

    let (name, set_name) = create_signal(String::new());

    let on_create = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let name_val = name.get();
        if !name_val.is_empty() {
            create_sheet_action.dispatch(crate::state::CreateSheet { name: name_val });
            set_name.set(String::new());
        }
    };

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
                <p>"Gerencie suas fichas de Mago: A Ascensão"</p>
            </header>

            <section class="create-section">
                <h2>"Criar Nova Ficha"</h2>
                <form on:submit=on_create class="create-form">
                    <input
                        type="text"
                        placeholder="Nome do Personagem"
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        prop:value=name
                        required
                    />
                    <button type="submit" disabled=move || create_sheet_action.pending().get()>
                        {move || if create_sheet_action.pending().get() { "Criando..." } else { "Criar" }}
                    </button>
                </form>
            </section>

            <section class="list-section">
                <h2>"Fichas Existentes"</h2>
                <Suspense fallback=move || view! { <p class="loading">"Carregando fichas..."</p> }>
                    {move || sheets.get().map(|res| match res {
                        Ok(data) if data.is_empty() => view! { <p class="empty-msg">"Nenhuma ficha encontrada. Crie sua primeira ficha acima!"</p> }.into_view(),
                        Ok(data) => view! {
                            <div class="sheet-list-container">
                                <ul class="sheet-list">
                                    {data.into_iter().map(|summary| {
                                        let id = summary.id.clone();
                                        let id_for_delete = summary.id.clone();
                                        let name = summary.name.clone();
                                        view! {
                                            <li class="sheet-item">
                                                <A href=format!("/sheet/{}", id) class="sheet-link">
                                                    <div class="sheet-info">
                                                        <span class="sheet-name">{name}</span>
                                                        <span class="sheet-date">"Atualizado em: " {summary.updated_at}</span>
                                                    </div>
                                                </A>
                                                <button
                                                    class="delete-btn"
                                                    title="Excluir Ficha"
                                                    on:click=move |ev| {
                                                        ev.prevent_default();
                                                        if window().confirm_with_message(&format!("Deseja realmente excluir a ficha de {}?", summary.name)).unwrap_or(false) {
                                                            delete_sheet_action.dispatch(crate::state::DeleteSheet { id: id_for_delete.clone() });
                                                        }
                                                    }
                                                >
                                                    "✕"
                                                </button>
                                            </li>
                                        }
                                    }).collect_view()}
                                </ul>
                            </div>
                        }.into_view(),
                        Err(e) => view! { <p class="error">"Erro ao carregar fichas: " {e.to_string()}</p> }.into_view(),
                    })}
                </Suspense>
            </section>
        </div>
    }
}
