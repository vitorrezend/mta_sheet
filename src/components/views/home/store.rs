//! Estado da tela inicial: listas carregadas, pasta aberta, arrastar e soltar,
//! janelas abertas e as ações que alteram fichas e pastas.
//!
//! Fica no contexto: cards, visões e janelas leem daqui em vez de receber dezenas de parâmetros.

use leptos::*;
use crate::i18n::Language;
use crate::state::{
    create_folder, delete_folder, delete_sheet, get_folders, get_public_sheets, get_sheets, move_folder,
    move_sheet_to_folder, set_sheet_visibility, update_folder, CharacterSummary, SheetFolder,
};

/// Alvo de soltura que representa a raiz ("Minhas fichas")
pub const ROOT_DROP: &str = "__root__";

type ListResource<S, T> = Resource<S, Result<Vec<T>, ServerFnError>>;

pub fn use_lang() -> Signal<Language> {
    let ctx = use_context::<crate::i18n::LanguageContext>();
    Signal::derive(move || ctx.map(|c| c.lang.get()).unwrap_or_default())
}

/// Conteúdo da janela de pasta: criar (sem `editing`) ou renomear.
#[derive(Clone, Debug, PartialEq)]
pub struct FolderForm {
    pub editing: Option<SheetFolder>,
    /// Pasta onde a nova pasta será criada (None = raiz)
    pub parent: Option<String>,
    pub name: String,
    pub icon: String,
}

impl FolderForm {
    pub fn create_in(parent: Option<String>) -> Self {
        Self { editing: None, parent, name: String::new(), icon: "📁".to_string() }
    }

    pub fn edit(folder: &SheetFolder) -> Self {
        Self { editing: Some(folder.clone()), parent: None, name: folder.name.clone(), icon: folder.icon.clone() }
    }
}

/// O que está sendo arrastado e sobre qual alvo.
#[derive(Clone, Copy)]
pub struct DragState {
    pub sheet: RwSignal<Option<String>>,
    pub folder: RwSignal<Option<String>>,
    pub over: RwSignal<Option<String>>,
}

impl DragState {
    pub fn is_dragging(&self) -> bool {
        self.sheet.get().is_some() || self.folder.get().is_some()
    }

    pub fn is_over(&self, target: &str) -> bool {
        self.over.with(|o| o.as_deref() == Some(target))
    }

    pub fn enter(&self, ev: &ev::DragEvent, target: &str) {
        ev.prevent_default();
        self.over.set(Some(target.to_string()));
    }

    pub fn leave(&self, target: &str) {
        if self.over.with_untracked(|o| o.as_deref() == Some(target)) {
            self.over.set(None);
        }
    }

    pub fn end(&self) {
        self.sheet.set(None);
        self.folder.set(None);
        self.over.set(None);
    }
}

/// Necessário em `dragover` para o navegador aceitar a soltura.
pub fn allow_drop(ev: ev::DragEvent) {
    ev.prevent_default();
    if let Some(dt) = ev.data_transfer() {
        dt.set_drop_effect("move");
    }
}

/// Começa a arrastar `id`, guardando-o em `slot`.
pub fn start_drag(ev: &ev::DragEvent, slot: RwSignal<Option<String>>, id: &str) {
    slot.set(Some(id.to_string()));
    if let Some(dt) = ev.data_transfer() {
        let _ = dt.set_data("text/plain", id);
        dt.set_effect_allowed("move");
    }
}

#[derive(Clone, Copy)]
pub struct HomeStore {
    pub tab: RwSignal<&'static str>,
    pub sheets: ListResource<(), CharacterSummary>,
    pub folders: ListResource<(&'static str, bool), SheetFolder>,
    pub public_sheets: ListResource<&'static str, CharacterSummary>,
    /// Pasta aberta (None = raiz)
    pub selected_folder: RwSignal<Option<String>>,
    /// Na raiz: "unorganized" (só fichas fora de pastas) ou "all"
    pub root_view_mode: RwSignal<&'static str>,
    pub error: RwSignal<Option<String>>,
    pub drag: DragState,
    pub sheet_to_delete: RwSignal<Option<CharacterSummary>>,
    pub sheet_to_move: RwSignal<Option<CharacterSummary>>,
    pub folder_form: RwSignal<Option<FolderForm>>,
    pub folder_to_delete: RwSignal<Option<SheetFolder>>,
    pub folder_to_move: RwSignal<Option<SheetFolder>>,
    pub folder_to_share: RwSignal<Option<SheetFolder>>,
    pub is_saving_folder: RwSignal<bool>,
    /// Arrastar e soltar só com mouse ou caneta: no toque o WebKit não dispara `click`
    /// em elementos arrastáveis, e os cards deixariam de abrir (mover usa os botões)
    pub can_drag: RwSignal<bool>,
}

impl HomeStore {
    pub fn new(is_logged_in: Signal<bool>) -> Self {
        let tab = create_rw_signal("my_sheets");
        // Só no navegador (efeitos não rodam no servidor): começa sem arrastar e liga depois da hidratação
        let can_drag = create_rw_signal(false);
        create_effect(move |_| {
            let fine_pointer = web_sys::window()
                .and_then(|w| w.match_media("(pointer: fine)").ok().flatten())
                .is_some_and(|m| m.matches());
            can_drag.set(fine_pointer);
        });
        Self {
            tab,
            sheets: create_local_resource(|| (), |_| get_sheets()),
            folders: create_local_resource(
                move || (tab.get(), is_logged_in.get()),
                |(tab, logged)| async move {
                    if tab == "my_sheets" && logged { get_folders().await } else { Ok(Vec::new()) }
                },
            ),
            public_sheets: create_local_resource(
                move || tab.get(),
                |tab| async move {
                    if tab == "public_sheets" { get_public_sheets().await } else { Ok(Vec::new()) }
                },
            ),
            selected_folder: create_rw_signal(None),
            root_view_mode: create_rw_signal("unorganized"),
            error: create_rw_signal(None),
            drag: DragState { sheet: create_rw_signal(None), folder: create_rw_signal(None), over: create_rw_signal(None) },
            sheet_to_delete: create_rw_signal(None),
            sheet_to_move: create_rw_signal(None),
            folder_form: create_rw_signal(None),
            folder_to_delete: create_rw_signal(None),
            folder_to_move: create_rw_signal(None),
            folder_to_share: create_rw_signal(None),
            is_saving_folder: create_rw_signal(false),
            can_drag,
        }
    }

