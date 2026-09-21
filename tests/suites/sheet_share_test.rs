use mta_sheet::state::models::{SheetAclEntry, SheetShareSettings};

#[test]
fn test_sheet_share_models_serialization() {
    let acl = SheetAclEntry {
        id: "acl-1".to_string(),
        sheet_id: "sheet-100".to_string(),
        grantee_type: "user".to_string(),
        grantee_id: Some("user-5".to_string()),
        grantee_name: "Hermes".to_string(),
        permission: "read".to_string(),
        created_at: "2026-09-19 10:00:00".to_string(),
    };

    let json = serde_json::to_string(&acl).expect("serialize acl");
    assert!(json.contains("\"grantee_name\":\"Hermes\""));
    assert!(json.contains("\"permission\":\"read\""));

    let deserialized: SheetAclEntry = serde_json::from_str(&json).expect("deserialize acl");
    assert_eq!(acl, deserialized);

    let settings = SheetShareSettings {
        sheet_id: "sheet-100".to_string(),
        sheet_name: "Dante Alighieri".to_string(),
        share_token: Some("token-secret-123".to_string()),
        share_permission: "view".to_string(),
        is_public: false,
        acls: vec![acl],
    };

    let json_settings = serde_json::to_string(&settings).expect("serialize settings");
    assert!(json_settings.contains("\"share_permission\":\"view\""));
    assert!(json_settings.contains("\"share_token\":\"token-secret-123\""));

    let deserialized_settings: SheetShareSettings = serde_json::from_str(&json_settings).expect("deserialize settings");
    assert_eq!(settings, deserialized_settings);
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_database_sheet_acls_schema_and_migrations() {
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::Row;

    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("in-memory db");

    mta_sheet::database::ensure_schema(&pool)
        .await
        .expect("ensure_schema should succeed");

    // Verifica se a tabela sheet_acls existe
    let table_exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='sheet_acls'"
    )
    .fetch_one(&pool)
    .await
    .expect("check table");
    assert_eq!(table_exists, 1, "Tabela sheet_acls deve existir no banco de dados");

    // Verifica se colunas evolutivas share_token e share_permission foram adicionadas
    let pragma_rows = sqlx::query("PRAGMA table_info(character_sheets)")
        .fetch_all(&pool)
        .await
        .expect("pragma character_sheets");

    let col_names: Vec<String> = pragma_rows.into_iter().map(|r| r.get("name")).collect();
    assert!(col_names.iter().any(|c| c.eq_ignore_ascii_case("share_token")), "character_sheets deve conter share_token");
    assert!(col_names.iter().any(|c| c.eq_ignore_ascii_case("share_permission")), "character_sheets deve conter share_permission");
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_sheet_acl_repository_grant_and_revoke() {
    use sqlx::sqlite::SqlitePoolOptions;
    use mta_sheet::repositories::SheetAclRepository;

    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("in-memory db");

    mta_sheet::database::ensure_schema(&pool)
        .await
        .expect("ensure_schema");

    // Setup: Usuários e Ficha
    sqlx::query("INSERT INTO users (id, username, password_hash) VALUES ('owner-1', 'mestre_vitor', 'hash')")
        .execute(&pool).await.expect("insert owner");
    sqlx::query("INSERT INTO users (id, username, password_hash) VALUES ('player-1', 'jogador_leo', 'hash')")
        .execute(&pool).await.expect("insert player");
    sqlx::query("INSERT INTO users (id, username, password_hash) VALUES ('player-2', 'jogador_carol', 'hash')")
        .execute(&pool).await.expect("insert player 2");

    sqlx::query(
        "INSERT INTO character_sheets (id, user_id, name, data, is_public) 
         VALUES ('sheet-1', 'owner-1', 'Dante', '{}', 0)"
    )
    .execute(&pool).await.expect("insert sheet");

    // 1. O proprietário tem permissão total (admin, write, read)
    assert!(SheetAclRepository::check_sheet_permission(&pool, "sheet-1", "owner-1", "admin").await.unwrap());
    assert!(SheetAclRepository::check_sheet_permission(&pool, "sheet-1", "owner-1", "write").await.unwrap());
    assert!(SheetAclRepository::check_sheet_permission(&pool, "sheet-1", "owner-1", "read").await.unwrap());

    // 2. Jogador comum não possui acesso inicialmente
    assert!(!SheetAclRepository::check_sheet_permission(&pool, "sheet-1", "player-1", "read").await.unwrap());
    assert!(!SheetAclRepository::check_sheet_permission(&pool, "sheet-1", "player-1", "write").await.unwrap());

    // 3. Conceder permissão 'read' (Leitor) ao player-1
    SheetAclRepository::grant(&pool, "acl-1", "sheet-1", "user", Some("player-1"), "read")
        .await
        .expect("grant read");

    assert!(SheetAclRepository::check_sheet_permission(&pool, "sheet-1", "player-1", "read").await.unwrap());
    assert!(!SheetAclRepository::check_sheet_permission(&pool, "sheet-1", "player-1", "write").await.unwrap(), "Leitor não pode ter permissão de escrita");

    // 4. Conceder permissão 'write' (Editor) ao player-2
    SheetAclRepository::grant(&pool, "acl-2", "sheet-1", "user", Some("player-2"), "write")
        .await
        .expect("grant write");

    assert!(SheetAclRepository::check_sheet_permission(&pool, "sheet-1", "player-2", "read").await.unwrap());
    assert!(SheetAclRepository::check_sheet_permission(&pool, "sheet-1", "player-2", "write").await.unwrap());

    // 5. Listar permissões com resolução de nomes de usuário
    let acls = SheetAclRepository::list_for_sheet(&pool, "sheet-1").await.expect("list");
    assert_eq!(acls.len(), 2);
    assert!(acls.iter().any(|a| a.grantee_name == "jogador_leo" && a.permission == "read"));
    assert!(acls.iter().any(|a| a.grantee_name == "jogador_carol" && a.permission == "write"));

    // 6. Revogar permissão
    SheetAclRepository::revoke(&pool, "acl-1").await.expect("revoke");
    assert!(!SheetAclRepository::check_sheet_permission(&pool, "sheet-1", "player-1", "read").await.unwrap(), "Permissão revogada deve bloquear acesso");
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_sheet_share_token_and_link_permission() {
    use sqlx::sqlite::SqlitePoolOptions;
    use mta_sheet::repositories::SheetAclRepository;

    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("in-memory db");

    mta_sheet::database::ensure_schema(&pool)
        .await
        .expect("ensure_schema");

    sqlx::query("INSERT INTO users (id, username, password_hash) VALUES ('u1', 'mestre', 'hash')")
        .execute(&pool).await.expect("insert user");

    sqlx::query(
        "INSERT INTO character_sheets (id, user_id, name, data, is_public, share_permission) 
         VALUES ('s1', 'u1', 'Fausto', '{}', 0, 'none')"
    )
    .execute(&pool).await.expect("insert sheet");

    // 1. Inicialmente share_permission é 'none', acesso por link bloqueado
    assert!(!SheetAclRepository::verify_token_or_public(&pool, "s1", None).await.unwrap());
    assert!(!SheetAclRepository::verify_token_or_public(&pool, "s1", Some("qualquer_token")).await.unwrap());

    // 2. Ativar compartilhamento por link ('view')
    let settings = SheetAclRepository::update_share_link(&pool, "s1", "view", true)
        .await
        .expect("update link")
        .expect("some settings");

    assert_eq!(settings.share_permission, "view");
    let token = settings.share_token.expect("token should be generated");
    assert!(!token.is_empty());

    // 3. Validação com o token correto concede acesso de visualização
    assert!(SheetAclRepository::verify_token_or_public(&pool, "s1", Some(&token)).await.unwrap());

    // 4. Token incorreto bloqueia acesso
    assert!(!SheetAclRepository::verify_token_or_public(&pool, "s1", Some("token-errado")).await.unwrap());

    // 5. Redefinir link (regenerar token) invalida o token antigo
    let updated = SheetAclRepository::update_share_link(&pool, "s1", "view", true)
        .await
        .expect("regenerate link")
        .expect("updated settings");

    let new_token = updated.share_token.unwrap();
    assert_ne!(token, new_token, "Novo token deve ser diferente do anterior");
    assert!(!SheetAclRepository::verify_token_or_public(&pool, "s1", Some(&token)).await.unwrap(), "Token antigo deve estar invalidado");
    assert!(SheetAclRepository::verify_token_or_public(&pool, "s1", Some(&new_token)).await.unwrap(), "Novo token deve conceder acesso");

    // 6. Desativar link ('none') bloqueia mesmo com o token correto
    SheetAclRepository::update_share_link(&pool, "s1", "none", false).await.unwrap();
    assert!(!SheetAclRepository::verify_token_or_public(&pool, "s1", Some(&new_token)).await.unwrap(), "Link desativado bloqueia acesso");
}
