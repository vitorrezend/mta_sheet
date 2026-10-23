use leptos::*;
use crate::i18n::{tr, Language};
use crate::state::SheetFolder;
use super::store::{allow_drop, start_drag, use_home, FolderForm, HomeStore};

/// Card de pasta: abre ao clicar, aceita fichas e pastas soltas e tem os atalhos de mover, compartilhar, renomear e excluir.
#[component]
pub fn FolderCard(folder: SheetFolder, count: usize, lang: Language) -> impl IntoView {
    let home = use_home();
    let drag = home.drag;
    let item_label = tr(if count == 1 { "drive_item_count_single" } else { "drive_item_count_plural" }, lang);
    let id = store_value(folder.id.clone());
    let folder = store_value(folder);

    let is_dragging = move || id.with_value(|id| drag.folder.with(|d| d.as_deref() == Some(id)));
    let is_drag_over = move || id.with_value(|id| drag.is_over(id));
    // Qualquer ficha pode entrar; uma pasta não pode entrar nela mesma
    let is_targetable = move || drag.sheet.get().is_some() || (drag.folder.get().is_some() && !is_dragging());

    // Os botões do card não devem abrir a pasta (sem delegação, como o card: o
    // stop_propagation só segura o evento nativo se o botão também escutar direto)
    let action = move |f: fn(HomeStore, &SheetFolder)| {
        move |ev: ev::MouseEvent| {
            ev.stop_propagation();
            folder.with_value(|folder| f(home, folder));
        }
    };
    let (name, icon) = folder.with_value(|f| (f.name.clone(), f.icon.clone()));

    view! {
        <div
            class="drive-folder-card"
            class:drag-over=is_drag_over
            class:drag-targetable=is_targetable
            class:is-dragging=is_dragging
            draggable=move || if home.can_drag.get() { "true" } else { "false" }
            on:dragstart=move |ev: ev::DragEvent| id.with_value(|id| start_drag(&ev, drag.folder, id))
            on:dragend=move |_| drag.end()
            // Sem delegação: no iPhone o toque num <div> só vira clique se o próprio elemento escuta cliques
            on:click:undelegated=move |_| {
                if drag.sheet.get_untracked().is_none() && drag.folder.get_untracked().is_none() {
                    home.selected_folder.set(Some(id.get_value()));
                }
            }
            on:dragover=allow_drop
            on:dragenter=move |ev: ev::DragEvent| id.with_value(|id| drag.enter(&ev, id))
            on:dragleave=move |_| id.with_value(|id| drag.leave(id))
            on:drop=move |ev: ev::DragEvent| home.drop_on(&ev, Some(id.get_value()))
        >
            <div class="drive-folder-icon-badge">
                <span class="drive-folder-emoji">{icon}</span>
            </div>
            <div class="drive-folder-info">
                <span class="drive-folder-name" title=name.clone()>{name}</span>
                <span class="drive-folder-count">{format!("{} {}", count, item_label)}</span>
            </div>
            <div class="drive-folder-menu-actions">
                // Alternativa ao arrastar-e-soltar, que não funciona bem em telas de toque (iPhone)
                <button
                    type="button"
                    class="drive-folder-menu-btn"
                    title="Mover pasta"
                    aria-label="Mover pasta"
                    on:click:undelegated=action(|home, f| home.folder_to_move.set(Some(f.clone())))
                >
                    "📂"
                </button>
                <button
                    type="button"
                    class="drive-folder-menu-btn"
                    title=tr("drive_share_folder", lang)
                    on:click:undelegated=action(|home, f| home.folder_to_share.set(Some(f.clone())))
                >
                    "👥"
                </button>
                <button
                    type="button"
                    class="drive-folder-menu-btn"
                    title=tr("folder_modal_edit_title", lang)
                    on:click:undelegated=action(|home, f| home.folder_form.set(Some(FolderForm::edit(f))))
                >
                    "✏️"
                </button>
                <button
                    type="button"
                    class="drive-folder-menu-btn"
                    title=tr("folder_modal_delete_title", lang)
                    on:click:undelegated=action(|home, f| home.folder_to_delete.set(Some(f.clone())))
                >
                    "🗑️"
                </button>
            </div>
        </div>
    }
}
