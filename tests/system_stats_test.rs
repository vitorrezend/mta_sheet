use mta_sheet::database::ensure_schema;
use mta_sheet::repositories::SheetRepository;
use mta_sheet::state::models::SystemStats;
use sqlx::sqlite::SqlitePoolOptions;
use uuid::Uuid;

#[tokio::test]
async fn test_system_stats_aggregation_accuracy() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory SQLite");

    ensure_schema(&pool)
        .await
        .expect("Failed to ensure schema in test db");

    // 1. Criar 3 usuários de teste
    let u1 = Uuid::new_v4().to_string();
    let u2 = Uuid::new_v4().to_string();
    let u3 = Uuid::new_v4().to_string();

    for (uid, name) in [(&u1, "hermes"), (&u2, "thoth"), (&u3, "agrippa")] {
        sqlx::query("INSERT INTO users (id, username, password_hash, is_admin) VALUES (?, ?, 'hash', 0)")
            .bind(uid)
            .bind(name)
            .execute(&pool)
            .await
            .expect("Failed to insert user");
    }

    // 2. Criar 2 salas de teste
    let r1 = Uuid::new_v4().to_string();
    let r2 = Uuid::new_v4().to_string();
    for (rid, code, name) in [(&r1, "MTA-0001", "Cabala Hermética"), (&r2, "MTA-0002", "Dô Akashayana")] {
        sqlx::query("INSERT INTO rooms (id, name, code, gm_id) VALUES (?, ?, ?, ?)")
            .bind(rid)
            .bind(name)
            .bind(code)
            .bind(&u1)
            .execute(&pool)
            .await
            .expect("Failed to insert room");
    }

    // 3. Inserir fichas M20:
    // - 2 públicas
    // - 3 privadas
    // - 1 com sheet_type vazio (legado, deve contar como M20 privada)
    for _ in 0..2 {
        let s_id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO character_sheets (id, user_id, name, data, sheet_type, is_public) VALUES (?, ?, 'Mago Pub', '{}', 'mage', 1)")
            .bind(&s_id)
            .bind(&u1)
            .execute(&pool)
            .await
            .expect("Failed to insert m20 pub");
    }

    for _ in 0..3 {
        let s_id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO character_sheets (id, user_id, name, data, sheet_type, is_public) VALUES (?, ?, 'Mago Priv', '{}', 'mage', 0)")
            .bind(&s_id)
            .bind(&u2)
            .execute(&pool)
            .await
            .expect("Failed to insert m20 priv");
    }

    let legacy_id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO character_sheets (id, user_id, name, data, sheet_type, is_public) VALUES (?, ?, 'Mago Legado', '{}', '', 0)")
        .bind(&legacy_id)
        .bind(&u3)
        .execute(&pool)
        .await
        .expect("Failed to insert legacy m20 sheet");

    // 4. Inserir fichas Gods & Monsters:
    // - 1 pública
    // - 2 privadas
    let gm_pub_id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO character_sheets (id, user_id, name, data, sheet_type, is_public) VALUES (?, ?, 'Familiar Pub', '{}', 'gods_and_monsters', 1)")
        .bind(&gm_pub_id)
        .bind(&u1)
        .execute(&pool)
        .await
        .expect("Failed to insert gm pub");

    for _ in 0..2 {
        let s_id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO character_sheets (id, user_id, name, data, sheet_type, is_public) VALUES (?, ?, 'Monstro Priv', '{}', 'gods_and_monsters', 0)")
            .bind(&s_id)
            .bind(&u2)
            .execute(&pool)
            .await
            .expect("Failed to insert gm priv");
    }

    // 5. Consulta estatísticas via SheetRepository
    let stats = SheetRepository::get_system_stats(&pool)
        .await
        .expect("Failed to query get_system_stats");

    // Validações
    assert_eq!(stats.total_users, 3, "Total de usuários deve ser 3");
    assert_eq!(stats.total_rooms, 2, "Total de salas deve ser 2");
    assert_eq!(stats.mage_sheets_public, 2, "Fichas M20 públicas devem ser 2");
    assert_eq!(stats.mage_sheets_private, 4, "Fichas M20 privadas devem ser 4 (3 normais + 1 com sheet_type vazio)");
    assert_eq!(stats.total_mage(), 6, "Total de M20 deve ser 6");

    assert_eq!(stats.gm_sheets_public, 1, "Fichas G&M públicas devem ser 1");
    assert_eq!(stats.gm_sheets_private, 2, "Fichas G&M privadas devem ser 2");
    assert_eq!(stats.total_gm(), 3, "Total de G&M deve ser 3");

    assert_eq!(stats.total_sheets(), 9, "Total de fichas deve ser 9");
    assert_eq!(stats.total_public(), 3, "Total de públicas deve ser 3");
    assert_eq!(stats.total_private(), 6, "Total de privadas deve ser 6");

    // Valida serialização / deserialização JSON para Server Functions Leptos
    let json = serde_json::to_string(&stats).expect("Failed to serialize SystemStats");
    let deserialized: SystemStats = serde_json::from_str(&json).expect("Failed to deserialize SystemStats");
    assert_eq!(stats, deserialized);
}

#[test]
fn test_system_stats_styles_and_tokens_integrity() {
    let home_css = std::fs::read_to_string("styles/home.css").expect("styles/home.css deve existir");

    // Estrutura de classes dos cards
    assert!(home_css.contains(".home-stats-section"), "styles/home.css deve conter .home-stats-section");
    assert!(home_css.contains(".stats-live-badge"), "styles/home.css deve conter .stats-live-badge");
    assert!(home_css.contains(".live-pulse-dot"), "styles/home.css deve conter .live-pulse-dot");
    assert!(home_css.contains(".home-stats-grid"), "styles/home.css deve conter .home-stats-grid");
    assert!(home_css.contains(".stat-card"), "styles/home.css deve conter .stat-card");
    assert!(home_css.contains(".stat-card-number"), "styles/home.css deve conter .stat-card-number");
    assert!(home_css.contains(".stat-pill-public"), "styles/home.css deve conter .stat-pill-public");
    assert!(home_css.contains(".stat-pill-private"), "styles/home.css deve conter .stat-pill-private");

    // Tokens canônicos utilizados
    assert!(home_css.contains("var(--purple-badge-bg"), "Deve usar token canônico purple-badge-bg");
    assert!(home_css.contains("var(--purple-accent"), "Deve usar token canônico purple-accent");
    assert!(home_css.contains("var(--surface-paper"), "Deve usar token canônico surface-paper");
    assert!(home_css.contains("var(--color-success"), "Deve usar token canônico color-success");

    // Responsividade mobile (1 coluna no mobile e 2 no tablet)
    assert!(home_css.contains(".home-stats-grid") && home_css.contains("grid-template-columns: repeat(4, 1fr)"), "Desktop deve ter 4 colunas");
    assert!(home_css.contains("grid-template-columns: repeat(2, 1fr)"), "Tablet deve ter 2 colunas");
    assert!(home_css.contains("grid-template-columns: 1fr"), "Mobile deve ajustar para 1 coluna");
}
