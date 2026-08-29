use leptos::*;
use crate::components::{Callback, StableTextArea, StableTextInput};
use crate::state::CharacterData;

#[component]
pub fn DescriptionSection() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    view! {
        <div class="group-box description-box">
            <span class="group-title">{move || crate::i18n::tr("description_title", lang())}</span>

            <div class="description-grid-container">
                // Coluna Esquerda: Dados Pessoais e Demográficos
                <div class="description-demographics-col">
                    <div class="demographic-row">
                        <label class="demographic-label">{move || crate::i18n::tr("age", lang())}</label>
                        <StableTextInput 
                            class="demographic-input"
                            placeholder="Ex: 28"
                            value=Signal::derive(move || data.with(|d| d.description_data.age.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.age = val);
                            })
                        />
                    </div>

                    <div class="demographic-row">
                        <label class="demographic-label">{move || crate::i18n::tr("apparent_age", lang())}</label>
                        <StableTextInput 
                            class="demographic-input"
                            placeholder="Ex: 22"
                            value=Signal::derive(move || data.with(|d| d.description_data.apparent_age.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.apparent_age = val);
                            })
                        />
                    </div>

                    <div class="demographic-row">
                        <label class="demographic-label">{move || crate::i18n::tr("date_of_birth", lang())}</label>
                        <StableTextInput 
                            class="demographic-input"
                            placeholder="Ex: 14/05/1998"
                            value=Signal::derive(move || data.with(|d| d.description_data.date_of_birth.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.date_of_birth = val);
                            })
                        />
                    </div>

                    <div class="demographic-row">
                        <label class="demographic-label">{move || crate::i18n::tr("age_of_awakening", lang())}</label>
                        <StableTextInput 
                            class="demographic-input"
                            placeholder="Ex: 19"
                            value=Signal::derive(move || data.with(|d| d.description_data.age_of_awakening.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.age_of_awakening = val);
                            })
                        />
                    </div>

                    <div class="demographic-row">
                        <label class="demographic-label">{move || crate::i18n::tr("hair", lang())}</label>
                        <StableTextInput 
                            class="demographic-input"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Ex: Castanhos".to_string(),
                                crate::i18n::Language::EnUs => "Ex: Brown".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.description_data.hair.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.hair = val);
                            })
                        />
                    </div>

                    <div class="demographic-row">
                        <label class="demographic-label">{move || crate::i18n::tr("eyes", lang())}</label>
                        <StableTextInput 
                            class="demographic-input"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Ex: Âmbar".to_string(),
                                crate::i18n::Language::EnUs => "Ex: Amber".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.description_data.eyes.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.eyes = val);
                            })
                        />
                    </div>

                    <div class="demographic-row">
                        <label class="demographic-label">{move || crate::i18n::tr("race_ethnicity", lang())}</label>
                        <StableTextInput 
                            class="demographic-input"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Ex: Latina".to_string(),
                                crate::i18n::Language::EnUs => "Ex: Latino".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.description_data.race.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.race = val);
                            })
                        />
                    </div>

                    <div class="demographic-row">
                        <label class="demographic-label">{move || crate::i18n::tr("nationality", lang())}</label>
                        <StableTextInput 
                            class="demographic-input"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Ex: Brasileiro".to_string(),
                                crate::i18n::Language::EnUs => "Ex: Brazilian".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.description_data.nationality.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.nationality = val);
                            })
                        />
                    </div>

                    <div class="demographic-row">
                        <label class="demographic-label">{move || crate::i18n::tr("height", lang())}</label>
                        <StableTextInput 
                            class="demographic-input"
                            placeholder="Ex: 1,82m"
                            value=Signal::derive(move || data.with(|d| d.description_data.height.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.height = val);
                            })
                        />
                    </div>

                    <div class="demographic-row">
                        <label class="demographic-label">{move || crate::i18n::tr("weight", lang())}</label>
                        <StableTextInput 
                            class="demographic-input"
                            placeholder="Ex: 78kg"
                            value=Signal::derive(move || data.with(|d| d.description_data.weight.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.weight = val);
                            })
                        />
                    </div>

                    <div class="demographic-row">
                        <label class="demographic-label">{move || crate::i18n::tr("gender", lang())}</label>
                        <StableTextInput 
                            class="demographic-input"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Ex: Masculino".to_string(),
                                crate::i18n::Language::EnUs => "Ex: Male".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.description_data.sex.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.sex = val);
                            })
                        />
                    </div>
                </div>

                // Coluna Direita: Descrição Física e Natureza do Avatar
                <div class="description-narrative-col">
                    <div class="narrative-item">
                        <label class="narrative-label">{move || match lang() {
                            crate::i18n::Language::PtBr => "DESCRIÇÃO FÍSICA (Aparência & Estilo)",
                            crate::i18n::Language::EnUs => "PHYSICAL DESCRIPTION (Appearance & Style)",
                        }}</label>
                        <StableTextArea 
                            class="narrative-textarea"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Porte físico, estilo de vestimenta, cicatrizes, tatuagens místicas, maneirismos e tom de voz...".to_string(),
                                crate::i18n::Language::EnUs => "Physical build, clothing style, scars, mystic tattoos, mannerisms, voice tone...".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.description_data.physical_description.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.physical_description = val);
                            })
                        />
                    </div>

                    <div class="narrative-item">
                        <label class="narrative-label">{move || match lang() {
                            crate::i18n::Language::PtBr => "APARÊNCIA E NATUREZA DO AVATAR",
                            crate::i18n::Language::EnUs => "APPEARANCE & NATURE OF AVATAR",
                        }}</label>
                        <StableTextArea 
                            class="narrative-textarea narrative-avatar-textarea"
                            placeholder=Signal::derive(move || match lang() {
                                crate::i18n::Language::PtBr => "Essência do Avatar (Dinâmico, Estático, Primordial, Infinito), forma espiritual visível em meditação, voz e manifestações durante a magia...".to_string(),
                                crate::i18n::Language::EnUs => "Avatar essence (Dynamic, Static, Primordial, Infinite), astral form seen in meditation, voice, manifestations during casting...".to_string(),
                            })
                            value=Signal::derive(move || data.with(|d| d.description_data.avatar_nature.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.avatar_nature = val);
                            })
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}
