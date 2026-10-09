use leptos::*;
use std::collections::HashMap;
use crate::state::{CharacterSummary, SheetFolder};
use super::folder_card::render_single_folder_card;
use super::sheet_card::render_character_grid;

#[derive(Clone, Copy)]
pub struct DriveState {
    pub drag_over_folder_id: ReadSignal<Option<String>>,
    pub dragged_sheet_id: ReadSignal<Option<String>>,
    pub dragged_folder_id: ReadSignal<Option<String>>,
    pub set_drag_over_folder_id: WriteSignal<Option<String>>,
    pub set_dragged_sheet_id: WriteSignal<Option<String>>,
    pub set_dragged_folder_id: WriteSignal<Option<String>>,
    pub set_selected_folder_id: WriteSignal<Option<String>>,
    pub set_create_folder_parent: WriteSignal<Option<String>>,
    pub set_folder_input_name: WriteSignal<String>,
    pub set_folder_input_icon: WriteSignal<String>,
    pub set_is_create_folder_open: WriteSignal<bool>,
    pub set_folder_edit_name: WriteSignal<String>,
    pub set_folder_edit_icon: WriteSignal<String>,
    pub set_folder_to_edit: WriteSignal<Option<SheetFolder>>,
    pub set_folder_to_delete: WriteSignal<Option<SheetFolder>>,
    pub set_sheet_to_delete: WriteSignal<Option<CharacterSummary>>,
    pub set_sheet_to_move: WriteSignal<Option<CharacterSummary>>,
}

