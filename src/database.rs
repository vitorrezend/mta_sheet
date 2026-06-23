#[cfg(feature = "ssr")]
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqlitePool};
#[cfg(feature = "ssr")]
use std::str::FromStr;

#[cfg(feature = "ssr")]
pub async fn get_db() -> SqlitePool {
    let database_url_raw = std::env::var("DATABASE_URL").unwrap_or_else(|_| "mta_sheet.db".to_string());

    let db_path = if database_url_raw.starts_with("sqlite:") {
        database_url_raw[7..].to_string()
    } else {
        database_url_raw.clone()
    };

    // Ensure parent directory exists
    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).ok();
        }
    }

    let database_url = if !database_url_raw.starts_with("sqlite:") {
        format!("sqlite:{}", database_url_raw)
    } else {
        database_url_raw
    };

    println!("Connecting to database: {}", database_url);
    if let Ok(path) = std::fs::canonicalize(&db_path) {
        println!("Absolute database path: {:?}", path);
    }

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
    use crate::state::{create_sheet, get_sheets, get_sheet, update_sheet};
    use leptos::*;
    use sqlx::Row;

    #[tokio::test]
    async fn test_db_initialization() {
        let test_db = "test_mta_init.db";
        if std::path::Path::new(test_db).exists() {
            let _ = std::fs::remove_file(test_db);
        }

        unsafe {
            std::env::set_var("DATABASE_URL", test_db);
        }

        let pool = get_db().await;

        let table_exists: bool = sqlx::query("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='character_sheets'")
            .fetch_one(&pool)
            .await
            .unwrap()
            .get::<i32, _>(0) > 0;

        assert!(table_exists);

        pool.close().await;
        if std::path::Path::new(test_db).exists() {
            let _ = std::fs::remove_file(test_db);
        }
    }

    #[tokio::test]
    async fn test_full_crud_flow() {
        let test_db = "test_mta_crud.db";
        if std::path::Path::new(test_db).exists() {
            let _ = std::fs::remove_file(test_db);
        }

        unsafe {
            std::env::set_var("DATABASE_URL", test_db);
        }

        let pool = get_db().await;
        let runtime = create_runtime();
        provide_context(pool.clone());

        let name = "Test Mage".to_string();
        let id = create_sheet(name.clone()).await.expect("Failed to create sheet");
        assert!(!id.is_empty());

        let sheets = get_sheets().await.expect("Failed to get sheets");
        assert_eq!(sheets.len(), 1);

        let mut data = get_sheet(id.clone()).await.expect("Failed to get sheet");
        assert_eq!(data.name, name);

        data.name = "Updated Mage".to_string();
        update_sheet(id.clone(), data.clone()).await.expect("Failed to update sheet");

        let updated_data = get_sheet(id.clone()).await.expect("Failed to get updated sheet");
        assert_eq!(updated_data.name, "Updated Mage");

        runtime.dispose();
        pool.close().await;
        if std::path::Path::new(test_db).exists() {
            let _ = std::fs::remove_file(test_db);
        }
    }
}
