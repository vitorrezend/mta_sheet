use leptos::*;
use crate::components::{Callback, StableTextArea, StableTextInput};
use crate::state::CharacterData;

#[component]
pub fn DescriptionSection() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    view! {
        <div class="group-box description-box">
            <span class="group-title">"DESCRIPTION"</span>

            <div class="description-grid-container">
                // Coluna Esquerda: Dados Pessoais e Demográficos
                <div class="description-demographics-col">
                    <div class="demographic-row">
                        <label class="demographic-label">"Age (Idade):"</label>
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
                        <label class="demographic-label">"Apparent Age (Idade Aparente):"</label>
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
                        <label class="demographic-label">"Date of Birth (Nascimento):"</label>
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
                        <label class="demographic-label">"Age of Awakening (Despertar):"</label>
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
                        <label class="demographic-label">"Hair (Cabelos):"</label>
                        <StableTextInput 
                            class="demographic-input"
                            placeholder="Ex: Castanhos"
                            value=Signal::derive(move || data.with(|d| d.description_data.hair.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.hair = val);
                            })
                        />
                    </div>

                    <div class="demographic-row">
                        <label class="demographic-label">"Eyes (Olhos):"</label>
                        <StableTextInput 
                            class="demographic-input"
                            placeholder="Ex: Âmbar"
                            value=Signal::derive(move || data.with(|d| d.description_data.eyes.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.eyes = val);
                            })
                        />
                    </div>

                    <div class="demographic-row">
                        <label class="demographic-label">"Race (Etnia):"</label>
                        <StableTextInput 
                            class="demographic-input"
                            placeholder="Ex: Latina"
                            value=Signal::derive(move || data.with(|d| d.description_data.race.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.race = val);
                            })
                        />
                    </div>

                    <div class="demographic-row">
                        <label class="demographic-label">"Nationality (Nacionalidade):"</label>
                        <StableTextInput 
                            class="demographic-input"
                            placeholder="Ex: Brasileiro"
                            value=Signal::derive(move || data.with(|d| d.description_data.nationality.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.nationality = val);
                            })
                        />
                    </div>

                    <div class="demographic-row">
                        <label class="demographic-label">"Height (Altura):"</label>
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
                        <label class="demographic-label">"Weight (Peso):"</label>
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
                        <label class="demographic-label">"Sex (Sexo):"</label>
                        <StableTextInput 
                            class="demographic-input"
                            placeholder="Ex: Masculino"
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
                        <label class="narrative-label">"PHYSICAL DESCRIPTION (Aparência Física & Estilo)"</label>
                        <StableTextArea 
                            class="narrative-textarea"
                            placeholder="Porte físico, estilo de vestimenta, cicatrizes, tatuagens místicas, maneirismos e tom de voz..."
                            value=Signal::derive(move || data.with(|d| d.description_data.physical_description.clone()))
                            on_change=Callback::new(move |val| {
                                set_data.update(|s| s.description_data.physical_description = val);
                            })
                        />
                    </div>

                    <div class="narrative-item">
                        <label class="narrative-label">"APPEARANCE / NATURE OF AVATAR (Aparência e Natureza do Avatar)"</label>
                        <StableTextArea 
                            class="narrative-textarea narrative-avatar-textarea"
                            placeholder="Essência do Avatar (Dinâmico, Estático, Primordial, Infinito), forma espiritual visível em meditação, voz e manifestações durante a magia..."
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
