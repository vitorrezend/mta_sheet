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

    let absolute_path = std::fs::canonicalize(db_path)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| db_path.to_string());

    if !database_url.starts_with("sqlite:") {
        database_url = format!("sqlite:{}", database_url);
    }

    println!("Connecting to database: {}", database_url);
    println!("Database absolute path: {}", absolute_path);

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
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_db_initialization() {
        // Use a temporary database file for testing
        let test_db = "test_mta_sheet.db";
        unsafe { env::set_var("DATABASE_URL", test_db); }

        let pool = get_db().await;

        // Check if table exists
        let result = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='character_sheets'")
            .fetch_one(&pool)
            .await;

        assert!(result.is_ok(), "Table character_sheets should exist");

        // Check if index exists
        let index_result = sqlx::query("SELECT name FROM sqlite_master WHERE type='index' AND name='idx_character_sheets_updated_at'")
            .fetch_one(&pool)
            .await;

        assert!(index_result.is_ok(), "Index idx_character_sheets_updated_at should exist");

        // Cleanup
        pool.close().await;
        let _ = std::fs::remove_file(test_db);
        let _ = std::fs::remove_file(format!("{}-journal", test_db));
    }
}
