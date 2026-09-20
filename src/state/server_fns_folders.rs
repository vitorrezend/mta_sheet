use leptos::*;
use crate::state::models::{SheetFolder, FolderAclEntry};

#[cfg(feature = "ssr")]
use crate::repositories::{FolderRepository, AclRepository, SheetRepository};

// ==========================================
// Folder Management Server Functions
// ==========================================

#[server(endpoint = "get_folders")]
pub async fn get_folders() -> Result<Vec<SheetFolder>, ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Database pool not found in get_folders", None);
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None);
    if auth_user_id.is_none() {
        return Ok(Vec::new());
    }
    let user_id = auth_user_id.unwrap_or_default();

    let folders = FolderRepository::list_by_user(&pool, &user_id).await.map_err(|e| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to fetch folders", Some(&e.to_string()));
        ServerFnError::new("Falha ao consultar pastas.")
    })?;

    Ok(folders)
}

#[server(endpoint = "create_folder")]
pub async fn create_folder(
    name: String,
    icon: Option<String>,
    color: Option<String>,
    parent_id: Option<String>,
) -> Result<SheetFolder, ServerFnError> {
    use sqlx::SqlitePool;
    use uuid::Uuid;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Você precisa estar autenticado para criar pastas."))?;

    let clean_name = name.trim().to_string();
    if clean_name.is_empty() {
        return Err(ServerFnError::new("O nome da pasta não pode ser vazio."));
    }
    if clean_name.len() > 60 {
        return Err(ServerFnError::new("O nome da pasta não pode ter mais de 60 caracteres."));
    }

    if let Some(ref pid) = parent_id {
        let parent_owner = FolderRepository::find_owner_id(&pool, pid).await
            .map_err(|_| ServerFnError::new("Erro ao validar pasta pai."))?;
        match parent_owner {
            Some(owner_id) if owner_id == auth_user_id => {},
            _ => return Err(ServerFnError::new("Pasta pai não encontrada ou acesso negado.")),
        }
    }

    let count = FolderRepository::count_by_user(&pool, &auth_user_id).await.unwrap_or(0);
    if count >= 60 {
        return Err(ServerFnError::new("Limite de 60 pastas por conta atingido."));
    }

    let id = Uuid::new_v4().to_string();
    let folder_icon = icon.unwrap_or_else(|| "📁".to_string());
    let folder_color = color.unwrap_or_else(|| "#b89347".to_string());

    FolderRepository::create(
        &pool,
        &id,
        &auth_user_id,
        parent_id.as_deref(),
        &clean_name,
        &folder_icon,
        &folder_color,
        count as i32,
    )
    .await
    .map_err(|e| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to create folder", Some(&e.to_string()));
        ServerFnError::new("Falha ao criar pasta no banco de dados.")
    })?;

    Ok(SheetFolder {
        id,
        user_id: auth_user_id,
        parent_id,
        name: clean_name,
        icon: folder_icon,
        color: folder_color,
        sort_order: count as i32,
        sheet_count: 0,
        created_at: String::new(),
    })
}

#[server(endpoint = "move_folder")]
pub async fn move_folder(folder_id: String, target_parent_id: Option<String>) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado."))?;

    let folder_owner = FolderRepository::find_owner_id(&pool, &folder_id).await
        .map_err(|_| ServerFnError::new("Erro ao buscar pasta."))?;

    match folder_owner {
        Some(owner_id) if owner_id == auth_user_id => {},
        _ => return Err(ServerFnError::new("Pasta não encontrada ou acesso não autorizado.")),
    }

    if let Some(ref target_id) = target_parent_id {
        if target_id == &folder_id {
            return Err(ServerFnError::new("Não é possível mover uma pasta para dentro de si mesma."));
        }

        let target_owner = FolderRepository::find_owner_id(&pool, target_id).await
            .map_err(|_| ServerFnError::new("Erro ao buscar pasta destino."))?;

        match target_owner {
            Some(owner_id) if owner_id == auth_user_id => {},
            _ => return Err(ServerFnError::new("Pasta destino não encontrada ou acesso negado.")),
        }

        let is_descendant = FolderRepository::is_descendant(&pool, &folder_id, target_id).await
            .map_err(|_| ServerFnError::new("Erro ao validar hierarquia de pastas."))?;

        if is_descendant {
            return Err(ServerFnError::new("Operação inválida: não é possível mover uma pasta para dentro de uma de suas subpastas."));
        }
    }

    FolderRepository::move_folder(&pool, &folder_id, target_parent_id.as_deref(), &auth_user_id).await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to move folder", Some(&e.to_string()));
            ServerFnError::new("Falha ao mover pasta.")
        })?;

    Ok(())
}

