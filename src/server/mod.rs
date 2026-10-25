pub mod handlers;
pub mod middleware;

use axum::Router;
use leptos::*;
use leptos_axum::LeptosRoutes;
use leptos_router::RouteListing;
use sqlx::SqlitePool;
use tower_http::services::ServeDir;

pub use handlers::*;
pub use middleware::*;

/// Constrói o roteador Axum completo da aplicação, integrando rotas de API,
/// Leptos SSR, arquivos estáticos e middlewares de segurança e auditoria.
pub fn create_app(
    leptos_options: LeptosOptions,
    db: SqlitePool,
    routes: Vec<RouteListing>,
) -> Router {
    let db_for_server_fn = db.clone();
    let db_for_routes = db.clone();

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
        .route("/api/form_login", axum::routing::post(form_login_handler))
        .route("/api/form_register", axum::routing::post(form_register_handler))
        .route("/api/upload_image", axum::routing::post(upload_image_handler))
        .route("/api/export_json/:id", axum::routing::get(export_json_handler))
        .route("/api/room_events/:id", axum::routing::get(room_events_sse_handler))
        .route("/api/compendium/:section", axum::routing::get(compendium_api_handler));

    #[cfg(debug_assertions)]
    let app = app
        .nest_service("/pkg", ServeDir::new("target/site/pkg").fallback(axum::routing::get(pkg_handler)))
        .nest_service("/assets", ServeDir::new("target/site/assets").fallback(axum::routing::get(assets_handler)))
        .nest_service("/fonts", ServeDir::new("target/site/fonts").fallback(axum::routing::get(fonts_handler)))
        .nest_service("/styles", ServeDir::new("styles").fallback(axum::routing::get(styles_handler)));

    #[cfg(not(debug_assertions))]
    let app = app
        .route("/pkg", axum::routing::get(pkg_handler))
        .route("/pkg/*path", axum::routing::get(pkg_handler))
        .route("/assets", axum::routing::get(assets_handler))
        .route("/assets/*path", axum::routing::get(assets_handler))
        .route("/fonts", axum::routing::get(fonts_handler))
        .route("/fonts/*path", axum::routing::get(fonts_handler))
        .route("/styles", axum::routing::get(styles_handler))
        .route("/styles/*path", axum::routing::get(styles_handler));

    app
        .route("/style.css", axum::routing::get(style_css_handler))
        .route("/robots.txt", axum::routing::get(robots_txt_handler))
        .route("/sitemap.xml", axum::routing::get(sitemap_xml_handler))
        .route("/favicon.svg", axum::routing::get(favicon_svg_handler))
        .route("/favicon.ico", axum::routing::get(favicon_ico_handler))
        .route("/banner_og.jpg", axum::routing::get(banner_og_handler))
        .route("/banner_og.png", axum::routing::get(banner_og_handler))
        .nest_service("/uploads", ServeDir::new("uploads"))
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || {
                provide_context(db_for_routes.clone());
            },
            crate::App,
        )
        .layer(axum::middleware::from_fn(api_rate_limit_middleware))
        .layer(axum::middleware::from_fn(csrf_protection_middleware))
        // Brotli/gzip conforme o Accept-Encoding do navegador: a WebAssembly cai de ~7,5 MB
        // para ~2 MB. O predicado padrão não comprime eventos SSE (tempo real das salas),
        // imagens nem respostas minúsculas; nível 5 equilibra tamanho e CPU por requisição.
        // Em produção a WebAssembly, o JS e o CSS já saem pré-comprimidos (handlers::precompressed),
        // e esta camada pula respostas que já têm Content-Encoding.
        .layer(
            tower_http::compression::CompressionLayer::new()
                .quality(tower_http::CompressionLevel::Precise(5))
                .compress_when(compression_predicate()),
        )
        .layer(axum::middleware::from_fn(security_headers_middleware))
        .layer(axum::middleware::from_fn(access_log_middleware))
        // Camada mais externa: resolve o IP real antes de logs e rate limit
        .layer(axum::middleware::from_fn(client_ip_middleware))
        .with_state(leptos_options)
}

/// Quando comprimir dinamicamente. Em desenvolvimento a WebAssembly tem dezenas de MB com
/// depuração e muda a cada build: comprimi-la a cada carga custaria segundos por página.
fn compression_predicate() -> impl tower_http::compression::Predicate {
    use tower_http::compression::predicate::{DefaultPredicate, NotForContentType, Predicate};
    let skip_wasm_in_dev = if cfg!(debug_assertions) { "application/wasm" } else { "application/x-never-skip" };
    DefaultPredicate::new().and(NotForContentType::const_new(skip_wasm_in_dev))
}