pub fn render_folder_view<FShare, FMoveSheet, FMoveFolder, FTogglePrivacy>(
    current_fid: &str,
    folder_list: &[SheetFolder],
    all_sheets: Vec<CharacterSummary>,
    folder_counts: &HashMap<String, usize>,
    folders_info: HashMap<String, (String, String)>,
    current_lang: crate::i18n::Language,
    ds: &DriveState,
    open_share_modal: FShare,
    handle_move_sheet: FMoveSheet,
    handle_move_folder: FMoveFolder,
    toggle_privacy: FTogglePrivacy,
) -> View
where
    FShare: Fn(SheetFolder) + Copy + 'static,
    FMoveSheet: Fn(String, Option<String>) + Copy + 'static,
    FMoveFolder: Fn(String, Option<String>) + Copy + 'static,
    FTogglePrivacy: Fn(String, bool) + Copy + 'static,
{
    let current_folder = folder_list.iter().find(|f| f.id == current_fid).cloned();
    let folder_sheets: Vec<CharacterSummary> = all_sheets
        .into_iter()
        .filter(|s| s.folder_id.as_deref() == Some(current_fid))
        .collect();
    let sheets_count = folder_sheets.len();

    let current_folder = match current_folder {
        Some(f) => f,
        None => {
            ds.set_selected_folder_id.set(None);
            return view! { <div class="drive-container"></div> }.into_view();
        }
    };
    let f_parent_id = current_folder.parent_id.clone();
    let f_for_edit = current_folder.clone();
    let f_for_del = current_folder.clone();
    let f_for_share = current_folder.clone();

    // Compute ancestry path for multi-level breadcrumbs
    let mut ancestry = Vec::new();
    let mut curr_id = Some(current_fid.to_string());
    while let Some(cid) = curr_id {
        if let Some(f) = folder_list.iter().find(|item| item.id == cid) {
            ancestry.push(f.clone());
            curr_id = f.parent_id.clone();
        } else {
            break;
        }
    }
    ancestry.reverse();

    // Subfolders
    let subfolders: Vec<SheetFolder> = folder_list
        .iter()
        .filter(|f| f.parent_id.as_deref() == Some(current_fid))
        .cloned()
        .collect();
    let has_subfolders = !subfolders.is_empty();

    let drag_over_folder_id = ds.drag_over_folder_id;
    let dragged_sheet_id = ds.dragged_sheet_id;
    let dragged_folder_id = ds.dragged_folder_id;
    let set_selected_folder_id = ds.set_selected_folder_id;
    let set_drag_over_folder_id = ds.set_drag_over_folder_id;
    let set_dragged_sheet_id = ds.set_dragged_sheet_id;
    let set_dragged_folder_id = ds.set_dragged_folder_id;
    let set_create_folder_parent = ds.set_create_folder_parent;
    let set_folder_input_name = ds.set_folder_input_name;
    let set_folder_input_icon = ds.set_folder_input_icon;
    let set_is_create_folder_open = ds.set_is_create_folder_open;
    let set_folder_edit_name = ds.set_folder_edit_name;
    let set_folder_edit_icon = ds.set_folder_edit_icon;
    let set_folder_to_edit = ds.set_folder_to_edit;
    let set_folder_to_delete = ds.set_folder_to_delete;
    let set_sheet_to_delete = ds.set_sheet_to_delete;
    let set_sheet_to_move = ds.set_sheet_to_move;

    let cur_fid_str = current_fid.to_string();

    view! {
        <div class="drive-container">
            <div class="drive-nav-bar">
                <div class="drive-breadcrumbs">
                    <button
                        type="button"
                        class="drive-breadcrumb-item clickable"
                        class:drag-over=move || drag_over_folder_id.get().as_deref() == Some("__root__")
                        class:drag-targetable=move || dragged_sheet_id.get().is_some() || dragged_folder_id.get().is_some()
                        on:click=move |_| set_selected_folder_id.set(None)
                        on:dragover=move |ev: ev::DragEvent| {
                            ev.prevent_default();
                            if let Some(dt) = ev.data_transfer() {
                                let _ = dt.set_drop_effect("move");
                            }
                        }
                        on:dragenter=move |ev: ev::DragEvent| {
                            ev.prevent_default();
                            set_drag_over_folder_id.set(Some("__root__".to_string()));
                        }
                        on:dragleave=move |_| {
                            if drag_over_folder_id.get_untracked().as_deref() == Some("__root__") {
                                set_drag_over_folder_id.set(None);
                            }
                        }
                        on:drop=move |ev: ev::DragEvent| {
                            ev.prevent_default();
                            set_drag_over_folder_id.set(None);
                            if let Some(s_id) = dragged_sheet_id.get_untracked() {
                                handle_move_sheet(s_id, None);
                                set_dragged_sheet_id.set(None);
                            } else if let Some(f_id) = dragged_folder_id.get_untracked() {
                                handle_move_folder(f_id, None);
                                set_dragged_folder_id.set(None);
                            }
                        }
                        title=crate::i18n::tr("drive_move_to_root", current_lang)
                    >
                        <span class="drive-bc-icon">"📁"</span>
                        <span>{crate::i18n::tr("drive_my_sheets", current_lang)}</span>
                    </button>

                    {
                        let total_anc = ancestry.len();
                        ancestry.into_iter().enumerate().map(|(idx, anc_f)| {
                            let is_last = idx == total_anc - 1;
                            let anc_id = anc_f.id.clone();
                            let anc_id_drop = anc_f.id.clone();
                            let anc_id_over = anc_f.id.clone();
                            let anc_icon = anc_f.icon.clone();
                            let anc_name = anc_f.name.clone();

                            if is_last {
                                view! {
                                    <span class="drive-breadcrumb-sep">"/"</span>
                                    <span class="drive-breadcrumb-item active">
                                        <span class="drive-bc-icon">{anc_icon}</span>
                                        <span>{anc_name}</span>
                                    </span>
                                }.into_view()
                            } else {
                                view! {
                                    <span class="drive-breadcrumb-sep">"/"</span>
                                    <button
                                        type="button"
                                        class="drive-breadcrumb-item clickable"
                                        class:drag-over={
                                            let chk = anc_id_over.clone();
                                            move || drag_over_folder_id.get().as_deref() == Some(chk.as_str())
                                        }
                                        class:drag-targetable=move || dragged_sheet_id.get().is_some() || dragged_folder_id.get().is_some()
                                        on:click={
                                            let cid = anc_id.clone();
                                            move |_| set_selected_folder_id.set(Some(cid.clone()))
                                        }
                                        on:dragover=move |ev: ev::DragEvent| {
                                            ev.prevent_default();
                                            if let Some(dt) = ev.data_transfer() {
                                                let _ = dt.set_drop_effect("move");
                                            }
                                        }
                                        on:dragenter={
                                            let eid = anc_id_drop.clone();
                                            move |ev: ev::DragEvent| {
                                                ev.prevent_default();
                                                set_drag_over_folder_id.set(Some(eid.clone()));
                                            }
                                        }
                                        on:dragleave={
                                            let lid = anc_id_drop.clone();
                                            move |_| {
                                                if drag_over_folder_id.get_untracked().as_deref() == Some(lid.as_str()) {
                                                    set_drag_over_folder_id.set(None);
                                                }
                                            }
                                        }
                                        on:drop={
                                            let did = anc_id_drop.clone();
                                            move |ev: ev::DragEvent| {
                                                ev.prevent_default();
                                                set_drag_over_folder_id.set(None);
                                                if let Some(s_id) = dragged_sheet_id.get_untracked() {
                                                    handle_move_sheet(s_id, Some(did.clone()));
                                                    set_dragged_sheet_id.set(None);
                                                } else if let Some(f_id) = dragged_folder_id.get_untracked() {
                                                    if f_id != did {
                                                        handle_move_folder(f_id, Some(did.clone()));
                                                    }
                                                    set_dragged_folder_id.set(None);
                                                }
                                            }
                                        }
                                    >
                                        <span class="drive-bc-icon">{anc_icon}</span>
                                        <span>{anc_name}</span>
                                    </button>
                                }.into_view()
                            }
                        }).collect_view()
                    }
                </div>

                <div class="drive-toolbar-actions">
                    <button
                        type="button"
                        class="drive-primary-btn"
                        on:click={
                            let cf_id = cur_fid_str.clone();
                            move |_| {
                                set_create_folder_parent.set(Some(cf_id.clone()));
                                set_folder_input_name.set(String::new());
                                set_folder_input_icon.set("📁".to_string());
                                set_is_create_folder_open.set(true);
                            }
                        }
                    >
                        <span class="btn-icon">"➕"</span>
                        <span>{crate::i18n::tr("drive_btn_new_subfolder", current_lang)}</span>
                    </button>
                    <button
                        type="button"
                        class="drive-action-btn btn-folder-share"
                        title=crate::i18n::tr("drive_share_folder", current_lang)
                        on:click={
                            let fs = f_for_share.clone();
                            move |_| open_share_modal(fs.clone())
                        }
                    >
                        <span class="btn-icon">"👥"</span>
                        <span class="btn-label">{crate::i18n::tr("drive_share_folder", current_lang)}</span>
                    </button>
                    <button
                        type="button"
                        class="drive-action-btn btn-folder-rename"
                        title=crate::i18n::tr("folder_modal_edit_title", current_lang)
                        on:click={
                            let fe = f_for_edit.clone();
                            move |_| {
                                set_folder_edit_name.set(fe.name.clone());
                                set_folder_edit_icon.set(fe.icon.clone());
                                set_folder_to_edit.set(Some(fe.clone()));
                            }
                        }
                    >
                        <span class="btn-icon">"✏️"</span>
                        <span class="btn-label">{crate::i18n::tr("folder_modal_edit_title", current_lang)}</span>
                    </button>
                    <button
                        type="button"
                        class="drive-action-btn btn-folder-delete"
                        title=crate::i18n::tr("folder_modal_delete_title", current_lang)
                        on:click={
                            let fd = f_for_del.clone();
                            move |_| {
                                set_folder_to_delete.set(Some(fd.clone()));
                            }
                        }
                    >
                        <span class="btn-icon">"🗑️"</span>
                        <span class="btn-label">{crate::i18n::tr("folder_modal_delete_title", current_lang)}</span>
                    </button>
                    <button
                        type="button"
                        class="drive-action-btn btn-drive-back"
                        on:click={
                            let p_id = f_parent_id.clone();
                            move |_| set_selected_folder_id.set(p_id.clone())
                        }
                    >
                        <span>{crate::i18n::tr("drive_btn_back", current_lang)}</span>
                    </button>
                </div>
            </div>

            // Subfolders Section (if any subfolders exist)
            {if has_subfolders {
                let f_counts = folder_counts.clone();
                view! {
                    <div class="drive-section">
                        <div class="drive-section-header">
                            <h3 class="drive-section-title">
                                <span>{crate::i18n::tr("drive_subfolders_title", current_lang)}</span>
                                <span class="drive-count-badge">{subfolders.len()}</span>
                            </h3>
                        </div>
                        <div class="drive-folders-grid">
                            {subfolders.into_iter().map(|f| {
                                let count = f_counts.get(&f.id).copied().unwrap_or(0);
                                let item_label = if count == 1 {
                                    crate::i18n::tr("drive_item_count_single", current_lang)
                                } else {
                                    crate::i18n::tr("drive_item_count_plural", current_lang)
                                };
                                render_single_folder_card(
                                    f,
                                    count,
                                    item_label,
                                    current_lang,
                                    dragged_sheet_id,
                                    dragged_folder_id,
                                    drag_over_folder_id,
                                    set_dragged_sheet_id,
                                    set_dragged_folder_id,
                                    set_drag_over_folder_id,
                                    set_selected_folder_id,
                                    set_folder_edit_name,
                                    set_folder_edit_icon,
                                    set_folder_to_edit,
                                    set_folder_to_delete,
                                    open_share_modal,
                                    handle_move_sheet,
                                    handle_move_folder,
                                )
                            }).collect_view()}
                        </div>
                    </div>
                }.into_view()
            } else {
                ().into_view()
            }}

            {if folder_sheets.is_empty() && !has_subfolders {
                view! {
                    <div class="drive-empty-folder-box">
                        <span class="drive-empty-icon">"📂"</span>
                        <p class="drive-empty-title">{crate::i18n::tr("folder_empty_in_folder", current_lang)}</p>
                        <p class="drive-empty-desc">{crate::i18n::tr("drive_folder_empty", current_lang)}</p>
                    </div>
                }.into_view()
            } else if !folder_sheets.is_empty() {
                view! {
                    <div class="drive-section">
                        <div class="drive-section-header">
                            <h3 class="drive-section-title">
                                <span>{crate::i18n::tr("drive_sheets_title", current_lang)}</span>
                                <span class="drive-count-badge">{sheets_count}</span>
                            </h3>
                        </div>
                        {render_character_grid(
                            folder_sheets,
                            set_sheet_to_delete,
                            set_sheet_to_move,
                            toggle_privacy,
                            folders_info,
                            current_lang,
                            dragged_sheet_id,
                            set_dragged_sheet_id,
                            set_drag_over_folder_id,
                        )}
                    </div>
                }.into_view()
            } else {
                ().into_view()
            }}
        </div>
    }.into_view()
}

