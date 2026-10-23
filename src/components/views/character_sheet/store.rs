//! Estado da ficha aberta: dados, status de gravação e as regras de salvamento
//! (automático com espera de 1,2 s, periódico a cada 30 s, manual e ao sair da página).

use leptos::*;
use crate::components::mta_sheet::sheet::SaveStatus;
use crate::state::CharacterData;
use super::super::sheet_saver::SheetSaver;

/// Espera depois da última edição antes de salvar sozinho
const DEBOUNCE_MS: u32 = 1_200;
/// Redundância: salva o que estiver pendente a cada 30 s
const PERIODIC_MS: u32 = 30_000;

fn saved_now() -> SaveStatus {
    SaveStatus::Saved("agora")
}

fn log_info(channel: &str, message: &str, id: &str) {
    crate::logging::log_client(channel, "INFO", message, Some(&format!("id={}", id)));
}

/// Tudo que a tela da ficha precisa para ler, alterar e gravar os dados.
///
/// É `Copy`: cada ação pode capturar o store sem clones. Depois que a tela é
/// desmontada os sinais deixam de existir, e as ações viram operações vazias.
#[derive(Clone, Copy)]
pub struct SheetStore {
    pub data: ReadSignal<CharacterData>,
    pub set_data: WriteSignal<CharacterData>,
    pub status: RwSignal<SaveStatus>,
    pub is_dirty: RwSignal<bool>,
    pub is_loaded: RwSignal<bool>,
    /// A ficha mudou em outro lugar: as gravações ficam pausadas até o usuário decidir
    pub conflict: RwSignal<bool>,
    saver: StoredValue<SheetSaver>,
    /// Número da última edição; a gravação atrasada só vale se nenhuma outra veio depois
    change_seq: StoredValue<u64>,
    id: Signal<String>,
    /// Id da ficha carregada. A URL já mudou quando a tela é desmontada; a gravação
    /// final precisa saber qual ficha estava aberta.
    loaded_id: StoredValue<String>,
    token: Signal<Option<String>>,
}

impl SheetStore {
    pub fn new(id: Signal<String>, token: Signal<Option<String>>) -> Self {
        let (data, set_data) = create_signal(CharacterData::default());
        let saver = SheetSaver::new(data, set_data);
        Self {
            data,
            set_data,
            status: create_rw_signal(SaveStatus::Idle),
            is_dirty: create_rw_signal(false),
            is_loaded: create_rw_signal(false),
            conflict: saver.conflict,
            saver: store_value(saver),
            change_seq: store_value(0),
            id,
            loaded_id: store_value(String::new()),
            token,
        }
    }

    pub fn id(&self) -> String {
        match self.loaded_id.try_get_value().filter(|id| !id.is_empty()) {
            Some(id) => id,
            None => self.id.try_get_untracked().unwrap_or_default(),
        }
    }

    pub fn token(&self) -> Option<String> {
        self.token.try_get_untracked().flatten()
    }

    fn can_edit(&self) -> bool {
        self.data.try_with_untracked(|d| d.can_edit).unwrap_or(false)
    }

    fn set_status(&self, status: SaveStatus) {
        let _ = self.status.try_set(status);
    }

    /// Adota a ficha que veio do servidor como ponto de partida.
    pub fn loaded(&self, fetched: CharacterData) {
        if let Some(saver) = self.saver.try_get_value() {
            saver.loaded(&fetched);
        }
        self.loaded_id.set_value(fetched.id.clone());
        let _ = self.set_data.try_set(fetched);
        // Só na primeira carga: notificar de novo remontaria todas as páginas
        if self.is_loaded.try_get_untracked() == Some(false) {
            let _ = self.is_loaded.try_set(true);
        }
        let _ = self.is_dirty.try_set(false);
        self.set_status(saved_now());
        log_info("user_actions", "Ficha carregada no navegador", &self.id());
    }

    /// Grava agora. `still_current` diz se a edição salva ainda é a mais recente;
    /// se não for, a ficha continua marcada como alterada.
    async fn save_now(self, still_current: impl Fn() -> bool) -> Result<(), String> {
        let id = self.id();
        let Some(saver) = self.saver.try_get_value() else { return Ok(()) };
        if id.is_empty() || !self.can_edit() {
            return Ok(());
        }
        self.set_status(SaveStatus::Saving);
        let result = saver.save(id).await;
        match &result {
            Ok(()) if still_current() => {
                let _ = self.is_dirty.try_set(false);
                self.set_status(saved_now());
            }
            Ok(()) => {}
            Err(e) => self.set_status(SaveStatus::Error(e.clone())),
        }
        result
    }

