#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use mta_sheet::database;
    use tower_http::services::ServeDir;

    let conf = match get_configuration(Some("Cargo.toml")).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Falha crítica ao ler configuração do Leptos: {}", e);
            return;
        }
    };
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(mta_sheet::App);
    let site_root = conf.leptos_options.site_root.clone();

    let db = database::get_db().await;

    let db_for_server_fn = db.clone();
    let db_for_routes = db.clone();

    let _ = tokio::fs::create_dir_all("uploads").await;

    // build our application with a route
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
        .nest_service("/uploads", ServeDir::new("uploads"))
        .nest_service("/styles", ServeDir::new("styles"))
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
            provide_context(db_for_routes.clone());
        }, mta_sheet::App)
        .layer(axum::middleware::from_fn(security_headers_middleware))
        .with_state(conf.leptos_options);

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
