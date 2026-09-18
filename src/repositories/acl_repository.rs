#[cfg(feature = "ssr")]
use sqlx::{SqlitePool, Row};
#[cfg(feature = "ssr")]
use crate::state::models::FolderAclEntry;

pub struct AclRepository;

#[cfg(feature = "ssr")]
impl AclRepository {
    pub async fn check_folder_permission(
        pool: &SqlitePool,
        folder_id: &str,
        user_id: &str,
        required_permission: &str,
    ) -> Result<bool, sqlx::Error> {
        // 1. Check direct owner
        let owner = sqlx::query_scalar::<_, String>("SELECT user_id FROM sheet_folders WHERE id = ?")
            .bind(folder_id)
            .fetch_optional(pool)
            .await?;

        if let Some(ref oid) = owner {
            if oid == user_id {
                return Ok(true);
            }
        } else {
            return Ok(false);
        }

        // 2. Recursive walk-up resolution through folder ancestry
        let row = sqlx::query(
            "WITH RECURSIVE folder_ancestry AS (
                SELECT id, parent_id, 0 AS depth
                FROM sheet_folders WHERE id = ?
                UNION ALL
                SELECT f.id, f.parent_id, a.depth + 1
                FROM sheet_folders f
                JOIN folder_ancestry a ON f.id = a.parent_id
            )
            SELECT acl.permission
            FROM folder_acls acl
            JOIN folder_ancestry fa ON acl.folder_id = fa.id
            WHERE (acl.grantee_type = 'user' AND acl.grantee_id = ?)
               OR (acl.grantee_type = 'public')
               OR (acl.grantee_type = 'room' AND acl.grantee_id IN (SELECT room_id FROM room_members WHERE user_id = ?))
            ORDER BY fa.depth ASC,
                     CASE acl.permission WHEN 'admin' THEN 3 WHEN 'write' THEN 2 WHEN 'read' THEN 1 ELSE 0 END DESC
            LIMIT 1"
        )
        .bind(folder_id)
        .bind(user_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        if let Some(r) = row {
            let perm: String = r.get("permission");
            let satisfies = match (required_permission, perm.as_str()) {
                ("read", "read" | "write" | "admin") => true,
                ("write", "write" | "admin") => true,
                ("admin", "admin") => true,
                _ => false,
            };
            return Ok(satisfies);
        }

        Ok(false)
    }

    pub async fn list_for_folder(pool: &SqlitePool, folder_id: &str) -> Result<Vec<FolderAclEntry>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT a.id, a.folder_id, a.grantee_type, a.grantee_id, a.permission, a.created_at,
                    COALESCE(u.username, r.name, 'Público') as grantee_name
             FROM folder_acls a
             LEFT JOIN users u ON a.grantee_type = 'user' AND a.grantee_id = u.id
             LEFT JOIN rooms r ON a.grantee_type = 'room' AND a.grantee_id = r.id
             WHERE a.folder_id = ?
             ORDER BY a.created_at ASC"
        )
        .bind(folder_id)
        .fetch_all(pool)
        .await?;

        let entries = rows.into_iter().map(|r| {
            FolderAclEntry {
                id: r.get("id"),
                folder_id: r.get("folder_id"),
                grantee_type: r.get("grantee_type"),
                grantee_id: r.get("grantee_id"),
                grantee_name: r.get("grantee_name"),
                permission: r.get("permission"),
                created_at: r.get("created_at"),
            }
        }).collect();

        Ok(entries)
    }

    pub async fn find_user_by_name(pool: &SqlitePool, username: &str) -> Result<Option<(String, String)>, sqlx::Error> {
        let u_row = sqlx::query("SELECT id, username FROM users WHERE LOWER(username) = LOWER(?)")
            .bind(username)
            .fetch_optional(pool)
            .await?;

        Ok(u_row.map(|r| (r.get("id"), r.get("username"))))
    }

    pub async fn find_room_by_id_or_code(pool: &SqlitePool, identifier: &str) -> Result<Option<(String, String)>, sqlx::Error> {
        let r_row = sqlx::query("SELECT id, name FROM rooms WHERE id = ? OR code = ?")
            .bind(identifier)
            .bind(identifier)
            .fetch_optional(pool)
            .await?;

        Ok(r_row.map(|r| (r.get("id"), r.get("name"))))
    }

    pub async fn grant(
        pool: &SqlitePool,
        acl_id: &str,
        folder_id: &str,
        grantee_type: &str,
        grantee_id: Option<&str>,
        permission: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO folder_acls (id, folder_id, grantee_type, grantee_id, permission)
             VALUES (?, ?, ?, ?, ?)
             ON CONFLICT(folder_id, grantee_type, grantee_id) DO UPDATE SET permission = excluded.permission"
        )
        .bind(acl_id)
        .bind(folder_id)
        .bind(grantee_type)
        .bind(grantee_id)
        .bind(permission)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find_folder_id_by_acl(pool: &SqlitePool, acl_id: &str) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar::<_, String>("SELECT folder_id FROM folder_acls WHERE id = ?")
            .bind(acl_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn revoke(pool: &SqlitePool, acl_id: &str) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM folder_acls WHERE id = ?")
            .bind(acl_id)
            .execute(pool)
            .await?;

        Ok(res.rows_affected())
    }
}
