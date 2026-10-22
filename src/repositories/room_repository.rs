#[cfg(feature = "ssr")]
use sqlx::{SqlitePool, Row};
#[cfg(feature = "ssr")]
use crate::rooms::models::{RoomSummary, RoomMemberInfo, RoomChatMessage, ChatRollData};

pub struct RoomRepository;

#[cfg(feature = "ssr")]
impl RoomRepository {
    pub async fn list_user_rooms(pool: &SqlitePool, user_id: &str) -> Result<Vec<RoomSummary>, sqlx::Error> {
        let query = "
            SELECT 
                r.id, r.name, r.code, r.description, r.created_at, r.is_public,
                (r.password_hash IS NOT NULL AND r.password_hash != '') as has_password,
                u.username as gm_username,
                (r.gm_id = ?) as is_gm,
                (SELECT COUNT(*) FROM room_members rm WHERE rm.room_id = r.id) as member_count,
                (SELECT COUNT(*) FROM character_sheets cs WHERE cs.room_id = r.id) as sheet_count
            FROM rooms r
            JOIN users u ON r.gm_id = u.id
            LEFT JOIN room_members rm ON rm.room_id = r.id AND rm.user_id = ?
            WHERE r.gm_id = ? OR rm.user_id IS NOT NULL
            GROUP BY r.id
            ORDER BY r.created_at DESC
        ";

        let rows = sqlx::query(query)
            .bind(user_id)
            .bind(user_id)
            .bind(user_id)
            .fetch_all(pool)
            .await?;

        let summaries = rows.into_iter().map(|row| RoomSummary {
            id: row.get("id"),
            name: row.get("name"),
            code: row.get("code"),
            description: row.get("description"),
            gm_username: row.get("gm_username"),
            is_gm: row.get("is_gm"),
            is_public: row.try_get::<i64, _>("is_public").map(|v| v == 1).unwrap_or(false),
            has_password: row.get("has_password"),
            member_count: row.get("member_count"),
            sheet_count: row.get("sheet_count"),
            created_at: row.get("created_at"),
        }).collect();

        Ok(summaries)
    }

    pub async fn list_public_rooms(pool: &SqlitePool, _current_user_id: &str, limit: i64) -> Result<Vec<RoomSummary>, sqlx::Error> {
        let query = "
            SELECT 
                r.id, r.name, r.code, r.description, r.created_at, r.is_public,
                (r.password_hash IS NOT NULL AND r.password_hash != '') as has_password,
                u.username as gm_username,
                (r.gm_id = ?) as is_gm,
                (SELECT COUNT(*) FROM room_members rm WHERE rm.room_id = r.id) as member_count,
                (SELECT COUNT(*) FROM character_sheets cs WHERE cs.room_id = r.id) as sheet_count
            FROM rooms r
            JOIN users u ON r.gm_id = u.id
            WHERE r.is_public = 1
            ORDER BY r.created_at DESC
            LIMIT ?
        ";

        let rows = sqlx::query(query)
            .bind(_current_user_id)
            .bind(limit)
            .fetch_all(pool)
            .await?;

        let summaries = rows.into_iter().map(|row| RoomSummary {
            id: row.get("id"),
            name: row.get("name"),
            code: row.get("code"),
            description: row.get("description"),
            gm_username: row.get("gm_username"),
            is_gm: row.get("is_gm"),
            is_public: true,
            has_password: row.get("has_password"),
            member_count: row.get("member_count"),
            sheet_count: row.get("sheet_count"),
            created_at: row.get("created_at"),
        }).collect();

        Ok(summaries)
    }

