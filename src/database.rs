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

    let options = match SqliteConnectOptions::from_str(&database_url) {
        Ok(opts) => opts
            .create_if_missing(true)
            .log_statements(log::LevelFilter::Debug),
        Err(e) => {
            eprintln!("Error parsing DATABASE_URL '{}': {}", database_url, e);
            panic!("Invalid DATABASE_URL");
        }
    };

    let pool = match SqlitePool::connect_with(options).await {
        Ok(p) => {
            println!("Successfully connected to SQLite database.");
            p
        },
        Err(e) => {
            eprintln!("Failed to connect to SQLite at '{}': {}", database_url, e);
            panic!("Database connection failed");
        }
    };

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

    // Add index for updated_at to optimize listing
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_character_sheets_updated_at ON character_sheets (updated_at DESC)"
    )
    .execute(&pool)
    .await
    .expect("Failed to create index");

    println!("Database schema initialized successfully.");

    pool
}

#[cfg(test)]
#[cfg(feature = "ssr")]
mod tests {
    use super::*;
    use crate::state::{create_sheet, get_sheets, get_sheet, update_sheet, delete_sheet};
    use leptos::provide_context;

    async fn setup_test_db() -> SqlitePool {
        let test_db_url = format!("sqlite:test_mta_{}.db", uuid::Uuid::new_v4());
        unsafe {
            std::env::set_var("DATABASE_URL", &test_db_url);
        }
        get_db().await
    }

    async fn teardown_test_db(pool: SqlitePool) {
        pool.close().await;
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let path = db_url.strip_prefix("sqlite:").unwrap_or(&db_url);
        let _ = std::fs::remove_file(path);
    }

    #[tokio::test]
    async fn test_db_initialization() {
        let pool = setup_test_db().await;

        // Verify table exists
        let table_exists: bool = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='character_sheets'")
            .fetch_optional(&pool)
            .await
            .unwrap()
            .is_some();
        assert!(table_exists);

        // Verify index exists
        let index_exists: bool = sqlx::query("SELECT name FROM sqlite_master WHERE type='index' AND name='idx_character_sheets_updated_at'")
            .fetch_optional(&pool)
            .await
            .unwrap()
            .is_some();
        assert!(index_exists);

        teardown_test_db(pool).await;
    }

    #[tokio::test]
    async fn test_full_crud_flow() {
        let pool = setup_test_db().await;

        let runtime = leptos::create_runtime();
        provide_context(pool.clone());

        // Create
        let name = "Test Character".to_string();
        let id = create_sheet(name.clone()).await.expect("Failed to create sheet");
        assert!(!id.is_empty());

        // Read (List)
        let sheets = get_sheets().await.expect("Failed to get sheets");
        assert_eq!(sheets.len(), 1);
        assert_eq!(sheets[0].name, name);
        assert_eq!(sheets[0].id, id);

        // Read (Single)
        let sheet = get_sheet(id.clone()).await.expect("Failed to get sheet");
        assert_eq!(sheet.name, name);
        assert_eq!(sheet.id, id);

        // Update
        let mut updated_data = sheet.clone();
        updated_data.name = "Updated Name".to_string();
        update_sheet(id.clone(), updated_data.clone()).await.expect("Failed to update sheet");

        let fetched_updated = get_sheet(id.clone()).await.expect("Failed to get updated sheet");
        assert_eq!(fetched_updated.name, "Updated Name");

        // Delete
        delete_sheet(id.clone()).await.expect("Failed to delete sheet");
        let sheets_after_delete = get_sheets().await.expect("Failed to get sheets after delete");
        assert_eq!(sheets_after_delete.len(), 0);

        runtime.dispose();
        teardown_test_db(pool).await;
    }
}
