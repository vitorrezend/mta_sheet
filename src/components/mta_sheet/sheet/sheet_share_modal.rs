use leptos::*;
use crate::components::common::{Modal, ModalSize, SafeCallback};
use crate::state::models::{SheetAclEntry, SheetShareSettings};
use crate::state::{get_sheet_share_settings, update_sheet_share_link, grant_sheet_acl, revoke_sheet_acl};

#[component]
pub fn SheetShareModal(
    show_modal: ReadSignal<bool>,
    set_show_modal: WriteSignal<bool>,
    sheet_id: Signal<String>,
    sheet_name: Signal<String>,
) -> impl IntoView {
    let (settings, set_settings) = create_signal(Option::<SheetShareSettings>::None);
    let (is_loading, set_is_loading) = create_signal(false);
    let (error_msg, set_error_msg) = create_signal(Option::<String>::None);
    let (copied_toast, set_copied_toast) = create_signal(false);

    let (grantee_type, set_grantee_type) = create_signal("user".to_string());
    let (grantee_input, set_grantee_input) = create_signal(String::new());
    let (grantee_perm, set_grantee_perm) = create_signal("read".to_string());
    let (is_submitting, set_is_submitting) = create_signal(false);

    // Carrega as configurações quando o modal abre
    create_effect(move |_| {
        if show_modal.get() {
            let sid = sheet_id.get();
            if !sid.is_empty() {
                set_is_loading.set(true);
                set_error_msg.set(None);
                spawn_local(async move {
                    match get_sheet_share_settings(sid).await {
                        Ok(data) => {
                            set_settings.set(Some(data));
                            set_is_loading.set(false);
                        }
                        Err(e) => {
                            set_error_msg.set(Some(e.to_string()));
                            set_is_loading.set(false);
                        }
                    }
                });
            }
        }
    });

    let on_close = SafeCallback::new(move |_| {
        set_show_modal.set(false);
        set_error_msg.set(None);
        set_copied_toast.set(false);
    });

    // Copiar Link para a área de transferência
    let copy_link = {
        let sheet_id = sheet_id.clone();
        move |_| {
            if let Some(s) = settings.get() {
                let sid = sheet_id.get();
                let token_str = s.share_token.unwrap_or_default();
                
                #[cfg(target_arch = "wasm32")]
                if let Some(window) = web_sys::window() {
                    let origin = window.location().origin().unwrap_or_default();
                    let url = if token_str.is_empty() {
                        format!("{}/sheet/{}", origin, sid)
                    } else {
                        format!("{}/sheet/{}?token={}", origin, sid, token_str)
                    };

                    let _ = window.navigator().clipboard().write_text(&url);
                    set_copied_toast.set(true);

                    // Auto-hide toast após 3 segundos
                    set_timeout(
                        move || {
                            set_copied_toast.set(false);
                        },
                        std::time::Duration::from_secs(3),
                    );
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let _ = (&sid, &token_str);
                }
            }
        }
    };

    // Alterar Modo do Link Geral (Restrito vs Qualquer pessoa com o link)
    let on_link_perm_change = {
        let sheet_id = sheet_id.clone();
        move |ev: ev::Event| {
            let new_perm = event_target_value(&ev);
            let sid = sheet_id.get();
            set_error_msg.set(None);

            spawn_local(async move {
                match update_sheet_share_link(sid, new_perm, false).await {
                    Ok(updated) => {
                        set_settings.set(Some(updated));
                    }
                    Err(e) => {
                        set_error_msg.set(Some(e.to_string()));
                    }
                }
            });
        }
    };

    // Redefinir Link de Compartilhamento (Gerar novo token)
    let on_reset_link = {
        let sheet_id = sheet_id.clone();
        move |_| {
            let sid = sheet_id.get();
            set_error_msg.set(None);

            spawn_local(async move {
                match update_sheet_share_link(sid, "view".to_string(), true).await {
                    Ok(updated) => {
                        set_settings.set(Some(updated));
                        set_copied_toast.set(false);
                    }
                    Err(e) => {
                        set_error_msg.set(Some(e.to_string()));
                    }
                }
            });
        }
    };

    // Conceder Permissão Nominal
    let handle_grant = {
        let sheet_id = sheet_id.clone();
        move |ev: ev::SubmitEvent| {
            ev.prevent_default();
            let sid = sheet_id.get();
            let gtype = grantee_type.get();
            let gtarget = grantee_input.get();
            let perm = grantee_perm.get();

            if gtype != "public" && gtarget.trim().is_empty() {
                return;
            }

            set_is_submitting.set(true);
            set_error_msg.set(None);

            spawn_local(async move {
                match grant_sheet_acl(sid, gtype, gtarget, perm).await {
                    Ok(new_entry) => {
                        set_settings.update(|opt| {
                            if let Some(s) = opt {
                                s.acls.retain(|a| !(a.grantee_type == new_entry.grantee_type && a.grantee_id == new_entry.grantee_id));
                                s.acls.push(new_entry);
                            }
                        });
                        set_grantee_input.set(String::new());
                        set_is_submitting.set(false);
                    }
                    Err(e) => {
                        set_error_msg.set(Some(e.to_string()));
                        set_is_submitting.set(false);
                    }
                }
            });
        }
    };

    // Revogar Permissão Nominal
    let handle_revoke = move |acl_id: String| {
        set_error_msg.set(None);
        let aid_copy = acl_id.clone();
        spawn_local(async move {
            match revoke_sheet_acl(aid_copy.clone()).await {
                Ok(_) => {
                    set_settings.update(|opt| {
                        if let Some(s) = opt {
                            s.acls.retain(|a| a.id != aid_copy);
                        }
                    });
                }
                Err(e) => {
                    set_error_msg.set(Some(e.to_string()));
                }
            }
        });
    };

    let title_prop: MaybeSignal<String> = "Compartilhar Ficha".to_string().into();
    let subtitle_prop = Some(Signal::derive(move || format!("{} • Permissões & Acesso", sheet_name.get())).into());

    view! {
        <Modal
            is_open=show_modal
            on_close=on_close
            title=title_prop
            subtitle=subtitle_prop
            icon=Some("🔗")
            size=ModalSize::Md
            extra_class="sheet-share-modal".to_string()
        >
            <div class="sheet-share-modal-body">
                {move || error_msg.get().map(|err| view! {
                    <div class="alert-box alert-error" style="margin-bottom: 0.5rem; padding: 0.5rem 0.75rem; font-size: 0.85rem;">
                        <span>{err}</span>
                    </div>
                })}

                // ── SEÇÃO 1: COMPARTILHAR COM PESSOAS OU CRÔNICAS ──
                <div class="share-section-heading">
                    <span>"👤"</span>
                    <span>"Compartilhar com Pessoas e Crônicas"</span>
                </div>

                <div class="share-type-tabs">
                    <button
                        type="button"
                        class="share-type-tab-btn"
                        class:active=move || grantee_type.get() == "user"
                        on:click=move |_| set_grantee_type.set("user".to_string())
                    >
                        <span>"👤"</span>
                        <span>"Jogador"</span>
                    </button>
                    <button
                        type="button"
                        class="share-type-tab-btn"
                        class:active=move || grantee_type.get() == "room"
                        on:click=move |_| set_grantee_type.set("room".to_string())
                    >
                        <span>"🏰"</span>
                        <span>"Crônica / Sala"</span>
                    </button>
                    <button
                        type="button"
                        class="share-type-tab-btn"
                        class:active=move || grantee_type.get() == "public"
                        on:click=move |_| set_grantee_type.set("public".to_string())
                    >
                        <span>"🌐"</span>
                        <span>"Público Geral"</span>
                    </button>
                </div>

                <form on:submit=handle_grant class="share-grant-form">
                    {move || {
                        let gtype = grantee_type.get();
                        if gtype == "public" {
                            view! {
                                <div class="share-public-banner">
                                    <span class="share-public-icon">"🌐"</span>
                                    <div class="share-public-text">
                                        <strong>"Acesso Público Geral"</strong>
                                        <span>"Qualquer usuário cadastrado no sistema poderá acessar esta ficha nominalmente."</span>
                                    </div>
                                </div>
                            }.into_view()
                        } else {
                            let (label_text, placeholder) = if gtype == "user" {
                                ("Nome do Jogador / Usuário", "Digite o @usuario exato...")
                            } else {
                                ("Sala de Crônica (Código ou ID)", "Digite o código MTA-XXXX...")
                            };

                            view! {
                                <div class="share-field-group">
                                    <label class="share-field-label">{label_text}</label>
                                    <input
                                        class="share-target-input"
                                        type="text"
                                        placeholder=placeholder
                                        prop:value=grantee_input
                                        on:input=move |ev| set_grantee_input.set(event_target_value(&ev))
                                        required
                                    />
                                </div>
                            }.into_view()
                        }
                    }}

                    <div class="share-actions-row">
                        <div class="share-perm-select-col">
                            <label class="share-field-label">"Nível de Acesso"</label>
                            <select
                                class="share-perm-select"
                                on:change=move |ev| set_grantee_perm.set(event_target_value(&ev))
                                prop:value=grantee_perm
                            >
                                <option value="read">"Leitor (Visualização Estática)"</option>
                                <option value="write">"Editor (Pode Alterar)"</option>
                            </select>
                        </div>
                        <button
                            type="submit"
                            class="share-add-btn"
                            disabled=is_submitting
                        >
                            <span>"➕"</span>
                            <span>"Adicionar"</span>
                        </button>
                    </div>
                </form>

                <div class="share-section-heading">
                    <span>"👥"</span>
                    <span>"Pessoas com Acesso"</span>
                </div>

                <div class="share-entries-list">
                    // Entrada do Proprietário
                    <div class="share-entry-item">
                        <div class="share-entry-left">
                            <span class="share-entry-icon">"👑"</span>
                            <span class="share-entry-name">"Você (Proprietário)"</span>
                        </div>
                        <div class="share-entry-right">
                            <span class="share-perm-badge badge-owner">"Proprietário"</span>
                        </div>
                    </div>

                    // Lista de Outras Permissões
                    {move || {
                        if is_loading.get() {
                            view! { <p style="font-size: 0.85rem; color: #64748b; padding: 0.5rem 0;">"Carregando acessos..."</p> }.into_view()
                        } else if let Some(ref s) = settings.get() {
                            if s.acls.is_empty() {
                                view! { <p style="font-size: 0.82rem; color: #94a3b8; font-style: italic; padding: 0.25rem 0;">"Nenhuma permissão nominal adicionada."</p> }.into_view()
                            } else {
                                s.acls.iter().map(|entry| {
                                    let acl_id = entry.id.clone();
                                    let type_icon = match entry.grantee_type.as_str() {
                                        "user" => "👤",
                                        "room" => "🏰",
                                        _ => "🌐",
                                    };
                                    let (badge_class, perm_label) = match entry.permission.as_str() {
                                        "admin" => ("badge-admin", "Admin"),
                                        "write" => ("badge-write", "Editor"),
                                        _ => ("badge-read", "Leitor"),
                                    };

                                    let aid_del = acl_id.clone();
                                    let handle_rev = handle_revoke.clone();

                                    view! {
                                        <div class="share-entry-item">
                                            <div class="share-entry-left">
                                                <span class="share-entry-icon">{type_icon}</span>
                                                <span class="share-entry-name">{&entry.grantee_name}</span>
                                            </div>
                                            <div class="share-entry-right">
                                                <span class=format!("share-perm-badge {}", badge_class)>{perm_label}</span>
                                                <button
                                                    type="button"
                                                    class="share-revoke-btn"
                                                    title="Remover acesso"
                                                    on:click=move |_| handle_rev(aid_del.clone())
                                                >
                                                    "✕"
                                                </button>
                                            </div>
                                        </div>
                                    }
                                }).collect_view().into_view()
                            }
                        } else {
                            view! { <div></div> }.into_view()
                        }
                    }}
                </div>

                <div class="share-section-divider"></div>

                // ── SEÇÃO 2: ACESSO GERAL POR LINK (ESTILO GOOGLE DRIVE) ──
                <div class="share-section-heading">
                    <span>"🌐"</span>
                    <span>"Acesso Geral por Link"</span>
                </div>

                <div class="share-general-box">
                    <div class="share-general-row">
                        <div class="share-general-info">
                            <span class="share-general-icon">
                                {move || match settings.get().map(|s| s.share_permission).unwrap_or_default().as_str() {
                                    "view" => "🔗",
                                    _ => "🔒",
                                }}
                            </span>
                            <div class="share-general-text">
                                <strong>
                                    {move || match settings.get().map(|s| s.share_permission).unwrap_or_default().as_str() {
                                        "view" => "Qualquer pessoa com o link",
                                        _ => "Restrito",
                                    }}
                                </strong>
                                <span>
                                    {move || match settings.get().map(|s| s.share_permission).unwrap_or_default().as_str() {
                                        "view" => "Qualquer pessoa na internet com este link pode visualizar em modo estático e clonar.",
                                        _ => "Somente pessoas com acesso nominal adicionadas acima podem abrir.",
                                    }}
                                </span>
                            </div>
                        </div>

                        <div class="share-general-select-col">
                            <select
                                class="share-general-select"
                                on:change=on_link_perm_change
                                prop:value=move || settings.get().map(|s| s.share_permission).unwrap_or_else(|| "none".to_string())
                            >
                                <option value="none">"Restrito"</option>
                                <option value="view">"Qualquer pessoa com o link (Leitor)"</option>
                            </select>
                        </div>
                    </div>

                    <div class="share-link-actions-row">
                        <button
                            type="button"
                            class="share-copy-link-btn"
                            class:copied=move || copied_toast.get()
                            on:click=copy_link
                            disabled=move || settings.get().map(|s| s.share_permission == "none").unwrap_or(true)
                        >
                            <span class="btn-icon">{move || if copied_toast.get() { "✓" } else { "🔗" }}</span>
                            <span class="btn-text">
                                {move || if copied_toast.get() { "Link Copiado!" } else { "Copiar Link" }}
                            </span>
                        </button>

                        {move || {
                            let is_link_active = settings.get().map(|s| s.share_permission == "view").unwrap_or(false);
                            if is_link_active {
                                view! {
                                    <button
                                        type="button"
                                        class="share-reset-link-btn"
                                        on:click=on_reset_link
                                        title="Gera um novo link e invalida o link anterior imediatamente"
                                    >
                                        "🔄 Redefinir Link"
                                    </button>
                                }.into_view()
                            } else {
                                view! { <div></div> }.into_view()
                            }
                        }}
                    </div>
                </div>
            </div>
        </Modal>
    }
}
