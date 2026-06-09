#[cfg(feature = "ssr")]
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqlitePool};
#[cfg(feature = "ssr")]
use std::str::FromStr;

#[cfg(feature = "ssr")]
pub async fn get_db() -> SqlitePool {
    use dotenvy::dotenv;
    let _ = dotenv();

    let mut database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "mta_sheet.db".to_string());

    if !database_url.starts_with("sqlite:") {
        database_url = format!("sqlite:{}", database_url);
    }

    println!("Iniciando conexão com o banco de dados: {}", database_url);

    let options = match SqliteConnectOptions::from_str(&database_url) {
        Ok(opt) => opt.create_if_missing(true).log_statements(log::LevelFilter::Debug),
        Err(e) => {
            eprintln!("ERRO: DATABASE_URL inválida ({}): {}", database_url, e);
            panic!("Configuração de banco de dados inválida");
        }
    };

    let pool = match SqlitePool::connect_with(options).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("ERRO: Falha ao conectar ao SQLite ({}): {}", database_url, e);
            panic!("Falha na conexão com o banco de dados");
        }
    };

    // Inicialização das tabelas
    if let Err(e) = sqlx::query(
        "CREATE TABLE IF NOT EXISTS character_sheets (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            data TEXT NOT NULL,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await {
        eprintln!("ERRO: Falha ao criar tabela 'character_sheets': {}", e);
        panic!("Falha na inicialização do esquema do banco de dados");
    }

    println!("Banco de dados inicializado com sucesso.");
    pool
}
