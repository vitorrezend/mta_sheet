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

    let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(mta_sheet::App);
    let site_root = conf.leptos_options.site_root.clone();

    let db = database::get_db().await;

    // build our application with a route
    let app = Router::new()
        .route("/pkg/mta_sheet_bg.wasm", axum::routing::get(|state: axum::extract::State<leptos::LeptosOptions>| async move {
            let site_root = state.site_root.clone();
            let path_bg = format!("{}/pkg/mta_sheet_bg.wasm", &site_root);
            let path_std = format!("{}/pkg/mta_sheet.wasm", &site_root);
            if let Ok(bytes) = tokio::fs::read(&path_bg).await {
                (
                    [(http::header::CONTENT_TYPE, "application/wasm")],
                    bytes
                )
            } else if let Ok(bytes) = tokio::fs::read(&path_std).await {
                (
                    [(http::header::CONTENT_TYPE, "application/wasm")],
                    bytes
                )
            } else {
                (
                    [(http::header::CONTENT_TYPE, "text/plain")],
                    Vec::new()
                )
            }
        }))
        .nest_service("/pkg", ServeDir::new(format!("{}/pkg", site_root)))
        .nest_service("/assets", ServeDir::new(format!("{}/assets", site_root)))
        .route("/style.css", axum::routing::get(|| async {
            match tokio::fs::read_to_string("style.css").await {
                Ok(css) => (
                    [(http::header::CONTENT_TYPE, "text/css")],
                    css
                ),
                Err(_) => (
                    [(http::header::CONTENT_TYPE, "text/css")],
                    String::new()
                )
            }
        }))
        .leptos_routes_with_context(&conf.leptos_options, routes, move || {
            provide_context(db.clone());
        }, mta_sheet::App)
        .with_state(conf.leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main; use hydrate() in lib.rs
}
