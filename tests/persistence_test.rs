#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_persistence() {
    use mta_sheet::database::get_db;
    use sqlx::Row;
    use std::env;

    // Use a temporary database for testing
    unsafe {
        env::set_var("DATABASE_URL", "sqlite:test_persistence.db");
    }

    let pool = get_db().await;

    // Test insert
    let id = "test-id-123";
    let name = "Test Character";
    let data = "{\"name\": \"Test Character\"}";

    sqlx::query("INSERT INTO character_sheets (id, name, data) VALUES (?, ?, ?)")
        .bind(id)
        .bind(name)
        .bind(data)
        .execute(&pool)
        .await
        .expect("Failed to insert test data");

    // Test select
    let row = sqlx::query("SELECT name FROM character_sheets WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch test data");

    let fetched_name: String = row.get("name");
    assert_eq!(fetched_name, name);

    // Clean up
    pool.close().await;
    let _ = std::fs::remove_file("test_persistence.db");
}
