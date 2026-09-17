use leptos::*;
use crate::components::BoundAttributeField;

#[component]
pub fn GodsAndMonstersAttributes() -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    view! {
        <div class="group-box gods-box">
            <span class="group-title">{move || crate::i18n::tr("attributes", lang())}</span>
            <div class="attributes-block">
                <div class="attribute-column">
                    <h3 class="column-title">{move || crate::i18n::tr("physical", lang())}</h3>
                    <BoundAttributeField name="Strength" enable_compendium=false />
                    <BoundAttributeField name="Dexterity" enable_compendium=false />
                    <BoundAttributeField name="Stamina" enable_compendium=false />
                </div>
                
                <div class="attribute-column">
                    <h3 class="column-title">{move || crate::i18n::tr("social", lang())}</h3>
                    <BoundAttributeField name="Charisma" enable_compendium=false />
                    <BoundAttributeField name="Manipulation" enable_compendium=false />
                    <BoundAttributeField name="Appearance" enable_compendium=false />
                </div>

                <div class="attribute-column">
                    <h3 class="column-title">{move || crate::i18n::tr("mental", lang())}</h3>
                    <BoundAttributeField name="Perception" enable_compendium=false />
                    <BoundAttributeField name="Intelligence" enable_compendium=false />
                    <BoundAttributeField name="Wits" enable_compendium=false />
                </div>
            </div>
        </div>
    }
}
