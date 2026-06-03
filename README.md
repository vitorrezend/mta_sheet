# MTA Sheet - Mage: The Ascension Character Manager

**MTA Sheet** is a full-stack character sheet application for *Mage: The Ascension* built with Rust and the Leptos framework. It allows you to create, manage, and save character sheets with persistent storage using SQLite.

---

## Features
- **Character Management**: Create multiple character sheets and manage them from a central landing page.
- **Persistence**: All character data is saved automatically to an SQLite database.
- **Responsive Design**: Custom 3‑column grid layout that adapts to different screen sizes.
- **Interactive Sheet**: Track attributes, abilities, spheres, advantages (Arete, Willpower, Quintessence/Paradox), and vitality with an intuitive point-and-click interface.

---

## Prerequisites
- **Rust** (stable) – install via [rustup](https://rustup.rs/).
- **cargo-leptos** – install with `cargo install cargo-leptos`.

---

## Getting Started

### 1. Configuration
Copy the `.env.example` file to `.env` and adjust the `DATABASE_URL` if necessary:
```bash
cp .env.example .env
```
The application will automatically create the SQLite database file if it doesn't exist.

### 2. Running the Development Server
Use the provided scripts or `cargo-leptos`:
```bash
# On Linux/macOS
./run_project.sh

# On Windows
run_project.bat

# Or directly with cargo-leptos
cargo leptos watch
```
The application will be available at `http://127.0.0.1:3000`.

---

## Project Structure
```
src/
  components/       # UI components (Home, CharacterSheet, etc.)
  database.rs       # SQLite database initialization and connection
  state.rs          # Server functions and data structures
  lib.rs            # Root App component and routing
  main.rs           # Server entry point (Axum)
style.css           # Global CSS and layout system
```

---

## License
[MIT License](LICENSE).
