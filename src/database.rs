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

    // Add index for better listing performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_character_sheets_updated_at ON character_sheets (updated_at DESC)")
        .execute(&pool)
        .await
        .expect("Failed to create index");

    pool
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[tokio::test]
    async fn test_db_initialization() {
        let test_db = "test_init.db";
        if fs::metadata(test_db).is_ok() {
            fs::remove_file(test_db).unwrap();
        }

        unsafe { env::set_var("DATABASE_URL", format!("sqlite:{}", test_db)); }

        let pool = get_db().await;

        // Verify file exists
        assert!(fs::metadata(test_db).is_ok());

        // Verify table exists
        let table_exists: (i32,) = sqlx::query_as("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='character_sheets'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(table_exists.0, 1);

        pool.close().await;
        fs::remove_file(test_db).unwrap();
    }

    #[tokio::test]
    async fn test_full_crud_flow() {
        let test_db = "test_crud.db";
        if fs::metadata(test_db).is_ok() {
            fs::remove_file(test_db).unwrap();
        }

        // Manual environment manipulation for testing purposes.
        // In Rust Edition 2024, set_var is considered unsafe in multithreaded environments.
        unsafe { env::set_var("DATABASE_URL", format!("sqlite:{}", test_db)); }

        let pool = get_db().await;

        let id = "test-id";
        let name = "Test Character";
        let data = "{\"id\":\"test-id\",\"name\":\"Test Character\",\"attributes\":{},\"labels\":{},\"custom_lists\":{}}";

        // Create
        sqlx::query("INSERT INTO character_sheets (id, name, data) VALUES (?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(data)
            .execute(&pool)
            .await
            .unwrap();

        // Read
        let row: (String, String) = sqlx::query_as("SELECT name, data FROM character_sheets WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row.0, name);
        assert_eq!(row.1, data);

        // Update
        let new_name = "Updated Character";
        sqlx::query("UPDATE character_sheets SET name = ? WHERE id = ?")
            .bind(new_name)
            .bind(id)
            .execute(&pool)
            .await
            .unwrap();

        let updated_name: (String,) = sqlx::query_as("SELECT name FROM character_sheets WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(updated_name.0, new_name);

        // Delete
        sqlx::query("DELETE FROM character_sheets WHERE id = ?")
            .bind(id)
            .execute(&pool)
            .await
            .unwrap();

        let count: (i32,) = sqlx::query_as("SELECT count(*) FROM character_sheets WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count.0, 0);

        pool.close().await;
        fs::remove_file(test_db).unwrap();
    }
}
