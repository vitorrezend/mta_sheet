use mta_sheet::repositories::RoomRepository;
use sqlx::SqlitePool;

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.expect("Failed to connect to in-memory SQLite");
    mta_sheet::database::ensure_schema(&pool).await.expect("Failed to initialize database");
    pool
}

/// update_room_chantry e update_room_chronicle_notes só permitem edição pelo mestre e membros.
#[tokio::test]
async fn test_room_notes_editable_only_by_gm_and_members() {
    let pool = setup_test_db().await;

    for (uid, uname) in [("gm-1", "mestre"), ("player-1", "jogador"), ("outsider-1", "intruso")] {
        sqlx::query("INSERT INTO users (id, username, password_hash) VALUES (?, ?, 'hash')")
            .bind(uid)
            .bind(uname)
            .execute(&pool)
            .await
            .expect("insert user");
    }

    sqlx::query("INSERT INTO rooms (id, name, code, gm_id) VALUES ('room-1', 'Crônica', 'MTA-TEST', 'gm-1')")
        .execute(&pool)
        .await
        .expect("insert room");
    RoomRepository::add_member_if_not_exists(&pool, "room-1", "player-1", "player")
        .await
        .expect("add member");

    let can = |user: &'static str, room: &'static str| {
        let pool = pool.clone();
        async move { RoomRepository::check_is_gm_or_member(&pool, room, user).await.expect("query") }
    };

    assert!(can("gm-1", "room-1").await, "o mestre deve poder editar");
    assert!(can("player-1", "room-1").await, "membros devem poder editar");
    assert!(!can("outsider-1", "room-1").await, "quem não é da sala não pode editar");
    assert!(!can("player-1", "room-inexistente").await, "sala inexistente nunca autoriza");
}
