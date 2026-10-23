fn main() {
    // Migrações são embutidas por `sqlx::migrate!`: recompila quando mudarem
    println!("cargo:rerun-if-changed=migrations");
    println!("cargo:rerun-if-changed=build.rs");
    // Reexecuta se os assets do front sumirem (para recriar os stubs de teste em debug)
    println!("cargo:rerun-if-changed=target/site/pkg/mta_sheet.js");
    println!("cargo:rerun-if-changed=target/site/pkg/mta_sheet.wasm");

    // Garante a existência dos diretórios de assets esperados pelo rust-embed
    let _ = std::fs::create_dir_all("target/site/pkg");
    let _ = std::fs::create_dir_all("styles");

    // Stubs só em builds de desenvolvimento/teste, para os testes rodarem antes do
    // `cargo leptos build`. Em release o servidor EMBUTE target/site no binário: um stub
    // aqui seria publicado no lugar da WebAssembly real (o cargo-leptos compila front e
    // servidor em paralelo), e o site abriria sem interatividade em qualquer navegador.
    if std::env::var("PROFILE").as_deref() != Ok("debug") {
        return;
    }
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
