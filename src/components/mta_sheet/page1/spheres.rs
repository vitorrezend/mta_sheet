use leptos::*;
use crate::components::{Callback, ValueField};
use crate::state::{CharacterData, DotOrigin};
use crate::components::character_sheet::ActiveDotOriginContext;

fn find_sphere_attr<'a>(d: &'a CharacterData, act: &str) -> Option<&'a crate::state::AttributeValue> {
    if let Some(a) = d.attributes.get(act) {
        return Some(a);
    }
    let alias = match act {
        "Dados" => Some("Data"),
        "Data" => Some("Dados"),
        "Correspondência" => Some("Correspondence"),
        "Correspondence" => Some("Correspondência"),
        "Primórdio" => Some("Prime"),
        "Prime" => Some("Primórdio"),
        "Utilidade Primordial" => Some("Primal Utility"),
        "Primal Utility" => Some("Utilidade Primordial"),
        "Espírito" => Some("Spirit"),
        "Spirit" => Some("Espírito"),
        "Ciência Dimensional" => Some("Dimensional Science"),
        "Dimensional Science" => Some("Ciência Dimensional"),
        _ => None,
    };
    if let Some(alias_key) = alias {
        if let Some(a) = d.attributes.get(alias_key) {
            return Some(a);
        }
    }
    None
}

