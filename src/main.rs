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
fn get_cache_control_static() -> &'static str {
    if cfg!(debug_assertions) {
        "no-cache, no-store, must-revalidate"
    } else {
        "public, max-age=86400, stale-while-revalidate=3600"
    }
}

#[cfg(feature = "ssr")]
fn get_cache_control_css() -> &'static str {
    if cfg!(debug_assertions) {
        "no-cache, no-store, must-revalidate"
    } else {
        "public, max-age=3600, must-revalidate"
    }
}

#[cfg(feature = "ssr")]
async fn pkg_handler(
    axum::extract::Path(path): axum::extract::Path<String>,
) -> impl IntoResponse {
    let disk_path = format!("target/site/pkg/{}", path);
    let cache_hdr = get_cache_control_static().to_string();

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
                (http::header::CACHE_CONTROL, cache_hdr),
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
                (http::header::CACHE_CONTROL, cache_hdr),
            ],
            file.data.into_owned(),
        )
            .into_response();
    }

    // 3. Fallback especial para CSS caso o build WASM ainda não tenha rodado
    if path == "mta_sheet.css" || path == "style.css" {
        let css_cache = get_cache_control_css().to_string();
        if let Ok(css) = tokio::fs::read_to_string("style.css").await {
            return (
                [
                    (http::header::CONTENT_TYPE, "text/css".to_string()),
                    (http::header::CACHE_CONTROL, css_cache),
                ],
                css,
            ).into_response();
        } else {
            return (
                [
                    (http::header::CONTENT_TYPE, "text/css".to_string()),
                    (http::header::CACHE_CONTROL, css_cache),
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
    let cache_hdr = get_cache_control_static().to_string();

    if let Some(file) = SiteAssets::get(&key) {
        let mime = mime_guess::from_path(&path).first_or_octet_stream().to_string();
        return (
            [
                (http::header::CONTENT_TYPE, mime),
                (http::header::CACHE_CONTROL, cache_hdr),
            ],
            file.data.into_owned(),
        )
            .into_response();
    }

    let disk_path = format!("target/site/assets/{}", path);
    if let Ok(bytes) = tokio::fs::read(&disk_path).await {
        let mime = mime_guess::from_path(&path).first_or_octet_stream().to_string();
        return (
            [
                (http::header::CONTENT_TYPE, mime),
                (http::header::CACHE_CONTROL, cache_hdr),
            ],
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
    let cache_hdr = get_cache_control_css().to_string();

    if let Ok(bytes) = tokio::fs::read(&disk_path).await {
        return (
            [
                (http::header::CONTENT_TYPE, "text/css".to_string()),
                (http::header::CACHE_CONTROL, cache_hdr),
            ],
            bytes,
        )
            .into_response();
    }

    if let Some(file) = StyleAssets::get(&path) {
        return (
            [
                (http::header::CONTENT_TYPE, "text/css".to_string()),
                (http::header::CACHE_CONTROL, cache_hdr),
            ],
            file.data.into_owned(),
        )
            .into_response();
    }

    (http::StatusCode::NOT_FOUND, "Estilo não encontrado").into_response()
}

#[cfg(feature = "ssr")]
async fn style_css_handler() -> impl IntoResponse {
    let cache_hdr = get_cache_control_css().to_string();
    if let Ok(css) = tokio::fs::read_to_string("style.css").await {
        (
            [
                (http::header::CONTENT_TYPE, "text/css".to_string()),
                (http::header::CACHE_CONTROL, cache_hdr),
            ],
            css,
        )
            .into_response()
    } else {
        (
            [
                (http::header::CONTENT_TYPE, "text/css".to_string()),
                (http::header::CACHE_CONTROL, cache_hdr),
            ],
            EMBEDDED_STYLE_CSS.to_string(),
        )
            .into_response()
    }
}

#[cfg(feature = "ssr")]
#[derive(serde::Deserialize)]
struct FormAuthPayload {
    username: String,
    password: String,
    #[serde(default)]
    confirm_password: Option<String>,
}

#[cfg(feature = "ssr")]
async fn form_login_handler(
    axum::extract::Form(payload): axum::extract::Form<FormAuthPayload>,
) -> impl IntoResponse {
    let clean_user = payload.username.trim().to_string();
    if clean_user.is_empty() || payload.password.is_empty() {
        return (http::StatusCode::BAD_REQUEST, "Usuário e senha são obrigatórios").into_response();
    }

    let pool = mta_sheet::database::get_db().await;
    use sqlx::Row;

    let row = match sqlx::query("SELECT id, username, password_hash FROM users WHERE username = ? COLLATE NOCASE")
        .bind(&clean_user)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(r)) => r,
        _ => return (http::StatusCode::UNAUTHORIZED, "Usuário ou senha incorretos").into_response(),
    };

    let user_id: String = row.get("id");
    let password_hash: String = row.get("password_hash");

    if bcrypt::verify(&payload.password, &password_hash).unwrap_or(false) {
        let session_token = uuid::Uuid::new_v4().to_string();
        let _ = sqlx::query("INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, datetime('now', '+30 days'))")
            .bind(&session_token)
            .bind(&user_id)
            .execute(&pool)
            .await;

        let cookie_str = format!("session_token={}; Path=/; SameSite=Lax; HttpOnly; Max-Age=2592000", session_token);
        (
            [
                (http::header::SET_COOKIE, cookie_str),
                (http::header::LOCATION, "/".to_string()),
            ],
            http::StatusCode::SEE_OTHER,
        )
            .into_response()
    } else {
        (http::StatusCode::UNAUTHORIZED, "Usuário ou senha incorretos").into_response()
    }
}

#[cfg(feature = "ssr")]
async fn form_register_handler(
    axum::extract::Form(payload): axum::extract::Form<FormAuthPayload>,
) -> impl IntoResponse {
    let clean_user = payload.username.trim().to_string();
    if clean_user.len() < 3 || payload.password.len() < 4 {
        return (http::StatusCode::BAD_REQUEST, "Usuário (mínimo 3 caracteres) ou senha (mínimo 4 caracteres) inválidos").into_response();
    }

    if let Some(confirm) = payload.confirm_password {
        if !confirm.is_empty() && confirm != payload.password {
            return (http::StatusCode::BAD_REQUEST, "As senhas não conferem").into_response();
        }
    }

    let pool = mta_sheet::database::get_db().await;
    use sqlx::Row;

    let user_count: i64 = sqlx::query("SELECT COUNT(*) as count FROM users")
        .fetch_one(&pool)
        .await
        .map(|r| r.get("count"))
        .unwrap_or(0);

    let is_admin = user_count == 0 || mta_sheet::auth::is_username_in_admin_env(&clean_user);

    let password_hash = match bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return (http::StatusCode::INTERNAL_SERVER_ERROR, "Erro ao criptografar senha").into_response(),
    };

    let user_id = uuid::Uuid::new_v4().to_string();
    let insert_res = sqlx::query("INSERT INTO users (id, username, password_hash, is_admin) VALUES (?, ?, ?, ?)")
        .bind(&user_id)
        .bind(&clean_user)
        .bind(&password_hash)
        .bind(if is_admin { 1i64 } else { 0i64 })
        .execute(&pool)
        .await;

    if let Err(_) = insert_res {
        return (http::StatusCode::CONFLICT, "Este nome de usuário já está em uso").into_response();
    }

    let session_token = uuid::Uuid::new_v4().to_string();
    let _ = sqlx::query("INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, datetime('now', '+30 days'))")
        .bind(&session_token)
        .bind(&user_id)
        .execute(&pool)
        .await;

    let cookie_str = format!("session_token={}; Path=/; SameSite=Lax; HttpOnly; Max-Age=2592000", session_token);
    (
        [
            (http::header::SET_COOKIE, cookie_str),
            (http::header::LOCATION, "/".to_string()),
        ],
        http::StatusCode::SEE_OTHER,
    )
        .into_response()
}

#[cfg(feature = "ssr")]
async fn export_json_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let pool = mta_sheet::database::get_db().await;
    use sqlx::Row;

    let row = match sqlx::query("SELECT name, data FROM character_sheets WHERE id = ?")
        .bind(&id)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(r)) => r,
        _ => return (http::StatusCode::NOT_FOUND, "Ficha não encontrada").into_response(),
    };

    let name: String = row.get("name");
    let data_str: String = row.get("data");

    let safe_name = if name.trim().is_empty() {
        "ficha_mta".to_string()
    } else {
        name.trim().replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "_")
    };

    let filename = format!("{}_mta_sheet.json", safe_name);
    let disposition = format!("attachment; filename=\"{}\"", filename);

    (
        [
            (http::header::CONTENT_TYPE, "application/json; charset=utf-8".to_string()),
            (http::header::CONTENT_DISPOSITION, disposition),
            (http::header::CACHE_CONTROL, "no-cache".to_string()),
        ],
        data_str,
    )
        .into_response()
}

