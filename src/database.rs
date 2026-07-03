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

    let db_path_log = match std::fs::canonicalize(db_path) {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(_) => db_path.to_string(),
    };

    if !database_url.starts_with("sqlite:") {
        database_url = format!("sqlite:{}", database_url);
    }

    println!("Conectando ao banco de dados: {}", db_path_log);

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

    // Add index for updated_at
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_character_sheets_updated_at ON character_sheets (updated_at DESC)"
    )
    .execute(&pool)
    .await
    .expect("Failed to create index");

    println!("Banco de dados inicializado com sucesso.");

    pool
}

#[cfg(all(feature = "ssr", test))]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_db_initialization() {
        // Use a temporary database for testing
        let test_db = "test_mta_sheet.db";
        unsafe { env::set_var("DATABASE_URL", test_db) };

        let pool = get_db().await;

        // Check if table exists
        let row: (i64,) = sqlx::query_as("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='character_sheets'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row.0, 1);

        // Check if index exists
        let row: (i64,) = sqlx::query_as("SELECT count(*) FROM sqlite_master WHERE type='index' AND name='idx_character_sheets_updated_at'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row.0, 1);

        // Cleanup
        pool.close().await;
        let _ = std::fs::remove_file(test_db);
    }

    #[tokio::test]
    async fn test_full_crud_flow() {
        let test_db = "test_crud_mta.db";
        unsafe { env::set_var("DATABASE_URL", test_db) };
        let pool = get_db().await;

        let id = "test-id".to_string();
        let name = "Test Char".to_string();
        let data = "{}";

        // Create
        sqlx::query("INSERT INTO character_sheets (id, name, data) VALUES (?, ?, ?)")
            .bind(&id)
            .bind(&name)
            .bind(data)
            .execute(&pool)
            .await
            .unwrap();

        // Read
        let row: (String,) = sqlx::query_as("SELECT name FROM character_sheets WHERE id = ?")
            .bind(&id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row.0, name);

        // Update
        let new_name = "Updated Char".to_string();
        sqlx::query("UPDATE character_sheets SET name = ? WHERE id = ?")
            .bind(&new_name)
            .bind(&id)
            .execute(&pool)
            .await
            .unwrap();

        let row: (String,) = sqlx::query_as("SELECT name FROM character_sheets WHERE id = ?")
            .bind(&id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row.0, new_name);

        // Delete
        sqlx::query("DELETE FROM character_sheets WHERE id = ?")
            .bind(&id)
            .execute(&pool)
            .await
            .unwrap();

        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM character_sheets")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count.0, 0);

        pool.close().await;
        let _ = std::fs::remove_file(test_db);
    }
}
