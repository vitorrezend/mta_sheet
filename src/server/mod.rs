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

    Router::new()
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
        .nest_service("/pkg", ServeDir::new("target/site/pkg").fallback(axum::routing::get(pkg_handler)))
        .nest_service("/assets", ServeDir::new("target/site/assets").fallback(axum::routing::get(assets_handler)))
        .nest_service("/styles", ServeDir::new("styles").fallback(axum::routing::get(styles_handler)))
        .route("/style.css", axum::routing::get(style_css_handler))
        .route("/favicon.ico", axum::routing::get(|| async { (http::StatusCode::NO_CONTENT, "") }))
        .nest_service("/uploads", ServeDir::new("uploads"))
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || {
                provide_context(db_for_routes.clone());
            },
            crate::App,
        )
        .layer(axum::middleware::from_fn(security_headers_middleware))
        .layer(axum::middleware::from_fn(access_log_middleware))
        .with_state(leptos_options)
}
