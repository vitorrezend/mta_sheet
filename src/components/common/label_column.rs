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
            if key == "Nome" || key == crate::state::keys::HEADER_NOME {
                s.set_display_name(&value);
            } else {
                s.labels.insert(key, value);
            }
        });
    };

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    view! {
        <div class="info-column">
            {fields.into_iter().map(|(label, key)| {
                let key_str = key.to_string();
                let key_str2 = key.to_string();
                let is_name_field = key == "Nome" || key == crate::state::keys::HEADER_NOME;
                let value = Signal::derive({
                    let key = key_str.clone();
                    move || {
                        data.with(|d| {
                            if is_name_field {
                                d.get_display_name()
                            } else {
                                d.labels.get(&key).cloned().unwrap_or_default()
                            }
                        })
                    }
                });
                let translated_label = Signal::derive(move || crate::i18n::tr_header_label(label, lang()).to_string());
                view! {
                    <LabelField 
                        label=translated_label 
                        value=value
                        on_change=move |v| update_label(key_str2.clone(), v)
                    />
                }
            }).collect_view()}
        </div>
    }
}
