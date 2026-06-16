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

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use crate::state::*;
    use leptos::*;

    #[tokio::test]
    async fn test_full_crud_flow() {
        // Use a unique database for the test
        let test_db_file = "test_mta_sheet.db";
        let _ = std::fs::remove_file(test_db_file);

        // Safety: in tests we often need to set env vars, though it is technically unsafe in 2024 edition
        unsafe { std::env::set_var("DATABASE_URL", test_db_file) };

        let pool = get_db().await;
        let runtime = create_runtime();
        provide_context(pool.clone());

        // Create
        let name = "Test Hero".to_string();
        let id = create_sheet(name.clone()).await.expect("Failed to create sheet");
        assert!(!id.is_empty());

        // Read List
        let sheets = get_sheets().await.expect("Failed to get sheets");
        assert!(sheets.iter().any(|s| s.id == id && s.name == name));

        // Read One
        let mut data = get_sheet(id.clone()).await.expect("Failed to get sheet");
        assert_eq!(data.name, name);

        // Update
        data.name = "Updated Hero".to_string();
        update_sheet(id.clone(), data.clone()).await.expect("Failed to update sheet");

        let updated_data = get_sheet(id.clone()).await.expect("Failed to get updated sheet");
        assert_eq!(updated_data.name, "Updated Hero");

        // Delete
        delete_sheet(id.clone()).await.expect("Failed to delete sheet");
        let sheets_after_delete = get_sheets().await.expect("Failed to get sheets after delete");
        assert!(!sheets_after_delete.iter().any(|s| s.id == id));

        runtime.dispose();
        let _ = std::fs::remove_file(test_db_file);
    }
}
