//! Capela e Diário da sala: textos compartilhados, editados por qualquer membro.
//!
//! Cada um é um [`SyncedDoc`]: o formulário local, a última versão confirmada pelo
//! servidor e, se outra pessoa salvar enquanto alguém edita, a versão remota pendente.
//! Ninguém perde edições em silêncio: quem não estava editando recebe a mudança na hora;
//! quem estava, vê um aviso e escolhe.

use leptos::*;
use crate::components::common::{Callback, WysiwygEditor};
use crate::rooms::{update_room_chantry, update_room_chronicle_notes, ChantryPoolData};
use crate::state::SHEET_CONFLICT_ERROR;

/// Documento compartilhado com controle de versão otimista.
pub struct SyncedDoc<T: 'static> {
    /// Valor sendo editado na tela
    pub form: RwSignal<T>,
    /// Último valor confirmado pelo servidor
    synced: RwSignal<T>,
    version: RwSignal<i64>,
    initialized: RwSignal<bool>,
    /// Versão salva por outra pessoa enquanto havia edição local não salva
    pub pending_remote: RwSignal<Option<(T, i64)>>,
    /// Mensagem de status: (é erro?, texto)
    pub status: RwSignal<Option<(bool, String)>>,
    pub is_saving: RwSignal<bool>,
}

impl<T: 'static> Clone for SyncedDoc<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: 'static> Copy for SyncedDoc<T> {}

impl<T: Clone + PartialEq + Default + 'static> SyncedDoc<T> {
    pub fn new() -> Self {
        Self {
            form: create_rw_signal(T::default()),
            synced: create_rw_signal(T::default()),
            version: create_rw_signal(0),
            initialized: create_rw_signal(false),
            pending_remote: create_rw_signal(None),
            status: create_rw_signal(None),
            is_saving: create_rw_signal(false),
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.form.with_untracked(|f| self.synced.with_untracked(|s| f != s))
    }

    fn apply(&self, value: T, version: i64) {
        self.form.set(value.clone());
        self.synced.set(value);
        self.version.set(version);
        self.pending_remote.set(None);
    }

    /// Valor vindo do servidor (carga da sala ou evento em tempo real).
    pub fn receive(&self, value: T, version: i64) {
        if !self.initialized.get_untracked() {
            self.initialized.set(true);
            self.apply(value, version);
        } else if version > self.version.get_untracked() {
            if self.is_dirty() {
                self.pending_remote.set(Some((value, version)));
            } else {
                self.apply(value, version);
            }
        }
    }

    /// Descarta a edição local e usa a versão salva pela outra pessoa.
    pub fn accept_remote(&self) {
        if let Some((value, version)) = self.pending_remote.get_untracked() {
            self.apply(value, version);
            self.status.set(None);
        }
    }

    /// Salva o formulário. `persist` recebe (valor, versão esperada) e devolve a nova versão.
    pub fn save<F, Fut>(&self, persist: F)
    where
        F: FnOnce(T, i64) -> Fut + 'static,
        Fut: std::future::Future<Output = Result<i64, ServerFnError>> + 'static,
    {
        if self.is_saving.get_untracked() {
            return;
        }
        let doc = *self;
        let value = self.form.get_untracked();
        // "Manter as minhas": grava por cima da versão remota que já conhecemos
        let expected = self.pending_remote.with_untracked(|p| p.as_ref().map(|(_, v)| *v)).unwrap_or_else(|| self.version.get_untracked());
        doc.is_saving.set(true);
        spawn_local(async move {
            match persist(value.clone(), expected).await {
                Ok(version) => {
                    doc.synced.set(value);
                    doc.version.set(version);
                    doc.pending_remote.set(None);
                    doc.status.set(Some((false, "Salvo".to_string())));
                }
                Err(e) if e.to_string().contains(SHEET_CONFLICT_ERROR) => {
                    doc.status.set(Some((true, "Outra pessoa salvou antes de você. Veja o aviso acima.".to_string())));
                }
                Err(e) => doc.status.set(Some((true, e.to_string()))),
            }
            doc.is_saving.set(false);
        });
    }
}

