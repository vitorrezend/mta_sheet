use leptos::*;
use crate::components::{Callback, StableTextArea};
use crate::state::CharacterData;

#[component]
pub fn HistorySection() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    view! {
        <div class="group-box history-box">
            <span class="group-title">{move || crate::i18n::tr("history_title", lang())}</span>

            <div class="history-item">
                <label class="history-label">{move || crate::i18n::tr("char_history_label", lang())}</label>
                <StableTextArea 
                    class="history-textarea history-main-textarea"
                    placeholder=Signal::derive(move || match lang() {
                        crate::i18n::Language::PtBr => "Origens, infância, momento do Despertar, mentor, tradição e eventos marcantes...".to_string(),
                        crate::i18n::Language::EnUs => "Origins, childhood, Awakening moment, mentor, tradition, and defining life events...".to_string(),
                    })
                    value=Signal::derive(move || data.with(|d| d.history_data.history.clone()))
                    on_change=Callback::new(move |val| {
                        set_data.update(|s| s.history_data.history = val);
                    })
                />
            </div>

            <div class="history-item">
                <label class="history-label">{move || crate::i18n::tr("goals_destiny_label", lang())}</label>
                <StableTextArea 
                    class="history-textarea"
                    placeholder=Signal::derive(move || match lang() {
                        crate::i18n::Language::PtBr => "Metas pessoais, ambições místicas na Ascensão, profecias de destino ou dívidas cármicas...".to_string(),
                        crate::i18n::Language::EnUs => "Personal goals, mystical ambitions towards Ascension, prophecies, karmic debts...".to_string(),
                    })
                    value=Signal::derive(move || data.with(|d| d.history_data.goals_destiny.clone()))
                    on_change=Callback::new(move |val| {
                        set_data.update(|s| s.history_data.goals_destiny = val);
                    })
                />
            </div>

            <div class="history-grid-2col">
                <div class="history-item">
                    <label class="history-label">{move || crate::i18n::tr("seekings_label", lang())}</label>
                    <StableTextArea 
                        class="history-textarea history-seeking-textarea"
                        placeholder=Signal::derive(move || match lang() {
                            crate::i18n::Language::PtBr => "Jornadas de iluminação, enigmas do Avatar, ritos de passagem e epifanias...".to_string(),
                            crate::i18n::Language::EnUs => "Enlightenment journeys, Avatar riddles, rites of passage, and epiphanies...".to_string(),
                        })
                        value=Signal::derive(move || data.with(|d| d.history_data.seekings.clone()))
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| s.history_data.seekings = val);
                        })
                    />
                </div>

                <div class="history-item">
                    <label class="history-label">{move || crate::i18n::tr("quiets_label", lang())}</label>
                    <StableTextArea 
                        class="history-textarea history-seeking-textarea"
                        placeholder=Signal::derive(move || match lang() {
                            crate::i18n::Language::PtBr => "Episódios de desconexão, distorções de realidade, perda de controle do Paradoxo...".to_string(),
                            crate::i18n::Language::EnUs => "Episodes of detachment, reality warping, losing control to Paradox...".to_string(),
                        })
                        value=Signal::derive(move || data.with(|d| d.history_data.quiets.clone()))
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| s.history_data.quiets = val);
                        })
                    />
                </div>
            </div>
        </div>
    }
}
