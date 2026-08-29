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

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

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
            let current_lang = lang();
            
            view! {
                <div class="modal-overlay" on:click=move |_| save_and_close()>
                    <div class="modal-card quiz-modal-card" on:click=move |ev| ev.stop_propagation()>
                        <div class="modal-header quiz-modal-header">
                            <div class="modal-title-group">
                                <h2 class="modal-title">{crate::i18n::tr("dossier_modal_title", current_lang)}</h2>
                                <span class="modal-subtitle">
                                    {crate::i18n::tr("dossier_modal_sub", current_lang)}
                                </span>
                            </div>
                            <button 
                                class="modal-close-btn" 
                                on:click=move |_| save_and_close() 
                                title=match current_lang {
                                    crate::i18n::Language::PtBr => "Salvar e Fechar",
                                    crate::i18n::Language::EnUs => "Save & Close",
                                }
                            >
                                "✕"
                            </button>
                        </div>

                        <div class="quiz-modal-body">
                            <div class="quiz-section-header">
                                <h3 class="quiz-section-title">{crate::i18n::tr("quiz_char_section", current_lang)}</h3>
                                <p class="quiz-section-desc">{crate::i18n::tr("quiz_char_desc", current_lang)}</p>
                            </div>

                            <div class="quiz-cards-list">
                                {entries_snapshot.clone().into_iter().enumerate().filter(|(_, e)| e.category != "player").map(|(idx, entry)| {
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

                                    let q_title = crate::i18n::tr_quiz_title(&entry.id, current_lang).to_string();
                                    let q_prompt = crate::i18n::tr_quiz_prompt(&entry.id, current_lang).to_string();
                                    let placeholder_txt = match current_lang {
                                        crate::i18n::Language::PtBr => "Escreva sua resposta detalhada aqui...",
                                        crate::i18n::Language::EnUs => "Write your detailed answer here...",
                                    };

                                    view! {
                                        <div class="quiz-question-card">
                                            <div class="quiz-question-header">
                                                <h3 class="quiz-question-title">{q_title}</h3>
                                            </div>
                                            <p class="quiz-question-prompt">{q_prompt}</p>
                                            <div class="quiz-textarea-wrapper">
                                                <StableTextArea
                                                    value=answer_val
                                                    on_change=on_change
                                                    placeholder=placeholder_txt
                                                    class="quiz-textarea"
                                                />
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>

                            <div class="quiz-path-banner">
                                <div class="quiz-section-header">
                                    <h3 class="quiz-section-title">
                                        {match current_lang {
                                            crate::i18n::Language::PtBr => "🧭 Dicas & Perguntas para os Jogadores sobre o Caminho",
                                            crate::i18n::Language::EnUs => "🧭 Guide & Questions for Players on the Path",
                                        }}
                                    </h3>
                                </div>
                                <p class="quiz-path-text">
                                    {match current_lang {
                                        crate::i18n::Language::PtBr => "O Caminho de um mago significa muito mais do que a Tradição a qual ele pertence, se ele pertencer a uma. As quatro facções apenas indicam com quem o seu mago anda. Seu Caminho define quem ele é.",
                                        crate::i18n::Language::EnUs => "A mage's Path means far more than the Tradition they belong to, if any. The factions merely indicate who your mage walks with. Their Path defines who they are.",
                                    }}
                                </p>
                                <p class="quiz-path-text">
                                    {match current_lang {
                                        crate::i18n::Language::PtBr => "O Caminho do seu mago oferece muitas sugestões sobre como o seu sósia místiko irá se comportar. Entretanto, lembre-se de que um Caminho não é um 'alinhamento' rígido — é uma predisposição. Você não tem que criá-la com antecedência — apenas imagine onde você eventualmente quer chegar e como. Pense no Caminho como um mapa, com um destino ideal, alguns marcos e um plano de viagem. A jornada verdadeira — o jogo — irá começar significativamente a partir daquele mapa. Todavia, ter alguma noção de onde você está indo deixará o caminho mais claro e as partidas mais fáceis de serem administradas.",
                                        crate::i18n::Language::EnUs => "Your mage's Path offers suggestions on how your mystical persona behaves. Remember that a Path is not a rigid 'alignment' — it is a predisposition. You don't have to map everything in advance — just imagine where you want to end up and how. Think of the Path as a road map with an ideal destination, milestones, and travel plans. Having a sense of direction clarifies your journey and enriches the chronicle.",
                                    }}
                                </p>
                            </div>

                            <div class="quiz-cards-list">
                                {entries_snapshot.clone().into_iter().enumerate().filter(|(_, e)| e.category == "player").map(|(idx, entry)| {
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

                                    let q_title = crate::i18n::tr_quiz_title(&entry.id, current_lang).to_string();
                                    let q_prompt = crate::i18n::tr_quiz_prompt(&entry.id, current_lang).to_string();
                                    let placeholder_txt = match current_lang {
                                        crate::i18n::Language::PtBr => "Escreva sua resposta detalhada aqui...",
                                        crate::i18n::Language::EnUs => "Write your detailed answer here...",
                                    };

                                    view! {
                                        <div class="quiz-question-card">
                                            <div class="quiz-question-header">
                                                <h3 class="quiz-question-title">{q_title}</h3>
                                            </div>
                                            <p class="quiz-question-prompt">{q_prompt}</p>
                                            <div class="quiz-textarea-wrapper">
                                                <StableTextArea
                                                    value=answer_val
                                                    on_change=on_change
                                                    placeholder=placeholder_txt
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
                                {match current_lang {
                                    crate::i18n::Language::PtBr => "💾 Concluir / Salvar Dossiê",
                                    crate::i18n::Language::EnUs => "💾 Save & Close Dossier",
                                }}
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
