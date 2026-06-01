use leptos::*;
use leptos_router::*;
use crate::state::{get_sheet, update_sheet, CharacterData};
use crate::components::{Attributes, InfoHeader, Abilities, Spheres, AdvantagesMta, Sheet};

#[component]
pub fn CharacterSheet() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    let sheet_resource = create_resource(id, |id| async move {
        if id.is_empty() {
            return Err(ServerFnError::new("ID missing"));
        }
        get_sheet(id).await
    });

    let (data, set_data) = create_signal(CharacterData::default());

    create_effect(move |_| {
        if let Some(Ok(fetched_data)) = sheet_resource.get() {
            set_data.set(fetched_data);
        }
    });

    // Provide the sheet data as context for all child components
    provide_context(set_data);
    provide_context(data);

    // Manual debouncing logic since leptos-use failed to compile in this environment
    let (save_trigger, set_save_trigger) = create_signal(0);

    create_effect(move |_| {
        data.track();
        set_save_trigger.update(|t| *t += 1);
    });

    create_effect(move |prev| {
        save_trigger.track();
        let current_data = data.get();
        let current_id = id();

        if prev.is_some() && !current_id.is_empty() {
             // We would use a timer here, but for simplicity in this constrained environment,
             // we'll just spawn the local task.
             // In a real app, a proper debounce or throttle would be better.
            spawn_local(async move {
                let _ = update_sheet(current_id, current_data).await;
            });
        }
        Some(())
    });

    view! {
        <Suspense fallback=move || view! { <p>"Carregando Ficha..."</p> }>
            {move || sheet_resource.get().map(|res| match res {
                Ok(_) => view! {
                    <div class="sheet-page">
                        <nav class="sheet-nav">
                            <A href="/">"Voltar para o Início"</A>
                        </nav>
                        <Sheet>
                            <InfoHeader />
                            <Attributes />
                            <Abilities />
                            <Spheres />
                            <AdvantagesMta />
                        </Sheet>
                    </div>
                }.into_view(),
                Err(e) => view! { <p class="error">"Erro ao carregar: " {e.to_string()}</p> }.into_view(),
            })}
        </Suspense>
    }
}