#[component]
pub fn Spheres() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");
    let active_origin_ctx = use_context::<ActiveDotOriginContext>();

    let update_sphere = move |name: String, level: Option<i32>, modifier: Option<String>| {
        let current_origin = active_origin_ctx.map(|a| a.origin.get()).unwrap_or(DotOrigin::Base);
        set_data.update(|s| {
            s.set_attribute_with_origin(&name, level, modifier, current_origin);
        });
    };

    let update_sphere_dot = move |name: String, dot_idx: usize, origin: DotOrigin| {
        set_data.update(|s| {
            s.set_attribute_dot_origin(&name, dot_idx, origin);
        });
    };

    let compendium_ctx = use_context::<crate::components::mta_sheet::page5::PracticeCompendiumContext>();
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let sphere_field = move |slot_key: &'static str, mystic_name: &'static str, techno_name: Option<&'static str>| {
        let mystic = mystic_name.to_string();
        let techno = techno_name.map(|t| t.to_string());
        let slot = slot_key.to_string();

        let active_name = Signal::derive({
            let mystic = mystic.clone();
            let techno = techno.clone();
            let slot = slot.clone();
            move || {
                data.with(|d| {
                    if let Some(ref t) = techno {
                        if let Some(variant) = d.labels.get(&format!("sphere_slot_{}", slot)) {
                            let is_techno = match slot.as_str() {
                                "correspondence" => variant.eq_ignore_ascii_case("Dados") || variant.eq_ignore_ascii_case("Data") || variant.eq_ignore_ascii_case("techno"),
                                "prime" => variant.eq_ignore_ascii_case("Utilidade Primordial") || variant.eq_ignore_ascii_case("Primal Utility") || variant.eq_ignore_ascii_case("techno"),
                                "spirit" => variant.eq_ignore_ascii_case("Ciência Dimensional") || variant.eq_ignore_ascii_case("Dimensional Science") || variant.eq_ignore_ascii_case("techno"),
                                _ => variant.eq_ignore_ascii_case(t) || variant.eq_ignore_ascii_case("techno"),
                            };
                            let is_mystic = match slot.as_str() {
                                "correspondence" => variant.eq_ignore_ascii_case("Correspondência") || variant.eq_ignore_ascii_case("Correspondence") || variant.eq_ignore_ascii_case("mystic"),
                                "prime" => variant.eq_ignore_ascii_case("Primórdio") || variant.eq_ignore_ascii_case("Prime") || variant.eq_ignore_ascii_case("mystic"),
                                "spirit" => variant.eq_ignore_ascii_case("Espírito") || variant.eq_ignore_ascii_case("Spirit") || variant.eq_ignore_ascii_case("mystic"),
                                _ => variant.eq_ignore_ascii_case(&mystic) || variant.eq_ignore_ascii_case("mystic"),
                            };
                            if is_techno {
                                return t.clone();
                            } else if is_mystic {
                                return mystic.clone();
                            }
                        }
                        // Fallback: se attributes contém a variante tecnocrática mas não a mística
                        let has_techno_attr = match slot.as_str() {
                            "correspondence" => d.attributes.contains_key("Dados") || d.attributes.contains_key("Data"),
                            "prime" => d.attributes.contains_key("Utilidade Primordial") || d.attributes.contains_key("Primal Utility"),
                            "spirit" => d.attributes.contains_key("Ciência Dimensional") || d.attributes.contains_key("Dimensional Science"),
                            _ => d.attributes.contains_key(t),
                        };
                        let has_mystic_attr = match slot.as_str() {
                            "correspondence" => d.attributes.contains_key("Correspondência") || d.attributes.contains_key("Correspondence"),
                            "prime" => d.attributes.contains_key("Primórdio") || d.attributes.contains_key("Prime"),
                            "spirit" => d.attributes.contains_key("Espírito") || d.attributes.contains_key("Spirit"),
                            _ => d.attributes.contains_key(&mystic),
                        };
                        if has_techno_attr && !has_mystic_attr {
                            return t.clone();
                        }
                    }
                    mystic.clone()
                })
            }
        });

        let level = Signal::derive({
            let mystic = mystic.clone();
            let techno = techno.clone();
            move || {
                let act = active_name.get();
                data.with(|d| {
                    if let Some(a) = find_sphere_attr(d, &act) {
                        if a.level > 0 {
                            return a.level;
                        }
                    }
                    if let Some(ref t) = techno {
                        let other = if act == *t { &mystic } else { t };
                        if let Some(a) = find_sphere_attr(d, other) {
                            return a.level;
                        }
                    }
                    0
                })
            }
        });

        let modifier = Signal::derive({
            let mystic = mystic.clone();
            let techno = techno.clone();
            move || {
                let act = active_name.get();
                data.with(|d| {
                    if let Some(a) = find_sphere_attr(d, &act) {
                        if !a.modifier.is_empty() {
                            return a.modifier.clone();
                        }
                    }
                    if let Some(ref t) = techno {
                        let other = if act == *t { &mystic } else { t };
                        if let Some(a) = find_sphere_attr(d, other) {
                            if !a.modifier.is_empty() {
                                return a.modifier.clone();
                            }
                        }
                    }
                    String::new()
                })
            }
        });

        let origins = Signal::derive({
            let mystic = mystic.clone();
            let techno = techno.clone();
            move || {
                let act = active_name.get();
                data.with(|d| {
                    if let Some(a) = find_sphere_attr(d, &act) {
                        return a.get_origins(5);
                    }
                    if let Some(ref t) = techno {
                        let other = if act == *t { &mystic } else { t };
                        if let Some(a) = find_sphere_attr(d, other) {
                            return a.get_origins(5);
                        }
                    }
                    vec![DotOrigin::Base; 5]
                })
            }
        });

        let is_affinity = Signal::derive({
            let mystic = mystic.clone();
            let techno = techno.clone();
            move || {
                let act = active_name.get();
                data.with(|d| {
                    d.get_affinity_sphere().map(|s| {
                        s.eq_ignore_ascii_case(&act)
                            || (techno.as_ref().map(|t| s.eq_ignore_ascii_case(t)).unwrap_or(false))
                            || s.eq_ignore_ascii_case(&mystic)
                    }).unwrap_or(false)
                })
            }
        });

        let on_toggle_affinity = Callback::new(move |_| {
            let act = active_name.get();
            let current_aff = data.with_untracked(|d| d.get_affinity_sphere());
            let is_current = current_aff.as_ref().map(|s| s.eq_ignore_ascii_case(&act)).unwrap_or(false);
            set_data.update(|s| {
                if is_current {
                    s.set_affinity_sphere(None);
                } else {
                    s.set_affinity_sphere(Some(act));
                }
            });
        });

        let on_level_change = move |v| {
            let act = active_name.get();
            update_sphere(act, Some(v), None);
        };

        let on_modifier_change = move |m| {
            let act = active_name.get();
            update_sphere(act, None, Some(m));
        };

        let on_dot_origin_change = Callback::new(move |(idx, orig)| {
            let act = active_name.get();
            update_sphere_dot(act, idx, orig);
        });

        let on_open_compendium = {
            let ctx = compendium_ctx.clone();
            Callback::new(move |_| {
                let act = active_name.get();
                if let Some(ref c) = ctx {
                    c.open_sphere.call((None, act));
                }
            })
        };

        let sphere_page_ref = Signal::derive(move || {
            let act = active_name.get();
            crate::compendium::spheres::find_sphere(&act)
                .map(|s| s.page_ref.to_string())
                .unwrap_or_else(|| "M20, pp. 511-534".to_string())
        });

        let suggested_specialties = Signal::derive(move || {
            let act = active_name.get();
            crate::compendium::spheres::get_suggested_specialties(&act, lang())
        });

        view! {
            <ValueField 
                label=Signal::derive(move || crate::i18n::tr_sphere(&active_name.get(), lang()).to_string())
                level=level
                modifier=modifier
                origins=origins
                on_level_change=on_level_change
                on_modifier_change=on_modifier_change
                on_dot_origin_change=on_dot_origin_change
                is_starred=is_affinity
                on_toggle_star=on_toggle_affinity
                star_tooltip="Esfera de Afinidade ativa (XP: Atual × 7). Clique para alternar."
                suggested_specialties=suggested_specialties
                on_open_compendium=on_open_compendium
                compendium_page_ref=sphere_page_ref
                min_level=0
                max_chars=18
            />
        }
    };

    view! {
        <div class="group-box spheres-group-box">
            <span class="group-title">{move || crate::i18n::tr("spheres", lang())}</span>
            <div class="attributes-block">
                <div class="attribute-column">
                    {sphere_field("correspondence", "Correspondência", Some("Dados"))}
                    {sphere_field("entropy", "Entropia", None)}
                    {sphere_field("forces", "Forças", None)}
                </div>
                <div class="attribute-column">
                    {sphere_field("life", "Vida", None)}
                    {sphere_field("matter", "Matéria", None)}
                    {sphere_field("mind", "Mente", None)}
                </div>
                <div class="attribute-column">
                    {sphere_field("prime", "Primórdio", Some("Utilidade Primordial"))}
                    {sphere_field("spirit", "Espírito", Some("Ciência Dimensional"))}
                    {sphere_field("time", "Tempo", None)}
                </div>
            </div>
        </div>
    }
}
