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

    // Add index for performance on large lists
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_character_sheets_updated_at ON character_sheets (updated_at DESC)"
    )
    .execute(&pool)
    .await
    .ok();

    pool
}

#[cfg(all(feature = "ssr", test))]
mod tests {
    use super::*;
    use sqlx::Row;

    #[tokio::test]
    async fn test_full_crud_flow() {
        let test_db = "test_mta.db";
        unsafe { std::env::set_var("DATABASE_URL", test_db); }

        // Clean up any previous test db
        let _ = std::fs::remove_file(test_db);

        let pool = get_db().await;

        // CREATE
        let id = "test-uuid-123";
        let name = "Test Character";
        let data = r#"{"id":"test-uuid-123","name":"Test Character","attributes":{},"labels":{},"custom_lists":{}}"#;

        sqlx::query("INSERT INTO character_sheets (id, name, data) VALUES (?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(data)
            .execute(&pool)
            .await
            .unwrap();

        // READ
        let row = sqlx::query("SELECT name, data FROM character_sheets WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(row.get::<String, _>("name"), name);
        assert_eq!(row.get::<String, _>("data"), data);

        // UPDATE
        let new_name = "Updated Character";
        sqlx::query("UPDATE character_sheets SET name = ? WHERE id = ?")
            .bind(new_name)
            .bind(id)
            .execute(&pool)
            .await
            .unwrap();

        let row_updated = sqlx::query("SELECT name FROM character_sheets WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row_updated.get::<String, _>("name"), new_name);

        // DELETE
        sqlx::query("DELETE FROM character_sheets WHERE id = ?")
            .bind(id)
            .execute(&pool)
            .await
            .unwrap();

        let row_count = sqlx::query("SELECT COUNT(*) FROM character_sheets")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row_count.get::<i64, _>(0), 0);

        // Cleanup
        drop(pool);
        if std::env::var("KEEP_TEST_DB").is_err() {
            let _ = std::fs::remove_file(test_db);
        }
    }
}
