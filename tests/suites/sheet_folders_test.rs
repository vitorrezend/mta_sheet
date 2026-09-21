use sqlx::SqlitePool;
use uuid::Uuid;

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.expect("Failed to connect to in-memory SQLite");
    mta_sheet::database::ensure_schema(&pool).await.expect("Failed to initialize database");
    pool
}

#[tokio::test]
async fn test_sheet_folders_migration_and_crud() {
    let pool = setup_test_db().await;

    let user_id = Uuid::new_v4().to_string();
    let username = "magus_test";
    sqlx::query("INSERT INTO users (id, username, password_hash) VALUES (?, ?, ?)")
        .bind(&user_id)
        .bind(username)
        .bind("hash")
        .execute(&pool)
        .await
        .expect("Failed to insert user");

    // 1. Inserir pasta
    let folder_id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO sheet_folders (id, user_id, name, icon, color) VALUES (?, ?, ?, ?, ?)")
        .bind(&folder_id)
        .bind(&user_id)
        .bind("Cabala de Praga")
        .bind("🔮")
        .bind("#6366f1")
        .execute(&pool)
        .await
        .expect("Failed to insert folder");

    // 2. Inserir ficha vinculada à pasta
    let sheet_id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO character_sheets (id, user_id, name, data, folder_id) VALUES (?, ?, ?, ?, ?)")
        .bind(&sheet_id)
        .bind(&user_id)
        .bind("Hermes Trismegisto")
        .bind(r#"{"name":"Hermes Trismegisto"}"#)
        .bind(&folder_id)
        .execute(&pool)
        .await
        .expect("Failed to insert sheet with folder");

    // 3. Consultar pasta com contagem de fichas
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM character_sheets WHERE folder_id = ?")
        .bind(&folder_id)
        .fetch_one(&pool)
        .await
        .expect("Failed to count sheets in folder");
    assert_eq!(count, 1);

    // 4. Mover ficha para raiz (folder_id = NULL)
    sqlx::query("UPDATE character_sheets SET folder_id = NULL WHERE id = ? AND user_id = ?")
        .bind(&sheet_id)
        .bind(&user_id)
        .execute(&pool)
        .await
        .expect("Failed to unassign folder");

    let count_after_move: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM character_sheets WHERE folder_id = ?")
        .bind(&folder_id)
        .fetch_one(&pool)
        .await
        .expect("Failed to count sheets");
    assert_eq!(count_after_move, 0);

    // 5. Associar novamente e depois deletar a pasta, testando que a ficha NÃO é deletada
    sqlx::query("UPDATE character_sheets SET folder_id = ? WHERE id = ?")
        .bind(&folder_id)
        .bind(&sheet_id)
        .execute(&pool)
        .await
        .expect("Reassign folder");

    // Simula exclusão segura de pasta
    sqlx::query("UPDATE character_sheets SET folder_id = NULL WHERE folder_id = ?")
        .bind(&folder_id)
        .execute(&pool)
        .await
        .expect("Set null on delete");

    sqlx::query("DELETE FROM sheet_folders WHERE id = ?")
        .bind(&folder_id)
        .execute(&pool)
        .await
        .expect("Delete folder");

    // Verifica que a ficha continua existindo
    let sheet_name: String = sqlx::query_scalar("SELECT name FROM character_sheets WHERE id = ?")
        .bind(&sheet_id)
        .fetch_one(&pool)
        .await
        .expect("Sheet must still exist");
    assert_eq!(sheet_name, "Hermes Trismegisto");

    let final_folder_id: Option<String> = sqlx::query_scalar("SELECT folder_id FROM character_sheets WHERE id = ?")
        .bind(&sheet_id)
        .fetch_one(&pool)
        .await
        .expect("Fetch folder_id");
    assert!(final_folder_id.is_none());
}

#[tokio::test]
async fn test_nested_subfolders_and_safe_recursive_delete() {
    let pool = setup_test_db().await;
    let user_id = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO users (id, username, password_hash) VALUES (?, ?, ?)")
        .bind(&user_id)
        .bind("chronicler")
        .bind("hash")
        .execute(&pool)
        .await
        .expect("Insert user");

    // Root folder: "Crônicas"
    let root_id = "folder-root".to_string();
    sqlx::query("INSERT INTO sheet_folders (id, user_id, name, parent_id) VALUES (?, ?, ?, NULL)")
        .bind(&root_id)
        .bind(&user_id)
        .bind("Crônicas")
        .execute(&pool)
        .await
        .expect("Insert root");

    // Level 1: "Campanha 1999"
    let child_id = "folder-child".to_string();
    sqlx::query("INSERT INTO sheet_folders (id, user_id, name, parent_id) VALUES (?, ?, ?, ?)")
        .bind(&child_id)
        .bind(&user_id)
        .bind("Campanha 1999")
        .bind(&root_id)
        .execute(&pool)
        .await
        .expect("Insert child");

    // Level 2: "NPCs e Vilões"
    let grandchild_id = "folder-grandchild".to_string();
    sqlx::query("INSERT INTO sheet_folders (id, user_id, name, parent_id) VALUES (?, ?, ?, ?)")
        .bind(&grandchild_id)
        .bind(&user_id)
        .bind("NPCs e Vilões")
        .bind(&child_id)
        .execute(&pool)
        .await
        .expect("Insert grandchild");

    // Insere fichas nos 3 níveis
    let sheet1 = "sheet-1".to_string();
    let sheet2 = "sheet-2".to_string();
    let sheet3 = "sheet-3".to_string();

    for (s_id, f_id, s_name) in [
        (&sheet1, &root_id, "Mago Raiz"),
        (&sheet2, &child_id, "Mago Filho"),
        (&sheet3, &grandchild_id, "Vilão Neto"),
    ] {
        sqlx::query("INSERT INTO character_sheets (id, user_id, name, data, folder_id) VALUES (?, ?, ?, ?, ?)")
            .bind(s_id)
            .bind(&user_id)
            .bind(s_name)
            .bind(r#"{"name":"test"}"#)
            .bind(f_id)
            .execute(&pool)
            .await
            .expect("Insert sheet");
    }

    // Testa CTE recursiva de subpastas
    let subfolder_ids: Vec<String> = sqlx::query_scalar(
        "WITH RECURSIVE subfolders AS (
            SELECT id FROM sheet_folders WHERE id = ?
            UNION ALL
            SELECT f.id FROM sheet_folders f
            INNER JOIN subfolders s ON f.parent_id = s.id
        )
        SELECT id FROM subfolders"
    )
    .bind(&root_id)
    .fetch_all(&pool)
    .await
    .expect("Fetch subfolders CTE");

    assert_eq!(subfolder_ids.len(), 3);
    assert!(subfolder_ids.contains(&root_id));
    assert!(subfolder_ids.contains(&child_id));
    assert!(subfolder_ids.contains(&grandchild_id));

    // Desvincula todas as fichas da árvore antes de deletar (exclusão segura que implementamos em delete_folder)
    sqlx::query(
        "WITH RECURSIVE subfolders AS (
            SELECT id FROM sheet_folders WHERE id = ?
            UNION ALL
            SELECT f.id FROM sheet_folders f
            INNER JOIN subfolders s ON f.parent_id = s.id
        )
        UPDATE character_sheets
        SET folder_id = NULL
        WHERE folder_id IN (SELECT id FROM subfolders)"
    )
    .bind(&root_id)
    .execute(&pool)
    .await
    .expect("Safe unlink sheets");

    // Deleta a pasta raiz (CASCADE removerá as subpastas no SQLite)
    sqlx::query("DELETE FROM sheet_folders WHERE id = ?")
        .bind(&root_id)
        .execute(&pool)
        .await
        .expect("Delete root folder");

    // As pastas devem ter sido deletadas
    let remaining_folders: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sheet_folders WHERE user_id = ?")
        .bind(&user_id)
        .fetch_one(&pool)
        .await
        .expect("Count remaining folders");
    assert_eq!(remaining_folders, 0);

    // Todas as 3 fichas DEVEM continuar existindo e estar na raiz (folder_id IS NULL)
    let remaining_sheets: Vec<(String, Option<String>)> = sqlx::query_as("SELECT name, folder_id FROM character_sheets WHERE user_id = ? ORDER BY id ASC")
        .bind(&user_id)
        .fetch_all(&pool)
        .await
        .expect("Fetch sheets");

    assert_eq!(remaining_sheets.len(), 3);
    for (name, fid) in remaining_sheets {
        assert!(fid.is_none(), "Ficha '{}' deveria ter folder_id nulo após exclusão da pasta", name);
    }
}

