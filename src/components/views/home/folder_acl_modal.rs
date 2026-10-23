use leptos::*;
use crate::state::{get_folder_acls, grant_folder_acl, revoke_folder_acl, FolderAclEntry};
use super::store::{use_home, use_lang};

/// Estado da janela de compartilhamento: formulário de novo acesso e a lista de acessos da pasta.
#[derive(Clone, Copy)]
struct AclState {
    entries: RwSignal<Vec<FolderAclEntry>>,
    loading: RwSignal<bool>,
    granting: RwSignal<bool>,
    error: RwSignal<Option<String>>,
    grantee_type: RwSignal<String>,
    target: RwSignal<String>,
    permission: RwSignal<String>,
}

impl AclState {
    fn load(self, folder_id: String) {
        self.loading.set(true);
        self.error.set(None);
        spawn_local(async move {
            match get_folder_acls(folder_id).await {
                Ok(entries) => self.entries.set(entries),
                Err(e) => self.error.set(Some(format!("Erro ao carregar permissões: {}", e))),
            }
            self.loading.set(false);
        });
    }

    fn grant(self, folder_id: String) {
        if self.granting.get_untracked() {
            return;
        }
        let grantee_type = self.grantee_type.get_untracked();
        let target = self.target.get_untracked().trim().to_string();
        if grantee_type != "public" && target.is_empty() {
            self.error.set(Some("Por favor, preencha o destinatário.".to_string()));
            return;
        }
        self.granting.set(true);
        self.error.set(None);
        let permission = self.permission.get_untracked();
        spawn_local(async move {
            match grant_folder_acl(folder_id.clone(), grantee_type, target, permission).await {
                Ok(_) => {
                    self.target.set(String::new());
                    self.load(folder_id);
                }
                Err(e) => self.error.set(Some(format!("Erro ao conceder permissão: {}", e))),
            }
            self.granting.set(false);
        });
    }

    fn revoke(self, folder_id: String, acl_id: String) {
        spawn_local(async move {
            match revoke_folder_acl(acl_id).await {
                Ok(_) => self.load(folder_id),
                Err(e) => self.error.set(Some(format!("Erro ao revogar permissão: {}", e))),
            }
        });
    }
}

