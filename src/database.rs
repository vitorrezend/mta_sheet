#[cfg(feature = "ssr")]
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqlitePool};
#[cfg(feature = "ssr")]
use std::str::FromStr;

#[cfg(feature = "ssr")]
pub async fn get_db() -> SqlitePool {
    let mut database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "mta_sheet.db".to_string());

    let db_path = if database_url.starts_with("sqlite:") {
        database_url[7..].to_string()
    } else {
        database_url.clone()
    };

    // Ensure parent directory exists
    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).ok();
        }
    }

    if !database_url.starts_with("sqlite:") {
        database_url = format!("sqlite:{}", database_url);
    }

    if let Ok(abs_path) = std::fs::canonicalize(&db_path) {
        println!("Connecting to database: {}", abs_path.display());
    } else {
        println!("Connecting to database: {}", database_url);
    }

    let options = SqliteConnectOptions::from_str(&database_url)
        .map_err(|e| format!("Invalid DATABASE_URL '{}': {}", database_url, e))
        .expect("Database configuration error")
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

    // Create index for performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_character_sheets_updated_at ON character_sheets (updated_at DESC)")
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
        let test_db_path = "test_mta_sheet.db";
        unsafe { env::set_var("DATABASE_URL", test_db_path) };

        let pool = get_db().await;

        // Verify tables
        let table_exists: (i64,) = sqlx::query_as("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='character_sheets'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(table_exists.0, 1);

        // Verify index
        let index_exists: (i64,) = sqlx::query_as("SELECT count(*) FROM sqlite_master WHERE type='index' AND name='idx_character_sheets_updated_at'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(index_exists.0, 1);

        pool.close().await;
        std::fs::remove_file(test_db_path).ok();
    }
}
