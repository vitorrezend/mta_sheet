use leptos::*;
use crate::components::{Callback, ValueField};
use crate::state::{CharacterData, DotOrigin};

const OTHER_TRAITS_KEY: &str = "other_traits";

fn is_default_other_trait_name(s: &str, slot_idx: usize) -> bool {
    let s_trim = s.trim();
    if s_trim.is_empty() {
        return true;
    }
    let s_lower = s_trim.to_lowercase();
    let num = slot_idx + 1;
    let pt_default = format!("outro traço {}", num).to_lowercase();
    let pt_default_no_accent = format!("outro traco {}", num).to_lowercase();
    let en_default = format!("other trait {}", num).to_lowercase();

    s_lower == pt_default
        || s_lower == pt_default_no_accent
        || s_lower == en_default
        || s_lower == "outro traço"
        || s_lower == "outro traco"
        || s_lower == "other trait"
}

fn get_slot_attr_keys(slot_idx: usize, stored_opt: Option<&str>) -> Vec<String> {
    let mut keys = Vec::new();
    if let Some(stored) = stored_opt {
        let trimmed = stored.trim();
        if !trimmed.is_empty() {
            keys.push(trimmed.to_string());
        }
    }
    let pt_key = format!("Outro Traço {}", slot_idx + 1);
    let en_key = format!("Other Trait {}", slot_idx + 1);
    if !keys.contains(&pt_key) {
        keys.push(pt_key);
    }
    if !keys.contains(&en_key) {
        keys.push(en_key);
    }
    keys
}

fn get_write_key(slot_idx: usize, stored_opt: Option<&str>, lang: crate::i18n::Language, data: &CharacterData) -> String {
    if let Some(stored) = stored_opt {
        let trimmed = stored.trim();
        if !trimmed.is_empty() && !is_default_other_trait_name(trimmed, slot_idx) {
            return trimmed.to_string();
        }
    }
    let keys = get_slot_attr_keys(slot_idx, stored_opt);
    for k in &keys {
        if data.attributes.contains_key(k) {
            return k.clone();
        }
    }
    format!("{} {}", crate::i18n::tr("other_trait_slot", lang), slot_idx + 1)
}

