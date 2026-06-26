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

    let absolute_path = std::fs::canonicalize(&db_path)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| db_path.clone());

    println!("Conectando ao banco de dados SQLite em: {}", absolute_path);

    let options = SqliteConnectOptions::from_str(&database_url)
        .expect("String DATABASE_URL inválida")
        .create_if_missing(true)
        .log_statements(log::LevelFilter::Debug);

    let pool = SqlitePool::connect_with(options).await.expect("Falha ao conectar ao SQLite");

    // Initialize tables and indexes
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
    .expect("Falha ao criar tabela character_sheets");

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_character_sheets_updated_at ON character_sheets (updated_at DESC)"
    )
    .execute(&pool)
    .await
    .expect("Falha ao criar índice para character_sheets");

    pool
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_db_initialization() {
        // Use a temporary database for testing
        let test_db = "test_mta.db";
        unsafe {
            env::set_var("DATABASE_URL", test_db);
        }

        let pool = get_db().await;

        // Verify if table exists
        let table_exists: bool = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='character_sheets'")
            .fetch_optional(&pool)
            .await
            .unwrap()
            .is_some();

        assert!(table_exists);

        // Clean up
        let _ = std::fs::remove_file(test_db);
    }

    #[tokio::test]
    async fn test_full_crud_flow() {
        let test_db = "test_crud.db";
        unsafe {
            env::set_var("DATABASE_URL", test_db);
        }
        let pool = get_db().await;

        let id = "test-id-123";
        let name = "Test Mage";
        let data = "{\"id\":\"test-id-123\",\"name\":\"Test Mage\",\"attributes\":{},\"labels\":{},\"custom_lists\":{}}";

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
        let new_name = "Updated Mage";
        sqlx::query("UPDATE character_sheets SET name = ? WHERE id = ?")
            .bind(new_name)
            .bind(id)
            .execute(&pool)
            .await
            .unwrap();

        let updated_name: String = sqlx::query_scalar("SELECT name FROM character_sheets WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(updated_name, new_name);

        // Delete
        sqlx::query("DELETE FROM character_sheets WHERE id = ?")
            .bind(id)
            .execute(&pool)
            .await
            .unwrap();

        let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM character_sheets")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(count, 0);

        // Clean up
        let _ = std::fs::remove_file(test_db);
    }
}
