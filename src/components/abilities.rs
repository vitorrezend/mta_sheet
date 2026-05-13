use leptos::*;
use crate::components::ValueField;
use crate::state::{CharacterState, AttributeValue};

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
    // Lista consolidada para carregamento inicial
    let all_names: Vec<&'static str> = TALENTOS.iter()
        .chain(PERICIAS.iter())
        .chain(CONHECIMENTOS.iter())
        .cloned()
        .collect();
    
    // Carrega o estado inicial do LocalStorage para estes campos
    let (state, set_state) = create_signal(CharacterState::load_all(&all_names, &[]));

    // Estado para as listas de campos personalizados
    let (custom_talentos, set_custom_talentos) = create_signal(CharacterState::load_custom_list("Talentos"));
    let (custom_pericias, set_custom_pericias) = create_signal(CharacterState::load_custom_list("Perícias"));
    let (custom_conhecimentos, set_custom_conhecimentos) = create_signal(CharacterState::load_custom_list("Conhecimentos"));

    // Função para atualizar e persistir uma habilidade
    let update_ability = store_value(move |name: String, level: Option<i32>, modifier: Option<String>| {
        set_state.update(|s| {
            let attr = s.attributes.entry(name.clone()).or_insert(AttributeValue { level: 0, modifier: String::new() });
            if let Some(l) = level { attr.level = l; }
            if let Some(m) = modifier { attr.modifier = m; }
            attr.save_individual(&name);
        });
    });

    // Função para adicionar novo campo
    let add_custom = move |category: &'static str, set_list: WriteSignal<Vec<String>>| {
        set_list.update(|l| {
            l.push(format!("Novo_{}_{}", category, l.len())); // Nome temporário único
            CharacterState::save_custom_list(category, l);
        });
    };

    // Função para remover campo
    let remove_custom = move |category: &'static str, set_list: WriteSignal<Vec<String>>, name: String| {
        set_list.update(|l| {
            l.retain(|n| n != &name);
            CharacterState::save_custom_list(category, l);
        });
    };

    // Função para atualizar o nome de um campo personalizado
    let update_custom_name = move |category: &'static str, set_list: WriteSignal<Vec<String>>, old_name: String, new_name: String| {
        set_list.update(|l| {
            if let Some(pos) = l.iter().position(|n| n == &old_name) {
                l[pos] = new_name;
            }
            CharacterState::save_custom_list(category, l);
        });
    };

    // Helper para criar o campo de habilidade (estático ou dinâmico)
    let render_field = move |name: String, is_custom: bool, category: &'static str, set_list: Option<WriteSignal<Vec<String>>>| {
        let n_level = name.clone();
        let n_mod = name.clone();
        let n_label = name.clone();
        let n_change_label = name.clone();
        let n_update_level = name.clone();
        let n_update_mod = name.clone();
        let n_remove = name.clone();
        
        let level = Signal::derive(move || {
            state.get().attributes.get(&n_level).map(|a| a.level).unwrap_or(0)
        });
        let modifier = Signal::derive(move || {
            state.get().attributes.get(&n_mod).map(|a| a.modifier.clone()).unwrap_or_default()
        });

        view! {
            <ValueField 
                label=Signal::derive(move || n_label.clone())
                level=level
                modifier=modifier
                on_level_change=move |v| update_ability.with_value(|cb| cb(n_update_level.clone(), Some(v), None))
                on_modifier_change=move |m| update_ability.with_value(|cb| cb(n_update_mod.clone(), None, Some(m)))
                min_level=0
                max_chars=if is_custom { 10 } else { 12 }
                is_editable=is_custom
                on_label_change=set_list.map(|sl| {
                    let old = n_change_label.clone();
                    Callback::new(move |new_n| update_custom_name(category, sl, old.clone(), new_n))
                })
                on_remove=set_list.map(|sl| {
                    let n = n_remove.clone();
                    Callback::new(move |_| remove_custom(category, sl, n.clone()))
                })
            />
        }
    };

    view! {
        <div class="group-box">
            <span class="group-title">"Habilidades"</span>
            <div class="attributes-block">
                <AbilityColumn 
                    title="Talentos" 
                    on_add=Callback::new(move |_| add_custom("Talentos", set_custom_talentos))
                >
                    {TALENTOS.iter().map(|&n| render_field(n.to_string(), false, "Talentos", None)).collect_view()}
                    {move || custom_talentos.get().into_iter().map(|n| render_field(n, true, "Talentos", Some(set_custom_talentos))).collect_view()}
                </AbilityColumn>
                
                <AbilityColumn 
                    title="Perícias"
                    on_add=Callback::new(move |_| add_custom("Perícias", set_custom_pericias))
                >
                    {PERICIAS.iter().map(|&n| render_field(n.to_string(), false, "Perícias", None)).collect_view()}
                    {move || custom_pericias.get().into_iter().map(|n| render_field(n, true, "Perícias", Some(set_custom_pericias))).collect_view()}
                </AbilityColumn>

                <AbilityColumn 
                    title="Conhecimentos"
                    on_add=Callback::new(move |_| add_custom("Conhecimentos", set_custom_conhecimentos))
                >
                    {CONHECIMENTOS.iter().map(|&n| render_field(n.to_string(), false, "Conhecimentos", None)).collect_view()}
                    {move || custom_conhecimentos.get().into_iter().map(|n| render_field(n, true, "Conhecimentos", Some(set_custom_conhecimentos))).collect_view()}
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
