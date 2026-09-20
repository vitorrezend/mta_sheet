use leptos::*;
use crate::state::models::{SheetAclEntry, SheetShareSettings};

#[cfg(feature = "ssr")]
use crate::repositories::SheetAclRepository;

// ============================================================================
// Sheet Sharing & Access Control Server Functions (Google Drive Style)
// ============================================================================

#[server(endpoint = "get_sheet_share_settings")]
pub async fn get_sheet_share_settings(sheet_id: String) -> Result<SheetShareSettings, ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado: Autenticação necessária."))?;

    let is_allowed = SheetAclRepository::check_sheet_permission(&pool, &sheet_id, &auth_user_id, "admin").await
        .map_err(|_| ServerFnError::new("Erro ao verificar permissão da ficha."))?;

    if !is_allowed {
        return Err(ServerFnError::new("Apenas o proprietário ou administrador pode visualizar as configurações de compartilhamento desta ficha."));
    }

    let settings = SheetAclRepository::get_share_settings(&pool, &sheet_id).await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to fetch sheet share settings", Some(&e.to_string()));
            ServerFnError::new("Falha ao consultar configurações de compartilhamento.")
        })?
        .ok_or_else(|| ServerFnError::new("Ficha não encontrada."))?;

    Ok(settings)
}

#[server(endpoint = "update_sheet_share_link")]
pub async fn update_sheet_share_link(
    sheet_id: String,
    share_permission: String,
    regenerate_token: bool,
) -> Result<SheetShareSettings, ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado: Autenticação necessária."))?;

    let is_allowed = SheetAclRepository::check_sheet_permission(&pool, &sheet_id, &auth_user_id, "admin").await
        .map_err(|_| ServerFnError::new("Erro ao verificar permissão da ficha."))?;

    if !is_allowed {
        return Err(ServerFnError::new("Apenas o proprietário ou administrador pode alterar o link de compartilhamento."));
    }

    let clean_perm = match share_permission.to_lowercase().as_str() {
        "view" | "read" => "view".to_string(),
        _ => "none".to_string(),
    };

    let updated = SheetAclRepository::update_share_link(&pool, &sheet_id, &clean_perm, regenerate_token).await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to update sheet share link", Some(&e.to_string()));
            ServerFnError::new("Falha ao atualizar link de compartilhamento.")
        })?
        .ok_or_else(|| ServerFnError::new("Ficha não encontrada."))?;

    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("Link de compartilhamento da ficha '{}' atualizado para '{}' (regen={})", sheet_id, clean_perm, regenerate_token),
        None,
    );

    Ok(updated)
}

#[server(endpoint = "grant_sheet_acl")]
pub async fn grant_sheet_acl(
    sheet_id: String,
    grantee_type: String,
    grantee_identifier: String,
    permission: String,
) -> Result<SheetAclEntry, ServerFnError> {
    use sqlx::SqlitePool;
    use uuid::Uuid;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado: Autenticação necessária."))?;

    let is_allowed = SheetAclRepository::check_sheet_permission(&pool, &sheet_id, &auth_user_id, "admin").await
        .map_err(|_| ServerFnError::new("Erro ao verificar permissão."))?;

    if !is_allowed {
        return Err(ServerFnError::new("Apenas o proprietário ou administrador pode conceder permissões nesta ficha."));
    }

    let clean_perm = match permission.to_lowercase().as_str() {
        "admin" => "admin".to_string(),
        "write" | "editor" => "write".to_string(),
        _ => "read".to_string(),
    };

    let (grantee_id, grantee_name) = match grantee_type.to_lowercase().as_str() {
        "user" => {
            let clean_user = grantee_identifier.trim();
            if clean_user.is_empty() {
                return Err(ServerFnError::new("Digite o nome de usuário para compartilhar."));
            }
            let u_opt = SheetAclRepository::find_user_by_name(&pool, clean_user).await
                .map_err(|_| ServerFnError::new("Erro ao buscar usuário."))?;

            match u_opt {
                Some((uid, uname)) => {
                    if uid == auth_user_id {
                        return Err(ServerFnError::new("Você já é o proprietário desta ficha."));
                    }
                    (Some(uid), uname)
                }
                None => return Err(ServerFnError::new(format!("Usuário '{}' não encontrado.", clean_user))),
            }
        }
        "room" => {
            let clean_room = grantee_identifier.trim();
            if clean_room.is_empty() {
                return Err(ServerFnError::new("Digite o código ou selecione a sala de crônica."));
            }
            let r_opt = SheetAclRepository::find_room_by_id_or_code(&pool, clean_room).await
                .map_err(|_| ServerFnError::new("Erro ao buscar sala."))?;

            match r_opt {
                Some((rid, rname)) => (Some(rid), rname),
                None => return Err(ServerFnError::new("Sala de crônica não encontrada.")),
            }
        }
        "public" => (None, "Público Geral".to_string()),
        _ => return Err(ServerFnError::new("Tipo de destinatário inválido.")),
    };

    let acl_id = Uuid::new_v4().to_string();
    SheetAclRepository::grant(&pool, &acl_id, &sheet_id, &grantee_type, grantee_id.as_deref(), &clean_perm)
        .await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to grant sheet ACL", Some(&e.to_string()));
            ServerFnError::new("Falha ao salvar permissão da ficha.")
        })?;

    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("Permissão concedida na ficha '{}': tipo='{}', target='{}', perm='{}'", sheet_id, grantee_type, grantee_name, clean_perm),
        None,
    );

    Ok(SheetAclEntry {
        id: acl_id,
        sheet_id,
        grantee_type,
        grantee_id,
        grantee_name,
        permission: clean_perm,
        created_at: String::new(),
    })
}

#[server(endpoint = "revoke_sheet_acl")]
pub async fn revoke_sheet_acl(acl_id: String) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado: Autenticação necessária."))?;

    let sheet_id = SheetAclRepository::find_sheet_id_by_acl(&pool, &acl_id).await
        .map_err(|_| ServerFnError::new("Erro ao buscar permissão."))?;

    let sid = match sheet_id {
        Some(s) => s,
        None => return Err(ServerFnError::new("Regra de permissão não encontrada.")),
    };

    let is_allowed = SheetAclRepository::check_sheet_permission(&pool, &sid, &auth_user_id, "admin").await
        .map_err(|_| ServerFnError::new("Erro ao verificar permissão."))?;

    if !is_allowed {
        return Err(ServerFnError::new("Acesso negado: apenas o proprietário ou administrador pode revogar permissões."));
    }

    SheetAclRepository::revoke(&pool, &acl_id).await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to revoke sheet ACL", Some(&e.to_string()));
            ServerFnError::new("Falha ao revogar permissão da ficha.")
        })?;

    crate::logging::server::write_log(
        crate::logging::LogCategory::Database,
        "INFO",
        &format!("Permissão '{}' revogada na ficha '{}'", acl_id, sid),
        None,
    );

    Ok(())
}
