//! Visões do "drive" de fichas: a raiz e o conteúdo de uma pasta.

use leptos::*;
use std::collections::HashMap;
use crate::i18n::{tr, Language};
use crate::state::{CharacterSummary, SheetFolder};
use super::folder_card::FolderCard;
use super::sheet_card::{folders_info, CharacterGrid};
use super::store::{allow_drop, use_home, FolderForm, ROOT_DROP};

/// Quantas fichas há em cada pasta.
fn folder_counts(sheets: &[CharacterSummary]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for fid in sheets.iter().filter_map(|s| s.folder_id.clone()) {
        *counts.entry(fid).or_insert(0) += 1;
    }
    counts
}

/// Caminho da raiz até a pasta `fid` (inclusive).
fn ancestry(fid: &str, folders: &[SheetFolder]) -> Vec<SheetFolder> {
    let mut path = Vec::new();
    let mut current = Some(fid.to_string());
    while let Some(id) = current {
        let Some(folder) = folders.iter().find(|f| f.id == id) else { break };
        path.push(folder.clone());
        current = folder.parent_id.clone();
    }
    path.reverse();
    path
}

#[component]
fn DriveSectionTitle(title: &'static str, count: usize) -> impl IntoView {
    view! {
        <h3 class="drive-section-title">
            <span>{title}</span>
            <span class="drive-count-badge">{count}</span>
        </h3>
    }
}

