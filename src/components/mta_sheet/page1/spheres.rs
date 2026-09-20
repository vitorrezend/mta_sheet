use leptos::*;
use crate::components::{Callback, ValueField};
use crate::state::{CharacterData, DotOrigin};
use crate::components::character_sheet::ActiveDotOriginContext;

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

    let affinity_name = Signal::derive(move || {
        data.with(|d| d.get_affinity_sphere().unwrap_or_default())
    });

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
                            if variant.eq_ignore_ascii_case(t) || variant.eq_ignore_ascii_case("techno") {
                                return t.clone();
                            } else if variant.eq_ignore_ascii_case(&mystic) || variant.eq_ignore_ascii_case("mystic") {
                                return mystic.clone();
                            }
                        }
                        // Fallback: se attributes contém a variante tecnocrática mas não a mística
                        if d.attributes.contains_key(t) && !d.attributes.contains_key(&mystic) {
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
                    let lvl = d.get_attribute_level(&act, 0);
                    if lvl > 0 {
                        return lvl;
                    }
                    if let Some(ref t) = techno {
                        let other = if act == *t { &mystic } else { t };
                        return d.get_attribute_level(other, 0);
                    }
                    lvl
                })
            }
        });

        let modifier = Signal::derive({
            let mystic = mystic.clone();
            let techno = techno.clone();
            move || {
                let act = active_name.get();
                data.with(|d| {
                    let mod_str = d.get_attribute_modifier(&act);
                    if !mod_str.is_empty() {
                        return mod_str;
                    }
                    if let Some(ref t) = techno {
                        let other = if act == *t { &mystic } else { t };
                        let other_mod = d.get_attribute_modifier(other);
                        if !other_mod.is_empty() {
                            return other_mod;
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
                    if let Some(attr) = d.attributes.get(&act) {
                        return attr.get_origins(5);
                    }
                    if let Some(ref t) = techno {
                        let other = if act == *t { &mystic } else { t };
                        if let Some(attr) = d.attributes.get(other) {
                            return attr.get_origins(5);
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

        let on_flip = techno.clone().map(|t| {
            let mystic = mystic.clone();
            let slot = slot.clone();
            Callback::new(move |_| {
                let current_act = active_name.get();
                let (from_name, to_name) = if current_act == t {
                    (t.clone(), mystic.clone())
                } else {
                    (mystic.clone(), t.clone())
                };

                set_data.update(|s| {
                    if let Some(attr) = s.attributes.remove(&from_name) {
                        s.attributes.insert(to_name.clone(), attr);
                    }
                    if let Some(ref aff) = s.get_affinity_sphere() {
                        if aff.eq_ignore_ascii_case(&from_name) {
                            s.set_affinity_sphere(Some(to_name.clone()));
                        }
                    }
                    s.labels.insert(format!("sphere_slot_{}", slot), to_name);
                });
            })
        });

        let flip_tooltip = techno.map(|t| {
            let mystic = mystic.clone();
            Signal::derive(move || {
                let current_lang = lang();
                let act = active_name.get();
                if act == t {
                    let target_tr = crate::i18n::tr_sphere(&mystic, current_lang);
                    match current_lang {
                        crate::i18n::Language::PtBr => format!("Alternar para {} (Místico)", target_tr),
                        crate::i18n::Language::EnUs => format!("Switch to {} (Mystic)", target_tr),
                    }
                } else {
                    let target_tr = crate::i18n::tr_sphere(&t, current_lang);
                    match current_lang {
                        crate::i18n::Language::PtBr => format!("Alternar para {} (Tecnocracia)", target_tr),
                        crate::i18n::Language::EnUs => format!("Switch to {} (Technocracy)", target_tr),
                    }
                }
            })
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
                on_flip=on_flip
                flip_tooltip=flip_tooltip
                min_level=0
                max_chars=18
            />
        }
    };

    view! {
        <div class="group-box spheres-group-box">
            <span class="group-title">{move || crate::i18n::tr("spheres", lang())}</span>
            <div class="spheres-header-bar">
                <span class="spheres-affinity-badge">
                    <span class="affinity-star-icon active" style="font-size: 0.85rem; margin-right: 4px;">"★"</span>
                    {move || {
                        let aff = affinity_name.get();
                        let current_lang = lang();
                        if aff.is_empty() {
                            view! {
                                <span class="affinity-badge-text empty">
                                    {match current_lang {
                                        crate::i18n::Language::PtBr => "Clique na estrela ao lado de uma Esfera para marcar como Afinidade",
                                        crate::i18n::Language::EnUs => "Click the star next to a Sphere to mark as Affinity",
                                    }}
                                </span>
                            }.into_view()
                        } else {
                            let aff_translated = crate::i18n::tr_sphere(&aff, current_lang);
                            view! {
                                <span class="affinity-badge-text selected">
                                    {match current_lang {
                                        crate::i18n::Language::PtBr => format!("Afinidade: {} (XP: Atual × 7)", aff_translated),
                                        crate::i18n::Language::EnUs => format!("Affinity: {} (XP: Current × 7)", aff_translated),
                                    }}
                                </span>
                            }.into_view()
                        }
                    }}
                </span>
            </div>
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
