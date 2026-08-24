use leptos::*;
use crate::components::{Callback, ValueField};
use crate::state::{CharacterData, DotOrigin};

const OTHER_TRAITS_KEY: &str = "other_traits";

#[component]
pub fn OtherTraits() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let render_slot = move |slot_idx: usize| {
        let label_signal = Signal::derive(move || {
            data.with(|d| {
                d.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|l| l.get(slot_idx))
                    .cloned()
                    .unwrap_or_else(|| format!("Outro Traço {}", slot_idx + 1))
            })
        });

        let level_signal = Signal::derive(move || {
            let name = data.with(|d| {
                d.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|l| l.get(slot_idx))
                    .cloned()
                    .unwrap_or_else(|| format!("Outro Traço {}", slot_idx + 1))
            });
            data.with(|d| d.get_attribute_level(&name, 0))
        });

        let mod_signal = Signal::derive(move || {
            let name = data.with(|d| {
                d.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|l| l.get(slot_idx))
                    .cloned()
                    .unwrap_or_else(|| format!("Outro Traço {}", slot_idx + 1))
            });
            data.with(|d| d.get_attribute_modifier(&name))
        });

        let origins_signal = Signal::derive(move || {
            let name = data.with(|d| {
                d.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|l| l.get(slot_idx))
                    .cloned()
                    .unwrap_or_else(|| format!("Outro Traço {}", slot_idx + 1))
            });
            data.with(|d| d.attributes.get(&name).map(|a| a.dot_origins.clone()).unwrap_or_default())
        });

        let on_level_change = move |new_lvl: i32| {
            let name = data.with_untracked(|d| {
                d.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|l| l.get(slot_idx))
                    .cloned()
                    .unwrap_or_else(|| format!("Outro Traço {}", slot_idx + 1))
            });
            set_data.update(|s| {
                s.set_attribute(&name, Some(new_lvl), None);
            });
        };

        let on_mod_change = move |new_mod: String| {
            let name = data.with_untracked(|d| {
                d.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|l| l.get(slot_idx))
                    .cloned()
                    .unwrap_or_else(|| format!("Outro Traço {}", slot_idx + 1))
            });
            set_data.update(|s| {
                s.set_attribute(&name, None, Some(new_mod));
            });
        };

        let on_label_change = Callback::new(move |new_name: String| {
            if !new_name.trim().is_empty() {
                let old_name = data.with_untracked(|d| {
                    d.custom_lists
                        .get(OTHER_TRAITS_KEY)
                        .and_then(|l| l.get(slot_idx))
                        .cloned()
                        .unwrap_or_else(|| format!("Outro Traço {}", slot_idx + 1))
                });

                if new_name != old_name {
                    set_data.update(|s| {
                        let list = s.custom_lists.entry(OTHER_TRAITS_KEY.to_string()).or_default();
                        while list.len() <= slot_idx {
                            let next_num = list.len() + 1;
                            list.push(format!("Outro Traço {}", next_num));
                        }
                        list[slot_idx] = new_name.clone();

                        if let Some(old_val) = s.attributes.remove(&old_name) {
                            s.attributes.insert(new_name, old_val);
                        }
                    });
                }
            }
        });

        let on_dot_origin_change = Callback::new(move |(dot_idx, orig): (usize, DotOrigin)| {
            let name = data.with_untracked(|d| {
                d.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|l| l.get(slot_idx))
                    .cloned()
                    .unwrap_or_else(|| format!("Outro Traço {}", slot_idx + 1))
            });
            set_data.update(|s| {
                s.set_attribute_dot_origin(&name, dot_idx, orig);
            });
        });

        view! {
            <div class="trait-field-wrapper">
                <ValueField
                    label=label_signal
                    level=level_signal
                    modifier=mod_signal
                    on_level_change=on_level_change
                    on_modifier_change=on_mod_change
                    is_editable=true
                    on_label_change=on_label_change
                    origins=origins_signal
                    on_dot_origin_change=on_dot_origin_change
                />
            </div>
        }
    };

    view! {
        <div class="group-box other-traits-box">
            <div class="group-box-header">
                <span class="group-box-title">"OUTRAS CARACTERÍSTICAS (OTHER TRAITS)"</span>
            </div>

            <div class="other-traits-grid">
                <div class="traits-column">
                    {(0..3).map(render_slot).collect_view()}
                </div>
                <div class="traits-column">
                    {(3..6).map(render_slot).collect_view()}
                </div>
                <div class="traits-column">
                    {(6..9).map(render_slot).collect_view()}
                </div>
            </div>
        </div>
    }
}
