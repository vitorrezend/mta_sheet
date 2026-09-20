---
name: mta-clean-architecture
description: >-
  Guia de Arquitetura Limpa, Padrão de Cores (Design Tokens) e Modularização
  para o projeto MTA Sheet (Rust + Leptos + Axum). Impõe o teto de 1000 linhas por arquivo,
  proíbe cores hexadecimais avulsas e exige o reuso de componentes canônicos.
---

# 🔮 MTA Clean Architecture & Design System Guide

Este guia estabelece os padrões arquiteturais, regras de modularização, convenções de código limpo e o design system do projeto **MTA Sheet**.

---

## 📏 1. Regra de Ouro: Limite de Tamanho de Arquivo (1.000 Linhas)

1. **Teto Máximo:** Nenhum arquivo de código (`.rs` ou `.css`) deve ultrapassar **1.000 linhas**.
2. **Exceção Canônica para Compêndios Estáticos (`src/compendium/` e `data/compendium/`):**
   - Arquivos puramente declarativos de dados estáticos, tabelas de regras de RPG e catálogos de compêndio (como `backgrounds/data.rs`, `spheres/data.rs`, `practices.rs`, etc.) estão **isentos** do teto de 1.000 linhas.
   - **Proibição de Fatiamento Numérico:** É expressamente **proibido** criar arquivos artificiais como `data_part1.rs`, `data_part2.rs`. Dados de compêndio devem ser unificados em arquivos semânticos coesos (`data.rs` ou por categorias de jogo como `social.rs`, `mystic.rs`).
3. **Decomposição por Domínio:**
   - Ao aproximar-se de 1.000 linhas em arquivos de **lógica de aplicação, estado ou UI**, o módulo deve ser decomposto em uma pasta com `mod.rs` e submódulos temáticos.
   - Separar **Dados Estáticos** de **Lógica de Apresentação** (ex: catálogos estáticos em `catalog.rs`, modelos em `models.rs`, visualização em `view.rs`).
4. **Single Responsibility:** Cada componente de UI deve cuidar exclusivamente de sua sub-árvore (ex: separar `sidebar`, `weapon_card`, `maneuver_card` e `callouts`).

---

## 🎨 2. Padrão de Cores & Design Tokens (`styles/00-tokens.css`)

**NUNCA utilize códigos hexadecimais (`#caa75d`, `#991b1b`, etc.) diretamente no CSS novo ou em edições.**
Sempre utilize as variáveis semânticas oficiais definidas em `styles/00-tokens.css`:

| Propósito | Token Recomendado | Valor / Cor |
| :--- | :--- | :--- |
| **Ouro Primário / Hermético** | `var(--gold-primary)` / `var(--gold-light)` | `#caa75d` / `#f7e6b5` |
| **Ouro Brilhante / Destaque** | `var(--gold-bright)` / `var(--gold-glow)` | `#ffd700` |
| **Púrpura Profundo (Akashiano)** | `var(--purple-deep)` / `var(--purple-mid)` | `#170428` / `#2d0a4e` |
| **Púrpura Chi / Dô / Destaque** | `var(--purple-bright)` / `var(--purple-light)`| `#9333ea` / `#d8b4fe` |
| **Dano / Combate** | `var(--color-damage)` | `#991b1b` |
| **Dificuldade / Paradas** | `var(--color-difficulty)` / `var(--color-pool)` | `#1e3a8a` / `#0f766e` |
| **Ações / Cadência** | `var(--color-actions)` | `#6b21a8` |
| **Paradoxo / Alertas Críticos** | `var(--color-paradox)` | `#dc2626` |
| **Superfícies Nobres** | `var(--surface-paper)` / `var(--surface-card)` | `#ffffff` / `#f4eee1` |
| **Bordas Sutis de Card** | `var(--surface-card-border)` | `#e5dccb` |

---

## 🧩 3. Componentes Reutilizáveis Obrigatórios

Para evitar duplicação de CSS e markup no DOM:

1. **Modais:** Sempre utilize o componente `<Modal isOpen title icon size onClose>` em `src/components/common/modal.rs`. Nunca recrie estruturas manuais com `.modal-overlay` e `.modal-card`.
2. **Caixas de Estatísticas:** Utilize `<StatBox label value sub color density />` em `src/components/common/stat_box.rs` para exibir métricas com contenção contra overflow.
3. **Pílulas de Filtro:** Centralize padrões de seleção em botões estilo pílula ativa/inativa.
4. **Callouts de Regras:** Utilize containers consistentes com borda dupla ou gradiente místico para citações canônicas de M20.
5. **App Shell & Navegação Canônica:** Toda nova página/rota da aplicação (com exceção de `/sheet/:id` que é a folha A4 especializada) DEVE ser registrada como rota filha do `<AppLayout>` em `src/lib.rs`. Nunca instancie `<Navbar />` manualmente dentro de visualizações individuais. Páginas secundárias (Feed, Perfil, etc.) DEVEM conter a barra de retorno `<div class="page-breadcrumb-nav">` com link `"← Voltar para Minhas Fichas"`.

---

## ⚡ 4. Boas Práticas Rust & Leptos 0.6

1. **Focus-Lock em Inputs:** Campos de digitação contínua devem usar `StableTextInput`, `StableTextArea` ou `ValueField` para eliminar perda de foco ou reset do cursor na digitação.
2. **Sem Clones Desnecessários no Render:** Mantenha closures reativas enxutas (`move || signal.get()`).
3. **Internacionalização Centralizada:** Em vez de aninhar `match current_lang.get() { ... }` repetitivo em JSX, utilize métodos do domínio (`model.name(lang)`, `model.damage(lang)`) ou chaves centralizadas em `crate::i18n::tr(...)`.
4. **Resiliência a JSON Antigo:** Modelos do estado (`CharacterData`) devem implementar serde default e sanitização graciosa (`src/state/sanitization.rs`).

---

## 🧪 5. Regra de Verificação Local-First

Toda alteração de arquitetura DEVE ser validada localmente com:
1. `cargo check --target wasm32-unknown-unknown --no-default-features --features hydrate`
2. `cargo test --features ssr`
Nenhum build Docker deve ser utilizado para testes intermediários.
