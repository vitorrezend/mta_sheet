#[cfg(feature = "ssr")]
use sqlx::{SqlitePool, Row};
#[cfg(feature = "ssr")]
use crate::state::models::{CharacterData, CharacterSummary, QuizQuestionEntry};

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
            "SELECT id, user_id, name, summary_json, sheet_type, is_public, updated_at, \
             CASE WHEN summary_json IS NULL OR summary_json = '' THEN data ELSE '' END as fallback_data \
             FROM character_sheets WHERE is_public = 1 ORDER BY updated_at DESC LIMIT 100"
        )
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

            Self::parse_summary_or_fallback(
                &id,
                &name,
                summary_opt.as_deref(),
                &fallback_data,
                &sheet_type,
                &updated_at,
                true,
                is_owner,
                &mut to_backfill,
            )
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
        let res = sqlx::query("DELETE FROM character_sheets WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

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

                    return Ok(Err("Permissão negada: Você não é o proprietário desta ficha"));
                } else {
                    return Ok(Err("Autenticação necessária para alterar esta ficha"));
                }
            }
        }

        Ok(Ok(()))
    }
}
