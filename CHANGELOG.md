# 📜 Registro de Alterações (Patch Notes & Changelog)

Todas as modificações notáveis no projeto **MTA Sheet** são documentadas neste arquivo seguindo o padrão de [Versionamento Semântico (SemVer)](https://semver.org/).

---

## [v0.14.1] - 2026-08-29

### 🚀 Melhorias & Compatibilidade
- **🐧 Suporte Nativo a Ambientes Linux:** Atualização e travamento estrito do `wasm-bindgen` para `=0.2.121` em `Cargo.toml`, scripts de automação (`scripts/dev.sh`, `scripts/build_release.sh`) e Dockerfile.
- **⚡ Resolução de Divergência de Schema:** Eliminação de erros de incompatibilidade de versão de schema entre o Rust Wasm e o CLI local ao rodar `cargo leptos watch` ou `./scripts/dev.sh` no Linux.
- **📦 Build Standalone Linux:** Scripts de compilação release (`./scripts/build_release.sh`) totalmente funcionais em distribuições Linux.


## [v0.11.0] - 2026-08-27

### 🚀 Novas Funcionalidades (Features)
- **🎁 Clonagem & Entrega de Fichas pelo Mestre:** Narradores agora podem duplicar qualquer ficha de seu inventário e atribuí-la diretamente a um jogador da sala através do botão *🎁 Entregar Ficha* com modal de seleção interativo.
- **⚡ Login & Cadastro Estático Híbrido:** Formulários com atributos HTML nativos e rotas `/api/form_login` e `/api/form_register` permitindo autenticação instantânea mesmo em conexões lentas antes do WebAssembly inicializar.
- **🖼️ Upload Direto Multipart de Imagens:** Endpoint `/api/upload_image` via `FormData` e `gloo-net`, reduzindo o consumo de memória RAM do navegador e acelerando uploads de fotos de perfil e retratos.
- **📥 Streaming Nativo de Exportação de Fichas:** Endpoint `GET /api/export_json/:id` para download direto de arquivos JSON sem sobrecarregar buffers de memória JavaScript no cliente.
- **🎨 Seletor de Fichas de Alto Contraste:** Melhoria de contraste visual no seletor de tipos de ficha (Mago vs Gods & Monsters).

### 🐛 Correções de Bugs (Bug Fixes)
- **Correção de Exportação WebAssembly em Release:** Adição da flag `--lib` e sincronização do `wasm-bindgen` para garantir que `export function hydrate()` seja sempre gerado corretamente.
- **Segurança de Descarte de Escopo Reativo:** Prevenção de acessos a sinais descartados em modais com `try_get_untracked` e `on_cleanup`.
- **Regra Global Anti-Pattern 7:** Verificação automatizada em tempo de build para prevenir memory leaks em listeners de janela.

---

## [v0.10.0] - 2026-08-25

### 🚀 Novas Funcionalidades (Features)
- **👥 HUD da Cabala & Monitor de Vitalidade:** Painel em tempo real na rota /room/:id exibindo a trilha de dano calculada (*Íntegro*, *Escoriado*, *Ferido*, *Gravemente Ferido*, *Espancado*, *Estropiado*, *Aleijado*, *Incapacitado*) e recursos de cada mago (Arete, Força de Vontade, Quintessência, Paradoxo).
- **🏛️ Capela & Recursos Compartilhados (*Chantry Pool*):** Aba dedicada para gerenciar a reserva coletiva de Quintessência da Cabala com botões rápidos de incremento/decremento, nível do Nodo, Biblioteca e anotações.
- **📜 Diário da Crônica & Mural:** Aba para anotações de sessões, pistas investigativas e histórico da campanha.
- **🕶️ Fichas Ocultas de NPCs & Segredos:** Narrador e donos de fichas podem alternar a visibilidade de qualquer personagem com o botão *Ocultar/Revelar*.
- **⚡ Auto-Sync de Salas:** Atualização automática em segundo plano a cada 15 segundos para refletir alterações sem necessidade de F5.
- **📂 Dossiê do Personagem (14 Perguntas Oficiais M20):**
  - **👤 Seção 1 (8 Perguntas para o Personagem):** Idade, Quando Percebeu Que Era Diferente, Desenvolvimento de Habilidades, Quem É Importante, Primeiro Encontro com a Mágika, Mentor, Como Conheceu a Cabala e Vida Comum/Identidade Secreta.
  - **🧭 Seção 2 (6 Dicas & Perguntas para o Jogador sobre o Caminho):** Guia com o conceito do Caminho do Mago, O Que o Jogador Quer Fazer, Destino que Persegue, Visão do Destino, Natureza do Avatar, Relação com o Avatar e Conflitos ao Longo do Caminho.

### 🐛 Correções de Bugs (Bug Fixes)
- **Isomorfismo de Autenticação & Hidratação:** Resolução definitiva de panics de hidratação (`left: "DIV" != right: "HEADER"`). O estado de autenticação agora é derivado de forma isomórfica sem mutações no mount do WASM.
- **Redirecionamento Limpo de Sessão:** Login e logout utilizam redirecionamento com recarga de cookies (`window.location().set_href("/")`), prevenindo conflitos de `RefCell`.

### ⚡ Performance & Segurança (VPS Hardening)
- **8 Índices B-Tree no SQLite:** Criação de índices para lookups rápidos em character_sheets, sessions, rooms e room_members.
- **Fixação Estrita do WebAssembly:** Dependência `wasm-bindgen` travada em `=0.2.93` para garantir paridade 100% idêntica entre compilação local no Windows e no Docker de produção.
- **Cache Estático de Produção:** Cabeçalhos Cache-Control: public, max-age=86400, stale-while-revalidate=3600 para assets /pkg/, CSS e imagens.
- **Purga de Sessões & Rotação de Logs:** Limpeza automática de sessões SQLite expiradas e retenção de arquivos de log em 30 dias.
- **Políticas de Cota:** Limite de 50 fichas por conta, 5MB de JSON payload e 5MB para uploads com validação de bytes mágicos (*Magic Bytes Security*).

### 🧪 Testes Automatizados
- **65 Testes Automatizados** cobrindo autenticação, sanitização, resiliência de dados, detecção de bots, integridade de componentes reativos, regras anti-pattern de hidratação SSR/CSR e travamento de dependências.

---

## [v0.9.2] - 2026-08-20
- **Suplemento Gods & Monsters:** Suporte a criação de fichas com custos e pools de Deuses & Monstros.
- **Auditoria de Acesso:** Painel administrativo /logs com classificação de tráfego Humano vs Bot/Crawler.
- **Componentes Focus-Lock:** Implementação do StableTextArea e StableTextInput para digitação sem engasgo de frame.
