//! Estado do mapa no navegador.
//!
//! - `confirmed`: o mapa na versão `version` confirmada pelo servidor;
//! - `pending`: edições locais enviadas e ainda não incorporadas a `confirmed`;
//! - `view`: o que aparece na tela = `confirmed` + `pending` (resposta instantânea).
//!
//! O servidor repassa cada lote de edições com o número da versão. Se a sequência
//! pular (evento perdido) ou vier sem detalhes, o mapa é recarregado do servidor.

use leptos::*;
use crate::rooms::{apply_map_ops, get_room_map, MapOp, RoomMapData};

#[derive(Clone)]
struct Batch {
    id: u64,
    ops: Vec<MapOp>,
    /// Versão do mapa que o servidor criou ao aplicar este lote
    acked_version: Option<i64>,
}

#[derive(Clone, Copy)]
pub struct MapStore {
    pub view: RwSignal<RoomMapData>,
    pub version: RwSignal<i64>,
    pub is_gm: RwSignal<bool>,
    pub my_sheet_ids: RwSignal<Vec<String>>,
    pub loaded: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
    pub saving: RwSignal<bool>,
    confirmed: StoredValue<RoomMapData>,
    pending: StoredValue<Vec<Batch>>,
    next_batch: StoredValue<u64>,
    room_id: Signal<String>,
}

impl MapStore {
    pub fn new(room_id: Signal<String>) -> Self {
        Self {
            view: create_rw_signal(RoomMapData::default()),
            version: create_rw_signal(-1),
            is_gm: create_rw_signal(false),
            my_sheet_ids: create_rw_signal(Vec::new()),
            loaded: create_rw_signal(false),
            error: create_rw_signal(None),
            saving: create_rw_signal(false),
            confirmed: store_value(RoomMapData::default()),
            pending: store_value(Vec::new()),
            next_batch: store_value(0),
            room_id,
        }
    }

    fn recompute(&self) {
        let mut view = self.confirmed.get_value();
        self.pending.with_value(|batches| {
            for batch in batches {
                for op in &batch.ops {
                    let _ = view.apply(op);
                }
            }
        });
        self.view.set(view);
        self.saving.set(self.pending.with_value(|b| b.iter().any(|x| x.acked_version.is_none())));
    }

    /// Descarta lotes que o estado confirmado já contém.
    fn prune(&self) {
        let version = self.version.get_untracked();
        self.pending.update_value(|b| b.retain(|x| x.acked_version.map_or(true, |v| v > version)));
        self.recompute();
    }

    /// Busca o mapa completo no servidor (abertura, reconexão, evento perdido).
    pub fn reload(&self) {
        let store = *self;
        let room_id = self.room_id.get_untracked();
        if room_id.is_empty() {
            return;
        }
        spawn_local(async move {
            match get_room_map(room_id).await {
                Ok(snapshot) => {
                    store.confirmed.set_value(snapshot.map);
                    store.version.set(snapshot.version);
                    store.is_gm.set(snapshot.is_gm);
                    store.my_sheet_ids.set(snapshot.my_sheet_ids);
                    store.loaded.set(true);
                    store.error.set(None);
                    store.prune();
                }
                Err(e) => store.error.set(Some(crate::components::rooms::room_chat::store::clean_error(&e.to_string()))),
            }
        });
    }

    /// Edições feitas por qualquer pessoa da sala (vindas do evento em tempo real).
    pub fn on_remote_change(&self, version: i64, ops: Option<&Vec<MapOp>>) {
        let current = self.version.get_untracked();
        if version <= current {
            return;
        }
        match ops {
            Some(ops) if version == current + 1 => {
                self.confirmed.update_value(|m| {
                    for op in ops {
                        let _ = m.apply(op);
                    }
                });
                self.version.set(version);
                self.prune();
            }
            _ => self.reload(),
        }
    }

    /// Aplica na tela na hora e envia ao servidor. Se o servidor recusar, desfaz.
    pub fn send(&self, ops: Vec<MapOp>) {
        if ops.is_empty() {
            return;
        }
        let store = *self;
        let id = self.next_batch.get_value();
        self.next_batch.set_value(id + 1);
        self.pending.update_value(|b| b.push(Batch { id, ops: ops.clone(), acked_version: None }));
        self.recompute();

        let room_id = self.room_id.get_untracked();
        spawn_local(async move {
            match apply_map_ops(room_id, ops).await {
                Ok(version) => {
                    store.pending.update_value(|b| {
                        if let Some(batch) = b.iter_mut().find(|x| x.id == id) {
                            batch.acked_version = Some(version);
                        }
                    });
                    store.error.set(None);
                    store.prune();
                }
                Err(e) => {
                    store.pending.update_value(|b| b.retain(|x| x.id != id));
                    store.error.set(Some(crate::components::rooms::room_chat::store::clean_error(&e.to_string())));
                    store.recompute();
                }
            }
        });
    }

    /// Pré-visualização local sem enviar (ex.: pincelada em andamento).
    pub fn preview(&self, ops: &[MapOp]) {
        self.recompute();
        if ops.is_empty() {
            return;
        }
        self.view.update(|m| {
            for op in ops {
                let _ = m.apply(op);
            }
        });
    }

    pub fn can_move_token(&self, token: &crate::rooms::MapToken) -> bool {
        self.is_gm.get_untracked()
            || token.sheet_id.as_ref().is_some_and(|id| self.my_sheet_ids.with_untracked(|mine| mine.contains(id)))
    }
}