#[component]
fn SaveBar<T: Clone + PartialEq + Default + 'static>(doc: SyncedDoc<T>, label: &'static str, on_save: Callback<()>) -> impl IntoView {
    let on_keep_mine = on_save.clone();
    view! {
        <Show when=move || doc.pending_remote.with(Option::is_some)>
            <div class="room-notes-conflict" role="alert">
                <span>"⚠️ Outra pessoa salvou uma versão nova enquanto você editava."</span>
                <button type="button" on:click=move |_| doc.accept_remote()>"Usar a versão nova"</button>
                <button type="button" on:click={let on_keep_mine = on_keep_mine.clone(); move |_| on_keep_mine.call(())}>"Manter as minhas e salvar"</button>
            </div>
        </Show>
        <div class="room-notes-savebar">
            <Show when=move || doc.status.with(Option::is_some)>
                <span class="room-notes-status" class:is-error=move || doc.status.with(|s| s.as_ref().is_some_and(|(e, _)| *e))>
                    {move || doc.status.get().map(|(_, m)| m).unwrap_or_default()}
                </span>
            </Show>
            <button class="save-chantry-btn" disabled=move || doc.is_saving.get() on:click=move |_| on_save.call(())>
                {move || if doc.is_saving.get() { "Salvando…" } else { label }}
            </button>
        </div>
    }
}

#[component]
pub fn ChantrySection(doc: SyncedDoc<ChantryPoolData>, room_id: Signal<String>) -> impl IntoView {
    let save = Callback::new(move |_: ()| {
        let r_id = room_id.get_untracked();
        doc.save(move |chantry, version| update_room_chantry(r_id, chantry, version));
    });
    let quint = move || doc.form.with(|c| c.quintessence_pool);
    let max_quint = move || doc.form.with(|c| if c.max_quintessence > 0 { c.max_quintessence } else { 20 });

    view! {
        <section class="room-chantry-section">
            <div class="section-header">
                <h2>"Santuário da Cabala"</h2>
                <SaveBar doc=doc label="Salvar Capela" on_save=save />
            </div>
            <div class="chantry-grid">
                <div class="chantry-card">
                    <h3>"Quintessência"</h3>
                    <div class="pool-numbers">{quint}"/"{max_quint}</div>
                    <div class="pool-actions">
                        <button on:click=move |_| doc.form.update(|c| c.quintessence_pool = (c.quintessence_pool - 1).max(0))>"-"</button>
                        <button on:click=move |_| {
                            let max = max_quint();
                            doc.form.update(|c| c.quintessence_pool = (c.quintessence_pool + 1).min(max));
                        }>"+"</button>
                    </div>
                </div>
                <div class="chantry-card">
                    <h3>"Localização / Nome"</h3>
                    <input
                        type="text"
                        prop:value=move || doc.form.with(|c| c.location_name.clone())
                        on:input=move |e| doc.form.update(|c| c.location_name = event_target_value(&e))
                    />
                </div>
                <div class="chantry-card chantry-card-full">
                    <h3>"Notas e Recursos Comuns"</h3>
                    <WysiwygEditor
                        value=Signal::derive(move || doc.form.with(|c| c.notes.clone()))
                        on_change=Callback::new(move |val: String| doc.form.update(|c| c.notes = val))
                        placeholder="Notas, recursos, aliados, defesas e relíquias guardadas na Capela..."
                        min_height="120px"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn ChronicleSection(doc: SyncedDoc<String>, room_id: Signal<String>) -> impl IntoView {
    let save = Callback::new(move |_: ()| {
        let r_id = room_id.get_untracked();
        doc.save(move |notes, version| update_room_chronicle_notes(r_id, notes, version));
    });

    view! {
        <section class="room-chronicle-section">
            <div class="section-header">
                <h2>"Diário da Crônica & Mural"</h2>
                <SaveBar doc=doc label="Salvar Diário" on_save=save />
            </div>
            <WysiwygEditor
                value=Signal::derive(move || doc.form.get())
                on_change=Callback::new(move |val: String| doc.form.set(val))
                placeholder="Diário da crônica, resumo das sessões, pistas investigativas e avisos para a Cabala..."
                min_height="320px"
            />
        </section>
    }
}
