use axum::response::IntoResponse;

#[derive(rust_embed::RustEmbed)]
#[folder = "target/site/"]
pub struct SiteAssets;

#[derive(rust_embed::RustEmbed)]
#[folder = "styles/"]
pub struct StyleAssets;

#[derive(rust_embed::RustEmbed)]
#[folder = "fonts/"]
pub struct FontAssets;

pub const EMBEDDED_STYLE_CSS: &str = include_str!("../../../style.css");

pub fn is_local_request(req: &axum::extract::Request) -> bool {
    if let Some(host) = req.headers().get(http::header::HOST) {
        if let Ok(host_str) = host.to_str() {
            return host_str.starts_with("localhost")
                || host_str.starts_with("127.0.0.1")
                || host_str.starts_with("[::1]");
        }
    }
    false
}

pub fn get_cache_control_static(req: &axum::extract::Request) -> &'static str {
    if cfg!(debug_assertions) || is_local_request(req) {
        "no-cache, no-store, must-revalidate"
    } else {
        "public, max-age=86400, stale-while-revalidate=3600"
    }
}

pub fn get_cache_control_css(req: &axum::extract::Request) -> &'static str {
    if cfg!(debug_assertions) || is_local_request(req) {
        "no-cache, no-store, must-revalidate"
    } else {
        "public, max-age=3600, must-revalidate"
    }
}

pub fn is_safe_relative_path(path: &str) -> bool {
    if path.is_empty()
        || path.contains("..")
        || path.contains('\\')
        || path.starts_with('/')
        || path.contains('\0')
    {
        return false;
    }
    path.chars().all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '/' | '@'))
}

