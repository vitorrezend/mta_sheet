use leptos::*;
use crate::state::{CharacterSummary, SheetFolder};
use super::types::{get_folder_path, FOLDER_ICON_PRESETS};

pub fn render_create_folder_modal<FCancel, FSubmit>(
    is_open: ReadSignal<bool>,
    cancel_create_folder: FCancel,
    handle_create_folder: FSubmit,
    folder_input_name: ReadSignal<String>,
    set_folder_input_name: WriteSignal<String>,
    folder_input_icon: ReadSignal<String>,
    set_folder_input_icon: WriteSignal<String>,
    is_saving_folder: ReadSignal<bool>,
    current_lang: crate::i18n::Language,
) -> impl IntoView
where
    FCancel: Fn(ev::MouseEvent) + Copy + 'static,
    FSubmit: Fn(ev::SubmitEvent) + Copy + 'static,
{
    view! {
        {move || if is_open.get() {
            Some(view! {
                <div class="modal-overlay" on:click=cancel_create_folder>
                    <div class="modal-card folder-modal-card" on:click=move |ev| ev.stop_propagation()>
                        <h3 class="modal-title">{crate::i18n::tr("folder_modal_create_title", current_lang)}</h3>
                        <form on:submit=handle_create_folder class="folder-form">
                            <div class="folder-form-field">
                                <label class="folder-field-label">{crate::i18n::tr("folder_name_label", current_lang)}</label>
                                <input
                                    type="text"
                                    class="folder-input"
                                    placeholder=crate::i18n::tr("folder_name_ph", current_lang)
                                    prop:value=folder_input_name
                                    on:input=move |ev| set_folder_input_name.set(event_target_value(&ev))
                                    required
                                    autofocus
                                />
                            </div>
                            <div class="folder-form-field">
                                <label class="folder-field-label">{crate::i18n::tr("folder_icon_label", current_lang)}</label>
                                <div class="folder-icon-presets">
                                    {FOLDER_ICON_PRESETS.iter().map(|&icon| {
                                        let icon_str = icon.to_string();
                                        let icon_val = icon_str.clone();
                                        let is_active = move || folder_input_icon.get() == icon_val;
                                        view! {
                                            <button
                                                type="button"
                                                class="folder-icon-preset-btn"
                                                class:active=is_active
                                                on:click=move |_| set_folder_input_icon.set(icon_str.clone())
                                            >
                                                {icon}
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                            <div class="modal-actions">
                                <button type="button" class="modal-btn btn-cancel" on:click=cancel_create_folder>
                                    {crate::i18n::tr("home_btn_cancel", current_lang)}
                                </button>
                                <button type="submit" class="modal-btn btn-primary" disabled=is_saving_folder>
                                    {crate::i18n::tr("folder_create", current_lang)}
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            })
        } else {
            None
        }}
    }
}

pub fn render_edit_folder_modal<FCancel, FSubmit>(
    folder_to_edit: ReadSignal<Option<SheetFolder>>,
    cancel_edit_folder: FCancel,
    handle_update_folder: FSubmit,
    folder_edit_name: ReadSignal<String>,
    set_folder_edit_name: WriteSignal<String>,
    folder_edit_icon: ReadSignal<String>,
    set_folder_edit_icon: WriteSignal<String>,
    is_saving_folder: ReadSignal<bool>,
    current_lang: crate::i18n::Language,
) -> impl IntoView
where
    FCancel: Fn(ev::MouseEvent) + Copy + 'static,
    FSubmit: Fn(ev::SubmitEvent) + Copy + 'static,
{
    view! {
        {move || folder_to_edit.get().map(|_target| view! {
            <div class="modal-overlay" on:click=cancel_edit_folder>
                <div class="modal-card folder-modal-card" on:click=move |ev| ev.stop_propagation()>
                    <h3 class="modal-title">{crate::i18n::tr("folder_modal_edit_title", current_lang)}</h3>
                    <form on:submit=handle_update_folder class="folder-form">
                        <div class="folder-form-field">
                            <label class="folder-field-label">{crate::i18n::tr("folder_name_label", current_lang)}</label>
                            <input
                                type="text"
                                class="folder-input"
                                placeholder=crate::i18n::tr("folder_name_ph", current_lang)
                                prop:value=folder_edit_name
                                on:input=move |ev| set_folder_edit_name.set(event_target_value(&ev))
                                required
                                autofocus
                            />
                        </div>
                        <div class="folder-form-field">
                            <label class="folder-field-label">{crate::i18n::tr("folder_icon_label", current_lang)}</label>
                            <div class="folder-icon-presets">
                                {FOLDER_ICON_PRESETS.iter().map(|&icon| {
                                    let icon_str = icon.to_string();
                                    let icon_val = icon_str.clone();
                                    let is_active = move || folder_edit_icon.get() == icon_val;
                                    view! {
                                        <button
                                            type="button"
                                            class="folder-icon-preset-btn"
                                            class:active=is_active
                                            on:click=move |_| set_folder_edit_icon.set(icon_str.clone())
                                        >
                                            {icon}
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                        <div class="modal-actions">
                            <button type="button" class="modal-btn btn-cancel" on:click=cancel_edit_folder>
                                {crate::i18n::tr("home_btn_cancel", current_lang)}
                            </button>
                            <button type="submit" class="modal-btn btn-primary" disabled=is_saving_folder>
                                {crate::i18n::tr("folder_save", current_lang)}
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        })}
    }
}

pub fn render_delete_folder_modal<FCancel, FDelete>(
    folder_to_delete: ReadSignal<Option<SheetFolder>>,
    cancel_delete_folder: FCancel,
    handle_delete_folder: FDelete,
    current_lang: crate::i18n::Language,
) -> impl IntoView
where
    FCancel: Fn(ev::MouseEvent) + Copy + 'static,
    FDelete: Fn() + Copy + 'static,
{
    view! {
        {move || folder_to_delete.get().map(|target| view! {
            <div class="modal-overlay" on:click=cancel_delete_folder>
                <div class="modal-card" on:click=move |ev| ev.stop_propagation()>
                    <h3 class="modal-title">{crate::i18n::tr("folder_modal_delete_title", current_lang)}</h3>
                    <p class="modal-text">
                        {crate::i18n::tr("home_delete_prompt", current_lang)}
                        <strong>{target.name}</strong>"?"
                    </p>
                    <p class="modal-subtext">{crate::i18n::tr("folder_modal_delete_desc", current_lang)}</p>
                    <div class="modal-actions">
                        <button type="button" class="modal-btn btn-cancel" on:click=cancel_delete_folder>
                            {crate::i18n::tr("home_btn_cancel", current_lang)}
                        </button>
                        <button type="button" class="modal-btn btn-danger" on:click=move |_| handle_delete_folder()>
                            {crate::i18n::tr("home_btn_confirm_delete", current_lang)}
                        </button>
                    </div>
                </div>
            </div>
        })}
    }
}

pub fn render_move_sheet_modal<FCancel, FMove>(
    sheet_to_move: ReadSignal<Option<CharacterSummary>>,
    folder_list: Vec<SheetFolder>,
    cancel_move_sheet: FCancel,
    handle_move_sheet: FMove,
    current_lang: crate::i18n::Language,
) -> impl IntoView
where
    FCancel: Fn(ev::MouseEvent) + Copy + 'static,
    FMove: Fn(String, Option<String>) + Copy + 'static,
{
    view! {
        {move || sheet_to_move.get().map(|target| {
            let sheet_id = target.id.clone();
            let current_fid = target.folder_id.clone();
            let all_f_modal = folder_list.clone();

            view! {
                <div class="modal-overlay" on:click=cancel_move_sheet>
                    <div class="modal-card folder-modal-card" on:click=move |ev| ev.stop_propagation()>
                        <h3 class="modal-title">{crate::i18n::tr("folder_modal_move_title", current_lang)}</h3>
                        <p class="modal-text">
                            "Mover "<strong>{target.name.clone()}</strong>" para:"
                        </p>
                        <div class="move-folder-options-list">
                            {
                                let sid = sheet_id.clone();
                                let is_curr = current_fid.is_none();
                                view! {
                                    <button
                                        type="button"
                                        class="move-folder-opt-btn"
                                        class:active=is_curr
                                        on:click=move |_| handle_move_sheet(sid.clone(), None)
                                    >
                                        <span class="move-opt-icon">"📁"</span>
                                        <span class="move-opt-name">{crate::i18n::tr("drive_move_to_root", current_lang)}</span>
                                        {if is_curr {
                                            view! { <span class="move-opt-curr">"✓ Atual"</span> }.into_view()
                                        } else {
                                            ().into_view()
                                        }}
                                    </button>
                                }
                            }

                            {folder_list.clone().into_iter().map(|f| {
                                let sid = sheet_id.clone();
                                let fid = f.id.clone();
                                let is_curr = current_fid.as_deref() == Some(fid.as_str());
                                let display_path = get_folder_path(&fid, &all_f_modal);
                                view! {
                                    <button
                                        type="button"
                                        class="move-folder-opt-btn"
                                        class:active=is_curr
                                        on:click=move |_| handle_move_sheet(sid.clone(), Some(fid.clone()))
                                    >
                                        <span class="move-opt-icon">{f.icon.clone()}</span>
                                        <span class="move-opt-name">{display_path}</span>
                                        {if is_curr {
                                            view! { <span class="move-opt-curr">"✓ Atual"</span> }.into_view()
                                        } else {
                                            ().into_view()
                                        }}
                                    </button>
                                }
                            }).collect_view()}
                        </div>
                        <div class="modal-actions">
                            <button type="button" class="modal-btn btn-cancel" on:click=cancel_move_sheet>
                                {crate::i18n::tr("home_btn_cancel", current_lang)}
                            </button>
                        </div>
                    </div>
                </div>
            }
        })}
    }
}