    pub fn folder_list(&self) -> Vec<SheetFolder> {
        self.folders.get().and_then(|r| r.ok()).unwrap_or_default()
    }

    fn fail(&self, what: &str, e: impl std::fmt::Display) {
        self.error.set(Some(format!("Erro ao {}: {}", what, e)));
    }

    fn refetch_sheets(&self) {
        self.sheets.refetch();
        if self.tab.get_untracked() == "public_sheets" {
            self.public_sheets.refetch();
        }
    }

    pub fn toggle_privacy(self, sheet_id: String, is_public: bool) {
        spawn_local(async move {
            match set_sheet_visibility(sheet_id, !is_public).await {
                Ok(_) => self.refetch_sheets(),
                Err(e) => self.fail("alterar privacidade", e),
            }
        });
    }

    pub fn confirm_delete_sheet(self) {
        let Some(target) = self.sheet_to_delete.get_untracked() else { return };
        self.sheet_to_delete.set(None);
        self.error.set(None);
        spawn_local(async move {
            match delete_sheet(target.id).await {
                Ok(_) => {
                    self.refetch_sheets();
                    self.folders.refetch();
                }
                Err(e) => self.fail("excluir ficha", e),
            }
        });
    }

    pub fn move_sheet(self, sheet_id: String, target: Option<String>) {
        spawn_local(async move {
            match move_sheet_to_folder(sheet_id, target).await {
                Ok(_) => {
                    self.sheets.refetch();
                    self.folders.refetch();
                }
                Err(e) => self.fail("mover ficha", e),
            }
            self.sheet_to_move.set(None);
        });
    }

    pub fn move_folder(self, folder_id: String, target: Option<String>) {
        self.folder_to_move.set(None);
        spawn_local(async move {
            match move_folder(folder_id, target).await {
                Ok(_) => self.folders.refetch(),
                Err(e) => self.fail("mover pasta", e),
            }
        });
    }

    /// Solta o que está sendo arrastado em `target` (None = raiz).
    pub fn drop_on(self, ev: &ev::DragEvent, target: Option<String>) {
        ev.prevent_default();
        self.drag.over.set(None);
        if let Some(sheet_id) = self.drag.sheet.get_untracked() {
            self.move_sheet(sheet_id, target);
            self.drag.sheet.set(None);
        } else if let Some(folder_id) = self.drag.folder.get_untracked() {
            // Uma pasta não pode ser solta nela mesma
            if target.as_deref() != Some(folder_id.as_str()) {
                self.move_folder(folder_id, target);
            }
            self.drag.folder.set(None);
        }
    }

    /// Cria ou renomeia a pasta da janela aberta.
    pub fn save_folder_form(self) {
        let Some(form) = self.folder_form.get_untracked() else { return };
        let name = form.name.trim().to_string();
        if name.is_empty() || self.is_saving_folder.get_untracked() {
            return;
        }
        self.is_saving_folder.set(true);
        let what = if form.editing.is_some() { "atualizar pasta" } else { "criar pasta" };
        spawn_local(async move {
            let result = match form.editing {
                Some(folder) => update_folder(folder.id, name, Some(form.icon), None).await.map(|_| true),
                None => create_folder(name, Some(form.icon), None, form.parent).await.map(|_| false),
            };
            match result {
                Ok(renamed) => {
                    self.folders.refetch();
                    // A pasta aparece nos cards das fichas: renomear exige recarregá-las
                    if renamed {
                        self.sheets.refetch();
                    }
                    self.folder_form.set(None);
                }
                Err(e) => self.fail(what, e),
            }
            self.is_saving_folder.set(false);
        });
    }

    pub fn confirm_delete_folder(self) {
        let Some(target) = self.folder_to_delete.get_untracked() else { return };
        spawn_local(async move {
            let id = target.id.clone();
            match delete_folder(target.id).await {
                Ok(_) => {
                    self.folders.refetch();
                    self.sheets.refetch();
                    if self.selected_folder.get_untracked().as_deref() == Some(id.as_str()) {
                        self.selected_folder.set(None);
                    }
                }
                Err(e) => self.fail("excluir pasta", e),
            }
            self.folder_to_delete.set(None);
        });
    }
}

pub fn use_home() -> HomeStore {
    expect_context::<HomeStore>()
}
