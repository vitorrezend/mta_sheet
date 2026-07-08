#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_database_initialization() {
    use mta_sheet::database::get_db;
    use std::fs;
    use std::path::Path;

    // 1. Test default creation
    let _ = fs::remove_file("mta_sheet.db");
    let pool = get_db().await;
    assert!(Path::new("mta_sheet.db").exists(), "Default database file should be created");

    // Verify table exists
    let res = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='character_sheets'")
        .fetch_one(&pool)
        .await;
    assert!(res.is_ok(), "Table character_sheets should exist");
    drop(pool);

    // 2. Test custom creation via environment variable
    let custom_db = "custom_test.db";
    let _ = fs::remove_file(custom_db);
    unsafe {
        std::env::set_var("DATABASE_URL", custom_db);
    }

    let pool_custom = get_db().await;
    assert!(Path::new(custom_db).exists(), "Custom database file should be created");

    let res_custom = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='character_sheets'")
        .fetch_one(&pool_custom)
        .await;
    assert!(res_custom.is_ok(), "Table character_sheets should exist in custom db");

    // Clean up
    drop(pool_custom);
    let _ = fs::remove_file(custom_db);
    let _ = fs::remove_file("mta_sheet.db");
}
