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

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    view! {
        <div class="group-box chantry-box">
            <div class="chantry-header-row">
                <span class="group-title">{move || crate::i18n::tr("chantry_title", lang())}</span>
                <span class="chantry-subtitle">{move || crate::i18n::tr("chantry_sub", lang())}</span>
                <button 
                    type="button" 
                    class="btn-add-chantry"
                    on:click=add_row
                    title=move || crate::i18n::tr("add_location", lang())
                >
                    {move || crate::i18n::tr("add_location", lang())}
                </button>
            </div>

            <div class="chantry-table-header">
                <div class="chantry-col-header chantry-loc-header">{move || crate::i18n::tr("location_header", lang())}</div>
                <div class="chantry-col-header chantry-desc-header">{move || crate::i18n::tr("description_header", lang())}</div>
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
                                        placeholder=Signal::derive(move || match lang() {
                                            crate::i18n::Language::PtBr => "Ex: Biblioteca Subterrânea, Sanctum Principal...".to_string(),
                                            crate::i18n::Language::EnUs => "Ex: Underground Library, Main Sanctum...".to_string(),
                                        })
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
                                        placeholder=Signal::derive(move || match lang() {
                                            crate::i18n::Language::PtBr => "Ex: Espaço protegido por Prime 3 / Mind 2 para rituais coletivos...".to_string(),
                                            crate::i18n::Language::EnUs => "Ex: Space warded by Prime 3 / Mind 2 for rituals...".to_string(),
                                        })
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
