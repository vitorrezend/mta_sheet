use leptos::*;
use crate::state::CharacterData;
use crate::components::mta_sheet::sheet::sheet_tabs::SheetPageTab;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PdfExportMode {
    Smart,   // Oculta páginas e seções vazias automaticamente
    Current, // Apenas a aba ativa no momento
    All,     // Todas as páginas da ficha
    Custom,  // Seleção manual de páginas
}

#[component]
pub fn PdfExportModal(
    show_modal: ReadSignal<bool>,
    set_show_modal: WriteSignal<bool>,
    data: ReadSignal<CharacterData>,
    current_active_tab: Signal<SheetPageTab>,
    #[prop(into, default = false.into())] is_gods_and_monsters: MaybeSignal<bool>,
) -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let export_mode = create_rw_signal(PdfExportMode::Smart);
    let hide_empty_sections = create_rw_signal(true);

    // Seleção de páginas para o modo customizado (Páginas 1 a 6)
    let p1_checked = create_rw_signal(true);
    let p2_checked = create_rw_signal(true);
    let p3_checked = create_rw_signal(true);
    let p4_checked = create_rw_signal(true);
    let p5_checked = create_rw_signal(true);
    let p6_checked = create_rw_signal(true);

    // Atualiza a seleção padrão quando o modal abre baseado no modo Smart
    create_effect(move |_| {
        if show_modal.get() {
            let is_gm = is_gods_and_monsters.get();
            if is_gm {
                p1_checked.set(true);
                p2_checked.set(true);
                p3_checked.set(false);
                p4_checked.set(false);
                p5_checked.set(false);
                p6_checked.set(false);
            } else {
                p1_checked.set(true);
                p2_checked.set(data.with(|d| d.is_page_has_content(1)));
                p3_checked.set(data.with(|d| d.is_page_has_content(2)));
                p4_checked.set(data.with(|d| d.is_page_has_content(3)));
                p5_checked.set(data.with(|d| d.is_page_has_content(4)));
                p6_checked.set(data.with(|d| d.is_page_has_content(5)));
            }
        }
    });

    let on_confirm_print = move |_| {
        let is_gm = is_gods_and_monsters.get();
        let mode = export_mode.get();
        let cur_tab = current_active_tab.get();

        let (p1, p2, p3, p4, p5, p6) = match mode {
            PdfExportMode::Smart => {
                if is_gm {
                    (true, true, false, false, false, false)
                } else {
                    (
                        true,
                        data.with(|d| d.is_page_has_content(1)),
                        data.with(|d| d.is_page_has_content(2)),
                        data.with(|d| d.is_page_has_content(3)),
                        data.with(|d| d.is_page_has_content(4)),
                        data.with(|d| d.is_page_has_content(5)),
                    )
                }
            }
            PdfExportMode::Current => (
                cur_tab == SheetPageTab::Main,
                cur_tab == SheetPageTab::MagicCombat,
                cur_tab == SheetPageTab::Expanded,
                cur_tab == SheetPageTab::HistoryVisuals,
                cur_tab == SheetPageTab::Grimoire,
                cur_tab == SheetPageTab::Notes,
            ),
            PdfExportMode::All => {
                if is_gm {
                    (true, true, false, false, false, false)
                } else {
                    (true, true, true, true, true, true)
                }
            }
            PdfExportMode::Custom => (
                p1_checked.get(),
                p2_checked.get(),
                p3_checked.get(),
                p4_checked.get(),
                p5_checked.get(),
                p6_checked.get(),
            ),
        };

        // Aplica as classes no body para o @media print
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(body) = doc.body() {
                let class_list = body.class_list();
                let _ = class_list.toggle_with_force("print-hide-p1", !p1);
                let _ = class_list.toggle_with_force("print-hide-p2", !p2);
                let _ = class_list.toggle_with_force("print-hide-p3", !p3);
                let _ = class_list.toggle_with_force("print-hide-p4", !p4);
                let _ = class_list.toggle_with_force("print-hide-p5", !p5);
                let _ = class_list.toggle_with_force("print-hide-p6", !p6);
                let _ = class_list.toggle_with_force("print-hide-empty", hide_empty_sections.get());
            }
        }

        set_show_modal.set(false);

        // Dispara a impressão após fechar o modal
        request_animation_frame(move || {
            if let Some(w) = web_sys::window() {
                let _ = w.print();
            }
        });
    };

    view! {
        {move || if show_modal.get() {
            let is_gm = is_gods_and_monsters.get();
            let current_lang = lang();
            let mode = export_mode.get();

            let p2_has = data.with(|d| d.is_page_has_content(1));
            let p3_has = data.with(|d| d.is_page_has_content(2));
            let p4_has = data.with(|d| d.is_page_has_content(3));
            let p5_has = data.with(|d| d.is_page_has_content(4));
            let p6_has = data.with(|d| d.is_page_has_content(5));

            view! {
                <div class="modal-overlay" on:click=move |_| set_show_modal.set(false)>
                    <div class="modal-card pdf-export-modal" on:click=move |ev| ev.stop_propagation()>
                        <div class="modal-header">
                            <div class="modal-title-group">
                                <h2 class="modal-title">
                                    {match current_lang {
                                        crate::i18n::Language::PtBr => "🖨️ Opções de Exportação para PDF",
                                        crate::i18n::Language::EnUs => "🖨️ PDF Export Options",
                                    }}
                                </h2>
                                <span class="modal-subtitle">
                                    {match current_lang {
                                        crate::i18n::Language::PtBr => "Escolha como deseja gerar o documento oficial em folha A4",
                                        crate::i18n::Language::EnUs => "Choose how you want to generate the official A4 document",
                                    }}
                                </span>
                            </div>
                            <button class="modal-close-btn" on:click=move |_| set_show_modal.set(false)>"✕"</button>
                        </div>

                        <div class="pdf-modal-body">
                            // 1. Modos de Exportação
                            <div class="pdf-options-section">
                                <span class="pdf-section-title">
                                    {match current_lang {
                                        crate::i18n::Language::PtBr => "Modo de Impressão:",
                                        crate::i18n::Language::EnUs => "Print Mode:",
                                    }}
                                </span>

                                <div class="pdf-modes-grid">
                                    // Modo Inteligente / Econômico (Recomendado)
                                    <label class="pdf-mode-card" class:selected=move || mode == PdfExportMode::Smart>
                                        <input 
                                            type="radio" 
                                            name="pdf_mode" 
                                            checked=move || mode == PdfExportMode::Smart
                                            on:change=move |_| export_mode.set(PdfExportMode::Smart)
                                        />
                                        <div class="pdf-mode-info">
                                            <div class="pdf-mode-header">
                                                <strong>
                                                    {match current_lang {
                                                        crate::i18n::Language::PtBr => "🌱 Modo Inteligente (Econômico)",
                                                        crate::i18n::Language::EnUs => "🌱 Smart Mode (Economical)",
                                                    }}
                                                </strong>
                                                <span class="badge-recommended">
                                                    {match current_lang {
                                                        crate::i18n::Language::PtBr => "Recomendado",
                                                        crate::i18n::Language::EnUs => "Recommended",
                                                    }}
                                                </span>
                                            </div>
                                            <p>
                                                {match current_lang {
                                                    crate::i18n::Language::PtBr => "Oculta automaticamente páginas e seções que não têm informações preenchidas, poupando folhas em branco.",
                                                    crate::i18n::Language::EnUs => "Automatically skips empty pages and unpopulated sections, saving blank sheets.",
                                                }}
                                            </p>
                                        </div>
                                    </label>

                                    // Apenas a Página Atual
                                    <label class="pdf-mode-card" class:selected=move || mode == PdfExportMode::Current>
                                        <input 
                                            type="radio" 
                                            name="pdf_mode" 
                                            checked=move || mode == PdfExportMode::Current
                                            on:change=move |_| export_mode.set(PdfExportMode::Current)
                                        />
                                        <div class="pdf-mode-info">
                                            <strong>
                                                {match current_lang {
                                                    crate::i18n::Language::PtBr => "📄 Apenas Página Atual (Na Tela)",
                                                    crate::i18n::Language::EnUs => "📄 Current Page Only (On Screen)",
                                                }}
                                            </strong>
                                            <p>
                                                {match current_lang {
                                                    crate::i18n::Language::PtBr => "Exporta apenas a aba que você está visualizando no momento.",
                                                    crate::i18n::Language::EnUs => "Exports only the tab currently visible on your screen.",
                                                }}
                                            </p>
                                        </div>
                                    </label>

                                    // Todas as Páginas
                                    <label class="pdf-mode-card" class:selected=move || mode == PdfExportMode::All>
                                        <input 
                                            type="radio" 
                                            name="pdf_mode" 
                                            checked=move || mode == PdfExportMode::All
                                            on:change=move |_| export_mode.set(PdfExportMode::All)
                                        />
                                        <div class="pdf-mode-info">
                                            <strong>
                                                {match current_lang {
                                                    crate::i18n::Language::PtBr => "📚 Todas as Páginas (Completa)",
                                                    crate::i18n::Language::EnUs => "📚 All Pages (Full)",
                                                }}
                                            </strong>
                                            <p>
                                                {match current_lang {
                                                    crate::i18n::Language::PtBr => "Imprime todas as folhas da ficha, mesmo que vazias.",
                                                    crate::i18n::Language::EnUs => "Prints all sheet pages, even if empty.",
                                                }}
                                            </p>
                                        </div>
                                    </label>

                                    // Personalizado
                                    <label class="pdf-mode-card" class:selected=move || mode == PdfExportMode::Custom>
                                        <input 
                                            type="radio" 
                                            name="pdf_mode" 
                                            checked=move || mode == PdfExportMode::Custom
                                            on:change=move |_| export_mode.set(PdfExportMode::Custom)
                                        />
                                        <div class="pdf-mode-info">
                                            <strong>
                                                {match current_lang {
                                                    crate::i18n::Language::PtBr => "⚙️ Seleção Manual de Páginas",
                                                    crate::i18n::Language::EnUs => "⚙️ Manual Page Selection",
                                                }}
                                            </strong>
                                            <p>
                                                {match current_lang {
                                                    crate::i18n::Language::PtBr => "Escolha exatamente quais páginas deseja incluir.",
                                                    crate::i18n::Language::EnUs => "Choose exactly which pages to include.",
                                                }}
                                            </p>
                                        </div>
                                    </label>
                                </div>
                            </div>

                            // 2. Checklist de Páginas (Apenas se Modo Customizado ou Informativo)
                            {move || if mode == PdfExportMode::Custom || mode == PdfExportMode::Smart {
                                view! {
                                    <div class="pdf-pages-checklist-container">
                                        <span class="pdf-section-title">
                                            {match current_lang {
                                                crate::i18n::Language::PtBr => "Páginas que serão incluídas:",
                                                crate::i18n::Language::EnUs => "Pages to be included:",
                                            }}
                                        </span>

                                        <div class="pdf-pages-checklist">
                                            {if is_gm {
                                                view! {
                                                    // Gods and Monsters (2 Páginas)
                                                    <label class="pdf-page-check-row">
                                                        <input 
                                                            type="checkbox" 
                                                            checked=move || p1_checked.get()
                                                            disabled=move || mode != PdfExportMode::Custom
                                                            on:change=move |ev| p1_checked.set(event_target_checked(&ev))
                                                        />
                                                        <span>"Página 1: Atributos, Habilidades e Vantagens"</span>
                                                        <span class="badge-has-content">"✓ Com dados"</span>
                                                    </label>
                                                    <label class="pdf-page-check-row">
                                                        <input 
                                                            type="checkbox" 
                                                            checked=move || p2_checked.get()
                                                            disabled=move || mode != PdfExportMode::Custom
                                                            on:change=move |ev| p2_checked.set(event_target_checked(&ev))
                                                        />
                                                        <span>"Página 2: Poderes, História e Descrição"</span>
                                                        <span class="badge-has-content">"✓ Com dados"</span>
                                                    </label>
                                                }.into_view()
                                            } else {
                                                view! {
                                                    // Mago: A Ascensão (6 Páginas)
                                                    <label class="pdf-page-check-row">
                                                        <input 
                                                            type="checkbox" 
                                                            checked=move || p1_checked.get()
                                                            disabled=move || mode != PdfExportMode::Custom
                                                            on:change=move |ev| p1_checked.set(event_target_checked(&ev))
                                                        />
                                                        <span>"Página 1: Principal (Atributos, Esferas, Vantagens)"</span>
                                                        <span class="badge-has-content">"✓ Com dados"</span>
                                                    </label>

                                                    <label class="pdf-page-check-row">
                                                        <input 
                                                            type="checkbox" 
                                                            checked=move || if mode == PdfExportMode::Smart { p2_has } else { p2_checked.get() }
                                                            disabled=move || mode != PdfExportMode::Custom
                                                            on:change=move |ev| p2_checked.set(event_target_checked(&ev))
                                                        />
                                                        <span>"Página 2: Mágika & Combate (Méritos, Armas, Focos)"</span>
                                                        {if p2_has {
                                                            view! { <span class="badge-has-content">"✓ Com dados"</span> }.into_view()
                                                        } else {
                                                            view! { <span class="badge-empty">"○ Vazia (Omitida)"</span> }.into_view()
                                                        }}
                                                    </label>

                                                    <label class="pdf-page-check-row">
                                                        <input 
                                                            type="checkbox" 
                                                            checked=move || if mode == PdfExportMode::Smart { p3_has } else { p3_checked.get() }
                                                            disabled=move || mode != PdfExportMode::Custom
                                                            on:change=move |ev| p3_checked.set(event_target_checked(&ev))
                                                        />
                                                        <span>"Página 3: Antecedentes Expandidos, Posses & Capela"</span>
                                                        {if p3_has {
                                                            view! { <span class="badge-has-content">"✓ Com dados"</span> }.into_view()
                                                        } else {
                                                            view! { <span class="badge-empty">"○ Vazia (Omitida)"</span> }.into_view()
                                                        }}
                                                    </label>

                                                    <label class="pdf-page-check-row">
                                                        <input 
                                                            type="checkbox" 
                                                            checked=move || if mode == PdfExportMode::Smart { p4_has } else { p4_checked.get() }
                                                            disabled=move || mode != PdfExportMode::Custom
                                                            on:change=move |ev| p4_checked.set(event_target_checked(&ev))
                                                        />
                                                        <span>"Página 4: História, Descrição & Galeria Visual"</span>
                                                        {if p4_has {
                                                            view! { <span class="badge-has-content">"✓ Com dados"</span> }.into_view()
                                                        } else {
                                                            view! { <span class="badge-empty">"○ Vazia (Omitida)"</span> }.into_view()
                                                        }}
                                                    </label>

                                                    <label class="pdf-page-check-row">
                                                        <input 
                                                            type="checkbox" 
                                                            checked=move || if mode == PdfExportMode::Smart { p5_has } else { p5_checked.get() }
                                                            disabled=move || mode != PdfExportMode::Custom
                                                            on:change=move |ev| p5_checked.set(event_target_checked(&ev))
                                                        />
                                                        <span>"Página 5: Grimório & Rituais Místicos"</span>
                                                        {if p5_has {
                                                            view! { <span class="badge-has-content">"✓ Com dados"</span> }.into_view()
                                                        } else {
                                                            view! { <span class="badge-empty">"○ Vazia (Omitida)"</span> }.into_view()
                                                        }}
                                                    </label>

                                                    <label class="pdf-page-check-row">
                                                        <input 
                                                            type="checkbox" 
                                                            checked=move || if mode == PdfExportMode::Smart { p6_has } else { p6_checked.get() }
                                                            disabled=move || mode != PdfExportMode::Custom
                                                            on:change=move |ev| p6_checked.set(event_target_checked(&ev))
                                                        />
                                                        <span>"Página 6: Diário de Sessão & Anotações"</span>
                                                        {if p6_has {
                                                            view! { <span class="badge-has-content">"✓ Com dados"</span> }.into_view()
                                                        } else {
                                                            view! { <span class="badge-empty">"○ Vazia (Omitida)"</span> }.into_view()
                                                        }}
                                                    </label>
                                                }.into_view()
                                            }}
                                        </div>
                                    </div>
                                }.into_view()
                            } else {
                                ().into_view()
                            }}

                            // 3. Compactação Adicional
                            <div class="pdf-compact-option">
                                <label class="pdf-compact-checkbox">
                                    <input 
                                        type="checkbox" 
                                        checked=move || hide_empty_sections.get()
                                        on:change=move |ev| hide_empty_sections.set(event_target_checked(&ev))
                                    />
                                    <span>
                                        {match current_lang {
                                            crate::i18n::Language::PtBr => "Ocultar listas e itens não preenchidos (ex: armas/méritos vazios) para compactar o PDF",
                                            crate::i18n::Language::EnUs => "Hide unfilled item rows and lists (e.g. empty weapons/merits) to compact the PDF",
                                        }}
                                    </span>
                                </label>
                            </div>
                        </div>

                        <div class="modal-footer">
                            <button 
                                type="button" 
                                class="btn-modal-secondary" 
                                on:click=move |_| set_show_modal.set(false)
                            >
                                {match current_lang {
                                    crate::i18n::Language::PtBr => "Cancelar",
                                    crate::i18n::Language::EnUs => "Cancel",
                                }}
                            </button>

                            <button 
                                type="button" 
                                class="btn-modal-primary btn-generate-pdf"
                                on:click=on_confirm_print
                            >
                                "🖨️ "
                                {match current_lang {
                                    crate::i18n::Language::PtBr => "Gerar PDF / Imprimir",
                                    crate::i18n::Language::EnUs => "Generate PDF / Print",
                                }}
                            </button>
                        </div>
                    </div>
                </div>
            }.into_view()
        } else {
            ().into_view()
        }}
    }
}
