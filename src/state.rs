use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use leptos::*;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AttributeValue {
    pub level: i32,
    pub modifier: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CharacterData {
    pub id: String,
    pub name: String,
    pub attributes: HashMap<String, AttributeValue>,
    pub labels: HashMap<String, String>,
    pub custom_lists: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CharacterSummary {
    pub id: String,
    pub name: String,
    pub updated_at: String,
}

#[server(GetSheets, "/api")]
pub async fn get_sheets() -> Result<Vec<CharacterSummary>, ServerFnError> {
    use sqlx::{SqlitePool, Row, sqlite::SqliteRow};
    let pool = use_context::<SqlitePool>().ok_or_else(|| ServerFnError::new("DB Pool not found"))?;

    let rows = sqlx::query("SELECT id, name, updated_at FROM character_sheets ORDER BY updated_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|e: sqlx::Error| ServerFnError::new(e.to_string()))?;

    let summaries = rows.into_iter().map(|row: SqliteRow| CharacterSummary {
        id: row.get("id"),
        name: row.get("name"),
        updated_at: row.get("updated_at"),
    }).collect();

    Ok(summaries)
}

#[server(GetSheet, "/api")]
pub async fn get_sheet(id: String) -> Result<CharacterData, ServerFnError> {
    use sqlx::{SqlitePool, Row, sqlite::SqliteRow};
    let pool = use_context::<SqlitePool>().ok_or_else(|| ServerFnError::new("DB Pool not found"))?;

    let row = sqlx::query("SELECT data FROM character_sheets WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e: sqlx::Error| ServerFnError::new(e.to_string()))?;

    let data_json: String = row.get("data");
    let data: CharacterData = serde_json::from_str(&data_json).map_err(|e: serde_json::Error| ServerFnError::new(e.to_string()))?;

    Ok(data)
}

#[server(CreateSheet, "/api")]
pub async fn create_sheet(name: String) -> Result<String, ServerFnError> {
    use sqlx::SqlitePool;
    use uuid::Uuid;
    let pool = use_context::<SqlitePool>().ok_or_else(|| ServerFnError::new("DB Pool not found"))?;

    let id = Uuid::new_v4().to_string();
    let initial_data = CharacterData {
        id: id.clone(),
        name: name.clone(),
        ..Default::default()
    };
    let data_json = serde_json::to_string(&initial_data).map_err(|e: serde_json::Error| ServerFnError::new(e.to_string()))?;

    sqlx::query("INSERT INTO character_sheets (id, name, data) VALUES (?, ?, ?)")
        .bind(&id)
        .bind(&name)
        .bind(data_json)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| ServerFnError::new(e.to_string()))?;

    Ok(id)
}

#[server(UpdateSheet, "/api")]
pub async fn update_sheet(id: String, data: CharacterData) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| ServerFnError::new("DB Pool not found"))?;

    let data_json = serde_json::to_string(&data).map_err(|e: serde_json::Error| ServerFnError::new(e.to_string()))?;

    sqlx::query("UPDATE character_sheets SET name = ?, data = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(&data.name)
        .bind(data_json)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e: sqlx::Error| ServerFnError::new(e.to_string()))?;

    Ok(())
}
