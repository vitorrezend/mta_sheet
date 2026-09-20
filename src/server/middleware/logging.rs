/// Verifica se o caminho corresponde a um asset estático ou rota de telemetria interna que não deve poluir os logs de acesso
pub fn is_ignorable_path(path: &str) -> bool {
    let p = path.to_ascii_lowercase();

    // 1. Evita loop de eco: telemetria de log do cliente enviada via API
    if p.starts_with("/api/record_client_log") {
        return true;
    }

    // 2. Prefixos de diretórios de arquivos estáticos
    if p.starts_with("/styles/")
        || p.starts_with("/pkg/")
        || p.starts_with("/fonts/")
        || p.starts_with("/uploads/")
    {
        return true;
    }

    // 3. Extensões de arquivos estáticos (CSS, JS, WASM, Imagens, Fontes, SourceMaps)
    const STATIC_EXTENSIONS: &[&str] = &[
        ".css", ".js", ".wasm", ".png", ".jpg", ".jpeg", ".webp",
        ".gif", ".ico", ".svg", ".woff", ".woff2", ".ttf", ".eot", ".map",
    ];

    if STATIC_EXTENSIONS.iter().any(|ext| p.ends_with(ext)) {
        return true;
    }

    // 4. Ícones soltos da raiz
    if p == "/favicon.ico" || p == "/favicon.svg" {
        return true;
    }

    false
}

pub async fn access_log_middleware(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let path = request.uri().path().to_string();
    let method = request.method().as_str().to_string();

    let user_agent = request.headers().get(http::header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let sec_fetch_mode = request.headers().get("sec-fetch-mode")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let sec_ch_ua = request.headers().get("sec-ch-ua")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let accept_lang = request.headers().get(http::header::ACCEPT_LANGUAGE)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let ip = request.headers().get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .or_else(|| {
            request.headers().get("x-real-ip")
                .and_then(|h| h.to_str().ok())
        })
        .unwrap_or("127.0.0.1")
        .trim()
        .to_string();

    let start_time = std::time::Instant::now();
    let response = next.run(request).await;
    let duration = start_time.elapsed().as_millis();
    let status = response.status().as_u16();

    // Filtra assets estáticos e ruídos para manter os logs enxutos e focados em tráfego real
    if !is_ignorable_path(&path) {
        let classification = crate::logging::classify_traffic(
            user_agent.as_deref(),
            sec_fetch_mode.as_deref(),
            sec_ch_ua.as_deref(),
            accept_lang.as_deref(),
        );

        crate::logging::server::write_access_log(
            &method,
            &path,
            status,
            duration,
            &ip,
            &classification,
        );
    }

    response
}