    pub async fn create_room(
        pool: &SqlitePool,
        room_id: &str,
        name: &str,
        code: &str,
        description: &str,
        user_id: &str,
        is_public: bool,
        password_hash: &str,
    ) -> Result<(), sqlx::Error> {
        let is_pub_val: i64 = if is_public { 1 } else { 0 };

        sqlx::query(
            "INSERT INTO rooms (id, name, code, description, gm_id, is_public, password_hash) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(room_id)
        .bind(name)
        .bind(code)
        .bind(description)
        .bind(user_id)
        .bind(is_pub_val)
        .bind(password_hash)
        .execute(pool)
        .await?;

        // Add GM to room members
        sqlx::query(
            "INSERT INTO room_members (room_id, user_id, role) VALUES (?, ?, 'gm')"
        )
        .bind(room_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn add_member_if_not_exists(
        pool: &SqlitePool,
        room_id: &str,
        user_id: &str,
        role: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT OR IGNORE INTO room_members (room_id, user_id, role) VALUES (?, ?, ?)"
        )
        .bind(room_id)
        .bind(user_id)
        .bind(role)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_room_members(
        pool: &SqlitePool,
        room_id: &str,
    ) -> Result<Vec<RoomMemberInfo>, sqlx::Error> {
        let member_rows = sqlx::query(
            "SELECT rm.user_id, u.username, rm.role, rm.joined_at 
             FROM room_members rm 
             JOIN users u ON rm.user_id = u.id 
             WHERE rm.room_id = ? 
             ORDER BY rm.role DESC, rm.joined_at ASC"
        )
        .bind(room_id)
        .fetch_all(pool)
        .await?;

        Ok(member_rows.into_iter().map(|r| RoomMemberInfo {
            user_id: r.get("user_id"),
            username: r.get("username"),
            role: r.get("role"),
            joined_at: r.get("joined_at"),
        }).collect())
    }

    pub async fn check_is_gm(pool: &SqlitePool, room_id: &str, user_id: &str) -> Result<bool, sqlx::Error> {
        let is_gm: bool = sqlx::query_scalar::<_, i64>("SELECT 1 FROM rooms WHERE id = ? AND gm_id = ?")
            .bind(room_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await?
            .is_some();
        Ok(is_gm)
    }

    /// Mestre da sala ou membro registrado em `room_members`.
    pub async fn check_is_gm_or_member(pool: &SqlitePool, room_id: &str, user_id: &str) -> Result<bool, sqlx::Error> {
        let allowed: bool = sqlx::query_scalar::<_, i64>(
            "SELECT 1 FROM rooms WHERE id = ? AND gm_id = ?
             UNION
             SELECT 1 FROM room_members WHERE room_id = ? AND user_id = ?"
        )
        .bind(room_id)
        .bind(user_id)
        .bind(room_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .is_some();
        Ok(allowed)
    }
    pub async fn save_room_message(pool: &SqlitePool, msg: &RoomChatMessage) -> Result<(), sqlx::Error> {
        let roll_json = msg.roll_data.as_ref().map(|r| serde_json::to_string(r).unwrap_or_default());
        sqlx::query(
            "INSERT INTO room_messages (id, room_id, sender_id, sender_name, sender_character, sender_avatar, content, message_type, roll_data, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))"
        )
        .bind(&msg.id)
        .bind(&msg.room_id)
        .bind(&msg.sender_id)
        .bind(&msg.sender_name)
        .bind(&msg.sender_character)
        .bind(&msg.sender_avatar)
        .bind(&msg.content)
        .bind(&msg.message_type)
        .bind(roll_json)
        .execute(pool)
        .await?;

        // Limpeza oportunística: mantém no máximo 200 mensagens por sala para economia de espaço
        let _ = sqlx::query(
            "DELETE FROM room_messages WHERE id NOT IN (
                SELECT id FROM room_messages WHERE room_id = ? ORDER BY created_at DESC, rowid DESC LIMIT 200
            ) AND room_id = ?"
        )
        .bind(&msg.room_id)
        .bind(&msg.room_id)
        .execute(pool)
        .await;

        Ok(())
    }

    pub async fn get_room_messages(pool: &SqlitePool, room_id: &str, limit: i64) -> Result<Vec<RoomChatMessage>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, room_id, sender_id, sender_name, sender_character, sender_avatar, content, message_type, roll_data, created_at
             FROM (
                 SELECT id, room_id, sender_id, sender_name, sender_character, sender_avatar, content, message_type, roll_data, created_at
                 FROM room_messages
                 WHERE room_id = ?
                 ORDER BY created_at DESC, rowid DESC
                 LIMIT ?
             )
             ORDER BY created_at ASC, id ASC"
        )
        .bind(room_id)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        let msgs = rows.into_iter().map(|r| {
            let roll_str: Option<String> = r.get("roll_data");
            let roll_data = roll_str.and_then(|s| serde_json::from_str::<ChatRollData>(&s).ok());
            RoomChatMessage {
                id: r.get("id"),
                room_id: r.get("room_id"),
                sender_id: r.get("sender_id"),
                sender_name: r.get("sender_name"),
                sender_character: r.get("sender_character"),
                sender_avatar: r.get("sender_avatar"),
                content: r.get("content"),
                message_type: r.get("message_type"),
                roll_data,
                created_at: r.get("created_at"),
            }
        }).collect();

        Ok(msgs)
    }

    pub async fn clear_room_messages(pool: &SqlitePool, room_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM room_messages WHERE room_id = ?")
            .bind(room_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

