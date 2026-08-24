use leptos::*;
use crate::components::{Callback, StableTextArea};
use crate::state::CharacterData;

#[component]
pub fn HistorySection() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    view! {
        <div class="group-box history-box">
            <span class="group-title">"HISTORY"</span>

            <div class="history-item">
                <label class="history-label">"HISTORY (Histórico do Personagem)"</label>
                <StableTextArea 
                    class="history-textarea history-main-textarea"
                    placeholder="Origens, infância, momento do Despertar, mentor, tradição e eventos marcantes..."
                    value=Signal::derive(move || data.with(|d| d.history_data.history.clone()))
                    on_change=Callback::new(move |val| {
                        set_data.update(|s| s.history_data.history = val);
                    })
                />
            </div>

            <div class="history-item">
                <label class="history-label">"GOALS / DESTINY (Objetivos & Destino)"</label>
                <StableTextArea 
                    class="history-textarea"
                    placeholder="Metas pessoais, ambições místicas na Ascensão, profecias de destino ou dívidas cármicas..."
                    value=Signal::derive(move || data.with(|d| d.history_data.goals_destiny.clone()))
                    on_change=Callback::new(move |val| {
                        set_data.update(|s| s.history_data.goals_destiny = val);
                    })
                />
            </div>

            <div class="history-grid-2col">
                <div class="history-item">
                    <label class="history-label">"SEEKINGS (Buscas de Avatar)"</label>
                    <StableTextArea 
                        class="history-textarea history-seeking-textarea"
                        placeholder="Jornadas de iluminação, enigmas do Avatar, ritos de passagem e epifanias..."
                        value=Signal::derive(move || data.with(|d| d.history_data.seekings.clone()))
                        on_change=Callback::new(move |val| {
                            set_data.update(|s| s.history_data.seekings = val);
                        })
                    />
                </div>

                <div class="history-item">
                    <label class="history-label">"QUIETS (Silêncios / Loucura Mística)"</label>
                    <StableTextArea 
                        class="history-textarea history-seeking-textarea"
                        placeholder="Episódios de desconexão, distorções de realidade, perda de controle do Paradoxo..."
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
