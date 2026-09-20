#[cfg(feature = "ssr")]
use sqlx::{SqlitePool, Row};
#[cfg(feature = "ssr")]
use crate::state::models::{CharacterData, CharacterSummary, QuizQuestionEntry, SystemStats};

pub struct SheetRepository;

#[cfg(feature = "ssr")]
impl SheetRepository {
    pub fn parse_summary_or_fallback(
        id: &str,
        name: &str,
        summary_opt: Option<&str>,
        fallback_data: &str,
        sheet_type: &str,
        updated_at: &str,
        is_public: bool,
        is_owner: bool,
        to_backfill: &mut Vec<(String, String)>,
    ) -> CharacterSummary {
        if let Some(summary_str) = summary_opt.filter(|s| !s.trim().is_empty()) {
            if let Ok(mut s) = serde_json::from_str::<CharacterSummary>(summary_str) {
                s.id = id.to_string();
                s.updated_at = updated_at.to_string();
                s.is_public = is_public;
                s.is_owner = is_owner;
                if !sheet_type.is_empty() {
                    s.sheet_type = sheet_type.to_string();
                }
                return s;
            }
        }

        // Fallback para linhas legadas que ainda não possuem summary_json
        if let Some(mut data) = CharacterData::parse_from_db(id, fallback_data) {
            if data.id.is_empty() {
                data.id = id.to_string();
            }
            if data.name.is_empty() || (data.name == "Novo Mago" && !name.is_empty() && name != "Novo Mago") {
                data.set_display_name(name);
            }
            data.sanitize();
            if (data.sheet_type.is_empty() || data.sheet_type == "mage") && !sheet_type.is_empty() && sheet_type != "mage" {
                data.sheet_type = sheet_type.to_string();
            }
            let summary = data.to_summary(updated_at.to_string(), is_public, is_owner);
            if let Ok(s_json) = serde_json::to_string(&summary) {
                to_backfill.push((id.to_string(), s_json));
            }
            summary
        } else {
            let clean_name = if name.trim().is_empty() { "Novo Mago".to_string() } else { name.to_string() };
            CharacterSummary::fallback(id.to_string(), clean_name, updated_at.to_string(), is_public, is_owner)
        }
    }

    pub async fn list_by_user(pool: &SqlitePool, user_id: &str) -> Result<Vec<CharacterSummary>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, name, summary_json, is_public, updated_at, folder_id, sheet_type, \
             CASE WHEN summary_json IS NULL OR summary_json = '' THEN data ELSE '' END as fallback_data \
             FROM character_sheets WHERE user_id = ? ORDER BY updated_at DESC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        let mut to_backfill: Vec<(String, String)> = Vec::new();

        let summaries: Vec<CharacterSummary> = rows.into_iter().map(|row| {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let summary_opt: Option<String> = row.try_get("summary_json").unwrap_or(None);
            let fallback_data: String = row.try_get("fallback_data").unwrap_or_default();
            let is_public: bool = row.get::<i32, _>("is_public") == 1;
            let updated_at: String = row.get("updated_at");
            let sheet_type = row.try_get::<String, _>("sheet_type").unwrap_or_else(|_| "mage".to_string());
            let folder_id = row.try_get::<Option<String>, _>("folder_id").unwrap_or(None);

            let mut summary = Self::parse_summary_or_fallback(
                &id,
                &name,
                summary_opt.as_deref(),
                &fallback_data,
                &sheet_type,
                &updated_at,
                is_public,
                true,
                &mut to_backfill,
            );
            summary.folder_id = folder_id;
            summary
        }).collect();

        // Auto-migra em segundo plano as fichas legadas
        for (sheet_id, s_json) in to_backfill {
            let _ = sqlx::query("UPDATE character_sheets SET summary_json = ? WHERE id = ?")
                .bind(s_json)
                .bind(sheet_id)
                .execute(pool)
                .await;
        }

