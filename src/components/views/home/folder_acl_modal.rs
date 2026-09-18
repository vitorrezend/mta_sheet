use leptos::*;
use crate::state::{FolderAclEntry, SheetFolder};

pub fn render_folder_acl_modal<FCancel, FGrant, FRevoke>(
    folder_to_share: ReadSignal<Option<SheetFolder>>,
    cancel_share_folder: FCancel,
    acl_error_msg: ReadSignal<Option<String>>,
    share_grantee_type: ReadSignal<String>,
    set_share_grantee_type: WriteSignal<String>,
    share_target_input: ReadSignal<String>,
    set_share_target_input: WriteSignal<String>,
    share_permission_level: ReadSignal<String>,
    set_share_permission_level: WriteSignal<String>,
    is_granting_acl: ReadSignal<bool>,
    handle_grant_acl: FGrant,
    is_loading_acls: ReadSignal<bool>,
    folder_acls_list: ReadSignal<Vec<FolderAclEntry>>,
    handle_revoke_acl: FRevoke,
    current_lang: crate::i18n::Language,
) -> impl IntoView
where
    FCancel: Fn(ev::MouseEvent) + Copy + 'static,
    FGrant: Fn(ev::SubmitEvent) + Copy + 'static,
    FRevoke: Fn(String) + Copy + 'static,
{
    view! {
        {move || folder_to_share.get().map(|target_folder| {
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
                                on:click=move |_| set_share_grantee_type.set("user".to_string())
                            >
                                <span>"👤"</span>
                                <span>{crate::i18n::tr("drive_share_with_user", current_lang)}</span>
                            </button>
                            <button
                                type="button"
                                class="share-type-tab-btn"
                                class:active=move || share_grantee_type.get() == "room"
                                on:click=move |_| set_share_grantee_type.set("room".to_string())
                            >
                                <span>"🏰"</span>
                                <span>{crate::i18n::tr("drive_share_with_room", current_lang)}</span>
                            </button>
                            <button
                                type="button"
                                class="share-type-tab-btn"
                                class:active=move || share_grantee_type.get() == "public"
                                on:click=move |_| set_share_grantee_type.set("public".to_string())
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
                                                on:input=move |ev| set_share_target_input.set(event_target_value(&ev))
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
                                        on:change=move |ev| set_share_permission_level.set(event_target_value(&ev))
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
        })}
    }
}
