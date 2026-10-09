use leptos::*;
use super::super::models::{QuickStatAction, RoomSheetSummary, RoomBroadcastEvent, RoomInitiativeData, build_room_sheet_summary};
use super::super::events::broadcast_to_room;

#[server(endpoint = "update_room_sheet_order")]
pub async fn update_room_sheet_order(room_id: String, ordered_sheet_ids: Vec<String>) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    use crate::auth::get_auth_user_id;

    if room_id.trim().is_empty() {
        return Err(ServerFnError::new("ID da sala não fornecido"));
    }

    let current_user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Autenticação necessária para reordenar fichas")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    // Verifica se é narrador ou membro da sala
    let is_authorized = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM rooms r 
         LEFT JOIN room_members rm ON rm.room_id = r.id AND rm.user_id = ? 
         WHERE r.id = ? AND (r.gm_id = ? OR rm.user_id IS NOT NULL)"
    )
    .bind(&current_user_id)
    .bind(&room_id)
    .bind(&current_user_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))? > 0;

    if !is_authorized {
        return Err(ServerFnError::new("Você não tem permissão para reordenar as fichas nesta sala"));
    }

    let order_json = serde_json::to_string(&ordered_sheet_ids)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    sqlx::query("UPDATE rooms SET sheet_order = ? WHERE id = ?")
        .bind(&order_json)
        .bind(&room_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[server(endpoint = "assign_sheet_to_room")]
pub async fn assign_sheet_to_room(sheet_id: String, room_id: String) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    sqlx::query("UPDATE character_sheets SET room_id = ? WHERE id = ?")
        .bind(&room_id)
        .bind(&sheet_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao associar ficha à sala: {}", e)))?;

    Ok(())
}

#[server(endpoint = "remove_sheet_from_room")]
pub async fn remove_sheet_from_room(sheet_id: String) -> Result<(), ServerFnError> {
    use sqlx::{SqlitePool, Row};
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    // 1. Descobre a sala a que a ficha pertencia antes de desvincular
    let room_id: Option<String> = sqlx::query_scalar("SELECT room_id FROM character_sheets WHERE id = ?")
        .bind(&sheet_id)
        .fetch_optional(&pool)
        .await
        .unwrap_or_default();

    // 2. Desvincula a ficha
    sqlx::query("UPDATE character_sheets SET room_id = NULL WHERE id = ?")
        .bind(&sheet_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao desvincular ficha: {}", e)))?;

    // 3. Se estava vinculada a uma sala, limpa a iniciativa e a ordem de fichas dessa sala
    if let Some(r_id) = room_id {
        if let Ok(Some(row)) = sqlx::query("SELECT initiative_data, sheet_order FROM rooms WHERE id = ?")
            .bind(&r_id)
            .fetch_optional(&pool)
            .await
        {
            let init_raw: String = row.try_get("initiative_data").unwrap_or_default();
            if !init_raw.is_empty() {
                if let Ok(mut init) = serde_json::from_str::<RoomInitiativeData>(&init_raw) {
                    let old_len = init.entries.len();
                    init.entries.retain(|e| e.id != sheet_id);
                    if init.entries.len() != old_len {
                        if let Ok(new_json) = serde_json::to_string(&init) {
                            let _ = sqlx::query("UPDATE rooms SET initiative_data = ? WHERE id = ?")
                                .bind(&new_json)
                                .bind(&r_id)
                                .execute(&pool)
                                .await;

                            broadcast_to_room(&r_id, RoomBroadcastEvent {
                                event_type: "INITIATIVE_UPDATE".to_string(),
                                initiative: init,
                                play_sound: false,
                            });
                        }
                    }
                }
            }

            let order_raw: String = row.try_get("sheet_order").unwrap_or_default();
            if !order_raw.is_empty() {
                if let Ok(mut order) = serde_json::from_str::<Vec<String>>(&order_raw) {
                    let old_len = order.len();
                    order.retain(|id| id != &sheet_id);
                    if order.len() != old_len {
                        if let Ok(new_order) = serde_json::to_string(&order) {
                            let _ = sqlx::query("UPDATE rooms SET sheet_order = ? WHERE id = ?")
                                .bind(&new_order)
                                .bind(&r_id)
                                .execute(&pool)
                                .await;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

#[server(endpoint = "toggle_sheet_room_visibility")]
pub async fn toggle_sheet_room_visibility(sheet_id: String, is_hidden: bool) -> Result<(), ServerFnError> {
    use sqlx::{SqlitePool, Row};
    use crate::auth::get_auth_user_id;

    let user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para alterar a visibilidade")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    // Check if user is GM of the room or owner of the sheet
    let row = sqlx::query(
        "SELECT cs.user_id, r.gm_id FROM character_sheets cs 
         LEFT JOIN rooms r ON cs.room_id = r.id 
         WHERE cs.id = ?"
    )
    .bind(&sheet_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?
    .ok_or_else(|| ServerFnError::new("Ficha não encontrada"))?;

    let sheet_owner: Option<String> = row.get("user_id");
    let room_gm: Option<String> = row.get("gm_id");

    if sheet_owner.as_deref() != Some(&user_id) && room_gm.as_deref() != Some(&user_id) {
        return Err(ServerFnError::new("Você não tem permissão para alterar a visibilidade desta ficha"));
    }

    let val: i64 = if is_hidden { 1 } else { 0 };
    sqlx::query("UPDATE character_sheets SET is_hidden_in_room = ? WHERE id = ?")
        .bind(val)
        .bind(&sheet_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao alterar visibilidade: {}", e)))?;

    Ok(())
}

#[server(endpoint = "clone_and_assign_sheet_to_member")]
pub async fn clone_and_assign_sheet_to_member(
    room_id: String,
    sheet_id: String,
    target_user_id: String,
) -> Result<String, ServerFnError> {
    use sqlx::{SqlitePool, Row};
    use crate::auth::get_auth_user_id;
    use crate::state::CharacterData;

    let caller_user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para realizar esta ação")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    // 1. Valida se o chamador é o Narrador da sala
    let is_gm_row = sqlx::query("SELECT gm_id FROM rooms WHERE id = ?")
        .bind(&room_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao verificar sala: {}", e)))?;

    let gm_row = is_gm_row.ok_or_else(|| ServerFnError::new("Sala não encontrada"))?;
    let room_gm_id: String = gm_row.get("gm_id");

    if room_gm_id != caller_user_id {
        return Err(ServerFnError::new("Apenas o Narrador da sala pode clonar e entregar fichas para membros"));
    }

    // 2. Valida se o target_user_id é membro desta sala
    let is_member = sqlx::query("SELECT 1 FROM room_members WHERE room_id = ? AND user_id = ?")
        .bind(&room_id)
        .bind(&target_user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao verificar membro: {}", e)))?;

    if is_member.is_none() && target_user_id != room_gm_id {
        return Err(ServerFnError::new("O jogador selecionado não é membro desta sala"));
    }

    // 3. Validação de cota de 50 fichas para o jogador de destino
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM character_sheets WHERE user_id = ?")
        .bind(&target_user_id)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    if count >= 50 {
        return Err(ServerFnError::new("O jogador selecionado atingiu o limite de 50 fichas. Ele precisa excluir fichas antigas para receber uma nova."));
    }

    // 4. Busca a ficha original do Narrador
    let sheet_row = sqlx::query("SELECT name, data, sheet_type FROM character_sheets WHERE id = ? AND user_id = ?")
        .bind(&sheet_id)
        .bind(&caller_user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao buscar ficha original: {}", e)))?;

    let sheet_data = sheet_row.ok_or_else(|| ServerFnError::new("Ficha original não encontrada no seu inventário"))?;
    let sheet_name: String = sheet_data.get("name");
    let sheet_json: String = sheet_data.get("data");
    let sheet_type: String = sheet_data.get("sheet_type");

    // 5. Cria nova ficha com novo UUID pertencente ao jogador e já associada à sala
    let new_sheet_id = uuid::Uuid::new_v4().to_string();

    let mut updated_data: CharacterData = serde_json::from_str(&sheet_json)
        .unwrap_or_else(|_| CharacterData::new(new_sheet_id.clone(), sheet_name.clone()));
    updated_data.id = new_sheet_id.clone();
    let updated_json = serde_json::to_string(&updated_data).unwrap_or(sheet_json);

    let summary = updated_data.to_summary(String::new(), false, true);
    let summary_json = serde_json::to_string(&summary).unwrap_or_default();

    sqlx::query(
        "INSERT INTO character_sheets (id, user_id, room_id, name, data, sheet_type, is_public, is_hidden_in_room, summary_json, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, 0, 0, ?, datetime('now'))"
    )
    .bind(&new_sheet_id)
    .bind(&target_user_id)
    .bind(&room_id)
    .bind(&sheet_name)
    .bind(&updated_json)
    .bind(&sheet_type)
    .bind(&summary_json)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Erro ao criar ficha clonada: {}", e)))?;

    // 6. Clona respostas do Dossiê/Questionário, se houverem
    let quiz_answers = sqlx::query("SELECT question_id, answer FROM character_quiz_answers WHERE character_id = ?")
        .bind(&sheet_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    for qa in quiz_answers {
        let q_id: String = qa.get("question_id");
        let ans: String = qa.get("answer");
        let _ = sqlx::query(
            "INSERT OR REPLACE INTO character_quiz_answers (character_id, question_id, answer, updated_at) \
             VALUES (?, ?, ?, datetime('now'))"
        )
        .bind(&new_sheet_id)
        .bind(&q_id)
        .bind(&ans)
        .execute(&pool)
        .await;
    }

    crate::logging::server::write_log(
        crate::logging::LogCategory::UserActions,
        "INFO",
        &format!("CLONE & ASSIGN SHEET: Ficha '{}' ({}) clonada com sucesso para o jogador '{}' na sala '{}'", sheet_name, new_sheet_id, target_user_id, room_id),
        None,
    );

    Ok(new_sheet_id)
}

#[server(endpoint = "update_room_sheet_stats")]
pub async fn update_room_sheet_stats(
    room_id: String,
    sheet_id: String,
    action: QuickStatAction,
) -> Result<RoomSheetSummary, ServerFnError> {
    use sqlx::{SqlitePool, Row};
    use crate::auth::get_auth_user_id;
    use crate::state::CharacterData;

    let caller_user_id = get_auth_user_id().await?.ok_or_else(|| {
        ServerFnError::new("Você precisa estar logado para realizar esta ação")
    })?;

    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Conexão com o banco de dados indisponível")
    })?;

    // 1. Valida se o chamador é o Narrador da sala
    let is_gm_row = sqlx::query("SELECT gm_id FROM rooms WHERE id = ?")
        .bind(&room_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao verificar sala: {}", e)))?;

    let gm_row = is_gm_row.ok_or_else(|| ServerFnError::new("Sala não encontrada"))?;
    let room_gm_id: String = gm_row.get("gm_id");

    if room_gm_id != caller_user_id {
        return Err(ServerFnError::new("Apenas o Narrador da sala pode alterar os atributos táticos da ficha"));
    }

    // 2. Busca a ficha associada à sala
    let sheet_row = sqlx::query(
        "SELECT id, name, data, user_id, is_hidden_in_room, updated_at \
         FROM character_sheets WHERE id = ? AND room_id = ?"
    )
    .bind(&sheet_id)
    .bind(&room_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Erro ao buscar ficha na sala: {}", e)))?;

    let sheet_record = sheet_row.ok_or_else(|| ServerFnError::new("Ficha não encontrada nesta sala"))?;
    let sheet_name: String = sheet_record.get("name");
    let sheet_json: String = sheet_record.get("data");
    let sheet_user_id: Option<String> = sheet_record.get("user_id");
    let is_hidden_val: i64 = sheet_record.get("is_hidden_in_room");
    let is_hidden = is_hidden_val == 1;
    let is_owner = sheet_user_id.as_deref() == Some(&caller_user_id);

    let mut char_data = CharacterData::parse_from_db(&sheet_id, &sheet_json)
        .unwrap_or_else(|| CharacterData::new(sheet_id.clone(), sheet_name.clone()));

    // 3. Aplica a ação tática
    match action {
        QuickStatAction::CycleHealthBox { index } => {
            char_data.click_health_box(index);
        }
        QuickStatAction::HealHealthBox { index } => {
            char_data.heal_health_box(index);
        }
        QuickStatAction::ClearHealth => {
            char_data.clear_health();
        }
        QuickStatAction::AddQuintessence => {
            char_data.add_quintessence();
        }
        QuickStatAction::RemoveQuintessence => {
            char_data.remove_quintessence();
        }
        QuickStatAction::AddParadox => {
            char_data.add_paradox();
        }
        QuickStatAction::RemoveParadox => {
            char_data.remove_paradox();
        }
        QuickStatAction::CycleQuintessenceParadoxBox { index } => {
            char_data.cycle_quintessence_paradox_box(index);
        }
        QuickStatAction::ClearQuintessenceParadoxBox { index } => {
            char_data.set_quintessence_paradox_box(index, '0');
        }
        QuickStatAction::SetWillpowerTotal { value } => {
            char_data.set_willpower_total(value);
        }
        QuickStatAction::SetWillpowerCurrent { value } => {
            char_data.set_willpower_current(value);
        }
        QuickStatAction::AdjustWillpowerCurrent { delta } => {
            let (tot, cur) = char_data.get_willpower();
            let new_cur = (cur + delta).clamp(0, tot);
            char_data.set_willpower_current(new_cur);
        }
        QuickStatAction::AdjustWillpowerTotal { delta } => {
            let (tot, _) = char_data.get_willpower();
            let new_tot = (tot + delta).clamp(1, 10);
            char_data.set_willpower_total(new_tot);
        }
    }

    char_data.sanitize();
    let updated_json = serde_json::to_string(&char_data)
        .map_err(|e| ServerFnError::new(format!("Erro ao serializar ficha: {}", e)))?;

    let summary = char_data.to_summary(String::new(), char_data.is_public, true);
    let summary_json = serde_json::to_string(&summary).unwrap_or_default();

    // 4. Salva no banco de dados SQLite
    sqlx::query("UPDATE character_sheets SET data = ?, summary_json = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(&updated_json)
        .bind(&summary_json)
        .bind(&sheet_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Erro ao salvar alterações na ficha: {}", e)))?;

    let updated_at: String = sqlx::query_scalar("SELECT updated_at FROM character_sheets WHERE id = ?")
        .bind(&sheet_id)
        .fetch_one(&pool)
        .await
        .unwrap_or_default();

    let summary = build_room_sheet_summary(
        sheet_id,
        sheet_name,
        &updated_json,
        updated_at,
        is_hidden,
        is_owner,
    );

    crate::logging::server::write_log(
        crate::logging::LogCategory::UserActions,
        "INFO",
        &format!("GM QUICK STAT UPDATE: Ficha '{}' ({}) atualizada pelo Narrador na sala '{}'", summary.name, summary.id, room_id),
        None,
    );

    Ok(summary)
}
