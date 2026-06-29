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

### Configuração do Banco de Dados (Database Configuration)

O aplicativo utiliza SQLite para persistência. Você pode configurar o caminho do banco de dados sem precisar recompilar o projeto através da variável de ambiente `DATABASE_URL` ou de um arquivo `.env`.

Se não for especificado, o padrão será `mta_sheet.db` na raiz do projeto.

#### Como configurar:

1.  **Via arquivo `.env`**:
    Crie um arquivo chamado `.env` na raiz do projeto (use o `.env.example` como base):
    ```bash
    DATABASE_URL=sqlite:meu_banco_de_dados.db
    ```

2.  **Via variável de ambiente**:
    ```bash
    # Linux/macOS
    export DATABASE_URL=sqlite:/caminho/para/banco.db
    cargo run --features ssr

    # Windows (PowerShell)
    $env:DATABASE_URL="sqlite:C:\caminho\para\banco.db"
    cargo run --features ssr
    ```

**Nota:** O aplicativo criará automaticamente o arquivo do banco de dados e as tabelas necessárias se eles não existirem. Não é necessário criar o arquivo manualmente.

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
