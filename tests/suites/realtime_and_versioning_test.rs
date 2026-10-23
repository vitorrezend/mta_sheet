use mta_sheet::repositories::SheetRepository;
use mta_sheet::rooms::events::{get_or_create_room_channel, join_presence, notify_sheet_changed, online_users};
use mta_sheet::rooms::RoomBroadcastEvent;
use sqlx::SqlitePool;

async fn setup_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.expect("sqlite em memória");
    mta_sheet::database::ensure_schema(&pool).await.expect("schema");
    sqlx::query("INSERT INTO users (id, username, password_hash) VALUES ('u1', 'mestre', 'hash')")
        .execute(&pool).await.expect("usuário");
    pool
}

async fn insert_sheet(pool: &SqlitePool, id: &str, room_id: Option<&str>, hidden: bool) {
    let data = serde_json::to_string(&mta_sheet::state::CharacterData::new(id.to_string(), "Herói".to_string())).expect("json");
    sqlx::query("INSERT INTO character_sheets (id, user_id, room_id, name, data, is_hidden_in_room) VALUES (?, 'u1', ?, 'Herói', ?, ?)")
        .bind(id)
        .bind(room_id)
        .bind(data)
        .bind(if hidden { 1 } else { 0 })
        .execute(pool)
        .await
        .expect("ficha");
}

/// Duas gravações a partir da mesma versão: a segunda é recusada em vez de apagar a primeira.
#[tokio::test]
async fn test_sheet_update_rejects_stale_version() {
    let pool = setup_db().await;
    insert_sheet(&pool, "s1", None, false).await;
    assert_eq!(SheetRepository::find_version(&pool, "s1").await.expect("versão"), Some(0));

    let first = SheetRepository::update(&pool, "s1", 0, "Herói", "{}", "mage", false, "{}").await.expect("update");
    assert_eq!(first, Some(1), "primeira gravação avança a versão");

    let stale = SheetRepository::update(&pool, "s1", 0, "Herói", "{\"x\":1}", "mage", false, "{}").await.expect("update");
    assert_eq!(stale, None, "gravação com versão antiga deve ser recusada");

    let next = SheetRepository::update(&pool, "s1", 1, "Herói", "{}", "mage", false, "{}").await.expect("update");
    assert_eq!(next, Some(2));
    assert_eq!(SheetRepository::find_version(&pool, "inexistente").await.expect("versão"), None);
}

/// Presença: várias abas do mesmo usuário contam uma vez; ao fechar todas, ele sai.
#[tokio::test]
async fn test_presence_tracks_connections_and_broadcasts() {
    let room = "sala-presenca";
    let mut rx = get_or_create_room_channel(room).subscribe();

    let tab1 = join_presence(room, "ana");
    let tab2 = join_presence(room, "ana");
    let bob = join_presence(room, "bob");
    assert_eq!(online_users(room), vec!["ana".to_string(), "bob".to_string()]);

    drop(tab1);
    assert_eq!(online_users(room), vec!["ana".to_string(), "bob".to_string()], "ainda há uma aba da ana");
    drop(tab2);
    drop(bob);
    assert!(online_users(room).is_empty());

    // A última notificação reflete a sala vazia
    let mut last = None;
    while let Ok(event) = rx.try_recv() {
        last = Some(event);
    }
    assert_eq!(last, Some(RoomBroadcastEvent::PresenceChanged { online_user_ids: vec![] }));
}

/// Ficha visível: a sala recebe o resumo pronto. Ficha oculta: só um aviso para recarregar
/// (o servidor filtra quem pode ver), para não vazar os dados a outros jogadores.
#[tokio::test]
async fn test_sheet_change_notification_respects_hidden_sheets() {
    let pool = setup_db().await;
    sqlx::query("INSERT INTO rooms (id, name, code, gm_id) VALUES ('r-notify', 'Mesa', 'MTA-NTFY', 'u1')")
        .execute(&pool).await.expect("sala");
    insert_sheet(&pool, "visivel", Some("r-notify"), false).await;
    insert_sheet(&pool, "oculta", Some("r-notify"), true).await;
    insert_sheet(&pool, "fora", None, false).await;

    let mut rx = get_or_create_room_channel("r-notify").subscribe();

    notify_sheet_changed(&pool, "visivel").await;
    match rx.try_recv().expect("evento da ficha visível") {
        RoomBroadcastEvent::SheetUpdated(summary) => {
            assert_eq!(summary.id, "visivel");
            assert!(!summary.is_owner, "o servidor nunca decide is_owner pelo cliente");
        }
        other => panic!("evento inesperado: {other:?}"),
    }

    notify_sheet_changed(&pool, "oculta").await;
    assert_eq!(rx.try_recv().expect("evento da ficha oculta"), RoomBroadcastEvent::RoomUpdated);

    notify_sheet_changed(&pool, "fora").await;
    assert!(rx.try_recv().is_err(), "ficha sem sala não gera evento");
}
