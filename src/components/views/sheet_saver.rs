//! Salvamento da ficha com controle de versão (concorrência otimista) e envio parcial.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use leptos::*;
use crate::state::json_patch::merge_diff;
use crate::state::{patch_sheet, update_sheet, CharacterData, SHEET_CONFLICT_ERROR};

/// Campos definidos pelo servidor a cada leitura; não fazem parte do conteúdo da ficha.
const SERVER_OWNED_FIELDS: [&str; 3] = ["version", "can_edit", "author_username"];

/// Conteúdo da ficha em JSON, sem os campos controlados pelo servidor (base dos diffs).
fn content_json(data: &CharacterData) -> serde_json::Value {
    let mut value = serde_json::to_value(data).unwrap_or(serde_json::Value::Null);
    if let Some(map) = value.as_object_mut() {
        for field in SERVER_OWNED_FIELDS {
            map.remove(field);
        }
    }
    value
}

/// Grava a ficha no servidor, uma gravação por vez, mantendo a versão local em dia.
///
/// Salvamento automático, periódico e manual podem disparar juntos; se duas gravações
/// da mesma aba saíssem com a mesma versão, a segunda seria recusada como conflito.
/// Por isso elas fazem fila, e cada uma lê os dados (e a versão) mais recentes.
///
/// Normalmente só o que mudou desde a última versão confirmada é enviado (JSON Merge
/// Patch): marcar uma bolinha manda poucos bytes em vez da ficha inteira.
#[derive(Clone)]
pub struct SheetSaver {
    pub in_flight: Rc<Cell<bool>>,
    /// Última versão confirmada pelo servidor. Fica fora dos sinais para funcionar
    /// também no salvamento ao sair da página, quando os sinais já foram descartados.
    pub version: Rc<Cell<i64>>,
    /// Conteúdo correspondente a `version`, base para calcular o que mudou
    base: Rc<RefCell<Option<serde_json::Value>>>,
    pub data: ReadSignal<CharacterData>,
    pub set_data: WriteSignal<CharacterData>,
    /// A ficha foi alterada em outro lugar: salvamentos pausados até o usuário decidir
    pub conflict: RwSignal<bool>,
}

impl SheetSaver {
    pub fn new(data: ReadSignal<CharacterData>, set_data: WriteSignal<CharacterData>) -> Self {
        Self {
            in_flight: Rc::new(Cell::new(false)),
            version: Rc::new(Cell::new(0)),
            base: Rc::new(RefCell::new(None)),
            data,
            set_data,
            conflict: create_rw_signal(false),
        }
    }

    /// Registra a ficha recém-carregada do servidor como ponto de partida.
    pub fn loaded(&self, data: &CharacterData) {
        self.version.set(data.version);
        *self.base.borrow_mut() = Some(content_json(data));
    }

    /// Salva o estado atual da ficha.
    pub async fn save(&self, id: String) -> Result<(), String> {
        self.save_inner(id, None, false).await
    }

    /// Salva um retrato já capturado (usado ao sair da página).
    pub async fn save_snapshot(&self, id: String, snapshot: CharacterData) -> Result<(), String> {
        self.save_inner(id, Some(snapshot), false).await
    }

    /// Resolve um conflito mantendo as alterações locais: adota a versão atual do banco
    /// e grava a ficha inteira por cima.
    pub async fn overwrite_with_mine(&self, id: String, token: Option<String>) -> Result<(), String> {
        let latest = crate::state::get_sheet(id.clone(), token).await.map_err(|e| e.to_string())?;
        self.version.set(latest.version);
        self.set_data.try_update_untracked(|d| d.version = latest.version);
        let _ = self.conflict.try_set(false);
        self.save_inner(id, None, true).await
    }

    async fn save_inner(&self, id: String, snapshot: Option<CharacterData>, full: bool) -> Result<(), String> {
        while self.in_flight.get() {
            gloo_timers::future::TimeoutFuture::new(40).await;
        }
        if self.conflict.try_get_untracked().unwrap_or(false) {
            return Err("Ficha alterada em outro lugar: escolha recarregar ou manter suas alterações".to_string());
        }
        // Lê os dados só depois da fila: assim pega as edições feitas enquanto esperava
        let Some(mut snapshot) = snapshot.or_else(|| self.data.try_get_untracked()) else {
            return Err("Ficha indisponível".to_string());
        };
        snapshot.version = self.version.get();
        let content = content_json(&snapshot);

        let patch = if full { None } else { self.base.borrow().as_ref().map(|base| merge_diff(base, &content)) };
        self.in_flight.set(true);
        let result = match patch {
            // Nada mudou desde a última gravação: não há o que enviar
            Some(None) => Ok(snapshot.version),
            Some(Some(diff)) => patch_sheet(id, snapshot.version, diff.to_string()).await,
            None => update_sheet(id, snapshot).await,
        };
        self.in_flight.set(false);
        match result {
            Ok(version) => {
                self.version.set(version);
                *self.base.borrow_mut() = Some(content);
                // Sem notificar: mudar só a versão não deve marcar a ficha como alterada
                self.set_data.try_update_untracked(|d| d.version = version);
                Ok(())
            }
            Err(e) => {
                let msg = e.to_string();
                if msg.contains(SHEET_CONFLICT_ERROR) {
                    let _ = self.conflict.try_set(true);
                    return Err("Alterada em outro lugar".to_string());
                }
                Err(msg)
            }
        }
    }
}
