use leptos::*;
use crate::components::Callback;

#[component]
pub fn ImageModal(
    image_url: Signal<Option<String>>,
    on_close: Callback<()>,
) -> impl IntoView {
    view! {
        {
            let on_close = on_close.clone();
            move || {
                if let Some(url) = image_url.get() {
                    let u = url.clone();
                    let on_close_backdrop = on_close.clone();
                    let on_close_btn = on_close.clone();
                    view! {
                        <div class="image-lightbox-backdrop" on:click=move |_| on_close_backdrop.call(())>
                            <div class="image-lightbox-content" on:click=move |ev| ev.stop_propagation()>
                                <div class="image-lightbox-header">
                                    <span class="image-lightbox-title">"Visualização da Imagem"</span>
                                    <button 
                                        type="button" 
                                        class="image-lightbox-close-btn"
                                        on:click=move |_| on_close_btn.call(())
                                        title="Fechar (Esc)"
                                    >
                                        "✕"
                                    </button>
                                </div>
                                <div class="image-lightbox-body">
                                    <img src=u alt="Imagem ampliada" class="image-lightbox-img" />
                                </div>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }
        }
    }
}
