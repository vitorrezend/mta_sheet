use leptos::*;
use crate::components::LabelField;
use crate::state::CharacterData;

#[component]
pub fn GodsAndMonstersHeader() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let update_name = move |new_val: String| {
        set_data.update(|d| d.set_display_name(&new_val));
    };

    let update_label = move |key: &'static str, val: String| {
        set_data.update(|d| {
            d.set_label(key, val);
        });
    };

    view! {
        <div class="gods-header-container">
            <div class="gods-title-banner">
                <h1 class="gods-main-title">{move || crate::i18n::tr("gods_monsters_title", lang())}</h1>
            </div>

            <div class="header-grid gods-header-grid">
                // Column 1
                <div class="header-column">
                    <LabelField 
                        label=Signal::derive(move || format!("{}:", crate::i18n::tr_header_label("Nome", lang()))) 
                        value=Signal::derive(move || data.with(|d| d.get_display_name()))
                        on_change=update_name
                    />
                    <LabelField 
                        label=Signal::derive(move || format!("{}:", crate::i18n::tr_header_label("Cronica", lang()))) 
                        value=Signal::derive(move || data.with(|d| d.get_label("Chronicle")))
                        on_change=move |v| update_label("Chronicle", v)
                    />
                </div>

                // Column 2
                <div class="header-column">
                    <LabelField 
                        label=Signal::derive(move || format!("{}:", crate::i18n::tr_header_label("Natureza", lang()))) 
                        value=Signal::derive(move || data.with(|d| d.get_label("Nature")))
                        on_change=move |v| update_label("Nature", v)
                    />
                    <LabelField 
                        label=Signal::derive(move || format!("{}:", crate::i18n::tr_header_label("Comportamento", lang()))) 
                        value=Signal::derive(move || data.with(|d| d.get_label("Demeanor")))
                        on_change=move |v| update_label("Demeanor", v)
                    />
                </div>

                // Column 3
                <div class="header-column">
                    <LabelField 
                        label=Signal::derive(move || format!("{}:", crate::i18n::tr_header_label("Tipo", lang()))) 
                        value=Signal::derive(move || data.with(|d| d.get_label("Type")))
                        on_change=move |v| update_label("Type", v)
                    />
                    <LabelField 
                        label=Signal::derive(move || format!("{}:", crate::i18n::tr_header_label("Conceito", lang()))) 
                        value=Signal::derive(move || data.with(|d| d.get_label("Concept")))
                        on_change=move |v| update_label("Concept", v)
                    />
                </div>
            </div>
        </div>
    }
}
