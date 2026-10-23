use leptos::*;
use crate::i18n::{tr, LanguageContext, Language};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Notificador flutuante discreto para atualizações a quente da aplicação.
///
/// Monitora periodicamente o endpoint `/api/version` sem interromper formulários
/// ou ações em andamento. Permite ao usuário aplicar a atualização com um clique
/// ou dispensar a notificação durante a sessão ativa.
#[component]
pub fn VersionUpdateToast() -> impl IntoView {
    let (update_version, set_update_version) = create_signal(None::<String>);
    let (dismissed, set_dismissed) = create_signal(false);
    let _ = set_update_version;

    let lang_ctx = use_context::<LanguageContext>();
    let current_lang = move || lang_ctx.as_ref().map(|c| c.lang.get()).unwrap_or(Language::PtBr);

    #[cfg(target_arch = "wasm32")]
    {
        create_effect(move |_| {
            // 1. Checagem inicial com atraso para não competir com a hidratação e recursos iniciais
            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(4_000).await;
                check_server_version(set_update_version).await;

                // 2. Loop periódico a cada 3 minutos (180 segundos)
                loop {
                    gloo_timers::future::TimeoutFuture::new(180_000).await;
                    check_server_version(set_update_version).await;
                }
            });

            // 3. Checagem ao retornar o foco à aba (visibilitychange)
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    let doc_for_event = document.clone();
                    let closure = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
                        if doc_for_event.visibility_state() == web_sys::VisibilityState::Visible {
                            spawn_local(async move {
                                check_server_version(set_update_version).await;
                            });
                        }
                    });

                    let _ = document.add_event_listener_with_callback(
                        "visibilitychange",
                        closure.as_ref().unchecked_ref(),
                    );
                    closure.forget();
                }
            }
        });
    }

    let on_reload = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                let _ = window.location().reload();
            }
        }
    };

    let on_dismiss = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(ref ver) = update_version.get_untracked() {
                if let Some(storage) = web_sys::window().and_then(|w| w.session_storage().ok().flatten()) {
                    let _ = storage.set_item(&format!("mta_dismissed_update_{}", ver), "true");
                }
            }
        }
        set_dismissed.set(true);
    };

    view! {
        {move || {
            if let Some(server_ver) = update_version.get() {
                if !dismissed.get() {
                    let lang = current_lang();
                    let title = tr("update_available_title", lang);
                    let desc = tr("update_available_desc", lang);
                    let update_btn = tr("update_now", lang);
                    let later_btn = tr("update_later", lang);

                    view! {
                        <div class="mta-version-update-toast" role="alert" aria-live="polite">
                            <div class="mta-version-update-icon">"🔮"</div>
                            <div class="mta-version-update-content">
                                <div class="mta-version-update-header">
                                    <strong class="mta-version-update-title">{title}</strong>
                                    <span class="mta-version-badge">"v"{server_ver.clone()}</span>
                                </div>
                                <p class="mta-version-update-desc">{desc}</p>
                                <div class="mta-version-update-actions">
                                    <button
                                        type="button"
                                        class="mta-version-btn-update"
                                        on:click=on_reload
                                        title=update_btn
                                    >
                                        "🔄 " {update_btn}
                                    </button>
                                    <button
                                        type="button"
                                        class="mta-version-btn-dismiss"
                                        on:click=on_dismiss
                                        title=later_btn
                                    >
                                        {later_btn}
                                    </button>
                                </div>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! { <></> }.into_view()
                }
            } else {
                view! { <></> }.into_view()
            }
        }}
    }
}

#[cfg(target_arch = "wasm32")]
async fn check_server_version(set_update_version: WriteSignal<Option<String>>) {
    if let Ok(resp) = gloo_net::http::Request::get("/api/version").send().await {
        if resp.ok() {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(server_ver) = json.get("version").and_then(|v| v.as_str()) {
                    if server_ver != CLIENT_VERSION {
                        // Verifica se foi dispensado nesta sessão
                        let is_dismissed = web_sys::window()
                            .and_then(|w| w.session_storage().ok().flatten())
                            .and_then(|storage| storage.get_item(&format!("mta_dismissed_update_{}", server_ver)).ok().flatten())
                            .is_some();

                        if !is_dismissed {
                            set_update_version.set(Some(server_ver.to_string()));
                        }
                    }
                }
            }
        }
    }
}
