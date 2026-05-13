# Skill: Manutenção de Testes em Rust/WASM (Leptos)

Esta skill define as diretrizes para garantir que o projeto `mta_sheet` permaneça robusto e livre de regressões através de testes automatizados.

## Contexto do Projeto
O projeto utiliza **Leptos (WASM)** com renderização client-side (CSR). A persistência é feita via `localStorage` e o estado é gerenciado de forma reativa.

## Diretrizes de Ouro

### 1. Testes de Lógica (State)
Sempre que uma estrutura de dados no `src/state.rs` for alterada (ex: novos campos em `AttributeValue`), os testes correspondentes **devem** ser atualizados.
- Use `#[test]` para lógica pura.
- Use `#[wasm_bindgen_test]` para lógica que interage com o navegador (Storage, Window).

### 2. Testes de Componentes
Cada componente novo ou modificado em `src/components/` deve ter um módulo `mod tests` no final do arquivo.
- **Flexibilidade:** Prefira usar `Signal<T>` em vez de `ReadSignal<T>` nos props dos componentes. Isso facilita a passagem de sinais derivados e mocks durante os testes.
- **Verificação:** No ambiente WASM, verifique se o componente instancia corretamente sem pânico.

### 3. Ferramental
- **Local:** Use `cargo test` para feedback rápido sobre lógica de Rust.
- **WASM:** Use `wasm-pack test --headless --chrome` para validar interações reais com a API do navegador.

### 4. Fluxo de Trabalho
> [!IMPORTANT]
> Nunca considere uma tarefa "concluída" sem verificar se os testes existentes ainda passam e se novos testes foram adicionados para a funcionalidade recém-criada.

## Exemplos de Referência
- **Estado:** [src/state.rs](file:///c:/Users/MT8-02/Documents/pessoal/programa%C3%A7%C3%A3o/mta_sheet/src/state.rs)
- **Componente:** [src/components/value_field.rs](file:///c:/Users/MT8-02/Documents/pessoal/programa%C3%A7%C3%A3o/mta_sheet/src/components/value_field.rs)
