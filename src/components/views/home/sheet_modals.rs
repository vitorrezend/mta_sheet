use leptos::*;
use crate::i18n::tr;
use super::store::{use_home, use_lang};

#[component]
pub fn DeleteSheetModal() -> impl IntoView {
    let home = use_home();
    let lang = use_lang();
    let close = move |_| home.sheet_to_delete.set(None);
    move || {
        let lang = lang.get();
        home.sheet_to_delete.get().map(|target| view! {
            <div class="modal-overlay" on:click=close>
                <div class="modal-card" on:click=move |ev| ev.stop_propagation()>
                    <h3 class="modal-title">{tr("home_delete_title", lang)}</h3>
                    <p class="modal-text">
                        {tr("home_delete_prompt", lang)}
                        <strong>{target.name}</strong>"?"
                    </p>
                    <p class="modal-subtext">{tr("home_delete_sub", lang)}</p>
                    <div class="modal-actions">
                        <button class="modal-btn btn-cancel" on:click=close>
                            {tr("home_btn_cancel", lang)}
                        </button>
                        <button class="modal-btn btn-danger" on:click=move |_| home.confirm_delete_sheet()>
                            {tr("home_btn_confirm_delete", lang)}
                        </button>
                    </div>
                </div>
            </div>
        })
    }
}
