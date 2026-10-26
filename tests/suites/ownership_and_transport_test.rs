//! Fichas sem dono são somente leitura; HSTS e cookie `Secure` seguem a mesma regra
//! nos dois caminhos de login.

use mta_sheet::auth::{cookie_should_be_secure, session_cookie};
use mta_sheet::database::ensure_schema;
use mta_sheet::repositories::sheet_repository::OWNERLESS_SHEET_READ_ONLY;
use mta_sheet::repositories::SheetRepository;
use mta_sheet::server::middleware::security::HSTS_VALUE;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

async fn pool_with_sheets() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory SQLite");
    ensure_schema(&pool).await.expect("Failed to ensure schema in test db");

    sqlx::query("INSERT INTO users (id, username, password_hash, is_admin) VALUES ('owner', 'dono', 'x', 0), ('other', 'outro', 'x', 0)")
        .execute(&pool)
        .await
        .expect("Failed to create users");
    sqlx::query("INSERT INTO character_sheets (id, user_id, name, data) VALUES ('owned', 'owner', 'Com dono', '{}'), ('orphan', NULL, 'Sem dono', '{}')")
        .execute(&pool)
        .await
        .expect("Failed to create sheets");
    pool
}

#[tokio::test]
async fn test_ownerless_sheets_are_read_only_for_everyone() {
    let pool = pool_with_sheets().await;

    for caller in [None, Some("owner"), Some("other")] {
        let result = SheetRepository::verify_write_permission(&pool, "orphan", caller)
            .await
            .expect("query");
        assert_eq!(result, Err(OWNERLESS_SHEET_READ_ONLY), "ficha sem dono não pode ser alterada por {:?}", caller);
    }
}

#[tokio::test]
async fn test_owned_sheets_keep_their_permissions() {
    let pool = pool_with_sheets().await;

    assert_eq!(SheetRepository::verify_write_permission(&pool, "owned", Some("owner")).await.expect("query"), Ok(()));
    assert!(SheetRepository::verify_write_permission(&pool, "owned", Some("other")).await.expect("query").is_err());
    assert!(SheetRepository::verify_write_permission(&pool, "owned", None).await.expect("query").is_err());
}

#[test]
fn test_hsts_has_no_preload() {
    assert!(HSTS_VALUE.contains("max-age="));
    assert!(!HSTS_VALUE.contains("preload"), "preload é difícil de desfazer: {}", HSTS_VALUE);
}

#[test]
fn test_session_cookie_secure_rule() {
    // Produção ou HTTPS informado pelo proxy: Secure. Desenvolvimento por HTTP: não
    assert!(cookie_should_be_secure(true, false, None));
    assert!(cookie_should_be_secure(false, true, None));
    assert!(cookie_should_be_secure(false, false, Some("https")));
    assert!(cookie_should_be_secure(false, false, Some(" HTTPS ")));
    assert!(!cookie_should_be_secure(false, false, Some("http")));
    assert!(!cookie_should_be_secure(false, false, None));

    let secure = session_cookie("tok", 60, true);
    assert!(secure.contains("HttpOnly") && secure.contains("SameSite=Lax") && secure.contains("; Secure"));
    assert!(!session_cookie("tok", 60, false).contains("Secure"));
}
