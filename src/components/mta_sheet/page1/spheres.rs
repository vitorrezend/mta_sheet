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

    let sphere_field = move |name: &'static str| {
        let name_str = name.to_string();
        let name_str2 = name.to_string();
        let name_str3 = name.to_string();
        let name_str4 = name.to_string();
        let name_str5 = name.to_string();
        let name_str6 = name.to_string();
        let name_str7 = name.to_string();
        let name_str8 = name.to_string();

        let level = Signal::derive({
            let name = name_str.clone();
            move || data.with(|d| d.get_attribute_level(&name, 0))
        });
        let modifier = Signal::derive({
            let name = name_str2.clone();
            move || data.with(|d| d.get_attribute_modifier(&name))
        });
        let origins = Signal::derive({
            let name = name_str3.clone();
            move || data.with(|d| d.attributes.get(&name).map(|a| a.get_origins(5)).unwrap_or_else(|| vec![DotOrigin::Base; 5]))
        });
        let is_affinity = Signal::derive({
            let name = name_str7.clone();
            move || data.with(|d| d.get_affinity_sphere().map(|s| s.eq_ignore_ascii_case(&name)).unwrap_or(false))
        });

        let on_toggle_affinity = {
            let name = name_str8.clone();
            Callback::new(move |_| {
                let current_aff = data.with_untracked(|d| d.get_affinity_sphere());
                let is_current = current_aff.as_ref().map(|s| s.eq_ignore_ascii_case(&name)).unwrap_or(false);
                set_data.update(|s| {
                    if is_current {
                        s.set_affinity_sphere(None);
                    } else {
                        s.set_affinity_sphere(Some(name.clone()));
                    }
                });
            })
        };
        
        let on_level_change = {
            let name = name_str4.clone();
            move |v| update_sphere(name.clone(), Some(v), None)
        };
        let on_modifier_change = {
            let name = name_str5.clone();
            move |m| update_sphere(name.clone(), None, Some(m))
        };
        let on_dot_origin_change = {
            let name = name_str6.clone();
            Callback::new(move |(idx, orig)| update_sphere_dot(name.clone(), idx, orig))
        };

        view! {
            <ValueField 
                label=Signal::derive(move || name.to_string())
                level=level
                modifier=modifier
                origins=origins
                on_level_change=on_level_change
                on_modifier_change=on_modifier_change
                on_dot_origin_change=on_dot_origin_change
                is_starred=is_affinity
                on_toggle_star=on_toggle_affinity
                star_tooltip="Esfera de Afinidade ativa (XP: Atual × 7). Clique para alternar."
                min_level=0
                max_chars=18
            />
        }
    };

    view! {
        <div class="group-box spheres-group-box">
            <span class="group-title">"Esferas"</span>
            <div class="spheres-header-bar">
                <span class="spheres-affinity-badge">
                    <span class="affinity-star-icon active" style="font-size: 0.85rem; margin-right: 4px;">"★"</span>
                    {move || {
                        let aff = affinity_name.get();
                        if aff.is_empty() {
                            view! {
                                <span class="affinity-badge-text empty">"Clique na estrela ao lado de uma Esfera para marcar como Afinidade"</span>
                            }.into_view()
                        } else {
                            view! {
                                <span class="affinity-badge-text selected">"Afinidade: " <strong>{aff}</strong> " (XP: Atual × 7)"</span>
                            }.into_view()
                        }
                    }}
                </span>
            </div>
            <div class="attributes-block">
                <div class="attribute-column">
                    {sphere_field("Correspondência")}
                    {sphere_field("Entropia")}
                    {sphere_field("Forças")}
                </div>
                <div class="attribute-column">
                    {sphere_field("Vida")}
                    {sphere_field("Matéria")}
                    {sphere_field("Mente")}
                </div>
                <div class="attribute-column">
                    {sphere_field("Primórdio")}
                    {sphere_field("Espírito")}
                    {sphere_field("Tempo")}
                </div>
            </div>
        </div>
    }
}
