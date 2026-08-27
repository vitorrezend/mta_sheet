#[cfg(feature = "ssr")]
use sqlx::{sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous}, ConnectOptions, SqlitePool};
#[cfg(feature = "ssr")]
use std::str::FromStr;

#[cfg(feature = "ssr")]
pub async fn get_db() -> SqlitePool {
    let mut database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "mta_sheet.db".to_string());

    let db_path = if database_url.starts_with("sqlite:") {
        &database_url[7..]
    } else {
        &database_url
    };

    // Ensure parent directory exists
    if let Some(parent) = std::path::Path::new(db_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).ok();
        }
    }

    if !database_url.starts_with("sqlite:") {
        database_url = format!("sqlite:{}", database_url);
    }

    log::info!("Connecting to SQLite database at {}", database_url);

    let options = SqliteConnectOptions::from_str(&database_url)
        .expect("Invalid DATABASE_URL")
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(std::time::Duration::from_secs(5))
        .foreign_keys(true)
        .log_statements(log::LevelFilter::Debug);

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect_with(options)
        .await
        .expect("Failed to connect to SQLite");

    // Initialize tables
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            is_admin INTEGER NOT NULL DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create users table");

    // Migration suave para bancos já existentes
    let _ = sqlx::query("ALTER TABLE users ADD COLUMN is_admin INTEGER NOT NULL DEFAULT 0")
        .execute(&pool)
        .await;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            expires_at DATETIME NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create sessions table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS rooms (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            code TEXT UNIQUE NOT NULL,
            description TEXT DEFAULT '',
            gm_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create rooms table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS room_members (
            room_id TEXT NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
            user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            role TEXT NOT NULL DEFAULT 'player',
            joined_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (room_id, user_id)
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create room_members table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS character_sheets (
            id TEXT PRIMARY KEY,
            user_id TEXT REFERENCES users(id) ON DELETE SET NULL,
            room_id TEXT REFERENCES rooms(id) ON DELETE SET NULL,
            name TEXT NOT NULL,
            data TEXT NOT NULL,
            sheet_type TEXT NOT NULL DEFAULT 'mage',
            is_public INTEGER NOT NULL DEFAULT 0,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create character_sheets table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS media_assets (
            id TEXT PRIMARY KEY,
            sheet_id TEXT,
            block TEXT,
            file_path TEXT NOT NULL,
            mime_type TEXT NOT NULL,
            size_bytes INTEGER NOT NULL,
            data_blob BLOB NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create media_assets table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS quiz_questions (
            id TEXT PRIMARY KEY,
            splat TEXT NOT NULL DEFAULT 'mage',
            category TEXT NOT NULL DEFAULT 'character',
            title TEXT NOT NULL,
            prompt TEXT NOT NULL,
            sort_order INTEGER NOT NULL DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create quiz_questions table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS character_quiz_answers (
            character_id TEXT NOT NULL REFERENCES character_sheets(id) ON DELETE CASCADE,
            question_id TEXT NOT NULL REFERENCES quiz_questions(id) ON DELETE CASCADE,
            answer TEXT NOT NULL DEFAULT '',
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (character_id, question_id)
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create character_quiz_answers table");

    // Graceful migrations for existing databases
    let _ = sqlx::query("ALTER TABLE character_sheets ADD COLUMN user_id TEXT REFERENCES users(id) ON DELETE SET NULL").execute(&pool).await;
    let _ = sqlx::query("ALTER TABLE character_sheets ADD COLUMN room_id TEXT REFERENCES rooms(id) ON DELETE SET NULL").execute(&pool).await;
    let _ = sqlx::query("ALTER TABLE character_sheets ADD COLUMN sheet_type TEXT NOT NULL DEFAULT 'mage'").execute(&pool).await;
    let _ = sqlx::query("ALTER TABLE character_sheets ADD COLUMN is_public INTEGER NOT NULL DEFAULT 0").execute(&pool).await;
    let _ = sqlx::query("ALTER TABLE character_sheets ADD COLUMN is_hidden_in_room INTEGER NOT NULL DEFAULT 0").execute(&pool).await;
    let _ = sqlx::query("ALTER TABLE rooms ADD COLUMN chantry_data TEXT DEFAULT ''").execute(&pool).await;
    let _ = sqlx::query("ALTER TABLE rooms ADD COLUMN chronicle_notes TEXT DEFAULT ''").execute(&pool).await;

    // Índices de alta performance para evitar Full Table Scans no SQLite
    let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_sheets_user_id ON character_sheets (user_id)").execute(&pool).await;
    let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_sheets_public ON character_sheets (is_public)").execute(&pool).await;
    let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_sheets_room_id ON character_sheets (room_id)").execute(&pool).await;
    let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_sheets_updated_at ON character_sheets (updated_at DESC)").execute(&pool).await;
    let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions (user_id)").execute(&pool).await;
    let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON sessions (expires_at)").execute(&pool).await;
    let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_rooms_code ON rooms (code)").execute(&pool).await;
    let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_room_members_user_id ON room_members (user_id)").execute(&pool).await;
    let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_quiz_questions_splat ON quiz_questions (splat, sort_order)").execute(&pool).await;
    let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_character_quiz_answers_char ON character_quiz_answers (character_id)").execute(&pool).await;

    // Limpeza de sessões expiradas na inicialização
    let _ = sqlx::query("DELETE FROM sessions WHERE expires_at < CURRENT_TIMESTAMP").execute(&pool).await;

    // Automatic extraction of legacy base64 images from JSON to static uploads and media_assets
    migrate_and_extract_base64_images(&pool).await;

    // Automatic re-hydration of uploads from database backup if missing on disk
    rehydrate_media_assets_if_needed(&pool).await;

    // Seed das perguntas clássicas e migração de respostas existentes
    seed_quiz_questions(&pool).await;
    migrate_existing_quiz_answers(&pool).await;

    pool
}

#[cfg(feature = "ssr")]
async fn migrate_and_extract_base64_images(pool: &SqlitePool) {
    use sqlx::Row;
    use base64::Engine;

    let rows = match sqlx::query("SELECT id, data FROM character_sheets").fetch_all(pool).await {
        Ok(r) => r,
        Err(_) => return,
    };

    for row in rows {
        let sheet_id: String = row.get("id");
        let data_json: String = row.get("data");

        let mut char_data: crate::state::CharacterData = match serde_json::from_str(&data_json) {
            Ok(d) => d,
            Err(_) => {
                if let Some(d) = crate::state::CharacterData::from_raw_json_resilient(&sheet_id, &data_json) {
                    d
                } else {
                    continue;
                }
            }
        };

        let mut modified = false;

        for wonder in &mut char_data.wonders {
            if wonder.image_url.starts_with("data:image") {
                let img_data = &wonder.image_url;
                if let Some(idx) = img_data.find(";base64,") {
                    let mime_type = if img_data.starts_with("data:") {
                        &img_data[5..idx]
                    } else {
                        "image/webp"
                    };
                    let payload = &img_data[idx + 8..];

                    if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(payload.trim()) {
                        let ext = match mime_type {
                            "image/png" => "png",
                            "image/jpeg" | "image/jpg" => "jpg",
                            "image/gif" => "gif",
                            "image/svg+xml" => "svg",
                            _ => "webp",
                        };

                        let asset_id = format!("img_{}", uuid::Uuid::new_v4());
                        let safe_filename = format!("{}_{}.{}", wonder.id, asset_id, ext);
                        let dir_path = format!("uploads/sheets/{}/wonders", sheet_id);
                        let file_path = format!("{}/{}", dir_path, safe_filename);
                        let relative_url = format!("/uploads/sheets/{}/wonders/{}", sheet_id, safe_filename);

                        let _ = tokio::fs::create_dir_all(&dir_path).await;
                        let _ = tokio::fs::write(&file_path, &bytes).await;

                        let _ = sqlx::query(
                            "INSERT OR REPLACE INTO media_assets (id, sheet_id, block, file_path, mime_type, size_bytes, data_blob) VALUES (?, ?, ?, ?, ?, ?, ?)"
                        )
                        .bind(&asset_id)
                        .bind(&sheet_id)
                        .bind("wonders")
                        .bind(&file_path)
                        .bind(mime_type)
                        .bind(bytes.len() as i64)
                        .bind(&bytes)
                        .execute(pool)
                        .await;

                        wonder.image_url = relative_url;
                        modified = true;
                    }
                }
            }
        }

        // Migrate profile photo if it is stored as inline base64
        if let Some(photo_data) = char_data.labels.get("profile_photo").cloned() {
            if photo_data.starts_with("data:image") {
                if let Some(idx) = photo_data.find(";base64,") {
                    let mime_type = if photo_data.starts_with("data:") {
                        &photo_data[5..idx]
                    } else {
                        "image/webp"
                    };
                    let payload = &photo_data[idx + 8..];

                    if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(payload.trim()) {
                        let ext = match mime_type {
                            "image/png" => "png",
                            "image/jpeg" | "image/jpg" => "jpg",
                            "image/gif" => "gif",
                            "image/svg+xml" => "svg",
                            _ => "webp",
                        };

                        let asset_id = format!("img_{}", uuid::Uuid::new_v4());
                        let safe_filename = format!("portrait_{}.{}", asset_id, ext);
                        let dir_path = format!("uploads/sheets/{}/profile", sheet_id);
                        let file_path = format!("{}/{}", dir_path, safe_filename);
                        let relative_url = format!("/uploads/sheets/{}/profile/{}", sheet_id, safe_filename);

                        let _ = tokio::fs::create_dir_all(&dir_path).await;
                        let _ = tokio::fs::write(&file_path, &bytes).await;

                        let _ = sqlx::query(
                            "INSERT OR REPLACE INTO media_assets (id, sheet_id, block, file_path, mime_type, size_bytes, data_blob) VALUES (?, ?, ?, ?, ?, ?, ?)"
                        )
                        .bind(&asset_id)
                        .bind(&sheet_id)
                        .bind("profile")
                        .bind(&file_path)
                        .bind(mime_type)
                        .bind(bytes.len() as i64)
                        .bind(&bytes)
                        .execute(pool)
                        .await;

                        char_data.labels.insert("profile_photo".to_string(), relative_url);
                        modified = true;
                    }
                }
            }
        }

        if modified {
            if let Ok(new_json) = serde_json::to_string(&char_data) {
                let _ = sqlx::query("UPDATE character_sheets SET data = ? WHERE id = ?")
                    .bind(new_json)
                    .bind(&sheet_id)
                    .execute(pool)
                    .await;
                log::info!("Migrated inline base64 images to static uploads for sheet: {}", sheet_id);
            }
        }
    }
}

#[cfg(feature = "ssr")]
async fn rehydrate_media_assets_if_needed(pool: &SqlitePool) {
    use sqlx::Row;
    if let Ok(rows) = sqlx::query("SELECT file_path, data_blob FROM media_assets").fetch_all(pool).await {
        for row in rows {
            let file_path: String = row.get("file_path");
            let data_blob: Vec<u8> = row.get("data_blob");
            let path = std::path::Path::new(&file_path);
            if !path.exists() {
                if let Some(parent) = path.parent() {
                    let _ = tokio::fs::create_dir_all(parent).await;
                }
                let _ = tokio::fs::write(path, data_blob).await;
                log::info!("Re-hydrated media asset from database: {}", file_path);
            }
        }
    }
}

#[cfg(feature = "ssr")]
async fn seed_quiz_questions(pool: &SqlitePool) {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM quiz_questions")
        .fetch_one(pool)
        .await
        .unwrap_or(0);

    if count == 0 {
        let questions = crate::state::models::default_quiz_questions();
        for (i, q) in questions.iter().enumerate() {
            let _ = sqlx::query(
                "INSERT INTO quiz_questions (id, splat, category, title, prompt, sort_order) VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind(&q.id)
            .bind("mage")
            .bind(&q.category)
            .bind(&q.title)
            .bind(&q.prompt)
            .bind(i as i64)
            .execute(pool)
            .await;
        }
        log::info!("Seeded {} standard Mage quiz questions into quiz_questions table", questions.len());
    }
}

#[cfg(feature = "ssr")]
async fn migrate_existing_quiz_answers(pool: &SqlitePool) {
    use sqlx::Row;
    let rows = match sqlx::query("SELECT id, data FROM character_sheets").fetch_all(pool).await {
        Ok(r) => r,
        Err(_) => return,
    };

    for row in rows {
        let sheet_id: String = row.get("id");
        let data_json: String = row.get("data");

        if let Ok(char_data) = serde_json::from_str::<crate::state::CharacterData>(&data_json) {
            for entry in char_data.quiz_data.entries {
                let clean_ans = entry.answer.trim();
                if !clean_ans.is_empty() {
                    let _ = sqlx::query(
                        "INSERT OR IGNORE INTO character_quiz_answers (character_id, question_id, answer) VALUES (?, ?, ?)"
                    )
                    .bind(&sheet_id)
                    .bind(&entry.id)
                    .bind(clean_ans)
                    .execute(pool)
                    .await;
                }
            }
        }
    }
}
