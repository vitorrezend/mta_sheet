use leptos::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SheetPageTab {
    Main,
    MagicCombat,
    Expanded,
    HistoryVisuals,
}

#[component]
pub fn SheetTabs(
    active_tab: ReadSignal<SheetPageTab>,
    set_active_tab: WriteSignal<SheetPageTab>,
    #[prop(optional)] is_gods_and_monsters: Option<Signal<bool>>,
) -> impl IntoView {
    let is_gm = move || is_gods_and_monsters.map(|s| s.get()).unwrap_or(false);

    view! {
        <nav class="sheet-tabs-container" aria-label="Páginas da Ficha">
            <div class="sheet-tabs-nav">
                {move || if is_gm() {
                    view! {
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
                    }.into_view()
                } else {
                    view! {
                        <button 
                            class="sheet-tab-btn"
                            class:active=move || active_tab.get() == SheetPageTab::Main
                            on:click=move |_| set_active_tab.set(SheetPageTab::Main)
                            title="Página 1: Atributos, Habilidades, Esferas e Vantagens"
                        >
                            <span class="sheet-tab-icon">"📜"</span>
                            <span class="sheet-tab-title">"Ficha Principal"</span>
                            <span class="sheet-tab-page-tag">"Pág. 1"</span>
                        </button>

                        <button 
                            class="sheet-tab-btn"
                            class:active=move || active_tab.get() == SheetPageTab::MagicCombat
                            on:click=move |_| set_active_tab.set(SheetPageTab::MagicCombat)
                            title="Página 2: Qualidades & Defeitos, Outros Traços, Maravilhas, Rotes e Combate"
                        >
                            <span class="sheet-tab-icon">"⚔️"</span>
                            <span class="sheet-tab-title">"Magia & Combate"</span>
                            <span class="sheet-tab-page-tag">"Pág. 2"</span>
                        </button>

                        <button 
                            class="sheet-tab-btn"
                            class:active=move || active_tab.get() == SheetPageTab::Expanded
                            on:click=move |_| set_active_tab.set(SheetPageTab::Expanded)
                            title="Página 3: Antecedentes Expandidos, Posses, Focos, Grimório e Capela"
                        >
                            <span class="sheet-tab-icon">"🏛️"</span>
                            <span class="sheet-tab-title">"Antecedentes & Posses"</span>
                            <span class="sheet-tab-page-tag">"Pág. 3"</span>
                        </button>

                        <button 
                            class="sheet-tab-btn"
                            class:active=move || active_tab.get() == SheetPageTab::HistoryVisuals
                            on:click=move |_| set_active_tab.set(SheetPageTab::HistoryVisuals)
                            title="Página 4: História, Objetivos, Descrição Física, Avatar, Cabal e Retrato"
                        >
                            <span class="sheet-tab-icon">"👤"</span>
                            <span class="sheet-tab-title">"História & Visual"</span>
                            <span class="sheet-tab-page-tag">"Pág. 4"</span>
                        </button>
                    }.into_view()
                }}
            </div>
        </nav>
    }
}