pub async fn safe_disk_read(base_dir: &str, relative_path: &str) -> Option<Vec<u8>> {
    if !is_safe_relative_path(relative_path) {
        return None;
    }
    let base = tokio::fs::canonicalize(base_dir).await.ok()?;
    let candidate = base.join(relative_path);
    let target = tokio::fs::canonicalize(&candidate).await.ok()?;
    if target.starts_with(&base) && target.is_file() {
        tokio::fs::read(&target).await.ok()
    } else {
        None
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

    if !is_safe_relative_path(&clean_path) {
        return (
            http::StatusCode::BAD_REQUEST,
            [(http::header::CONTENT_TYPE, "text/plain; charset=utf-8".to_string())],
            "Caminho de arquivo inválido ou inseguro".as_bytes().to_vec(),
        )
            .into_response();
    }
    let cache_hdr = get_cache_control_static(&req).to_string();

    let alt_name = if clean_path == "mta_sheet_bg.wasm" {
        "mta_sheet.wasm"
    } else if clean_path == "mta_sheet.wasm" {
        "mta_sheet_bg.wasm"
    } else {
        &clean_path
    };

    let candidate_keys = [
        format!("pkg/{}", clean_path),
        format!("pkg/{}", alt_name),
        clean_path.clone(),
        alt_name.to_string(),
    ];

    let mut candidate_paths = vec![
        format!("target/site/pkg/{}", clean_path),
        format!("target/site/pkg/{}", alt_name),
        format!("target/site/{}", clean_path),
        format!("site/pkg/{}", clean_path),
        format!("site/pkg/{}", alt_name),
        format!("pkg/{}", clean_path),
        format!("target/front/wasm32-unknown-unknown/debug/{}", clean_path),
        format!("target/front/wasm32-unknown-unknown/debug/{}", alt_name),
        format!("target/wasm32-unknown-unknown/debug/{}", clean_path),
        format!("target/wasm32-unknown-unknown/release/{}", clean_path),
        format!("../../target/site/pkg/{}", clean_path),
        format!("../../target/site/pkg/{}", alt_name),
    ];

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            candidate_paths.push(format!("{}/target/site/pkg/{}", exe_dir.display(), clean_path));
            candidate_paths.push(format!("{}/../../target/site/pkg/{}", exe_dir.display(), clean_path));
            candidate_paths.push(format!("{}/site/pkg/{}", exe_dir.display(), clean_path));
            candidate_paths.push(format!("{}/pkg/{}", exe_dir.display(), clean_path));
        }
    }

    // Em modo RELEASE (ou se houver assets embutidos prioritários),
    // consultar PRIMEIRO o SiteAssets (embutido no binário) para blindar contra arquivos fantasmas do dev
    #[cfg(not(debug_assertions))]
    {
        for key in &candidate_keys {
            if let Some(file) = SiteAssets::get(key) {
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

        // Fallback para disco em release apenas se não estiver no binário embutido
        for path in &candidate_paths {
            if let Ok(bytes) = tokio::fs::read(path).await {
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
    }

    // Em modo DEBUG, ler primeiro do disco para permitir hot-reload do cargo-leptos watch
    #[cfg(debug_assertions)]
    {
        for path in &candidate_paths {
            if let Ok(bytes) = tokio::fs::read(path).await {
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

        for key in &candidate_keys {
            if let Some(file) = SiteAssets::get(key) {
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
    }

    // Fallback especial para CSS
    if clean_path == "mta_sheet.css" || clean_path == "style.css" {
        let css_cache = get_cache_control_css(&req).to_string();
        if let Ok(css) = tokio::fs::read_to_string("style.css").await {
            return (
                [
                    (http::header::CONTENT_TYPE, "text/css".to_string()),
                    (http::header::CACHE_CONTROL, css_cache),
                ],
                css,
            ).into_response();
        }
        return (
            [
                (http::header::CONTENT_TYPE, "text/css".to_string()),
                (http::header::CACHE_CONTROL, css_cache),
            ],
            EMBEDDED_STYLE_CSS.to_string(),
        ).into_response();
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

    if !is_safe_relative_path(&clean_path) {
        return (
            http::StatusCode::BAD_REQUEST,
            [(http::header::CONTENT_TYPE, "text/plain; charset=utf-8".to_string())],
            "Caminho de asset inválido".as_bytes().to_vec(),
        )
            .into_response();
    }

    let key = format!("assets/{}", clean_path);
    let cache_hdr = get_cache_control_static(&req).to_string();

    #[cfg(not(debug_assertions))]
    {
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

        if let Some(bytes) = safe_disk_read("target/site/assets", &clean_path).await {
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
    }

    #[cfg(debug_assertions)]
    {
        if let Some(bytes) = safe_disk_read("target/site/assets", &clean_path).await {
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

    if !is_safe_relative_path(&clean_path) {
        return (
            http::StatusCode::BAD_REQUEST,
            [(http::header::CONTENT_TYPE, "text/plain; charset=utf-8".to_string())],
            "Caminho de estilo inválido".as_bytes().to_vec(),
        )
            .into_response();
    }

    let cache_hdr = get_cache_control_css(&req).to_string();

    #[cfg(not(debug_assertions))]
    {
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

        if let Some(bytes) = safe_disk_read("styles", &clean_path).await {
            return (
                [
                    (http::header::CONTENT_TYPE, "text/css".to_string()),
                    (http::header::CACHE_CONTROL, cache_hdr),
                ],
                bytes,
            )
                .into_response();
        }
    }

    #[cfg(debug_assertions)]
    {
        if let Some(bytes) = safe_disk_read("styles", &clean_path).await {
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
    }

    (http::StatusCode::NOT_FOUND, "Estilo não encontrado").into_response()
}

pub async fn style_css_handler(req: axum::extract::Request) -> impl IntoResponse {
    let cache_hdr = get_cache_control_css(&req).to_string();

    #[cfg(not(debug_assertions))]
    {
        (
            [
                (http::header::CONTENT_TYPE, "text/css".to_string()),
                (http::header::CACHE_CONTROL, cache_hdr),
            ],
            EMBEDDED_STYLE_CSS.to_string(),
        )
            .into_response()
    }

    #[cfg(debug_assertions)]
    {
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
}

pub async fn fonts_handler(
    req: axum::extract::Request,
) -> impl IntoResponse {
    let uri_path = req.uri().path().to_string();
    let clean_path = uri_path
        .trim_start_matches('/')
        .trim_start_matches("fonts")
        .trim_start_matches('/')
        .trim()
        .to_string();

    if !is_safe_relative_path(&clean_path) {
        return (
            http::StatusCode::BAD_REQUEST,
            [(http::header::CONTENT_TYPE, "text/plain; charset=utf-8".to_string())],
            "Caminho de fonte inválido".as_bytes().to_vec(),
        )
            .into_response();
    }

    let key = format!("fonts/{}", clean_path);
    let cache_hdr = if cfg!(debug_assertions) || is_local_request(&req) {
        "no-cache, no-store, must-revalidate".to_string()
    } else {
        "public, max-age=31536000, immutable".to_string()
    };

    let mime = if clean_path.ends_with(".woff2") {
        "font/woff2"
    } else if clean_path.ends_with(".woff") {
        "font/woff"
    } else if clean_path.ends_with(".ttf") {
        "font/ttf"
    } else {
        "application/octet-stream"
    };

    #[cfg(not(debug_assertions))]
    {
        if let Some(file) = FontAssets::get(&clean_path) {
            return (
                [
                    (http::header::CONTENT_TYPE, mime.to_string()),
                    (http::header::CACHE_CONTROL, cache_hdr),
                ],
                file.data.into_owned(),
            )
                .into_response();
        }

        if let Some(file) = SiteAssets::get(&key) {
            return (
                [
                    (http::header::CONTENT_TYPE, mime.to_string()),
                    (http::header::CACHE_CONTROL, cache_hdr),
                ],
                file.data.into_owned(),
            )
                .into_response();
        }

        if let Some(bytes) = safe_disk_read("target/site/fonts", &clean_path).await {
            return (
                [
                    (http::header::CONTENT_TYPE, mime.to_string()),
                    (http::header::CACHE_CONTROL, cache_hdr),
                ],
                bytes,
            )
                .into_response();
        }

        if let Some(bytes) = safe_disk_read("fonts", &clean_path).await {
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

    #[cfg(debug_assertions)]
    {
        if let Some(bytes) = safe_disk_read("target/site/fonts", &clean_path).await {
            return (
                [
                    (http::header::CONTENT_TYPE, mime.to_string()),
                    (http::header::CACHE_CONTROL, cache_hdr),
                ],
                bytes,
            )
                .into_response();
        }

        if let Some(bytes) = safe_disk_read("fonts", &clean_path).await {
            return (
                [
                    (http::header::CONTENT_TYPE, mime.to_string()),
                    (http::header::CACHE_CONTROL, cache_hdr),
                ],
                bytes,
            )
                .into_response();
        }

        if let Some(file) = SiteAssets::get(&key) {
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

    (http::StatusCode::NOT_FOUND, "Fonte não encontrada").into_response()
}
