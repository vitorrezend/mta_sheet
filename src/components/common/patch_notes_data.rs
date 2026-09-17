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
        version: "v0.18.0",
        date: "2026-09-17",
        tag: "v0.18.0",
        title: "Compêndio M20 Expandido, Arsenal Completo (Brancas & Fogo), Aceleração da Home e Estabilidade WASM",
        highlight: "Lançamento do Compêndio Oficial M20 integrado com arsenal completo de Armas Brancas e de Fogo/À Distância, carregamento da Home até 15x mais rápido com summary_json, segurança com mascaramento de tokens de sessão nos logs e resolução definitiva de sincronização WebAssembly.",
        sections: &[
            PatchSection {
                category: "Compêndio Oficial M20 & Arsenal Completo",
                icon: "⚔️",
                items: &[
                    "🏹 Tabela Completa de Armas de Fogo e Longo Alcance: Adicionado suporte integral a pistolas, fuzis, escopetas, submetralhadoras, arcos e bestas com estatísticas oficiais M20 (Dano Letal, Alcance, Cadência/Rate, Pente/Clip e Ocultação).",
                    "🗡️ Arsenal de Armas Brancas & Corpo a Corpo: Catálogo unificado com cálculo automático de dano por Força, dificuldades e tipos de dano (Contundente/Letal).",
                    "🎯 Seletor de Especialidades Reutilizável: Componente SpecialtyPicker desacoplado para seleção rápida de especialidades em Atributos, Habilidades e Combate.",
                    "📖 Botão '📖 M20' e Harmonização Visual: Estilo refinado em ouro arcano para o grimório (Página 5) e compêndio de práticas e instrumentos.",
                ],
            },
            PatchSection {
                category: "Performance & Arquitetura de Dados",
                icon: "⚡",
                items: &[
                    "🚀 Aceleração da Home (15x mais rápida): Implementada coluna summary_json no banco de dados SQLite com migração automática, eliminando a leitura pesada de megabytes de JSON na tela inicial.",
                    "📦 Migração Preguiçosa & Resiliente: Atualização automática e transparente de fichas legadas em segundo plano ao serem consultadas.",
                ],
            },
            PatchSection {
                category: "Segurança & Proteção de Credenciais",
                icon: "🛡️",
                items: &[
                    "🔒 Mascaramento de Tokens de Sessão: Eliminação do risco de vazamento de credenciais nos logs do servidor através da ofuscação dos tokens de sessão.",
                    "🚫 Prevenção Contra Submissão Fallback: Garantia de integridade do formulário de autenticação na SPA sem expor credenciais na URL.",
                ],
            },
            PatchSection {
                category: "Estabilidade WebAssembly & Clean Architecture",
                icon: "🧩",
                items: &[
                    "⚙️ Sincronização Contínua de WASM: Correção definitiva de LinkError no WebAssembly com sincronização bidirecional entre mta_sheet.wasm e mta_sheet_bg.wasm.",
                    "🎨 CSS Modular (12-compendium.css): Separação de mais de 1.400 linhas de estilos do compêndio em folha dedicada, otimizando o CSS do Grimório.",
                    "♻️ Componente BoundAttributeField & Sinais Unificados: Reutilização limpa de campos de atributos e gestão atômica dos estados dos modais.",
                ],
            },
        ],
    },
    PatchRelease {
        version: "v0.17.0",
        date: "2026-09-15",
        tag: "v0.17.0",
        title: "Aura Visual Autêntica M20: Moldura Art Deco, Divisores Dourados e Tipografia Clássica",
        highlight: "Recriação fiel da estética original do livro físico de Mago: A Ascensão (20th Anniversary Edition) com moldura ornamental vetorial, cantos Art Deco de 70px com faixas chanfradas, divisores dourados triplos sem caixotes cinzas, tipografia clássica Cinzel/Garamond e arquitetura anti-patterns aprimorada.",
        sections: &[
            PatchSection {
                category: "Moldura Ornamental & Cantos Art Deco M20",
                icon: "🏛️",
                items: &[
                    "✨ Cantos Art Deco Vetoriais (70px × 70px): Redesenhados com geometria de precisão, faixas brancas chanfradas a 45º e filetes triplos que abraçam os cantos sem invadir os campos de texto.",
                    "📏 Conectores Vetoriais Pixel-Perfect: Barras horizontais e verticais em SVG unificam os 4 cantos de ponta a ponta sem degraus, quebras ou desníveis.",
                    "📜 Pátina Perimetral de Ouro Velho: Suave vinheta de pergaminho antigo e mármore dourado nos 26px das margens perimetrais com hairline exterior, preservando a pureza e legibilidade do miolo da página.",
                    "🖱️ Usabilidade 100% Preservada: Moldura e ornamentos com cliques transparentes ('pointer-events: none'), mantendo bolinhas, inputs e modais totalmente acessíveis.",
                ],
            },
            PatchSection {
                category: "Divisores Dourados Triplos & Tipografia",
                icon: "✒️",
                items: &[
                    "💎 Fim dos Caixotes Cinzas: Eliminação completa das caixas retangulares estilo formulário web nos blocos principais (Atributos, Habilidades, Esferas, Vantagens).",
                    "⚜️ Divisores Triplos com Losangos: Cada seção agora conta com um divisor triplo horizontal em ouro antigo ladeado por diamantes lapidados (◆ ATRIBUTOS ◆).",
                    "📖 Tipografia Clássica Serifada: Importação da fonte 'Cinzel' com fallback nativo elegante para 'Garamond' e 'Georgia', títulos de colunas em âmbar nobre (#4a3e2c) e sublinhado duplo clássico.",
                ],
            },
            PatchSection {
                category: "Estabilidade Reativa & Qualidade de Código",
                icon: "🛡️",
                items: &[
                    "🎯 Correção de NodeRef em Modais: Eliminação do aviso de 'NodeRef already been filled' no input de especialização do modal sobrenatural através de Focus-Lock desacoplado.",
                    "🧪 Regra 11 de Anti-Patterns: Nova validação arquitetural automatizada em tests/anti_patterns_test.rs garantindo que nenhum NodeRef estático seja associado dentro de closures dinâmicas.",
                    "✅ 118 Testes Automatizados: Suíte completa aprovada com zero warnings no backend Axum e frontend WASM.",
                ],
            },
        ],
    },
    PatchRelease {
        version: "v0.16.0",
        date: "2026-09-15",
        tag: "v0.16.0",
        title: "Atributos Sobrenaturais (6-10), Focus-Lock e Resolução de Tradição nos Cards",
        highlight: "Nova interface compacta para traços sobrenaturais (níveis 6 a 10) com painel flutuante dedicado e identificação de traço, campo de especialização com Focus-Lock e correção definitiva da pílula de Tradição e Essência nos cards.",
        sections: &[
            PatchSection {
                category: "Atributos & Habilidades Sobrenaturais (6-10)",
                icon: "✦",
                items: &[
                    "💎 Design Minimalista e Compacto: Traços sobrenaturais permanecem 100% ocultos em níveis mortais (0-4), eliminando truncamento de nomes de atributos.",
                    "⚡ Ativação Inteligente no Nível 5: Separador e losango místico clicável surgem apenas ao completar o quinto ponto, abrindo o painel de expansão épica.",
                    "🔮 Painel Flutuante com Escala de Losangos: Controle dedicado de 6 a 10 (+1 a +5) com seleção direta, botão de reset e menu de origem de pontos.",
                    "🏷️ Identificação Clara do Traço: Cabeçalho do modal destaca o nome do atributo/habilidade sendo editado e badge dinâmica de nível total.",
                    "✍️ Modificador / Especialização Integrada: Campo de texto curto para especialização sobrenatural no modal com Focus-Lock (salvamento seguro no blur ou fechamento).",
                    "🎨 Badges de Alto Contraste: Quando recolhido, o traço sobrenatural exibe botão ultra-compacto (+1 a +5) com cores sólidas indicativas da origem dos pontos (Bônus roxo, XP esmeralda, Buff dourado, Base ardósia).",
                ],
            },
            PatchSection {
                category: "Sincronização de Tradição & Cards",
                icon: "🃏",
                items: &[
                    "🏷️ Resolução de Tradição e Essência: Resolutores resilientes eliminam o problema onde o card inicial mostrava sempre 'Tradição não definida', sincronizando chaves internas e acentuadas.",
                    "🏰 Suporte Unificado em Salas: A visão da mesa e membros da cabala agora exibe com fidelidade a Tradição e Essência dos magos e o Tipo dos seres de Gods & Monsters.",
                    "💬 Tooltip de Leitura: Adicionado atributo title nas tags de metadados dos cards para visualização imediata do texto completo.",
                ],
            },
            PatchSection {
                category: "Qualidade & Testes",
                icon: "🧪",
                items: &[
                    "✅ 115 Testes Automatizados: Expansão da suíte de testes cobrindo limites 6-10, compatibilidade de esquemas, SSR e persistência de apelidos de traços.",
                    "🛡️ Zero Warnings: Compilação limpa tanto no backend Axum quanto no frontend WASM Leptos.",
                ],
            },
        ],
    },
    PatchRelease {
        version: "v0.15.0",
        date: "2026-09-11",
        tag: "v0.15.0",
        title: "Arquitetura em Camadas, Clean Code e Sincronização Definitiva de Nome",
        highlight: "Refatoração estrutural completa do backend em camadas desacopladas (servidor, modelos e regras puras), empacotamento otimizado de CSS de 1 única requisição e sincronização bidirecional do nome do personagem em todos os cards, salas e exportações.",
        sections: &[
            PatchSection {
                category: "Sincronização de Personagem & Cards",
                icon: "🧙",
                items: &[
                    "🏷️ Resolução Definitiva de Nome: O nome digitado na Página 1 agora é sincronizado atomicamente entre o rótulo e a raiz do personagem, eliminando o problema onde os cards ficavam fixos em 'Novo Mago'.",
                    "🃏 Cards da Home & Mesas Atualizados: Os resumos dos cards e a visualização da cabala na mesa de crônica agora refletem fielmente o nome real do personagem.",
                    "🌐 Título Dinâmico na Aba: A aba do navegador agora exibe dinamicamente '{Nome do Personagem} | MTA Sheet'.",
                    "📦 Exportação Fiel de Arquivos: Downloads de JSON e nomes de anexos agora levam o nome real do personagem.",
                ],
            },
            PatchSection {
                category: "Arquitetura em Camadas & Clean Code",
                icon: "🏛️",
                items: &[
                    "🚀 Desacoplamento HTTP (src/server/): Servidor refatorado em handlers dedicados (auth, media, static_files, sse) e middlewares de segurança, reduzindo main.rs de ~780 para ~79 linhas.",
                    "📦 Domínio Modular (src/state/models/): Modelo de dados desmembrado em módulos coesos (keys, traits, items, pages, dossier, summary, character).",
                    "⚖️ Motor Puro de Regras (src/rules/): Regras de vitalidade M20, esteira de quintessência/paradoxo e fórmulas de iniciativa isoladas em módulos puros e testáveis.",
                    "🧹 Eliminação de Duplicação (DRY): Centralização do mapeamento de resumos de fichas com blindagem contra vazamento de erros de banco para o cliente.",
                ],
            },
            PatchSection {
                category: "Performance & Infraestrutura",
                icon: "⚡",
                items: &[
                    "🎨 Empacotamento de CSS (Bundle): Build de release agora concatena arquivos modulares em um único mta_sheet.css, eliminando cascata de 13 requisições HTTP.",
                    "🛡️ Controle de Cache Anti-LinkError: Middleware de segurança força no-cache em modo dev, impedindo dessincronia de cache entre binários WASM e scripts JS.",
                    "🔄 Gerenciamento Concorrente de Canais: Limpeza inteligente de canais SSE inativos em salas, prevenindo panics e vazamento de memória.",
                ],
            },
        ],
    },
    PatchRelease {
        version: "v0.14.5",
        date: "2026-09-09",
        tag: "v0.14.5",
        title: "Características Sobrenaturais (6ª Bolinha em Losango) e Internacionalização Completa de Deuses & Monstros",
        highlight: "Implementação da 6ª bolinha em formato de losango (| ◇) para características sobrenaturais em atributos e habilidades (Mago e Deuses & Monstros), acompanhada da internacionalização completa (PT-BR e EN-US) da ficha de Deuses & Monstros.",
        sections: &[
            PatchSection {
                category: "Características Sobrenaturais",
                icon: "💎",
                items: &[
                    "💎 6ª Bolinha em Losango (| ◇): Atributos e habilidades agora suportam expansão para a 6ª bolinha sobrenatural, separada por barra vertical (|) e exibida como losango para preservar a identidade visual clássica de 5 bolinhas.",
                    "⚡ Atalho Ágil no Hover: Ao passar o mouse sobre o campo, um losango fantasma (◇) surge permitindo ativar a característica sobrenatural com apenas 1 clique direto, além do menu de contexto com botão direito.",
                    "🐉 Suporte Unificado: Disponível tanto na ficha padrão de Mago: A Ascensão quanto na ficha de Deuses & Monstros, com persistência segura no banco de dados e exportação em PDF.",
                ],
            },
            PatchSection {
                category: "Internacionalização (i18n)",
                icon: "🌐",
                items: &[
                    "🌐 Suporte Bilíngue em Deuses & Monstros (PT-BR / EN-US): A ficha de Gods & Monsters agora reage instantaneamente ao seletor de idioma na barra de navegação.",
                    "🔄 Tradução Bi-direcional de Atributos e Habilidades: Mecanismo resiliente em tr_attr e tr_ability que traduz chaves canônicas em português ou inglês sem alterar as referências de banco de dados.",
                    "⚔️ Páginas e Seções Traduzidas: Cabeçalho, vantagens (Encantos, Dons, Gnose, Vantagens Especiais, Paradoxo, Essência), tabela de combate, qualidades & defeitos, histórico, descrição e abas de navegação 100% traduzidos.",
                ],
            },
        ],
    },
    PatchRelease {
        version: "v0.14.4",
        date: "2026-09-09",
        tag: "v0.14.4",
        title: "Entrega de Fichas para Membros da Sala e Preview de Construção no Battle Grid",
        highlight: "Correção crítica na clonagem e entrega de fichas do Narrador para jogadores na sala com preservação do Dossiê/Questionário, além de preview holográfico na colocação de estruturas no grid tático de batalha.",
        sections: &[
            PatchSection {
                category: "Salas de Jogo & Crônica",
                icon: "🧙",
                items: &[
                    "🎁 Entrega de Fichas para Jogadores: Correção no endpoint clone_and_assign_sheet_to_member para operar corretamente na tabela character_sheets, sincronizando o novo UUID interno e associando automaticamente o personagem à crônica.",
                    "📂 Clonagem de Respostas do Dossiê: Ao entregar uma ficha para um membro, todas as respostas salvas do questionário de criação (character_quiz_answers) são replicadas para a nova ficha.",
                    "🛡️ Proteção de Cota do Jogador: Validação rigorosa garantindo que o jogador destinatário não exceda o limite de 50 fichas ativas na sua conta.",
                ],
            },
            PatchSection {
                category: "Grid Tático & Battle Grid",
                icon: "🗺️",
                items: &[
                    "🧱 Preview Fantasma de Construção: Exibição holográfica semitransparente com ícone e borda tracejada indicando exatamente onde a estrutura selecionada (parede, porta, cobertura, etc.) será posicionada antes do clique.",
                    "🎯 Otimização de Eventos no Grid: Camada de preview e estruturas isolada com pointer-events otimizados, eliminando engasgos de clique na colocação e remoção de elementos.",
                ],
            },
        ],
    },
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
