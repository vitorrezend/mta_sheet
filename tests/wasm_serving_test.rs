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
async fn test_auth_form_has_no_raw_action_or_method() {
    let auth_page = std::fs::read_to_string("src/components/views/auth_page.rs")
        .expect("auth_page.rs deve existir");

    assert!(
        !auth_page.contains("action="),
        "auth_page.rs NÃO deve conter 'action=' na tag form para impedir navegação HTTP nativa para fora da SPA"
    );
    assert!(
        !auth_page.contains("method=\"POST\""),
        "auth_page.rs NÃO deve conter 'method=\"POST\"' na tag form"
    );
    assert!(
        auth_page.contains("on:submit=on_submit"),
        "auth_page.rs deve utilizar 'on:submit=on_submit' reativo do Leptos com ev.prevent_default()"
    );
}
