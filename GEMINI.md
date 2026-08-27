# 🔮 MTA Sheet — Documentação Técnica & Arquitetura

O **MTA Sheet** é uma aplicação web completa, reativa e ultra-otimizada desenvolvida em **Rust Full-Stack** para criação, gerenciamento e automação de fichas de personagens para o sistema de RPG **Mago: A Ascensão (20th Anniversary Edition - M20)** e o suplemento **Gods & Monsters (Deuses & Monstros)** do cenário *World of Darkness*.

---

## 🚀 1. Tecnologias Utilizadas (Tech Stack)

### Backend (SSR & Servidor de Aplicação)
- **Linguagem:** Rust (Edição 2024).
- **Framework Web:** [Axum 0.7](https://github.com/tokio-rs/axum) integrado ao runtime assíncrono [Tokio](https://tokio.rs/).
- **Banco de Dados:** SQLite via [SQLx 0.7](https://github.com/launchbadge/sqlx) com pool assíncrono e modo **WAL (Write-Ahead Logging)** para alta concorrência.
- **Autenticação:** Sessões criptográficas com tokens UUIDv4 e senhas protegidas com hash **Bcrypt**. Cookies com flags `HttpOnly`, `SameSite=Lax` e `Secure`.
- **Segurança de Rede:** Cabeçalhos de segurança nativos (`Content-Security-Policy`, `X-Content-Type-Options: nosniff`, `X-Frame-Options: SAMEORIGIN`, `Referrer-Policy`).

### Frontend (WASM & Reatividade)
- **Framework UI:** [Leptos 0.6](https://leptos.dev/) em modo **SSR + WASM Hydration** via `wasm-bindgen`.
- **Estilos:** CSS3 Modular e Design System responsivo com adaptação nativa para impressão/escala em folha **A4**.
- **Componentes de Entrada Estáveis:** Componentes com **Focus-Lock** (`StableTextArea`, `StableTextInput`, `ValueField`) para eliminar saltos de cursor e engasgos de frame durante digitações longas.

### Infraestrutura & Deploy
- **Containerização:** Docker Multi-Stage Build em **Alpine Linux 3.20** com binário estático compilado via `musl`.
- **Tamanho da Imagem:** **~38.7 MB** (Consumo de RAM em repouso: **~18 MB a 30 MB**).
- **Segurança do Container:** Execução com usuário sem privilégios de root (`appuser:10001`).
- **Orquestração:** `docker-compose.yml` com persistência em volumes isolados para o banco (`/app/data`) e uploads (`/app/uploads`).

---

## 📋 2. Páginas & Rotas da Aplicação

| Rota | Descrição | Acesso |
| :--- | :--- | :--- |
| **`/`** | **Home / Hub:** Lista de personagens do usuário, criação de nova ficha, importação/exportação e feed de fichas públicas da comunidade. | Público / Autenticado |
| **`/sheet/:id`** | **Ficha Interativa do Personagem:** Interface completa de 6 páginas, controle de pontos (Criação, Bônus, XP), Dossiê Anexo e Extrato de Custos. | Proprietário / Leitura Pública |
| **`/rooms`** | **Hub de Crônicas & Salas:** Listagem de mesas ativas do Narrador e opção de ingressar via código (`MTA-XXXX`). | Autenticado |
| **`/room/:id`** | **Central da Crônica & HUD da Cabala:** Monitor de Vitalidade dos magos em tempo real, Capela da Cabala (Chantry Pool de Quintessência), Diário da Crônica e controle de Fichas Ocultas de NPCs. | Membros da Sala |
| **`/login`** | **Autenticação:** Telas de Login e Cadastro de novos jogadores/mestres. | Público |
| **`/logs`** | **Painel de Auditoria & Segurança:** Visualização em tempo real de logs de acesso (Humano vs Bot), banco de dados, requisições e erros. | **Exclusivo Administradores** |

---

## 🧙 3. Estrutura da Ficha de Personagem (`/sheet/:id`)

A ficha divide-se em **6 páginas principais** mais um **Dossiê Anexo**:

### Página 1: Atributos, Habilidades, Esferas & Vantagens
- **Cabeçalho:** Nome, Jogador, Crônica, Natureza, Essência, Comportamento, Tradição/Convenção, Conceito e Cabala.
- **Atributos (9):** Físicos (*Força, Destreza, Vigor*), Sociais (*Carisma, Manipulação, Aparência*) e Mentais (*Percepção, Inteligência, Raciocínio*).
- **Habilidades (30+):** Talentos, Perícias e Conhecimentos (com suporte a especializações e linhas customizadas).
- **Esferas da Magia (9):** Correspondência, Entropia, Forças, Vida, Matéria, Mente, Primórdio, Espírito e Tempo (com indicador de Esfera de Afinidade).
- **Vantagens & Trilhas:**
  - **Antecedentes:** Nome, especificação/label do antecedente e esferas de pontuação.
  - **Ressonância:** Dinâmico, Estático e Entrópico com labels personalizáveis.
  - **Arete & Força de Vontade:** Trilha total e controle de Força de Vontade temporária.
  - **Esteira de Quintessência & Paradoxo:** 20 posições dinâmicas com marcação de Quintessência e acúmulo de Paradoxo.
  - **Trilha de Vitalidade:** 7 níveis de dano com ciclo de tipos de dano do World of Darkness (*Contundente [/], Letal [X], Agravado [*]*).

### Página 2: Qualidades, Defeitos & Maravilhas
- Listagem dinâmica de **Qualidades (Merits)** e **Defeitos (Flaws)** com custos em pontos.
- **Maravilhas (Wonders / Talismãs):** Nome, tipo, pontuação, níveis de poder e reserva de Quintessência da maravilha.
- **Armas e Armaduras:** Estatísticas de combate, dano, alcance e penalidades.

### Página 3: Focos, Paradigmas & Posses
- **Filosofia da Magia:** Paradigmas Centrais, Práticas Mágikas e Instrumentos Essenciais.
- **Antecedentes Expandidos & Propriedades:** Santuário, Mentor, Biblioteca e Recursos.
- **Cabala & Capela (Chantry):** Estrutura e detalhes da sede da cabala.

### Página 4: História, Descrição & Visual
- **Histórico Completo:** Linha do tempo, eventos marcantes e objetivos do personagem.
- **Descrição Física:** Idade aparente, etnia, cicatrizes, vestimentas e peculiaridades.
- **Retrato / Imagem do Personagem:** Upload com validação de bytes mágicos (*Magic Bytes Security*) para PNG, JPG e WebP.

### Página 5: Grimório de Rotinas & Feitiços
- Catálogo de **Rotinas Mágikas (Rotes)** personalizadas com Esferas requeridas, nível de dificuldade, sucessos e descrição do efeito.

### Página 6: Diário de Sessões & Anotações
- Bloco de anotações livres para registros de sessões, pistas investigativas e inventário temporário.

### 📂 Dossiê do Personagem (Questionário de Criação Anexo)
- Acessível pelo botão **`📂 Dossiê`** na barra superior da ficha.
- Dividido em duas seções oficiais do livro de regras:
  - **👤 8 Perguntas para o Personagem:** Idade, Infância/Despertar, Desenvolvimento de Habilidades, Pessoas Importantes, Primeiro Encontro com a Mágika, Mentor, Relação com a Cabala e Vida Comum/Identidade Secreta.
  - **🧭 6 Dicas & Perguntas para os Jogadores sobre o Caminho:** Conceito do Caminho do Mago, Vontade do Jogador, Destino Perseguido, Visão do Destino, Manifestação do Avatar, Conflitos com o Avatar e Obstáculos na Trilha da Ascensão.
- Utiliza buffer local isolado garantindo que a rolagem permaneça fixa e sem perda de foco durante a escrita de textos longos.

---

## ⚖️ 4. Regras de Negócio & Cálculos Automatizados

### Sistema de Origem de Pontos (3 Modos de Edição)
1. **⚫ Modo Criação (Base):** Pontos iniciais atribuídos na concepção do personagem.
2. **🟣 Modo Bônus (Freebie Points):** Controle dos **15 Pontos de Bônus** iniciais:
   - *Atributos:* 5 pts por bolinha.
   - *Habilidades:* 2 pts por bolinha.
   - *Esferas:* 7 pts por bolinha.
   - *Arete:* 4 pts por bolinha (com alerta de regra se ultrapassar 3 no início).
   - *Antecedentes:* 1 pt por bolinha.
   - *Força de Vontade:* 1 pt por bolinha.
   - *Qualidades e Defeitos:* Balanço automático de custos e ganhos.
3. **🟢 Modo Experiência (XP):** Registro de evolução contínua durante as sessões da crônica.

### Extrato de Custos (`CostBreakdownModal`)
- Modal analítico que audita em tempo real todos os pontos gastos de Bônus e XP, indicando se a ficha está legal perante as regras oficiais de M20.

### Resiliência & Sanitização de Dados (`src/state/sanitization.rs`)
- Parser resiliente que aceita JSONs antigos, parciais ou corrompidos, corrigindo automaticamente limites de Arete (1-10), Força de Vontade (1-10), esteira de Quintessência (20 chars) e injetando campos padrão faltantes sem quebrar a UI.

---

## 🛡️ 5. Auditoria, Segurança & RBAC

### Motor de Detecção Humano vs Máquina / Bot (`src/logging.rs`)
- Classificação heurística em tempo real de cada requisição HTTP:
  - 🟢 **[HUMANO]:** Navegador moderno com cabeçalhos consistentes (`Sec-Fetch-Mode`, `Sec-Ch-Ua`, `Accept-Language`).
  - 🤖 **[CRAWLER]:** Robôs de busca legítimos (`Googlebot`, `Bingbot`, `DuckDuckBot`, `Yandex`, etc.).
  - 🔴 **[MÁQUINA / BOT]:** Ferramentas automatizadas e scrapers (`curl`, `python-requests`, `aiohttp`, `Go-http`, `Postman`, `Playwright`, `Selenium`, etc.).
  - ⚠️ **[SUSPEITO]:** Cabeçalhos contraditórios ou ausentes.

### Controle de Acesso de Administrador (RBAC)
- Acesso à rota `/logs` e à API `get_system_logs` restrito a contas com `is_admin = true`.
- **Configuração de Múltiplos Admins via `.env`:**
  ```env
  ADMIN_USERNAMES=vitor, mestre, co_narrador
  ```
- **Promoção Dinâmica no SQLite (Zero Downtime):**
  ```bash
  docker exec mta_sheet_app sqlite3 /app/data/mta_sheet.db "UPDATE users SET is_admin = 1 WHERE username = 'vitor';"
  ```

### Proteções Contra Abuso & Limites de Cota
- **Cota de Fichas por Conta:** Máximo de **50 fichas** por usuário em `create_sheet` e `import_sheet`.
- **Limite de Payload JSON:** Máximo de **5 MB** por ficha em `update_sheet` e `import_sheet`.
- **Limite de Upload de Imagens:** Máximo de **5 MB** por arquivo com validação rigorosa de *Magic Bytes* (PNG, JPG, WebP, GIF, SVG).

---

## 🎨 6. Design System & Padrões Visuais

- **Tema:** Dark Gothic Moderno / Arcano (*World of Darkness*).
- **Cores Principais:**
  - Fundo & Superfícies: `#0f172a` (Slate 900), `#1e293b` (Slate 800), `#ffffff` (Folha de Ficha).
  - Acentos Primários: `#6366f1` (Índigo Místico), `#3b82f6` (Azul Arcano).
  - Indicadores de Origem: `#000000` (Base), `#9333ea` (Bônus Roxo), `#16a34a` (XP Verde).
  - Alertas & Erros: `#dc2626` (Vermelho Paradoxo), `#d97706` (Âmbar Alerta).
- **Escala de Impressão:** Módulo `applyMobileScale()` que adapta dinamicamente a largura da folha física A4 (793px) em telas de tablets e celulares.

---

## 🧪 7. Qualidade de Código & Testes

O projeto conta com **63 testes automatizados** divididos em suítes especializadas:
- `tests/access_detection_test.rs`: Testes unitários do classificador de tráfego (Humano vs Bot).
- `tests/admin_auth_test.rs`: Testes de autenticação, migração SQLite e permissões de administrador.
- `tests/quiz_data_test.rs`: Testes de inicialização, preservação de respostas e resolução síncrona do Dossiê.
- `tests/security_limits_test.rs`: Testes das cotas de 50 fichas, 5MB JSON e validação de Magic Bytes em imagens.
- `tests/anti_patterns_test.rs`: Testes de conformidade arquitetural (bloqueia loops reativos, closures instáveis e destruição indevida do DOM).

---

## 📦 8. Scripts & Automação de Workflows (`scripts/`)

O projeto segue a filosofia **Local-First**: todo o ciclo de desenvolvimento e testes ocorre localmente no host (com feedback em ~2s), deixando o Docker exclusivamente como empacotador final de produção.

| Script | Finalidade | Comando (Windows / Linux) |
| :--- | :--- | :--- |
| **`dev.bat` / `dev.sh`** | Inicia o ambiente de desenvolvimento local com hot-reload (Leptos + WASM + Axum) no perfil **Dev** (`opt-level=0`, debug info total). Aceita argumento `build` para apenas compilar sem watcher. | `.\scripts\dev.bat` / `./scripts/dev.sh` |
| **`build_release.bat` / `build_release.sh`** | Compila o binário standalone e o frontend WASM com otimizações máximas (`release`, `opt-level='z'`, LTO, strip), gerando o executável final autocontido. | `.\scripts\build_release.bat` / `./scripts/build_release.sh` |
| **`docker.bat` / `docker.sh`** | Constrói a imagem Docker multi-stage ou executa a aplicação via Docker Compose em container. | `.\scripts\docker.bat` / `./scripts/docker.sh` |
