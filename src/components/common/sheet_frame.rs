use leptos::*;

#[component]
pub fn CornerOrnament() -> impl IntoView {
    view! {
        <svg 
            viewBox="0 0 70 70" 
            class="corner-ornament-svg" 
            xmlns="http://www.w3.org/2000/svg" 
            aria-hidden="true"
        >
            <defs>
                <linearGradient id="m20GoldCore" x1="0%" y1="0%" x2="100%" y2="100%">
                    <stop offset="0%" stop-color="#ecd79e"/>
                    <stop offset="25%" stop-color="#c5a059"/>
                    <stop offset="50%" stop-color="#a88238"/>
                    <stop offset="75%" stop-color="#c5a059"/>
                    <stop offset="100%" stop-color="#7a5b20"/>
                </linearGradient>
                <linearGradient id="m20GoldLight" x1="0%" y1="0%" x2="100%" y2="100%">
                    <stop offset="0%" stop-color="#fff4d9"/>
                    <stop offset="50%" stop-color="#e2c887"/>
                    <stop offset="100%" stop-color="#b08b3c"/>
                </linearGradient>
                <linearGradient id="m20GoldDark" x1="0%" y1="0%" x2="100%" y2="100%">
                    <stop offset="0%" stop-color="#99752f"/>
                    <stop offset="100%" stop-color="#4d370f"/>
                </linearGradient>
            </defs>

            // 1. Bloco chanfrado dourado base (borda externa)
            <polygon points="0,0 70,0 70,7 28,7 7,28 7,70 0,70" fill="url(#m20GoldCore)" />

            // 2. Filete externo com brilho (hairline)
            <path d="M70,10 L31,10 L10,31 L10,70" fill="none" stroke="url(#m20GoldLight)" stroke-width="1.5" />

            // 3. FAIXA BRANCA DIAGONAL MARCANTE M20 (corta o canto a 45 graus)
            <polygon points="36,7 44,7 7,44 7,36" fill="#ffffff" stroke="url(#m20GoldDark)" stroke-width="0.75" />

            // 4. Segunda faixa branca chanfrada fina
            <polygon points="46,7 50,7 7,50 7,46" fill="#ffffff" stroke="url(#m20GoldDark)" stroke-width="0.5" />

            // 5. Linha dourada intermediária entre as faixas brancas e os filetes
            <line x1="53" y1="7" x2="7" y2="53" stroke="url(#m20GoldLight)" stroke-width="1" />

            // 6. FILETES TRIPLOS INTERNOS QUE SE CONECTAM ÀS BORDAS DA PÁGINA (sem losango, sem invasão de texto)
            <path d="M70,20 L38,20 L20,38 L20,70" fill="none" stroke="url(#m20GoldCore)" stroke-width="1.75" />
            <path d="M70,23 L41,23 L23,41 L23,70" fill="none" stroke="url(#m20GoldLight)" stroke-width="1" />
            <path d="M70,26 L44,26 L26,44 L26,70" fill="none" stroke="url(#m20GoldDark)" stroke-width="0.75" opacity="0.85" />
        </svg>
    }
}

#[component]
pub fn FrameHorizontalLine() -> impl IntoView {
    view! {
        <svg viewBox="0 0 100 28" preserveAspectRatio="none" class="frame-line-svg" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <linearGradient id="m20GoldCoreH" x1="0%" y1="0%" x2="0%" y2="100%">
                    <stop offset="0%" stop-color="#ecd79e"/>
                    <stop offset="25%" stop-color="#c5a059"/>
                    <stop offset="50%" stop-color="#a88238"/>
                    <stop offset="75%" stop-color="#c5a059"/>
                    <stop offset="100%" stop-color="#7a5b20"/>
                </linearGradient>
            </defs>
            <rect x="0" y="0" width="100" height="7" fill="url(#m20GoldCoreH)" />
            <line x1="0" y1="10" x2="100" y2="10" stroke="#e2c887" stroke-width="1.5" />
            <line x1="0" y1="20" x2="100" y2="20" stroke="#a88238" stroke-width="1.75" />
            <line x1="0" y1="23" x2="100" y2="23" stroke="#e2c887" stroke-width="1" />
            <line x1="0" y1="26" x2="100" y2="26" stroke="#4d370f" stroke-width="0.75" opacity="0.85" />
        </svg>
    }
}

#[component]
pub fn FrameVerticalLine() -> impl IntoView {
    view! {
        <svg viewBox="0 0 28 100" preserveAspectRatio="none" class="frame-line-svg" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <linearGradient id="m20GoldCoreV" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" stop-color="#ecd79e"/>
                    <stop offset="25%" stop-color="#c5a059"/>
                    <stop offset="50%" stop-color="#a88238"/>
                    <stop offset="75%" stop-color="#c5a059"/>
                    <stop offset="100%" stop-color="#7a5b20"/>
                </linearGradient>
            </defs>
            <rect x="0" y="0" width="7" height="100" fill="url(#m20GoldCoreV)" />
            <line x1="10" y1="0" x2="10" y2="100" stroke="#e2c887" stroke-width="1.5" />
            <line x1="20" y1="0" x2="20" y2="100" stroke="#a88238" stroke-width="1.75" />
            <line x1="23" y1="0" x2="23" y2="100" stroke="#e2c887" stroke-width="1" />
            <line x1="26" y1="0" x2="26" y2="100" stroke="#4d370f" stroke-width="0.75" opacity="0.85" />
        </svg>
    }
}

#[component]
pub fn SheetFrame() -> impl IntoView {
    view! {
        <div class="sheet-frame" aria-hidden="true">
            // Moldura perimetral de mármore dourado envelhecido
            <div class="sheet-patina-wash"></div>

            // Linha externa fina dourada
            <div class="frame-outer-hairline"></div>
            
            // 4 Cantos Art Deco M20 com proporção equilibrada (70x70)
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

            // Filetes triplos vetoriais com alinhamento pixel-perfect contínuo
            <div class="frame-border frame-border-top">
                <FrameHorizontalLine />
            </div>
            <div class="frame-border frame-border-bottom">
                <FrameHorizontalLine />
            </div>
            <div class="frame-border frame-border-left">
                <FrameVerticalLine />
            </div>
            <div class="frame-border frame-border-right">
                <FrameVerticalLine />
            </div>
        </div>
    }
}
