//! Campo de especialização / modificador, com sugestões de especialidade.

use leptos::*;
use super::super::callback::Callback;
use super::super::specialty_picker::SpecialtyPicker;
use super::TraitField;

#[component]
pub fn ModifierCell(field: TraitField) -> impl IntoView {
    let TraitField {
        label,
        level,
        modifier,
        suggested_specialties,
        page_ref_str,
        modifier_ref,
        last_modifier_value,
        on_modifier_change,
        on_open_compendium,
        ..
    } = field;

    // Focus-lock: o valor externo só entra no campo quando ele não está em foco (não perde o cursor)
    let is_focused = create_rw_signal(false);
    create_effect(move |_| {
        let val = modifier.get();
        if !is_focused.get_untracked() {
            if let Some(elem) = modifier_ref.get() {
                elem.set_value(&val);
            }
            let _ = last_modifier_value.try_set(val);
        }
    });

    let on_blur = {
        let on_change = on_modifier_change.clone();
        move |_| {
            let _ = is_focused.try_set(false);
            if let Some(elem) = modifier_ref.get() {
                let val = elem.value();
                on_change.call(val.clone());
                let _ = last_modifier_value.try_set(val);
            }
        }
    };

    let picker = suggested_specialties.map(|specs| {
        let on_change = Callback::new(move |new_val: String| {
            let _ = last_modifier_value.try_set(new_val.clone());
            on_modifier_change.call(new_val);
        });
        view! {
            <SpecialtyPicker
                level=level
                modifier=modifier
                label=label
                page_ref_str=page_ref_str
                suggested_specialties=specs
                on_change=on_change
                on_open_compendium=on_open_compendium
                modifier_ref=modifier_ref
            />
        }
        .into_view()
    });

    view! {
        <div class="modifier-container tooltip-container" class:has-specialty-picker=suggested_specialties.is_some()>
            <input
                type="text"
                node_ref=modifier_ref
                class="field-modifier"
                placeholder="..."
                maxlength="60"
                on:focus=move |_| { let _ = is_focused.try_set(true); }
                on:input=move |ev| { let _ = last_modifier_value.try_set(event_target_value(&ev)); }
                on:blur=on_blur
            />
            <span class="tooltip-text" class:hidden=move || modifier.get().is_empty()>
                {modifier}
            </span>
            {picker.unwrap_or_else(|| view! { <span></span> }.into_view())}
        </div>
    }
}
