use sqlx::SqlitePool;
use uuid::Uuid;
use mta_sheet::repositories::SheetRepository;
use mta_sheet::state::models::{keys, CharacterData};

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory SQLite");
    mta_sheet::database::ensure_schema(&pool)
        .await
        .expect("Failed to initialize database");
    pool
}

#[tokio::test]
async fn test_sheet_likes_toggle_and_counts() {
    let pool = setup_test_db().await;

    // 1. Criar usuários alice e bob
    let alice_id = Uuid::new_v4().to_string();
    let bob_id = Uuid::new_v4().to_string();

    for (uid, uname) in [(&alice_id, "alice_like"), (&bob_id, "bob_like")] {
        sqlx::query("INSERT INTO users (id, username, password_hash) VALUES (?, ?, 'hash')")
            .bind(uid)
            .bind(uname)
            .execute(&pool)
            .await
            .unwrap();
    }

    // 2. Alice cria uma ficha pública
    let sheet_id = "sheet_like_test_1".to_string();
    let mut sheet_data = CharacterData::new(sheet_id.clone(), "Mago Iluminado".to_string());
    sheet_data.labels.insert(keys::HEADER_TRADICAO.to_string(), "Ordem de Hermes".to_string());
    let json_data = serde_json::to_string(&sheet_data).unwrap();
    let sum_data = sheet_data.to_summary("2026-09-19".to_string(), true, true);
    let sum_json = serde_json::to_string(&sum_data).unwrap();

    SheetRepository::create(
        &pool,
        &sheet_id,
        Some(&alice_id),
        "Mago Iluminado",
        &json_data,
        "mage",
        None,
        &sum_json,
    )
    .await
    .unwrap();
    sqlx::query("UPDATE character_sheets SET is_public = 1 WHERE id = ?")
        .bind(&sheet_id)
        .execute(&pool)
        .await
        .unwrap();

    // 3. Estado inicial: 0 curtidas
    let initial_count = SheetRepository::count_likes(&pool, &sheet_id).await.unwrap();
    assert_eq!(initial_count, 0, "Contagem inicial de likes deve ser 0");

    let alice_initial_liked = SheetRepository::is_liked(&pool, &sheet_id, &alice_id).await.unwrap();
    assert!(!alice_initial_liked, "Alice ainda não curtiu");

    // 4. Alice curte a ficha (toggle ON)
    let (liked, count) = SheetRepository::toggle_like(&pool, &sheet_id, &alice_id).await.unwrap();
    assert!(liked, "Toggle deve retornar true (curtiu)");
    assert_eq!(count, 1, "Contagem após like deve ser 1");

    let alice_liked_now = SheetRepository::is_liked(&pool, &sheet_id, &alice_id).await.unwrap();
    assert!(alice_liked_now, "Alice agora deve constar como curtiu");

    // Bob ainda não curtiu
    let bob_liked = SheetRepository::is_liked(&pool, &sheet_id, &bob_id).await.unwrap();
    assert!(!bob_liked, "Bob ainda não curtiu");

    // 5. Bob curte a ficha também (segundo like)
    let (bob_liked_res, total_count) = SheetRepository::toggle_like(&pool, &sheet_id, &bob_id).await.unwrap();
    assert!(bob_liked_res, "Bob deve curtir com sucesso");
    assert_eq!(total_count, 2, "Contagem total agora deve ser 2");

    // 6. Testar listagem pública com context de usuário
    // Alice vê curtido por ela
    let feed_for_alice = SheetRepository::list_public(&pool, Some(&alice_id)).await.unwrap();
    let feed_item = feed_for_alice.iter().find(|s| s.id == sheet_id).expect("Ficha no feed");
    assert_eq!(feed_item.likes_count, 2);
    assert!(feed_item.is_liked, "Para Alice, is_liked deve ser true");

    // Deslogado (None) vê likes_count = 2, mas is_liked = false
    let feed_anon = SheetRepository::list_public(&pool, None).await.unwrap();
    let anon_item = feed_anon.iter().find(|s| s.id == sheet_id).expect("Ficha no feed anon");
    assert_eq!(anon_item.likes_count, 2);
    assert!(!anon_item.is_liked, "Para visitante anônimo, is_liked deve ser false");

    // 7. Alice descurte (toggle OFF)
    let (alice_unliked, count_after_unlike) = SheetRepository::toggle_like(&pool, &sheet_id, &alice_id).await.unwrap();
    assert!(!alice_unliked, "Toggle deve retornar false (descurtiu)");
    assert_eq!(count_after_unlike, 1, "Contagem deve cair para 1");

    let alice_still_liked = SheetRepository::is_liked(&pool, &sheet_id, &alice_id).await.unwrap();
    assert!(!alice_still_liked, "Alice não deve mais constar como curtiu");

    // 8. Testar exclusão em cascata: deletar a ficha deve remover as curtidas associadas
    SheetRepository::delete(&pool, &sheet_id).await.unwrap();

    let orphan_likes: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheet_likes WHERE sheet_id = ?")
        .bind(&sheet_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(orphan_likes.0, 0, "Curtidas devem ser excluídas em cascata com a ficha");
}
