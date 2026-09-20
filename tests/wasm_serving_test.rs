
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
            js_content.contains("__wbg_init") || js_content.contains("initSync"),
            "mta_sheet.js deve exportar a função de inicialização (__wbg_init ou initSync)"
        );
        assert!(
            js_content.contains("WebAssembly.instantiate") || js_content.contains("WebAssembly.instantiateStreaming"),
            "mta_sheet.js deve conter chamadas para WebAssembly.instantiate"
        );
    }
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_localhost_cache_control_bypass() {
    use mta_sheet::server::handlers::{get_cache_control_static, get_cache_control_css, is_local_request};

    let req_local = axum::extract::Request::builder()
        .header(http::header::HOST, "localhost:3000")
        .body(axum::body::Body::empty())
        .unwrap();
    assert!(is_local_request(&req_local));
    assert_eq!(get_cache_control_static(&req_local), "no-cache, no-store, must-revalidate");
    assert_eq!(get_cache_control_css(&req_local), "no-cache, no-store, must-revalidate");

    let req_ip = axum::extract::Request::builder()
        .header(http::header::HOST, "127.0.0.1:3000")
        .body(axum::body::Body::empty())
        .unwrap();
    assert!(is_local_request(&req_ip));
    assert_eq!(get_cache_control_static(&req_ip), "no-cache, no-store, must-revalidate");
    assert_eq!(get_cache_control_css(&req_ip), "no-cache, no-store, must-revalidate");
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_embedded_style_assets_integrity() {
    use mta_sheet::server::handlers::StyleAssets;

    // Garante que os arquivos essenciais de estilo estão presentes nos assets embutidos
    let essential_styles = [
        "00-tokens.css",
        "01-variables.css",
        "02-common.css",
        "05-page2-magic-combat.css",
        "12-compendium.css",
        "patch_notes.css",
    ];

    for style_file in &essential_styles {
        assert!(
            StyleAssets::get(style_file).is_some(),
            "O arquivo styles/{} deve estar embutido no StyleAssets",
            style_file
        );
    }
}
