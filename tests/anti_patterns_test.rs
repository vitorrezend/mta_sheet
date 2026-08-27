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
