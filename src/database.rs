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
        .map_err(|e| format!("URL de banco de dados inválida (DATABASE_URL): {}. Erro: {}", database_url, e))
        .expect("Falha ao configurar opções de conexão com SQLite");

    let options = options.create_if_missing(true)
        .log_statements(log::LevelFilter::Debug);

    let pool = SqlitePool::connect_with(options).await.expect("Não foi possível conectar ao banco de dados SQLite");

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

    // Add index for performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_character_sheets_updated_at ON character_sheets (updated_at DESC)")
        .execute(&pool)
        .await
        .ok();

    pool
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use crate::state::CharacterData;
    use sqlx::Row;

    #[tokio::test]
    async fn test_full_crud_flow() {
        // Setup temporary database
        let db_path = "test_mta_sheet.db";
        if std::path::Path::new(db_path).exists() {
            std::fs::remove_file(db_path).ok();
        }

        unsafe { std::env::set_var("DATABASE_URL", db_path); }
        let pool = get_db().await;

        // 1. Create
        let id = "test-id".to_string();
        let name = "Test Character".to_string();
        let initial_data = CharacterData {
            id: id.clone(),
            name: name.clone(),
            ..Default::default()
        };
        let data_json = serde_json::to_string(&initial_data).unwrap();

        sqlx::query("INSERT INTO character_sheets (id, name, data) VALUES (?, ?, ?)")
            .bind(&id)
            .bind(&name)
            .bind(&data_json)
            .execute(&pool)
            .await
            .unwrap();

        // 2. Read Summary
        let row = sqlx::query("SELECT id, name FROM character_sheets WHERE id = ?")
            .bind(&id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(row.get::<String, _>("id"), id);
        assert_eq!(row.get::<String, _>("name"), name);

        // 3. Update
        let mut updated_data = initial_data.clone();
        updated_data.name = "Updated Name".to_string();
        let updated_json = serde_json::to_string(&updated_data).unwrap();

        sqlx::query("UPDATE character_sheets SET name = ?, data = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(&updated_data.name)
            .bind(updated_json)
            .bind(&id)
            .execute(&pool)
            .await
            .unwrap();

        // 4. Verify Update
        let updated_row = sqlx::query("SELECT name FROM character_sheets WHERE id = ?")
            .bind(&id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(updated_row.get::<String, _>("name"), "Updated Name");

        // Cleanup
        drop(pool);
        if std::path::Path::new(db_path).exists() {
            std::fs::remove_file(db_path).ok();
        }
    }
}
