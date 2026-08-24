#[cfg(test)]
mod compliance_tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    fn collect_rs_files(dir: &Path, files: &mut Vec<PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
                    if dir_name != "target" && dir_name != ".git" {
                        collect_rs_files(&path, files);
                    }
                } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    files.push(path);
                }
            }
        }
    }

    #[test]
    fn test_no_forbidden_unwrap_in_production_code() {
        let mut files = Vec::new();
        collect_rs_files(Path::new("src"), &mut files);

        let mut violations = Vec::new();

        for file in files {
            let file_str = file.to_string_lossy();
            // Skip dedicated test files
            if file_str.contains("tests.rs") || file_str.contains("compliance") {
                continue;
            }

            if let Ok(content) = fs::read_to_string(&file) {
                let mut in_test_mod = false;
                for (line_idx, line) in content.lines().enumerate() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("#[cfg(test)]") || trimmed.starts_with("mod tests") {
                        in_test_mod = true;
                    }

                    if !in_test_mod && !trimmed.starts_with("//") {
                        // Check for raw unwrap() calls on options/results
                        if trimmed.contains(".unwrap()") && !trimmed.contains("unwrap_or") && !trimmed.contains("unwrap_err") {
                            violations.push(format!(
                                "{}:{}: Linha contém '.unwrap()' proibido. Use 'if let', 'match', '?' ou 'unwrap_or_default()'.",
                                file_str,
                                line_idx + 1
                            ));
                        }
                    }
                }
            }
        }

        assert!(
            violations.is_empty(),
            "\n❌ Violações de conformidade de código detectadas (Risco de Panic):\n{}\n",
            violations.join("\n")
        );
    }

    #[test]
    fn test_async_spawns_use_safe_try_set_in_components() {
        let mut files = Vec::new();
        collect_rs_files(Path::new("src/components"), &mut files);

        let mut violations = Vec::new();

        for file in files {
            let file_str = file.to_string_lossy();
            if let Ok(content) = fs::read_to_string(&file) {
                // If a component spawns an async task, check that it doesn't call naked .set( on signals
                let lines: Vec<&str> = content.lines().collect();
                let mut inside_async_spawn = false;
                let mut brace_depth = 0;

                for (line_idx, line) in lines.iter().enumerate() {
                    let trimmed = line.trim();
                    if trimmed.contains("spawn_local(async move {") || trimmed.contains("spawn_local(async {") {
                        inside_async_spawn = true;
                    }

                    if inside_async_spawn {
                        for c in trimmed.chars() {
                            if c == '{' { brace_depth += 1; }
                            if c == '}' {
                                brace_depth -= 1;
                                if brace_depth <= 0 {
                                    inside_async_spawn = false;
                                }
                            }
                        }

                        // Check for dangerous .set( inside async block (must be .try_set( or signal without context)
                        if trimmed.contains(".set(") && !trimmed.contains("try_set") && !trimmed.starts_with("//") {
                            // Some local signal sets like set_modal(None) inside same tick are okay, but alert if updating global state
                            if trimmed.contains("set_is_dirty.set(") || trimmed.contains("set_data.set(") || trimmed.contains("set_save_status.set(") {
                                violations.push(format!(
                                    "{}:{}: Chamada assíncrona '.set()' detectada em componente. Use '.try_set()' para prevenir panics de OwnerDisposed ao desmontar a página.",
                                    file_str,
                                    line_idx + 1
                                ));
                            }
                        }
                    }
                }
            }
        }

        assert!(
            violations.is_empty(),
            "\n❌ Violações de ciclo de vida reativo Leptos detectadas:\n{}\n",
            violations.join("\n")
        );
    }

    #[test]
    fn test_no_dynamic_closures_wrapping_optional_callbacks() {
        let mut files = Vec::new();
        collect_rs_files(Path::new("src/components"), &mut files);

        let mut violations = Vec::new();

        for file in files {
            let file_str = file.to_string_lossy();
            if let Ok(content) = fs::read_to_string(&file) {
                for (line_idx, line) in content.lines().enumerate() {
                    let trimmed = line.trim();
                    // Detect anti-pattern: {move || if let Some(...) = on_...
                    if (trimmed.contains("move || if let Some(") || trimmed.contains("move || { if let Some("))
                        && (trimmed.contains("on_") || trimmed.contains("callback") || trimmed.contains("cb"))
                    {
                        violations.push(format!(
                            "{}:{}: Anti-padrão detectado: '{}'. Use '.map(|cb| view! {{ ... }})' estático em vez de fecho reativo dinâmico para evitar recriação de escopo e panic de 'could not get stored value'.",
                            file_str,
                            line_idx + 1,
                            trimmed
                        ));
                    }
                }
            }
        }

        assert!(
            violations.is_empty(),
            "
❌ Anti-padrão de renderização de Callbacks detectado:
{}
",
            violations.join("
")
        );
    }


    #[test]
    fn test_no_unwrapped_signal_get_in_view_attributes() {
        let mut files = Vec::new();
        collect_rs_files(Path::new("src/components"), &mut files);

        let mut violations = Vec::new();

        for file in files {
            let file_str = file.to_string_lossy();
            if file_str.contains("tests.rs") || file_str.contains("stable_textarea.rs") || file_str.contains("value_field.rs") {
                continue;
            }

            if let Ok(content) = fs::read_to_string(&file) {
                for (line_idx, line) in content.lines().enumerate() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("//") {
                        continue;
                    }

                    // Detect format!(... .get()) assigned directly to view properties without 'move ||'
                    if (trimmed.contains("title=format!(")
                        || trimmed.contains("class=format!(")
                        || trimmed.contains("placeholder=format!(")
                        || trimmed.contains("href=format!(")
                        || trimmed.contains("src=format!("))
                        && (trimmed.contains(".get()") || trimmed.contains(".with("))
                        && !trimmed.contains("move ||")
                    {
                        violations.push(format!(
                            "{}:{}: Acesso a sinal '.get()' fora de contexto reativo em atributo do view!: '{}'. Use 'move || format!(...)' para garantir rastreamento reativo.",
                            file_str,
                            line_idx + 1,
                            trimmed
                        ));
                    }
                }
            }
        }

        assert!(
            violations.is_empty(),
            "
❌ Acessos a sinais fora de contexto reativo detectados:
{}
",
            violations.join("
")
        );
    }


    #[test]
    fn test_stable_inputs_use_safe_try_set_for_local_signals() {
        let path = Path::new("src/components/common/stable_textarea.rs");
        if let Ok(content) = fs::read_to_string(path) {
            for (line_idx, line) in content.lines().enumerate() {
                let trimmed = line.trim();
                if (trimmed.contains("is_focused.set(") || trimmed.contains("last_synced_value.set("))
                    && !trimmed.starts_with("//")
                {
                    panic!(
                        "Linha {}: Uso de '.set(' em sinal local de input estável. Use '.try_set(' para evitar avisos de Signal Disposed ao desmontar/re-renderizar.",
                        line_idx + 1
                    );
                }
            }
        }
    }

    #[test]
    fn test_modular_style_assets_exist_and_non_empty() {
        let style_css = Path::new("style.css");
        assert!(style_css.exists(), "❌ Arquivo 'style.css' obrigatório não encontrado na raiz!");

        let content = fs::read_to_string(style_css).expect("Falha ao ler style.css");
        assert!(!content.trim().is_empty(), "❌ 'style.css' não pode estar vazio!");

        // Verifica que todos os arquivos referenciados em @import url('/styles/...') existem
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("@import url('/styles/") || trimmed.starts_with("@import url(\"/styles/") {
                let filename = trimmed
                    .replace("@import url('/styles/", "")
                    .replace("@import url(\"/styles/", "")
                    .replace("');", "")
                    .replace("\");", "");
                
                let modular_file = Path::new("styles").join(&filename);
                assert!(
                    modular_file.exists(),
                    "❌ Arquivo de estilo modular referenciado em style.css não encontrado: {:?}",
                    modular_file
                );

                let size = fs::metadata(&modular_file).map(|m| m.len()).unwrap_or(0);
                assert!(size > 0, "❌ Arquivo de estilo {:?} está com 0 bytes!", modular_file);
            }
        }
    }

    #[test]
    fn test_no_top_level_hidden_inputs_causing_hydration_mismatch() {
        let mut files = Vec::new();
        collect_rs_files(Path::new("src/components"), &mut files);

        let mut violations = Vec::new();

        for file in files {
            let file_str = file.to_string_lossy();
            if file_str.contains("tests.rs") || file_str.contains("stable_textarea.rs") || file_str.contains("value_field.rs") {
                continue;
            }

            if let Ok(content) = fs::read_to_string(&file) {
                let mut in_view = false;
                for (line_idx, line) in content.lines().enumerate() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("view! {") {
                        in_view = true;
                        continue;
                    }

                    if in_view {
                        // Detect <input type="file" as direct first child of view! before a semantic container
                        if (trimmed.starts_with(r#"<input type="file""#) || trimmed.starts_with("<input"))
                            && !trimmed.contains("class=")
                        {
                            // Check next non-empty lines to see if it immediately precedes <header> or <div class="home-container">
                            violations.push(format!(
                                "{}:{}: <input> oculto encontrado no topo de bloco view!: '{}'. Inputs de upload/importação devem ficar encapsulados dentro de seu grupo/container local para evitar mismatch de hidratação SSR/CSR.",
                                file_str,
                                line_idx + 1,
                                trimmed
                            ));
                        }
                        in_view = false;
                    }
                }
            }
        }

        assert!(
            violations.is_empty(),
            "
❌ Elementos de input soltos causando risco de mismatch de hidratação:
{}
",
            violations.join("
")
        );
    }

    #[test]
    fn test_event_handlers_in_dynamic_views_use_callback_or_reusable_closures() {
        let mut files = Vec::new();
        collect_rs_files(Path::new("src/components/views"), &mut files);

        let mut violations = Vec::new();

        for file in files {
            let file_str = file.to_string_lossy();
            if file_str.contains("tests.rs") || file_str.contains("stable_textarea.rs") || file_str.contains("value_field.rs") {
                continue;
            }

            if let Ok(content) = fs::read_to_string(&file) {
                for (line_idx, line) in content.lines().enumerate() {
                    let trimmed = line.trim();
                    // Detect moving bare closure variable into on:change/on:click inside {move || ...}
                    if trimmed.starts_with("on:change=on_") && !trimmed.contains("move |") && !trimmed.contains(".call(") {
                        violations.push(format!(
                            "{}:{}: Tratador de evento direto '{}' dentro de view dinâmica pode causar erro FnOnce/Fn. Use 'Callback::new' e '.call(ev)'.",
                            file_str,
                            line_idx + 1,
                            trimmed
                        ));
                    }
                }
            }
        }

        assert!(
            violations.is_empty(),
            "
❌ Tratadores de eventos com risco de captura FnOnce em views dinâmicas:
{}
",
            violations.join("
")
        );
    }

    #[test]
    fn test_no_empty_leptos_output_name_configured() {
        let cargo_toml = Path::new("Cargo.toml");
        assert!(cargo_toml.exists(), "Cargo.toml deve existir na raiz!");

        let content = fs::read_to_string(cargo_toml).expect("Falha ao ler Cargo.toml");
        assert!(
            content.contains("output-name = \"mta_sheet\"") || content.contains("output_name = \"mta_sheet\""),
            "❌ 'output-name' deve estar explicitamente configurado como 'mta_sheet' em [package.metadata.leptos] para evitar geração de links vazios (/pkg/.js)."
        );
    }

    #[test]
    fn test_safe_callback_survives_runtime_disposal() {
        use crate::components::common::callback::Callback;
        use std::cell::Cell;
        use std::rc::Rc;

        let result = Rc::new(Cell::new(0));
        let result_clone = result.clone();

        let cb: Callback<i32> = {
            let runtime = leptos::create_runtime();
            let cb = Callback::new(move |val| {
                result_clone.set(val * 2);
            });
            runtime.dispose();
            cb
        };

        // Mesmo apos o descarte completo do runtime reativo do Leptos,
        // o SafeCallback NUNCA entra em panic com 'could not get stored value'.
        cb.call(21);
        assert_eq!(result.get(), 42);
    }
}