        Ok(summaries)
    }

    pub async fn list_public(pool: &SqlitePool, auth_user_id: Option<&str>) -> Result<Vec<CharacterSummary>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT cs.id, cs.user_id, cs.name, cs.summary_json, cs.sheet_type, cs.is_public, cs.updated_at, \
             u.username as author_username, \
             (SELECT COUNT(*) FROM sheet_likes sl WHERE sl.sheet_id = cs.id) as likes_count, \
             CASE WHEN ? IS NOT NULL AND EXISTS(SELECT 1 FROM sheet_likes sl WHERE sl.sheet_id = cs.id AND sl.user_id = ?) THEN 1 ELSE 0 END as is_liked, \
             CASE WHEN cs.summary_json IS NULL OR cs.summary_json = '' THEN cs.data ELSE '' END as fallback_data \
             FROM character_sheets cs \
             LEFT JOIN users u ON cs.user_id = u.id \
             WHERE cs.is_public = 1 ORDER BY cs.updated_at DESC LIMIT 100"
        )
        .bind(auth_user_id)
        .bind(auth_user_id)
        .fetch_all(pool)
        .await?;

        let mut to_backfill: Vec<(String, String)> = Vec::new();

        let summaries: Vec<CharacterSummary> = rows.into_iter().map(|row| {
            let id: String = row.get("id");
            let owner_id: Option<String> = row.get("user_id");
            let name: String = row.get("name");
            let summary_opt: Option<String> = row.try_get("summary_json").unwrap_or(None);
            let fallback_data: String = row.try_get("fallback_data").unwrap_or_default();
            let updated_at: String = row.get("updated_at");
            let sheet_type = row.try_get::<String, _>("sheet_type").unwrap_or_else(|_| "mage".to_string());
            let is_owner = auth_user_id.is_some() && auth_user_id == owner_id.as_deref();
            let author_username: Option<String> = row.try_get("author_username").unwrap_or(None);
            let likes_count: i64 = row.try_get::<i64, _>("likes_count").unwrap_or(0);
            let is_liked: bool = row.try_get::<i32, _>("is_liked").unwrap_or(0) == 1;

            let mut summary = Self::parse_summary_or_fallback(
                &id,
                &name,
                summary_opt.as_deref(),
                &fallback_data,
                &sheet_type,
                &updated_at,
                true,
                is_owner,
                &mut to_backfill,
            );
            summary.author_username = author_username;
            summary.likes_count = likes_count;
            summary.is_liked = is_liked;
            summary
        }).collect();

        for (sheet_id, s_json) in to_backfill {
            let _ = sqlx::query("UPDATE character_sheets SET summary_json = ? WHERE id = ?")
                .bind(s_json)
                .bind(sheet_id)
                .execute(pool)
                .await;
        }

        Ok(summaries)
    }

    pub async fn toggle_like(
        pool: &SqlitePool,
        sheet_id: &str,
        user_id: &str,
    ) -> Result<(bool, i64), sqlx::Error> {
        let exists: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sheet_likes WHERE sheet_id = ? AND user_id = ?"
        )
        .bind(sheet_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        let is_liked = if exists > 0 {
            sqlx::query("DELETE FROM sheet_likes WHERE sheet_id = ? AND user_id = ?")
                .bind(sheet_id)
                .bind(user_id)
                .execute(pool)
                .await?;
            false
        } else {
            sqlx::query("INSERT INTO sheet_likes (sheet_id, user_id) VALUES (?, ?)")
                .bind(sheet_id)
                .bind(user_id)
                .execute(pool)
                .await?;
            true
        };

        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sheet_likes WHERE sheet_id = ?"
        )
        .bind(sheet_id)
        .fetch_one(pool)
        .await?;

        Ok((is_liked, count))
    }

    pub async fn count_likes(pool: &SqlitePool, sheet_id: &str) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar("SELECT COUNT(*) FROM sheet_likes WHERE sheet_id = ?")
            .bind(sheet_id)
            .fetch_one(pool)
            .await
    }

    pub async fn is_liked(pool: &SqlitePool, sheet_id: &str, user_id: &str) -> Result<bool, sqlx::Error> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sheet_likes WHERE sheet_id = ? AND user_id = ?"
        )
        .bind(sheet_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;
        Ok(count > 0)
    }

    pub async fn find_raw_by_id(
        pool: &SqlitePool,
        id: &str,
    ) -> Result<Option<(Option<String>, Option<String>, String, String, bool)>, sqlx::Error> {
        let row = sqlx::query("SELECT user_id, room_id, data, sheet_type, is_public FROM character_sheets WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        match row {
            Some(r) => {
                let user_id: Option<String> = r.get("user_id");
                let room_id: Option<String> = r.get("room_id");
                let data_json: String = r.get("data");
                let sheet_type: String = r.try_get::<String, _>("sheet_type").unwrap_or_else(|_| "mage".to_string());
                let is_public: bool = r.get::<i32, _>("is_public") == 1;
                Ok(Some((user_id, room_id, data_json, sheet_type, is_public)))
            }
            None => Ok(None),
        }
    }

    pub async fn check_room_gm(pool: &SqlitePool, room_id: &str, user_id: &str) -> Result<bool, sqlx::Error> {
        let room = sqlx::query("SELECT gm_id FROM rooms WHERE id = ?")
            .bind(room_id)
            .fetch_optional(pool)
            .await?;

        Ok(room.map(|r| r.get::<String, _>("gm_id") == user_id).unwrap_or(false))
    }

    pub async fn find_quiz_answers(pool: &SqlitePool, character_id: &str) -> Result<Vec<(String, String)>, sqlx::Error> {
        let rows = sqlx::query("SELECT question_id, answer FROM character_quiz_answers WHERE character_id = ?")
            .bind(character_id)
            .fetch_all(pool)
            .await?;

        Ok(rows.into_iter().map(|r| (r.get("question_id"), r.get("answer"))).collect())
    }

    pub async fn count_by_user(pool: &SqlitePool, user_id: &str) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM character_sheets WHERE user_id = ?")
            .bind(user_id)
            .fetch_one(pool)
            .await
            .unwrap_or(0);
        Ok(count)
    }

    pub async fn check_folder_exists_for_user(pool: &SqlitePool, folder_id: &str, user_id: &str) -> Result<bool, sqlx::Error> {
        let exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM sheet_folders WHERE id = ? AND user_id = ?")
            .bind(folder_id)
            .bind(user_id)
            .fetch_one(pool)
            .await
            .unwrap_or(0);
        Ok(exists > 0)
    }

    pub async fn create(
        pool: &SqlitePool,
        id: &str,
        user_id: Option<&str>,
        name: &str,
        data_json: &str,
        sheet_type: &str,
        folder_id: Option<&str>,
        summary_json: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO character_sheets (id, user_id, name, data, sheet_type, folder_id, summary_json) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(id)
            .bind(user_id)
            .bind(name)
            .bind(data_json)
            .bind(sheet_type)
            .bind(folder_id)
            .bind(summary_json)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn update(
        pool: &SqlitePool,
        id: &str,
        name: &str,
        data_json: &str,
        sheet_type: &str,
        is_public: bool,
        summary_json: &str,
    ) -> Result<u64, sqlx::Error> {
        let is_public_int = if is_public { 1 } else { 0 };
        let res = sqlx::query("UPDATE character_sheets SET name = ?, data = ?, sheet_type = ?, is_public = ?, summary_json = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(name)
            .bind(data_json)
            .bind(sheet_type)
            .bind(is_public_int)
            .bind(summary_json)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(res.rows_affected())
    }

    pub async fn sync_quiz_answers(pool: &SqlitePool, sheet_id: &str, entries: &[QuizQuestionEntry]) -> Result<(), sqlx::Error> {
        for entry in entries {
            let clean_ans = entry.answer.trim();
            if clean_ans.is_empty() {
                let _ = sqlx::query("DELETE FROM character_quiz_answers WHERE character_id = ? AND question_id = ?")
                    .bind(sheet_id)
                    .bind(&entry.id)
                    .execute(pool)
                    .await;
            } else {
                let _ = sqlx::query(
                    "INSERT INTO character_quiz_answers (character_id, question_id, answer, updated_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP)
                     ON CONFLICT(character_id, question_id) DO UPDATE SET answer = excluded.answer, updated_at = CURRENT_TIMESTAMP"
                )
                .bind(sheet_id)
                .bind(&entry.id)
                .bind(clean_ans)
                .execute(pool)
                .await;
            }
        }
        Ok(())
    }

    pub async fn set_visibility(pool: &SqlitePool, id: &str, is_public: bool) -> Result<u64, sqlx::Error> {
        let is_public_int = if is_public { 1 } else { 0 };
        let res = sqlx::query("UPDATE character_sheets SET is_public = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(is_public_int)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(res.rows_affected())
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<u64, sqlx::Error> {
        let room_id: Option<String> = sqlx::query_scalar("SELECT room_id FROM character_sheets WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .unwrap_or_default();

        let res = sqlx::query("DELETE FROM character_sheets WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        if let Some(r_id) = room_id {
            if let Ok(Some(row)) = sqlx::query("SELECT initiative_data, sheet_order FROM rooms WHERE id = ?")
                .bind(&r_id)
                .fetch_optional(pool)
                .await
            {
                let init_raw: String = row.try_get("initiative_data").unwrap_or_default();
                if !init_raw.is_empty() {
                    if let Ok(mut init) = serde_json::from_str::<crate::rooms::RoomInitiativeData>(&init_raw) {
                        let old_len = init.entries.len();
                        init.entries.retain(|e| e.id != id);
                        if init.entries.len() != old_len {
                            if let Ok(new_json) = serde_json::to_string(&init) {
                                let _ = sqlx::query("UPDATE rooms SET initiative_data = ? WHERE id = ?")
                                    .bind(&new_json)
                                    .bind(&r_id)
                                    .execute(pool)
                                    .await;
                            }
                        }
                    }
                }

                let order_raw: String = row.try_get("sheet_order").unwrap_or_default();
                if !order_raw.is_empty() {
                    if let Ok(mut order) = serde_json::from_str::<Vec<String>>(&order_raw) {
                        let old_len = order.len();
                        order.retain(|sheet_id| sheet_id != id);
                        if order.len() != old_len {
                            if let Ok(new_order) = serde_json::to_string(&order) {
                                let _ = sqlx::query("UPDATE rooms SET sheet_order = ? WHERE id = ?")
                                    .bind(&new_order)
                                    .bind(&r_id)
                                    .execute(pool)
                                    .await;
                            }
                        }
                    }
                }
            }
        }

        Ok(res.rows_affected())
    }

    pub async fn move_to_folder(pool: &SqlitePool, sheet_id: &str, folder_id: Option<&str>, user_id: &str) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("UPDATE character_sheets SET folder_id = ? WHERE id = ? AND user_id = ?")
            .bind(folder_id)
            .bind(sheet_id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(res.rows_affected())
    }

    pub async fn verify_write_permission(pool: &SqlitePool, sheet_id: &str, auth_user_id: Option<&str>) -> Result<Result<(), &'static str>, sqlx::Error> {
        let row = sqlx::query("SELECT user_id, room_id FROM character_sheets WHERE id = ?")
            .bind(sheet_id)
            .fetch_optional(pool)
            .await?;

        if let Some(r) = row {
            let sheet_owner: Option<String> = r.get("user_id");
            let room_id: Option<String> = r.get("room_id");

            if let Some(owner_id) = sheet_owner {
                if let Some(user_id) = auth_user_id {
                    if user_id == owner_id {
                        return Ok(Ok(()));
                    }

                    if let Some(r_id) = room_id {
                        let is_gm = sqlx::query("SELECT 1 FROM rooms WHERE id = ? AND gm_id = ?")
                            .bind(r_id)
                            .bind(user_id)
                            .fetch_optional(pool)
                            .await?;
                        if is_gm.is_some() {
                            return Ok(Ok(()));
                        }
                    }

                    // Checa se o usuário possui permissão de edição em sheet_acls
                    if let Ok(true) = crate::repositories::SheetAclRepository::check_sheet_permission(pool, sheet_id, user_id, "write").await {
                        return Ok(Ok(()));
                    }

                    return Ok(Err("Permissão negada: Você não possui permissão para alterar esta ficha"));
                } else {
                    return Ok(Err("Autenticação necessária para alterar esta ficha"));
                }
            }
        }

        Ok(Ok(()))
    }

    pub async fn find_username_by_id(pool: &SqlitePool, user_id: &str) -> Result<Option<String>, sqlx::Error> {
        let row = sqlx::query("SELECT username FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;
        Ok(row.map(|r| r.get("username")))
    }

    pub async fn find_user_by_username(pool: &SqlitePool, username: &str) -> Result<Option<(String, String, String)>, sqlx::Error> {
        let row = sqlx::query("SELECT id, username, created_at FROM users WHERE LOWER(username) = LOWER(?)")
            .bind(username)
            .fetch_optional(pool)
            .await?;
        Ok(row.map(|r| (r.get("id"), r.get("username"), r.try_get("created_at").unwrap_or_default())))
    }

    pub async fn find_user_by_id(pool: &SqlitePool, user_id: &str) -> Result<Option<(String, String, String)>, sqlx::Error> {
        let row = sqlx::query("SELECT id, username, created_at FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;
        Ok(row.map(|r| (r.get("id"), r.get("username"), r.try_get("created_at").unwrap_or_default())))
    }

    pub async fn count_rooms_by_user(pool: &SqlitePool, user_id: &str) -> Result<i64, sqlx::Error> {
        let row = sqlx::query(
            "SELECT COUNT(DISTINCT r.id) as count FROM rooms r \
             LEFT JOIN room_members rm ON r.id = rm.room_id \
             WHERE r.gm_id = ? OR rm.user_id = ?"
        )
        .bind(user_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;
        Ok(row.get("count"))
    }

    pub async fn count_public_by_user(pool: &SqlitePool, user_id: &str) -> Result<i64, sqlx::Error> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM character_sheets WHERE user_id = ? AND is_public = 1")
            .bind(user_id)
            .fetch_one(pool)
            .await?;
        Ok(row.get("count"))
    }

    pub async fn list_by_user_public_only(pool: &SqlitePool, user_id: &str, author_username: Option<&str>) -> Result<Vec<CharacterSummary>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, name, summary_json, is_public, updated_at, folder_id, sheet_type, \
             CASE WHEN summary_json IS NULL OR summary_json = '' THEN data ELSE '' END as fallback_data \
             FROM character_sheets WHERE user_id = ? AND is_public = 1 ORDER BY updated_at DESC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        let mut to_backfill: Vec<(String, String)> = Vec::new();

        let summaries: Vec<CharacterSummary> = rows.into_iter().map(|row| {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let summary_opt: Option<String> = row.try_get("summary_json").unwrap_or(None);
            let fallback_data: String = row.try_get("fallback_data").unwrap_or_default();
            let updated_at: String = row.get("updated_at");
            let sheet_type = row.try_get::<String, _>("sheet_type").unwrap_or_else(|_| "mage".to_string());
            let folder_id = row.try_get::<Option<String>, _>("folder_id").unwrap_or(None);

            let mut summary = Self::parse_summary_or_fallback(
                &id,
                &name,
                summary_opt.as_deref(),
                &fallback_data,
                &sheet_type,
                &updated_at,
                true,
                false,
                &mut to_backfill,
            );
            summary.folder_id = folder_id;
            summary.author_username = author_username.map(|s| s.to_string());
            summary
        }).collect();

        for (sheet_id, s_json) in to_backfill {
            let _ = sqlx::query("UPDATE character_sheets SET summary_json = ? WHERE id = ?")
                .bind(s_json)
                .bind(sheet_id)
                .execute(pool)
                .await;
        }

        Ok(summaries)
    }

    pub async fn count_sheets_by_splat(pool: &SqlitePool, user_id: &str) -> Result<(i64, i64), sqlx::Error> {
        let rows = sqlx::query("SELECT sheet_type, COUNT(*) as count FROM character_sheets WHERE user_id = ? GROUP BY sheet_type")
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        let mut mage_count = 0i64;
        let mut gm_count = 0i64;
        for r in rows {
            let st: String = r.try_get("sheet_type").unwrap_or_else(|_| "mage".to_string());
            let cnt: i64 = r.try_get("count").unwrap_or(0);
            if st == "gods_and_monsters" || st == "gm" {
                gm_count += cnt;
            } else {
                mage_count += cnt;
            }
        }
        Ok((mage_count, gm_count))
    }

    pub async fn count_folders_by_user(pool: &SqlitePool, user_id: &str) -> Result<i64, sqlx::Error> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM sheet_folders WHERE user_id = ?")
            .bind(user_id)
            .fetch_one(pool)
            .await?;
        Ok(row.try_get("count").unwrap_or(0))
    }

    pub async fn count_player_rooms_by_user(pool: &SqlitePool, user_id: &str) -> Result<i64, sqlx::Error> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM room_members WHERE user_id = ? AND role != 'gm'")
            .bind(user_id)
            .fetch_one(pool)
            .await?;
        Ok(row.try_get("count").unwrap_or(0))
    }

    pub async fn count_active_sessions_by_user(pool: &SqlitePool, user_id: &str) -> Result<i64, sqlx::Error> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM sessions WHERE user_id = ? AND expires_at > CURRENT_TIMESTAMP")
            .bind(user_id)
            .fetch_one(pool)
            .await?;
        Ok(row.try_get("count").unwrap_or(0))
    }

    pub async fn get_system_stats(pool: &SqlitePool) -> Result<SystemStats, sqlx::Error> {
        let row = sqlx::query(
            "SELECT 
                (SELECT COUNT(*) FROM users) AS total_users,
                (SELECT COUNT(*) FROM rooms) AS total_rooms,
                (SELECT COUNT(*) FROM character_sheets WHERE (sheet_type = 'mage' OR sheet_type = '' OR sheet_type IS NULL) AND is_public = 1) AS mage_public,
                (SELECT COUNT(*) FROM character_sheets WHERE (sheet_type = 'mage' OR sheet_type = '' OR sheet_type IS NULL) AND is_public = 0) AS mage_private,
                (SELECT COUNT(*) FROM character_sheets WHERE (sheet_type = 'gods_and_monsters' OR sheet_type = 'gm') AND is_public = 1) AS gm_public,
                (SELECT COUNT(*) FROM character_sheets WHERE (sheet_type = 'gods_and_monsters' OR sheet_type = 'gm') AND is_public = 0) AS gm_private"
        )
        .fetch_one(pool)
        .await?;

        Ok(SystemStats {
            total_users: row.try_get("total_users").unwrap_or(0),
            total_rooms: row.try_get("total_rooms").unwrap_or(0),
            mage_sheets_public: row.try_get("mage_public").unwrap_or(0),
            mage_sheets_private: row.try_get("mage_private").unwrap_or(0),
            gm_sheets_public: row.try_get("gm_public").unwrap_or(0),
            gm_sheets_private: row.try_get("gm_private").unwrap_or(0),
        })
    }
}


