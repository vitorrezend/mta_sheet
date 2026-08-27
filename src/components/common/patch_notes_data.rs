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
