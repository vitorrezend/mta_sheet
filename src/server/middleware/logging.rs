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

    // Filtra assets muito triviais para não poluir os logs (ex: favicon, fonts)
    let is_trivial_asset = path.ends_with(".ico") || path.ends_with(".woff2") || path.ends_with(".woff");
    if !is_trivial_asset {
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
