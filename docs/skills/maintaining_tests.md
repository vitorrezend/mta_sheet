# Skill: Manutenção de Testes em Rust/WASM (Leptos Full-Stack)

Esta skill define as diretrizes para garantir que o projeto `mta_sheet` permaneça robusto e livre de regressões através de testes automatizados.

## Contexto do Projeto
O projeto utiliza **Leptos 0.6 Full-Stack** com renderização SSR (Axum + SQLite) e hidratação no cliente (WASM). A persistência é feita via `SQLx` e Server Functions (`#[server]`), e o estado é gerenciado de forma reativa.

## Diretrizes de Ouro

### 1. Testes de Lógica de Domínio (State)
Sempre que uma estrutura de dados ou regra no `src/state.rs` for alterada (ex: limites de Arete, Força de Vontade, Paradoxo/Quintessência), os testes correspondentes **devem** ser atualizados ou criados.
- Use `#[test]` para lógica pura e testes de regras de negócio.
- Valide sanitização e integridade de dados.

### 2. Testes de Componentes
Componentes que possuem testes devem criar e descartar o runtime do Leptos adequadamente (`let runtime = create_runtime(); ... runtime.dispose();`).
- **Flexibilidade:** Prefira usar `Signal<T>` em vez de `ReadSignal<T>` nos props dos componentes para facilitar passagem de sinais derivados.

### 3. Ferramental
- **Local:** Use `cargo test` para feedback rápido sobre lógica de Rust e estado.
- **Checagem Full-Stack:** Use `cargo check --features ssr` e `cargo check --features hydrate`.

### 4. Fluxo de Trabalho
> [!IMPORTANT]
> Nunca considere uma tarefa "concluída" sem verificar se os testes existentes ainda passam e se a compilação de ambas as features (`ssr` e `hydrate`) é bem-sucedida.
