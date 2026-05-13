use leptos::*;
use crate::components::ValueField;
use crate::state::{CharacterState, AttributeValue};

const SPHERE_NAMES: &[&'static str] = &[
    "Correspondência", "Entropia", "Forças",
    "Vida", "Mente", "Matéria",
    "Primórdio", "Espírito", "Tempo",
];

#[component]
pub fn Spheres() -> impl IntoView {
    // Carrega cada campo individualmente
    let (state, set_state) = create_signal(CharacterState::load_all(SPHERE_NAMES, &[]));

    // Função auxiliar para atualizar uma esfera específica
    let update_sphere = move |name: &'static str, level: Option<i32>, modifier: Option<String>| {
        set_state.update(|s| {
            let attr = s.attributes.entry(name.to_string()).or_insert(AttributeValue::default());
            if let Some(l) = level { attr.level = l; }
            if let Some(m) = modifier { attr.modifier = m; }
            
            // Persistência Granular
            attr.save_individual(name);
        });
    };

    // Função auxiliar para renderizar um ValueField conectado ao estado central
    let sphere_field = move |name: &'static str| {
        let level = Signal::derive(move || state.get().attributes.get(name).map(|a| a.level).unwrap_or(0));
        let modifier = Signal::derive(move || state.get().attributes.get(name).map(|a| a.modifier.clone()).unwrap_or_default());
        
        view! {
            <ValueField 
                label=Signal::derive(move || name.to_string()) 
                level=level
                modifier=modifier
                on_level_change=move |v| update_sphere(name, Some(v), None)
                on_modifier_change=move |m| update_sphere(name, None, Some(m))
                min_level=0
                max_chars=11
                on_remove=None
                on_label_change=None
            />
        }
    };

    view! {
        <div class="group-box">
            <span class="group-title">"Esferas"</span>
            <div class="attributes-block">
                <SphereColumn>
                    {sphere_field("Correspondência")}
                    {sphere_field("Entropia")}
                    {sphere_field("Forças")}
                </SphereColumn>
                
                <SphereColumn>
                    {sphere_field("Vida")}
                    {sphere_field("Mente")}
                    {sphere_field("Matéria")}
                </SphereColumn>

                <SphereColumn>
                    {sphere_field("Primórdio")}
                    {sphere_field("Espírito")}
                    {sphere_field("Tempo")}
                </SphereColumn>
            </div>
        </div>
    }
}

#[component]
fn SphereColumn(children: Children) -> impl IntoView {
    view! {
        <div class="attribute-column">
            <div class="column-spacer" style="height: 0.8rem;"></div>
            {children()}
        </div>
    }
}
