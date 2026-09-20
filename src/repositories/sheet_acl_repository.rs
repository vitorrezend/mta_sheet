#[cfg(feature = "ssr")]
use sqlx::{SqlitePool, Row};
#[cfg(feature = "ssr")]
use crate::state::models::{SheetAclEntry, SheetShareSettings};
#[cfg(feature = "ssr")]
use uuid::Uuid;

pub struct SheetAclRepository;

#[cfg(feature = "ssr")]
impl SheetAclRepository {
    pub async fn check_sheet_permission(
        pool: &SqlitePool,
        sheet_id: &str,
        user_id: &str,
        required_permission: &str,
    ) -> Result<bool, sqlx::Error> {
        // 1. Checa proprietário direto da ficha
        let sheet_row = sqlx::query("SELECT user_id, room_id FROM character_sheets WHERE id = ?")
            .bind(sheet_id)
            .fetch_optional(pool)
            .await?;

        let (sheet_owner, room_id) = match sheet_row {
            Some(r) => {
                let u: Option<String> = r.get("user_id");
                let rm: Option<String> = r.get("room_id");
                (u, rm)
            }
            None => return Ok(false),
        };

        if let Some(ref oid) = sheet_owner {
            if oid == user_id {
                return Ok(true);
            }
        }

        // 2. Checa se o usuário é narrador (GM) da sala à qual a ficha pertence
        if let Some(ref r_id) = room_id {
            let is_gm = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM rooms WHERE id = ? AND gm_id = ?")
                .bind(r_id)
                .bind(user_id)
                .fetch_one(pool)
                .await
                .unwrap_or(0);
            if is_gm > 0 {
                return Ok(true);
            }
        }

        // 3. Checa permissões em sheet_acls (usuário direto, crônica ou público cadastrado)
        let row = sqlx::query(
            "SELECT permission FROM sheet_acls
             WHERE sheet_id = ?
               AND (
                 (grantee_type = 'user' AND grantee_id = ?)
                 OR (grantee_type = 'public')
                 OR (grantee_type = 'room' AND grantee_id IN (SELECT room_id FROM room_members WHERE user_id = ?))
               )
             ORDER BY CASE permission WHEN 'admin' THEN 3 WHEN 'write' THEN 2 WHEN 'read' THEN 1 ELSE 0 END DESC
             LIMIT 1"
        )
        .bind(sheet_id)
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

    pub async fn verify_token_or_public(
        pool: &SqlitePool,
        sheet_id: &str,
        token_opt: Option<&str>,
    ) -> Result<bool, sqlx::Error> {
        let row = sqlx::query("SELECT is_public, share_token, share_permission FROM character_sheets WHERE id = ?")
            .bind(sheet_id)
            .fetch_optional(pool)
            .await?;

        if let Some(r) = row {
            let is_public: bool = r.get::<i32, _>("is_public") == 1;
            if is_public {
                return Ok(true);
            }

            let share_permission = r.try_get::<String, _>("share_permission").unwrap_or_else(|_| "none".to_string());
            if share_permission == "view" {
                let db_token: Option<String> = r.try_get("share_token").unwrap_or(None);
                if let (Some(token_given), Some(token_in_db)) = (token_opt, db_token.as_deref()) {
                    if !token_given.trim().is_empty() && token_given == token_in_db {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    pub async fn list_for_sheet(pool: &SqlitePool, sheet_id: &str) -> Result<Vec<SheetAclEntry>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT a.id, a.sheet_id, a.grantee_type, a.grantee_id, a.permission, a.created_at,
                    COALESCE(u.username, r.name, 'Público') as grantee_name
             FROM sheet_acls a
             LEFT JOIN users u ON a.grantee_type = 'user' AND a.grantee_id = u.id
             LEFT JOIN rooms r ON a.grantee_type = 'room' AND a.grantee_id = r.id
             WHERE a.sheet_id = ?
             ORDER BY a.created_at ASC"
        )
        .bind(sheet_id)
        .fetch_all(pool)
        .await?;

        let entries = rows.into_iter().map(|r| {
            SheetAclEntry {
                id: r.get("id"),
                sheet_id: r.get("sheet_id"),
                grantee_type: r.get("grantee_type"),
                grantee_id: r.get("grantee_id"),
                grantee_name: r.get("grantee_name"),
                permission: r.get("permission"),
                created_at: r.get("created_at"),
            }
        }).collect();

        Ok(entries)
    }

    pub async fn grant(
        pool: &SqlitePool,
        acl_id: &str,
        sheet_id: &str,
        grantee_type: &str,
        grantee_id: Option<&str>,
        permission: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO sheet_acls (id, sheet_id, grantee_type, grantee_id, permission)
             VALUES (?, ?, ?, ?, ?)
             ON CONFLICT(sheet_id, grantee_type, grantee_id) DO UPDATE SET permission = excluded.permission"
        )
        .bind(acl_id)
        .bind(sheet_id)
        .bind(grantee_type)
        .bind(grantee_id)
        .bind(permission)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find_sheet_id_by_acl(pool: &SqlitePool, acl_id: &str) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar::<_, String>("SELECT sheet_id FROM sheet_acls WHERE id = ?")
            .bind(acl_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn revoke(pool: &SqlitePool, acl_id: &str) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM sheet_acls WHERE id = ?")
            .bind(acl_id)
            .execute(pool)
            .await?;

        Ok(res.rows_affected())
    }

    pub async fn get_share_settings(pool: &SqlitePool, sheet_id: &str) -> Result<Option<SheetShareSettings>, sqlx::Error> {
        let row = sqlx::query("SELECT id, name, share_token, share_permission, is_public FROM character_sheets WHERE id = ?")
            .bind(sheet_id)
            .fetch_optional(pool)
            .await?;

        if let Some(r) = row {
            let name: String = r.get("name");
            let mut share_token: Option<String> = r.try_get("share_token").unwrap_or(None);
            let share_permission = r.try_get::<String, _>("share_permission").unwrap_or_else(|_| "none".to_string());
            let is_public: bool = r.get::<i32, _>("is_public") == 1;

            // Auto-gera token caso não exista ainda
            if share_token.is_none() || share_token.as_deref().unwrap_or_default().is_empty() {
                let new_tok = Uuid::new_v4().to_string();
                let _ = sqlx::query("UPDATE character_sheets SET share_token = ? WHERE id = ?")
                    .bind(&new_tok)
                    .bind(sheet_id)
                    .execute(pool)
                    .await;
                share_token = Some(new_tok);
            }

            let acls = Self::list_for_sheet(pool, sheet_id).await?;

            Ok(Some(SheetShareSettings {
                sheet_id: sheet_id.to_string(),
                sheet_name: name,
                share_token,
                share_permission,
                is_public,
                acls,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn update_share_link(
        pool: &SqlitePool,
        sheet_id: &str,
        share_permission: &str,
        regenerate_token: bool,
    ) -> Result<Option<SheetShareSettings>, sqlx::Error> {
        let current = Self::get_share_settings(pool, sheet_id).await?;
        if current.is_none() {
            return Ok(None);
        }

        let new_token = if regenerate_token {
            Some(Uuid::new_v4().to_string())
        } else {
            None
        };

        if let Some(ref tok) = new_token {
            sqlx::query("UPDATE character_sheets SET share_permission = ?, share_token = ? WHERE id = ?")
                .bind(share_permission)
                .bind(tok)
                .bind(sheet_id)
                .execute(pool)
                .await?;
        } else {
            sqlx::query("UPDATE character_sheets SET share_permission = ? WHERE id = ?")
                .bind(share_permission)
                .bind(sheet_id)
                .execute(pool)
                .await?;
        }

        Self::get_share_settings(pool, sheet_id).await
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
}
