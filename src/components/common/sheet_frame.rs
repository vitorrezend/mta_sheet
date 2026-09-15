use leptos::*;

#[component]
pub fn CornerOrnament() -> impl IntoView {
    view! {
        <svg 
            viewBox="0 0 105 105" 
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

            // 1. Bloco chanfrado dourado base
            <polygon points="0,0 105,0 105,7 28,7 7,28 7,105 0,105" fill="url(#m20GoldCore)" />

            // 2. Filete externo com brilho
            <path d="M105,10 L31,10 L10,31 L10,105" fill="none" stroke="url(#m20GoldLight)" stroke-width="1.5" />

            // 3. FAIXA BRANCA DIAGONAL MARCANTE M20 (corta a moldura em 45 graus)
            <polygon points="40,7 48,7 7,48 7,40" fill="#ffffff" stroke="url(#m20GoldDark)" stroke-width="0.75" />

            // 4. Faixa dourada sólida chanfrada
            <polygon points="49,7 57,7 7,57 7,49" fill="url(#m20GoldCore)" />

            // 5. Segunda faixa branca chanfrada
            <polygon points="58,7 63,7 7,63 7,58" fill="#ffffff" stroke="url(#m20GoldDark)" stroke-width="0.5" />

            // 6. PIRÂMIDE / TRIÂNGULO ART DECO M20 apontando para o centro da folha
            // Triângulo dourado externo
            <polygon points="66,7 7,66 42,42" fill="url(#m20GoldCore)" stroke="url(#m20GoldDark)" stroke-width="1" />
            
            // Faixa branca interna do triângulo
            <polygon points="73,11 11,73 47,47" fill="#ffffff" stroke="url(#m20GoldDark)" stroke-width="0.75" />
            
            // Triângulo médio dourado
            <polygon points="79,15 15,79 50,50" fill="url(#m20GoldCore)" stroke="url(#m20GoldLight)" stroke-width="0.75" />
            
            // Linha branca fina concêntrica
            <polygon points="85,19 19,85 53,53" fill="none" stroke="#ffffff" stroke-width="1.25" />

            // 7. LOSANGO CENTRAL LAPIDADO M20
            <polygon points="53,53 59,59 53,65 47,59" fill="url(#m20GoldLight)" stroke="url(#m20GoldDark)" stroke-width="1" />
            <circle cx="53" cy="59" r="1.5" fill="#3b2b0e" />

            // 8. FILETES TRIPLOS INTERNOS QUE SE CONECTAM ÀS BORDAS DA PÁGINA
            <path d="M105,20 L37,20 L20,37 L20,105" fill="none" stroke="url(#m20GoldCore)" stroke-width="1.75" />
            <path d="M105,23 L40,23 L23,40 L23,105" fill="none" stroke="url(#m20GoldLight)" stroke-width="1" />
            <path d="M105,26 L42,26 L26,42 L26,105" fill="none" stroke="url(#m20GoldDark)" stroke-width="0.75" opacity="0.85" />
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
            
            // 4 Cantos Art Deco M20 com peso marcante
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

            // Filetes triplos dourados que unem os cantos
            <div class="frame-border frame-border-top"></div>
            <div class="frame-border frame-border-bottom"></div>
            <div class="frame-border frame-border-left"></div>
            <div class="frame-border frame-border-right"></div>
        </div>
    }
}
