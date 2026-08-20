use leptos::*;
use crate::components::ValueField;
use crate::state::{CharacterData, DotOrigin};
use crate::components::character_sheet::ActiveDotOriginContext;

const TALENTOS: &[&'static str] = &[
    "Prontidão", "Esportes", "Briga", "Esquiva", "Empatia", 
    "Expressão", "Intimidação", "Liderança", "Manha", "Lábia"
];

const PERICIAS: &[&'static str] = &[
    "Empatia c/ Animais", "Condução", "Etiqueta", "Armas de Fogo", "Meditação", 
    "Armas Brancas", "Performance", "Furtividade", "Sobrevivência", "Tecnologia"
];

const CONHECIMENTOS: &[&'static str] = &[
    "Acadêmicos", "Computador", "Cosmologia", "Enigmas", "Investigação", 
    "Direito", "Medicina", "Ocultismo", "Política", "Ciência"
];

#[component]
pub fn Abilities() -> impl IntoView {
    let set_data = use_context::<WriteSignal<CharacterData>>().expect("CharacterData context not found");
    let data = use_context::<ReadSignal<CharacterData>>().expect("CharacterData context not found");
    let active_origin_ctx = use_context::<ActiveDotOriginContext>();

    // Função para atualizar uma habilidade
    let update_ability = move |name: String, level: Option<i32>, modifier: Option<String>| {
        let current_origin = active_origin_ctx.map(|a| a.origin.get()).unwrap_or(DotOrigin::Base);
        set_data.update(|s| {
            s.set_attribute_with_origin(&name, level, modifier, current_origin);
        });
    };

    let update_ability_dot = move |name: String, dot_idx: usize, origin: DotOrigin| {
        set_data.update(|s| {
            s.set_attribute_dot_origin(&name, dot_idx, origin);
        });
    };

    // Função para adicionar novo campo
    let add_custom = move |category: &'static str| {
        set_data.update(|s| {
            let list = s.custom_lists.entry(category.to_string()).or_default();
            list.push(format!("Novo_{}_{}", category, list.len()));
        });
    };

    // Função para remover campo
    let remove_custom = move |category: &'static str, name: String| {
        set_data.update(|s| {
            if let Some(list) = s.custom_lists.get_mut(category) {
                list.retain(|n| n != &name);
            }
        });
    };

    // Função para atualizar o nome de um campo personalizado
    let update_custom_name = move |category: &'static str, old_name: String, new_name: String| {
        set_data.update(|s| {
            if let Some(list) = s.custom_lists.get_mut(category) {
                if let Some(pos) = list.iter().position(|n| n == &old_name) {
                    list[pos] = new_name.clone();
                }
            }
            // Mover os dados da habilidade do nome antigo para o novo
            if let Some(attr) = s.attributes.remove(&old_name) {
                s.attributes.insert(new_name, attr);
            }
        });
    };

    // Helper para criar o campo de habilidade (estático ou dinâmico)
    let render_field = move |name: String, is_custom: bool, category: &'static str| {
        let n_level = name.clone();
        let n_mod = name.clone();
        let n_origins = name.clone();
        let n_label = name.clone();
        let n_change_label = name.clone();
        let n_update_level = name.clone();
        let n_update_mod = name.clone();
        let n_update_dot = name.clone();
        let n_remove = name.clone();
        
        let level = Signal::derive({
            let name = n_level.clone();
            move || data.get().get_attribute_level(&name, 0)
        });
        let modifier = Signal::derive({
            let name = n_mod.clone();
            move || data.get().get_attribute_modifier(&name)
        });
        let origins = Signal::derive({
            let name = n_origins.clone();
            move || data.get().attributes.get(&name).map(|a| a.get_origins(5)).unwrap_or_else(|| vec![DotOrigin::Base; 5])
        });

        let on_dot_origin_change = {
            let name = n_update_dot.clone();
            Callback::new(move |(idx, orig)| update_ability_dot(name.clone(), idx, orig))
        };

        if is_custom {
            let old = n_change_label.clone();
            let n = n_remove.clone();
            view! {
                <ValueField 
                    label=Signal::derive(move || n_label.clone())
                    level=level
                    modifier=modifier
                    origins=origins
                    on_level_change=move |v| update_ability(n_update_level.clone(), Some(v), None)
                    on_modifier_change=move |m| update_ability(n_update_mod.clone(), None, Some(m))
                    on_dot_origin_change=on_dot_origin_change
                    min_level=0
                    max_chars=18
                    is_editable=true
                    on_label_change=Callback::new(move |new_n| update_custom_name(category, old.clone(), new_n))
                    on_remove=Callback::new(move |_| remove_custom(category, n.clone()))
                />
            }.into_view()
        } else {
            view! {
                <ValueField 
                    label=Signal::derive(move || n_label.clone())
                    level=level
                    modifier=modifier
                    origins=origins
                    on_level_change=move |v| update_ability(n_update_level.clone(), Some(v), None)
                    on_modifier_change=move |m| update_ability(n_update_mod.clone(), None, Some(m))
                    on_dot_origin_change=on_dot_origin_change
                    min_level=0
                    max_chars=18
                    is_editable=false
                />
            }.into_view()
        }
    };

    view! {
        <div class="group-box">
            <span class="group-title">"Habilidades"</span>
            <div class="attributes-block">
                <AbilityColumn 
                    title="Talentos" 
                    on_add=Callback::new(move |_| add_custom("Talentos"))
                >
                    {TALENTOS.iter().map(|&n| render_field(n.to_string(), false, "Talentos")).collect_view()}
                    {move || data.get().custom_lists.get("Talentos").cloned().unwrap_or_default().into_iter().map(|n| render_field(n, true, "Talentos")).collect_view()}
                </AbilityColumn>
                
                <AbilityColumn 
                    title="Perícias"
                    on_add=Callback::new(move |_| add_custom("Perícias"))
                >
                    {PERICIAS.iter().map(|&n| render_field(n.to_string(), false, "Perícias")).collect_view()}
                    {move || data.get().custom_lists.get("Perícias").cloned().unwrap_or_default().into_iter().map(|n| render_field(n, true, "Perícias")).collect_view()}
                </AbilityColumn>

                <AbilityColumn 
                    title="Conhecimentos"
                    on_add=Callback::new(move |_| add_custom("Conhecimentos"))
                >
                    {CONHECIMENTOS.iter().map(|&n| render_field(n.to_string(), false, "Conhecimentos")).collect_view()}
                    {move || data.get().custom_lists.get("Conhecimentos").cloned().unwrap_or_default().into_iter().map(|n| render_field(n, true, "Conhecimentos")).collect_view()}
                </AbilityColumn>
            </div>
        </div>
    }
}

#[component]
fn AbilityColumn(title: &'static str, children: Children, on_add: Callback<()>) -> impl IntoView {
    view! {
        <div class="attribute-column">
            <h3 class="column-title">{title}</h3>
            {children()}
            <button class="add-field-btn" on:click=move |_| on_add.call(())>"+"</button>
        </div>
    }
}
