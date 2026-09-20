use mta_sheet::auth::{create_session_with_fifo, MAX_SESSIONS_PER_USER};
use mta_sheet::database::ensure_schema;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Row;
use uuid::Uuid;

#[tokio::test]
async fn test_fifo_session_policy_limits_to_max_sessions() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory SQLite");

    ensure_schema(&pool)
        .await
        .expect("Failed to ensure schema in test db");

    let test_user_id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO users (id, username, password_hash, is_admin) VALUES (?, ?, ?, 0)")
        .bind(&test_user_id)
        .bind("test_wizard")
        .bind("dummy_hash")
        .execute(&pool)
        .await
        .expect("Failed to create test user");

    let mut generated_tokens = Vec::new();

    // Simula 8 logins consecutivos do mesmo usuário
    for _ in 0..8 {
        let token = create_session_with_fifo(&pool, &test_user_id)
            .await
            .expect("Failed to create session with FIFO");
        generated_tokens.push(token);
    }

    // Verifica que a contagem total de sessões ativas nunca ultrapassa MAX_SESSIONS_PER_USER (5)
    let count_row = sqlx::query("SELECT COUNT(*) as count FROM sessions WHERE user_id = ?")
        .bind(&test_user_id)
        .fetch_one(&pool)
        .await
        .expect("Failed to query session count");
    let count: i64 = count_row.get("count");

    assert_eq!(
        count as usize, MAX_SESSIONS_PER_USER,
        "A contagem de sessões ativas deve ser exatamente MAX_SESSIONS_PER_USER ({})",
        MAX_SESSIONS_PER_USER
    );

    // As 3 primeiras sessões devem ter sido eliminadas (FIFO: First In, First Out)
    for old_token in &generated_tokens[0..3] {
        let exists = sqlx::query("SELECT id FROM sessions WHERE id = ?")
            .bind(old_token)
            .fetch_optional(&pool)
            .await
            .expect("Failed to query session token");
        assert!(
            exists.is_none(),
            "A sessão antiga {} deveria ter sido revogada pela política FIFO!",
            old_token
        );
    }

    // As 5 últimas sessões geradas devem continuar ativas
    for active_token in &generated_tokens[3..8] {
        let exists = sqlx::query("SELECT id FROM sessions WHERE id = ?")
            .bind(active_token)
            .fetch_optional(&pool)
            .await
            .expect("Failed to query session token");
        assert!(
            exists.is_some(),
            "A sessão recente {} deveria permanecer ativa!",
            active_token
        );
    }
}
