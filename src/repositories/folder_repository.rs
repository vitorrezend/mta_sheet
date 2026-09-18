#[cfg(feature = "ssr")]
use sqlx::{SqlitePool, Row};
#[cfg(feature = "ssr")]
use crate::state::models::SheetFolder;

pub struct FolderRepository;

#[cfg(feature = "ssr")]
impl FolderRepository {
    pub async fn list_by_user(pool: &SqlitePool, user_id: &str) -> Result<Vec<SheetFolder>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT f.id, f.user_id, f.parent_id, f.name, f.icon, f.color, f.sort_order, f.created_at, 
                    COUNT(s.id) as sheet_count
             FROM sheet_folders f
             LEFT JOIN character_sheets s ON s.folder_id = f.id
             WHERE f.user_id = ?
             GROUP BY f.id
             ORDER BY f.sort_order ASC, f.created_at ASC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        let folders = rows.into_iter().map(|row| {
            SheetFolder {
                id: row.get("id"),
                user_id: row.get("user_id"),
                parent_id: row.get("parent_id"),
                name: row.get("name"),
                icon: row.get("icon"),
                color: row.get("color"),
                sort_order: row.get("sort_order"),
                sheet_count: row.get::<i64, _>("sheet_count"),
                created_at: row.get("created_at"),
            }
        }).collect();

        Ok(folders)
    }

    pub async fn find_owner_id(pool: &SqlitePool, folder_id: &str) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar::<_, String>("SELECT user_id FROM sheet_folders WHERE id = ?")
            .bind(folder_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn count_by_user(pool: &SqlitePool, user_id: &str) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM sheet_folders WHERE user_id = ?")
            .bind(user_id)
            .fetch_one(pool)
            .await
            .unwrap_or(0);
        Ok(count)
    }

    pub async fn create(
        pool: &SqlitePool,
        id: &str,
        user_id: &str,
        parent_id: Option<&str>,
        name: &str,
        icon: &str,
        color: &str,
        sort_order: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO sheet_folders (id, user_id, parent_id, name, icon, color, sort_order) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id)
        .bind(user_id)
        .bind(parent_id)
        .bind(name)
        .bind(icon)
        .bind(color)
        .bind(sort_order)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update(
        pool: &SqlitePool,
        folder_id: &str,
        user_id: &str,
        name: &str,
        icon: &str,
        color: &str,
    ) -> Result<u64, sqlx::Error> {
        let res = sqlx::query(
            "UPDATE sheet_folders SET name = ?, icon = ?, color = ? WHERE id = ? AND user_id = ?"
        )
        .bind(name)
        .bind(icon)
        .bind(color)
        .bind(folder_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(res.rows_affected())
    }

    pub async fn is_descendant(pool: &SqlitePool, folder_id: &str, target_id: &str) -> Result<bool, sqlx::Error> {
        let row = sqlx::query(
            "WITH RECURSIVE ancestry AS (
                SELECT id, parent_id FROM sheet_folders WHERE id = ?
                UNION ALL
                SELECT f.id, f.parent_id FROM sheet_folders f JOIN ancestry a ON f.id = a.parent_id
            )
            SELECT 1 FROM ancestry WHERE id = ? LIMIT 1"
        )
        .bind(target_id)
        .bind(folder_id)
        .fetch_optional(pool)
        .await?;

        Ok(row.is_some())
    }

    pub async fn move_folder(
        pool: &SqlitePool,
        folder_id: &str,
        target_parent_id: Option<&str>,
        user_id: &str,
    ) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("UPDATE sheet_folders SET parent_id = ? WHERE id = ? AND user_id = ?")
            .bind(target_parent_id)
            .bind(folder_id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(res.rows_affected())
    }

    pub async fn delete(pool: &SqlitePool, folder_id: &str, user_id: &str) -> Result<u64, sqlx::Error> {
        // Safely unlink all character sheets in this folder AND all its recursive subfolders to root
        let _ = sqlx::query(
            "WITH RECURSIVE subfolders AS (
                SELECT id FROM sheet_folders WHERE id = ? AND user_id = ?
                UNION ALL
                SELECT f.id FROM sheet_folders f JOIN subfolders s ON f.parent_id = s.id
            )
            UPDATE character_sheets SET folder_id = NULL WHERE folder_id IN (SELECT id FROM subfolders)"
        )
        .bind(folder_id)
        .bind(user_id)
        .execute(pool)
        .await;

        let res = sqlx::query("DELETE FROM sheet_folders WHERE id = ? AND user_id = ?")
            .bind(folder_id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(res.rows_affected())
    }
}
