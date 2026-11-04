use mta_sheet::auth::{username_taken, validate_username};
use mta_sheet::server::handlers::auth::form_url_encode;
use sqlx::SqlitePool;

async fn migrated_pool() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.expect("sqlite em memória");
    mta_sheet::database::ensure_schema(&pool).await.expect("esquema base");
    mta_sheet::database::run_migrations(&pool).await.expect("migrações");
    pool
}

async fn insert_user(pool: &SqlitePool, id: &str, username: &str, is_admin: i64) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (id, username, password_hash, is_admin) VALUES (?, ?, 'hash', ?)")
        .bind(id)
        .bind(username)
        .bind(is_admin)
        .execute(pool)
        .await
        .map(|_| ())
}

#[test]
fn test_validate_username_rules() {
    assert!(validate_username("vitor").is_ok());
    assert!(validate_username("Mestre_GM-01.x").is_ok());
    assert!(validate_username(&"a".repeat(32)).is_ok());

    assert!(validate_username("ab").is_err(), "curto demais");
    assert!(validate_username(&"a".repeat(33)).is_err(), "longo demais");
    assert!(validate_username("vitor rezende").is_err(), "espaço");
    assert!(validate_username("vítor").is_err(), "acento");
    assert!(validate_username("v\u{0456}tor").is_err(), "i cirílico parecido com o latino");
    assert!(validate_username("<script>").is_err(), "HTML");
}

/// Com "vitor" cadastrado, "VITOR" ou "Vitor" não podem virar uma segunda conta
/// (herdariam o papel de admin de ADMIN_USERNAMES e confundiriam o login).
#[tokio::test]
async fn test_username_unique_ignoring_case() {
    let pool = migrated_pool().await;
    insert_user(&pool, "u1", "vitor", 1).await.expect("primeira conta");

    assert!(username_taken(&pool, "vitor").await.expect("consulta"));
    assert!(username_taken(&pool, "VITOR").await.expect("consulta"));
    assert!(username_taken(&pool, "Vitor").await.expect("consulta"));
    assert!(!username_taken(&pool, "vitor2").await.expect("consulta"));

    let dup = insert_user(&pool, "u2", "VITOR", 0).await;
    assert!(dup.is_err(), "o índice único deve barrar o nome com outras maiúsculas");
    assert!(dup.err().map(|e| e.to_string()).unwrap_or_default().contains("UNIQUE"));
}

/// Banco existente com nomes repetidos: a conta mais antiga mantém o nome; as outras
/// são renomeadas e perdem o admin, e a partir daí o índice impede novas colisões.
#[tokio::test]
async fn test_migration_renames_existing_case_duplicates() {
    let pool = SqlitePool::connect("sqlite::memory:").await.expect("sqlite em memória");
    mta_sheet::database::ensure_schema(&pool).await.expect("esquema base");
    insert_user(&pool, "aaaaaaaa-1111", "vitor", 1).await.expect("original");
    insert_user(&pool, "bbbbbbbb-2222", "VITOR", 1).await.expect("cópia (antes do índice)");
    insert_user(&pool, "cccccccc-3333", "jogador", 0).await.expect("outro");

    mta_sheet::database::run_migrations(&pool).await.expect("migrações sobre banco com duplicatas");

    let rows: Vec<(String, String, i64)> = sqlx::query_as("SELECT id, username, is_admin FROM users ORDER BY rowid")
        .fetch_all(&pool)
        .await
        .expect("usuários");
    assert_eq!(rows[0], ("aaaaaaaa-1111".to_string(), "vitor".to_string(), 1));
    assert_eq!(rows[1], ("bbbbbbbb-2222".to_string(), "VITOR_bbbbbbbb".to_string(), 0));
    assert_eq!(rows[2], ("cccccccc-3333".to_string(), "jogador".to_string(), 0));

    assert!(insert_user(&pool, "dddddddd-4444", "Vitor", 0).await.is_err());
}

#[test]
fn test_form_url_encode() {
    assert_eq!(form_url_encode("Nome de usuário"), "Nome+de+usu%C3%A1rio");
    assert_eq!(form_url_encode("a&b=c"), "a%26b%3Dc");
}