/// Grade de pastas com o contador de fichas de cada uma.
#[component]
fn FolderGrid(title: &'static str, folders: Vec<SheetFolder>, counts: HashMap<String, usize>, lang: Language) -> impl IntoView {
    view! {
        <div class="drive-section">
            <div class="drive-section-header">
                <DriveSectionTitle title=title count=folders.len() />
            </div>
            <div class="drive-folders-grid">
                {folders.into_iter().map(|f| {
                    let count = counts.get(&f.id).copied().unwrap_or(0);
                    view! { <FolderCard folder=f count=count lang=lang /> }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Item da trilha de navegação que também aceita fichas e pastas soltas.
#[component]
fn BreadcrumbDrop(target: Option<String>, icon: String, name: String, title: Option<&'static str>) -> impl IntoView {
    let home = use_home();
    let drag = home.drag;
    let key = store_value(target.clone().unwrap_or_else(|| ROOT_DROP.to_string()));
    let target = store_value(target);
    view! {
        <button
            type="button"
            class="drive-breadcrumb-item clickable"
            class:drag-over=move || key.with_value(|k| drag.is_over(k))
            class:drag-targetable=move || drag.is_dragging()
            on:click=move |_| home.selected_folder.set(target.get_value())
            on:dragover=allow_drop
            on:dragenter=move |ev: ev::DragEvent| key.with_value(|k| drag.enter(&ev, k))
            on:dragleave=move |_| key.with_value(|k| drag.leave(k))
            on:drop=move |ev: ev::DragEvent| home.drop_on(&ev, target.get_value())
            title=title
        >
            <span class="drive-bc-icon">{icon}</span>
            <span>{name}</span>
        </button>
    }
}

/// Conteúdo de uma pasta: trilha, ações da pasta, subpastas e fichas.
#[component]
pub fn FolderView(fid: String, folders: Vec<SheetFolder>, sheets: Vec<CharacterSummary>, lang: Language) -> impl IntoView {
    let home = use_home();
    let Some(current) = folders.iter().find(|f| f.id == fid).cloned() else {
        // A pasta sumiu (excluída em outra aba): volta para a raiz
        home.selected_folder.set(None);
        return view! { <div class="drive-container"></div> }.into_view();
    };
    let counts = folder_counts(&sheets);
    let info = folders_info(&folders);
    let folder_sheets: Vec<CharacterSummary> = sheets.into_iter().filter(|s| s.folder_id.as_deref() == Some(fid.as_str())).collect();
    let subfolders: Vec<SheetFolder> = folders.iter().filter(|f| f.parent_id.as_deref() == Some(fid.as_str())).cloned().collect();
    let mut path = ancestry(&fid, &folders);
    path.pop();

    let folder = store_value(current.clone());
    let parent_id = current.parent_id.clone();
    let is_empty = folder_sheets.is_empty() && subfolders.is_empty();

    view! {
        <div class="drive-container">
            <div class="drive-nav-bar">
                <div class="drive-breadcrumbs">
                    <BreadcrumbDrop target=None icon="📁".into() name=tr("drive_my_sheets", lang).into() title=Some(tr("drive_move_to_root", lang)) />
                    {path.into_iter().map(|f| view! {
                        <span class="drive-breadcrumb-sep">"/"</span>
                        <BreadcrumbDrop target=Some(f.id.clone()) icon=f.icon name=f.name title=None />
                    }).collect_view()}
                    <span class="drive-breadcrumb-sep">"/"</span>
                    <span class="drive-breadcrumb-item active">
                        <span class="drive-bc-icon">{current.icon.clone()}</span>
                        <span>{current.name.clone()}</span>
                    </span>
                </div>

                <div class="drive-toolbar-actions">
                    <button
                        type="button"
                        class="drive-primary-btn"
                        on:click=move |_| home.folder_form.set(Some(FolderForm::create_in(Some(folder.with_value(|f| f.id.clone())))))
                    >
                        <span class="btn-icon">"➕"</span>
                        <span>{tr("drive_btn_new_subfolder", lang)}</span>
                    </button>
                    <button
                        type="button"
                        class="drive-action-btn btn-folder-share"
                        title=tr("drive_share_folder", lang)
                        on:click=move |_| home.folder_to_share.set(Some(folder.get_value()))
                    >
                        <span class="btn-icon">"👥"</span>
                        <span class="btn-label">{tr("drive_share_folder", lang)}</span>
                    </button>
                    <button
                        type="button"
                        class="drive-action-btn btn-folder-rename"
                        title=tr("folder_modal_edit_title", lang)
                        on:click=move |_| home.folder_form.set(Some(folder.with_value(FolderForm::edit)))
                    >
                        <span class="btn-icon">"✏️"</span>
                        <span class="btn-label">{tr("folder_modal_edit_title", lang)}</span>
                    </button>
                    <button
                        type="button"
                        class="drive-action-btn btn-folder-delete"
                        title=tr("folder_modal_delete_title", lang)
                        on:click=move |_| home.folder_to_delete.set(Some(folder.get_value()))
                    >
                        <span class="btn-icon">"🗑️"</span>
                        <span class="btn-label">{tr("folder_modal_delete_title", lang)}</span>
                    </button>
                    <button type="button" class="drive-action-btn btn-drive-back" on:click=move |_| home.selected_folder.set(parent_id.clone())>
                        <span>{tr("drive_btn_back", lang)}</span>
                    </button>
                </div>
            </div>

            {(!subfolders.is_empty()).then(|| view! {
                <FolderGrid title=tr("drive_subfolders_title", lang) folders=subfolders counts=counts lang=lang />
            })}

            {if is_empty {
                view! {
                    <div class="drive-empty-folder-box">
                        <span class="drive-empty-icon">"📂"</span>
                        <p class="drive-empty-title">{tr("folder_empty_in_folder", lang)}</p>
                        <p class="drive-empty-desc">{tr("drive_folder_empty", lang)}</p>
                    </div>
                }.into_view()
            } else if !folder_sheets.is_empty() {
                view! {
                    <div class="drive-section">
                        <div class="drive-section-header">
                            <DriveSectionTitle title=tr("drive_sheets_title", lang) count=folder_sheets.len() />
                        </div>
                        <CharacterGrid sheets=folder_sheets folders_info=info lang=lang />
                    </div>
                }.into_view()
            } else {
                ().into_view()
            }}
        </div>
    }
    .into_view()
}

/// Raiz do drive: pastas de primeiro nível e as fichas fora de pastas (ou todas).
#[component]
pub fn RootDriveView(
    folders: Vec<SheetFolder>,
    sheets: Vec<CharacterSummary>,
    /// Mostrar todas as fichas, não só as que estão fora de pastas
    show_all: bool,
    lang: Language,
) -> impl IntoView {
    let home = use_home();
    let drag = home.drag;
    let mode = home.root_view_mode;
    let counts = folder_counts(&sheets);
    let info = folders_info(&folders);
    let total_count = sheets.len();
    let unorganized_count = sheets.iter().filter(|s| s.folder_id.is_none()).count();
    let root_folders: Vec<SheetFolder> = folders.into_iter().filter(|f| f.parent_id.is_none()).collect();
    let has_folders = !root_folders.is_empty();
    let root_sheets: Vec<CharacterSummary> = if show_all { sheets } else { sheets.into_iter().filter(|s| s.folder_id.is_none()).collect() };

    // A raiz também é alvo de soltura: tira fichas e pastas de dentro de pastas
    let root_over = move || drag.is_over(ROOT_DROP);
    let root_enter = move |ev: ev::DragEvent| drag.enter(&ev, ROOT_DROP);
    let root_leave = move |_| drag.leave(ROOT_DROP);
    let root_drop = move |ev: ev::DragEvent| home.drop_on(&ev, None);

    view! {
        <div class="drive-container">
            <div class="drive-nav-bar">
                <div class="drive-breadcrumbs">
                    <span
                        class="drive-breadcrumb-item active"
                        class:drag-over=root_over
                        on:dragover=allow_drop
                        on:dragenter=root_enter
                        on:dragleave=root_leave
                        on:drop=root_drop
                    >
                        <span class="drive-bc-icon">"📁"</span>
                        <span>{tr("drive_my_sheets", lang)}</span>
                    </span>
                </div>

                <div class="drive-toolbar-actions">
                    <button type="button" class="drive-primary-btn" on:click=move |_| home.folder_form.set(Some(FolderForm::create_in(None)))>
                        <span class="btn-icon">"➕"</span>
                        <span>{tr("drive_btn_new_folder", lang)}</span>
                    </button>
                </div>
            </div>

            {has_folders.then(|| view! {
                <FolderGrid title=tr("drive_folders_title", lang) folders=root_folders counts=counts lang=lang />
            })}

            <div class="drive-section">
                <div class="drive-section-header">
                    <DriveSectionTitle
                        title=tr(if show_all { "drive_all_sheets" } else { "drive_root_sheets" }, lang)
                        count=root_sheets.len()
                    />
                    {has_folders.then(|| view! {
                        <div class="drive-view-mode-toggle">
                            <button
                                type="button"
                                class="drive-toggle-btn"
                                class:active=move || mode.get() == "unorganized"
                                class:drag-over=root_over
                                on:click=move |_| mode.set("unorganized")
                                on:dragover=allow_drop
                                on:dragenter=root_enter
                                on:dragleave=root_leave
                                on:drop=root_drop
                            >
                                {format!("{} ({})", tr("drive_root_sheets", lang), unorganized_count)}
                            </button>
                            <button type="button" class="drive-toggle-btn" class:active=move || mode.get() == "all" on:click=move |_| mode.set("all")>
                                {format!("{} ({})", tr("drive_all_sheets", lang), total_count)}
                            </button>
                        </div>
                    })}
                </div>

                {if root_sheets.is_empty() {
                    view! { <p class="empty-msg">{tr("home_empty_my", lang)}</p> }.into_view()
                } else {
                    view! { <CharacterGrid sheets=root_sheets folders_info=info lang=lang /> }.into_view()
                }}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn folder(id: &str, parent: Option<&str>) -> SheetFolder {
        SheetFolder { id: id.into(), name: id.to_uppercase(), parent_id: parent.map(Into::into), ..Default::default() }
    }

    #[test]
    fn ancestry_walks_from_root_to_the_folder() {
        let folders = vec![folder("a", None), folder("b", Some("a")), folder("c", Some("b"))];
        let ids: Vec<String> = ancestry("c", &folders).into_iter().map(|f| f.id).collect();
        assert_eq!(ids, ["a", "b", "c"]);
        assert!(ancestry("missing", &folders).is_empty());
    }
}
