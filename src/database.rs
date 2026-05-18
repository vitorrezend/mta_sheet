use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqlitePool};
use std::str::FromStr;

#[cfg(feature = "ssr")]
pub async fn get_db() -> SqlitePool {
    use dotenvy::dotenv;
    let _ = dotenv();
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:mta_sheet.db".to_string());

    log::info!("Connecting to database at: {}", database_url);

    let options = SqliteConnectOptions::from_str(&database_url)
        .expect("Invalid DATABASE_URL format")
        .create_if_missing(true)
        .log_statements(log::LevelFilter::Debug);

    let pool = SqlitePool::connect_with(options).await.expect("Failed to connect to SQLite pool");

    // Initialize tables
    if let Err(e) = sqlx::query(
        "CREATE TABLE IF NOT EXISTS character_sheets (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            data TEXT NOT NULL,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await {
        log::error!("Failed to initialize database tables: {:?}", e);
        panic!("Database initialization failed");
    } else {
        log::info!("Database tables initialized successfully");
    }

    pool
}
