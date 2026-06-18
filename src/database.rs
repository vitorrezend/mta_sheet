#[cfg(feature = "ssr")]
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqlitePool};
#[cfg(feature = "ssr")]
use std::str::FromStr;

#[cfg(feature = "ssr")]
pub async fn get_db() -> SqlitePool {
    let mut database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "mta_sheet.db".to_string());

    let db_path = if database_url.starts_with("sqlite:") {
        &database_url[7..]
    } else {
        &database_url
    };

    // Ensure parent directory exists
    if let Some(parent) = std::path::Path::new(db_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).ok();
        }
    }

    if !database_url.starts_with("sqlite:") {
        database_url = format!("sqlite:{}", database_url);
    }

    println!("Connecting to database: {}", database_url);

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

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_character_sheets_updated_at ON character_sheets (updated_at DESC)"
    )
    .execute(&pool)
    .await
    .expect("Failed to create index");

    pool
}
#[cfg(test)]
mod database_tests {
    use crate::database::get_db;
    use std::fs;
    use std::path::Path;

    #[tokio::test]
    async fn test_db_initialization() {
        let test_db = "test_data/test_mta.db";
        // Clean up before test
        if Path::new("test_data").exists() {
            fs::remove_dir_all("test_data").ok();
        }

        // Note: set_var is unsafe in Rust Edition 2024 because it is not thread-safe.
        // We use it here in a controlled test environment.
        unsafe { std::env::set_var("DATABASE_URL", test_db); }

        let pool = get_db().await;

        // Check if file exists
        assert!(Path::new(test_db).exists());

        // Check if table exists
        let table_exists: bool = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='character_sheets'")
            .fetch_optional(&pool)
            .await
            .unwrap()
            .is_some();
        assert!(table_exists);

        // Check if index exists
        let index_exists: bool = sqlx::query("SELECT name FROM sqlite_master WHERE type='index' AND name='idx_character_sheets_updated_at'")
            .fetch_optional(&pool)
            .await
            .unwrap()
            .is_some();
        assert!(index_exists);

        pool.close().await;
        fs::remove_dir_all("test_data").ok();
    }
}
