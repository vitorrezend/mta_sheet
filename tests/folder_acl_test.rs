use sqlx::SqlitePool;
use uuid::Uuid;

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.expect("Failed to connect to in-memory SQLite");
    mta_sheet::database::ensure_schema(&pool).await.expect("Failed to initialize database");
    pool
}

#[tokio::test]
async fn test_folder_acl_crud_and_inheritance() {
    let pool = setup_test_db().await;

    // 1. Criar usuários (owner, editor, viewer, bystander)
    let owner_id = Uuid::new_v4().to_string();
    let editor_id = Uuid::new_v4().to_string();
    let viewer_id = Uuid::new_v4().to_string();
    let bystander_id = Uuid::new_v4().to_string();

    for (uid, uname) in [
        (&owner_id, "folder_owner"),
        (&editor_id, "folder_editor"),
        (&viewer_id, "folder_viewer"),
        (&bystander_id, "folder_bystander"),
    ] {
        sqlx::query("INSERT INTO users (id, username, password_hash) VALUES (?, ?, 'hash')")
            .bind(uid)
            .bind(uname)
            .execute(&pool)
            .await
            .unwrap();
    }

    // 2. Criar hierarquia de pastas: ParentFolder -> ChildFolder
    let parent_fid = "folder-parent".to_string();
    let child_fid = "folder-child".to_string();

    sqlx::query("INSERT INTO sheet_folders (id, user_id, name, parent_id) VALUES (?, ?, 'Parent', NULL)")
        .bind(&parent_fid)
        .bind(&owner_id)
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO sheet_folders (id, user_id, name, parent_id) VALUES (?, ?, 'Child', ?)")
        .bind(&child_fid)
        .bind(&owner_id)
        .bind(&parent_fid)
        .execute(&pool)
        .await
        .unwrap();

    // 3. Conceder permissão 'write' para editor_id na ParentFolder
    let acl1_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO folder_acls (id, folder_id, grantee_type, grantee_id, permission)
         VALUES (?, ?, 'user', ?, 'write')"
    )
    .bind(&acl1_id)
    .bind(&parent_fid)
    .bind(&editor_id)
    .execute(&pool)
    .await
    .unwrap();

    // 4. Conceder permissão 'read' para viewer_id na ChildFolder
    let acl2_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO folder_acls (id, folder_id, grantee_type, grantee_id, permission)
         VALUES (?, ?, 'user', ?, 'read')"
    )
    .bind(&acl2_id)
    .bind(&child_fid)
    .bind(&viewer_id)
    .execute(&pool)
    .await
    .unwrap();

    // Helper query de checagem com herança recursiva ascendente (walk-up)
    async fn check_perm(pool: &SqlitePool, folder_id: &str, user_id: &str, required_perm: &str) -> bool {
        // Owner sempre tem admin/write/read
        let is_owner: bool = sqlx::query_scalar::<_, i32>(
            "SELECT COUNT(*) FROM sheet_folders WHERE id = ? AND user_id = ?"
        )
        .bind(folder_id)
        .bind(user_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0) > 0;

        if is_owner {
            return true;
        }

        let valid_perms: Vec<&str> = match required_perm {
            "read" => vec!["read", "write", "admin"],
            "write" => vec!["write", "admin"],
            "admin" => vec!["admin"],
            _ => vec![],
        };

        let placeholder = valid_perms.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let sql = format!(
            "WITH RECURSIVE ancestry AS (
                SELECT id, parent_id FROM sheet_folders WHERE id = ?
                UNION ALL
                SELECT f.id, f.parent_id FROM sheet_folders f
                INNER JOIN ancestry a ON f.id = a.parent_id
            )
            SELECT COUNT(*) FROM folder_acls a
            INNER JOIN ancestry anc ON a.folder_id = anc.id
            WHERE ((a.grantee_type = 'user' AND a.grantee_id = ?) OR a.grantee_type = 'public')
              AND a.permission IN ({})",
            placeholder
        );

        let mut q = sqlx::query_scalar::<_, i32>(&sql)
            .bind(folder_id)
            .bind(user_id);

        for p in valid_perms {
            q = q.bind(p);
        }

        q.fetch_one(pool).await.unwrap_or(0) > 0
    }

    // Verificações:
    // Owner tem permissão em ambas as pastas
    assert!(check_perm(&pool, &parent_fid, &owner_id, "admin").await);
    assert!(check_perm(&pool, &child_fid, &owner_id, "write").await);

    // Editor recebeu 'write' na parent: deve herdar 'write' e 'read' na child!
    assert!(check_perm(&pool, &parent_fid, &editor_id, "write").await);
    assert!(check_perm(&pool, &child_fid, &editor_id, "write").await, "Child deve herdar permissão write da Parent");
    assert!(check_perm(&pool, &child_fid, &editor_id, "read").await);
    // Mas editor não é admin
    assert!(!check_perm(&pool, &child_fid, &editor_id, "admin").await);

    // Viewer só recebeu 'read' na child: não tem acesso na parent!
    assert!(!check_perm(&pool, &parent_fid, &viewer_id, "read").await);
    assert!(check_perm(&pool, &child_fid, &viewer_id, "read").await);
    assert!(!check_perm(&pool, &child_fid, &viewer_id, "write").await);

    // Bystander não tem acesso a nenhuma
    assert!(!check_perm(&pool, &parent_fid, &bystander_id, "read").await);
    assert!(!check_perm(&pool, &child_fid, &bystander_id, "read").await);

    // 5. Testar atualização de permissão (ON CONFLICT DO UPDATE)
    sqlx::query(
        "INSERT INTO folder_acls (id, folder_id, grantee_type, grantee_id, permission)
         VALUES ('acl-new', ?, 'user', ?, 'admin')
         ON CONFLICT(folder_id, grantee_type, grantee_id) DO UPDATE SET permission = excluded.permission"
    )
    .bind(&parent_fid)
    .bind(&editor_id)
    .execute(&pool)
    .await
    .unwrap();

    // Agora editor foi promovido a admin da parent: deve ter admin herdado na child também!
    assert!(check_perm(&pool, &child_fid, &editor_id, "admin").await);

    // 6. Testar revogação de ACL
    sqlx::query("DELETE FROM folder_acls WHERE folder_id = ? AND grantee_id = ?")
        .bind(&parent_fid)
        .bind(&editor_id)
        .execute(&pool)
        .await
        .unwrap();

    assert!(!check_perm(&pool, &child_fid, &editor_id, "read").await, "Após revogação, o editor perde o acesso herdado");

    // 7. Testar exclusão em cascata: deletar parent_fid remove seus ACLs
    let acls_before: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM folder_acls").fetch_one(&pool).await.unwrap();
    assert_eq!(acls_before, 1); // Resta acl2_id na child_fid

    sqlx::query("DELETE FROM sheet_folders WHERE id = ?").bind(&parent_fid).execute(&pool).await.unwrap();

    let acls_after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM folder_acls").fetch_one(&pool).await.unwrap();
    assert_eq!(acls_after, 0, "Ao deletar a pasta pai, as subpastas e seus ACLs são cascateados pelo banco");
}
