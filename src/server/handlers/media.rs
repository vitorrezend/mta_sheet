use axum::response::IntoResponse;
use sqlx::Row;

pub async fn export_json_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let pool = crate::database::get_db().await;

    let row = match sqlx::query("SELECT name, data FROM character_sheets WHERE id = ?")
        .bind(&id)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(r)) => r,
        _ => return (http::StatusCode::NOT_FOUND, "Ficha não encontrada").into_response(),
    };

    let name: String = row.get("name");
    let data_str: String = row.get("data");

    let display_name = if let Some(parsed) = crate::state::CharacterData::parse_from_db(&id, &data_str) {
        parsed.get_display_name()
    } else if !name.trim().is_empty() {
        name.trim().to_string()
    } else {
        "ficha_mta".to_string()
    };

    let safe_name = if display_name.is_empty() {
        "ficha_mta".to_string()
    } else {
        display_name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "_")
    };

    let filename = format!("{}_mta_sheet.json", safe_name);
    let disposition = format!("attachment; filename=\"{}\"", filename);

    (
        [
            (http::header::CONTENT_TYPE, "application/json; charset=utf-8".to_string()),
            (http::header::CONTENT_DISPOSITION, disposition),
            (http::header::CACHE_CONTROL, "no-cache".to_string()),
        ],
        data_str,
    )
        .into_response()
}

pub async fn upload_image_handler(
    mut multipart: axum::extract::Multipart,
) -> impl IntoResponse {
    let mut file_bytes = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        if let Ok(bytes) = field.bytes().await {
            file_bytes = bytes.to_vec();
            break;
        }
    }

    if file_bytes.is_empty() {
        return (
            http::StatusCode::BAD_REQUEST,
            [(http::header::CONTENT_TYPE, "application/json")],
            serde_json::json!({"error": "Nenhum arquivo enviado"}).to_string(),
        )
            .into_response();
    }

    if file_bytes.len() > 5 * 1024 * 1024 {
        return (
            http::StatusCode::PAYLOAD_TOO_LARGE,
            [(http::header::CONTENT_TYPE, "application/json")],
            serde_json::json!({"error": "Imagem excede 5MB"}).to_string(),
        )
            .into_response();
    }

    let (_mime_type, ext) = match crate::state::validate_image_magic_bytes(&file_bytes) {
        Ok(res) => res,
        Err(e) => {
            return (
                http::StatusCode::BAD_REQUEST,
                [(http::header::CONTENT_TYPE, "application/json")],
                serde_json::json!({"error": e.to_string()}).to_string(),
            )
                .into_response();
        }
    };

    let file_id = uuid::Uuid::new_v4().to_string();
    let file_path = format!("uploads/{}.{}", file_id, ext);
    let public_url = format!("/uploads/{}.{}", file_id, ext);

    if let Err(e) = tokio::fs::write(&file_path, &file_bytes).await {
        return (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            [(http::header::CONTENT_TYPE, "application/json")],
            serde_json::json!({"error": format!("Falha ao salvar: {}", e)}).to_string(),
        )
            .into_response();
    }

    (
        http::StatusCode::OK,
        [(http::header::CONTENT_TYPE, "application/json")],
        serde_json::json!({ "url": public_url }).to_string(),
    )
        .into_response()
}
