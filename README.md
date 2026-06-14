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
A aplicação utiliza SQLite para persistência. Você pode configurar o caminho do banco de dados sem precisar recompilar o projeto através da variável de ambiente `DATABASE_URL` ou de um arquivo `.env`.

```bash
# Exemplo de .env
DATABASE_URL=sqlite:meu_banco_de_dados.db
```

**Notas Importantes:**
- Se `DATABASE_URL` não for informada, o sistema usará por padrão `mta_sheet.db`.
- O prefixo `sqlite:` é opcional (será adicionado automaticamente se estiver ausente).
- A aplicação criará automaticamente o arquivo do banco de dados e as tabelas necessárias na primeira execução, se eles não existirem.
- Certifique-se de que o diretório onde o banco será criado tenha permissões de escrita.

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
