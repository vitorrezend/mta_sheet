use leptos::*;
use crate::components::common::StableTextArea;
use crate::components::Callback;
use crate::state::{CharacterData, QuizQuestionEntry};

#[component]
pub fn QuizModal(
    show_quiz: ReadSignal<bool>,
    set_show_quiz: WriteSignal<bool>,
    #[prop(into)] data: Signal<CharacterData>,
    set_data: WriteSignal<CharacterData>,
) -> impl IntoView {
    // Estado local desacoplado da reatividade externa do sheet
    // Inicializa de forma síncrona com os dados da ficha para renderizar imediatamente no primeiro clique
    let initial_entries = data.with_untracked(|d| {
        let e = d.quiz_data.entries.clone();
        if e.is_empty() {
            crate::state::default_quiz_questions()
        } else {
            e
        }
    });
    let local_entries = create_rw_signal(initial_entries);

    // Função para salvar alterações e fechar
    let save_and_close = move || {
        let current_entries = local_entries.get_untracked();
        if !current_entries.is_empty() {
            set_data.update(|d| {
                d.quiz_data.entries = current_entries;
            });
        }
        set_show_quiz.set(false);
    };

    view! {
        {move || if show_quiz.get() {
            // Sincroniza e captura os dados de forma síncrona e untracked na abertura do modal
            let entries = data.with_untracked(|d| {
                let e = d.quiz_data.entries.clone();
                if e.is_empty() {
                    crate::state::default_quiz_questions()
                } else {
                    e
                }
            });
            local_entries.set_untracked(entries.clone());
            let entries_snapshot = entries;
            
            view! {
                <div class="modal-overlay" on:click=move |_| save_and_close()>
                    <div class="modal-card quiz-modal-card" on:click=move |ev| ev.stop_propagation()>
                        <div class="modal-header quiz-modal-header">
                            <div class="modal-title-group">
                                <h2 class="modal-title">"📂 Dossiê do Personagem — Questionário de Criação"</h2>
                                <span class="modal-subtitle">
                                    "Material Suplementar & Guia de Interpretação (Mago: A Ascensão)"
                                </span>
                            </div>
                            <button class="modal-close-btn" on:click=move |_| save_and_close() title="Salvar e Fechar">"✕"</button>
                        </div>

                        <div class="quiz-modal-body">
                            <div class="quiz-section-header">
                                <h3 class="quiz-section-title">"👤 Perguntas para o Personagem (Histórico & Identidade)"</h3>
                                <p class="quiz-section-desc">"Perguntas essenciais sobre quem era o personagem antes do Despertar, infância, mentor, cabala e vida comum."</p>
                            </div>

                            <div class="quiz-cards-list">
                                {entries_snapshot.iter().enumerate().filter(|(_, e)| e.category != "player").map(|(idx, entry)| {
                                    let answer_val = Signal::derive(move || {
                                        local_entries.with(|entries| {
                                            entries.get(idx).map(|e| e.answer.clone()).unwrap_or_default()
                                        })
                                    });

                                    let on_change = Callback::new(move |new_text: String| {
                                        local_entries.update(|entries| {
                                            if let Some(e) = entries.get_mut(idx) {
                                                e.answer = new_text.clone();
                                            }
                                        });
                                        // Atualiza o dado raiz de forma untracked para garantir persistência contínua
                                        set_data.update_untracked(|d| {
                                            if let Some(e) = d.quiz_data.entries.get_mut(idx) {
                                                e.answer = new_text;
                                            }
                                        });
                                    });

                                    view! {
                                        <div class="quiz-question-card">
                                            <div class="quiz-question-header">
                                                <h3 class="quiz-question-title">{entry.title.clone()}</h3>
                                            </div>
                                            <p class="quiz-question-prompt">{entry.prompt.clone()}</p>
                                            <div class="quiz-textarea-wrapper">
                                                <StableTextArea
                                                    value=answer_val
                                                    on_change=on_change
                                                    placeholder="Escreva sua resposta detalhada aqui..."
                                                    class="quiz-textarea"
                                                />
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>

                            <div class="quiz-path-banner">
                                <div class="quiz-section-header">
                                    <h3 class="quiz-section-title">"🧭 Dicas & Perguntas para os Jogadores sobre o Caminho"</h3>
                                </div>
                                <p class="quiz-path-text">
                                    "O Caminho de um mago significa muito mais do que a Tradição a qual ele pertence, se ele pertencer a uma. As quatro facções apenas indicam com quem o seu mago anda. Seu Caminho define quem ele é."
                                </p>
                                <p class="quiz-path-text">
                                    "O Caminho do seu mago oferece muitas sugestões sobre como o seu sósia místiko irá se comportar. Entretanto, lembre-se de que um Caminho não é um 'alinhamento' rígido — é uma predisposição. Você não tem que criá-la com antecedência — apenas imagine onde você eventualmente quer chegar e como. Pense no Caminho como um mapa, com um destino ideal, alguns marcos e um plano de viagem. A jornada verdadeira — o jogo — irá começar significativamente a partir daquele mapa. Todavia, ter alguma noção de onde você está indo deixará o caminho mais claro e as partidas mais fáceis de serem administradas."
                                </p>
                            </div>

                            <div class="quiz-cards-list">
                                {entries_snapshot.iter().enumerate().filter(|(_, e)| e.category == "player").map(|(idx, entry)| {
                                    let answer_val = Signal::derive(move || {
                                        local_entries.with(|entries| {
                                            entries.get(idx).map(|e| e.answer.clone()).unwrap_or_default()
                                        })
                                    });

                                    let on_change = Callback::new(move |new_text: String| {
                                        local_entries.update(|entries| {
                                            if let Some(e) = entries.get_mut(idx) {
                                                e.answer = new_text.clone();
                                            }
                                        });
                                        // Atualiza o dado raiz de forma untracked para garantir persistência contínua
                                        set_data.update_untracked(|d| {
                                            if let Some(e) = d.quiz_data.entries.get_mut(idx) {
                                                e.answer = new_text;
                                            }
                                        });
                                    });

                                    view! {
                                        <div class="quiz-question-card">
                                            <div class="quiz-question-header">
                                                <h3 class="quiz-question-title">{entry.title.clone()}</h3>
                                            </div>
                                            <p class="quiz-question-prompt">{entry.prompt.clone()}</p>
                                            <div class="quiz-textarea-wrapper">
                                                <StableTextArea
                                                    value=answer_val
                                                    on_change=on_change
                                                    placeholder="Escreva sua resposta detalhada aqui..."
                                                    class="quiz-textarea"
                                                />
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </div>

                        <div class="modal-footer quiz-modal-footer">
                            <button 
                                type="button" 
                                class="btn-close-dossier"
                                on:click=move |_| save_and_close()
                            >
                                "💾 Concluir / Salvar Dossiê"
                            </button>
                        </div>
                    </div>
                </div>
            }.into_view()
        } else {
            view! { <div></div> }.into_view()
        }}
    }
}
