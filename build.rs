fn main() {
    // Garante a existência dos diretórios de assets esperados pelo rust-embed
    let _ = std::fs::create_dir_all("target/site/pkg");
    let _ = std::fs::create_dir_all("styles");

    // Cria stubs de fallback caso o cargo leptos build ainda não tenha sido executado
    let js_path = std::path::Path::new("target/site/pkg/mta_sheet.js");
    if !js_path.exists() {
        let _ = std::fs::write(
            js_path,
            "/* fallback stub for testing */\nfunction __wbg_init() { return WebAssembly.instantiate(); }\nfunction initSync() { return WebAssembly.instantiateStreaming(); }\n",
        );
    }
    let wasm_path = std::path::Path::new("target/site/pkg/mta_sheet.wasm");
    if !wasm_path.exists() {
        let _ = std::fs::write(wasm_path, b"\0asm\x01\0\0\0");
    }
}
