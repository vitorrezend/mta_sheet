use leptos::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SheetPageTab {
    Main,
    MagicCombat,
    Profile,
}

#[component]
pub fn SheetTabs(
    active_tab: ReadSignal<SheetPageTab>,
    set_active_tab: WriteSignal<SheetPageTab>,
) -> impl IntoView {
    view! {
        <nav class="sheet-tabs-container" aria-label="Páginas da Ficha">
            <div class="sheet-tabs-nav">
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
                    class:active=move || active_tab.get() == SheetPageTab::Profile
                    on:click=move |_| set_active_tab.set(SheetPageTab::Profile)
                    title="Página de Perfil: Retrato, História e Anotações Gerais"
                >
                    <span class="sheet-tab-icon">"👤"</span>
                    <span class="sheet-tab-title">"Perfil do Personagem"</span>
                    <span class="sheet-tab-page-tag">"Perfil"</span>
                </button>
            </div>
        </nav>
    }
}
