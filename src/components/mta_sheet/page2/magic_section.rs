use leptos::*;
use crate::components::Callback;
use crate::state::{CharacterData, WonderItem};
use super::wonder_card::WonderCard;
use super::image_modal::ImageModal;

#[component]
pub fn MagicSection() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    // Signal para abrir modal de visualização/zoom de imagem (Lightbox)
    let (modal_image_url, set_modal_image_url) = create_signal(Option::<String>::None);

    let add_wonder = move |_| {
        set_data.update(|s| {
            s.wonders.push(WonderItem::default());
        });
    };

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    view! {
        <div class="group-box magic-section-box">
            <span class="group-title">{move || crate::i18n::tr("wonders_title", lang())}</span>

            // Grade 2x2 preenchendo toda a largura da folha
            <div class="wonders-grid-2x2">
                {move || {
                    let count = data.with(|d| d.wonders.len());
                    (0..count).map(|idx| {
                        view! {
                            <WonderCard 
                                idx=idx 
                                on_image_click=Callback::new(move |url| set_modal_image_url.set(Some(url))) 
                            />
                        }
                    }).collect_view()
                }}
            </div>

            <div class="wonders-footer-actions">
                <button class="add-field-btn" on:click=add_wonder title=move || crate::i18n::tr("add_wonder", lang())>"+"</button>
            </div>

            // Modal Lightbox para visualização e zoom da imagem
            <ImageModal 
                image_url=modal_image_url.into() 
                on_close=Callback::new(move |_| set_modal_image_url.set(None)) 
            />
        </div>
    }
}
