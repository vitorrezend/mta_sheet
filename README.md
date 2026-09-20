# 🧙 MTA Sheet — Mage: The Ascension 20th Anniversary Edition

<p align="center">
  <a href="./README.md"><strong>🇧🇷 Português</strong></a> •
  <a href="./README.en.md"><strong>🇺🇸 English</strong></a>
</p>

[![Rust](https://img.shields.io/badge/Rust-1.80%2B%20(Edi%C3%A7%C3%A3o%202024)-orange.svg?style=flat&logo=rust)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/Leptos-0.6-blue.svg?style=flat)](https://leptos.dev/)
[![Axum](https://img.shields.io/badge/Axum-0.7-black.svg?style=flat&logo=tokio)](https://github.com/tokio-rs/axum)
[![SQLite](https://img.shields.io/badge/SQLite-sqlx-003B57.svg?style=flat&logo=sqlite)](https://sqlite.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**MTA Sheet** é uma plataforma web fullstack, reativa e ultra-otimizada desenvolvida em **Rust** para criação, gerenciamento e automação de fichas de personagens para o clássico RPG de mesa **Mago: A Ascensão (Edição de 20º Aniversário — M20)** e o suplemento **Gods & Monsters (Deuses & Monstros)** do cenário *World of Darkness*.

O projeto é **100% livre, de código aberto (Open Source sob licença MIT) e comunitário**, projetado sob a filosofia **Local-First**, renderização no servidor (**SSR**), hidratação em **WebAssembly (WASM)** e persistência em **SQLite embarcado com modo WAL**. Pode ser executado em container Docker ou empacotado como um **único arquivo executável standalone** para Windows e Linux.

---

## 🔮 Por Que o MTA Sheet Existe?

*Mago: A Ascensão* é mundialmente aclamado por seu sistema de magia flexível e profunda narrativa filosófica sobre a Guerra pela Realidade (o confronto milenar entre as **9 Tradições Místicas**, a **União Tecnocrática** e os **Dispares**). 

No entanto, o sistema de regras do M20 é notório por sua elevada complexidade matemática e exigência de cálculos contínuos:
- **Alocação Rígida de Pontos Iniciais & 15 Pontos de Bônus:** Custos distintos para Atributos (5), Habilidades (2), Esferas (7), Arete (4), Antecedentes (1) e Força de Vontade (1).
- **Esfera de Afinidade:** Influencia custos de evolução e afinidades paradigmáticas.
- **Fórmulas de Experiência (XP):** Cálculos exponenciais escalonados (*Nível Atual × Fator do Traço*).
- **Esteira Dupla de Quintessência & Paradoxo:** Trilha dinâmica de 20 posições com absorção de paradoxo e queima de quintessência.
- **Dificuldades Dinâmicas no Grimório:** Cálculo automático de dificuldade de feitiços e rotinas (*Coincidente +2, Vulgar +3, Vulgar com Testemunhas +4*).
- **Suplemento Deuses & Monstros:** Fichas dedicadas para Familiares, Espíritos, Bygones e Construtos.

O **MTA Sheet** automatiza todo esse cálculo em tempo real sem interferir na liberdade criativa do jogador, auditando a legalidade da ficha e eliminando planilhas desorganizadas e anotações rasuradas em papel.

---

## 📑 Sumário

- [Destaques e Funcionalidades](#-destaques-e-funcionalidades)
- [Arquitetura & Engenharia](#-arquitetura--engenharia)
- [Execução Rápida (Sem Compilar)](#-execução-rápida-sem-compilar)
  - [No Windows](#no-windows)
  - [No Linux](#no-linux)
- [Execução via Docker & Docker Compose](#-execução-via-docker--docker-compose)
- [Configuração de Banco de Dados & Ambiente](#-configuração-de-banco-de-dados--ambiente)
- [Como Compilar o Projeto do Zero](#-como-compilar-o-projeto-do-zero)
  - [Pré-requisitos](#pré-requisitos)
  - [Ambiente de Desenvolvimento (Hot-Reload)](#ambiente-de-desenvolvimento-hot-reload)
  - [Compilar Executável Standalone Release (Windows / Linux)](#compilar-executável-standalone-release)
- [Testes Automatizados & Qualidade](#-testes-automatizados--qualidade)
- [Estrutura de Pastas do Projeto](#-estrutura-de-pastas-do-projeto)
- [🤝 Como Colaborar com o Projeto](#-como-colaborar-com-o-projeto)
- [📄 Licença & Aviso Legal](#-licença--aviso-legal)

---

## ✨ Destaques e Funcionalidades

### 📜 Ficha Canônica M20 (6 Páginas Diagramadas em Padrão A4)
1. **Página 1 (Atributos, Habilidades, Esferas & Vitalidade)**:
   - 9 Atributos (Físicos, Sociais, Mentais) e 30+ Habilidades (Talentos, Perícias, Conhecimentos).
   - As 9 Esferas da Magia com seletor de **Esfera de Afinidade (⭐)**.
   - Vantagens, Antecedentes dinâmicos e Ressonância (Dinâmica, Estática, Entrópica).
   - Arete (1–10), Força de Vontade temporária/permanente, esteira de 20 pontos de Quintessência/Paradoxo.
   - Trilha de Vitalidade com ciclo oficial do World of Darkness: *Contusivo [/]*, *Letal [X]*, *Agravado [\*]*.
2. **Página 2 (Magia, Combate, Qualidades & Defeitos)**:
   - Qualidades & Defeitos com balanço de Pontos de Bônus e XP.
   - **Maravilhas (Wonders / Talismãs 1–20 pts)** com reserva de Quintessência e anexos visuais.
   - Armas Brancas, Armas de Fogo e Armaduras com cálculo de dano, alcance e penalidades.
3. **Página 3 (Antecedentes Expandidos, Posses & Capela)**:
   - Detalhamento de Aliados, Recursos, Mentor, Biblioteca e Santuário.
   - Gestão de Capela (*Chantry*) e inventário expandido com itens carregados e possuídos.
4. **Página 4 (História, Descrição, Cabala & Avatar)**:
   - Histórico narrativo, descrição física completa, croqui/avatar e upload de retrato com validação de segurança (*Magic Bytes*).
5. **Página 5 (📖 Grimório de Rotinas & Filosofia)**:
   - Paradigmas Centrais, Práticas Mágicas e Instrumentos Essenciais.
   - **Rotinas Mágicas (Rotes)** com seleção de esferas e cálculo automático de dificuldade M20 (Coincidente +2, Vulgar +3, Vulgar com Testemunhas +4).
6. **Página 6 (📝 Notas de Sessão & Diário)**:
   - Anotações livres em tela cheia com proteção de digitação fluida (`StableTextArea`), pistas de investigação e anexo de imagens e mapas de até 10MB.

### 🌐 Recursos de Comunidade, Compartilhamento & Crônica
- **Feed Comunitário (`/feed`)**: Descubra fichas públicas da comunidade, com sistema de **Curtidas (Likes)** reativo e contagem em tempo real.
- **Perfis de Usuário (`/user/:username`)**: Vitrine de fichas públicas, estatísticas e cronologia do jogador.
- **Compartilhamento Flexível (Estilo Google Drive)**:
  - Visibilidade Pública ou Privada.
  - Compartilhamento por Link Único com token de acesso.
  - Lista de Controle de Acesso (**ACL**) para conceder permissões nominais de *Leitura* ou *Edição* a usuários específicos ou salas.
- **Mesas & Salas de Jogo (`/rooms`)**:
  - Monitor HUD de vitalidade dos magos em tempo real.
  - Capela da Cabala (*Chantry Pool* de Quintessência compartilhado).
  - Diário de sessão e rolagem de dados integrados.
- **Dossiê do Personagem (`📂 Dossiê`)**: Questionário canônico oficial com 8 perguntas para o personagem e 6 diretrizes para os jogadores sobre o Caminho do Despertar.
- **Extrato de Custos (`CostBreakdownModal`)**: Auditoria analítica em tempo real da legalidade da ficha perante as regras oficiais (15 Pontos de Bônus e XP).
- **Exportação & Backup**:
  - Exportação e importação de arquivos `.json` com integridade garantida.
  - Impressão oficial vetorial e exportação para **PDF** diagramado em folha A4 física.

---

## 🏛️ Arquitetura & Engenharia

O MTA Sheet foi desenvolvido visando máxima eficiência de recursos, segurança estrita e zero dependências de serviços externos:

| Camada | Tecnologia | Destaques |
| :--- | :--- | :--- |
| **Backend / SSR** | Rust 2024 + Axum 0.7 + Tokio | Renderização inicial no servidor para indexação SEO e First Contentful Paint ultrarrápido. |
| **Frontend / Reatividade** | Leptos 0.6 + WASM | Reatividade granular em WebAssembly sem Virtual DOM, proporcionando 60 FPS estáveis. |
| **Banco de Dados** | SQLite 3 via SQLx 0.7 (Modo WAL) | Pool assíncrono com migrações automáticas embarcadas e integridade referencial em cascata. |
| **Estilos & UI** | CSS3 Modular + Design Tokens | Design System sombrio/arcano (*World of Darkness*), com escala dinâmica para impressão A4. |
| **Segurança & Auditoria** | Classificador Humano vs Bot | Tokens UUIDv4, senhas Bcrypt, validação de Magic Bytes em mídia e headers CSP nativos. |
| **Empacotamento** | Standalone Single-Binary / Docker | Binário único com todos os assets embutidos via `rust-embed` (~38 MB em repouso). |

---

## ⚡ Execução Rápida (Sem Compilar)

Se você já possui o binário standalone compilado:

### No Windows
1. Execute o arquivo `mta_sheet.exe` com dois cliques ou pelo terminal:
   ```cmd
   .\mta_sheet.exe
   ```
2. Acesse seu navegador em: **`http://localhost:3000`**.

### No Linux
1. Dê permissão de execução e inicie o binário:
   ```bash
   chmod +x ./mta_sheet
   ./mta_sheet
   ```
2. Acesse seu navegador em: **`http://localhost:3000`**.

> **Nota:** O executável é **100% estático e autocontido**. Não requer a instalação de Node.js, Python ou bibliotecas externas.

---

## 🐳 Execução via Docker & Docker Compose

A forma recomendada para hospedar o MTA Sheet em um servidor de produção ou homelab:

1. **Clone o repositório:**
   ```bash
   git clone https://github.com/vitorrezend/mta_sheet.git
   cd mta_sheet
   ```

2. **Configure o arquivo de ambiente:**
   ```bash
   cp .env.example .env
   ```
   *(Ajuste o `AUTH_SECRET` e `ADMIN_USERNAMES` se desejar)*

3. **Suba os containers com Docker Compose:**
   ```bash
   docker compose up -d --build
   ```

4. **Pronto!** O serviço estará respondendo em `http://localhost:3000`.
   - Os dados do banco SQLite ficam salvos com segurança no volume persistente `./data`.
   - As imagens enviadas pelos jogadores ficam salvas no volume `./uploads`.

---

## 🗄️ Configuração de Banco de Dados & Ambiente

O MTA Sheet utiliza **SQLite embarcado com SQLx**. Ao iniciar, o servidor cria automaticamente o arquivo do banco de dados e executa todas as migrações de esquemas, tabelas e índices.

### Variáveis de Ambiente Suportadas (`.env`):

```env
# URL de conexão com o SQLite (Padrão: sqlite:data/mta_sheet.db?mode=rwc)
DATABASE_URL=sqlite:data/mta_sheet.db?mode=rwc

# Endereço e porta do servidor web (Padrão: 0.0.0.0:3000)
LEPTOS_SITE_ADDR=0.0.0.0:3000

# Diretório para upload de fotos e imagens dos personagens (Padrão: ./uploads)
UPLOADS_DIR=./uploads

# Segredo criptográfico para proteção de sessões e tokens (Obrigatório em produção)
AUTH_SECRET=defina_uma_chave_longa_e_aleatoria_aqui

# Usuários que recebem privilégios de administrador (separados por vírgula)
ADMIN_USERNAMES=vitor,narrador
```

---

## 🔨 Como Compilar o Projeto do Zero

### Pré-requisitos

Para compilar o MTA Sheet a partir do código-fonte, instale:

1. **Rust (versão 1.80 ou superior — Edição 2024)**:
   - Instale via [rustup.rs](https://rustup.rs/).
2. **Target WebAssembly**:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. **CLI tools do ecossistema Leptos**:
   ```bash
   cargo install wasm-bindgen-cli --version 0.2.121
   cargo install cargo-leptos
   ```

---

### Ambiente de Desenvolvimento (Hot-Reload)

Para programar com recarga rápida automática ao alterar código Rust ou arquivos CSS:

#### No Windows:
```cmd
.\scripts\dev.bat
# ou diretamente:
cargo leptos watch
```

#### No Linux:
```bash
./scripts/dev.sh
# ou diretamente:
cargo leptos watch
```

O servidor iniciará em `http://127.0.0.1:3000` com *live-reload* no navegador.

---

### Compilar Executável Standalone Release

Gera um binário compilado nativamente com todos os assets estáticos (HTML, WASM, CSS e fontes) embutidos no executável:

#### No Windows (.exe standalone):
```cmd
.\scripts\build_release.bat
```
*O executável `mta_sheet.exe` otimizado será gerado na raiz do projeto.*

#### No Linux (binário ELF standalone):
```bash
./scripts/build_release.sh
```
*O executável `mta_sheet` será gerado na raiz do projeto.*

---

## 🧪 Testes Automatizados & Qualidade

O projeto conta com mais de 60 testes automatizados garantindo a estabilidade de regras de regras, banco de dados e interface.

Execute a suíte completa de testes no host:
```bash
cargo test --features ssr
```

Ou execute testes de suítes específicas:
```bash
# Teste do mecanismo de curtidas e feed público
cargo test --features ssr --test sheet_likes_test

# Teste da página informativa Sobre & Colaboração
cargo test --features ssr --test about_page_test

# Teste de conformidade de regras de M20 (criação e custos)
cargo test --features ssr --test creation_points_test

# Teste de conformidade arquitetural e anti-panic
cargo test --features ssr --test anti_patterns_test
```

---

## 📂 Estrutura de Pastas do Projeto

```
mta_sheet/
├── src/
│   ├── components/             # Componentes reativos da interface (Leptos)
│   │   ├── common/             # Navbar, modais, inputs estáveis, controles
│   │   ├── mta_sheet/          # Ficha Oficial M20 (Páginas 1 a 6 e TopBar)
│   │   ├── gods_and_monsters/  # Ficha de Deuses & Monstros
│   │   ├── compendium/         # Compêndio de Armas, Dô, Práticas, Instrumentos e Antecedentes
│   │   ├── rooms/              # Mesas de jogo, Battle Grid e HUD em tempo real
│   │   └── views/              # Páginas completas (Home, Feed, Perfil, Salas, Sobre, Login, Logs)
│   ├── state/                  # Modelos de dados, regras de custos M20 e Server Functions
│   ├── repositories/           # Camada de acesso a dados SQLite (SQLx)
│   ├── server/                 # Handlers Axum, middleware de segurança, SSE e arquivos estáticos
│   ├── auth.rs                 # Autenticação segura por cookie/sessão e Bcrypt
│   ├── database.rs             # Inicialização do banco SQLite, tabelas e migrações
│   └── logging.rs              # Auditoria em tempo real e classificação Humano vs Bot
├── styles/                     # CSS modular baseado em Design Tokens semânticos
├── data/                       # Arquivos JSON de compêndio e banco de dados SQLite
├── scripts/                    # Scripts utilitários de build, execução e backup
├── tests/                      # Suítes de integração e conformidade técnica
├── Cargo.toml                  # Dependências, perfis e configurações do Leptos
└── docker-compose.yml          # Orquestração de containers para produção
```

---

## 🤝 Como Colaborar com o Projeto

O **MTA Sheet** é um projeto de código aberto mantido com dedicação pela comunidade de RPG de mesa. Toda contribuição é bem-vinda!

### Onde Acompanhar o Código
- 🌐 **Repositório Oficial no GitHub**: [github.com/vitorrezend/mta_sheet](https://github.com/vitorrezend/mta_sheet)
- ✨ **Página Informativa na Aplicação**: Acesse `/about` (ou `/colabore`) diretamente na aplicação web.

### Formas de Ajudar:
1. 🐛 **Relatar Falhas & Sugestões**:
   - Encontrou um bug em alguma rolagem, cálculo de pontos de bônus, erro visual ou falha na exportação de PDF? [Abra uma Issue no GitHub](https://github.com/vitorrezend/mta_sheet/issues) detalhando o ocorrido.
2. 💻 **Contribuições com Código (Rust & WASM)**:
   - Faça um Fork do repositório, crie uma branch (`git checkout -b feature/minha-melhoria`), teste localmente com `cargo test --features ssr` e submeta um Pull Request.
3. 📜 **Lore, Regras & Grimório**:
   - Ajude a expandir o compêndio canônico com rotinas mágicas (*Rotes*), armas, antecedentes, práticas herméticas e artes marciais (Dô).
4. 🌐 **Traduções & Internacionalização**:
   - Auxilie na revisão dos termos em Português e na expansão do suporte a Inglês (`src/i18n.rs`).
5. 🎨 **Design & Usabilidade**:
   - Sugestões para aprimorar a experiência em celulares, tablets e na impressão física.

---

## 📄 Licença & Aviso Legal

Este software é livre e licenciado sob a [Licença MIT](LICENSE). Você pode utilizá-lo, modificá-lo, distribuí-lo e hospedá-lo livremente.

### 🌌 Aviso de Conteúdo de Fãs (World of Darkness)
*Partes dos materiais utilizados são marcas registradas e direitos autorais da **Paradox Interactive AB**, utilizados com permissão. Todos os direitos reservados. Para mais informações, consulte [worldofdarkness.com](https://www.worldofdarkness.com).*

Este é um projeto comunitário independente, gratuito e sem fins lucrativos, desenvolvido por fãs para fãs como ferramenta de apoio a jogadores e narradores de **Mago: A Ascensão (M20)**. Não possui afiliação, patrocínio ou endosso comercial da Paradox Interactive AB ou White Wolf Entertainment.
