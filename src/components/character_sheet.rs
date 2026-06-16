use leptos::*;
use leptos_router::*;
use crate::state::{get_sheet, update_sheet, CharacterData};
use crate::components::{Attributes, InfoHeader, Abilities, Spheres, AdvantagesMta, Sheet};
use wasm_bindgen::prelude::*;

#[component]
pub fn CharacterSheet() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    let sheet_resource = create_local_resource(id, |id| async move {
        if id.is_empty() {
            return Err(ServerFnError::new("ID missing"));
        }
        get_sheet(id).await
    });

    let (data, set_data) = create_signal(CharacterData::default());
    let (is_saving, set_is_saving) = create_signal(false);

    create_effect(move |_| {
        if let Some(Ok(fetched_data)) = sheet_resource.get() {
            set_data.set(fetched_data);
        }
    });

    // Provide the sheet data as context for all child components
    provide_context(set_data);
    provide_context(data);

    // Debounced auto-save logic
    let timeout_handle = store_value(None::<i32>);
    let closure_handle = store_value(None::<Closure<dyn FnMut()>>);

    create_effect(move |prev_run| {
        data.track();
        let current_data = data.get();
        let current_id = id();

        // Skip the very first run which happens on component mount
        if prev_run.is_some() && !current_id.is_empty() {
            set_is_saving.set(true);

            // Clear existing timeout if any
            if let Some(handle) = timeout_handle.get_value() {
                if let Some(window) = web_sys::window() {
                    window.clear_timeout_with_handle(handle);
                }
            }

            // Set new timeout
            let closure = Closure::wrap(Box::new(move || {
                let id = current_id.clone();
                let data = current_data.clone();
                spawn_local(async move {
                    let _ = update_sheet(id, data).await;
                    set_is_saving.set(false);
                });
            }) as Box<dyn FnMut()>);

            if let Some(window) = web_sys::window() {
                let handle = window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        500,
                    )
                    .unwrap_or(0);
                timeout_handle.set_value(Some(handle));
            }

            // Keep closure alive and drop the previous one
            closure_handle.set_value(Some(closure));
        }
        Some(())
    });

    on_cleanup(move || {
        if let Some(handle) = timeout_handle.get_value() {
            if let Some(window) = web_sys::window() {
                window.clear_timeout_with_handle(handle);
            }
        }
        // Closure will be dropped when closure_handle is dropped or cleared
        closure_handle.set_value(None);
    });

    view! {
        <link rel="stylesheet" href="/style.css"/>
        <div class="sheet-page-container">
            <nav class="sheet-nav">
                <A href="/" class="back-link">"← Voltar para o Início"</A>
                {move || is_saving.get().then(|| view! { <span class="saving-indicator">"Salvando..."</span> })}
            </nav>

            <Suspense fallback=move || view! { <p>"Carregando Ficha..."</p> }>
                {move || sheet_resource.get().map(|res| match res {
                    Ok(_) => view! {
                        <Sheet>
                            <InfoHeader />
                            <Attributes />
                            <Abilities />
                            <Spheres />
                            <AdvantagesMta />
                        </Sheet>
                    }.into_view(),
                    Err(e) => view! { <p class="error">"Erro ao carregar: " {e.to_string()}</p> }.into_view(),
                })}
            </Suspense>
        </div>
    }
}
