use sqlx::SqlitePool;
use uuid::Uuid;
use mta_sheet::repositories::SheetRepository;
use mta_sheet::state::models::{keys, CharacterData};

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.expect("Failed to connect to in-memory SQLite");
    mta_sheet::database::ensure_schema(&pool).await.expect("Failed to initialize database");
    pool
}

#[tokio::test]
async fn test_public_feed_and_author_username_resolution() {
    let pool = setup_test_db().await;

    // 1. Criar dois usuários: alice e bob
    let alice_id = Uuid::new_v4().to_string();
    let bob_id = Uuid::new_v4().to_string();

    for (uid, uname) in [(&alice_id, "alice_awakened"), (&bob_id, "bob_hermetic")] {
        sqlx::query("INSERT INTO users (id, username, password_hash) VALUES (?, ?, 'hash')")
            .bind(uid)
            .bind(uname)
            .execute(&pool)
            .await
            .unwrap();
    }

    // 2. Alice cria 1 ficha pública e 1 ficha privada
    let s1_id = "sheet_alice_pub".to_string();
    let mut sheet_alice_pub = CharacterData::new(s1_id.clone(), "Alice Pública".to_string());
    sheet_alice_pub.labels.insert(keys::HEADER_TRADICAO.to_string(), "Ordem de Hermes".to_string());
    let json_alice_pub = serde_json::to_string(&sheet_alice_pub).unwrap();
    let sum_alice_pub = sheet_alice_pub.to_summary("2026-09-19".to_string(), true, true);
    let sum_alice_pub_json = serde_json::to_string(&sum_alice_pub).unwrap();

    let s2_id = "sheet_alice_priv".to_string();
    let mut sheet_alice_priv = CharacterData::new(s2_id.clone(), "Alice Privada".to_string());
    sheet_alice_priv.labels.insert(keys::HEADER_TRADICAO.to_string(), "Eutanatos".to_string());
    let json_alice_priv = serde_json::to_string(&sheet_alice_priv).unwrap();
    let sum_alice_priv = sheet_alice_priv.to_summary("2026-09-19".to_string(), false, true);
    let sum_alice_priv_json = serde_json::to_string(&sum_alice_priv).unwrap();

    SheetRepository::create(&pool, &s1_id, Some(&alice_id), "Alice Pública", &json_alice_pub, "mage", None, &sum_alice_pub_json).await.unwrap();
    sqlx::query("UPDATE character_sheets SET is_public = 1 WHERE id = ?").bind(&s1_id).execute(&pool).await.unwrap();

    SheetRepository::create(&pool, &s2_id, Some(&alice_id), "Alice Privada", &json_alice_priv, "mage", None, &sum_alice_priv_json).await.unwrap();

    // 3. Bob cria 1 ficha pública
    let s3_id = "sheet_bob_pub".to_string();
    let mut sheet_bob_pub = CharacterData::new(s3_id.clone(), "Bob Público".to_string());
    sheet_bob_pub.labels.insert(keys::HEADER_TRADICAO.to_string(), "Filhos do Éter".to_string());
    let json_bob_pub = serde_json::to_string(&sheet_bob_pub).unwrap();
    let sum_bob_pub = sheet_bob_pub.to_summary("2026-09-19".to_string(), true, true);
    let sum_bob_pub_json = serde_json::to_string(&sum_bob_pub).unwrap();

    SheetRepository::create(&pool, &s3_id, Some(&bob_id), "Bob Público", &json_bob_pub, "mage", None, &sum_bob_pub_json).await.unwrap();
    sqlx::query("UPDATE character_sheets SET is_public = 1 WHERE id = ?").bind(&s3_id).execute(&pool).await.unwrap();

    // 4. Testar Feed Público: SheetRepository::list_public
    let public_feed = SheetRepository::list_public(&pool, None).await.unwrap();
    assert_eq!(public_feed.len(), 2, "Feed público deve conter exatamente as 2 fichas públicas");

    let alice_feed_entry = public_feed.iter().find(|s| s.id == s1_id).expect("Ficha de Alice deve estar no feed");
    assert_eq!(alice_feed_entry.author_username.as_deref(), Some("alice_awakened"));
    assert_eq!(alice_feed_entry.name, "Alice Pública");

    let bob_feed_entry = public_feed.iter().find(|s| s.id == s3_id).expect("Ficha de Bob deve estar no feed");
    assert_eq!(bob_feed_entry.author_username.as_deref(), Some("bob_hermetic"));
    assert_eq!(bob_feed_entry.name, "Bob Público");

    // 5. Testar Perfil Visitante: apenas fichas públicas de Alice
    let alice_public_only = SheetRepository::list_by_user_public_only(&pool, &alice_id, Some("alice_awakened")).await.unwrap();
    assert_eq!(alice_public_only.len(), 1);
    assert_eq!(alice_public_only[0].id, s1_id);
    assert_eq!(alice_public_only[0].author_username.as_deref(), Some("alice_awakened"));

    // 6. Testar Métricas do Perfil
    let alice_public_count = SheetRepository::count_public_by_user(&pool, &alice_id).await.unwrap();
    assert_eq!(alice_public_count, 1);

    let alice_rooms_count = SheetRepository::count_rooms_by_user(&pool, &alice_id).await.unwrap();
    assert_eq!(alice_rooms_count, 0);

    // 7. Simular criação de sala por Bob
    let room_id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO rooms (id, gm_id, code, name) VALUES (?, ?, 'MTA-TEST', 'Cabala do Éter')")
        .bind(&room_id)
        .bind(&bob_id)
        .execute(&pool)
        .await
        .unwrap();

    let bob_rooms_count = SheetRepository::count_rooms_by_user(&pool, &bob_id).await.unwrap();
    assert_eq!(bob_rooms_count, 1);

    // 8. Testar busca de usuário por username
    let found_user = SheetRepository::find_user_by_username(&pool, "alice_awakened").await.unwrap();
    assert!(found_user.is_some());
    let (uid, uname, _) = found_user.unwrap();
    assert_eq!(uid, alice_id);
    assert_eq!(uname, "alice_awakened");
}

