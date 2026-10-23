//! Mapa tático: leitura e edição por operações.

use leptos::*;
use leptos::server_fn::codec::Json;
use serde::{Deserialize, Serialize};
use super::super::map::{MapOp, RoomMapData};

/// Estado do mapa entregue ao cliente.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RoomMapSnapshot {
    pub map: RoomMapData,
    pub version: i64,
    pub is_gm: bool,
    /// Fichas do usuário (os tokens delas ele pode mover)
    pub my_sheet_ids: Vec<String>,
}

#[cfg(feature = "ssr")]
mod helpers {
    use leptos::*;
    use sqlx::{Row, SqlitePool};
    use crate::errors::{AppError, AppResult};
    use crate::rooms::map::RoomMapData;

    pub struct MapAccess {
        pub pool: SqlitePool,
        pub user_id: String,
        pub is_gm: bool,
    }

    /// Usuário logado, membro ou mestre da sala, com o recurso de mapa liberado.
    pub async fn require_map_access(room_id: &str) -> AppResult<MapAccess> {
        let user_id = crate::auth::get_auth_user_id()
            .await
            .map_err(|e| AppError::internal("Falha ao ler a sessão", e))?
            .ok_or(AppError::NotLoggedIn)?;
        let pool = use_context::<SqlitePool>().ok_or_else(|| AppError::internal("Pool do banco ausente no contexto", "mapa"))?;

        let allowed = crate::settings::is_feature_enabled_for_caller(&pool, "feature_tactical_grid")
            .await
            .map_err(|e| AppError::internal("Falha ao ler recursos do sistema", e))?;
        if !allowed {
            return Err(AppError::forbidden("O mapa tático está desativado nas configurações do sistema."));
        }
        if !crate::repositories::RoomRepository::check_is_gm_or_member(&pool, room_id, &user_id).await? {
            return Err(AppError::forbidden("Você não participa desta sala"));
        }
        let is_gm = crate::repositories::RoomRepository::check_is_gm(&pool, room_id, &user_id).await?;
        Ok(MapAccess { pool, user_id, is_gm })
    }

    pub async fn load_map(pool: &SqlitePool, room_id: &str) -> AppResult<(RoomMapData, i64)> {
        let row = sqlx::query("SELECT map_data, map_version FROM rooms WHERE id = ?")
            .bind(room_id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::not_found("Sala não encontrada"))?;
        let raw: String = row.try_get("map_data").unwrap_or_default();
        let version: i64 = row.try_get("map_version").unwrap_or(0);
        let mut map: RoomMapData = serde_json::from_str(&raw).unwrap_or_default();
        map.normalize();
        Ok((map, version))
    }

    pub async fn my_sheet_ids(pool: &SqlitePool, room_id: &str, user_id: &str) -> AppResult<Vec<String>> {
        Ok(sqlx::query_scalar("SELECT id FROM character_sheets WHERE room_id = ? AND user_id = ?")
            .bind(room_id)
            .bind(user_id)
            .fetch_all(pool)
            .await?)
    }
}

#[server(endpoint = "get_room_map")]
pub async fn get_room_map(room_id: String) -> Result<RoomMapSnapshot, ServerFnError> {
    let access = helpers::require_map_access(&room_id).await?;
    let (map, version) = helpers::load_map(&access.pool, &room_id).await?;
    let my_sheet_ids = helpers::my_sheet_ids(&access.pool, &room_id, &access.user_id).await?;
    let map = if access.is_gm { map } else { map.visible_to_players() };
    Ok(RoomMapSnapshot { map, version, is_gm: access.is_gm, my_sheet_ids })
}

/// Aplica um lote de edições e devolve a nova versão do mapa.
///
/// Mestre: qualquer operação. Jogador: só mover o token de uma ficha sua (e abrir/fechar
/// portas), nunca para uma célula bloqueada (rocha, abismo).
// JSON: a lista de operações tem enums aninhados que a codificação de formulário
// (padrão das server functions) não representa
#[server(endpoint = "apply_map_ops", input = Json)]
pub async fn apply_map_ops(room_id: String, ops: Vec<MapOp>) -> Result<i64, ServerFnError> {
    use crate::errors::AppError;
    use crate::rooms::map::MAX_OPS_PER_BATCH;
    use crate::rooms::RoomBroadcastEvent;

    if ops.is_empty() {
        return Err(AppError::invalid("Nenhuma alteração enviada").into());
    }
    if ops.len() > MAX_OPS_PER_BATCH {
        return Err(AppError::invalid("Alterações demais de uma vez").into());
    }
    let access = helpers::require_map_access(&room_id).await?;
    if !crate::server::middleware::security::check_action_rate_limit(&format!("map:{}", access.user_id), 120, std::time::Duration::from_secs(10)) {
        return Err(AppError::RateLimited("Edições demais em sequência. Aguarde um instante.".into()).into());
    }
    let my_sheets = if access.is_gm { Vec::new() } else { helpers::my_sheet_ids(&access.pool, &room_id, &access.user_id).await? };

    // Concorrência otimista: relê e reaplica se outra pessoa gravou no meio do caminho
    for _ in 0..5 {
        let (mut map, version) = helpers::load_map(&access.pool, &room_id).await?;
        let hides_details = ops.iter().any(|op| op.touches_hidden_token(&map));

        for op in &ops {
            map.authorize(op, access.is_gm, &my_sheets).map_err(AppError::Forbidden)?;
            map.apply(op).map_err(AppError::Invalid)?;
        }

        let json = serde_json::to_string(&map).map_err(|e| AppError::internal("Falha ao serializar o mapa", e))?;
        let new_version: Option<i64> = sqlx::query_scalar(
            "UPDATE rooms SET map_data = ?, map_version = map_version + 1 WHERE id = ? AND map_version = ? RETURNING map_version",
        )
        .bind(&json)
        .bind(&room_id)
        .bind(version)
        .fetch_optional(&access.pool)
        .await
        .map_err(AppError::from)?;

        if let Some(version) = new_version {
            let ops = if hides_details { None } else { Some(ops) };
            crate::rooms::events::broadcast_to_room(&room_id, RoomBroadcastEvent::MapChanged { version, ops });
            return Ok(version);
        }
    }
    Err(AppError::internal("Mapa muito disputado", "5 tentativas de gravação concorrente").into())
}
