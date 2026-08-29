use leptos::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SheetPageTab {
    Main,
    MagicCombat,
    Expanded,
    HistoryVisuals,
    Grimoire,
    Notes,
}

#[component]
pub fn SheetTabs(
    active_tab: ReadSignal<SheetPageTab>,
    set_active_tab: WriteSignal<SheetPageTab>,
    #[prop(optional)] is_gods_and_monsters: Option<Signal<bool>>,
) -> impl IntoView {
    let is_gm = move || is_gods_and_monsters.map(|s| s.get()).unwrap_or(false);

    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    view! {
        <nav class="sheet-tabs-container" aria-label="Páginas da Ficha">
            <div class="sheet-tabs-nav">
                {move || if is_gm() {
                    view! {
                        <div class="sheet-tabs-buttons">
                            <button 
                                class="sheet-tab-btn"
                                class:active=move || active_tab.get() == SheetPageTab::Main
                                on:click=move |_| set_active_tab.set(SheetPageTab::Main)
                                title="Page 1: Attributes, Abilities & Advantages"
                            >
                                <span class="sheet-tab-icon">"🐉"</span>
                                <span class="sheet-tab-title">"Attributes & Powers"</span>
                                <span class="sheet-tab-page-tag">"Page 1"</span>
                            </button>

                            <button 
                                class="sheet-tab-btn"
                                class:active=move || active_tab.get() == SheetPageTab::MagicCombat
                                on:click=move |_| set_active_tab.set(SheetPageTab::MagicCombat)
                                title="Page 2: Other Traits, Merits & Flaws, History, Rules & Combat"
                            >
                                <span class="sheet-tab-icon">"⚔️"</span>
                                <span class="sheet-tab-title">"Traits, Rules & Combat"</span>
                                <span class="sheet-tab-page-tag">"Page 2"</span>
                            </button>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="sheet-tabs-buttons">
                            <button 
                                class="sheet-tab-btn"
                                class:active=move || active_tab.get() == SheetPageTab::Main
                                on:click=move |_| set_active_tab.set(SheetPageTab::Main)
                                title="Página 1: Atributos, Habilidades, Esferas e Vantagens"
                            >
                                <span class="sheet-tab-icon">"📜"</span>
                                <span class="sheet-tab-title">
                                    {match lang() {
                                        crate::i18n::Language::PtBr => "Principal",
                                        crate::i18n::Language::EnUs => "Main",
                                    }}
                                </span>
                            </button>

                            <button 
                                class="sheet-tab-btn"
                                class:active=move || active_tab.get() == SheetPageTab::MagicCombat
                                on:click=move |_| set_active_tab.set(SheetPageTab::MagicCombat)
                                title="Página 2: Qualidades & Defeitos, Outros Traços, Maravilhas, Rotes e Combate"
                            >
                                <span class="sheet-tab-icon">"⚔️"</span>
                                <span class="sheet-tab-title">
                                    {match lang() {
                                        crate::i18n::Language::PtBr => "Magia & Combate",
                                        crate::i18n::Language::EnUs => "Magic & Combat",
                                    }}
                                </span>
                            </button>

                            <button 
                                class="sheet-tab-btn"
                                class:active=move || active_tab.get() == SheetPageTab::Expanded
                                on:click=move |_| set_active_tab.set(SheetPageTab::Expanded)
                                title="Página 3: Antecedentes Expandidos, Posses, Focos, Grimório e Capela"
                            >
                                <span class="sheet-tab-icon">"🏛️"</span>
                                <span class="sheet-tab-title">
                                    {match lang() {
                                        crate::i18n::Language::PtBr => "Antecedentes",
                                        crate::i18n::Language::EnUs => "Backgrounds",
                                    }}
                                </span>
                            </button>

                            <button 
                                class="sheet-tab-btn"
                                class:active=move || active_tab.get() == SheetPageTab::HistoryVisuals
                                on:click=move |_| set_active_tab.set(SheetPageTab::HistoryVisuals)
                                title="Página 4: História, Objetivos, Descrição Física, Avatar, Cabal e Retrato"
                            >
                                <span class="sheet-tab-icon">"📖"</span>
                                <span class="sheet-tab-title">
                                    {match lang() {
                                        crate::i18n::Language::PtBr => "História",
                                        crate::i18n::Language::EnUs => "History",
                                    }}
                                </span>
                            </button>

                            <button 
                                class="sheet-tab-btn"
                                class:active=move || active_tab.get() == SheetPageTab::Grimoire
                                on:click=move |_| set_active_tab.set(SheetPageTab::Grimoire)
                                title="Página 5: Grimório, Práticas, Instrumentos e Rotinas Mágicas (Rotes)"
                            >
                                <span class="sheet-tab-icon">"🔮"</span>
                                <span class="sheet-tab-title">
                                    {match lang() {
                                        crate::i18n::Language::PtBr => "Grimório",
                                        crate::i18n::Language::EnUs => "Grimoire",
                                    }}
                                </span>
                            </button>

                            <button 
                                class="sheet-tab-btn"
                                class:active=move || active_tab.get() == SheetPageTab::Notes
                                on:click=move |_| set_active_tab.set(SheetPageTab::Notes)
                                title="Página 6: Anotações da Crônica, Diário de Campanha e Documentos Visuais"
                            >
                                <span class="sheet-tab-icon">"📝"</span>
                                <span class="sheet-tab-title">
                                    {match lang() {
                                        crate::i18n::Language::PtBr => "Notas",
                                        crate::i18n::Language::EnUs => "Notes",
                                    }}
                                </span>
                            </button>
                        </div>
                    }.into_view()
                }}
            </div>
        </nav>
    }
}
