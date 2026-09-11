pub async fn security_headers_middleware(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let path = request.uri().path().to_string();
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    headers.insert(
        http::header::X_CONTENT_TYPE_OPTIONS,
        http::HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        http::header::X_FRAME_OPTIONS,
        http::HeaderValue::from_static("SAMEORIGIN"),
    );
    headers.insert(
        http::header::REFERRER_POLICY,
        http::HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        http::header::CONTENT_SECURITY_POLICY,
        http::HeaderValue::from_static(
            "default-src 'self'; script-src 'self' 'wasm-unsafe-eval' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob:; font-src 'self' data:; connect-src 'self' ws: wss:;"
        ),
    );

    if cfg!(debug_assertions) {
        // Em modo de desenvolvimento, forçamos o navegador a nunca usar cache de JS/WASM/CSS/HTML
        // para prevenir LinkError de WebAssembly quando o código for recompilado.
        headers.insert(
            http::header::CACHE_CONTROL,
            http::HeaderValue::from_static("no-cache, no-store, must-revalidate, max-age=0"),
        );
        headers.insert(
            http::header::PRAGMA,
            http::HeaderValue::from_static("no-cache"),
        );
    } else {
        let is_static_asset = path.starts_with("/pkg/")
            || path.starts_with("/assets/")
            || path.starts_with("/styles/")
            || path == "/style.css"
            || path == "/favicon.ico";
        if is_static_asset {
            if path.ends_with(".css") {
                headers.insert(
                    http::header::CACHE_CONTROL,
                    http::HeaderValue::from_static("public, max-age=3600, must-revalidate"),
                );
            } else {
                headers.insert(
                    http::header::CACHE_CONTROL,
                    http::HeaderValue::from_static("public, max-age=86400, stale-while-revalidate=3600"),
                );
            }
        } else if !path.starts_with("/api/") {
            headers.insert(
                http::header::CACHE_CONTROL,
                http::HeaderValue::from_static("private, no-cache"),
            );
        }
    }

    response
}
