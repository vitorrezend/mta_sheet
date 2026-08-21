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
}
