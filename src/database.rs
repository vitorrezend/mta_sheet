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
        if !parent.as_os_str().is_empty() && !parent.exists() {
            println!("Creating database directory: {:?}", parent);
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

    pool
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[tokio::test]
    async fn test_full_crud_flow() {
        let test_db = "test_crud/test_mta.db";
        if fs::metadata("test_crud").is_ok() {
            fs::remove_dir_all("test_crud").ok();
        }

        unsafe { std::env::set_var("DATABASE_URL", test_db); }
        let pool = get_db().await;

        // Mock character data
        let char_id = "test-uuid-123";
        let char_name = "Test Character";
        let char_data = crate::state::CharacterData {
            id: char_id.to_string(),
            name: char_name.to_string(),
            ..Default::default()
        };
        let data_json = serde_json::to_string(&char_data).unwrap();

        // 1. Create (Insert)
        sqlx::query("INSERT INTO character_sheets (id, name, data) VALUES (?, ?, ?)")
            .bind(char_id)
            .bind(char_name)
            .bind(&data_json)
            .execute(&pool)
            .await
            .unwrap();

        // 2. Read (Select)
        let row: (String, String) = sqlx::query_as("SELECT name, data FROM character_sheets WHERE id = ?")
            .bind(char_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row.0, char_name);
        assert_eq!(row.1, data_json);

        // 3. Update
        let new_name = "Updated Character";
        sqlx::query("UPDATE character_sheets SET name = ? WHERE id = ?")
            .bind(new_name)
            .bind(char_id)
            .execute(&pool)
            .await
            .unwrap();

        let updated_name: String = sqlx::query_scalar("SELECT name FROM character_sheets WHERE id = ?")
            .bind(char_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(updated_name, new_name);

        // 4. Delete
        sqlx::query("DELETE FROM character_sheets WHERE id = ?")
            .bind(char_id)
            .execute(&pool)
            .await
            .unwrap();

        let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM character_sheets")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 0);

        pool.close().await;
        fs::remove_dir_all("test_crud").ok();
    }

    #[tokio::test]
    async fn test_db_initialization() {
        let test_db = "test_dir/test_mta.db";
        if fs::metadata("test_dir").is_ok() {
            fs::remove_dir_all("test_dir").ok();
        }

        unsafe { std::env::set_var("DATABASE_URL", test_db); }
        let pool = get_db().await;

        // Check if file exists
        assert!(fs::metadata(test_db).is_ok());

        // Check if table exists
        let table_exists: bool = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='character_sheets';")
            .fetch_optional(&pool)
            .await
            .unwrap()
            .is_some();
        assert!(table_exists);

        pool.close().await;
        fs::remove_dir_all("test_dir").ok();
    }
}
