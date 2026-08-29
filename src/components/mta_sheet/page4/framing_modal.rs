use leptos::*;
use wasm_bindgen::JsCast;
use crate::components::Callback;

#[component]
pub fn CardFramingModal(
    image_url: String,
    initial_focus_x: i32,
    initial_focus_y: i32,
    on_save: Callback<(i32, i32)>,
    on_close: Callback<()>,
) -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let current_lang = lang_ctx.map(|c| c.lang.get_untracked()).unwrap_or_default();

    let temp_x = create_rw_signal(initial_focus_x.clamp(0, 100));
    let temp_y = create_rw_signal(initial_focus_y.clamp(0, 100));

    let is_dragging = create_rw_signal(false);
    let drag_start = create_rw_signal((0.0f64, 0.0f64, initial_focus_x as f64, initial_focus_y as f64));

    let on_pointer_down = move |ev: ev::PointerEvent| {
        ev.prevent_default();
        let client_x = ev.client_x() as f64;
        let client_y = ev.client_y() as f64;
        let cur_x = temp_x.get() as f64;
        let cur_y = temp_y.get() as f64;
        drag_start.set((client_x, client_y, cur_x, cur_y));
        is_dragging.set(true);

        if let Some(target) = ev.current_target() {
            if let Ok(elem) = target.dyn_into::<web_sys::Element>() {
                let _ = elem.set_pointer_capture(ev.pointer_id());
            }
        }
    };

    let on_pointer_move = move |ev: ev::PointerEvent| {
        if is_dragging.get() {
            let (start_px, start_py, start_fx, start_fy) = drag_start.get();
            let dx = (ev.client_x() as f64) - start_px;
            let dy = (ev.client_y() as f64) - start_py;

            // Dragging down (positive dy) pulls top of image into view -> focus_y decreases
            // Dragging right (positive dx) pulls left of image into view -> focus_x decreases
            let sensitivity_x = 2.4;
            let sensitivity_y = 1.6;

            let new_x = (start_fx - (dx / sensitivity_x)).clamp(0.0, 100.0);
            let new_y = (start_fy - (dy / sensitivity_y)).clamp(0.0, 100.0);

            temp_x.set(new_x.round() as i32);
            temp_y.set(new_y.round() as i32);
        }
    };

    let on_pointer_up = move |ev: ev::PointerEvent| {
        if is_dragging.get() {
            is_dragging.set(false);
            if let Some(target) = ev.current_target() {
                if let Ok(elem) = target.dyn_into::<web_sys::Element>() {
                    let _ = elem.release_pointer_capture(ev.pointer_id());
                }
            }
        }
    };

    let on_wheel = move |ev: ev::WheelEvent| {
        ev.prevent_default();
        let dy = ev.delta_y();
        let step = if dy > 0.0 { 3 } else { -3 };
        let cur_y = temp_y.get();
        temp_y.set((cur_y + step).clamp(0, 100));
    };

    let handle_save = {
        let on_save = on_save.clone();
        move |_| {
            on_save.call((temp_x.get(), temp_y.get()));
        }
    };

    let on_close_overlay = on_close.clone();
    let on_close_btn = on_close.clone();
    let on_close_cancel = on_close.clone();

    view! {
        <div class="framing-modal-overlay" on:click=move |_| on_close_overlay.call(())>
            <div class="framing-modal-container" on:click=move |ev: ev::MouseEvent| ev.stop_propagation()>
                // Modal Header
                <div class="framing-modal-header">
                    <div class="framing-modal-title-box">
                        <span class="framing-modal-icon">"📐"</span>
                        <div>
                            <h3 class="framing-modal-title">
                                {match current_lang {
                                    crate::i18n::Language::PtBr => "Ajustar Enquadramento no Card",
                                    crate::i18n::Language::EnUs => "Adjust Card Portrait Framing",
                                }}
                            </h3>
                            <p class="framing-modal-desc">
                                {match current_lang {
                                    crate::i18n::Language::PtBr => "Arraste a foto dentro do retângulo para definir qual parte aparece no card.",
                                    crate::i18n::Language::EnUs => "Drag the photo inside the frame to set which part appears on the card.",
                                }}
                            </p>
                        </div>
                    </div>
                    <button
                        type="button"
                        class="framing-modal-close-btn"
                        on:click=move |_| on_close_btn.call(())
                        title="Fechar"
                    >
                        "✕"
                    </button>
                </div>

                // Interactive Framing Stage
                <div class="framing-stage-wrapper">
                    <div
                        class="framing-stage-box"
                        class:dragging=move || is_dragging.get()
                        on:pointerdown=on_pointer_down
                        on:pointermove=on_pointer_move
                        on:pointerup=on_pointer_up
                        on:pointercancel=on_pointer_up
                        on:wheel=on_wheel
                    >
                        <img
                            src=image_url.clone()
                            alt="Card Framing Target"
                            class="framing-stage-img"
                            style=move || format!("object-position: {}% {}%;", temp_x.get(), temp_y.get())
                            draggable="false"
                        />
                        <div class="card-portrait-gradient"></div>
                        
                        // Rule-of-thirds alignment guide grid
                        <div class="framing-grid-overlay">
                            <div class="grid-line grid-line-h1"></div>
                            <div class="grid-line grid-line-h2"></div>
                            <div class="grid-line grid-line-v1"></div>
                            <div class="grid-line grid-line-v2"></div>
                        </div>

                        // Dynamic Drag Indicator Badge
                        <div class="framing-drag-hint-badge">
                            {move || if is_dragging.get() {
                                "✊ Posicionando..."
                            } else {
                                "🖐️ Clique e arraste para mover"
                            }}
                        </div>
                    </div>

                    // Quick Presets Row
                    <div class="framing-modal-presets">
                        <span class="presets-label">
                            {match current_lang {
                                crate::i18n::Language::PtBr => "Atalhos rápidos:",
                                crate::i18n::Language::EnUs => "Quick presets:",
                            }}
                        </span>
                        <button
                            type="button"
                            class="btn-framing-preset"
                            class:active=move || temp_y.get() == 15
                            on:click=move |_| temp_y.set(15)
                        >
                            "🔝 " {match current_lang {
                                crate::i18n::Language::PtBr => "Rosto (15%)",
                                crate::i18n::Language::EnUs => "Face (15%)",
                            }}
                        </button>
                        <button
                            type="button"
                            class="btn-framing-preset"
                            class:active=move || temp_y.get() == 35
                            on:click=move |_| temp_y.set(35)
                        >
                            "👔 " {match current_lang {
                                crate::i18n::Language::PtBr => "Busto (35%)",
                                crate::i18n::Language::EnUs => "Chest (35%)",
                            }}
                        </button>
                        <button
                            type="button"
                            class="btn-framing-preset"
                            class:active=move || temp_y.get() == 50
                            on:click=move |_| temp_y.set(50)
                        >
                            "🎯 " {match current_lang {
                                crate::i18n::Language::PtBr => "Centro (50%)",
                                crate::i18n::Language::EnUs => "Center (50%)",
                            }}
                        </button>
                        <button
                            type="button"
                            class="btn-framing-preset"
                            class:active=move || temp_y.get() == 80
                            on:click=move |_| temp_y.set(80)
                        >
                            "👖 " {match current_lang {
                                crate::i18n::Language::PtBr => "Base (80%)",
                                crate::i18n::Language::EnUs => "Bottom (80%)",
                            }}
                        </button>
                    </div>

                    // Info footer / coordinate badge
                    <div class="framing-coords-info">
                        <span class="coords-badge">
                            {move || format!("Posição Atual: Y: {}% | X: {}%", temp_y.get(), temp_x.get())}
                        </span>
                        <span class="wheel-hint">
                            "💡 Roda do mouse (scroll) também rola a imagem para cima e para baixo."
                        </span>
                    </div>
                </div>

                // Modal Actions Footer
                <div class="framing-modal-footer">
                    <button
                        type="button"
                        class="btn-framing-cancel"
                        on:click=move |_| on_close_cancel.call(())
                    >
                        {match current_lang {
                            crate::i18n::Language::PtBr => "Cancelar",
                            crate::i18n::Language::EnUs => "Cancel",
                        }}
                    </button>
                    <button
                        type="button"
                        class="btn-framing-save"
                        on:click=handle_save
                    >
                        "✓ "
                        {match current_lang {
                            crate::i18n::Language::PtBr => "Salvar Enquadramento",
                            crate::i18n::Language::EnUs => "Save Framing",
                        }}
                    </button>
                </div>
            </div>
        </div>
    }
}