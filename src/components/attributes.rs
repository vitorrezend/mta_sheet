use leptos::*;
use crate::components::ValueField;
use crate::state::{CharacterState, AttributeValue};

const ATTR_NAMES: &[&'static str] = &[
    "Força", "Destreza", "Vigor",
    "Carisma", "Manipulação", "Aparência",
    "Percepção", "Inteligência", "Raciocínio",
];

#[component]
pub fn Attributes() -> impl IntoView {
    // Carrega cada campo individualmente
    let (state, set_state) = create_signal(CharacterState::load_all(ATTR_NAMES, &[]));

    // Função auxiliar para atualizar um atributo específico
    let update_attr = move |name: &'static str, level: Option<i32>, modifier: Option<String>| {
        set_state.update(|s| {
            let attr = s.attributes.entry(name.to_string()).or_insert(AttributeValue::default());
            if let Some(l) = level { attr.level = l; }
            if let Some(m) = modifier { attr.modifier = m; }
            
            // Persistência Granular: Salva APENAS este campo no LocalStorage
            attr.save_individual(name);
        });
    };

    // Função auxiliar para renderizar um ValueField conectado ao estado central
    let attr_field = move |name: &'static str| {
        let level = Signal::derive(move || state.get().attributes.get(name).map(|a| a.level).unwrap_or(1).max(1));
        let modifier = Signal::derive(move || state.get().attributes.get(name).map(|a| a.modifier.clone()).unwrap_or_default());
        
        view! {
            <ValueField 
                label=Signal::derive(move || name.to_string()) 
                level=level
                modifier=modifier
                on_level_change=move |v| update_attr(name, Some(v), None)
                on_modifier_change=move |m| update_attr(name, None, Some(m))
                min_level=1
                max_chars=15
                on_remove=None
                on_label_change=None
            />
        }
    };

    view! {
        <div class="group-box">
            <span class="group-title">"Atributos"</span>
            <div class="attributes-block">
                <AttributeColumn title="Físicos">
                    {attr_field("Força")}
                    {attr_field("Destreza")}
                    {attr_field("Vigor")}
                </AttributeColumn>
                
                <AttributeColumn title="Sociais">
                    {attr_field("Carisma")}
                    {attr_field("Manipulação")}
                    {attr_field("Aparência")}
                </AttributeColumn>

                <AttributeColumn title="Mentais">
                    {attr_field("Percepção")}
                    {attr_field("Inteligência")}
                    {attr_field("Raciocínio")}
                </AttributeColumn>
            </div>
        </div>
    }
}

#[component]
fn AttributeColumn(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="attribute-column">
            <h3 class="column-title">{title}</h3>
            {children()}
        </div>
    }
}
