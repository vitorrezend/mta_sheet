#[cfg(feature = "ssr")]
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqlitePool};
#[cfg(feature = "ssr")]
use std::str::FromStr;

#[cfg(feature = "ssr")]
pub async fn get_db() -> SqlitePool {
    let mut database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "mta_sheet.db".to_string());

    // Processar o caminho para log e criação de diretório
    let db_path = if database_url.starts_with("sqlite:") {
        database_url[7..].to_string()
    } else {
        database_url.clone()
    };

    // Garante que o diretório pai existe
    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).ok();
        }
    }

    if !database_url.starts_with("sqlite:") {
        database_url = format!("sqlite:{}", database_url);
    }

    if let Ok(abs_path) = std::fs::canonicalize(&db_path) {
        println!("Caminho absoluto do banco de dados: {:?}", abs_path);
    }
    println!("Conectando ao banco de dados: {}", database_url);

    let options = SqliteConnectOptions::from_str(&database_url)
        .expect("String de conexão DATABASE_URL inválida")
        .create_if_missing(true)
        .log_statements(log::LevelFilter::Debug);

    let pool = SqlitePool::connect_with(options).await.expect("Falha ao conectar ao SQLite");

    // Inicializa as tabelas
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

    // Adiciona índice para listagem rápida por data
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_character_sheets_updated_at ON character_sheets (updated_at DESC)"
    )
    .execute(&pool)
    .await
    .expect("Falha ao criar índice para character_sheets");

    pool
}

#[cfg(feature = "ssr")]
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_initialization() {
        let test_db = "test_init.db";
        // Limpeza inicial
        if std::path::Path::new(test_db).exists() {
            let _ = std::fs::remove_file(test_db);
        }

        // Em Rust Edition 2024, set_var é unsafe
        unsafe {
            std::env::set_var("DATABASE_URL", test_db);
        }

        let pool = get_db().await;

        // Verifica se a tabela foi criada
        let result = sqlx::query("SELECT count(*) FROM character_sheets")
            .fetch_one(&pool)
            .await;

        assert!(result.is_ok());

        // Limpeza final
        drop(pool);
        let _ = std::fs::remove_file(test_db);
    }

    #[tokio::test]
    async fn test_full_crud_flow() {
        let test_db = "test_crud.db";
        if std::path::Path::new(test_db).exists() {
            let _ = std::fs::remove_file(test_db);
        }

        unsafe {
            std::env::set_var("DATABASE_URL", test_db);
        }

        let pool = get_db().await;

        // CREATE
        let id = "test-uuid";
        let name = "Test Mage";
        let data = "{}";

        sqlx::query("INSERT INTO character_sheets (id, name, data) VALUES (?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(data)
            .execute(&pool)
            .await
            .unwrap();

        // READ
        let row: (String,) = sqlx::query_as("SELECT name FROM character_sheets WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row.0, name);

        // UPDATE
        let new_name = "Updated Mage";
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

        // DELETE
        sqlx::query("DELETE FROM character_sheets WHERE id = ?")
            .bind(id)
            .execute(&pool)
            .await
            .unwrap();

        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM character_sheets")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count.0, 0);

        drop(pool);
        let _ = std::fs::remove_file(test_db);
    }
}
