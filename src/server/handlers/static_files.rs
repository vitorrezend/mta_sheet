use axum::response::IntoResponse;

#[derive(rust_embed::RustEmbed)]
#[folder = "target/site/"]
pub struct SiteAssets;

#[derive(rust_embed::RustEmbed)]
#[folder = "styles/"]
pub struct StyleAssets;

pub const EMBEDDED_STYLE_CSS: &str = include_str!("../../../style.css");

pub fn get_cache_control_static() -> &'static str {
    if cfg!(debug_assertions) {
        "no-cache, no-store, must-revalidate"
    } else {
        "public, max-age=86400, stale-while-revalidate=3600"
    }
}

pub fn get_cache_control_css() -> &'static str {
    if cfg!(debug_assertions) {
        "no-cache, no-store, must-revalidate"
    } else {
        "public, max-age=3600, must-revalidate"
    }
}

pub async fn pkg_handler(
    req: axum::extract::Request,
) -> impl IntoResponse {
    let uri_path = req.uri().path().to_string();
    let clean_path = uri_path
        .trim_start_matches('/')
        .trim_start_matches("pkg")
        .trim_start_matches('/')
        .trim()
        .to_string();
    let cache_hdr = get_cache_control_static().to_string();

    let alt_name = if clean_path == "mta_sheet_bg.wasm" {
        "mta_sheet.wasm"
    } else if clean_path == "mta_sheet.wasm" {
        "mta_sheet_bg.wasm"
    } else {
        &clean_path
    };

    let candidate_paths = [
        format!("target/site/pkg/{}", clean_path),
        format!("target/site/pkg/{}", alt_name),
        format!("target/site/{}", clean_path),
        format!("target/front/wasm32-unknown-unknown/debug/{}", clean_path),
        format!("target/front/wasm32-unknown-unknown/debug/{}", alt_name),
        format!("target/wasm32-unknown-unknown/debug/{}", clean_path),
        format!("target/wasm32-unknown-unknown/release/{}", clean_path),
    ];

    for path in candidate_paths {
        if let Ok(bytes) = tokio::fs::read(&path).await {
            let mime = if path.ends_with(".wasm") {
                "application/wasm"
            } else if path.ends_with(".js") {
                "text/javascript"
            } else if path.ends_with(".css") {
                "text/css"
            } else {
                "application/octet-stream"
            };
            return (
                [
                    (http::header::CONTENT_TYPE, mime.to_string()),
                    (http::header::CACHE_CONTROL, cache_hdr),
                ],
                bytes,
            )
                .into_response();
        }
    }

    // 2. Fallback para embutido
    let candidate_keys = [
        format!("pkg/{}", clean_path),
        format!("pkg/{}", alt_name),
        clean_path.clone(),
        alt_name.to_string(),
    ];

    for key in candidate_keys {
        if let Some(file) = SiteAssets::get(&key) {
            let mime = if key.ends_with(".wasm") {
                "application/wasm"
            } else if key.ends_with(".js") {
                "text/javascript"
            } else if key.ends_with(".css") {
                "text/css"
            } else {
                "application/octet-stream"
            };
            return (
                [
                    (http::header::CONTENT_TYPE, mime.to_string()),
                    (http::header::CACHE_CONTROL, cache_hdr),
                ],
                file.data.into_owned(),
            )
                .into_response();
        }
    }

    // Fallback especial para CSS
    if clean_path == "mta_sheet.css" || clean_path == "style.css" {
        let css_cache = get_cache_control_css().to_string();
        if let Ok(css) = tokio::fs::read_to_string("style.css").await {
            return (
                [
                    (http::header::CONTENT_TYPE, "text/css".to_string()),
                    (http::header::CACHE_CONTROL, css_cache),
                ],
                css,
            ).into_response();
        }
    }

    (
        http::StatusCode::NOT_FOUND,
        [(http::header::CONTENT_TYPE, "text/plain; charset=utf-8".to_string())],
        "Arquivo não encontrado".as_bytes().to_vec(),
    )
        .into_response()
}

pub async fn assets_handler(
    req: axum::extract::Request,
) -> impl IntoResponse {
    let uri_path = req.uri().path().to_string();
    let clean_path = uri_path
        .trim_start_matches('/')
        .trim_start_matches("assets")
        .trim_start_matches('/')
        .trim()
        .to_string();
    let key = format!("assets/{}", clean_path);
    let cache_hdr = get_cache_control_static().to_string();

    if let Some(file) = SiteAssets::get(&key) {
        let mime = mime_guess::from_path(&clean_path).first_or_octet_stream().to_string();
        return (
            [
                (http::header::CONTENT_TYPE, mime),
                (http::header::CACHE_CONTROL, cache_hdr),
            ],
            file.data.into_owned(),
        )
            .into_response();
    }

    let disk_path = format!("target/site/assets/{}", clean_path);
    if let Ok(bytes) = tokio::fs::read(&disk_path).await {
        let mime = mime_guess::from_path(&clean_path).first_or_octet_stream().to_string();
        return (
            [
                (http::header::CONTENT_TYPE, mime),
                (http::header::CACHE_CONTROL, cache_hdr),
            ],
            bytes,
        )
            .into_response();
    }

    (http::StatusCode::NOT_FOUND, "Asset não encontrado").into_response()
}

pub async fn styles_handler(
    req: axum::extract::Request,
) -> impl IntoResponse {
    let uri_path = req.uri().path().to_string();
    let clean_path = uri_path
        .trim_start_matches('/')
        .trim_start_matches("styles")
        .trim_start_matches('/')
        .trim()
        .to_string();
    let disk_path = format!("styles/{}", clean_path);
    let cache_hdr = get_cache_control_css().to_string();

    if let Ok(bytes) = tokio::fs::read(&disk_path).await {
        return (
            [
                (http::header::CONTENT_TYPE, "text/css".to_string()),
                (http::header::CACHE_CONTROL, cache_hdr),
            ],
            bytes,
        )
            .into_response();
    }

    if let Some(file) = StyleAssets::get(&clean_path) {
        return (
            [
                (http::header::CONTENT_TYPE, "text/css".to_string()),
                (http::header::CACHE_CONTROL, cache_hdr),
            ],
            file.data.into_owned(),
        )
            .into_response();
    }

    (http::StatusCode::NOT_FOUND, "Estilo não encontrado").into_response()
}

pub async fn style_css_handler() -> impl IntoResponse {
    let cache_hdr = get_cache_control_css().to_string();
    if let Ok(css) = tokio::fs::read_to_string("style.css").await {
        (
            [
                (http::header::CONTENT_TYPE, "text/css".to_string()),
                (http::header::CACHE_CONTROL, cache_hdr),
            ],
            css,
        )
            .into_response()
    } else {
        (
            [
                (http::header::CONTENT_TYPE, "text/css".to_string()),
                (http::header::CACHE_CONTROL, cache_hdr),
            ],
            EMBEDDED_STYLE_CSS.to_string(),
        )
            .into_response()
    }
}
