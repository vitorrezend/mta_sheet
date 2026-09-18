
#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_wasm_binary_serving_and_magic_bytes() {
    let wasm_paths = [
        "target/site/pkg/mta_sheet.wasm",
        "target/site/pkg/mta_sheet_bg.wasm",
    ];

    for path in &wasm_paths {
        if let Ok(bytes) = std::fs::read(path) {
            assert!(
                bytes.len() >= 4,
                "Arquivo WASM em {} deve ter pelo menos 4 bytes",
                path
            );
            let magic = &bytes[0..4];
            assert_eq!(
                magic,
                &[0x00, 0x61, 0x73, 0x6d],
                "O cabeçalho do arquivo WASM em {} DEVE ser 00 61 73 6d (\\0asm) e NUNCA texto de erro HTTP",
                path
            );
        }
    }
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_auth_form_has_safe_post_method_and_no_plaintext_get() {
    let auth_page = std::fs::read_to_string("src/components/views/auth_page.rs")
        .expect("auth_page.rs deve existir");

    assert!(
        auth_page.to_lowercase().contains("method=\"post\""),
        "auth_page.rs DEVE conter method=\"post\" para impedir vazamento de senhas em texto claro na URL caso o WASM falhe"
    );
    assert!(
        auth_page.contains("on:submit=on_submit"),
        "auth_page.rs deve utilizar 'on:submit=on_submit' reativo do Leptos com ev.prevent_default()"
    );
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_wasm_js_glue_code_integrity() {
    let js_path = "target/site/pkg/mta_sheet.js";
    if let Ok(js_content) = std::fs::read_to_string(js_path) {
        assert!(
            js_content.contains("__wbg_init"),
            "mta_sheet.js deve exportar a função de inicialização __wbg_init"
        );
        assert!(
            js_content.contains("WebAssembly.instantiate") || js_content.contains("WebAssembly.instantiateStreaming"),
            "mta_sheet.js deve conter chamadas para WebAssembly.instantiate"
        );
    }
}
