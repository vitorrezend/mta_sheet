pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatchRelease {
    pub version: &'static str,
    pub date: &'static str,
    pub tag: &'static str,
    pub title: &'static str,
    pub highlight: &'static str,
    pub sections: &'static [PatchSection],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatchSection {
    pub category: &'static str,
    pub icon: &'static str,
    pub items: &'static [&'static str],
}

pub static PATCH_RELEASES: &[PatchRelease] = &[
    PatchRelease {
        version: "v0.14.3",
        date: "2026-09-04",
        tag: "v0.14.3",
        title: "Acesso Wi-Fi / Rede Local (0.0.0.0:3000), Executavel Standalone Autonomo e Grid de Esferas Perfeito",
        highlight: "Escuta padronizada em 0.0.0.0:3000 permitindo acesso via Wi-Fi e rede local em dev e release, inicializacao limpa e autonoma sem dependencia de Cargo.toml, upgrade cargo-leptos 0.3.7 no Windows, correcao de sintaxe no dev.bat e alinhamento uniforme das 9 esferas nos cards de sala.",
        sections: &[
            PatchSection {
                category: "Rede & Compartilhamento",
                icon: "🌐",
                items: &[
                    "🌐 Acesso por Wi-Fi e Rede Local (0.0.0.0:3000): Servidor e scripts de desenvolvimento (dev.bat e dev.sh) configurados para escutar em todas as interfaces de rede por padrao, permitindo acesso imediato de celulares, tablets e outros computadores na mesma rede.",
                    "📢 Mensagens Informativas de Inicializacao: O console agora indica claramente as URLs de acesso local (localhost:3000) e via rede local Wi-Fi (<SEU_IP_LOCAL>:3000).",
                    "📦 Executavel Standalone Totalmente Autonomo: O binario mta_sheet.exe agora roda silenciosamente em qualquer pasta ou computador sem depender de arquivos externos ou Cargo.toml, com frontend WASM e assets 100% embutidos.",
                ],
            },
            PatchSection {
                category: "Interface & Salas",
                icon: "🔮",
                items: &[
                    "🎯 Alinhamento Uniforme das 9 Esferas: Ajuste no Grid CSS com minmax(0, 1fr) e truncamento inteligente, garantindo que esferas com nomes longos (como Correspondencia) mantenham largura e alinhamento simetricos perfeitos.",
                    "📖 Card da Sala como Atalho Direto: Clicar em qualquer parte do card de personagem na sala de jogo abre diretamente a ficha completa (/sheet/:id), mantendo a propagacao isolada nos botoes flutuantes de acao.",
                ],
            },
            PatchSection {
                category: "Build & Ferramental de Desenvolvimento",
                icon: "⚡",
                items: &[
                    "🚀 Sincronizacao cargo-leptos 0.3.7 no Windows: Atualizacao do utilitario global e instalador automatizado no dev.bat, eliminando divergencias de schema com o wasm-bindgen 0.2.121.",
                    "🛠️ Auto-Check no build_release.bat: Verificacao inteligente de versao do wasm-bindgen-cli com atualizacao automatica em caso de versao desatualizada.",
                    "🐛 Correcao de Sintaxe no dev.bat: Resolucao de escape de parenteses em blocos if do interpretador cmd.exe do Windows.",
                ],
            },
        ],
    },
    PatchRelease {
        version: "v0.14.2",
        date: "2026-09-02",
        tag: "v0.14.2",
        title: "Scripts de Backup SQLite (WAL/VACUUM), Suporte cargo-leptos 0.3 no Linux & Padronizacao de Porta 3000",
        highlight: "Scripts de backup a quente consolidando arquivos .wal e .shm com desfragmentacao VACUUM e estrutura YYYY/MM/DD, resolucao de compatibilidade com cargo-leptos 0.3 no Linux (bin-exe-name) e padronizacao global da porta 3000.",
        sections: &[
            PatchSection {
                category: "Infraestrutura & Backups",
                icon: "💾",
                items: &[
                    "💾 Scripts de Backup Seguro SQLite (WAL/VACUUM): Scripts dedicados em scripts/backup/ para backup online veloz (.backup) e com desfragmentacao de paginas livres (VACUUM INTO), gerando snapshots consistentes e absorvendo transacoes ativas do WAL.",
                    "📁 Organizacao Cronologica YYYY/MM/DD: Criacao automatica da arvore de diretorios por ano, mes e dia com checagem rigorosa de integridade pos-geracao (PRAGMA integrity_check).",
                    "🔌 Padronizacao Global da Porta 3000: Unificacao de todas as portas e variaveis de ambiente em containers Docker, compose e scripts locais para a porta 3000.",
                ],
            },
            PatchSection {
                category: "Compatibilidade & Ambiente Linux",
                icon: "🐧",
                items: &[
                    "🐧 Resolucao cargo-leptos 0.3 no Linux: Adicao de bin-exe-name = \"mta_sheet_server\" no Cargo.toml, eliminando o erro de leitura do binario ('No such file or directory') e permitindo hot-reload instantaneo com ./scripts/dev.sh.",
                    "🧹 Limpeza de Metadados Obsoletos: Remocao de chaves nao reconhecidas (bin-package, lib-package, wasm-opt) da secao [package.metadata.leptos].",
                    "⚙️ Sincronizacao de Build Multiplataforma: Padronizacao do schema wasm-bindgen 0.2.121 entre Linux, Docker e scripts de build.",
                ],
            },
        ],
    },
    PatchRelease {
        version: "v0.14.1",
        date: "2026-08-29",
        tag: "v0.14.1",
        title: "Compatibilidade com Linux & Sincronizacao WebAssembly (wasm-bindgen 0.2.121)",
        highlight: "Atualizacao da suite de WebAssembly para o schema wasm-bindgen 0.2.121, garantindo paridade e compilacao nativa perfeita no ambiente Linux e no Docker.",
        sections: &[
            PatchSection {
                category: "Compatibilidade & Build",
                icon: "🐧",
                items: &[
                    "🐧 Suporte Nativo a Ambientes Linux: Sincronizacao do schema WebAssembly para wasm-bindgen 0.2.121, compatibilizando os scripts dev.sh e build_release.sh com o cargo-leptos no Linux.",
                    "⚡ Executavel Standalone Otimizado: Geracao de binario Linux standalone autocontido de alta performance com assets e WASM embutidos.",
                    "🧪 109 Testes Automatizados Aprovados: Cobertura total e 100% de aprovacao nas suites de testes de integridade e regras.",
                ],
            },
        ],
    },
    PatchRelease {
        version: "v0.14.0",
        date: "2026-08-29",
        tag: "v0.14.0",
        title: "Enquadramento Interativo por Arraste, Compressão WebP & Roda Circular de Paradoxo",
        highlight: "Modal interativo com drag & drop e scroll para enquadramento de fotos, compressão automática no cliente com Canvas WebP (redução de 98%), nova Roda Circular de Paradoxo e Quintessência às 9h, Módulo de Combate Rápido com tooltips e cards enriquecidos na Home e Salas.",
        sections: &[
            PatchSection {
                category: "Visual & Retratos",
                icon: "📐",
                items: &[
                    "📐 Modal Interativo de Enquadramento por Arraste: Arraste a imagem diretamente com o mouse ou toque (drag & drop) para definir a área focal visível nos cards, com suporte a scroll do mouse e atalhos rápidos (Rosto, Busto, Centro, Base).",
                    "🖼️ Compressão e Limpeza WebP no Cliente: Redimensionamento proporcional automático (800px para retratos/maravilhas, 1400px para cabalas), remoção de metadados EXIF/GPS e conversão para WebP de alta qualidade reduzindo arquivos em mais de 98%.",
                    "🗂️ Cards de Personagens Enriquecidos: Exibição do retrato enquadrado na Página Inicial e nas Salas com caixas dinâmicas de Força de Vontade e Vitalidade.",
                ],
            },
            PatchSection {
                category: "Mecânica & Ficha M20",
                icon: "🔮",
                items: &[
                    "🎡 Roda Circular de Paradoxo & Quintessência: Disposição em círculo de 20 caixas a partir das 9h, adicionando Quintessência no sentido horário e Paradoxo no sentido anti-horário com botões dedicados de controle.",
                    "⚔️ Módulo de Combate Rápido & Estatísticas Derivadas: Cálculo instantâneo de Iniciativa, Defesa, Movimento e Absorção de Dano (com regra humana de Mago) e tooltips informativos.",
                    "📄 Exportação Inteligente para PDF: Supressão de barras de rolagem e opção para ignorar páginas sem conteúdo, gerando PDFs limpos e econômicos.",
                ],
            },
            PatchSection {
                category: "Estabilidade & Desempenho",
                icon: "⚡",
                items: &[
                    "🛡️ Correção de Pânico de Hidratação: Estabilização de containers de abas (SheetTabs) e corpo da ficha para transição suave entre Mago e Gods & Monsters.",
                    "🧪 49 Testes Automatizados 100% Aprovados: Cobertura total de persistência de enquadramento, cálculos de custos, validação de segurança e conformidade arquitetural.",
                ],
            },
        ],
    },
    PatchRelease {
        version: "v0.13.0",
        date: "2026-08-27",
        tag: "v0.13.0",
        title: "Sincronização em Tempo Real (SSE) na Iniciativa, Salas Públicas & Penalidade WoD",
        highlight: "Transmissão Server-Sent Events instantânea no Tracker de Iniciativa com áudio sincronizado, cálculo automático de penalidade de dano WoD, salas públicas com senha BCrypt e autenticação SPA com feedback inline.",
        sections: &[
            PatchSection {
                category: "Tempo Real & Iniciativa",
                icon: "⚡",
                items: &[
                    "⚡ Sincronização Broadcast em Tempo Real (SSE): Endpoint /api/room_events/:id com Tokio BroadcastStream para rolagens de iniciativa, inclusão de inimigos e turnos atualizados instantaneamente para todos na mesa.",
                    "🎲 Áudio Sincronizado de Dados & Mute Individual: Reprodução de efeitos sonoros de rolagem para todos os participantes com botão discreto 🔊/🔇 e persistência no localStorage.",
                    "🩸 Mecânica WoD de Penalidade de Vitalidade: Aplicação automática da penalidade de ferimentos na iniciativa com piso mínimo de 2 pontos conforme regras oficiais do M20.",
                ],
            },
            PatchSection {
                category: "Salas & Segurança",
                icon: "🔒",
                items: &[
                    "🛡️ Salas Públicas & Proteção por Senha: Suporte a mesas abertas ou protegidas por senha criptografada via BCrypt com verificação segura.",
                    "⚙️ Aba de Configurações da Mesa: Painel exclusivo para o Narrador configurar visibilidade pública/privada, alterar senhas e gerenciar parâmetros da sala.",
                    "🔮 Autenticação 100% SPA: Feedback de login/cadastro inline com alertas animados dentro do card, sem recarregamento ou navegação de página.",
                ],
            },
            PatchSection {
                category: "Estabilidade & Testes",
                icon: "🛡️",
                items: &[
                    "🌐 Entrega Otimizada de WebAssembly: Roteamento via ServeDir nativo com streaming e cabeçalhos application/wasm sem bloqueio.",
                    "🧪 85 Testes Automatizados 100% Aprovados: Cobertura completa de SSE, penalidades de dano, senhas BCrypt, integridade WASM e regras arquiteturais.",
                ],
            },
        ],
    },
    PatchRelease {
        version: "v0.12.0",
        date: "2026-08-27",
        tag: "v0.12.0",
        title: "Catálogo Relacional de Quiz, Exportação Compacta & Alta Performance de Inputs",
        highlight: "Banco de dados relacional para questionários de criação, redução de 85% no tamanho do JSON exportado, digitação com Focus-Lock a 60 FPS e chaves imutáveis em habilidades customizadas.",
        sections: &[
            PatchSection {
                category: "Novas Funcionalidades & Arquitetura",
                icon: "🚀",
                items: &[
                    "🏛️ Catálogo Relacional do Quiz (SQLite): Tabelas dedicadas quiz_questions e character_quiz_answers com chaves estrangeiras e ON DELETE CASCADE, eliminando repetição de enunciados.",
                    "📦 Exportação Compacta de JSON (-85%): Omissão inteligente de slots e campos vazios (skip_serializing_if), gerando arquivos leves (~1.5 KB), elegantes e 100% legíveis.",
                    "🔄 Compatibilidade Retroativa Total: Deserializador expansivo que reconstrói automaticamente todas as 14 perguntas e enunciados ao importar fichas compactas ou legadas.",
                ],
            },
            PatchSection {
                category: "Performance & Interface",
                icon: "⚡",
                items: &[
                    "⚡ Digitação com Zero Latência (Focus-Lock): Inputs de texto isolados no DOM a 60 FPS com sincronização no blur, eliminando re-renderizações desnecessárias de 6 páginas ao digitar.",
                    "🎯 Correção de Foco do Cursor em Habilidades: Chaves de lista imutáveis com UUIDs persistentes, impedindo a perda de foco ao nomear novos talentos, perícias e conhecimentos.",
                    "📏 Ajuste Visual de Linhas Tracejadas: Largura otimizada para campos dinâmicos de antecedentes e ressonância.",
                ],
            },
            PatchSection {
                category: "Qualidade & Testes",
                icon: "🛡️",
                items: &[
                    "🛡️ Novas Regras de Análise Estática (Regras 8 e 9): Testes automatizados que bloqueiam preventivamente mutações globais síncronas em on:input e chaves mutáveis em <For>.",
                    "🧪 77 Testes Automatizados 100% Aprovados: Cobertura total de integridade relacional, cálculo de custos, serialização compacta e hidratação WebAssembly.",
                ],
            },
        ],
    },
    PatchRelease {
        version: "v0.11.0",
        date: "2026-08-27",
        tag: "v0.11.0",
        title: "Clonagem de Fichas do Mestre & Otimizações de Resiliência",
        highlight: "Entrega de fichas clonadas pelo Narrador aos jogadores, login estático híbrido, upload direto multipart e exportação de fichas nativa.",
        sections: &[
            PatchSection {
                category: "Novas Funcionalidades",
                icon: "🚀",
                items: &[
                    "🎁 Clonagem & Entrega de Fichas pelo Mestre: O Narrador pode duplicar qualquer ficha de seu inventário e atribuí-la diretamente a um jogador dentro da sala.",
                    "⚡ Login & Cadastro Estático Híbrido: Formulários com endpoints dedicados (/api/form_login e /api/form_register) com envio HTTP nativo imediato mesmo em conexões lentas.",
                    "🖼️ Upload Direto Multipart (/api/upload_image): Envio binário via FormData reduzindo tráfego e liberando memória RAM em celulares.",
                    "📥 Streaming Nativo de Exportação (/api/export_json/:id): Download direto de arquivos JSON sem sobrecarregar a memória do navegador.",
                    "🎨 Seletor de Criação de Fichas de Alto Contraste: Nova paleta e destaque visual nítido para escolha entre Mago e Deuses & Monstros.",
                ],
            },
            PatchSection {
                category: "Correções & Estabilidade",
                icon: "🐛",
                items: &[
                    "Correção de Exportação WebAssembly: Adição da flag --lib e sincronização estrita do wasm-bindgen garantindo inicialização de hidratação perfeita em release.",
                    "Segurança de Descarte de Escopo: Modais agora utilizam try_get_untracked com limpeza em on_cleanup, evitando acessos a sinais descartados.",
                    "Auditoria SQL Completa: 100% das consultas utilizam Prepared Statements com Parameter Binding (?), protegendo o sistema contra SQL Injection.",
                ],
            },
        ],
    },
    PatchRelease {
        version: "v0.10.0",
        date: "2026-08-25",
        tag: "v0.10.0",
        title: "Dossiê Oficial M20 & Hardening de Produção",
        highlight: "14 Perguntas Oficiais do Dossiê, HUD da Cabala em tempo real, auto-sync de salas e arquitetura de hidratação blindada.",
        sections: &[
            PatchSection {
                category: "Novas Funcionalidades",
                icon: "🚀",
                items: &[
                    "👥 HUD da Cabala & Monitor de Vitalidade: Painel em tempo real na rota /room/:id com trilha de dano calculada e monitor de Arete, Força de Vontade, Quintessência e Paradoxo.",
                    "🏛️ Capela & Recursos Compartilhados (Chantry Pool): Aba dedicada para gerenciar a reserva coletiva de Quintessência da Cabala, Nível do Nodo, Biblioteca e anotações.",
                    "📜 Diário da Crônica & Mural: Aba para anotações de sessões, pistas investigativas e histórico compartilhado da campanha.",
                    "🕶️ Fichas Ocultas de NPCs & Segredos: Narrador e donos de fichas podem alternar a visibilidade de qualquer personagem com o botão Ocultar/Revelar.",
                    "⚡ Auto-Sync de Salas: Atualização automática em segundo plano a cada 15 segundos para sincronizar as mesas sem necessidade de F5.",
                    "📂 Dossiê do Personagem (14 Perguntas Oficiais M20): Seção 1 com 8 perguntas de histórico do personagem e Seção 2 com 6 reflexões sobre o Caminho do Mago.",
                ],
            },
            PatchSection {
                category: "Correções & Hidratação",
                icon: "🐛",
                items: &[
                    "Isomorfismo de Autenticação: Resolução definitiva dos panics de hidratação no navegador através de Signal derivado estável.",
                    "Eliminação de Conflitos de Meta no Body: Folhas de estilo centralizadas no cabeçalho evitando desvio de nós DOM pelo parser do navegador.",
                    "Auto-Escala Mobile em WebAssembly: Redimensionamento responsivo de fichas A4 executado nativamente em Rust via window_event_listener.",
                ],
            },
            PatchSection {
                category: "Performance & VPS Hardening",
                icon: "⚡",
                items: &[
                    "8 Índices B-Tree no SQLite: Otimização de consultas frequentes em fichas, sessões, salas e membros.",
                    "Fixação Estrita de WebAssembly: Dependência wasm-bindgen travada em =0.2.93 para paridade exata com containers Docker.",
                    "Cache Estático de Produção: Cabeçalhos Cache-Control com stale-while-revalidate para arquivos estáticos e WASM.",
                    "Proteção de Cota & Uploads: Limites de segurança de 50 fichas por conta e validação de Magic Bytes em imagens.",
                ],
            },
        ],
    },
    PatchRelease {
        version: "v0.9.2",
        date: "2026-08-20",
        tag: "v0.9.2",
        title: "Suplemento Gods & Monsters & Auditoria",
        highlight: "Criação de fichas para Deuses & Monstros e painel administrativo de auditoria de acessos.",
        sections: &[
            PatchSection {
                category: "Novas Funcionalidades",
                icon: "🚀",
                items: &[
                    "Suplemento Gods & Monsters: Criação de fichas personalizadas com cálculos de pools para entidades e criaturas extraordinárias.",
                    "Painel de Auditoria de Acesso (/logs): Registro detalhado de tráfego com classificação automática entre visitantes humanos e bots/crawlers.",
                    "Campos Focus-Lock: Implementação dos componentes StableTextArea e StableTextInput garantindo digitação fluida sem engasgo de frame.",
                ],
            },
            PatchSection {
                category: "Segurança",
                icon: "🛡️",
                items: &[
                    "Proteção contra Brute-Force: Rate limiting em rotas de autenticação e criação de salas.",
                    "Isolamento de Sessão: Cookies HttpOnly com SameSite=Lax e cabeçalhos de segurança avançados.",
                ],
            },
        ],
    },
    PatchRelease {
        version: "v0.8.0",
        date: "2026-08-10",
        tag: "v0.8.0",
        title: "Salas de Jogo & Crônicas Multijogador",
        highlight: "Lançamento do sistema de salas de jogo, compartilhamento seguro de fichas e exportação JSON.",
        sections: &[
            PatchSection {
                category: "Novas Funcionalidades",
                icon: "🚀",
                items: &[
                    "Sistema de Salas de Jogo (/rooms): Criação de mesas de RPG protegidas por código de 6 caracteres.",
                    "Vinculação de Personagens: Jogadores podem associar suas fichas à sala do Narrador com permissões granulares.",
                    "Exportação e Importação JSON: Backup individual e restauração de fichas com sanitização automática de esquema.",
                ],
            },
        ],
    },
];

pub fn get_latest_release() -> Option<&'static PatchRelease> {
    PATCH_RELEASES.first()
}
