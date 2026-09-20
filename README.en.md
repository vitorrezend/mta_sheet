# 🧙 MTA Sheet — Mage: The Ascension 20th Anniversary Edition

<p align="center">
  <a href="./README.md"><strong>🇧🇷 Português</strong></a> •
  <a href="./README.en.md"><strong>🇺🇸 English</strong></a>
</p>

[![Rust](https://img.shields.io/badge/Rust-1.80%2B%20(2024%20Edition)-orange.svg?style=flat&logo=rust)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/Leptos-0.6-blue.svg?style=flat)](https://leptos.dev/)
[![Axum](https://img.shields.io/badge/Axum-0.7-black.svg?style=flat&logo=tokio)](https://github.com/tokio-rs/axum)
[![SQLite](https://img.shields.io/badge/SQLite-sqlx-003B57.svg?style=flat&logo=sqlite)](https://sqlite.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**MTA Sheet** is a high-performance, full-stack, reactive web platform written in **Rust** for creating, managing, and automating character sheets for the tabletop RPG **Mage: The Ascension (20th Anniversary Edition — M20)** and its companion sourcebook **Gods & Monsters** from the *World of Darkness* setting.

The project is **100% free, open-source (MIT licensed), and community-driven**. Built with a **Local-First** philosophy, it features Server-Side Rendering (**SSR**), client hydration via **WebAssembly (WASM)**, and embedded **SQLite persistence with WAL mode**. It can run effortlessly in a Docker container or be packaged as a **single, standalone binary executable** for Windows and Linux.

---

## 🔮 Why Does MTA Sheet Exist?

*Mage: The Ascension* is celebrated worldwide for its extraordinarily flexible freeform magick system and profound philosophical narrative centered around the War for Reality (the millenary conflict between the **9 Mystick Traditions**, the **Technocratic Union**, and the **Disparate Crafts**).

However, the M20 rulebook is notoriously demanding regarding mathematics and bookkeeping:
- **Strict Character Creation Budgeting:** 15 Freebie/Bonus points with distinct category costs (Attributes = 5, Abilities = 2, Spheres = 7, Arete = 4, Willpower = 1, Backgrounds = 1).
- **Affinity Sphere Rules:** Modifies progression costs and paradigm alignment.
- **Experience Point (XP) Formulas:** Scaled exponential progression (*Current Rating × Trait Multiplier*).
- **Dual Quintessence & Paradox Wheel:** Dynamic 20-point tracker handling Paradox backlash and Quintessence channeling.
- **Dynamic Casting Difficulties in the Grimoire:** Automatic rote and spell difficulty calculation (*Coincident +2, Vulgar +3, Vulgar with Witnesses +4*).
- **Gods & Monsters Supplement:** Dedicated sheets for Familiars, Spirits, Bygones, and Constructs.

**MTA Sheet** automates all of these calculations in real time without constraining player creativity, auditing sheet rule legality while eliminating messy paper erasures and disorganized spreadsheets.

---

## 📑 Table of Contents

- [Highlights & Features](#-highlights--features)
- [Architecture & Technical Design](#-architecture--technical-design)
- [Quick Run (Pre-compiled)](#-quick-run-pre-compiled)
  - [On Windows](#on-windows)
  - [On Linux](#on-linux)
- [Running via Docker & Docker Compose](#-running-via-docker--docker-compose)
- [Database & Environment Configuration](#-database--environment-configuration)
- [Building from Source](#-building-from-source)
  - [Prerequisites](#prerequisites)
  - [Development Environment (Hot-Reload)](#development-environment-hot-reload)
  - [Compiling Standalone Release Binaries](#compiling-standalone-release-binaries)
- [Automated Testing & Quality Assurance](#-automated-testing--quality-assurance)
- [Project Directory Structure](#-project-directory-structure)
- [🤝 How to Contribute](#-how-to-contribute)
- [📄 License & Legal Notice](#-license--legal-notice)

---

## ✨ Highlights & Features

### 📜 Canonical M20 Sheet (6 Pages Formatted for Physical A4)
1. **Page 1 (Attributes, Abilities, Spheres & Health)**:
   - 9 Attributes (Physical, Social, Mental) and 30+ Abilities (Talents, Skills, Knowledges).
   - All 9 Magick Spheres with an interactive **Affinity Sphere Selector (⭐)**.
   - Advantages, expandable Backgrounds, and Resonance tracks (Dynamic, Static, Entropic).
   - Arete (1–10), temporary and permanent Willpower, and a 20-point Quintessence / Paradox wheel.
   - Health track with official World of Darkness damage cycling: *Bashing [/]*, *Lethal [X]*, *Aggravated [\*]*.
2. **Page 2 (Magick, Combat, Merits & Flaws)**:
   - Merits & Flaws with automatic Freebie and XP cost balancing.
   - **Wonders & Talismans (1–20 pts)** with Quintessence reserves and visual photo attachments.
   - Melee weapons, firearms, and armor with automatic damage, range, and penalty stats.
3. **Page 3 (Expanded Backgrounds, Possessions & Chantry)**:
   - Detailed management for Allies, Resources, Mentor, Library, and Sanctum.
   - Chantry foundation management and item inventory (Carried vs. Owned gear).
4. **Page 4 (History, Description, Cabal & Avatar)**:
   - Narrative background, detailed physical description, Cabal chart, Avatar appearance, and character portrait upload with *Magic Bytes* security verification.
5. **Page 5 (📖 Grimoire of Rotes & Philosophy)**:
   - Core Paradigms, Magickal Practices, and Essential Instruments.
   - **Rote Library** with required Spheres selector and automated M20 difficulty calculation (Coincident +2, Vulgar +3, Vulgar with Witnesses +4).
6. **Page 6 (📝 Session Notes & Chronicle Journal)**:
   - Freeform full-screen notes with cursor-lock typing protection (`StableTextArea`), investigation clues, and image/map attachments up to 10 MB.

### 🌐 Community, Sharing & Chronicle Tools
- **Community Feed (`/feed`)**: Explore publicly shared character sheets with reactive **Likes (curtidas)** and real-time counters.
- **User Profiles (`/user/:username`)**: Public sheet showcase, user statistics, and chronicle history.
- **Google Drive-Style Sharing**:
  - Public or Private visibility.
  - One-click shareable secret link with secure token.
  - Access Control List (**ACL**) granting nominal *Read* or *Edit* permissions to specific users or game rooms.
- **Game Rooms & Chronicle HUD (`/rooms`)**:
  - Real-time party health monitor HUD for the Storyteller.
  - Cabal Chantry Pool (shared Quintessence reserve).
  - Integrated session journal and dice roller.
- **Character Dossier (`📂 Dossiê`)**: Canonical questionnaire featuring 8 character creation questions and 6 player guidance prompts on the Path of Awakening.
- **Cost Breakdown (`CostBreakdownModal`)**: Real-time analytical audit verifying rule legality against official M20 point budgets (15 Freebies & XP).
- **Export & Backup**:
  - Import/Export `.json` character files with cryptographic schema integrity.
  - High-resolution vector print style formatted for physical **A4** paper.

---

## 🏛️ Architecture & Technical Design

MTA Sheet is engineered for extreme resource efficiency, strict security, and zero external runtime dependencies:

| Layer | Technology | Highlights |
| :--- | :--- | :--- |
| **Backend / SSR** | Rust 2024 + Axum 0.7 + Tokio | Server-side rendering for optimal SEO and instantaneous First Contentful Paint (FCP). |
| **Frontend / Reactivity** | Leptos 0.6 + WASM | Fine-grained WebAssembly reactivity without Virtual DOM overhead, delivering stable 60 FPS. |
| **Database** | SQLite 3 via SQLx 0.7 (WAL Mode) | Asynchronous connection pool with embedded automatic migrations and cascade referential integrity. |
| **Styles & UI** | Modular CSS3 + Design Tokens | Dark gothic / arcane aesthetic (*World of Darkness*), with dynamic responsive scaling for mobile and physical A4 printing. |
| **Security & Auditing** | Human vs. Bot Classifier | Cryptographic UUIDv4 session tokens, Bcrypt password hashing, Magic Bytes media verification, and strict CSP headers. |
| **Packaging** | Standalone Single-Binary / Docker | Standalone binary with all HTML, WASM, CSS, and fonts embedded via `rust-embed` (~38 MB idle container footprint). |

---

## ⚡ Quick Run (Pre-compiled)

If you already have a compiled standalone binary:

### On Windows
1. Double-click `mta_sheet.exe` or execute it from PowerShell / CMD:
   ```cmd
   .\mta_sheet.exe
   ```
2. Open your web browser at: **`http://localhost:3000`**.

### On Linux
1. Grant execution permissions and run the binary:
   ```bash
   chmod +x ./mta_sheet
   ./mta_sheet
   ```
2. Open your web browser at: **`http://localhost:3000`**.

> **Note:** The executable is **100% static and self-contained**. It requires no external runtime like Node.js, Python, or GCC/MinGW DLLs.

---

## 🐳 Running via Docker & Docker Compose

The easiest way to self-host MTA Sheet on a homelab or production VPS:

1. **Clone the repository:**
   ```bash
   git clone https://github.com/vitorrezend/mta_sheet.git
   cd mta_sheet
   ```

2. **Configure your environment file:**
   ```bash
   cp .env.example .env
   ```
   *(Set `AUTH_SECRET` and optional `ADMIN_USERNAMES` as desired)*

3. **Launch the stack with Docker Compose:**
   ```bash
   docker compose up -d --build
   ```

4. **Done!** The server will be accessible at `http://localhost:3000`.
   - SQLite database data is safely stored in the persistent volume `./data`.
   - User-uploaded character portraits are stored in `./uploads`.

---

## 🗄️ Database & Environment Configuration

MTA Sheet uses an **embedded SQLite database via SQLx**. On startup, the server automatically initializes the database file and applies all necessary tables, indexes, and migrations.

### Supported Environment Variables (`.env`):

```env
# Connection URL for SQLite (Default: sqlite:data/mta_sheet.db?mode=rwc)
DATABASE_URL=sqlite:data/mta_sheet.db?mode=rwc

# Web server listening address and port (Default: 0.0.0.0:3000)
LEPTOS_SITE_ADDR=0.0.0.0:3000

# Directory for user-uploaded character portraits (Default: ./uploads)
UPLOADS_DIR=./uploads

# Cryptographic secret used for session tokens (Required in production)
AUTH_SECRET=set_a_long_random_secret_string_here

# Administrator usernames with access to /logs and system audits (comma-separated)
ADMIN_USERNAMES=vitor,storyteller
```

---

## 🔨 Building from Source

### Prerequisites

To compile MTA Sheet from source, make sure you have:

1. **Rust (version 1.80+ — 2024 Edition)**:
   - Install via [rustup.rs](https://rustup.rs/).
2. **WebAssembly Target**:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. **Leptos CLI Tools**:
   ```bash
   cargo install wasm-bindgen-cli --version 0.2.121
   cargo install cargo-leptos
   ```

---

### Development Environment (Hot-Reload)

To work on the project with instant browser refresh on Rust or CSS modifications:

#### On Windows:
```cmd
.\scripts\dev.bat
# or directly:
cargo leptos watch
```

#### On Linux:
```bash
./scripts/dev.sh
# or directly:
cargo leptos watch
```

The application will start at `http://127.0.0.1:3000` with live reload.

---

### Compiling Standalone Release Binaries

To produce an optimized single-binary executable with all assets (HTML, WASM, CSS, and fonts) compiled inside:

#### On Windows (.exe standalone):
```cmd
.\scripts\build_release.bat
```
*Outputs an optimized `mta_sheet.exe` in the root folder.*

#### On Linux (ELF standalone binary):
```bash
./scripts/build_release.sh
```
*Outputs an optimized `mta_sheet` binary in the root folder.*

---

## 🧪 Automated Testing & Quality Assurance

MTA Sheet maintains an extensive test suite covering game rules, persistence, and architectural safety.

Run the test suite on the host:
```bash
cargo test --features ssr
```

Or execute targeted test suites:
```bash
# Feed likes & persistence integration test
cargo test --features ssr --test sheet_likes_test

# About & Open Source collaboration page test
cargo test --features ssr --test about_page_test

# M20 character creation & freebie rules test
cargo test --features ssr --test creation_points_test

# Clean architecture & anti-panic static validation
cargo test --features ssr --test anti_patterns_test
```

---

## 📂 Project Directory Structure

```
mta_sheet/
├── src/
│   ├── components/             # Reactive UI components (Leptos)
│   │   ├── common/             # Navbar, modals, focus-safe inputs, controls
│   │   ├── mta_sheet/          # Official M20 Sheet (Pages 1 to 6 and TopBar)
│   │   ├── gods_and_monsters/  # Gods & Monsters companion sheet
│   │   ├── compendium/         # Weapons, Dô, Practices, Instruments, Backgrounds compendium
│   │   ├── rooms/              # Multiplayer game rooms, Battle Grid, live HUD
│   │   └── views/              # Pages (Home, Feed, Profile, Rooms, About, Login, Logs)
│   ├── state/                  # Domain models, M20 cost formulas, Server Functions
│   ├── repositories/           # SQLite data access layer (SQLx)
│   ├── server/                 # Axum handlers, security middleware, SSE, static file serving
│   ├── auth.rs                 # Cookie-based authentication & Bcrypt password hashing
│   ├── database.rs             # SQLite pool initialization, schema, and migrations
│   └── logging.rs              # Real-time structured auditing & Human vs. Bot detector
├── styles/                     # Modular CSS architecture built on semantic Design Tokens
├── data/                       # Canonical compendium JSON files & SQLite database
├── scripts/                    # Build, development, and backup scripts
├── tests/                      # Automated integration & architecture compliance suites
├── Cargo.toml                  # Rust dependencies, features, and Leptos metadata
└── docker-compose.yml          # Container orchestration for production deployment
```

---

## 🤝 How to Contribute

**MTA Sheet** is open-source and welcoming to anyone who loves Mage: The Ascension or Rust development.

### Repository
- 🌐 **GitHub Repository**: [github.com/vitorrezend/mta_sheet](https://github.com/vitorrezend/mta_sheet)
- ✨ **In-App Information**: Visit `/about` (or `/colabore`) directly within the web app.

### Ways to Help:
1. 🐛 **Report Bugs & Suggest Features**:
   - Found a calculation discrepancy with M20 sourcebooks, an A4 print scaling bug, or a mobile UI glitch? [Open an Issue on GitHub](https://github.com/vitorrezend/mta_sheet/issues).
2. 💻 **Code Contributions (Rust & WASM)**:
   - Fork the repository, create a branch (`git checkout -b feature/my-enhancement`), test with `cargo test --features ssr`, and open a Pull Request.
3. 📜 **Lore, Rules & Grimoire**:
   - Help expand the canonical database with rotes, instruments, martial arts (Dô), and expanded backgrounds.
4. 🌐 **Internationalization & Translation**:
   - Improve English and Portuguese terminology in `src/i18n.rs`.
5. 🎨 **Design & Usability**:
   - Ideas for enhancing mobile responsiveness, battle grid usability, or physical print styles.

---

## 📄 License & Legal Notice

This software is licensed under the [MIT License](LICENSE). You are free to run, copy, modify, distribute, and self-host it.

### 🌌 Fan Content Policy (World of Darkness)
*Portions of the materials are the copyrights and trademarks of **Paradox Interactive AB**, and are used with permission. All rights reserved. For more information please visit [worldofdarkness.com](https://www.worldofdarkness.com).*

This is an independent, non-commercial, community-created fan project built to support players and Storytellers of **Mage: The Ascension (M20)**. It is not affiliated with, endorsed by, or sponsored by Paradox Interactive AB or White Wolf Entertainment.
