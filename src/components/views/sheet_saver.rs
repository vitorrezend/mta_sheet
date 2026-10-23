//! Salvamento da ficha com controle de versão (concorrência otimista).

use leptos::*;
use crate::state::{update_sheet, CharacterData, SHEET_CONFLICT_ERROR};

/// Grava a ficha no servidor, uma gravação por vez, mantendo a versão local em dia.
///
/// Salvamento automático, periódico e manual podem disparar juntos; se duas gravações
/// da mesma aba saíssem com a mesma versão, a segunda seria recusada como conflito.
/// Por isso elas fazem fila, e cada uma lê os dados (e a versão) mais recentes.
#[derive(Clone)]
pub struct SheetSaver {
    pub in_flight: std::rc::Rc<std::cell::Cell<bool>>,
    /// Última versão confirmada pelo servidor. Fica fora dos sinais para funcionar
    /// também no salvamento ao sair da página, quando os sinais já foram descartados.
    pub version: std::rc::Rc<std::cell::Cell<i64>>,
    pub data: ReadSignal<CharacterData>,
    pub set_data: WriteSignal<CharacterData>,
    /// A ficha foi alterada em outro lugar: salvamentos pausados até o usuário decidir
    pub conflict: RwSignal<bool>,
}

impl SheetSaver {
    /// Salva o estado atual da ficha.
    pub async fn save(&self, id: String) -> Result<(), String> {
        self.save_inner(id, None).await
    }

    /// Salva um retrato já capturado (usado ao sair da página).
    pub async fn save_snapshot(&self, id: String, snapshot: CharacterData) -> Result<(), String> {
        self.save_inner(id, Some(snapshot)).await
    }

    /// Resolve um conflito mantendo as alterações locais: adota a versão atual do banco
    /// e grava por cima.
    pub async fn overwrite_with_mine(&self, id: String, token: Option<String>) -> Result<(), String> {
        let latest = crate::state::get_sheet(id.clone(), token).await.map_err(|e| e.to_string())?;
        self.version.set(latest.version);
        self.set_data.try_update_untracked(|d| d.version = latest.version);
        let _ = self.conflict.try_set(false);
        self.save(id).await
    }

    async fn save_inner(&self, id: String, snapshot: Option<CharacterData>) -> Result<(), String> {
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
        self.in_flight.set(true);
        let result = update_sheet(id, snapshot).await;
        self.in_flight.set(false);
        match result {
            Ok(version) => {
                self.version.set(version);
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