#[component]
pub fn OtherTraits() -> impl IntoView {
    let data = use_context::<ReadSignal<CharacterData>>()
        .expect("CharacterData signal context not found");
    let set_data = use_context::<WriteSignal<CharacterData>>()
        .expect("WriteSignal<CharacterData> context not found");

    let active_origin_ctx = use_context::<crate::components::character_sheet::ActiveDotOriginContext>();
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let render_slot = move |slot_idx: usize| {
        let label_signal = Signal::derive(move || {
            let l = lang();
            data.with(|d| {
                let stored = d.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|list| list.get(slot_idx))
                    .map(|s| s.as_str());

                if let Some(s) = stored {
                    let s_trim = s.trim();
                    if !s_trim.is_empty() && !is_default_other_trait_name(s_trim, slot_idx) {
                        return s_trim.to_string();
                    }
                }
                format!("{} {}", crate::i18n::tr("other_trait_slot", l), slot_idx + 1)
            })
        });

        let level_signal = Signal::derive(move || {
            data.with(|d| {
                let stored = d.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|l| l.get(slot_idx))
                    .map(|s| s.as_str());
                let keys = get_slot_attr_keys(slot_idx, stored);
                for k in &keys {
                    if let Some(attr) = d.attributes.get(k) {
                        return attr.level;
                    }
                }
                0
            })
        });

        let mod_signal = Signal::derive(move || {
            data.with(|d| {
                let stored = d.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|l| l.get(slot_idx))
                    .map(|s| s.as_str());
                let keys = get_slot_attr_keys(slot_idx, stored);
                for k in &keys {
                    if let Some(attr) = d.attributes.get(k) {
                        return attr.modifier.clone();
                    }
                }
                String::new()
            })
        });

        let origins_signal = Signal::derive(move || {
            data.with(|d| {
                let stored = d.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|l| l.get(slot_idx))
                    .map(|s| s.as_str());
                let keys = get_slot_attr_keys(slot_idx, stored);
                for k in &keys {
                    if let Some(attr) = d.attributes.get(k) {
                        return attr.get_origins(10);
                    }
                }
                vec![DotOrigin::Base; 10]
            })
        });

        let is_supernatural = Signal::derive(move || {
            data.with(|d| {
                let stored = d.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|l| l.get(slot_idx))
                    .map(|s| s.as_str());
                let keys = get_slot_attr_keys(slot_idx, stored);
                for k in &keys {
                    if let Some(attr) = d.attributes.get(k) {
                        return attr.is_supernatural;
                    }
                }
                false
            })
        });

        let on_level_change = move |new_lvl: i32| {
            let (write_key, current_origin) = data.with_untracked(|d| {
                let stored = d.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|l| l.get(slot_idx))
                    .map(|s| s.as_str());
                let k = get_write_key(slot_idx, stored, lang(), d);
                let orig = active_origin_ctx.map(|a| a.origin.get()).unwrap_or(DotOrigin::Base);
                (k, orig)
            });
            set_data.update(|s| {
                s.set_attribute_with_origin(&write_key, Some(new_lvl), None, current_origin);
            });
        };

        let on_mod_change = move |new_mod: String| {
            let write_key = data.with_untracked(|d| {
                let stored = d.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|l| l.get(slot_idx))
                    .map(|s| s.as_str());
                get_write_key(slot_idx, stored, lang(), d)
            });
            set_data.update(|s| {
                s.set_attribute(&write_key, None, Some(new_mod));
            });
        };

        let on_label_change = Callback::new(move |new_name: String| {
            let trimmed = new_name.trim().to_string();
            if trimmed.is_empty() {
                return;
            }
            set_data.update(|s| {
                let stored = s.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|l| l.get(slot_idx))
                    .cloned();
                let keys = get_slot_attr_keys(slot_idx, stored.as_deref());

                let is_new_default = is_default_other_trait_name(&trimmed, slot_idx);
                let effective_new_name = if is_new_default {
                    format!("{} {}", crate::i18n::tr("other_trait_slot", lang()), slot_idx + 1)
                } else {
                    trimmed.clone()
                };

                let list = s.custom_lists.entry(OTHER_TRAITS_KEY.to_string()).or_default();
                while list.len() <= slot_idx {
                    let next_num = list.len() + 1;
                    list.push(format!("{} {}", crate::i18n::tr("other_trait_slot", lang()), next_num));
                }
                list[slot_idx] = effective_new_name.clone();

                let mut found_attr = None;
                for k in &keys {
                    if let Some(val) = s.attributes.remove(k) {
                        found_attr = Some(val);
                        break;
                    }
                }
                if let Some(val) = found_attr {
                    s.attributes.insert(effective_new_name, val);
                }
            });
        });

        let on_dot_origin_change = Callback::new(move |(dot_idx, orig): (usize, DotOrigin)| {
            let write_key = data.with_untracked(|d| {
                let stored = d.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|l| l.get(slot_idx))
                    .map(|s| s.as_str());
                get_write_key(slot_idx, stored, lang(), d)
            });
            set_data.update(|s| {
                s.set_attribute_dot_origin(&write_key, dot_idx, orig);
            });
        });

        let on_toggle_supernatural = Callback::new(move |_| {
            let write_key = data.with_untracked(|d| {
                let stored = d.custom_lists
                    .get(OTHER_TRAITS_KEY)
                    .and_then(|l| l.get(slot_idx))
                    .map(|s| s.as_str());
                get_write_key(slot_idx, stored, lang(), d)
            });
            set_data.update(|s| {
                s.toggle_attribute_supernatural(&write_key);
            });
        });

        view! {
            <div class="trait-field-wrapper">
                <ValueField
                    label=label_signal
                    level=level_signal
                    modifier=mod_signal
                    is_supernatural=is_supernatural
                    on_toggle_supernatural=on_toggle_supernatural
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
                <span class="group-box-title">{move || crate::i18n::tr("other_traits", lang()).to_uppercase()}</span>
            </div>

            <div class="other-traits-grid">
                <div class="traits-column">
                    {(0..4).map(render_slot).collect_view()}
                </div>
                <div class="traits-column">
                    {(4..8).map(render_slot).collect_view()}
                </div>
            </div>
        </div>
    }
}
