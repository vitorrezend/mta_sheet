use sqlx::SqlitePool;

/// Banco novo: o esquema base é criado e as migrações aplicadas uma única vez.
#[tokio::test]
async fn test_migrations_apply_once_after_baseline_schema() {
    let pool = SqlitePool::connect("sqlite::memory:").await.expect("sqlite em memória");
    mta_sheet::database::ensure_schema(&pool).await.expect("esquema base");
    mta_sheet::database::run_migrations(&pool).await.expect("primeira execução");
    mta_sheet::database::run_migrations(&pool).await.expect("segunda execução não deve falhar");

    let applied: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM _sqlx_migrations WHERE success = 1")
        .fetch_one(&pool)
        .await
        .expect("tabela de controle do sqlx");
    assert_eq!(applied, mta_sheet::database::MIGRATOR.iter().count() as i64, "cada migração registrada exatamente uma vez");
}

/// Banco "antigo" (criado antes do histórico de migrações): continua funcionando e
/// ganha as colunas novas pelo esquema base antes das migrações.
#[tokio::test]
async fn test_migrations_on_preexisting_database() {
    let pool = SqlitePool::connect("sqlite::memory:").await.expect("sqlite em memória");
    sqlx::query("CREATE TABLE character_sheets (id TEXT PRIMARY KEY, name TEXT NOT NULL, data TEXT NOT NULL)")
        .execute(&pool)
        .await
        .expect("tabela legada");
    mta_sheet::database::ensure_schema(&pool).await.expect("esquema base sobre banco legado");
    mta_sheet::database::run_migrations(&pool).await.expect("migrações sobre banco legado");

    let version: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pragma_table_info('character_sheets') WHERE name = 'version'")
        .fetch_one(&pool)
        .await
        .expect("pragma");
    assert_eq!(version, 1, "coluna de versão adicionada ao banco legado");
}
