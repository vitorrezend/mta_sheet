use leptos::*;
use crate::components::Callback;

#[component]
pub fn ImageModal(
    image_url: Signal<Option<String>>,
    on_close: Callback<()>,
) -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let current_lang = Signal::derive(move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default());

    // ESC key closes the modal
    #[cfg(target_arch = "wasm32")]
    {
        let on_close_esc = on_close.clone();
        let image_url_check = image_url;
        let handle = window_event_listener(ev::keydown, move |ev: web_sys::KeyboardEvent| {
            if image_url_check.try_get_untracked().flatten().is_some() && ev.key() == "Escape" {
                on_close_esc.call(());
            }
        });
        on_cleanup(move || handle.remove());
    }

    view! {
        {
            let on_close = on_close.clone();
            move || {
                if let Some(url) = image_url.get() {
                    let u = url.clone();
                    let on_close_backdrop = on_close.clone();
                    let on_close_btn = on_close.clone();
                    let lang = current_lang.get();
                    let on_close_backdrop_outer = on_close_backdrop.clone();
                    let on_close_btn_outer = on_close_btn.clone();
                    let u_outer = u.clone();
                    view! {
                        <Portal>
                            {
                                let on_close_backdrop = on_close_backdrop_outer.clone();
                                let on_close_btn = on_close_btn_outer.clone();
                                let u = u_outer.clone();
                                view! {
                                    <div class="image-lightbox-backdrop" on:click=move |_| on_close_backdrop.call(())>
                                        <div class="image-lightbox-content" on:click=move |ev| ev.stop_propagation()>
                                            <div class="image-lightbox-header">
                                                <span class="image-lightbox-title">
                                                    "🖼️ "
                                                    {match lang {
                                                        crate::i18n::Language::PtBr => "Visualização da Imagem",
                                                        crate::i18n::Language::EnUs => "Image Preview",
                                                    }}
                                                </span>
                                                <button 
                                                    type="button" 
                                                    class="image-lightbox-close-btn"
                                                    on:click=move |_| on_close_btn.call(())
                                                    title=match lang {
                                                        crate::i18n::Language::PtBr => "Fechar (Esc)",
                                                        crate::i18n::Language::EnUs => "Close (Esc)",
                                                    }
                                                >
                                                    "✕"
                                                </button>
                                            </div>
                                            <div class="image-lightbox-body">
                                                <img src=u alt="Imagem ampliada" class="image-lightbox-img" />
                                            </div>
                                            <div class="image-lightbox-hint">
                                                {match lang {
                                                    crate::i18n::Language::PtBr => "Toque fora ou no ✕ para fechar",
                                                    crate::i18n::Language::EnUs => "Tap outside or ✕ to close",
                                                }}
                                            </div>
                                        </div>
                                    </div>
                                }
                            }
                        </Portal>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }
        }
    }
}

