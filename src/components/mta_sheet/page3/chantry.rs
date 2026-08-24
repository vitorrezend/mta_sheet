use leptos::*;
use crate::components::{Callback, StableTextInput};
use crate::state::{CharacterData, ChantryEntry};

#[component]
pub fn Chantry() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let chantry_len = Signal::derive(move || data.with(|d| d.chantry.len()));

    let add_row = move |_| {
        set_data.update(|s| {
            s.chantry.push(ChantryEntry::default());
        });
    };

    let remove_row = move |idx: usize| {
        set_data.update(|s| {
            if s.chantry.len() > 1 && idx < s.chantry.len() {
                s.chantry.remove(idx);
            }
        });
    };

    view! {
        <div class="group-box chantry-box">
            <div class="chantry-header-row">
                <span class="group-title">"CHANTRY"</span>
                <span class="chantry-subtitle">"Capela / Fundação / Domínio Místico"</span>
                <button 
                    type="button" 
                    class="btn-add-chantry"
                    on:click=add_row
                    title="Adicionar Linha de Capela"
                >
                    "+ Adicionar Local"
                </button>
            </div>

            <div class="chantry-table-header">
                <div class="chantry-col-header chantry-loc-header">"LOCATION (Localização / Sala / Espaço)"</div>
                <div class="chantry-col-header chantry-desc-header">"DESCRIPTION (Descrição & Função)"</div>
                <div class="chantry-col-header chantry-action-header">""</div>
            </div>

            <div class="chantry-rows-container">
                {move || {
                    let len = chantry_len.get();
                    (0..len).map(|idx| {
                        view! {
                            <div class="chantry-row">
                                <div class="chantry-loc-cell">
                                    <StableTextInput 
                                        class="chantry-input chantry-loc-input"
                                        placeholder="Ex: Biblioteca Subterrânea, Sanctum Principal..."
                                        value=Signal::derive(move || {
                                            data.with(|d| {
                                                d.chantry.get(idx).map(|c| c.location.clone()).unwrap_or_default()
                                            })
                                        })
                                        on_change=Callback::new(move |val| {
                                            set_data.update(|s| {
                                                while s.chantry.len() <= idx { s.chantry.push(ChantryEntry::default()); }
                                                s.chantry[idx].location = val;
                                            });
                                        })
                                    />
                                </div>

                                <div class="chantry-desc-cell">
                                    <StableTextInput 
                                        class="chantry-input chantry-desc-input"
                                        placeholder="Ex: Espaço protegido por Prime 3 / Mind 2 para rituais coletivos..."
                                        value=Signal::derive(move || {
                                            data.with(|d| {
                                                d.chantry.get(idx).map(|c| c.description.clone()).unwrap_or_default()
                                            })
                                        })
                                        on_change=Callback::new(move |val| {
                                            set_data.update(|s| {
                                                while s.chantry.len() <= idx { s.chantry.push(ChantryEntry::default()); }
                                                s.chantry[idx].description = val;
                                            });
                                        })
                                    />
                                </div>

                                <div class="chantry-action-cell">
                                    {move || {
                                        if chantry_len.get() > 1 {
                                            view! {
                                                <button 
                                                    type="button" 
                                                    class="btn-delete-row" 
                                                    on:click=move |_| remove_row(idx)
                                                    title="Remover Linha"
                                                >
                                                    "×"
                                                </button>
                                            }.into_view()
                                        } else {
                                            view! { <span class="btn-delete-placeholder"></span> }.into_view()
                                        }
                                    }}
                                </div>
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}
