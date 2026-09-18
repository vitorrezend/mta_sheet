use leptos::*;
use crate::state::SheetFolder;

pub fn render_single_folder_card<FShare, FMoveSheet, FMoveFolder>(
    f: SheetFolder,
    count: usize,
    item_label: &'static str,
    current_lang: crate::i18n::Language,
    dragged_sheet_id: ReadSignal<Option<String>>,
    dragged_folder_id: ReadSignal<Option<String>>,
    drag_over_folder_id: ReadSignal<Option<String>>,
    set_dragged_sheet_id: WriteSignal<Option<String>>,
    set_dragged_folder_id: WriteSignal<Option<String>>,
    set_drag_over_folder_id: WriteSignal<Option<String>>,
    set_selected_folder_id: WriteSignal<Option<String>>,
    set_folder_edit_name: WriteSignal<String>,
    set_folder_edit_icon: WriteSignal<String>,
    set_folder_to_edit: WriteSignal<Option<SheetFolder>>,
    set_folder_to_delete: WriteSignal<Option<SheetFolder>>,
    open_share_modal: FShare,
    handle_move_sheet: FMoveSheet,
    handle_move_folder: FMoveFolder,
) -> View
where
    FShare: Fn(SheetFolder) + Copy + 'static,
    FMoveSheet: Fn(String, Option<String>) + Copy + 'static,
    FMoveFolder: Fn(String, Option<String>) + Copy + 'static,
{
    let fid = f.id.clone();
    let f_for_edit = f.clone();
    let f_for_del = f.clone();
    let f_for_share = f.clone();
    let fid_drag_chk = fid.clone();
    let fid_over_chk = fid.clone();
    let fid_target_chk = fid.clone();
    let fid_drag = fid.clone();
    let fid_enter = fid.clone();
    let fid_leave = fid.clone();
    let fid_drop = fid.clone();

    let is_dragging = move || dragged_folder_id.get().as_deref() == Some(fid_drag_chk.as_str());
    let is_drag_over = move || drag_over_folder_id.get().as_deref() == Some(fid_over_chk.as_str());
    let is_targetable = move || dragged_sheet_id.get().is_some() || (dragged_folder_id.get().is_some() && dragged_folder_id.get().as_deref() != Some(fid_target_chk.as_str()));

    view! {
        <div
            class="drive-folder-card"
            class:drag-over=is_drag_over
            class:drag-targetable=is_targetable
            class:is-dragging=is_dragging
            draggable="true"
            on:dragstart={
                let fid_d = fid_drag.clone();
                move |ev: ev::DragEvent| {
                    set_dragged_folder_id.set(Some(fid_d.clone()));
                    if let Some(dt) = ev.data_transfer() {
                        let _ = dt.set_data("text/plain", &fid_d);
                        let _ = dt.set_effect_allowed("move");
                    }
                }
            }
            on:dragend=move |_| {
                set_dragged_folder_id.set(None);
                set_drag_over_folder_id.set(None);
            }
            on:click=move |_| {
                if dragged_folder_id.get_untracked().is_some() || dragged_sheet_id.get_untracked().is_some() { return; }
                set_selected_folder_id.set(Some(fid.clone()))
            }
            on:dragover=move |ev: ev::DragEvent| {
                ev.prevent_default();
                if let Some(dt) = ev.data_transfer() {
                    let _ = dt.set_drop_effect("move");
                }
            }
            on:dragenter={
                let fid_e = fid_enter.clone();
                move |ev: ev::DragEvent| {
                    ev.prevent_default();
                    set_drag_over_folder_id.set(Some(fid_e.clone()));
                }
            }
            on:dragleave={
                let fid_l = fid_leave.clone();
                move |_| {
                    if drag_over_folder_id.get_untracked().as_deref() == Some(fid_l.as_str()) {
                        set_drag_over_folder_id.set(None);
                    }
                }
            }
            on:drop={
                let fid_dp = fid_drop.clone();
                move |ev: ev::DragEvent| {
                    ev.prevent_default();
                    set_drag_over_folder_id.set(None);
                    if let Some(sheet_id) = dragged_sheet_id.get_untracked() {
                        handle_move_sheet(sheet_id, Some(fid_dp.clone()));
                        set_dragged_sheet_id.set(None);
                    } else if let Some(folder_id) = dragged_folder_id.get_untracked() {
                        if folder_id != fid_dp {
                            handle_move_folder(folder_id, Some(fid_dp.clone()));
                        }
                        set_dragged_folder_id.set(None);
                    }
                }
            }
        >
            <div class="drive-folder-icon-badge">
                <span class="drive-folder-emoji">{f.icon.clone()}</span>
            </div>
            <div class="drive-folder-info">
                <span class="drive-folder-name" title=f.name.clone()>{f.name.clone()}</span>
                <span class="drive-folder-count">{format!("{} {}", count, item_label)}</span>
            </div>
            <div class="drive-folder-menu-actions">
                <button
                    type="button"
                    class="drive-folder-menu-btn"
                    title=crate::i18n::tr("drive_share_folder", current_lang)
                    on:click={
                        let fsh = f_for_share.clone();
                        move |ev: ev::MouseEvent| {
                            ev.stop_propagation();
                            open_share_modal(fsh.clone());
                        }
                    }
                >
                    "👥"
                </button>
                <button
                    type="button"
                    class="drive-folder-menu-btn"
                    title=crate::i18n::tr("folder_modal_edit_title", current_lang)
                    on:click={
                        let fe = f_for_edit.clone();
                        move |ev: ev::MouseEvent| {
                            ev.stop_propagation();
                            set_folder_edit_name.set(fe.name.clone());
                            set_folder_edit_icon.set(fe.icon.clone());
                            set_folder_to_edit.set(Some(fe.clone()));
                        }
                    }
                >
                    "✏️"
                </button>
                <button
                    type="button"
                    class="drive-folder-menu-btn"
                    title=crate::i18n::tr("folder_modal_delete_title", current_lang)
                    on:click={
                        let fd = f_for_del.clone();
                        move |ev: ev::MouseEvent| {
                            ev.stop_propagation();
                            set_folder_to_delete.set(Some(fd.clone()));
                        }
                    }
                >
                    "🗑️"
                </button>
            </div>
        </div>
    }.into_view()
}
