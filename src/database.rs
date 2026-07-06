#[cfg(feature = "ssr")]
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqlitePool};
#[cfg(feature = "ssr")]
use std::str::FromStr;

#[cfg(feature = "ssr")]
pub async fn get_db() -> SqlitePool {
    let database_url_env = std::env::var("DATABASE_URL").unwrap_or_else(|_| "mta_sheet.db".to_string());

    let db_path = if database_url_env.starts_with("sqlite:") {
        database_url_env[7..].to_string()
    } else {
        database_url_env.clone()
    };

    // Ensure parent directory exists
    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).ok();
        }
    }

    let mut database_url = database_url_env;
    if !database_url.starts_with("sqlite:") {
        database_url = format!("sqlite:{}", database_url);
    }

    let absolute_path = std::fs::canonicalize(&db_path)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| db_path.clone());

    println!("Conectando ao banco de dados SQLite:");
    println!("  URL: {}", database_url);
    println!("  Caminho Absoluto: {}", absolute_path);

    let options = SqliteConnectOptions::from_str(&database_url)
        .expect("Invalid DATABASE_URL")
        .create_if_missing(true)
        .log_statements(log::LevelFilter::Debug);

    let pool = SqlitePool::connect_with(options).await.expect("Failed to connect to SQLite");

    // Initialize tables
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS character_sheets (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            data TEXT NOT NULL,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    println!("Banco de dados inicializado com sucesso.");

    pool
}
