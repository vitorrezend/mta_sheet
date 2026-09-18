use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use std::fs;

const EMBEDDED_WEAPONS: &str = include_str!("../../../data/compendium/weapons.json");
const EMBEDDED_PRACTICES: &str = include_str!("../../../data/compendium/practices.json");
const EMBEDDED_INSTRUMENTS: &str = include_str!("../../../data/compendium/instruments.json");
const EMBEDDED_ARCHETYPES: &str = include_str!("../../../data/compendium/archetypes.json");
const EMBEDDED_ATTRIBUTES: &str = include_str!("../../../data/compendium/attributes.json");

/// Handler HTTP para servir dados do Compêndio M20 em JSON sob demanda
/// com suporte a Cache-Control imutável e validação por ETag (304 Not Modified).
pub async fn compendium_api_handler(
    Path(section): Path<String>,
    headers: HeaderMap,
) -> Response {
    let section = section.trim().to_lowercase();
    let (etag, embedded_data) = match section.as_str() {
        "weapons" => ("\"mta-comp-weapons-v1\"", EMBEDDED_WEAPONS),
        "practices" => ("\"mta-comp-practices-v1\"", EMBEDDED_PRACTICES),
        "instruments" => ("\"mta-comp-instruments-v1\"", EMBEDDED_INSTRUMENTS),
        "archetypes" => ("\"mta-comp-archetypes-v1\"", EMBEDDED_ARCHETYPES),
        "attributes" => ("\"mta-comp-attributes-v1\"", EMBEDDED_ATTRIBUTES),
        _ => return (StatusCode::NOT_FOUND, "Seção de compêndio não encontrada").into_response(),
    };

    // Suporte a ETag para resposta 304 rápida sem tráfego de rede
    if let Some(req_etag) = headers.get(header::IF_NONE_MATCH) {
        if let Ok(req_etag_str) = req_etag.to_str() {
            if req_etag_str == etag || req_etag_str == "*" {
                return (StatusCode::NOT_MODIFIED, "").into_response();
            }
        }
    }

    // Tenta ler do disco primeiro (permitindo edição viva de dados), com fallback para o embutido
    let disk_path = format!("data/compendium/{}.json", section);
    let content = if let Ok(disk_content) = fs::read_to_string(&disk_path) {
        disk_content
    } else {
        embedded_data.to_string()
    };

    (
        [
            (header::CONTENT_TYPE, "application/json; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
            (header::ETAG, etag),
        ],
        content,
    )
        .into_response()
}
