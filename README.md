# MTA Sheet

**MTA Sheet** is a character sheet application for *Mage: The Ascension* built with Rust and Trunk. It follows a custom 3‑column grid design system that provides a clean, responsive layout for tracking abilities, advantages, and other character data.

---

## Features
- Responsive 3‑column grid layout using CSS custom properties.
- Modular Rust components for abilities, advantages, vitality, etc.
- Server-Side Rendering (SSR) with hydration using **Leptos**.
- Persistence using **SQLite** with **sqlx**.
- Easy theming and styling via `style.css`.

---

## Prerequisites
- **Rust** (stable) – install via [rustup](https://rustup.rs/).
- **cargo-leptos** – install with `cargo install cargo-leptos`.

---

## Getting Started

### Database Configuration
The application uses SQLite. You can configure the database URL via the `DATABASE_URL` environment variable or a `.env` file. If not specified, it defaults to `mta_sheet.db`.

```bash
# Example .env
DATABASE_URL=sqlite:custom_path.db
```

A aplicação irá criar automaticamente o arquivo do banco de dados e as tabelas necessárias caso eles não existam. O caminho do banco de dados pode ser configurado sem a necessidade de recompilação do projeto, bastando alterar a variável de ambiente `DATABASE_URL`.

### Running the App
```bash
# Clone the repository (if not already present)
git clone <repo‑url>
cd mta_sheet

# Run the development server (automatically watches for changes)
cargo leptos watch
```
The application will be available at `http://127.0.0.1:3000`.

---

## Project Structure
```
src/                # Rust source files
  components/       # UI components (abilities, advantages, vitality, …)
  database.rs       # SQLite connection and initialization
  state.rs          # Server functions and data structures
style.css           # Global CSS, includes the 3‑column grid system
README.md           # This file
```

---

## Scripts
Convenience scripts are provided to start the project:
- `run_project.sh` (Linux/macOS)
- `run_project.bat` (Windows)

---

## License
[MIT License](LICENSE) (or your chosen license).
