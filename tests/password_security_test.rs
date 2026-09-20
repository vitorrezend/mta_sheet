#[cfg(feature = "ssr")]
mod ssr_tests {
    use sqlx::SqlitePool;
    use uuid::Uuid;
    use mta_sheet::state::models::summary::UserProfileData;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.expect("Failed to connect to in-memory SQLite");
        mta_sheet::database::ensure_schema(&pool).await.expect("Failed to initialize database");
        pool
    }

    #[test]
    fn test_password_complexity_rules() {
        use mta_sheet::auth::validate_password_strength;

        // Senhas fracas / inválidas
        assert_eq!(
            validate_password_strength("short1"),
            Err("A senha deve ter no mínimo 8 caracteres.")
        );
        assert_eq!(
            validate_password_strength("alllettersonly"),
            Err("A senha deve conter pelo menos uma letra e um número.")
        );
        assert_eq!(
            validate_password_strength("1234567890"),
            Err("A senha deve conter pelo menos uma letra e um número.")
        );
        assert_eq!(
            validate_password_strength(&"A".repeat(129)),
            Err("A senha não pode exceder 128 caracteres.")
        );

        // Senhas válidas (>= 8 chars, com letra e número)
        assert!(validate_password_strength("Hermes2026").is_ok());
        assert!(validate_password_strength("mago_arete_10").is_ok());
        assert!(validate_password_strength("SuperSecret#42").is_ok());
    }

    #[test]
    fn test_auth_rate_limiting_policy() {
        use std::time::Duration;
        use mta_sheet::server::handlers::auth::{check_auth_rate_limit, reset_auth_rate_limit_for_test};

        reset_auth_rate_limit_for_test();

        let attacker_ip = "198.51.100.42";
        let legit_ip = "203.0.113.10";

        // Simula 5 tentativas permitidas para cadastro
        for i in 1..=5 {
            assert!(
                check_auth_rate_limit(attacker_ip, 5, Duration::from_secs(60)),
                "Tentativa {} dentro do limite deveria ser aceita",
                i
            );
        }

        // 6ª tentativa consecutiva deve ser rejeitada pelo rate limiter
        assert!(
            !check_auth_rate_limit(attacker_ip, 5, Duration::from_secs(60)),
            "6ª tentativa do invasor deveria ser bloqueada"
        );

        // Outro IP legítimo não deve ser afetado pelo bloqueio do invasor
        assert!(
            check_auth_rate_limit(legit_ip, 5, Duration::from_secs(60)),
            "IP legítimo deve continuar funcionando normalmente"
        );

        reset_auth_rate_limit_for_test();
    }

    #[tokio::test]
    async fn test_password_update_bcrypt_and_session_revocation() {
        let pool = setup_test_db().await;

        let user_id = Uuid::new_v4().to_string();
        let initial_pwd = "OldPassword123";
        let initial_hash = bcrypt::hash(initial_pwd, 12).expect("Bcrypt hash inicial");

        // 1. Criar usuário no banco com hash Bcrypt
        sqlx::query("INSERT INTO users (id, username, password_hash) VALUES (?, 'hermetic_mage', ?)")
            .bind(&user_id)
            .bind(&initial_hash)
            .execute(&pool)
            .await
            .unwrap();

        // 2. Criar 3 sessões ativas para o usuário (dispositivo atual + 2 remotos)
        let current_session_id = "sess_current_browser".to_string();
        let remote_session_1 = "sess_phone_mobile".to_string();
        let remote_session_2 = "sess_work_laptop".to_string();

        for s_id in [&current_session_id, &remote_session_1, &remote_session_2] {
            sqlx::query("INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, CURRENT_TIMESTAMP)")
                .bind(s_id)
                .bind(&user_id)
                .execute(&pool)
                .await
                .unwrap();
        }

        // Verificar 3 sessões ativas
        let initial_sessions: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sessions WHERE user_id = ?")
            .bind(&user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(initial_sessions, 3);

        // 3. Simular verificação da senha atual
        let db_pwd_hash: String = sqlx::query_scalar("SELECT password_hash FROM users WHERE id = ?")
            .bind(&user_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        // Senha errada deve ser rejeitada
        assert!(!bcrypt::verify("WrongPassword99", &db_pwd_hash).unwrap());

        // Senha correta deve ser aprovada
        assert!(bcrypt::verify(initial_pwd, &db_pwd_hash).unwrap());

        // 4. Executar fluxo de troca de senha
        let new_pwd = "NewSecurePassword456";
        assert!(mta_sheet::auth::validate_password_strength(new_pwd).is_ok());

        let new_hash = bcrypt::hash(new_pwd, 12).expect("Bcrypt new hash");

        sqlx::query("UPDATE users SET password_hash = ? WHERE id = ?")
            .bind(&new_hash)
            .bind(&user_id)
            .execute(&pool)
            .await
            .unwrap();

        // 5. Revogação de segurança de todas as outras sessões (exceto a sessão atual)
        let deleted_rows = sqlx::query("DELETE FROM sessions WHERE user_id = ? AND id != ?")
            .bind(&user_id)
            .bind(&current_session_id)
            .execute(&pool)
            .await
            .unwrap()
            .rows_affected();

        assert_eq!(deleted_rows, 2, "As duas outras sessões remotas devem ter sido derrubadas");

        // Apenas a sessão atual deve restar
        let remaining_sessions: Vec<String> = sqlx::query_scalar("SELECT id FROM sessions WHERE user_id = ?")
            .bind(&user_id)
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(remaining_sessions.len(), 1);
        assert_eq!(remaining_sessions[0], current_session_id);

        // 6. Verificar que o novo hash autentica e o antigo não funciona mais
        let updated_hash: String = sqlx::query_scalar("SELECT password_hash FROM users WHERE id = ?")
            .bind(&user_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert!(!bcrypt::verify(initial_pwd, &updated_hash).unwrap());
        assert!(bcrypt::verify(new_pwd, &updated_hash).unwrap());
    }

    #[tokio::test]
    async fn test_revoke_other_sessions_endpoint_logic() {
        let pool = setup_test_db().await;

        let user_id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO users (id, username, password_hash) VALUES (?, 'cyber_virtualist', 'hash')")
            .bind(&user_id)
            .execute(&pool)
            .await
            .unwrap();

        let my_session = "my_active_session".to_string();
        let other_session = "unwanted_session".to_string();

        for s_id in [&my_session, &other_session] {
            sqlx::query("INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, CURRENT_TIMESTAMP)")
                .bind(s_id)
                .bind(&user_id)
                .execute(&pool)
                .await
                .unwrap();
        }

        // Revogar outras sessões
        let affected = sqlx::query("DELETE FROM sessions WHERE user_id = ? AND id != ?")
            .bind(&user_id)
            .bind(&my_session)
            .execute(&pool)
            .await
            .unwrap()
            .rows_affected();

        assert_eq!(affected, 1);

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sessions WHERE user_id = ?")
            .bind(&user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_user_profile_data_comprehensive_metrics() {
        let profile = UserProfileData {
            id: "u-100".to_string(),
            username: "VerbenaMaster".to_string(),
            created_at: "2026-09-19".to_string(),
            is_self: true,
            is_admin: true,
            total_sheets: 10,
            public_sheets_count: 4,
            private_sheets_count: 6,
            mage_sheets_count: 8,
            gods_monsters_sheets_count: 2,
            folders_count: 3,
            rooms_count: 5,
            gm_rooms_count: 2,
            player_rooms_count: 3,
            active_sessions_count: 1,
            sheets: Vec::new(),
        };

        let json = serde_json::to_string(&profile).expect("serialize profile");
        assert!(json.contains("\"is_admin\":true"));
        assert!(json.contains("\"private_sheets_count\":6"));
        assert!(json.contains("\"mage_sheets_count\":8"));
        assert!(json.contains("\"gods_monsters_sheets_count\":2"));
        assert!(json.contains("\"active_sessions_count\":1"));

        let deserialized: UserProfileData = serde_json::from_str(&json).expect("deserialize profile");
        assert_eq!(profile, deserialized);
    }
}