    /// Salvamento pedido pelo usuário (botão ou importação).
    pub fn save(self, reason: &'static str) {
        spawn_local(async move {
            match self.save_now(|| true).await {
                Ok(()) => log_info("user_actions", reason, &self.id()),
                Err(e) => crate::logging::log_client("errors", "ERROR", "Falha ao salvar a ficha", Some(&e)),
            }
        });
    }

    /// Salva o que estiver pendente e depois executa `then` (usado antes de sair da ficha).
    pub fn save_then(self, then: impl FnOnce() + 'static) {
        let pending = self.is_dirty.try_get_untracked().unwrap_or(false) && self.can_edit();
        let snapshot = self.data.try_get_untracked();
        match (pending, snapshot, self.saver.try_get_value()) {
            (true, Some(snapshot), Some(saver)) => {
                let id = self.id();
                self.set_status(SaveStatus::Saving);
                spawn_local(async move {
                    if !id.is_empty() {
                        let _ = saver.save_snapshot(id.clone(), snapshot).await;
                        log_info("user_actions", "Ficha salva antes de sair", &id);
                    }
                    then();
                });
            }
            _ => then(),
        }
    }

    /// Liga os salvamentos automáticos: após cada edição, a cada 30 s e ao desmontar a tela.
    pub fn install_autosave(self) {
        // Ao desmontar, os sinais já não existem: guarda agora o que a gravação final precisa
        let saver = self.saver.get_value();
        on_cleanup(move || {
            if !self.is_dirty.try_get_untracked().unwrap_or(false) {
                return;
            }
            let id = self.id();
            if let Some(snapshot) = self.data.try_get_untracked().filter(|d| d.can_edit)
                && !id.is_empty()
            {
                spawn_local(async move {
                    let _ = saver.save_snapshot(id.clone(), snapshot).await;
                    log_info("user_actions", "Ficha salva automaticamente ao sair da página", &id);
                });
            }
        });

        // Cada edição marca a ficha como alterada e agenda a gravação
        create_effect(move |_| {
            self.data.track();
            if !self.is_loaded.try_get_untracked().unwrap_or(false) || !self.can_edit() {
                return;
            }
            let _ = self.is_dirty.try_set(true);
            self.set_status(SaveStatus::Pending);
            let seq = self.change_seq.get_value() + 1;
            self.change_seq.set_value(seq);
            let is_latest = move || self.change_seq.try_get_value() == Some(seq);
            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(DEBOUNCE_MS).await;
                let pending = is_latest() && self.is_dirty.try_get_untracked().unwrap_or(false);
                if pending && self.save_now(is_latest).await.is_ok() && is_latest() {
                    log_info("database", "Salvamento automático executado", &self.id());
                }
            });
        });

        // Redundância periódica, iniciada uma única vez quando a ficha carrega
        create_effect(move |started: Option<bool>| {
            if started == Some(true) || !self.is_loaded.get() {
                return started.unwrap_or(false);
            }
            spawn_local(async move {
                loop {
                    gloo_timers::future::TimeoutFuture::new(PERIODIC_MS).await;
                    // Tela desmontada ou ficha só de leitura: encerra o laço
                    let Some(can_edit) = self.data.try_with_untracked(|d| d.can_edit) else { break };
                    if !can_edit {
                        break;
                    }
                    if self.is_dirty.try_get_untracked().unwrap_or(false) {
                        let _ = self.save_now(|| true).await;
                    }
                }
            });
            true
        });
    }

    /// Conflito de versão: grava por cima mantendo as alterações locais.
    pub fn keep_mine(self) {
        let Some(saver) = self.saver.try_get_value() else { return };
        let (id, token) = (self.id(), self.token());
        self.set_status(SaveStatus::Saving);
        spawn_local(async move {
            match saver.overwrite_with_mine(id, token).await {
                Ok(()) => {
                    let _ = self.is_dirty.try_set(false);
                    self.set_status(saved_now());
                }
                Err(e) => self.set_status(SaveStatus::Error(e)),
            }
        });
    }

    /// Substitui o conteúdo pela ficha importada (mantendo o id) e grava.
    pub fn import(self, mut imported: CharacterData) {
        imported.id = self.id();
        // Permissão e autoria vêm do servidor, não do arquivo
        self.data.with_untracked(|d| {
            imported.can_edit = d.can_edit;
            imported.author_username = d.author_username.clone();
        });
        self.set_data.set(imported);
        self.save("Ficha importada de arquivo JSON e salva no banco");
    }

    pub fn toggle_privacy(self) {
        let id = self.id();
        let is_public = !self.data.with_untracked(|d| d.is_public);
        self.set_data.update(|d| d.is_public = is_public);
        if !id.is_empty() {
            spawn_local(async move {
                let _ = crate::state::set_sheet_visibility(id, is_public).await;
            });
        }
    }
}
