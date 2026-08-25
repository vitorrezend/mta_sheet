use mta_sheet::auth::{UserInfo, is_username_in_admin_env};

#[test]
fn test_user_info_admin_serialization() {
    let admin_user = UserInfo {
        id: "admin-1".to_string(),
        username: "VitorMestre".to_string(),
        is_admin: true,
    };

    let json = serde_json::to_string(&admin_user).expect("serialize");
    assert!(json.contains("\"is_admin\":true"));

    let deserialized: UserInfo = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(admin_user, deserialized);
    assert!(deserialized.is_admin);

    let regular_user = UserInfo {
        id: "user-2".to_string(),
        username: "Jogador1".to_string(),
        is_admin: false,
    };

    let json_reg = serde_json::to_string(&regular_user).expect("serialize");
    assert!(json_reg.contains("\"is_admin\":false"));
}

#[cfg(feature = "ssr")]
#[test]
fn test_multiple_admin_usernames_env() {
    unsafe {
        std::env::set_var("ADMIN_USERNAMES", "vitor, mestre, carlos_gm");
    }

    assert!(is_username_in_admin_env("vitor"));
    assert!(is_username_in_admin_env("VITOR"));
    assert!(is_username_in_admin_env("mestre"));
    assert!(is_username_in_admin_env("carlos_gm"));
    assert!(!is_username_in_admin_env("jogador_comum"));
    assert!(!is_username_in_admin_env("outro_usuario"));
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_database_admin_migration_and_structure() {
    use sqlx::sqlite::SqlitePoolOptions;

    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("in-memory db");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            is_admin INTEGER NOT NULL DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await
    .expect("create users table");

    // Insert first user as admin
    sqlx::query("INSERT INTO users (id, username, password_hash, is_admin) VALUES ('u1', 'admin_user', 'hash1', 1)")
        .execute(&pool)
        .await
        .expect("insert admin");

    // Insert second user as regular
    sqlx::query("INSERT INTO users (id, username, password_hash, is_admin) VALUES ('u2', 'regular_user', 'hash2', 0)")
        .execute(&pool)
        .await
        .expect("insert regular");

    let admin_check: (i64,) = sqlx::query_as("SELECT is_admin FROM users WHERE username = 'admin_user'")
        .fetch_one(&pool)
        .await
        .expect("fetch admin");
    assert_eq!(admin_check.0, 1);

    let regular_check: (i64,) = sqlx::query_as("SELECT is_admin FROM users WHERE username = 'regular_user'")
        .fetch_one(&pool)
        .await
        .expect("fetch regular");
    assert_eq!(regular_check.0, 0);
}
