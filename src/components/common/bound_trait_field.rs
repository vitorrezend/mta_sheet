use leptos::*;
use crate::components::{Callback, ValueField};
use crate::state::{CharacterData, DotOrigin};
use crate::components::character_sheet::ActiveDotOriginContext;

#[component]
pub fn BoundAttributeField(
    name: &'static str,
    #[prop(default = true)] enable_compendium: bool,
    #[prop(default = 1)] min_level: i32,
    #[prop(default = 18)] max_chars: usize,
) -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData WriteSignal not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData ReadSignal not found");
    let active_origin_ctx = use_context::<ActiveDotOriginContext>();
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let level = Signal::derive(move || data.with(|d| d.get_attribute_level(name, min_level)));
    let modifier = Signal::derive(move || data.with(|d| d.get_attribute_modifier(name)));
    let origins = Signal::derive(move || {
        data.with(|d| {
            d.attributes
                .get(name)
                .map(|a| a.get_origins(10))
                .unwrap_or_else(|| vec![DotOrigin::Base; 10])
        })
    });
    let is_supernatural = Signal::derive(move || data.with(|d| d.is_attribute_supernatural(name)));

    let on_level_change = move |v| {
        let current_origin = active_origin_ctx.map(|a| a.origin.get_untracked()).unwrap_or(DotOrigin::Base);
        set_data.update(|s| {
            s.set_attribute_with_origin(name, Some(v), None, current_origin);
        });
    };

    let on_modifier_change = move |m| {
        let current_origin = active_origin_ctx.map(|a| a.origin.get_untracked()).unwrap_or(DotOrigin::Base);
        set_data.update(|s| {
            s.set_attribute_with_origin(name, None, Some(m), current_origin);
        });
    };

    let on_dot_origin_change = Callback::new(move |(idx, orig)| {
        set_data.update(|s| {
            s.set_attribute_dot_origin(name, idx, orig);
        });
    });

    let on_toggle_supernatural = Callback::new(move |_| {
        set_data.update(|s| {
            s.toggle_attribute_supernatural(name);
        });
    });

    let suggested_specialties = if enable_compendium {
        Signal::derive(move || {
            crate::compendium::attributes::get_suggested_specialties(name, lang()).to_vec()
        })
    } else {
        Signal::derive(Vec::new)
    };

    let page_ref = if enable_compendium {
        crate::compendium::attributes::find_attribute(name)
            .map(|a| a.page_ref)
            .unwrap_or("M20, p. 273")
    } else {
        ""
    };

    let compendium_ctx = use_context::<crate::components::mta_sheet::page5::PracticeCompendiumContext>();
    let on_open_compendium = if enable_compendium {
        Callback::new(move |_| {
            if let Some(ref c) = compendium_ctx {
                c.open_attribute.call((None, name.to_string()));
            }
        })
    } else {
        Callback::new(|_| ())
    };

    view! {
        <ValueField
            label=Signal::derive(move || crate::i18n::tr_attr(name, lang()).to_string())
            level=level
            modifier=modifier
            origins=origins
            is_supernatural=is_supernatural
            on_toggle_supernatural=on_toggle_supernatural
            on_level_change=on_level_change
            on_modifier_change=on_modifier_change
            on_dot_origin_change=on_dot_origin_change
            suggested_specialties=suggested_specialties
            on_open_compendium=on_open_compendium
            compendium_page_ref=page_ref
            min_level=min_level
            max_chars=max_chars
        />
    }
}
