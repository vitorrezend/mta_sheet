use leptos::*;
use crate::components::Callback;
use crate::i18n::tr;
use crate::state::SheetFolder;
use super::store::{use_home, use_lang, FolderForm};
use super::types::{get_folder_path, FOLDER_ICON_PRESETS};

/// Janela de criar ou renomear pasta: nome e ícone.
#[component]
pub fn FolderFormModal() -> impl IntoView {
    let home = use_home();
    let lang = use_lang();
    let form = home.folder_form;
    let close = move |_| form.set(None);
    let set_field = move |f: fn(&mut FolderForm, String), value: String| {
        form.update(|o| {
            if let Some(o) = o {
                f(o, value)
            }
        })
    };
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        home.save_folder_form();
    };

    move || {
        let lang = lang.get();
        let is_edit = form.with(|f| f.as_ref().map(|f| f.editing.is_some()))?;
        Some(view! {
            <div class="modal-overlay" on:click=close>
                <div class="modal-card folder-modal-card" on:click=move |ev| ev.stop_propagation()>
                    <h3 class="modal-title">{tr(if is_edit { "folder_modal_edit_title" } else { "folder_modal_create_title" }, lang)}</h3>
                    <form on:submit=on_submit class="folder-form">
                        <div class="folder-form-field">
                            <label class="folder-field-label">{tr("folder_name_label", lang)}</label>
                            <input
                                type="text"
                                class="folder-input"
                                placeholder=tr("folder_name_ph", lang)
                                prop:value=move || form.with(|f| f.as_ref().map(|f| f.name.clone()).unwrap_or_default())
                                on:input=move |ev| set_field(|f, v| f.name = v, event_target_value(&ev))
                                required
                                autofocus
                            />
                        </div>
                        <div class="folder-form-field">
                            <label class="folder-field-label">{tr("folder_icon_label", lang)}</label>
                            <div class="folder-icon-presets">
                                {FOLDER_ICON_PRESETS.iter().map(|&icon| view! {
                                    <button
                                        type="button"
                                        class="folder-icon-preset-btn"
                                        class:active=move || form.with(|f| f.as_ref().is_some_and(|f| f.icon == icon))
                                        on:click=move |_| set_field(|f, v| f.icon = v, icon.to_string())
                                    >
                                        {icon}
                                    </button>
                                }).collect_view()}
                            </div>
                        </div>
                        <div class="modal-actions">
                            <button type="button" class="modal-btn btn-cancel" on:click=close>
                                {tr("home_btn_cancel", lang)}
                            </button>
                            <button type="submit" class="modal-btn btn-primary" disabled=move || home.is_saving_folder.get()>
                                {tr(if is_edit { "folder_save" } else { "folder_create" }, lang)}
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        })
    }
}

#[component]
pub fn DeleteFolderModal() -> impl IntoView {
    let home = use_home();
    let lang = use_lang();
    let close = move |_| home.folder_to_delete.set(None);
    move || {
        let lang = lang.get();
        home.folder_to_delete.get().map(|target| view! {
            <div class="modal-overlay" on:click=close>
                <div class="modal-card" on:click=move |ev| ev.stop_propagation()>
                    <h3 class="modal-title">{tr("folder_modal_delete_title", lang)}</h3>
                    <p class="modal-text">
                        {tr("home_delete_prompt", lang)}
                        <strong>{target.name}</strong>"?"
                    </p>
                    <p class="modal-subtext">{tr("folder_modal_delete_desc", lang)}</p>
                    <div class="modal-actions">
                        <button type="button" class="modal-btn btn-cancel" on:click=close>
                            {tr("home_btn_cancel", lang)}
                        </button>
                        <button type="button" class="modal-btn btn-danger" on:click=move |_| home.confirm_delete_folder()>
                            {tr("home_btn_confirm_delete", lang)}
                        </button>
                    </div>
                </div>
            </div>
        })
    }
}

/// Opção de destino nas janelas de mover.
#[component]
fn MoveOption(icon: String, name: String, is_current: bool, on_pick: Callback<()>) -> impl IntoView {
    view! {
        <button type="button" class="move-folder-opt-btn" class:active=is_current on:click=move |_| on_pick.call(())>
            <span class="move-opt-icon">{icon}</span>
            <span class="move-opt-name">{name}</span>
            {is_current.then(|| view! { <span class="move-opt-curr">"✓ Atual"</span> })}
        </button>
    }
}

