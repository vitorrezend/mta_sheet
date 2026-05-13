# Skill: Compartimentalização e Arquitetura Modular

Esta skill orienta a organização do código no projeto `mta_sheet`, garantindo que cada arquivo tenha uma responsabilidade única e clara (Single Responsibility Principle).

## Estrutura de Pastas Recomendada

### 1. `src/components/` (Natureza: Interface)
Cada componente visual deve residir em seu próprio arquivo dentro desta pasta.
- **Regra:** Se um componente começar a ter muitas linhas ou sub-partes complexas, divida-o em novos arquivos.
- **Exemplo:** `Attributes.rs` gerencia o bloco de atributos, mas usa o `ValueField.rs` para as linhas individuais.

### 2. `src/state.rs` (Natureza: Lógica e Dados)
Contém as "regras do jogo" e a estrutura de dados.
- **O que fica aqui:** Structs, Enums, implementações de `Default`, e métodos de persistência (`save`/`load`).
- **O que NÃO fica aqui:** HTML, macros `view!`, ou lógica de estilo.

### 3. `style.css` (Natureza: Estética)
Centraliza o design system.
- **Regra:** Use variáveis CSS para cores e tamanhos para facilitar mudanças globais de tema.
- **Organização:** Agrupe estilos por componentes ou por utilitários (ex: layouts de grid, tooltips).

## Diretrizes de Desenvolvimento

### Separação de Preocupações (SoC)
- **Componentes são burros:** Eles devem apenas saber como exibir dados e avisar quando o usuário clicou em algo (via callbacks).
- **Estado é inteligente:** Ele decide como os dados são transformados e como são salvos.

### Modularidade em Rust
Sempre utilize o sistema de módulos do Rust de forma explícita:
- Use `mod components;` no `main.rs`.
- Cada arquivo novo em `components/` deve ser declarado no `src/main.rs` ou em um `src/components/mod.rs`.

> [!TIP]
> Antes de criar uma nova funcionalidade, pergunte-se: "Isso é uma regra de dados ou um elemento visual?". A resposta dirá se você deve mexer no `state.rs` ou criar um novo componente.
