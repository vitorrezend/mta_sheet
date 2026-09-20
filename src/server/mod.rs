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
        .layer(axum::middleware::from_fn(security_headers_middleware))
        .layer(axum::middleware::from_fn(access_log_middleware))
        .with_state(leptos_options)
}