#[component]
pub fn FolderAclModal() -> impl IntoView {
    let home = use_home();
    let lang = use_lang();
    let folder_to_share = home.folder_to_share;
    let acl = AclState {
        entries: create_rw_signal(Vec::new()),
        loading: create_rw_signal(false),
        granting: create_rw_signal(false),
        error: create_rw_signal(None),
        grantee_type: create_rw_signal("user".to_string()),
        target: create_rw_signal(String::new()),
        permission: create_rw_signal("read".to_string()),
    };
    // Ao abrir para uma pasta: limpa o formulário e carrega os acessos dela
    create_effect(move |_| {
        if let Some(id) = folder_to_share.with(|f| f.as_ref().map(|f| f.id.clone())) {
            acl.target.set(String::new());
            acl.error.set(None);
            acl.load(id);
        }
    });
    let shared_id = move || folder_to_share.with_untracked(|f| f.as_ref().map(|f| f.id.clone()));
    let cancel_share_folder = move |_| {
        folder_to_share.set(None);
        acl.error.set(None);
    };
    let handle_grant_acl = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if let Some(id) = shared_id() {
            acl.grant(id);
        }
    };
    let handle_revoke_acl = move |acl_id: String| {
        if let Some(id) = shared_id() {
            acl.revoke(id, acl_id);
        }
    };
    let AclState {
        entries: folder_acls_list,
        loading: is_loading_acls,
        granting: is_granting_acl,
        error: acl_error_msg,
        grantee_type: share_grantee_type,
        target: share_target_input,
        permission: share_permission_level,
    } = acl;

    view! {
        {move || { let current_lang = lang.get(); folder_to_share.get().map(|target_folder| {
            let tf_name = target_folder.name.clone();
            let tf_icon = target_folder.icon.clone();

            view! {
                <div class="modal-overlay" on:click=cancel_share_folder>
                    <div class="modal-card share-modal-card" on:click=move |ev| ev.stop_propagation()>
                        <div class="share-modal-header">
                            <div class="share-modal-header-info">
                                <h3 class="modal-title">{crate::i18n::tr("drive_share_modal_title", current_lang)}</h3>
                                <div class="share-modal-subtitle">
                                    <span>{tf_icon}</span>
                                    <strong>{tf_name}</strong>
                                </div>
                            </div>
                            <button type="button" class="share-close-x-btn" on:click=cancel_share_folder title="Fechar">
                                "✕"
                            </button>
                        </div>

                        {move || acl_error_msg.get().map(|msg| view! {
                            <div class="alert-box alert-error" style="margin-bottom: 0.75rem; padding: 0.5rem 0.75rem; font-size: 0.85rem;">
                                <span>{msg}</span>
                            </div>
                        })}

                        // Grantee type selector
                        <div class="share-type-tabs">
                            <button
                                type="button"
                                class="share-type-tab-btn"
                                class:active=move || share_grantee_type.get() == "user"
                                on:click=move |_| share_grantee_type.set("user".to_string())
                            >
                                <span>"👤"</span>
                                <span>{crate::i18n::tr("drive_share_with_user", current_lang)}</span>
                            </button>
                            <button
                                type="button"
                                class="share-type-tab-btn"
                                class:active=move || share_grantee_type.get() == "room"
                                on:click=move |_| share_grantee_type.set("room".to_string())
                            >
                                <span>"🏰"</span>
                                <span>{crate::i18n::tr("drive_share_with_room", current_lang)}</span>
                            </button>
                            <button
                                type="button"
                                class="share-type-tab-btn"
                                class:active=move || share_grantee_type.get() == "public"
                                on:click=move |_| share_grantee_type.set("public".to_string())
                            >
                                <span>"🌐"</span>
                                <span>"Público"</span>
                            </button>
                        </div>

                        // Add Access Form
                        <form on:submit=handle_grant_acl class="share-grant-form">
                            {move || {
                                let gtype = share_grantee_type.get();
                                if gtype == "public" {
                                    view! {
                                        <div class="share-public-banner">
                                            <span class="share-public-icon">"🌐"</span>
                                            <div class="share-public-text">
                                                <strong>"Acesso Público Geral"</strong>
                                                <span>"Qualquer usuário cadastrado no sistema poderá acessar esta pasta."</span>
                                            </div>
                                        </div>
                                    }.into_view()
                                } else {
                                    let (label_text, placeholder) = if gtype == "user" {
                                        ("Nome do Jogador / Usuário", "Digite o nome exato do usuário (ex: vitor, narrador)...")
                                    } else {
                                        ("Sala de Crônica (ID ou Código)", "Digite o código da sala (ex: MTA-AB12) ou ID...")
                                    };
                                    view! {
                                        <div class="share-field-group">
                                            <label class="share-field-label">{label_text}</label>
                                            <input class="share-target-input"
                                                type="text"
                                                placeholder=placeholder
                                                prop:value=share_target_input
                                                on:input=move |ev| share_target_input.set(event_target_value(&ev))
                                                required
                                            />
                                        </div>
                                    }.into_view()
                                }
                            }}

                            <div class="share-actions-row">
                                <div class="share-perm-select-col">
                                    <label class="share-field-label">"Nível de Permissão"</label>
                                    <select
                                        class="share-perm-select"
                                        on:change=move |ev| share_permission_level.set(event_target_value(&ev))
                                        prop:value=share_permission_level
                                    >
                                        <option value="read">{crate::i18n::tr("drive_share_permission_read", current_lang)}</option>
                                        <option value="write">{crate::i18n::tr("drive_share_permission_write", current_lang)}</option>
                                        <option value="admin">{crate::i18n::tr("drive_share_permission_admin", current_lang)}</option>
                                    </select>
                                </div>
                                <button
                                    type="submit"
                                    class="share-add-btn"
                                    disabled=is_granting_acl
                                >
                                    <span>"➕"</span>
                                    <span>{crate::i18n::tr("drive_share_add_btn", current_lang)}</span>
                                </button>
                            </div>
                        </form>

                        <div class="share-inherited-note">
                            {crate::i18n::tr("drive_share_inherited_note", current_lang)}
                        </div>

                        <div class="share-section-heading">"Acessos Ativos"</div>

                        <div class="share-entries-list">
                            // Owner entry
                            <div class="share-entry-item">
                                <div class="share-entry-left">
                                    <span class="share-entry-icon">"👑"</span>
                                    <span class="share-entry-name">"Você (Proprietário)"</span>
                                </div>
                                <div class="share-entry-right">
                                    <span class="share-perm-badge badge-owner">{crate::i18n::tr("drive_share_owner", current_lang)}</span>
                                </div>
                            </div>

                            // Other ACL entries
                            {move || {
                                if is_loading_acls.get() {
                                    view! { <p style="font-size: 0.85rem; color: #64748b; padding: 0.5rem 0;">"Carregando permissões..."</p> }.into_view()
                                } else {
                                    let acls = folder_acls_list.get();
                                    if acls.is_empty() {
                                        view! { <p style="font-size: 0.85rem; color: #94a3b8; font-style: italic; padding: 0.5rem 0;">{crate::i18n::tr("drive_share_empty", current_lang)}</p> }.into_view()
                                    } else {
                                        acls.into_iter().map(|entry| {
                                            let acl_id = entry.id.clone();
                                            let type_icon = match entry.grantee_type.as_str() {
                                                "user" => "👤",
                                                "room" => "🏰",
                                                _ => "🌐",
                                            };
                                            let badge_class = match entry.permission.as_str() {
                                                "admin" => "badge-admin",
                                                "write" => "badge-write",
                                                _ => "badge-read",
                                            };
                                            let perm_label = match entry.permission.as_str() {
                                                "admin" => crate::i18n::tr("drive_share_permission_admin", current_lang),
                                                "write" => crate::i18n::tr("drive_share_permission_write", current_lang),
                                                _ => crate::i18n::tr("drive_share_permission_read", current_lang),
                                            };
                                            let aid_del = acl_id.clone();
                                            view! {
                                                <div class="share-entry-item">
                                                    <div class="share-entry-left">
                                                        <span class="share-entry-icon">{type_icon}</span>
                                                        <span class="share-entry-name">{entry.grantee_name}</span>
                                                    </div>
                                                    <div class="share-entry-right">
                                                        <span class=format!("share-perm-badge {}", badge_class)>{perm_label}</span>
                                                        <button
                                                            type="button"
                                                            class="share-revoke-btn"
                                                            title=crate::i18n::tr("drive_share_revoke_btn", current_lang)
                                                            on:click=move |_| handle_revoke_acl(aid_del.clone())
                                                        >
                                                            "✕"
                                                        </button>
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()
                                    }
                                }
                            }}
                        </div>

                        <div class="modal-actions">
                            <button type="button" class="modal-btn btn-cancel" on:click=cancel_share_folder>
                                {crate::i18n::tr("drive_share_close", current_lang)}
                            </button>
                        </div>
                    </div>
                </div>
            }
        })}}
    }
}
