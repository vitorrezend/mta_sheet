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

    log::info!("Connecting to database: {}", database_url);

    let options = SqliteConnectOptions::from_str(&database_url)
        .map_err(|e| {
            log::error!("Invalid DATABASE_URL: {}", e);
            e
        })
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

    // Add index for performance if it doesn't exist
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_character_sheets_updated_at ON character_sheets (updated_at DESC)")
        .execute(&pool)
        .await
        .ok();

    pool
}

#[cfg(all(feature = "ssr", test))]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_db_initialization() {
        // Use a temporary in-memory database for testing
        unsafe { env::set_var("DATABASE_URL", "sqlite::memory:"); }
        let pool = get_db().await;

        let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await.unwrap();
        assert_eq!(row.0, 1);

        // Verify table exists
        let table_exists: (bool,) = sqlx::query_as("SELECT EXISTS (SELECT 1 FROM sqlite_master WHERE type='table' AND name='character_sheets')")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(table_exists.0);
    }

    #[tokio::test]
    async fn test_full_crud_flow() {
        unsafe { env::set_var("DATABASE_URL", "sqlite::memory:"); }
        let pool = get_db().await;
        let id = "test-id";
        let name = "Test Char";
        let data = "{}";

        // Create
        sqlx::query("INSERT INTO character_sheets (id, name, data) VALUES (?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(data)
            .execute(&pool)
            .await
            .unwrap();

        // Read
        let row: (String,) = sqlx::query_as("SELECT name FROM character_sheets WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row.0, name);

        // Update
        let new_name = "Updated Char";
        sqlx::query("UPDATE character_sheets SET name = ? WHERE id = ?")
            .bind(new_name)
            .bind(id)
            .execute(&pool)
            .await
            .unwrap();

        let row: (String,) = sqlx::query_as("SELECT name FROM character_sheets WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row.0, new_name);

        // Delete
        sqlx::query("DELETE FROM character_sheets WHERE id = ?")
            .bind(id)
            .execute(&pool)
            .await
            .unwrap();

        let exists: (bool,) = sqlx::query_as("SELECT EXISTS (SELECT 1 FROM character_sheets WHERE id = ?)")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(!exists.0);
    }
}
