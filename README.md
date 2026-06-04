# MTA Sheet

**MTA Sheet** is a character sheet application for *Mage: The Ascension* built with Rust using the [Leptos](https://leptos.dev/) framework (v0.6). It features a full-stack architecture with Server-Side Rendering (SSR) and SQLite persistence.

---

## Features
- **Landing Page**: Create new character sheets or open previously saved ones.
- **SQLite Persistence**: Character data is stored as JSON blobs in a local SQLite database.
- **Configurable**: Easily change the database path via environment variables without recompiling.
- **Responsive Design**: Custom 3-column grid system with mobile scaling support.
- **Modular Components**: Organized Rust components for attributes, abilities, spheres, and advantages.

---

## Prerequisites
- **Rust** (stable) – install via [rustup](https://rustup.rs/).
- **cargo-leptos** – install with `cargo install cargo-leptos`.

---

## Getting Started

### 1. Database Configuration
The application uses SQLite for storage. By default, it creates a file named `mta_sheet.db` in the project root.

You can customize the database path using the `DATABASE_URL` environment variable:
1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```
2. Edit `.env` and set `DATABASE_URL` to your preferred path (e.g., `sqlite:/path/to/your/database.db`).

### 2. Running the Project

The project is optimized for use with `cargo-leptos`.

**Development Mode (with live-reload):**
```bash
cargo leptos watch
```

**Convenience Scripts:**
- **Windows**: Run `run_project.bat`
- **Linux/macOS**: Run `./run_project.sh`

Once running, access the application at `http://127.0.0.1:3000`.

---

## Technical Architecture

- **Backend**: Axum web server with Leptos SSR integration.
- **Database**: SQLite managed via `sqlx` (only active on the server side).
- **Frontend**: Leptos components that hydrate in the browser for interactivity.
- **State Management**: Character data is managed using Leptos signals and persisted via Server Functions.

---

## Project Structure
```
src/
  components/       # UI components (attributes, abilities, etc.)
  database.rs       # Server-side SQLite connection and schema setup
  state.rs          # Shared data models and Leptos Server Functions
  lib.rs            # Main App component and routing
  main.rs           # Server entry point and Axum routing
style.css           # Global styles and design system
```

---

## License
[MIT License](LICENSE).
