#[cfg(feature = "ssr")]
use axum::response::IntoResponse;

#[cfg(feature = "ssr")]
#[derive(rust_embed::RustEmbed)]
#[folder = "target/site/"]
struct SiteAssets;

#[cfg(feature = "ssr")]
#[derive(rust_embed::RustEmbed)]
#[folder = "styles/"]
struct StyleAssets;

#[cfg(feature = "ssr")]
const EMBEDDED_STYLE_CSS: &str = include_str!("../style.css");

#[cfg(feature = "ssr")]
async fn pkg_handler(
    axum::extract::Path(path): axum::extract::Path<String>,
) -> impl IntoResponse {
    let disk_path = format!("target/site/pkg/{}", path);
    // 1. Tenta servir do disco local primeiro para garantir arquivos sempre atualizados em dev/build
    if let Ok(bytes) = tokio::fs::read(&disk_path).await {
        let mime = if path.ends_with(".wasm") {
            "application/wasm".to_string()
        } else if path.ends_with(".js") {
            "text/javascript".to_string()
        } else if path.ends_with(".css") {
            "text/css".to_string()
        } else {
            mime_guess::from_path(&path).first_or_octet_stream().to_string()
        };
        return (
            [
                (http::header::CONTENT_TYPE, mime),
                (http::header::CACHE_CONTROL, "no-cache, no-store, must-revalidate".to_string()),
            ],
            bytes,
        )
            .into_response();
    }

    let key = format!("pkg/{}", path);
    // 2. Fallback para o binário embutido
    if let Some(file) = SiteAssets::get(&key) {
        let mime = if path.ends_with(".wasm") {
            "application/wasm".to_string()
        } else if path.ends_with(".js") {
            "text/javascript".to_string()
        } else if path.ends_with(".css") {
            "text/css".to_string()
        } else {
            mime_guess::from_path(&path).first_or_octet_stream().to_string()
        };
        return (
            [
                (http::header::CONTENT_TYPE, mime),
                (http::header::CACHE_CONTROL, "no-cache, no-store, must-revalidate".to_string()),
            ],
            file.data.into_owned(),
        )
            .into_response();
    }

    // 3. Fallback especial para CSS caso o build WASM ainda não tenha rodado
    if path == "mta_sheet.css" || path == "style.css" {
        if let Ok(css) = tokio::fs::read_to_string("style.css").await {
            return (
                [
                    (http::header::CONTENT_TYPE, "text/css".to_string()),
                    (http::header::CACHE_CONTROL, "no-cache, no-store, must-revalidate".to_string()),
                ],
                css,
            ).into_response();
        } else {
            return (
                [
                    (http::header::CONTENT_TYPE, "text/css".to_string()),
                    (http::header::CACHE_CONTROL, "no-cache, no-store, must-revalidate".to_string()),
                ],
                EMBEDDED_STYLE_CSS.to_string(),
            ).into_response();
        }
    }

    (
        http::StatusCode::NOT_FOUND,
        [(http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "Arquivo pkg não encontrado",
    )
        .into_response()
}

#[cfg(feature = "ssr")]
async fn assets_handler(
    axum::extract::Path(path): axum::extract::Path<String>,
) -> impl IntoResponse {
    let key = format!("assets/{}", path);
    if let Some(file) = SiteAssets::get(&key) {
        let mime = mime_guess::from_path(&path).first_or_octet_stream().to_string();
        return (
            [(http::header::CONTENT_TYPE, mime)],
            file.data.into_owned(),
        )
            .into_response();
    }

    let disk_path = format!("target/site/assets/{}", path);
    if let Ok(bytes) = tokio::fs::read(&disk_path).await {
        let mime = mime_guess::from_path(&path).first_or_octet_stream().to_string();
        return (
            [(http::header::CONTENT_TYPE, mime)],
            bytes,
        )
            .into_response();
    }

    (http::StatusCode::NOT_FOUND, "Asset não encontrado").into_response()
}

#[cfg(feature = "ssr")]
async fn styles_handler(
    axum::extract::Path(path): axum::extract::Path<String>,
) -> impl IntoResponse {
    let disk_path = format!("styles/{}", path);
    if let Ok(bytes) = tokio::fs::read(&disk_path).await {
        return (
            [
                (http::header::CONTENT_TYPE, "text/css".to_string()),
                (http::header::CACHE_CONTROL, "no-cache, no-store, must-revalidate".to_string()),
            ],
            bytes,
        )
            .into_response();
    }

    if let Some(file) = StyleAssets::get(&path) {
        return (
            [
                (http::header::CONTENT_TYPE, "text/css".to_string()),
                (http::header::CACHE_CONTROL, "no-cache, no-store, must-revalidate".to_string()),
            ],
            file.data.into_owned(),
        )
            .into_response();
    }

    (http::StatusCode::NOT_FOUND, "Estilo não encontrado").into_response()
}

#[cfg(feature = "ssr")]
async fn style_css_handler() -> impl IntoResponse {
    if let Ok(css) = tokio::fs::read_to_string("style.css").await {
        (
            [
                (http::header::CONTENT_TYPE, "text/css".to_string()),
                (http::header::CACHE_CONTROL, "no-cache, no-store, must-revalidate".to_string()),
            ],
            css,
        )
            .into_response()
    } else {
        (
            [
                (http::header::CONTENT_TYPE, "text/css".to_string()),
                (http::header::CACHE_CONTROL, "no-cache, no-store, must-revalidate".to_string()),
            ],
            EMBEDDED_STYLE_CSS.to_string(),
        )
            .into_response()
    }
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use mta_sheet::database;
    use tower_http::services::ServeDir;
    use dotenvy::dotenv;

    let _ = dotenv();

    let manifest_path = if std::path::Path::new("Cargo.toml").exists() {
        Some("Cargo.toml")
    } else {
        None
    };

    // Obtém configuração do Leptos com fallback seguro caso Cargo.toml não exista
    let conf = match get_configuration(manifest_path).await {
        Ok(c) => c,
        Err(_) => {
            let mut opt = LeptosOptions::default();
            opt.output_name = "mta_sheet".into();
            opt.site_root = "target/site".into();
            opt.site_pkg_dir = "pkg".into();
            opt.site_addr = "0.0.0.0:3000".parse().unwrap_or_else(|_| std::net::SocketAddr::from(([0, 0, 0, 0], 3000)));
            leptos::leptos_config::ConfFile { leptos_options: opt }
        }
    };
    let mut leptos_options = conf.leptos_options;

    if leptos_options.output_name.is_empty() {
        leptos_options.output_name = "mta_sheet".into();
    }
    if leptos_options.site_pkg_dir.is_empty() {
        leptos_options.site_pkg_dir = "pkg".into();
    }
    if leptos_options.site_root.is_empty() {
        leptos_options.site_root = "target/site".into();
    }

    // Permite sobrescrever o endereço por variável de ambiente
    if let Ok(addr_str) = std::env::var("LEPTOS_SITE_ADDR") {
        if let Ok(parsed_addr) = addr_str.parse() {
            leptos_options.site_addr = parsed_addr;
        }
    }

    let addr = leptos_options.site_addr;
    let routes = generate_route_list(mta_sheet::App);

    let db = database::get_db().await;
    let db_for_server_fn = db.clone();
    let db_for_routes = db.clone();

    // Garante que o diretório de uploads exista
    let _ = tokio::fs::create_dir_all("uploads").await;

    // Monta o roteador da aplicação
    let app = Router::new()
        .route(
            "/api/*fn_name",
            axum::routing::post({
                let db = db_for_server_fn.clone();
                move |req: axum::extract::Request| async move {
                    leptos_axum::handle_server_fns_with_context(
                        {
                            let db = db.clone();
                            move || {
                                provide_context(db.clone());
                            }
                        },
                        req,
                    )
                    .await
                }
            })
            .get({
                let db = db_for_server_fn.clone();
                move |req: axum::extract::Request| async move {
                    leptos_axum::handle_server_fns_with_context(
                        {
                            let db = db.clone();
                            move || {
                                provide_context(db.clone());
                            }
                        },
                        req,
                    )
                    .await
                }
            }),
        )
        .route("/pkg/*path", axum::routing::get(pkg_handler))
        .route("/assets/*path", axum::routing::get(assets_handler))
        .route("/styles/*path", axum::routing::get(styles_handler))
        .route("/style.css", axum::routing::get(style_css_handler))
        .nest_service("/uploads", ServeDir::new("uploads"))
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || {
                provide_context(db_for_routes.clone());
            },
            mta_sheet::App,
        )
        .layer(axum::middleware::from_fn(security_headers_middleware))
        .layer(axum::middleware::from_fn(access_log_middleware))
        .with_state(leptos_options);

    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Falha crítica ao vincular servidor ao endereço {}: {}", addr, e);
            return;
        }
    };
    println!("listening on http://{}", addr);
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Erro na execução do servidor HTTP: {}", e);
    }
}

#[cfg(feature = "ssr")]
async fn access_log_middleware(
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
        let classification = mta_sheet::logging::classify_traffic(
            user_agent.as_deref(),
            sec_fetch_mode.as_deref(),
            sec_ch_ua.as_deref(),
            accept_lang.as_deref(),
        );

        mta_sheet::logging::server::write_access_log(
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

#[cfg(feature = "ssr")]
async fn security_headers_middleware(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
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

    response
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main; use hydrate() in lib.rs
}
