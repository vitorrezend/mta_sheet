# MTA Sheet

**MTA Sheet** é um gerenciador de fichas de personagem para *Mago: A Ascensão* construído com Rust e Leptos. Ele utiliza um design de grade de 3 colunas para fornecer um layout limpo e responsivo para rastrear habilidades, vantagens e outros dados do personagem.

---

## Recursos
- Layout de grade de 3 colunas responsivo usando propriedades CSS customizadas.
- Componentes modulares em Rust para atributos, habilidades, vantagens, vitalidade, etc.
- Renderização do lado do servidor (SSR) com hidratação usando **Leptos**.
- Persistência automática usando **SQLite** com **sqlx**.
- Gerenciamento de múltiplas fichas (criar, abrir, excluir).
- Configuração fácil via `style.css`.

---

## Pré-requisitos
- **Rust** (stable) – instale via [rustup](https://rustup.rs/).
- **cargo-leptos** – instale com `cargo install cargo-leptos`.

---

## Configuração

### Banco de Dados
A aplicação utiliza SQLite para persistência. Você pode configurar o caminho do banco de dados através da variável de ambiente `DATABASE_URL` ou um arquivo `.env`. Se não for especificado, o padrão será `mta_sheet.db`.

```bash
# Exemplo de .env
DATABASE_URL=sqlite:caminho_customizado.db
```

O aplicativo criará automaticamente o arquivo do banco de dados e as tabelas necessárias se eles não existirem. Não é necessário recompilar o projeto ao alterar o `DATABASE_URL` no `.env`.

### Executando o Projeto
```bash
# Clone o repositório
git clone <repo‑url>
cd mta_sheet

# Execute o servidor de desenvolvimento (monitora alterações automaticamente)
cargo leptos watch
```
A aplicação estará disponível em `http://127.0.0.1:3000`.

---

## Estrutura do Projeto
```
src/                # Arquivos fonte Rust
  components/       # Componentes UI (atributos, habilidades, home, etc.)
  database.rs       # Conexão e inicialização do SQLite
  state.rs          # Funções de servidor e estruturas de dados
style.css           # CSS global e sistema de design
README.md           # Este arquivo
```

---

## Scripts
Scripts de conveniência são fornecidos para iniciar o projeto:
- `run_project.sh` (Linux/macOS)
- `run_project.bat` (Windows)

---

## Licença
[MIT License](LICENSE).
