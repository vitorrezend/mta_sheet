//! Aviso exibido quando a ficha foi alterada em outro lugar enquanto estava aberta.

use leptos::*;

#[component]
pub fn SheetConflictBanner<R, K>(conflict: RwSignal<bool>, on_reload: R, on_keep: K) -> impl IntoView
where
    R: Fn(ev::MouseEvent) + Clone + 'static,
    K: Fn(ev::MouseEvent) + Clone + 'static,
{
    view! {
        <Show when=move || conflict.get()>
            <div class="sheet-conflict-banner" role="alert">
                <p>
                    <strong>"⚠️ Esta ficha foi alterada em outro lugar"</strong>
                    " (outra aba, outro aparelho ou o Narrador pela sala). O salvamento automático foi pausado para não apagar essas mudanças."
                </p>
                <div class="sheet-conflict-actions">
                    <button type="button" class="conflict-reload-btn" on:click=on_reload.clone()>"Recarregar versão mais recente"</button>
                    <button type="button" class="conflict-keep-btn" on:click=on_keep.clone()>"Manter minhas alterações"</button>
                </div>
            </div>
        </Show>
    }
}
