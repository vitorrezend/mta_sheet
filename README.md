# MTA Sheet

**MTA Sheet** is a character sheet application for *Mage: The Ascension* built with Rust and Trunk. It follows a custom 3‑column grid design system that provides a clean, responsive layout for tracking abilities, advantages, and other character data.

---

## Features
- Responsive 3‑column grid layout using CSS custom properties.
- Modular Rust components for abilities, advantages, vitality, etc.
- Live‑reload development server powered by **Trunk**.
- Easy theming and styling via `style.css`.

---

## Prerequisites
- **Rust** (stable) – install via [rustup](https://rustup.rs/).
- **Trunk** – install with `cargo install trunk`.
- **Node.js** (optional, only if you need additional tooling).

---

## Getting Started
```bash
# Clone the repository (if not already present)
git clone <repo‑url>
cd mta_sheet

# Install Rust dependencies
cargo build

# Run the development server
trunk serve
```
The application will be available at `http://localhost:8080` and will automatically reload on source changes.

---

## Project Structure
```
src/                # Rust source files
  components/       # UI components (abilities, advantages, vitality, …)
style.css           # Global CSS, includes the 3‑column grid system
README.md           # This file
```

---

## Running the Project (Windows)
A convenience batch script is provided:
```bat
start.bat
```
This script launches the Trunk development server with the appropriate environment.

---

## License
[MIT License](LICENSE) (or your chosen license).