#[server(endpoint = "update_folder")]
pub async fn update_folder(folder_id: String, name: String, icon: Option<String>, color: Option<String>) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado."))?;

    let clean_name = name.trim().to_string();
    if clean_name.is_empty() {
        return Err(ServerFnError::new("O nome da pasta não pode ser vazio."));
    }
    if clean_name.len() > 60 {
        return Err(ServerFnError::new("O nome da pasta não pode ter mais de 60 caracteres."));
    }

    let folder_icon = icon.unwrap_or_else(|| "📁".to_string());
    let folder_color = color.unwrap_or_else(|| "#b89347".to_string());

    let rows_affected = FolderRepository::update(
        &pool,
        &folder_id,
        &auth_user_id,
        &clean_name,
        &folder_icon,
        &folder_color,
    )
    .await
    .map_err(|e| {
        crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to update folder", Some(&e.to_string()));
        ServerFnError::new("Falha ao atualizar pasta.")
    })?;

    if rows_affected == 0 {
        return Err(ServerFnError::new("Pasta não encontrada ou acesso não autorizado."));
    }

    Ok(())
}

#[server(endpoint = "delete_folder")]
pub async fn delete_folder(folder_id: String) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado."))?;

    let rows_affected = FolderRepository::delete(&pool, &folder_id, &auth_user_id).await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to delete folder", Some(&e.to_string()));
            ServerFnError::new("Falha ao excluir pasta.")
        })?;

    if rows_affected == 0 {
        return Err(ServerFnError::new("Pasta não encontrada ou acesso não autorizado."));
    }

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn check_folder_permission_internal(
    pool: &sqlx::SqlitePool,
    folder_id: &str,
    user_id: &str,
    required_permission: &str,
) -> Result<bool, sqlx::Error> {
    AclRepository::check_folder_permission(pool, folder_id, user_id, required_permission).await
}

#[server(endpoint = "move_sheet_to_folder")]
pub async fn move_sheet_to_folder(sheet_id: String, folder_id: Option<String>) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado."))?;

    if let Some(ref fid) = folder_id {
        let is_allowed = check_folder_permission_internal(&pool, fid, &auth_user_id, "write").await
            .map_err(|_| ServerFnError::new("Erro ao validar permissões na pasta de destino."))?;

        if !is_allowed {
            return Err(ServerFnError::new("Pasta de destino não encontrada ou acesso negado."));
        }
    }

    let rows_affected = SheetRepository::move_to_folder(&pool, &sheet_id, folder_id.as_deref(), &auth_user_id)
        .await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to move sheet to folder", Some(&e.to_string()));
            ServerFnError::new("Falha ao mover ficha para a pasta.")
        })?;

    if rows_affected == 0 {
        return Err(ServerFnError::new("Ficha não encontrada ou acesso não autorizado."));
    }

    Ok(())
}

#[server(endpoint = "get_folder_acls")]
pub async fn get_folder_acls(folder_id: String) -> Result<Vec<FolderAclEntry>, ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado."))?;

    // Check ownership or admin
    let is_allowed = check_folder_permission_internal(&pool, &folder_id, &auth_user_id, "admin").await
        .map_err(|_| ServerFnError::new("Erro ao verificar permissão de administração da pasta."))?;

    if !is_allowed {
        return Err(ServerFnError::new("Apenas o proprietário ou administrador pode gerenciar permissões da pasta."));
    }

    let entries = AclRepository::list_for_folder(&pool, &folder_id).await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to fetch folder ACLs", Some(&e.to_string()));
            ServerFnError::new("Falha ao consultar permissões da pasta.")
        })?;

    Ok(entries)
}

