use std::fs;
use std::path::{Path, PathBuf};

fn get_all_rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).expect("Failed to read directory") {
            let entry = entry.expect("Invalid entry");
            let path = entry.path();
            if path.is_dir() {
                files.extend(get_all_rs_files(&path));
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                files.push(path);
            }
        }
    }
    files
}

#[test]
fn test_no_reactive_anti_patterns_in_components() {
    let components_dir = Path::new("src/components");
    let rs_files = get_all_rs_files(components_dir);
    assert!(!rs_files.is_empty(), "Nenhum arquivo .rs encontrado em src/components");

    let mut violations = Vec::new();

    for file_path in &rs_files {
        let content = fs::read_to_string(file_path).expect("Failed to read file");

        // Regra 1: Listas dinâmicas (custom_lists) NÃO devem usar {move || ... collect_view()}
        // Devem usar o componente <For /> para evitar recriação e perda de foco no DOM.
        if content.contains("custom_lists") {
            if content.contains(".into_iter().map(") && content.contains(".collect_view()") {
                violations.push(format!(
                    "[{:?}] Violação da Regra 1: Lista dinâmica usando .collect_view() em vez de <For />. Isso causa destruição e recriação do DOM e perda de foco.",
                    file_path
                ));
            }
        }

        // Regra 2: Intervalos estáticos numéricos (ex: 1..=5, 0..10) NÃO devem ser envolvidos em `move || (X..Y)`
        // Isso destruiria os elementos spans/dots a cada alteração de estado.
        let static_range_patterns = [
            "move || (1..=5).map",
            "move || (1..=10).map",
            "move || (0..10).map",
            "move || (10..20).map",
            "move || (0..7).map",
        ];
        for pattern in static_range_patterns {
            if content.contains(pattern) {
                violations.push(format!(
                    "[{:?}] Violação da Regra 2: Intervalo estático envolvido em closure reativa '{}'. Remova o 'move ||' para manter os nós DOM estáveis.",
                    file_path, pattern
                ));
            }
        }

        // Regra 3: Ações de remoção de itens dinâmicos devem adiar o desmonte com request_animation_frame
        // para evitar que closures ativas no JS sejam descartadas durante o clique.
        if content.contains("remove_item") || content.contains("remove_custom") {
            if content.contains("custom_lists.get_mut") && !content.contains("request_animation_frame") {
                violations.push(format!(
                    "[{:?}] Violação da Regra 3: Função de remoção modifica custom_lists sem 'request_animation_frame'. Isso pode disparar 'closure invoked recursively or after being dropped'.",
                    file_path
                ));
            }
        }

        // Regra 4: Campos editáveis não devem usar sinais de alternância (editing_label / local_label)
        // com morphing condicional span <-> input, pois isso causa perda de cursor e loops de renderização.
        if content.contains("editing_label") || content.contains("local_label") {
            violations.push(format!(
                "[{:?}] Violação da Regra 4: Uso de 'editing_label' / 'local_label'. Campos editáveis devem renderizar <input> diretamente quando is_editable for verdadeiro para evitar perda de cursor.",
                file_path
            ));
        }

        // Regra 5: Modais com rolagem interna de listas (ex: Quiz/Dossiê) não devem usar `data.with(`
        // diretamente no corpo do modal renderizado, pois isso recria os nós DOM e reseta a rolagem no blur.
        if file_path.to_string_lossy().contains("quiz_modal") {
            if content.contains("let quiz_entries = data.with(") {
                violations.push(format!(
                    "[{:?}] Violação da Regra 5: QuizModal assina 'data.with' no bloco do modal. Use estado local desacoplado ou 'with_untracked' para manter a rolagem estável.",
                    file_path
                ));
            }
        }

        // Regra 6: Componentes no body NÃO devem renderizar <link rel="stylesheet">
        // O navegador move <link> do body para o head antes da hidratação, quebrando o alinhamento de nós DOM.
        if content.contains("<link") && content.contains("stylesheet") {
            violations.push(format!(
                "[{:?}] Violação da Regra 6: Tag '<link rel=\"stylesheet\">' encontrada dentro do corpo do componente. Estilos devem ser registrados via <Stylesheet /> no App em src/lib.rs para evitar que o navegador desloque os nós DOM antes da hidratação.",
                file_path
            ));
        }

        // Regra 7: Listeners globais do navegador (window_event_listener) DEVEM conter on_cleanup
        // e usar .try_get() / .try_get_untracked() para evitar 'Attempted to get a signal after it was disposed'.
        let is_data_file = file_path.to_string_lossy().contains("patch_notes_data");
        if !is_data_file && content.contains("window_event_listener(") {
            if !content.contains("on_cleanup") {
                violations.push(format!(
                    "[{:?}] Violação da Regra 7: Uso de 'window_event_listener' sem 'on_cleanup'. Listeners globais no window DEVEM desregistrar o handle via on_cleanup(move || handle.remove()) para evitar listeners órfãos após o desmonte do componente.",
                    file_path
                ));
            }
            if content.contains(".get_untracked()") || content.contains(".get()") {
                // Checa se .get() foi usado na closure do listener
                let lines: Vec<&str> = content.lines().collect();
                for (idx, line) in lines.iter().enumerate() {
                    if line.contains("window_event_listener(") {
                        let end = (idx + 12).min(lines.len());
                        let snippet = lines[idx..end].join("\n");
                        if snippet.contains(".get_untracked()") || (snippet.contains(".get()") && !snippet.contains("try_get")) {
                            violations.push(format!(
                                "[{:?}:L{}] Violação da Regra 7: Leitura crua de sinal (.get() / .get_untracked()) dentro de 'window_event_listener'. Use .try_get_untracked().unwrap_or(...) para prevenir panics quando o componente for desmontado.",
                                file_path, idx + 1
                            ));
                        }
                    }
                }
            }
        }

        // Regra 8: Prevenção de Lag de Digitação & Ausência de Focus-Lock em Entradas de Texto da Ficha
        // Inputs de texto em componentes da ficha NÃO devem ligar 'on:input' diretamente com mutações síncronas de 'set_data.update'
        // ou 'event_target_value(&ev)' sem o padrão Focus-Lock (devem usar StableTextInput, StableTextArea, LabelField ou on:blur).
        let is_standalone_view = file_path.to_string_lossy().contains("views") || file_path.to_string_lossy().contains("rooms");
        if !is_standalone_view && !file_path.to_string_lossy().contains("stable_") && !file_path.to_string_lossy().contains("patch_notes_data") {
            let lines: Vec<&str> = content.lines().collect();
            for (idx, line) in lines.iter().enumerate() {
                if line.contains("<input") && !line.contains("type=\"checkbox\"") && !line.contains("type=\"file\"") && !line.contains("type=\"radio\"") {
                    let end = (idx + 15).min(lines.len());
                    let snippet = lines[idx..end].join("\n");
                    if snippet.contains("on:input=") && (snippet.contains("set_data.update") || snippet.contains("update_")) && !snippet.contains("is_focused") && !snippet.contains("is_label_focused") && !snippet.contains("is_modifier_focused") {
                        violations.push(format!(
                            "[{:?}:L{}] Violação da Regra 8: Tag '<input>' de texto da ficha ligando 'on:input' diretamente com mutação do estado global sem Focus-Lock. Use 'StableTextInput', 'StableTextArea', 'LabelField' ou comite no 'on:blur' para evitar lag de digitação.",
                            file_path, idx + 1
                        ));
                    }
                }
            }
        }

        // Regra 9: Prevenção de Perda de Cursor e Destruição de DOM em Listas Dinâmicas (<For />)
        // Em listas de custom_lists, o key do <For /> NÃO deve usar o valor editável (ex: 'update_custom_name' renomeando a lista).
        if content.contains("custom_lists") && content.contains("<For") {
            if content.contains("update_custom_name") && content.contains("list[pos] = new_name") {
                violations.push(format!(
                    "[{:?}] Violação da Regra 9: Lista dinâmica modificando o nome diretamente dentro de custom_lists[pos]. Use IDs únicos imutáveis (ex: uuid) como chave do <For /> e guarde o nome em 'labels' para evitar destruição de DOM e perda de cursor ao digitar.",
                    file_path
                ));
            }
        }

        // Regra 10: Prevenção de Hydration Mismatch em dyn_child.rs (Option::unwrap on None)
        // Múltiplos nós irmãos soltos (fragmentos dinâmicos sem tag pai envolvente) dentro de closures `move ||`
        // causam desalinhamento crítico no cursor de hidratação do Leptos 0.6.
        // Se uma closure reativa retornar mais de um nó DOM irmão (ex: view! { <span ...> <div ...> }),
        // eles DEVEM ser envolvidos em uma única tag container estável (ex: <div class="...">) ou controlados por class:hidden.
        if content.contains("view! {") {
            let mut search_idx = 0;
            while let Some(rel_idx) = content[search_idx..].find("view! {") {
                let view_start = search_idx + rel_idx + 7;
                search_idx = view_start;

                // Checa se este view! está genuinamente dentro de uma closure dinâmica `move ||` ou `move |_|` ativa
                let is_dynamic_closure = if let Some(closure_pos) = content[..view_start].rfind("move |") {
                    let between = &content[closure_pos..view_start];
                    if between.contains(';') {
                        false
                    } else {
                        let open_braces = between.chars().filter(|&c| c == '{').count();
                        let close_braces = between.chars().filter(|&c| c == '}').count();
                        open_braces > close_braces
                    }
                } else {
                    false
                };

                if is_dynamic_closure {
                    // Encontra o fechamento exato das chaves do view! { ... }
                    let mut brace_depth = 1;
                    let mut view_end = None;
                    for (offset, ch) in content[view_start..].char_indices() {
                        if ch == '{' {
                            brace_depth += 1;
                        } else if ch == '}' {
                            brace_depth -= 1;
                            if brace_depth == 0 {
                                view_end = Some(view_start + offset);
                                break;
                            }
                        }
                    }

                    if let Some(view_end) = view_end {
                        let view_body = &content[view_start..view_end];
                        // Analisa os nós raiz do corpo do view! desconsiderando blocos de código internos { ... }
                        let mut inner_code_depth = 0;
                        let mut element_depth = 0;
                        let mut root_elements_count = 0;
                        let mut first_tag = String::new();
                        let chars: Vec<(usize, char)> = view_body.char_indices().collect();
                        let mut i = 0;

                        while i < chars.len() {
                            let (byte_pos, ch) = chars[i];
                            if ch == '{' {
                                inner_code_depth += 1;
                            } else if ch == '}' {
                                inner_code_depth = (inner_code_depth - 1).max(0);
                            } else if inner_code_depth == 0 && ch == '<' {
                                let rest = &view_body[byte_pos..];
                                if !rest.starts_with("<!--") {
                                    if rest.starts_with("</") {
                                        element_depth = (element_depth - 1).max(0);
                                    } else if let Some(tag_end) = rest.find('>') {
                                        let tag_str = &rest[..=tag_end];
                                        // Garante que é uma tag HTML/RSX válida e não operador matemático
                                        let first_char_after_bracket = rest.chars().nth(1).unwrap_or(' ');
                                        if first_char_after_bracket.is_alphabetic() {
                                            let is_void_tag = tag_str.starts_with("<input") || tag_str.starts_with("<img") || tag_str.starts_with("<br") || tag_str.starts_with("<hr");
                                            let is_self_closing = tag_str.ends_with("/>") || is_void_tag;

                                            if element_depth == 0 {
                                                root_elements_count += 1;
                                                if root_elements_count == 1 {
                                                    first_tag = tag_str.split_whitespace().next().unwrap_or("<tag>").to_string();
                                                }
                                            }
                                            if !is_self_closing {
                                                element_depth += 1;
                                            }
                                        }
                                    }
                                }
                            }
                            i += 1;
                        }

                        if root_elements_count > 1 {
                            let line_num = content[..view_start].lines().count();
                            violations.push(format!(
                                "[{:?}:L{}] Violação da Regra 10: Múltiplos nós irmãos soltos ({} nós raiz detectados, iniciando em '{}') retornados dentro de uma closure dinâmica view! sem container pai único envolvente. \
                                 Isso causa descolamento do cursor de hidratação (Option::unwrap on None em dyn_child.rs). Envolva-os em um container único (<div class=\"...\">) ou use class:hidden.",
                                file_path, line_num, root_elements_count, first_tag
                            ));
                        }
                    }
                }
            }
        }

        // Regra 11: Prevenção de avisos 'You are setting the NodeRef ..., which has already been filled'
        // Um 'create_node_ref' declarado no escopo do componente NÃO deve ter seu identificador
        // associado como 'node_ref=...' dentro de uma closure dinâmica ('move ||' ou 'move |_|').
        // Elementos dentro de closures dinâmicas são recriados ao longo do ciclo de vida, o que faz com
        // que o Leptos tente preencher novamente o mesmo NodeRef que já contém valor, emitindo avisos no console.
        if content.contains("create_node_ref") && content.contains("node_ref=") {
            let mut node_refs = Vec::new();
            for line in content.lines() {
                if line.contains("create_node_ref") && line.contains("let ") {
                    if let Some(var_name) = line.split("let ").nth(1).and_then(|s| s.split('=').next()).map(|s| s.trim()) {
                        node_refs.push(var_name.to_string());
                    }
                }
            }

            for nr in &node_refs {
                let target_ref = format!("node_ref={}", nr);
                let mut search_idx = 0;
                while let Some(rel_idx) = content[search_idx..].find(&target_ref) {
                    let ref_pos = search_idx + rel_idx;
                    search_idx = ref_pos + target_ref.len();

                    if let Some(view_pos) = content[..ref_pos].rfind("view! {") {
                        let inner_view = &content[view_pos..ref_pos];
                        if let Some(closure_rel) = inner_view.rfind("move |") {
                            let after_closure = &inner_view[closure_rel..];
                            let open_braces = after_closure.chars().filter(|&c| c == '{').count();
                            let close_braces = after_closure.chars().filter(|&c| c == '}').count();
                            if open_braces > close_braces {
                                let line_num = content[..ref_pos].lines().count();
                                violations.push(format!(
                                    "[{:?}:L{}] Violação da Regra 11: 'node_ref={}' utilizado dentro de uma closure dinâmica 'move ||'. \
                                     Isso causa o aviso 'You are setting the NodeRef defined at ..., which has already been filled' \
                                     quando a closure for reexecutada. Use 'prop:value' / Focus-Lock reativo ou mantenha o elemento em DOM estável.",
                                    file_path, line_num, nr
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    if !violations.is_empty() {
        panic!(
            "\n=======================================================\n\
             VIOLAÇÕES DE ARQUITETURA REATIVA ENCONTRADAS:\n\
             {}\n\
             =======================================================\n",
            violations.join("\n\n")
        );
    }
}

#[test]
fn test_ssr_render_character_sheet_page1_with_supernatural_traits() {
    use leptos::*;
    use mta_sheet::state::{CharacterData, DotOrigin};
    use mta_sheet::components::mta_sheet::page1::attributes::Attributes;
    use mta_sheet::components::mta_sheet::page1::abilities::Abilities;

    let html = leptos::ssr::render_to_string(|| {
        let mut char_data = CharacterData::new("test_sheet".to_string(), "Mago Arcano".to_string());
        char_data.toggle_attribute_supernatural("Força");
        char_data.set_attribute_with_origin("Força", Some(6), None, DotOrigin::Experience);
        char_data.toggle_attribute_supernatural("Prontidão");

        let (data, set_data) = create_signal(char_data);
        provide_context(data);
        provide_context(set_data);

        view! {
            <div class="sheet-container">
                <Attributes />
                <Abilities />
            </div>
        }
    });

    assert!(!html.is_empty(), "HTML da página 1 não deve estar vazio");
    assert!(html.contains("supernatural-slot"), "HTML deve conter o container estável do 6º ponto");
    assert!(html.contains("dot-supernatural"), "HTML deve conter a bolinha em formato de losango");
    assert!(html.contains("supernatural-separator"), "HTML deve conter o separador vertical |");
    assert!(html.contains("label-context-menu"), "HTML deve conter o menu de contexto estável");
}

#[test]
fn test_ssr_render_gods_and_monsters_with_supernatural_traits() {
    use leptos::*;
    use mta_sheet::state::{CharacterData, DotOrigin};
    use mta_sheet::components::gods_and_monsters::attributes::GodsAndMonstersAttributes;
    use mta_sheet::components::gods_and_monsters::abilities::GodsAndMonstersAbilities;

    let html = leptos::ssr::render_to_string(|| {
        let mut char_data = CharacterData::new("test_bygone".to_string(), "Bygone Dragon".to_string());
        char_data.toggle_attribute_supernatural("Strength");
        char_data.set_attribute_with_origin("Strength", Some(6), None, DotOrigin::Experience);
        char_data.toggle_attribute_supernatural("Brawl");

        let (data, set_data) = create_signal(char_data);
        provide_context(data);
        provide_context(set_data);

        view! {
            <div class="gods-sheet-container">
                <GodsAndMonstersAttributes />
                <GodsAndMonstersAbilities />
            </div>
        }
    });

    assert!(!html.is_empty(), "HTML de Gods & Monsters não deve estar vazio");
    assert!(html.contains("supernatural-slot"), "HTML deve conter o slot sobrenatural");
    assert!(html.contains("dot-supernatural"), "HTML deve conter a bolinha em formato de losango");
    assert!(html.contains("supernatural-separator"), "HTML deve conter o separador vertical |");
    assert!(html.contains("label-context-menu"), "HTML deve conter o menu de contexto estável");
}

#[test]
fn test_isomorphic_auth_and_hydration_safety() {
    // 1. Validar integridade isomórfica em src/lib.rs
    let lib_rs = fs::read_to_string("src/lib.rs").expect("src/lib.rs deve existir");
    
    assert!(
        lib_rs.contains("create_local_resource(|| (), |_| async move { crate::auth::get_current_user"),
        "src/lib.rs DEVE usar 'create_local_resource' para autenticação global para garantir que o frame zero de hidratação CSR seja 100% idêntico ao SSR sem mutações assíncronas concorrentes."
    );
    assert!(
        lib_rs.contains("Signal::derive"),
        "src/lib.rs deve derivar o sinal de usuário diretamente via Signal::derive para sincronia de estado sem mutações manuais."
    );
    assert!(
        !lib_rs.contains("set_user.set("),
        "src/lib.rs não deve conter 'set_user.set' dentro de create_effect durante o mount, pois isso colide com o cursor de hidratação do WASM."
    );

    // 2. Validar que login/logout utilizam redirecionamento limpo
    let auth_page = fs::read_to_string("src/components/views/auth_page.rs").expect("auth_page.rs deve existir");
    assert!(
        auth_page.contains("window.location().set_href(\"/\")"),
        "auth_page.rs deve usar window.location().set_href('/') para reinicializar o ciclo SSR de forma limpa após login/cadastro."
    );

    let navbar = fs::read_to_string("src/components/common/navbar.rs").expect("navbar.rs deve existir");
    assert!(
        navbar.contains("window.location().set_href(\"/\")"),
        "navbar.rs deve usar window.location().set_href('/') no logout para limpar a árvore de hidratação."
    );
}

#[test]
fn test_no_nested_suspense_in_dynamic_views() {
    let components_dir = Path::new("src/components");
    let rs_files = get_all_rs_files(components_dir);

    for file_path in &rs_files {
        let content = fs::read_to_string(file_path).expect("Failed to read file");
        // Verifica se há <Suspense dentro de closures dinâmicas (move || match ...)
        if content.contains("move || match") && content.contains("<Suspense") {
            // Se houver, verifica se não é um falso positivo
            let lines: Vec<&str> = content.lines().collect();
            for (idx, line) in lines.iter().enumerate() {
                if line.contains("<Suspense") {
                    // Checa se as linhas anteriores recentes tinham 'move || match'
                    let start = idx.saturating_sub(10);
                    let snippet = lines[start..idx].join("\n");
                    assert!(
                        !snippet.contains("move || match"),
                        "[{:?}:L{}] Anti-pattern detectado: <Suspense> instanciado dentro de uma closure dinâmica 'move || match'. \
                         Substitua por 'create_local_resource' com matching determinístico direto.",
                        file_path, idx + 1
                    );
                }
            }
        }
    }
}

#[test]
fn test_wasm_bindgen_strict_version_pinning() {
    let cargo_toml = fs::read_to_string("Cargo.toml").expect("Cargo.toml deve existir");
    
    // Procura por wasm-bindgen = "=0.2.93" ou qualquer versão travada com sinal de igual '='
    let has_pinned_wasm_bindgen = cargo_toml.lines().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with("wasm-bindgen") && trimmed.contains("\"=")
    });

    assert!(
        has_pinned_wasm_bindgen,
        "Cargo.toml DEVE travar estritamente a versão do wasm-bindgen com '=' (ex: wasm-bindgen = \"=0.2.93\") \
         para evitar divergência de schema entre o cargo-leptos no Windows e no Docker."
    );
}

#[test]
fn test_ssr_render_app_and_home() {
    use leptos_router::{RouterIntegrationContext, ServerIntegration};
    use std::rc::Rc;
    let app_html = leptos::ssr::render_to_string(|| {
        leptos::provide_context(RouterIntegrationContext(Rc::new(ServerIntegration {
            path: "http://localhost:3000/".to_string(),
        })));
        leptos::view! { <mta_sheet::App /> }
    });
    println!("=== RENDERED APP SSR ===\n{}\n========================", app_html);
    assert!(!app_html.is_empty());
}

#[test]
fn test_sheet_loading_guard_prevents_owner_disposed_panic() {
    let sheet_view_rs = fs::read_to_string("src/components/views/character_sheet.rs")
        .expect("character_sheet.rs deve existir");

    // Garante que o switch da ficha verifica `if !is_loaded.get()` antes de renderizar
    assert!(
        sheet_view_rs.contains("if !is_loaded.get()"),
        "character_sheet.rs DEVE checar `if !is_loaded.get()` antes de instanciar a árvore de componentes da ficha \
         para impedir que a ficha padrão de Mago seja criada e descartada imediatamente na resolução de Gods & Monsters, \
         o que causaria pânico de 'OwnerDisposed'."
    );
}

#[test]
fn test_no_signal_storms_in_stable_inputs() {
    let stable_rs = fs::read_to_string("src/components/common/stable_textarea.rs")
        .expect("stable_textarea.rs deve existir");

    assert!(
        !stable_rs.contains("create_rw_signal"),
        "stable_textarea.rs NÃO deve usar 'create_rw_signal' para gerenciar foco e sincronização interna. \
         Use referências locais não reativas (Rc<Cell> e Rc<RefCell>) para evitar cascatas de re-render e 'OwnerDisposed'."
    );
}

