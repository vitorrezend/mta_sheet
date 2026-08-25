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