/// Janela de mover: título, lista de destinos (a raiz primeiro) e cancelar.
#[component]
fn MoveModal(title: String, item_name: String, lang: crate::i18n::Language, on_close: Callback<()>, children: Children) -> impl IntoView {
    let close = on_close.clone();
    view! {
        <div class="modal-overlay" on:click=move |_| close.call(())>
            <div class="modal-card folder-modal-card" on:click=move |ev| ev.stop_propagation()>
                <h3 class="modal-title">{title}</h3>
                <p class="modal-text">"Mover "<strong>{item_name}</strong>" para:"</p>
                <div class="move-folder-options-list">{children()}</div>
                <div class="modal-actions">
                    <button type="button" class="modal-btn btn-cancel" on:click=move |_| on_close.call(())>
                        {tr("home_btn_cancel", lang)}
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Destinos possíveis de uma janela de mover: a raiz e cada pasta, com o caminho completo.
fn move_options(
    folders: &[SheetFolder],
    destinations: Vec<SheetFolder>,
    current: Option<String>,
    lang: crate::i18n::Language,
    pick: impl Fn(Option<String>) -> Callback<()>,
) -> View {
    let root = view! {
        <MoveOption icon="📁".into() name=tr("drive_move_to_root", lang).into() is_current=current.is_none() on_pick=pick(None) />
    };
    let others = destinations.into_iter().map(|f| {
        let is_current = current.as_deref() == Some(f.id.as_str());
        view! { <MoveOption icon=f.icon.clone() name=get_folder_path(&f.id, folders) is_current=is_current on_pick=pick(Some(f.id)) /> }
    });
    std::iter::once(root.into_view()).chain(others.map(IntoView::into_view)).collect_view()
}

#[component]
pub fn MoveSheetModal() -> impl IntoView {
    let home = use_home();
    let lang = use_lang();
    move || {
        let lang = lang.get();
        let target = home.sheet_to_move.get()?;
        let folders = home.folder_list();
        let sheet_id = store_value(target.id.clone());
        let pick = move |dest: Option<String>| Callback::new(move |_| home.move_sheet(sheet_id.get_value(), dest.clone()));
        let options = move_options(&folders, folders.clone(), target.folder_id.clone(), lang, pick);
        Some(view! {
            <MoveModal
                title=tr("folder_modal_move_title", lang).to_string()
                item_name=target.name.clone()
                lang=lang
                on_close=Callback::new(move |_| home.sheet_to_move.set(None))
            >
                {options.clone()}
            </MoveModal>
        })
    }
}

/// A pasta `candidate` é `folder_id` ou está dentro dela (mover para lá criaria um ciclo).
fn is_same_or_descendant(candidate: &str, folder_id: &str, all: &[SheetFolder]) -> bool {
    let mut current = Some(candidate.to_string());
    while let Some(id) = current {
        if id == folder_id {
            return true;
        }
        current = all.iter().find(|f| f.id == id).and_then(|f| f.parent_id.clone());
    }
    false
}

#[component]
pub fn MoveFolderModal() -> impl IntoView {
    let home = use_home();
    let lang = use_lang();
    move || {
        let lang = lang.get();
        let target = home.folder_to_move.get()?;
        let folders = home.folder_list();
        let folder_id = store_value(target.id.clone());
        let pick = move |dest: Option<String>| Callback::new(move |_| home.move_folder(folder_id.get_value(), dest.clone()));
        // Não dá para mover uma pasta para dentro dela mesma ou de uma subpasta dela
        let destinations = folders.iter().filter(|f| !is_same_or_descendant(&f.id, &target.id, &folders)).cloned().collect();
        let options = move_options(&folders, destinations, target.parent_id.clone(), lang, pick);
        Some(view! {
            <MoveModal
                title="Mover pasta".to_string()
                item_name=target.name.clone()
                lang=lang
                on_close=Callback::new(move |_| home.folder_to_move.set(None))
            >
                {options.clone()}
            </MoveModal>
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_folder_cannot_move_into_its_own_subtree() {
        let folder = |id: &str, parent: Option<&str>| SheetFolder { id: id.into(), parent_id: parent.map(Into::into), ..Default::default() };
        let all = vec![folder("a", None), folder("b", Some("a")), folder("c", None)];
        assert!(is_same_or_descendant("a", "a", &all));
        assert!(is_same_or_descendant("b", "a", &all));
        assert!(!is_same_or_descendant("c", "a", &all));
    }
}