#[tokio::test]
async fn test_folder_cycle_detection_cte() {
    let pool = setup_test_db().await;
    let user_id = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO users (id, username, password_hash) VALUES (?, ?, ?)")
        .bind(&user_id)
        .bind("cycle_tester")
        .bind("hash")
        .execute(&pool)
        .await
        .expect("Insert user");

    // A -> B -> C
    let a_id = "folder-A".to_string();
    let b_id = "folder-B".to_string();
    let c_id = "folder-C".to_string();

    sqlx::query("INSERT INTO sheet_folders (id, user_id, name, parent_id) VALUES (?, ?, 'A', NULL)").bind(&a_id).bind(&user_id).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO sheet_folders (id, user_id, name, parent_id) VALUES (?, ?, 'B', ?)").bind(&b_id).bind(&user_id).bind(&a_id).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO sheet_folders (id, user_id, name, parent_id) VALUES (?, ?, 'C', ?)").bind(&c_id).bind(&user_id).bind(&b_id).execute(&pool).await.unwrap();

    // Query de ciclo: se tentarmos mover A para dentro de C:
    // Devemos verificar se C tem A em seus ancestrais!
    let target_parent = Some(c_id.clone());
    let folder_id = a_id.clone();

    let causes_cycle: bool = if let Some(ref target) = target_parent {
        let is_descendant = sqlx::query_scalar::<_, i32>(
            "WITH RECURSIVE ancestry AS (
                SELECT id, parent_id FROM sheet_folders WHERE id = ?
                UNION ALL
                SELECT f.id, f.parent_id FROM sheet_folders f
                INNER JOIN ancestry a ON f.id = a.parent_id
            )
            SELECT COUNT(*) FROM ancestry WHERE id = ?"
        )
        .bind(target)
        .bind(&folder_id)
        .fetch_one(&pool)
        .await
        .expect("Query cycle detection") > 0;
        is_descendant
    } else {
        false
    };

    assert!(causes_cycle, "Mover A para C deve ser detectado como ciclo pois C é descendente de A");

    // Agora testar mover C para a raiz (None) ou para A diretamente (não causa ciclo)
    let safe_move_c_to_a: bool = {
        let is_descendant = sqlx::query_scalar::<_, i32>(
            "WITH RECURSIVE ancestry AS (
                SELECT id, parent_id FROM sheet_folders WHERE id = ?
                UNION ALL
                SELECT f.id, f.parent_id FROM sheet_folders f
                INNER JOIN ancestry a ON f.id = a.parent_id
            )
            SELECT COUNT(*) FROM ancestry WHERE id = ?"
        )
        .bind(&a_id)
        .bind(&c_id)
        .fetch_one(&pool)
        .await
        .expect("Query cycle detection") > 0;
        is_descendant
    };

    assert!(!safe_move_c_to_a, "Mover C para A é uma operação válida e não causa ciclo");
}