pub fn render_root_drive_view<FShare, FMoveSheet, FMoveFolder, FTogglePrivacy>(
    folder_list: &[SheetFolder],
    all_sheets: Vec<CharacterSummary>,
    root_view_mode: ReadSignal<&'static str>,
    set_root_view_mode: WriteSignal<&'static str>,
    total_count: usize,
    unorganized_count: usize,
    folder_counts: &HashMap<String, usize>,
    folders_info: HashMap<String, (String, String)>,
    current_lang: crate::i18n::Language,
    ds: &DriveState,
    open_share_modal: FShare,
    handle_move_sheet: FMoveSheet,
    handle_move_folder: FMoveFolder,
    toggle_privacy: FTogglePrivacy,
) -> View
where
    FShare: Fn(SheetFolder) + Copy + 'static,
    FMoveSheet: Fn(String, Option<String>) + Copy + 'static,
    FMoveFolder: Fn(String, Option<String>) + Copy + 'static,
    FTogglePrivacy: Fn(String, bool) + Copy + 'static,
{
    let root_folders: Vec<SheetFolder> = folder_list.iter().filter(|f| f.parent_id.is_none()).cloned().collect();
    let has_folders = !root_folders.is_empty();
    let view_mode = root_view_mode.get();

    let root_sheets: Vec<CharacterSummary> = if view_mode == "all" {
        all_sheets
    } else {
        all_sheets.into_iter().filter(|s| s.folder_id.is_none()).collect()
    };
    let current_sheets_count = root_sheets.len();

    let drag_over_folder_id = ds.drag_over_folder_id;
    let dragged_sheet_id = ds.dragged_sheet_id;
    let dragged_folder_id = ds.dragged_folder_id;
    let set_drag_over_folder_id = ds.set_drag_over_folder_id;
    let set_dragged_sheet_id = ds.set_dragged_sheet_id;
    let set_dragged_folder_id = ds.set_dragged_folder_id;
    let set_selected_folder_id = ds.set_selected_folder_id;
    let set_create_folder_parent = ds.set_create_folder_parent;
    let set_folder_input_name = ds.set_folder_input_name;
    let set_folder_input_icon = ds.set_folder_input_icon;
    let set_is_create_folder_open = ds.set_is_create_folder_open;
    let set_folder_edit_name = ds.set_folder_edit_name;
    let set_folder_edit_icon = ds.set_folder_edit_icon;
    let set_folder_to_edit = ds.set_folder_to_edit;
    let set_folder_to_delete = ds.set_folder_to_delete;
    let set_sheet_to_delete = ds.set_sheet_to_delete;
    let set_sheet_to_move = ds.set_sheet_to_move;

    view! {
        <div class="drive-container">
            <div class="drive-nav-bar">
                <div class="drive-breadcrumbs">
                    <span
                        class="drive-breadcrumb-item active"
                        class:drag-over=move || drag_over_folder_id.get().as_deref() == Some("__root__")
                        on:dragover=move |ev: ev::DragEvent| {
                            ev.prevent_default();
                            if let Some(dt) = ev.data_transfer() {
                                let _ = dt.set_drop_effect("move");
                            }
                        }
                        on:dragenter=move |ev: ev::DragEvent| {
                            ev.prevent_default();
                            set_drag_over_folder_id.set(Some("__root__".to_string()));
                        }
                        on:dragleave=move |_| {
                            if drag_over_folder_id.get_untracked().as_deref() == Some("__root__") {
                                set_drag_over_folder_id.set(None);
                            }
                        }
                        on:drop=move |ev: ev::DragEvent| {
                            ev.prevent_default();
                            set_drag_over_folder_id.set(None);
                            if let Some(sheet_id) = dragged_sheet_id.get_untracked() {
                                handle_move_sheet(sheet_id, None);
                                set_dragged_sheet_id.set(None);
                            } else if let Some(folder_id) = dragged_folder_id.get_untracked() {
                                handle_move_folder(folder_id, None);
                                set_dragged_folder_id.set(None);
                            }
                        }
                    >
                        <span class="drive-bc-icon">"📁"</span>
                        <span>{crate::i18n::tr("drive_my_sheets", current_lang)}</span>
                    </span>
                </div>

                <div class="drive-toolbar-actions">
                    <button
                        type="button"
                        class="drive-primary-btn"
                        on:click=move |_| {
                            set_create_folder_parent.set(None);
                            set_folder_input_name.set(String::new());
                            set_folder_input_icon.set("📁".to_string());
                            set_is_create_folder_open.set(true);
                        }
                    >
                        <span class="btn-icon">"➕"</span>
                        <span>{crate::i18n::tr("drive_btn_new_folder", current_lang)}</span>
                    </button>
                </div>
            </div>

            // Folders Grid (if any root folders exist)
            {if has_folders {
                let f_counts = folder_counts.clone();
                view! {
                    <div class="drive-section">
                        <div class="drive-section-header">
                            <h3 class="drive-section-title">
                                <span>{crate::i18n::tr("drive_folders_title", current_lang)}</span>
                                <span class="drive-count-badge">{root_folders.len()}</span>
                            </h3>
                        </div>
                        <div class="drive-folders-grid">
                            {root_folders.into_iter().map(|f| {
                                let count = f_counts.get(&f.id).copied().unwrap_or(0);
                                let item_label = if count == 1 {
                                    crate::i18n::tr("drive_item_count_single", current_lang)
                                } else {
                                    crate::i18n::tr("drive_item_count_plural", current_lang)
                                };
                                render_single_folder_card(
                                    f,
                                    count,
                                    item_label,
                                    current_lang,
                                    dragged_sheet_id,
                                    dragged_folder_id,
                                    drag_over_folder_id,
                                    set_dragged_sheet_id,
                                    set_dragged_folder_id,
                                    set_drag_over_folder_id,
                                    set_selected_folder_id,
                                    set_folder_edit_name,
                                    set_folder_edit_icon,
                                    set_folder_to_edit,
                                    set_folder_to_delete,
                                    open_share_modal,
                                    handle_move_sheet,
                                    handle_move_folder,
                                )
                            }).collect_view()}
                        </div>
                    </div>
                }.into_view()
            } else {
                ().into_view()
            }}

            // Sheets Section
            <div class="drive-section">
                <div class="drive-section-header">
                    <h3 class="drive-section-title">
                        <span>{if view_mode == "all" {
                            crate::i18n::tr("drive_all_sheets", current_lang)
                        } else {
                            crate::i18n::tr("drive_root_sheets", current_lang)
                        }}</span>
                        <span class="drive-count-badge">{current_sheets_count}</span>
                    </h3>

                    {if has_folders {
                        view! {
                            <div class="drive-view-mode-toggle">
                                <button
                                    type="button"
                                    class="drive-toggle-btn"
                                    class:active=move || root_view_mode.get() == "unorganized"
                                    class:drag-over=move || drag_over_folder_id.get().as_deref() == Some("__root__")
                                    on:click=move |_| set_root_view_mode.set("unorganized")
                                    on:dragover=move |ev: ev::DragEvent| {
                                        ev.prevent_default();
                                        if let Some(dt) = ev.data_transfer() {
                                            let _ = dt.set_drop_effect("move");
                                        }
                                    }
                                    on:dragenter=move |ev: ev::DragEvent| {
                                        ev.prevent_default();
                                        set_drag_over_folder_id.set(Some("__root__".to_string()));
                                    }
                                    on:dragleave=move |_| {
                                        if drag_over_folder_id.get_untracked().as_deref() == Some("__root__") {
                                            set_drag_over_folder_id.set(None);
                                        }
                                    }
                                    on:drop=move |ev: ev::DragEvent| {
                                        ev.prevent_default();
                                        set_drag_over_folder_id.set(None);
                                        if let Some(sheet_id) = dragged_sheet_id.get_untracked() {
                                            handle_move_sheet(sheet_id, None);
                                            set_dragged_sheet_id.set(None);
                                        } else if let Some(folder_id) = dragged_folder_id.get_untracked() {
                                            handle_move_folder(folder_id, None);
                                            set_dragged_folder_id.set(None);
                                        }
                                    }
                                >
                                    {format!("{} ({})", crate::i18n::tr("drive_root_sheets", current_lang), unorganized_count)}
                                </button>
                                <button
                                    type="button"
                                    class="drive-toggle-btn"
                                    class:active=move || root_view_mode.get() == "all"
                                    on:click=move |_| set_root_view_mode.set("all")
                                >
                                    {format!("{} ({})", crate::i18n::tr("drive_all_sheets", current_lang), total_count)}
                                </button>
                            </div>
                        }.into_view()
                    } else {
                        ().into_view()
                    }}
                </div>

                {if root_sheets.is_empty() {
                    view! {
                        <p class="empty-msg">{crate::i18n::tr("home_empty_my", current_lang)}</p>
                    }.into_view()
                } else {
                    render_character_grid(
                        root_sheets,
                        set_sheet_to_delete,
                        set_sheet_to_move,
                        toggle_privacy,
                        folders_info,
                        current_lang,
                        dragged_sheet_id,
                        set_dragged_sheet_id,
                        set_drag_over_folder_id,
                    )
                }}
            </div>
        </div>
    }.into_view()
}
