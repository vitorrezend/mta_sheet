#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use dotenvy::dotenv;
    use leptos::*;
    use leptos_axum::generate_route_list;
    use mta_sheet::{database, server};

    let _ = dotenv();

    // Obtém configuração do Leptos: se existir Cargo.toml, lê o manifesto; caso contrário, monta fallback direto
    let conf = if std::path::Path::new("Cargo.toml").exists() {
        get_configuration(Some("Cargo.toml")).await.unwrap_or_else(|_| {
            let mut opt = LeptosOptions::default();
            opt.output_name = "mta_sheet".into();
            opt.site_root = "target/site".into();
            opt.site_pkg_dir = "pkg".into();
            opt.site_addr = "0.0.0.0:3000".parse().unwrap_or_else(|_| std::net::SocketAddr::from(([0, 0, 0, 0], 3000)));
            leptos::leptos_config::ConfFile { leptos_options: opt }
        })
    } else {
        let mut opt = LeptosOptions::default();
        opt.output_name = "mta_sheet".into();
        opt.site_root = "target/site".into();
        opt.site_pkg_dir = "pkg".into();
        opt.site_addr = "0.0.0.0:3000".parse().unwrap_or_else(|_| std::net::SocketAddr::from(([0, 0, 0, 0], 3000)));
        leptos::leptos_config::ConfFile { leptos_options: opt }
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

    // Garante que o endereço padrão seja sempre 0.0.0.0:3000 para acesso via Wi-Fi e rede local
    if leptos_options.site_addr.ip().is_loopback() && std::env::var("LEPTOS_SITE_ADDR").is_err() {
        leptos_options.site_addr = "0.0.0.0:3000".parse().unwrap_or_else(|_| std::net::SocketAddr::from(([0, 0, 0, 0], 3000)));
    }

    // Permite sobrescrever o endereço por variável de ambiente se desejado
    if let Ok(addr_str) = std::env::var("LEPTOS_SITE_ADDR") {
        if let Ok(parsed_addr) = addr_str.parse() {
            leptos_options.site_addr = parsed_addr;
        }
    }

    let addr = leptos_options.site_addr;
    let routes = generate_route_list(mta_sheet::App);
    let db = database::get_db().await;

    // Garante que os diretórios existam, limpa logs antigos (>14 dias) e aplica limites de rotação
    let _ = tokio::fs::create_dir_all("uploads").await;
    mta_sheet::logging::server::cleanup_old_logs(14);
    mta_sheet::logging::server::enforce_all_category_limits();

    // Tarefa em segundo plano para manutenção contínua e limpeza de logs a cada 12 horas
    tokio::spawn(async {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(12 * 3600));
        // O primeiro tick ocorre de imediato; pulamos ele pois a limpeza inicial já foi executada no startup
        interval.tick().await;
        loop {
            interval.tick().await;
            mta_sheet::logging::server::cleanup_old_logs(14);
            mta_sheet::logging::server::enforce_all_category_limits();
        }
    });

    // Sincroniza mta_sheet.wasm <-> mta_sheet_bg.wasm para que ambos existam e estejam sempre atualizados
    let wasm_file = std::path::Path::new("target/site/pkg/mta_sheet.wasm");
    let wasm_bg_file = std::path::Path::new("target/site/pkg/mta_sheet_bg.wasm");
    if wasm_file.exists() {
        let need_copy = match (wasm_file.metadata(), wasm_bg_file.metadata()) {
            (Ok(src), Ok(dst)) => {
                src.len() != dst.len()
                    || src.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                        > dst.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH)
            }
            (Ok(_), Err(_)) => true,
            _ => false,
        };
        if need_copy {
            let _ = tokio::fs::copy(wasm_file, wasm_bg_file).await;
        }
    } else if wasm_bg_file.exists() && !wasm_file.exists() {
        let _ = tokio::fs::copy(wasm_bg_file, wasm_file).await;
    }

    // Monta o roteador completo através do módulo server
    let app = server::create_app(leptos_options, db, routes);

    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Falha crítica ao vincular servidor ao endereço {}: {}", addr, e);
            return;
        }
    };
    println!("listening on http://{}", addr);
    if addr.ip().is_unspecified() {
        println!("   -> Acesso local: http://localhost:{}", addr.port());
        println!("   -> Acesso via Wi-Fi / Rede Local: http://<SEU_IP_LOCAL>:{}", addr.port());
    }
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Erro na execução do servidor HTTP: {}", e);
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main; use hydrate() in lib.rs
}
