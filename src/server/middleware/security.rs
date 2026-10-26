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

pub fn check_api_rate_limit(client_ip: &str, max_requests: usize, window: Duration) -> bool {
    let mut limiter = match API_RATE_LIMITER.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    };
    let now = Instant::now();
    prune_stale_entries(&mut limiter.attempts, now, window);
    let entry = limiter.attempts.entry(client_ip.to_string()).or_default();
    entry.retain(|&time| now.duration_since(time) < window);
    if entry.len() >= max_requests {
        false
    } else {
        entry.push(now);
        true
    }
}

static ACTION_RATE_LIMITER: LazyLock<Mutex<HashMap<String, Vec<Instant>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// Limite por ação de um usuário (ex.: `"chat:<user_id>"`). Separado dos limites de
/// login e da API por IP, para que cada um tenha a sua janela de tempo.
pub fn check_action_rate_limit(key: &str, max_actions: usize, window: Duration) -> bool {
    let mut attempts = ACTION_RATE_LIMITER.lock().unwrap_or_else(|p| p.into_inner());
    let now = Instant::now();
    prune_stale_entries(&mut attempts, now, window);
    let entry = attempts.entry(key.to_string()).or_default();
    entry.retain(|&time| now.duration_since(time) < window);
    if entry.len() >= max_actions {
        false
    } else {
        entry.push(now);
        true
    }
}

/// Limite de chaves (IPs) acima do qual entradas sem tentativas recentes são descartadas,
/// evitando que o mapa do rate limiter cresça indefinidamente.
const RATE_LIMIT_PRUNE_THRESHOLD: usize = 1024;

pub fn prune_stale_entries(attempts: &mut HashMap<String, Vec<Instant>>, now: Instant, window: Duration) {
    if attempts.len() > RATE_LIMIT_PRUNE_THRESHOLD {
        attempts.retain(|_, times| times.iter().any(|&t| now.duration_since(t) < window));
    }
}

/// HSTS de produção: 1 ano. Sem `preload`, que inscreve o domínio na lista embutida dos
/// navegadores e leva meses para desfazer se o HTTPS precisar sair do ar.
pub const HSTS_VALUE: &str = "max-age=31536000; includeSubDomains";

/// Content-Security-Policy da resposta.
///
/// - Arquivos enviados por usuários (`/uploads/`) não executam nada: só imagens, em sandbox.
/// - Páginas: scripts só do próprio site, do CDN do Eruda (diagnóstico com SRI) ou embutidos
///   com o `nonce` gerado na renderização. Sem `'unsafe-inline'`, uma injeção de HTML que
///   escape da sanitização não consegue executar script.
pub fn content_security_policy(path: &str, nonce: Option<&str>) -> String {
    if path.starts_with("/uploads/") {
        return "default-src 'none'; img-src 'self'; style-src 'unsafe-inline'; sandbox".to_string();
    }
    let nonce_source = nonce.map(|n| format!(" 'nonce-{}'", n)).unwrap_or_default();
    format!(
        "default-src 'self'; script-src 'self'{nonce_source} 'wasm-unsafe-eval' https://cdn.jsdelivr.net; \
         style-src 'self' 'unsafe-inline' https://cdn.jsdelivr.net; img-src 'self' data: blob:; \
         font-src 'self' data:; connect-src 'self' ws: wss:; object-src 'none'; base-uri 'self'; \
         frame-ancestors 'self'; form-action 'self'"
    )
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
    // Páginas já trazem o CSP com o nonce da renderização (ver `App`); arquivos enviados
    // por usuários sempre recebem a política fechada, mesmo que algo tenha definido outra
    if path.starts_with("/uploads/") || !headers.contains_key(http::header::CONTENT_SECURITY_POLICY) {
        if let Ok(value) = http::HeaderValue::from_str(&content_security_policy(&path, None)) {
            headers.insert(http::header::CONTENT_SECURITY_POLICY, value);
        }
    }

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

        let hsts = crate::auth::is_production()
            || std::env::var("ENABLE_HSTS").map(|e| e == "1" || e.eq_ignore_ascii_case("true")).unwrap_or(false);

        if hsts {
            headers.insert(http::header::STRICT_TRANSPORT_SECURITY, http::HeaderValue::from_static(HSTS_VALUE));
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
        // Isenção só para loopback REAL (IP resolvido pelo client_ip_middleware), nunca pelo cabeçalho Host
        let is_loopback = request
            .extensions()
            .get::<super::client_ip::ClientIp>()
            .is_some_and(|c| c.0.is_loopback());
        if !is_loopback && !cfg!(debug_assertions) {
            let client_ip = super::client_ip::client_ip_from_extensions(request.extensions());
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
