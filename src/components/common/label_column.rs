use leptos::*;
use super::LabelField;
use crate::state::CharacterData;

#[component]
pub fn LabelColumn(
    fields: Vec<(&'static str, &'static str)>,
) -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");

    let update_label = move |key: String, value: String| {
        set_data.update(|s| {
            s.set_label(&key, value);
        });
    };

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();
    let compendium_ctx = use_context::<crate::components::mta_sheet::page5::PracticeCompendiumContext>();

    view! {
        <div class="info-column">
            {fields.into_iter().map(|(label, key)| {
                let key_str = key.to_string();
                let key_str2 = key.to_string();
                let value = Signal::derive({
                    let key = key_str.clone();
                    move || {
                        data.with(|d| d.get_label(&key))
                    }
                });
                let translated_label = Signal::derive(move || crate::i18n::tr_header_label(label, lang()).to_string());

                let (on_lookup, lookup_title): (Option<crate::components::Callback<()>>, Option<Signal<String>>) = if key == "Natureza" {
                    let ctx = compendium_ctx.clone();
                    let val_sig = value;
                    let cb = crate::components::Callback::new(move |()| {
                        if let Some(ref c) = ctx {
                            c.open_archetype.call((Some(crate::components::mta_sheet::page5::ArchetypeTarget::Nature), val_sig.get()));
                        }
                    });
                    let title = Signal::derive(move || match lang() {
                        crate::i18n::Language::PtBr => "Consultar Natureza no Compêndio M20".to_string(),
                        crate::i18n::Language::EnUs => "Look up Nature in M20 Compendium".to_string(),
                    });
                    (Some(cb), Some(title))
                } else if key == "Comportamento" {
                    let ctx = compendium_ctx.clone();
                    let val_sig = value;
                    let cb = crate::components::Callback::new(move |()| {
                        if let Some(ref c) = ctx {
                            c.open_archetype.call((Some(crate::components::mta_sheet::page5::ArchetypeTarget::Demeanor), val_sig.get()));
                        }
                    });
                    let title = Signal::derive(move || match lang() {
                        crate::i18n::Language::PtBr => "Consultar Comportamento no Compêndio M20".to_string(),
                        crate::i18n::Language::EnUs => "Look up Demeanor in M20 Compendium".to_string(),
                    });
                    (Some(cb), Some(title))
                } else {
                    (None, None)
                };

                match (on_lookup, lookup_title) {
                    (Some(cb), Some(title)) => view! {
                        <LabelField 
                            label=translated_label 
                            value=value
                            on_change=move |v| update_label(key_str2.clone(), v)
                            on_lookup=cb
                            lookup_title=title
                        />
                    }.into_view(),
                    _ => view! {
                        <LabelField 
                            label=translated_label 
                            value=value
                            on_change=move |v| update_label(key_str2.clone(), v)
                        />
                    }.into_view(),
                }
            }).collect_view()}
        </div>
    }
}