#[server(endpoint = "grant_folder_acl")]
pub async fn grant_folder_acl(
    folder_id: String,
    grantee_type: String,
    grantee_identifier: String,
    permission: String,
) -> Result<FolderAclEntry, ServerFnError> {
    use sqlx::SqlitePool;
    use uuid::Uuid;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado."))?;

    let is_allowed = check_folder_permission_internal(&pool, &folder_id, &auth_user_id, "admin").await
        .map_err(|_| ServerFnError::new("Erro ao verificar permissão."))?;

    if !is_allowed {
        return Err(ServerFnError::new("Apenas o proprietário ou administrador pode compartilhar a pasta."));
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
            let u_opt = AclRepository::find_user_by_name(&pool, clean_user).await
                .map_err(|_| ServerFnError::new("Erro ao buscar usuário."))?;

            match u_opt {
                Some((uid, uname)) => {
                    if uid == auth_user_id {
                        return Err(ServerFnError::new("Você já é o proprietário desta pasta."));
                    }
                    (Some(uid), uname)
                }
                None => return Err(ServerFnError::new(format!("Usuário '{}' não encontrado.", clean_user))),
            }
        }
        "room" => {
            let clean_room = grantee_identifier.trim();
            if clean_room.is_empty() {
                return Err(ServerFnError::new("Selecione uma sala para compartilhar."));
            }
            let r_opt = AclRepository::find_room_by_id_or_code(&pool, clean_room).await
                .map_err(|_| ServerFnError::new("Erro ao buscar sala."))?;

            match r_opt {
                Some((rid, rname)) => (Some(rid), rname),
                None => return Err(ServerFnError::new("Sala de crônica não encontrada.")),
            }
        }
        "public" => (None, "Público".to_string()),
        _ => return Err(ServerFnError::new("Tipo de destinatário inválido.")),
    };

    let acl_id = Uuid::new_v4().to_string();
    AclRepository::grant(&pool, &acl_id, &folder_id, &grantee_type, grantee_id.as_deref(), &clean_perm)
        .await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to grant folder ACL", Some(&e.to_string()));
            ServerFnError::new("Falha ao salvar permissão da pasta.")
        })?;

    Ok(FolderAclEntry {
        id: acl_id,
        folder_id,
        grantee_type,
        grantee_id,
        grantee_name,
        permission: clean_perm,
        created_at: String::new(),
    })
}

#[server(endpoint = "revoke_folder_acl")]
pub async fn revoke_folder_acl(acl_id: String) -> Result<(), ServerFnError> {
    use sqlx::SqlitePool;
    let pool = use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::new("Erro interno: Conexão com o banco de dados indisponível")
    })?;

    let auth_user_id = crate::auth::get_auth_user_id().await.unwrap_or(None)
        .ok_or_else(|| ServerFnError::new("Acesso negado."))?;

    let folder_id = AclRepository::find_folder_id_by_acl(&pool, &acl_id).await
        .map_err(|_| ServerFnError::new("Erro ao buscar permissão."))?;

    let fid = match folder_id {
        Some(f) => f,
        None => return Err(ServerFnError::new("Regra de permissão não encontrada.")),
    };

    let is_allowed = check_folder_permission_internal(&pool, &fid, &auth_user_id, "admin").await
        .map_err(|_| ServerFnError::new("Erro ao verificar permissão."))?;

    if !is_allowed {
        return Err(ServerFnError::new("Acesso negado: apenas o proprietário ou administrador pode revogar permissões."));
    }

    AclRepository::revoke(&pool, &acl_id).await
        .map_err(|e| {
            crate::logging::server::write_log(crate::logging::LogCategory::Errors, "ERROR", "Failed to revoke folder ACL", Some(&e.to_string()));
            ServerFnError::new("Falha ao revogar permissão.")
        })?;

    Ok(())
}
