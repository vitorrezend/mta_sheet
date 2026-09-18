use leptos::*;
use crate::state::CharacterSummary;

pub fn render_delete_sheet_modal<FCancel, FConfirm>(
    sheet_to_delete: ReadSignal<Option<CharacterSummary>>,
    cancel_delete: FCancel,
    confirm_delete: FConfirm,
    current_lang: crate::i18n::Language,
) -> impl IntoView
where
    FCancel: Fn(ev::MouseEvent) + Copy + 'static,
    FConfirm: Fn() + Copy + 'static,
{
    view! {
        {move || sheet_to_delete.get().map(|target| view! {
            <div class="modal-overlay" on:click=cancel_delete>
                <div class="modal-card" on:click=move |ev| ev.stop_propagation()>
                    <h3 class="modal-title">{crate::i18n::tr("home_delete_title", current_lang)}</h3>
                    <p class="modal-text">
                        {crate::i18n::tr("home_delete_prompt", current_lang)}
                        <strong>{target.name}</strong>"?"
                    </p>
                    <p class="modal-subtext">{crate::i18n::tr("home_delete_sub", current_lang)}</p>
                    <div class="modal-actions">
                        <button class="modal-btn btn-cancel" on:click=cancel_delete>
                            {crate::i18n::tr("home_btn_cancel", current_lang)}
                        </button>
                        <button class="modal-btn btn-danger" on:click=move |_| confirm_delete()>
                            {crate::i18n::tr("home_btn_confirm_delete", current_lang)}
                        </button>
                    </div>
                </div>
            </div>
        })}
    }
}
