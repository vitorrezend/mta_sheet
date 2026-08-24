# 🧙 MTA Sheet — Mage: The Ascension 20th Anniversary Edition

[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg?style=flat&logo=rust)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/Leptos-0.6-blue.svg?style=flat)](https://leptos.dev/)
[![SQLite](https://img.shields.io/badge/SQLite-sqlx-003B57.svg?style=flat&logo=sqlite)](https://sqlite.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**MTA Sheet** é uma plataforma web fullstack em **Rust** para gerenciamento de fichas de personagens do RPG **Mago: A Ascensão (Edição de 20º Aniversário - M20)** e **Gods & Monsters**.

O sistema é 100% autocontido, com renderização no servidor (SSR), hidratação em WebAssembly (WASM), banco de dados SQLite embarcado e empacotamento em **arquivo executável único standalone** para Linux e Windows.

---

## 📑 Sumário

- [Destaques e Funcionalidades](#-destaques-e-funcionalidades)
- [Execução Rápida (Sem Compilar)](#-execução-rápida-sem-compilar)
  - [No Windows](#no-windows)
  - [No Linux](#no-linux)
- [Configuração de Banco de Dados (Já Pré-Configurado)](#-configuração-de-banco-de-dados-já-pré-configurado)
- [Como Compilar e Gerar Executáveis](#-como-compilar-e-gerar-executáveis)
  - [1. Compilar para Linux (Executável Único)](#1-compilar-para-linux-executável-único)
  - [2. Compilar para Windows (.exe Standalone)](#2-compilar-para-windows-exe-standalone)
- [Ambiente de Desenvolvimento (Modo Watch / Hot-Reload)](#-ambiente-de-desenvolvimento-modo-watch--hot-reload)
- [Estrutura do Projeto](#-estrutura-do-projeto)
- [Testes Automatizados e Resiliência](#-testes-automatizados-e-resiliência)

---

## ✨ Destaques e Funcionalidades

### 📜 Ficha Completa M20 (6 Páginas Diagramadas em Formato A4)
1. **Página 1 (Principal)**:
   - Atributos (Físicos, Sociais, Mentais) e Habilidades (Talentos, Perícias, Conhecimentos).
   - Vantagens: Esferas com **Seletor de Esfera de Afinidade (⭐ Estrela Dourada)**.
   - **Força de Vontade Flexível**: Evolução além do 5º ponto com Pontos de Bônus (1 pt) ou XP (Nível × 1), menu de contexto no botão direito e pílulas de extrato.
   - Vitalidade com cascata de dano WoD (Contusivo, Letal, Agravado).
2. **Página 2 (Magia & Combate)**:
   - Qualidades & Defeitos (com cálculo de pontos de bônus e XP).
   - Outras Características e Armas Brancas / Fogo / Armaduras.
   - **Maravilhas (Wonders 1–20 pts)**: Nível de Arete, Quintessência em grid dinâmico, upload de fotos com Lightbox.
3. **Página 3 (Antecedentes Expandidos & Capela)**:
   - Detalhamento de Aliados, Recursos, Mentor, Biblioteca, etc.
   - Gestão de Capela (*Chantry*) e Posses / Equipamentos.
4. **Página 4 (História, Descrição & Avatar)**:
   - Descrição visual, histórico narrativo, Arquétipos (Natureza/Comportamento), Diagrama de Cabala, Croqui e Avatar.
5. **Página 5 (📖 Grimório)**:
   - *Hero Banner* com Paradigma Central, Foco e Filosofia.
   - Listas dinâmicas de Práticas Mágicas e Instrumentos Mágicos.
   - **Cards de Rotinas Mágicas (Rotes)** com seletor interativo de 9 esferas, tags roxas removíveis, Habilidade Realçando e **cálculo automático de dificuldades M20** (Coincidente +2, Vulgar +3, Vulgar c/ Testemunha +4).
6. **Página 6 (📝 Notas & Diário de Campanha)**:
   - Blocos de texto expansíveis em tela cheia com proteção de digitação fluida (`StableTextArea`).
   - **Anexo de Imagens e Mapas de até 10MB** com validação de bytes e visualizador Lightbox full-screen.

### 🐾 Outros Recursos
- **Ficha Especial Gods & Monsters (2 Páginas)**: Para companheiros, familiares, construtos, fadas, vampiros e ciborgues.
- **Salas de Jogo em Tempo Real (Mesas de Crônica)**: Criação de salas com chat de dados e sincronização de fichas.
- **Exportação e Importação JSON**: Backup completo de fichas com integridade e validação de schema.
- **Exportação para PDF Vetorial Oficial**: Botão de impressão com CSS vetorial A4 para exportar todas as páginas diagramadas.
- **Auto-Save Inteligente**: Salvamento automático com debounce de 1.2s e salvamento imediato ao trocar de página.

---

## ⚡ Execução Rápida (Sem Compilar)

### No Windows
1. Copie o arquivo [`mta_sheet.exe`](./mta_sheet.exe) para qualquer pasta no seu computador.
2. Dê **dois cliques no arquivo** (ou execute via Prompt de Comando / PowerShell):
   ```cmd
   .\mta_sheet.exe
   ```
   *Ou execute através do script utilitário:*
   ```cmd
   .\scripts\run_release.bat
   ```
3. Abra seu navegador em: **`http://localhost:3000`**.

> **Nota**: O executável é **100% estático e autocontido**. Não requer a instalação de Node.js, Python, Rust, nem DLLs externas do MinGW/GCC.

### No Linux
1. Execute o binário pré-compilado na raiz:
   ```bash
   chmod +x ./mta_sheet
   ./mta_sheet
   ```
   *Ou execute através do script utilitário:*
   ```bash
   ./scripts/run_release.sh
   ```
2. Abra seu navegador em: **`http://localhost:3000`**.

---

## 🗄️ Configuração de Banco de Dados (Já Pré-Configurado)

O MTA Sheet utiliza **SQLite embarcado via SQLx** com migrações automáticas:

- **Zero Configuração Inicial**: Por padrão, ao iniciar, o executável cria automaticamente o arquivo `mta_sheet.db` na pasta onde está rodando, estruturando todas as tabelas e índices necessários.
- **Variáveis de Ambiente Suportadas** (opcional via arquivo `.env` ou variáveis do sistema):

```env
# Caminho do banco de dados SQLite (Padrão: sqlite:mta_sheet.db)
DATABASE_URL=sqlite:mta_sheet.db

# Endereço e porta do servidor (Padrão: 0.0.0.0:3000)
LEPTOS_SITE_ADDR=0.0.0.0:3000

# Diretório para armazenamento local de imagens/anexos (Padrão: ./uploads)
UPLOADS_DIR=./uploads

# Segredo para autenticação e sessões (Opcional, possui fallback seguro)
AUTH_SECRET=seu_segredo_super_seguro
```

---

## 🔨 Como Compilar e Gerar Executáveis

### Pré-requisitos de Compilação
- **Rust (versão 1.80+)**: [https://rustup.rs/](https://rustup.rs/)
- **Target WebAssembly**: `rustup target add wasm32-unknown-unknown`
- **wasm-bindgen-cli** e **cargo-leptos**: `cargo install wasm-bindgen-cli --version 0.2.121 cargo-leptos`

---

### 1. Compilar para Linux (Executável Único)

Gera um binário compilado nativamente para Linux com todos os assets (HTML, WASM, CSS, JS) embutidos:

```bash
./scripts/build_release.sh
```
*Gera o arquivo `./mta_sheet` na raiz.*

---

### 2. Compilar para Windows (.exe Standalone)

Gera um executável standalone otimizado para Windows com todos os assets embutidos:

```cmd
.\scripts\build_release.bat
```
*Gera o arquivo `.\mta_sheet.exe` na raiz.*

---

## 💻 Ambiente de Desenvolvimento (Modo Watch / Hot-Reload)

Para programar com recompilação automática ao salvar arquivos:

### No Linux:
```bash
./scripts/dev.sh
# ou diretamente:
cargo leptos watch
```

### No Windows:
```cmd
.\scripts\dev.bat
# ou diretamente:
cargo leptos watch
```

O servidor iniciará em `http://127.0.0.1:3000` com recarga automática de abas no navegador.

---

## 📂 Estrutura do Projeto

```
mta_sheet/
├── src/
│   ├── components/
│   │   ├── common/             # Componentes reutilizáveis (ValueField, StableTextArea, Navbar, Export/Import)
│   │   ├── gods_and_monsters/  # Ficha de 2 páginas Gods & Monsters
│   │   ├── mta_sheet/          # Ficha Oficial M20 (Páginas 1 a 6)
│   │   │   ├── page1/          # Atributos, Habilidades, Esferas, Vontade, Vitalidade
│   │   │   ├── page2/          # Magia, Combate, Qualidades/Defeitos, Maravilhas
│   │   │   ├── page3/          # Antecedentes Expandidos, Posses, Capela
│   │   │   ├── page4/          # História, Descrição, Cabala, Avatar, Croqui
│   │   │   ├── page5/          # Grimório: Paradigma, Práticas, Instrumentos e Rotinas
│   │   │   ├── page6/          # Notas de Sessão, Diário e Anexo de Imagens 10MB
│   │   │   └── sheet/          # TopBar, Abas A4, Modal de Extrato de Gastos
│   │   ├── profile/            # Perfil do Personagem e Gestão
│   │   ├── rooms/              # Mesas de Jogo e Chat em Tempo Real
│   │   └── views/              # Telas (Home/Dashboard, Ficha, Login, Logs)
│   ├── database.rs             # Conexão SQLite e migrações automáticas
│   ├── logging.rs              # Logger estruturado e painel /logs
│   ├── compliance_tests.rs     # Testes estáticos de conformidade anti-panic
│   └── state/                  # Modelos, regras de custo M20, sanitização e Server Functions
├── styles/                     # CSS Modular dividido em 11 domínios estéticos
├── scripts/                    # Scripts utilitários de build, dev e commit (Windows e Linux)
│   ├── dev.bat / dev.sh                    # Inicia ambiente de desenvolvimento com hot-reload
│   ├── build_release.bat / build_release.sh# Gera binário release standalone otimizado
│   ├── run_release.bat / run_release.sh    # Executa o binário release com configurações
│   └── commit.bat / commit.ps1             # Automação de commit assistido e push
└── Cargo.toml                  # Configurações do Rust, dependências e profiles
```

---

## 🧪 Testes Automatizados e Resiliência

O projeto possui **testes automatizados** cobrindo:
- **Regras Oficiais M20**: Cálculos de custos de Pontos de Bônus e Pontos de Experiência (Arete, Esferas, Atributos, Habilidades, Maravilhas, Qualidades/Defeitos).
- **Fuzzing & Sanitização Resiliente**: Tolerância a falhas e recuperação automática de JSONs corrompidos ou legados.
- **Segurança de Mídia**: Validação de Magic Bytes contra injeção de arquivos maliciosos em uploads de imagens.
- **Conformidade Reativa & Anti-Panic**: Proibição estática de `.unwrap()` em código de produção e proteção de scopes de sinais (`.try_set()`).

Para rodar toda a suíte de testes:
```bash
cargo test
```

---

## 📄 Licença

Distribuído sob a licença [MIT](LICENSE).
