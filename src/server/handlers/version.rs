use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;

/// Informações da versão e metadados de build do servidor
#[derive(Serialize, Clone, Debug)]
pub struct AppVersionInfo {
    pub version: &'static str,
    pub semver: &'static str,
}

/// Handler HTTP GET /api/version
///
/// Retorna a versão canônica do backend para detecção de atualizações
/// a quente e sincronização transparente com os clientes SPA/WASM.
pub async fn version_handler() -> impl IntoResponse {
    Json(AppVersionInfo {
        version: env!("CARGO_PKG_VERSION"),
        semver: concat!("v", env!("CARGO_PKG_VERSION")),
    })
}
