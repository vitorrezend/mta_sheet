use leptos::*;
use crate::components::ValueField;
use crate::state::{CharacterData, DotOrigin};

const OTHER_TRAITS_KEY: &str = "other_traits";
const DEFAULT_TRAIT_COUNT: usize = 9;

#[component]
pub fn OtherTraits() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let trait_names = Signal::derive(move || {
        data.with(|d| {
            let list = d.custom_lists.get(OTHER_TRAITS_KEY).cloned().unwrap_or_default();
            if list.is_empty() {
                (1..=DEFAULT_TRAIT_COUNT).map(|i| format!("Outro Traço {}", i)).collect::<Vec<_>>()
            } else {
                list
            }
        })
    });

    let add_trait = move |_| {
        set_data.update(|s| {
            let list = s.custom_lists.entry(OTHER_TRAITS_KEY.to_string()).or_default();
            let next_idx = list.len() + 1;
            list.push(format!("Novo Traço {}", next_idx));
        });
    };

    view! {
        <div class="group-box other-traits-box">
            <div class="group-box-header">
                <span class="group-box-title">"OUTRAS CARACTERÍSTICAS (OTHER TRAITS)"</span>
                <button type="button" class="add-trait-header-btn" on:click=add_trait title="Adicionar novo traço">
                    "+ Adicionar"
                </button>
            </div>

            <div class="other-traits-grid">
                {move || {
                    let traits = trait_names.get();
                    let total = traits.len();
                    let col_size = (total + 2) / 3;

                    (0..3).map(|col_idx| {
                        let start = col_idx * col_size;
                        let end = (start + col_size).min(total);
                        let slice = if start < total { traits[start..end].to_vec() } else { Vec::new() };

                        view! {
                            <div class="traits-column">
                                {slice.into_iter().enumerate().map(|(rel_idx, trait_name)| {
                                    let global_idx = start + rel_idx;
                                    let key = trait_name.clone();
                                    let key_signal = Signal::derive(move || key.clone());

                                    let level_signal = Signal::derive({
                                        let k = trait_name.clone();
                                        move || data.with(|d| d.get_attribute_level(&k, 0))
                                    });

                                    let mod_signal = Signal::derive({
                                        let k = trait_name.clone();
                                        move || data.with(|d| d.get_attribute_modifier(&k))
                                    });

                                    let origins_signal = Signal::derive({
                                        let k = trait_name.clone();
                                        move || data.with(|d| d.attributes.get(&k).map(|a| a.dot_origins.clone()).unwrap_or_default())
                                    });

                                    let on_level_change = {
                                        let k = trait_name.clone();
                                        move |new_lvl: i32| {
                                            set_data.update(|s| {
                                                s.set_attribute(&k, Some(new_lvl), None);
                                            });
                                        }
                                    };

                                    let on_mod_change = {
                                        let k = trait_name.clone();
                                        move |new_mod: String| {
                                            set_data.update(|s| {
                                                s.set_attribute(&k, None, Some(new_mod));
                                            });
                                        }
                                    };

                                    let on_label_change = Callback::new({
                                        let old_k = trait_name.clone();
                                        move |new_name: String| {
                                            if !new_name.trim().is_empty() && new_name != old_k {
                                                set_data.update(|s| {
                                                    let list = s.custom_lists.entry(OTHER_TRAITS_KEY.to_string()).or_default();
                                                    if global_idx < list.len() {
                                                        list[global_idx] = new_name.clone();
                                                    }
                                                    if let Some(old_val) = s.attributes.remove(&old_k) {
                                                        s.attributes.insert(new_name, old_val);
                                                    }
                                                });
                                            }
                                        }
                                    });

                                    let on_dot_origin_change = Callback::new({
                                        let k = trait_name.clone();
                                        move |(dot_idx, orig): (usize, DotOrigin)| {
                                            set_data.update(|s| {
                                                s.set_attribute_dot_origin(&k, dot_idx, orig);
                                            });
                                        }
                                    });

                                    let on_remove = Callback::new({
                                        let k = trait_name.clone();
                                        move |_| {
                                            set_data.update(|s| {
                                                let list = s.custom_lists.entry(OTHER_TRAITS_KEY.to_string()).or_default();
                                                if global_idx < list.len() {
                                                    list.remove(global_idx);
                                                }
                                                s.attributes.remove(&k);
                                            });
                                        }
                                    });

                                    view! {
                                        <div class="trait-field-wrapper">
                                            <ValueField
                                                label=key_signal
                                                level=level_signal
                                                modifier=mod_signal
                                                on_level_change=on_level_change
                                                on_modifier_change=on_mod_change
                                                is_editable=true
                                                on_label_change=on_label_change
                                                origins=origins_signal
                                                on_dot_origin_change=on_dot_origin_change
                                                on_remove=on_remove
                                            />
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }
                    }).collect_view().into_view()
                }}
            </div>
        </div>
    }
}
