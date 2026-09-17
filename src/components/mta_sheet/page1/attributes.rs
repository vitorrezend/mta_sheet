use leptos::*;
use crate::components::BoundAttributeField;

#[component]
pub fn Attributes() -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    view! {
        <div class="group-box">
            <span class="group-title">{move || crate::i18n::tr("attributes", lang())}</span>
            <div class="attributes-block">
                <AttributeColumn title=Signal::derive(move || crate::i18n::tr("physical", lang()).to_string())>
                    <BoundAttributeField name="Força" />
                    <BoundAttributeField name="Destreza" />
                    <BoundAttributeField name="Vigor" />
                </AttributeColumn>
                
                <AttributeColumn title=Signal::derive(move || crate::i18n::tr("social", lang()).to_string())>
                    <BoundAttributeField name="Carisma" />
                    <BoundAttributeField name="Manipulação" />
                    <BoundAttributeField name="Aparência" />
                </AttributeColumn>

                <AttributeColumn title=Signal::derive(move || crate::i18n::tr("mental", lang()).to_string())>
                    <BoundAttributeField name="Percepção" />
                    <BoundAttributeField name="Inteligência" />
                    <BoundAttributeField name="Raciocínio" />
                </AttributeColumn>
            </div>
        </div>
    }
}

#[component]
fn AttributeColumn(title: Signal<String>, children: Children) -> impl IntoView {
    view! {
        <div class="attribute-column">
            <h3 class="column-title">{move || title.get()}</h3>
            {children()}
        </div>
    }
}
