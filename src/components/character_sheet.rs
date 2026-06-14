use leptos::*;
use leptos_router::*;
use wasm_bindgen::JsCast;
use crate::state::{get_sheet, update_sheet, CharacterData};
use crate::components::{Attributes, InfoHeader, Abilities, Spheres, AdvantagesMta, Sheet};

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

    create_effect(move |_| {
        if let Some(Ok(fetched_data)) = sheet_resource.get() {
            set_data.set(fetched_data);
        }
    });

    // Provide the sheet data as context for all child components
    provide_context(set_data);
    provide_context(data);

    // Debouncing auto-save
    let (is_saving, set_is_saving) = create_signal(false);
    let active_closure = store_value(None::<wasm_bindgen::closure::Closure<dyn FnMut()>>);

    create_effect(move |prev_timer: Option<i32>| {
        data.track();
        let current_data = data.get();
        let current_id = id();

        // Clear previous timer
        if let Some(timer_id) = prev_timer {
            if timer_id != -1 {
                let _ = web_sys::window().unwrap().clear_timeout_with_handle(timer_id);
            }
        }

        if !current_id.is_empty() && !current_data.name.is_empty() {
            let window = web_sys::window().unwrap();

            // Create new closure
            let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                set_is_saving.set(true);
                let id = current_id.clone();
                let data = current_data.clone();
                spawn_local(async move {
                    let _ = update_sheet(id, data).await;
                    set_is_saving.set(false);
                });
            }) as Box<dyn FnMut()>);

            let handle = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                500 // 500ms debounce
            ).ok();

            // Store closure to keep it alive and drop the old one
            active_closure.set_value(Some(cb));

            return handle.unwrap_or(-1);
        }
        -1
    });

    on_cleanup(move || {
        active_closure.set_value(None);
    });

    view! {
        <link rel="stylesheet" href="/style.css"/>
        <div class="sheet-page-container">
            <nav class="sheet-nav">
                <A href="/" class="back-link">"← Voltar para o Início"</A>
                <div class="saving-indicator" class:visible=is_saving>
                    "Salvando..."
                </div>
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
