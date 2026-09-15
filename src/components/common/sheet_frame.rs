use leptos::*;

#[component]
pub fn CornerOrnament() -> impl IntoView {
    view! {
        <svg 
            viewBox="0 0 90 90" 
            class="corner-ornament-svg" 
            xmlns="http://www.w3.org/2000/svg" 
            aria-hidden="true"
        >
            <defs>
                <linearGradient id="m20GoldGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                    <stop offset="0%" stop-color="#dfc588"/>
                    <stop offset="50%" stop-color="#b89347"/>
                    <stop offset="100%" stop-color="#7d5e23"/>
                </linearGradient>
                <linearGradient id="m20GoldGradLight" x1="0%" y1="0%" x2="100%" y2="100%">
                    <stop offset="0%" stop-color="#f7ecd0"/>
                    <stop offset="60%" stop-color="#d4b368"/>
                    <stop offset="100%" stop-color="#997732"/>
                </linearGradient>
            </defs>

            // Borda externa sólida chanfrada
            <polygon points="0,0 90,0 90,3.5 16,3.5 3.5,16 3.5,90 0,90" fill="url(#m20GoldGrad)" />

            // Linha externa chanfrada com brilho
            <path d="M90,6.5 L19,6.5 L6.5,19 L6.5,90" fill="none" stroke="url(#m20GoldGradLight)" stroke-width="1.2" />

            // Faixas diagonais Art Deco em relevo
            <polygon points="26,6.5 31,6.5 6.5,31 6.5,26" fill="url(#m20GoldGrad)" />
            <polygon points="36,6.5 41,6.5 6.5,41 6.5,36" fill="url(#m20GoldGrad)" />
            <polygon points="46,6.5 50,6.5 6.5,50 6.5,46" fill="url(#m20GoldGrad)" />

            // Linha divisória intermediária
            <line x1="55" y1="6.5" x2="6.5" y2="55" stroke="url(#m20GoldGradLight)" stroke-width="1" />

            // Triângulo Art Deco central voltado para o centro da folha
            <polygon points="63,6.5 6.5,63 36,36" fill="url(#m20GoldGrad)" stroke="url(#m20GoldGradLight)" stroke-width="0.8" />
            <polygon points="69,11 11,69 41,41" fill="none" stroke="url(#m20GoldGradLight)" stroke-width="1.2" />
            <polygon points="75,15 15,75 45,45" fill="url(#m20GoldGrad)" opacity="0.9" />

            // Losango central lapidado
            <polygon points="45,45 50,50 45,55 40,50" fill="url(#m20GoldGradLight)" stroke="#664c18" stroke-width="0.8" />

            // Linhas internas que conectam ao restante da moldura
            <path d="M90,13.5 L27,13.5 L13.5,27 L13.5,90" fill="none" stroke="url(#m20GoldGrad)" stroke-width="1" opacity="0.95" />
            <path d="M90,16 L29.5,16 L16,29.5 L16,90" fill="none" stroke="url(#m20GoldGradLight)" stroke-width="0.75" opacity="0.85" />
        </svg>
    }
}

#[component]
pub fn SheetFrame() -> impl IntoView {
    view! {
        <div class="sheet-frame" aria-hidden="true">
            <div class="sheet-patina-wash"></div>
            
            // 4 Cantos Art Deco M20
            <div class="sheet-corner corner-top-left">
                <CornerOrnament />
            </div>
            <div class="sheet-corner corner-top-right">
                <CornerOrnament />
            </div>
            <div class="sheet-corner corner-bottom-left">
                <CornerOrnament />
            </div>
            <div class="sheet-corner corner-bottom-right">
                <CornerOrnament />
            </div>

            // Filetes dourados que unem os cantos
            <div class="frame-border frame-border-top"></div>
            <div class="frame-border frame-border-bottom"></div>
            <div class="frame-border frame-border-left"></div>
            <div class="frame-border frame-border-right"></div>
        </div>
    }
}
