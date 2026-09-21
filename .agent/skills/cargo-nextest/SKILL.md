---
name: cargo-nextest
description: >-
  Guia completo de uso, configuração e boas práticas do cargo-nextest no MTA Sheet.
  Acelera a execução de testes através de concorrência massiva, isolamento de processo
  por teste e filtros avançados (-E / --expr).
---

# 🚀 Next-Generation Test Runner: cargo-nextest

O **cargo-nextest** é o executor de testes de última geração para Rust, projetado para substituir o test harness padrão em projetos com alta densidade de testes de integração, componentes Leptos e banco de dados SQLite.

---

## ⚡ 1. Principais Vantagens no MTA Sheet

| Recurso | `cargo test` Padrão | `cargo-nextest` | Vantagem no Projeto |
| :--- | :--- | :--- | :--- |
| **Isolamento de Testes** | Threads no mesmo processo | **Processo isolado para cada teste** | Evita contaminação de estado reativo (Leptos runtime) e concorrência no SQLite |
| **Execução Concorrente** | Paralelismo básico | Concorrência paralela otimizada por CPU | Redução drástica do tempo de execução total |
| **Feedback de Falha** | Espera o final da suíte | **Imediato (Streaming)** | Mostra stdout/stderr da falha no momento exato em que ocorre |
| **Filtragem** | Filtro simples por substring | **Linguagem de Expressões (`-E`)** | Filtros booleanos sofisticados (`and`, `or`, `not`, regex) |
| **Relatórios de CI** | Texto simples ou JSON instável | JUnit XML, GitHub Actions, libtest | Integração nativa com esteiras de CI/CD |

---

## 🛠️ 2. Instalação & Pré-requisitos

Para instalar localmente no Windows ou Linux:

```bash
# Via Cargo (Recomendado)
cargo install cargo-nextest --locked

# Ou via Scoop (Windows)
scoop install cargo-nextest

# Ou via Homebrew (Linux / macOS)
brew install cargo-nextest
```

Para verificar a instalação:
```bash
cargo nextest --version
```

---

## 📋 3. Comandos do Dia a Dia

### Executar Todos os Testes
```bash
cargo nextest run --features ssr
```

### Executar um Teste Específico por Nome
```bash
cargo nextest run -E 'test(test_name)'
```

### Executar uma Suíte Específica (ex: Compêndio de Esferas)
```bash
cargo nextest run -E 'test(spheres_compendium_test)'
```

### Executar Múltiplas Suítes com Operadores Lógicos
```bash
# Executa apenas testes do compêndio OU sanitização:
cargo nextest run -E 'test(compendium) or test(sanitization)'

# Executa todos os testes EXCETO os de segurança de upload:
cargo nextest run -E 'not test(security_limits)'
```

### Modo Interativo de Depuração (Ver Saída de Logs e Stdout)
Por padrão, o nextest captura o output e só exibe em caso de falha. Para ver saídas completas:
```bash
cargo nextest run --no-capture -E 'test(test_name)'
```

### Modo Watch (com `cargo-watch`)
Para rodar testes automaticamente a cada arquivo salvo:
```bash
cargo watch -x "nextest run --features ssr"
```

---

## ⚙️ 4. Configuração Canônica (`.config/nextest.toml`)

O comportamento do `cargo-nextest` é controlado pelo arquivo `.config/nextest.toml` na raiz do repositório:

```toml
[profile.default]
# Concorrência máxima baseada em cores de CPU
test-threads = "num-cpus"

# Alerta caso um teste demore mais de 10 segundos (útil para detectar deadlocks)
slow-timeout = { period = "10s", terminate-after = 2 }

# Retentativas automáticas (desabilitado em dev para feedback fiel)
retries = 0

[profile.ci]
# No CI, falhar rápido se necessário e exportar JUnit
retries = 1
slow-timeout = { period = "30s" }
```

Para rodar com o perfil de CI:
```bash
cargo nextest run --profile ci
```

---

## 🛡️ 5. Boas Práticas para Leptos & SQLite no Nextest

1. **Bancos SQLite em Testes:**
   - Como cada teste roda em seu próprio processo pelo `nextest`, bancos de teste em memória (`sqlite::memory:`) ou temporários isolados em disco nunca sofrem colisões de lock (`database is locked`) entre testes concorrentes.
2. **Runtime Leptos:**
   - Qualquer panic em um teste não corrompe o pool de threads de outros testes.
3. **Binário Único (`all_tests`):**
   - O `cargo-nextest` aproveita ao máximo a suíte unificada em `tests/all_tests.rs`: ele compila o binário apenas uma vez e depois orquestra a execução individual de todos os testes cadastrados em milissegundos.
