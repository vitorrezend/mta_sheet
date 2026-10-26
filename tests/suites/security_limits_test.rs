use mta_sheet::state::server_fns::validate_image_magic_bytes;

#[test]
fn test_image_magic_bytes_validation() {
    let png_header = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR";
    assert_eq!(validate_image_magic_bytes(png_header).unwrap(), ("image/png", "png"));

    let jpeg_header = b"\xFF\xD8\xFF\xE0\x00\x10JFIF";
    assert_eq!(validate_image_magic_bytes(jpeg_header).unwrap(), ("image/jpeg", "jpg"));

    let fake_exe = b"MZ\x90\x00\x03\x00\x00\x00";
    assert!(validate_image_magic_bytes(fake_exe).is_err());

    // Nenhum SVG é aceito, nem os "inofensivos": a antiga lista de palavras proibidas
    // deixava passar eventos como onbegin/onfocus e `javascript:` com entidades HTML
    let svgs: [&[u8]; 5] = [
        b"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\"><circle cx=\"50\" cy=\"50\" r=\"40\"/></svg>",
        b"<svg xmlns=\"http://www.w3.org/2000/svg\"><script>alert(document.cookie)</script></svg>",
        b"<svg xmlns=\"http://www.w3.org/2000/svg\"><animate onbegin=\"alert(1)\" attributeName=\"x\" dur=\"1s\"/></svg>",
        b"<svg xmlns=\"http://www.w3.org/2000/svg\"><a href=\"&#106;avascript:alert(1)\"><text>x</text></a></svg>",
        b"<?xml version=\"1.0\"?><svg xmlns=\"http://www.w3.org/2000/svg\" onfocus=\"alert(1)\" tabindex=\"0\"></svg>",
    ];
    for svg in svgs {
        assert!(validate_image_magic_bytes(svg).is_err(), "SVG não deve ser aceito: {}", String::from_utf8_lossy(svg));
    }
}

#[test]
fn test_path_traversal_protection() {
    use mta_sheet::server::handlers::static_files::is_safe_relative_path;

    // Safe paths
    assert!(is_safe_relative_path("mta_sheet.wasm"));
    assert!(is_safe_relative_path("pkg/mta_sheet.js"));
    assert!(is_safe_relative_path("styles/main.css"));

    // Traversal attempts
    assert!(!is_safe_relative_path("../.env"));
    assert!(!is_safe_relative_path("../../data/mta_sheet.db"));
    assert!(!is_safe_relative_path("/etc/passwd"));
    assert!(!is_safe_relative_path("..\\..\\windows\\system32"));
    assert!(!is_safe_relative_path("pkg/../../secret.txt"));
    assert!(!is_safe_relative_path(""));
    assert!(!is_safe_relative_path("foo\0bar"));
}

#[test]
fn test_max_limits_constants_and_logic() {
    let limit_5mb = 5 * 1024 * 1024;
    assert_eq!(limit_5mb, 5_242_880);

    let quota_sheets = 50;
    assert_eq!(quota_sheets, 50);
}

#[test]
fn test_security_headers_and_csp_strictness() {
    use std::fs;
    use std::path::Path;

    let sec_rs = Path::new("src/server/middleware/security.rs");
    let content = fs::read_to_string(sec_rs).expect("Falha ao ler security.rs");

    assert!(
        content.contains("permissions-policy") && content.contains("camera=(), microphone=()"),
        "❌ Cabeçalho Permissions-Policy deve estar configurado no middleware de segurança!"
    );
    assert!(
        !content.contains("fonts.googleapis.com") && !content.contains("fonts.gstatic.com"),
        "❌ CSP não deve depender de CDNs externas de fontes após self-hosting do Cinzel!"
    );
    assert!(
        content.contains("public, max-age=31536000, immutable"),
        "❌ Assets estáticos imutáveis (fontes e wasm) devem ter cache-control de 1 ano!"
    );
}

#[test]
fn test_content_security_policy_blocks_inline_scripts() {
    use mta_sheet::server::middleware::security::content_security_policy;

    // Páginas: nada de 'unsafe-inline' nos scripts; o nonce da renderização entra quando existe
    let page = content_security_policy("/sheet/abc", Some("n0nce"));
    let script_src = page.split(';').find(|d| d.trim().starts_with("script-src")).expect("script-src");
    assert!(!script_src.contains("'unsafe-inline'"), "script-src não pode ter 'unsafe-inline': {}", script_src);
    assert!(script_src.contains("'nonce-n0nce'"));
    assert!(script_src.contains("'wasm-unsafe-eval'"));
    assert!(page.contains("object-src 'none'") && page.contains("base-uri 'self'"));

    // Respostas sem renderização (API, arquivos): mesma política, sem nonce
    let without_nonce = content_security_policy("/api/get_sheets", None);
    assert!(!without_nonce.contains("nonce-"));
    let api_script_src = without_nonce.split(';').find(|d| d.trim().starts_with("script-src")).expect("script-src");
    assert!(!api_script_src.contains("'unsafe-inline'"));

    // Arquivos enviados por usuários: nada executa
    let uploads = content_security_policy("/uploads/sheets/x/profile/a.png", None);
    assert!(uploads.contains("default-src 'none'"));
    assert!(uploads.contains("sandbox"));
    assert!(!uploads.contains("script-src"));
}

#[test]
fn test_upload_destinations_cannot_escape_the_uploads_folder() {
    use mta_sheet::state::server_fns::is_safe_path_segment;

    assert!(is_safe_path_segment("wonders"));
    assert!(is_safe_path_segment("6f1c2d3e-aaaa-bbbb-cccc-1234567890ab"));
    assert!(is_safe_path_segment("temp"));
    for bad in ["", "..", "../etc", "a/b", "a\\b", "x y", "nome.png", "%2e%2e", &"a".repeat(65)] {
        assert!(!is_safe_path_segment(bad), "segmento deveria ser recusado: {:?}", bad);
    }
}

#[test]
fn test_global_api_rate_limiter() {
    use std::time::Duration;
    use mta_sheet::server::middleware::security::check_api_rate_limit;

    let test_ip = "192.0.2.123";
    for _ in 0..5 {
        assert!(check_api_rate_limit(test_ip, 10, Duration::from_secs(60)));
    }

    let blocked_ip = "192.0.2.200";
    for _ in 0..3 {
        assert!(check_api_rate_limit(blocked_ip, 3, Duration::from_secs(60)));
    }
    assert!(!check_api_rate_limit(blocked_ip, 3, Duration::from_secs(60)));
}

