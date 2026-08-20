use leptos::*;
use leptos_router::*;
use crate::state::{get_sheet, update_sheet, CharacterData, DotOrigin};
use crate::components::{Attributes, InfoHeader, Abilities, Spheres, AdvantagesMta, Sheet};

#[derive(Clone, Copy)]
pub struct ActiveDotOriginContext {
    pub origin: ReadSignal<DotOrigin>,
    #[allow(dead_code)]
    pub set_origin: WriteSignal<DotOrigin>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SaveStatus {
    Idle,
    Saved(String),
    Saving,
    Pending,
    Error(String),
}

fn get_current_time_str() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::new_0()
            .to_locale_time_string("pt-BR")
            .as_string()
            .unwrap_or_else(|| "agora".to_string())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "agora".to_string()
    }
}

#[component]
pub fn CharacterSheet() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    let sheet_resource = create_local_resource(id, |id| async move {
        if id.is_empty() {
            return Err(ServerFnError::new("ID da ficha não fornecido"));
        }
        get_sheet(id).await
    });

    let (data, set_data) = create_signal(CharacterData::default());
    let (save_status, set_save_status) = create_signal(SaveStatus::Idle);
    let (save_seq, set_save_seq) = create_signal(0u64);
    let (is_loaded, set_is_loaded) = create_signal(false);
    let (active_origin, set_active_origin) = create_signal(DotOrigin::Base);

    // Provide the sheet data and active dot origin as context for all child components
    provide_context(set_data);
    provide_context(data);
    provide_context(ActiveDotOriginContext {
        origin: active_origin,
        set_origin: set_active_origin,
    });

    create_effect(move |_| {
        if let Some(Ok(fetched_data)) = sheet_resource.get() {
            set_data.set(fetched_data);
            set_is_loaded.set(true);
            set_save_status.set(SaveStatus::Saved(get_current_time_str()));
        }
    });

    // Real Debounced Auto-Save
    create_effect(move |_| {
        data.track();
        
        if is_loaded.get_untracked() {
            set_save_seq.update(|s| *s += 1);
            let current_seq = save_seq.get_untracked();
            set_save_status.set(SaveStatus::Pending);

            let current_id = id();
            let current_data = data.get();

            spawn_local(async move {
                // Debounce timeout of 500ms
                gloo_timers::future::TimeoutFuture::new(500).await;

                // Check if this is still the most recent change
                if save_seq.get_untracked() == current_seq && !current_id.is_empty() {
                    set_save_status.set(SaveStatus::Saving);
                    match update_sheet(current_id, current_data).await {
                        Ok(_) => {
                            set_save_status.set(SaveStatus::Saved(get_current_time_str()));
                        }
                        Err(e) => {
                            log::error!("Auto-save error: {:?}", e);
                            set_save_status.set(SaveStatus::Error(e.to_string()));
                        }
                    }
                }
            });
        }
    });

    let do_manual_save = move |_| {
        let current_id = id();
        let current_data = data.get_untracked();
        if !current_id.is_empty() {
            set_save_status.set(SaveStatus::Saving);
            spawn_local(async move {
                match update_sheet(current_id, current_data).await {
                    Ok(_) => {
                        set_save_status.set(SaveStatus::Saved(get_current_time_str()));
                    }
                    Err(e) => {
                        log::error!("Manual save error: {:?}", e);
                        set_save_status.set(SaveStatus::Error(e.to_string()));
                    }
                }
            });
        }
    };

    let total_counts = move || data.get().get_total_bonus_and_xp_dots();

    view! {
        <link rel="stylesheet" href="/style.css"/>
        <div class="sheet-page-container">
            <header class="sheet-top-bar">
                <div class="top-bar-left">
                    <A href="/" class="back-link">"← Início"</A>
                </div>

                <div class="top-bar-center">
                    <div class="mode-selector-container">
                        <span class="mode-label">"Modo:"</span>
                        <div class="mode-btn-group">
                            <button 
                                class="mode-btn mode-base"
                                class:active=move || active_origin.get() == DotOrigin::Base
                                on:click=move |_| set_active_origin.set(DotOrigin::Base)
                                title="Criação Base de Personagem (Preto)"
                            >
                                <span class="mode-dot-icon dot-base"></span>
                                "Criação"
                            </button>
                            <button 
                                class="mode-btn mode-bonus"
                                class:active=move || active_origin.get() == DotOrigin::Bonus
                                on:click=move |_| set_active_origin.set(DotOrigin::Bonus)
                                title="Pontos de Bônus Iniciais (Roxo)"
                            >
                                <span class="mode-dot-icon dot-bonus"></span>
                                "Bônus (" {move || total_counts().0} "/15)"
                            </button>
                            <button 
                                class="mode-btn mode-xp"
                                class:active=move || active_origin.get() == DotOrigin::Experience
                                on:click=move |_| set_active_origin.set(DotOrigin::Experience)
                                title="Experiência / XP Acumulado (Verde)"
                            >
                                <span class="mode-dot-icon dot-xp"></span>
                                "XP (" {move || total_counts().1} ")"
                            </button>
                            <button 
                                class="mode-btn mode-temp"
                                class:active=move || active_origin.get() == DotOrigin::Temporary
                                on:click=move |_| set_active_origin.set(DotOrigin::Temporary)
                                title="Bônus Temporário / Feitiço / Wonder (Dourado)"
                            >
                                <span class="mode-dot-icon dot-temp"></span>
                                "Buff / Magia"
                            </button>
                        </div>
                    </div>
                </div>

                <div class="top-bar-right">
                    <div class="save-status-container">
                        {move || match save_status.get() {
                            SaveStatus::Idle => view! { <span class="status-badge status-idle"></span> }.into_view(),
                            SaveStatus::Pending => view! {
                                <span class="status-badge status-pending" title="Alterações pendentes...">
                                    <span class="status-dot dot-pending"></span>
                                    "Pendente"
                                </span>
                            }.into_view(),
                            SaveStatus::Saving => view! {
                                <span class="status-badge status-saving" title="Gravando dados no banco...">
                                    <span class="status-spinner"></span>
                                    "Salvando..."
                                </span>
                            }.into_view(),
                            SaveStatus::Saved(t) => view! {
                                <span class="status-badge status-saved" title="Todas as alterações foram salvas">
                                    <span class="status-dot dot-saved"></span>
                                    {format!("Salvo ({})", t)}
                                </span>
                            }.into_view(),
                            SaveStatus::Error(err) => {
                                let err_title = err.clone();
                                view! {
                                    <span class="status-badge status-error" title=err_title>
                                        <span class="status-dot dot-error"></span>
                                        "Erro ao salvar"
                                    </span>
                                 }.into_view()
                            },
                        }}
                        <button class="manual-save-btn" on:click=do_manual_save title="Salvar imediatamente">
                            "Salvar"
                        </button>
                    </div>
                </div>
            </header>

            <Suspense fallback=move || view! { <div class="loading-state"><p>"Carregando Ficha..."</p></div> }>
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
                    Err(e) => view! { 
                        <div class="error-container">
                            <p class="error-title">"Erro ao carregar a ficha"</p>
                            <p class="error-detail">{e.to_string()}</p>
                            <A href="/" class="back-home-btn">"Voltar para a lista de fichas"</A>
                        </div>
                    }.into_view(),
                })}
            </Suspense>
        </div>
    }
}
