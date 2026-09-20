use leptos::*;
use leptos_meta::*;

const GITHUB_REPO_URL: &str = "https://github.com/vitorrezend/mta_sheet";
const GITHUB_ISSUES_URL: &str = "https://github.com/vitorrezend/mta_sheet/issues";
const GITHUB_CLONE_CMD: &str = "git clone https://github.com/vitorrezend/mta_sheet.git";

#[component]
pub fn AboutPage() -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default();

    let (copied_cmd, set_copied_cmd) = create_signal(false);

    let on_copy_clone = move |_| {
        #[cfg(target_arch = "wasm32")]
        if let Some(window) = web_sys::window() {
            let _ = window.navigator().clipboard().write_text(GITHUB_CLONE_CMD);
            set_copied_cmd.set(true);
            set_timeout(
                move || {
                    set_copied_cmd.set(false);
                },
                std::time::Duration::from_secs(3),
            );
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = set_copied_cmd;
        }
    };

    view! {
        <Title text="Sobre & Colaboração | MTA Sheet" />
        <Meta
            name="description"
            content="MTA Sheet é uma plataforma de código aberto (MIT) desenvolvida em Rust para criação e automação de fichas de Mago: A Ascensão (M20). Saiba como colaborar com o projeto."
        />

        <div class="about-container">
            // Cabeçalho da Página
            <header class="about-header">
                <div class="about-header-badge">"PROJETO OPEN SOURCE • LICENÇA MIT"</div>
                <h1 class="about-title">
                    <span class="about-title-icon">"🔮"</span>
                    "Sobre o MTA Sheet & Colaboração"
                </h1>
                <p class="about-subtitle">
                    {move || crate::i18n::tr("about_subtitle", lang())}
                </p>
            </header>

            // Card Principal do GitHub
            <section class="github-showcase-card">
                <div class="github-card-header">
                    <div class="github-brand-left">
                        <span class="github-brand-icon" aria-hidden="true">
                            // Ícone SVG limpo do GitHub
                            <svg width="32" height="32" viewBox="0 0 24 24" fill="currentColor">
                                <path fill-rule="evenodd" clip-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.53 1.032 1.53 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" />
                            </svg>
                        </span>
                        <div>
                            <h2 class="github-repo-title">"vitorrezend / mta_sheet"</h2>
                            <p class="github-repo-desc">
                                "Repositório oficial no GitHub com código-fonte completo, suíte de testes e documentação técnica."
                            </p>
                        </div>
                    </div>
                    <div class="github-badges-row">
                        <span class="tech-tag tag-rust">"🦀 Rust 2024"</span>
                        <span class="tech-tag tag-leptos">"⚡ Leptos 0.6"</span>
                        <span class="tech-tag tag-sqlite">"🗄️ SQLite / SQLx"</span>
                        <span class="tech-tag tag-mit">"⚖️ Licença MIT"</span>
                    </div>
                </div>

                <div class="github-clone-row">
                    <span class="clone-label">"Clonar Repositório:"</span>
                    <div class="clone-code-box">
                        <code>{GITHUB_CLONE_CMD}</code>
                        <button
                            type="button"
                            class="clone-copy-btn"
                            on:click=on_copy_clone
                            title="Copiar comando de clone"
                        >
                            {move || if copied_cmd.get() { "✅ Copiado!" } else { "📋 Copiar" }}
                        </button>
                    </div>
                </div>

                <div class="github-actions-row">
                    <a
                        href=GITHUB_REPO_URL
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn-github-action btn-github-primary"
                    >
                        <span>"⭐"</span>
                        <span>"Acessar Repositório no GitHub"</span>
                    </a>
                    <a
                        href=GITHUB_ISSUES_URL
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn-github-action btn-github-secondary"
                    >
                        <span>"🐛"</span>
                        <span>"Reportar Problema ou Ideia (Issues)"</span>
                    </a>
                </div>
            </section>

            // O Que é o Projeto & Visão Geral
            <section class="about-section">
                <h3 class="section-title">"📖 Sobre o Projeto"</h3>
                <div class="about-text-block">
                    <p>
                        "O " <strong>"MTA Sheet"</strong> " nasceu da paixão pelo cenário de " <em>"Mago: A Ascensão"</em> " (World of Darkness) e da constatação de que fichas em papel ou planilhas genéricas muitas vezes sobrecarregam narradores e jogadores com a complexa matemática do sistema: contagem estrita de 15 pontos de bônus, progressão de Arete, custos variáveis de esferas por afinidade, esteiras de Quintessência e Paradoxo, além das peculiaridades do suplemento " <em>"Deuses & Monstros"</em> "."
                    </p>
                    <p>
                        "A aplicação foi concebida sob os princípios de " <strong>"Local-First"</strong> ", " <strong>"eficiência de recursos"</strong> " e " <strong>"zero anúncios"</strong> ". O software pode ser executado inteiramente offline ou em servidores próprios (homelab), gerando uma experiência fluida tanto em computadores quanto em celulares e exportação fiel para impressão em padrão folha A4."
                    </p>
                </div>
            </section>

            // Como Contribuir (Pilares de Colaboração)
            <section class="about-section">
                <h3 class="section-title">"🤝 Como Você Pode Contribuir"</h3>
                <p class="section-lead">
                    "Toda contribuição — seja técnica, textual ou de regras — ajuda a manter e expandir o MTA Sheet como uma ferramenta aberta para a comunidade de RPG."
                </p>

                <div class="collab-grid">
                    <div class="collab-card">
                        <div class="collab-card-icon">"🐛"</div>
                        <h4 class="collab-card-title">"Testes & Relato de Falhas"</h4>
                        <p class="collab-card-desc">
                            "Encontrou algum cálculo incorreto de pontos de bônus, erro na progressão de XP, falha visual em impressão ou problema em navegadores móveis? Abrir uma Issue no GitHub nos ajuda a identificar e corrigir rapidamente."
                        </p>
                    </div>

                    <div class="collab-card">
                        <div class="collab-card-icon">"💻"</div>
                        <h4 class="collab-card-title">"Código & Engenharia"</h4>
                        <p class="collab-card-desc">
                            "O projeto é construído em Rust puro (Axum + Leptos + SQLite via SQLx) e CSS modular. Se você programa em Rust ou trabalha com front-end reativo e acessibilidade, Pull Requests bem testados são sempre bem-vindos."
                        </p>
                    </div>

                    <div class="collab-card">
                        <div class="collab-card-icon">"📜"</div>
                        <h4 class="collab-card-title">"Regras, Lore & Grimório"</h4>
                        <p class="collab-card-desc">
                            "Ajude a enriquecer o compêndio canônico com rotinas mágicas (Rotes), armas brancas e de fogo, instrumentos, práticas herméticas e artes marciais (Dô) fiéis aos livros da Edição de 20º Aniversário (M20)."
                        </p>
                    </div>

                    <div class="collab-card">
                        <div class="collab-card-icon">"🌐"</div>
                        <h4 class="collab-card-title">"Internacionalização & Textos"</h4>
                        <p class="collab-card-desc">
                            "Auxilie na revisão e clareza dos termos em Português ou contribua na expansão e refinamento das traduções em Inglês no nosso módulo de internacionalização nativo."
                        </p>
                    </div>
                </div>
            </section>

            // Diretrizes para Desenvolvedores
            <section class="about-section">
                <h3 class="section-title">"🛠️ Para Desenvolvedores"</h3>
                <div class="dev-guide-card">
                    <h4 class="dev-guide-title">"Começando em 3 Passos:"</h4>
                    <ol class="dev-steps-list">
                        <li>
                            <strong>"1. Instale os pré-requisitos:"</strong>
                            <span>" Rust 1.80+ (edição 2024), " <code>"rustup target add wasm32-unknown-unknown"</code> " e " <code>"cargo install wasm-bindgen-cli --version 0.2.121 cargo-leptos"</code> "."</span>
                        </li>
                        <li>
                            <strong>"2. Execute localmente:"</strong>
                            <span>" Use " <code>"cargo leptos watch"</code> " (ou " <code>".\\scripts\\dev.bat"</code> " / " <code>"./scripts/dev.sh"</code> ") para desenvolvimento com hot-reload."</span>
                        </li>
                        <li>
                            <strong>"3. Execute os testes antes do PR:"</strong>
                            <span>" Valide com " <code>"cargo test --features ssr"</code> " para garantir que todas as suítes de regras e integridade estão aprovadas."</span>
                        </li>
                    </ol>
                </div>
            </section>

            // Aviso Legal / Fan Content
            <footer class="about-legal-footer">
                <h4 class="legal-title">"🌌 Aviso Legal de Propriedade Intelectual"</h4>
                <p class="legal-text">
                    "Partes dos materiais utilizados são marcas registradas e direitos autorais da " <strong>"Paradox Interactive AB"</strong> ", utilizados com permissão. Todos os direitos reservados. Para mais informações, consulte " <a href="https://www.worldofdarkness.com" target="_blank" rel="noopener noreferrer">"worldofdarkness.com"</a> "."
                </p>
                <p class="legal-text">
                    "Este é um projeto comunitário independente, sem fins comerciais ou lucrativos, mantido como ferramenta de suporte gratuito para a comunidade de RPG de mesa."
                </p>
            </footer>
        </div>
    }
}