#[cfg(feature = "ssr")]
async fn upload_image_handler(
    mut multipart: axum::extract::Multipart,
) -> impl IntoResponse {
    let mut file_bytes = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        if let Ok(bytes) = field.bytes().await {
            file_bytes = bytes.to_vec();
            break;
        }
    }

    if file_bytes.is_empty() {
        return (http::StatusCode::BAD_REQUEST, [(http::header::CONTENT_TYPE, "application/json")], serde_json::json!({"error": "Nenhum arquivo enviado"}).to_string()).into_response();
    }

    if file_bytes.len() > 5 * 1024 * 1024 {
        return (http::StatusCode::PAYLOAD_TOO_LARGE, [(http::header::CONTENT_TYPE, "application/json")], serde_json::json!({"error": "Imagem excede 5MB"}).to_string()).into_response();
    }

    let (_mime_type, ext) = match mta_sheet::state::validate_image_magic_bytes(&file_bytes) {
        Ok(res) => res,
        Err(e) => return (http::StatusCode::BAD_REQUEST, [(http::header::CONTENT_TYPE, "application/json")], serde_json::json!({"error": e.to_string()}).to_string()).into_response(),
    };

    let file_id = uuid::Uuid::new_v4().to_string();
    let file_path = format!("uploads/{}.{}", file_id, ext);
    let public_url = format!("/uploads/{}.{}", file_id, ext);

    if let Err(e) = tokio::fs::write(&file_path, &file_bytes).await {
        return (http::StatusCode::INTERNAL_SERVER_ERROR, [(http::header::CONTENT_TYPE, "application/json")], serde_json::json!({"error": format!("Falha ao salvar: {}", e)}).to_string()).into_response();
    }

    (
        http::StatusCode::OK,
        [(http::header::CONTENT_TYPE, "application/json")],
        serde_json::json!({ "url": public_url }).to_string(),
    )
        .into_response()
}