#[tokio::test]
async fn test_sheet_cloning_and_ownership_transfer() {
    let pool = setup_test_db().await;

    let original_author_id = Uuid::new_v4().to_string();
    let cloner_id = Uuid::new_v4().to_string();

    for (uid, uname) in [(&original_author_id, "master_mage"), (&cloner_id, "apprentice")] {
        sqlx::query("INSERT INTO users (id, username, password_hash) VALUES (?, ?, 'hash')")
            .bind(uid)
            .bind(uname)
            .execute(&pool)
            .await
            .unwrap();
    }

    // Criar ficha pública original
    let orig_id = "sheet_orig".to_string();
    let mut original_data = CharacterData::new(orig_id.clone(), "Grimório Ancestral".to_string());
    original_data.labels.insert(keys::HEADER_TRADICAO.to_string(), "Verbena".to_string());
    let original_json = serde_json::to_string(&original_data).unwrap();
    let orig_summary = original_data.to_summary("2026-09-19".to_string(), true, true);
    let orig_summary_json = serde_json::to_string(&orig_summary).unwrap();

    SheetRepository::create(
        &pool,
        &orig_id,
        Some(&original_author_id),
        "Grimório Ancestral",
        &original_json,
        "mage",
        None,
        &orig_summary_json,
    ).await.unwrap();
    sqlx::query("UPDATE character_sheets SET is_public = 1 WHERE id = ?").bind(&orig_id).execute(&pool).await.unwrap();

    // Simular clonagem da ficha pelo cloner_id
    let raw = SheetRepository::find_raw_by_id(&pool, &orig_id).await.unwrap().unwrap();
    let fetched_json = raw.2;
    let mut cloned_data: CharacterData = serde_json::from_str(&fetched_json).unwrap();
    let cloned_id = Uuid::new_v4().to_string();
    cloned_data.id = cloned_id.clone();
    cloned_data.name = format!("{} (Cópia)", cloned_data.name);
    cloned_data.labels.insert(keys::HEADER_NOME.to_string(), cloned_data.name.clone());
    cloned_data.is_public = false;
    cloned_data.can_edit = true;

    let cloned_json = serde_json::to_string(&cloned_data).unwrap();
    let cloned_summary = cloned_data.to_summary("2026-09-19".to_string(), false, true);
    let cloned_summary_json = serde_json::to_string(&cloned_summary).unwrap();

    SheetRepository::create(
        &pool,
        &cloned_id,
        Some(&cloner_id),
        &cloned_data.name,
        &cloned_json,
        "mage",
        None,
        &cloned_summary_json,
    ).await.unwrap();

    // Verificar que a ficha clonada pertence ao cloner e tem nome atualizado
    let recovered_raw = SheetRepository::find_raw_by_id(&pool, &cloned_id).await.unwrap().unwrap();
    assert_eq!(recovered_raw.0.as_deref(), Some(cloner_id.as_str()));
    assert!(!recovered_raw.4, "Ficha clonada deve nascer privada por segurança");

    let recovered_data: CharacterData = serde_json::from_str(&recovered_raw.2).unwrap();
    assert_eq!(recovered_data.name, "Grimório Ancestral (Cópia)");
    assert_eq!(recovered_data.get_tradition(), "Verbena");
}
