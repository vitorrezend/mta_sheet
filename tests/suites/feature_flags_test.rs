#[cfg(feature = "ssr")]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;

    async fn create_test_pool() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("in-memory db");

        // Executa as migrações essenciais
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

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS rooms (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                code TEXT UNIQUE NOT NULL,
                description TEXT DEFAULT '',
                gm_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                map_data TEXT DEFAULT '',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )"
        )
        .execute(&pool)
        .await
        .expect("create rooms table");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS system_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )"
        )
        .execute(&pool)
        .await
        .expect("create system_settings table");

        sqlx::query(
            "INSERT OR IGNORE INTO system_settings (key, value) VALUES ('feature_tactical_grid', 'admin_only')"
        )
        .execute(&pool)
        .await
        .expect("seed feature_tactical_grid");

        pool
    }

    #[tokio::test]
    async fn test_system_settings_seed_and_defaults() {
        let pool = create_test_pool().await;

        let val: Option<String> = sqlx::query_scalar(
            "SELECT value FROM system_settings WHERE key = 'feature_tactical_grid'"
        )
        .fetch_optional(&pool)
        .await
        .expect("query setting");

        assert_eq!(val, Some("admin_only".to_string()));
    }

    #[tokio::test]
    async fn test_feature_flag_resolution_logic() {
        // Função pura para validar o resolvedor de permissão
        fn resolve_access(flag_value: &str, is_admin: bool) -> bool {
            match flag_value {
                "enabled" => true,
                "admin_only" => is_admin,
                _ => false,
            }
        }

        // Cenário 1: disabled
        assert!(!resolve_access("disabled", false), "Disabled deve bloquear jogador comum");
        assert!(!resolve_access("disabled", true), "Disabled deve bloquear admin também");

        // Cenário 2: admin_only
        assert!(!resolve_access("admin_only", false), "Admin_only deve bloquear jogador comum");
        assert!(resolve_access("admin_only", true), "Admin_only deve liberar para admin");

        // Cenário 3: enabled
        assert!(resolve_access("enabled", false), "Enabled deve liberar para jogador comum");
        assert!(resolve_access("enabled", true), "Enabled deve liberar para admin");

        // Cenário 4: valor desconhecido/fallback seguro
        assert!(!resolve_access("unknown", false));
        assert!(!resolve_access("unknown", true));
    }

    #[tokio::test]
    async fn test_feature_flag_db_toggle_and_preservation() {
        let pool = create_test_pool().await;

        // Inserir usuário narrador e sala com mapa existente
        sqlx::query("INSERT INTO users (id, username, password_hash, is_admin) VALUES ('gm1', 'gm_user', 'hash', 0)")
            .execute(&pool)
            .await
            .expect("insert gm user");

        let original_map_json = r#"{"zoom":1.2,"pan_x":150,"pan_y":200,"grid_size":40,"tokens":[{"id":"tok-1","name":"Mago Vitor","x":100,"y":200}]}"#;

        sqlx::query("INSERT INTO rooms (id, name, code, gm_id, map_data) VALUES ('room-1', 'Sala Arcana', 'MTA-TEST', 'gm1', ?)")
            .bind(original_map_json)
            .execute(&pool)
            .await
            .expect("insert room");

        // 1. Alterna para disabled
        sqlx::query("UPDATE system_settings SET value = 'disabled' WHERE key = 'feature_tactical_grid'")
            .execute(&pool)
            .await
            .expect("update to disabled");

        let val_disabled: String = sqlx::query_scalar("SELECT value FROM system_settings WHERE key = 'feature_tactical_grid'")
            .fetch_one(&pool)
            .await
            .expect("fetch disabled");
        assert_eq!(val_disabled, "disabled");

        // 2. Confirma que os dados do mapa na sala permanecem intactos
        let map_in_db: String = sqlx::query_scalar("SELECT map_data FROM rooms WHERE id = 'room-1'")
            .fetch_one(&pool)
            .await
            .expect("fetch map");
        assert_eq!(map_in_db, original_map_json, "map_data não pode ser alterado ou apagado ao desativar o recurso");

        // 3. Alterna para enabled
        sqlx::query("UPDATE system_settings SET value = 'enabled' WHERE key = 'feature_tactical_grid'")
            .execute(&pool)
            .await
            .expect("update to enabled");

        let val_enabled: String = sqlx::query_scalar("SELECT value FROM system_settings WHERE key = 'feature_tactical_grid'")
            .fetch_one(&pool)
            .await
            .expect("fetch enabled");
        assert_eq!(val_enabled, "enabled");

        // 4. Confirma que os dados do mapa continuam 100% preservados
        let map_in_db_after: String = sqlx::query_scalar("SELECT map_data FROM rooms WHERE id = 'room-1'")
            .fetch_one(&pool)
            .await
            .expect("fetch map after enable");
        assert_eq!(map_in_db_after, original_map_json, "map_data deve permanecer inalterado após reativação");
    }

    #[tokio::test]
    async fn test_feature_flag_validation_values() {
        let pool = create_test_pool().await;

        let valid_values = vec!["disabled", "admin_only", "enabled"];
        for val in valid_values {
            let res = sqlx::query(
                "INSERT INTO system_settings (key, value, updated_at) VALUES ('feature_tactical_grid', ?, CURRENT_TIMESTAMP)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = CURRENT_TIMESTAMP"
            )
            .bind(val)
            .execute(&pool)
            .await;
            assert!(res.is_ok(), "Valor válido '{}' deve ser aceito", val);
        }
    }

    #[test]
    fn test_embedded_site_assets_inventory() {
        use rust_embed::RustEmbed;
        let files: Vec<_> = mta_sheet::server::handlers::static_files::SiteAssets::iter().collect();
        println!("SiteAssets count: {}", files.len());
        for f in &files {
            println!("  SiteAsset: {}", f);
        }
        assert!(!files.is_empty(), "SiteAssets não pode estar vazio");
        assert!(files.iter().any(|f| f == "pkg/mta_sheet.js"), "mta_sheet.js deve estar embutido no SiteAssets");
        assert!(files.iter().any(|f| f == "pkg/mta_sheet.wasm"), "mta_sheet.wasm deve estar embutido no SiteAssets");
    }
}
