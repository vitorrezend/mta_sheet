use std::sync::{Arc, Mutex, LazyLock};
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct ApiRateLimiter {
    attempts: HashMap<String, Vec<Instant>>,
}

static API_RATE_LIMITER: LazyLock<Arc<Mutex<ApiRateLimiter>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(ApiRateLimiter {
        attempts: HashMap::new(),
    }))
});

pub fn extract_client_ip(headers: &http::HeaderMap) -> String {
    if let Some(forwarded) = headers.get("x-forwarded-for").and_then(|h| h.to_str().ok()) {
        if let Some(first_ip) = forwarded.split(',').next() {
            let trimmed = first_ip.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }
    if let Some(real_ip) = headers.get("x-real-ip").and_then(|h| h.to_str().ok()) {
        let trimmed = real_ip.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    "127.0.0.1".to_string()
}

pub fn check_api_rate_limit(client_ip: &str, max_requests: usize, window: Duration) -> bool {
    let mut limiter = match API_RATE_LIMITER.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    };
    let now = Instant::now();
    let entry = limiter.attempts.entry(client_ip.to_string()).or_default();
    entry.retain(|&time| now.duration_since(time) < window);
    if entry.len() >= max_requests {
        false
    } else {
        entry.push(now);
        true
    }
}

pub async fn security_headers_middleware(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let path = request.uri().path().to_string();
    let has_query = request.uri().query().is_some();
    let is_local = crate::server::is_local_request(&request);
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
        http::HeaderName::from_static("permissions-policy"),
        http::HeaderValue::from_static("camera=(), microphone=(), geolocation=(), interest-cohort=(), payment=()"),
    );
    headers.insert(
        http::header::CONTENT_SECURITY_POLICY,
        http::HeaderValue::from_static(
            "default-src 'self'; script-src 'self' 'wasm-unsafe-eval' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob:; font-src 'self' data:; connect-src 'self' ws: wss:;"
        ),
    );

    if cfg!(debug_assertions) || is_local {
        // Em modo de desenvolvimento ou acesso local (localhost / 127.0.0.1),
        // forçamos o navegador a nunca usar cache de JS/WASM/CSS/HTML
        // para prevenir divergências e travamento de estilos antigos.
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
            || path.starts_with("/fonts/")
            || path == "/style.css"
            || path == "/favicon.ico"
            || path == "/banner_og.jpg"
            || path == "/banner_og.png";
        if is_static_asset {
            if path.starts_with("/fonts/") || path.ends_with(".wasm") || has_query {
                headers.insert(
                    http::header::CACHE_CONTROL,
                    http::HeaderValue::from_static("public, max-age=31536000, immutable"),
                );
            } else if path.ends_with(".css") {
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

        let is_prod = std::env::var("LEPTOS_ENV").map(|e| e.to_lowercase() == "production" || e.to_lowercase() == "prod").unwrap_or(false)
            || std::env::var("ENABLE_HSTS").map(|e| e == "1" || e.to_lowercase() == "true").unwrap_or(false);

        if is_prod {
            headers.insert(
                http::header::STRICT_TRANSPORT_SECURITY,
                http::HeaderValue::from_static("max-age=63072000; includeSubDomains; preload"),
            );
        }
    }

    response
}

/// Middleware de proteção contra ataques CSRF (Cross-Site Request Forgery)
/// baseado na especificação W3C / OWASP de Fetch Metadata (`Sec-Fetch-Site`).
/// Rejeita requisições mutáveis (POST, PUT, DELETE, PATCH) originadas de domínios externos (`cross-site`).
pub async fn csrf_protection_middleware(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    let method = request.method().clone();
    if method == http::Method::POST || method == http::Method::PUT || method == http::Method::DELETE || method == http::Method::PATCH {
        let path = request.uri().path();
        if path.starts_with("/api/") {
            if let Some(sec_fetch_site) = request.headers().get("sec-fetch-site").and_then(|h| h.to_str().ok()) {
                if sec_fetch_site.eq_ignore_ascii_case("cross-site") {
                    log::warn!("🛡️ [SEGURANÇA] Bloqueada requisição cross-site suspeita (CSRF) para: {}", path);
                    return (
                        http::StatusCode::FORBIDDEN,
                        [(http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
                        "Acesso negado: Requisição cross-site bloqueada por política de segurança (CSRF Protection).",
                    ).into_response();
                }
            }
        }
    }

    next.run(request).await
}

/// Middleware de Rate Limiting Global para todos os endpoints sob `/api/*`.
/// Limita a 120 requisições por minuto por IP em produção, prevenindo ataques Layer-7 de negação de serviço (DoS) e scraping abusivo.
pub async fn api_rate_limit_middleware(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    let path = request.uri().path().to_string();
    if path.starts_with("/api/") {
        let is_local = crate::server::is_local_request(&request);
        if !is_local && !cfg!(debug_assertions) {
            let client_ip = extract_client_ip(request.headers());
            if !check_api_rate_limit(&client_ip, 120, Duration::from_secs(60)) {
                log::warn!("🛡️ [RATE LIMIT] IP {} excedeu o limite global de requisições na API: {}", client_ip, path);
                return (
                    http::StatusCode::TOO_MANY_REQUESTS,
                    [
                        (http::header::CONTENT_TYPE, "application/json; charset=utf-8"),
                        (http::header::RETRY_AFTER, "60"),
                    ],
                    r#"{"error":"Taxa de requisições excedida. Por favor, aguarde 60 segundos antes de tentar novamente."}"#,
                ).into_response();
            }
        }
    }

    next.run(request).await
}