#[cfg(feature = "ssr")]
async fn room_events_sse_handler(
    axum::extract::Path(room_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    use axum::response::sse::{Event, KeepAlive, Sse};
    use futures_util::stream::StreamExt;
    use tokio_stream::wrappers::BroadcastStream;

    let sender = mta_sheet::rooms::get_or_create_room_channel(&room_id);
    let rx = sender.subscribe();

    let stream = BroadcastStream::new(rx).filter_map(|msg| async move {
        match msg {
            Ok(event) => {
                let json = serde_json::to_string(&event).unwrap_or_default();
                Some(Ok::<_, std::convert::Infallible>(Event::default().data(json)))
            }
            Err(_) => None,
        }
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
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

    // Garante que o diretório de uploads exista e limpa logs antigos (>30 dias)
    let _ = tokio::fs::create_dir_all("uploads").await;
    mta_sheet::logging::server::cleanup_old_logs(30);

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
        .route("/api/form_login", axum::routing::post(form_login_handler))
        .route("/api/form_register", axum::routing::post(form_register_handler))
        .route("/api/upload_image", axum::routing::post(upload_image_handler))
        .route("/api/export_json/:id", axum::routing::get(export_json_handler))
        .route("/api/room_events/:id", axum::routing::get(room_events_sse_handler))
        .route("/pkg/*path", axum::routing::get(pkg_handler))
        .route("/assets/*path", axum::routing::get(assets_handler))
        .route("/styles/*path", axum::routing::get(styles_handler))
        .route("/style.css", axum::routing::get(style_css_handler))
        .route("/favicon.ico", axum::routing::get(|| async { (http::StatusCode::NO_CONTENT, "") }))
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
