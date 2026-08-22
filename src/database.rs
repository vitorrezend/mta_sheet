#[cfg(feature = "ssr")]
use sqlx::{sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous}, ConnectOptions, SqlitePool};
#[cfg(feature = "ssr")]
use std::str::FromStr;

#[cfg(feature = "ssr")]
pub async fn get_db() -> SqlitePool {
    use dotenvy::dotenv;
    let _ = dotenv();

    let mut database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "mta_sheet.db".to_string());

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
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create users table");

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

    // Graceful migrations for existing databases
    let _ = sqlx::query("ALTER TABLE character_sheets ADD COLUMN user_id TEXT REFERENCES users(id) ON DELETE SET NULL").execute(&pool).await;
    let _ = sqlx::query("ALTER TABLE character_sheets ADD COLUMN room_id TEXT REFERENCES rooms(id) ON DELETE SET NULL").execute(&pool).await;
    let _ = sqlx::query("ALTER TABLE character_sheets ADD COLUMN sheet_type TEXT NOT NULL DEFAULT 'mage'").execute(&pool).await;
    let _ = sqlx::query("ALTER TABLE character_sheets ADD COLUMN is_public INTEGER NOT NULL DEFAULT 0").execute(&pool).await;

    // Automatic extraction of legacy base64 images from JSON to static uploads and media_assets
    migrate_and_extract_base64_images(&pool).await;

    // Automatic re-hydration of uploads from database backup if missing on disk
    rehydrate_media_assets_if_needed(&pool).await;

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
