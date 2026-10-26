//! Cards da sala: o resumo de cada ficha é guardado com a versão em que foi calculado e
//! só é refeito quando a ficha muda.

use mta_sheet::repositories::RoomRepository;
use mta_sheet::state::CharacterData;
use sqlx::SqlitePool;

async fn room_with_sheet(willpower_current: i32) -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.expect("sqlite em memória");
    mta_sheet::database::ensure_schema(&pool).await.expect("esquema base");
    mta_sheet::database::run_migrations(&pool).await.expect("migrações");

    sqlx::query("INSERT INTO users (id, username, password_hash) VALUES ('gm', 'narrador', 'x'), ('p1', 'jogador', 'x')")
        .execute(&pool)
        .await
        .expect("usuários");
    sqlx::query("INSERT INTO rooms (id, name, code, gm_id) VALUES ('room', 'Mesa', 'MTA-TEST-0001', 'gm')")
        .execute(&pool)
        .await
        .expect("sala");

    let mut data = CharacterData::new("s1".into(), "Maga Cache".into());
    data.set_willpower_total(7);
    data.set_willpower_current(willpower_current);
    sqlx::query("INSERT INTO character_sheets (id, user_id, room_id, name, data, version) VALUES ('s1', 'p1', 'room', 'Maga Cache', ?, 3)")
        .bind(serde_json::to_string(&data).expect("json"))
        .execute(&pool)
        .await
        .expect("ficha");
    pool
}

async fn cached_version(pool: &SqlitePool) -> Option<i64> {
    sqlx::query_scalar("SELECT room_summary_version FROM character_sheets WHERE id = 's1'")
        .fetch_one(pool)
        .await
        .expect("versão do resumo")
}

#[tokio::test]
async fn test_room_summary_is_stored_and_reused_while_the_sheet_is_unchanged() {
    let pool = room_with_sheet(4).await;

    // Primeira leitura calcula e guarda o resumo da versão 3
    let first = RoomRepository::room_sheet_summaries(&pool, "room", "gm", true).await.expect("cards");
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].willpower_current, 4);
    assert_eq!(cached_version(&pool).await, Some(3));

    // Prova de que a segunda leitura usa o guardado: altera só o resumo, sem mexer na ficha
    sqlx::query("UPDATE character_sheets SET room_summary_json = json_set(room_summary_json, '$.willpower_current', 99) WHERE id = 's1'")
        .execute(&pool)
        .await
        .expect("altera o resumo guardado");
    let second = RoomRepository::room_sheet_summaries(&pool, "room", "gm", true).await.expect("cards");
    assert_eq!(second[0].willpower_current, 99);

    // A ficha mudou (versão subiu): o resumo é refeito a partir dos dados reais
    sqlx::query("UPDATE character_sheets SET version = version + 1 WHERE id = 's1'")
        .execute(&pool)
        .await
        .expect("nova versão");
    let third = RoomRepository::room_sheet_summaries(&pool, "room", "gm", true).await.expect("cards");
    assert_eq!(third[0].willpower_current, 4);
    assert_eq!(cached_version(&pool).await, Some(4));
}

#[tokio::test]
async fn test_viewer_dependent_fields_are_not_taken_from_the_cache() {
    let pool = room_with_sheet(5).await;

    let as_gm = RoomRepository::room_sheet_summaries(&pool, "room", "gm", true).await.expect("cards");
    assert!(!as_gm[0].is_owner);
    let as_owner = RoomRepository::room_sheet_summaries(&pool, "room", "p1", false).await.expect("cards");
    assert!(as_owner[0].is_owner, "o dono da ficha continua vendo a ficha como sua com o resumo guardado");

    // Ficha oculta: some para os outros jogadores, continua para o narrador e para o dono
    sqlx::query("UPDATE character_sheets SET is_hidden_in_room = 1 WHERE id = 's1'")
        .execute(&pool)
        .await
        .expect("oculta");
    assert!(RoomRepository::room_sheet_summaries(&pool, "room", "outro", false).await.expect("cards").is_empty());
    let gm_view = RoomRepository::room_sheet_summaries(&pool, "room", "gm", true).await.expect("cards");
    assert!(gm_view[0].is_hidden);
    assert_eq!(RoomRepository::room_sheet_summaries(&pool, "room", "p1", false).await.expect("cards").len(), 1);
}
