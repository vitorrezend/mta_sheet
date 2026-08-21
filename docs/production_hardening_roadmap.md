# 🛡️ Guia e Roadmap de Segurança & Prontidão para Produção

Este documento detalha a arquitetura de segurança, desempenho e infraestrutura recomendada para a publicação em ambiente de produção do **MTA Sheet (Mago: A Ascensão 20º Aniversário)**.

---

## 📋 1. Segurança & Controle de Acesso

### 1.1 Autorização por Propriedade (*Access Control*)
- **Objetivo:** Garantir que apenas usuários autorizados possam visualizar, editar ou excluir fichas.
- **Regras de Negócio:**
  - O criador da ficha (`user_id == sheet.user_id`) tem permissão total de edição e exclusão.
  - O Narrador da mesa (`room.gm_id == user_id`) onde a ficha está vinculada tem permissão de leitura e edição/auditoria.
  - Usuários não autorizados não podem sobrescrever dados via *server functions*.

### 1.2 Headers de Segurança HTTP
- **Configuração no Middleware Axum (`tower-http`):**
  - `Content-Security-Policy (CSP)`: Controla origens de scripts, imagens e fontes, prevenindo ataques XSS.
  - `X-Frame-Options: DENY`: Evita ataques de *Clickjacking* (incorporação indevida em iframes externos).
  - `X-Content-Type-Options: nosniff`: Impede que navegadores interpretem arquivos de forma divergente do MIME type declarado.
  - `Referrer-Policy: strict-origin-when-cross-origin`.
  - *Nota:* `Strict-Transport-Security (HSTS)` será ativado exclusivamente em ambiente de produção com certificado SSL/TLS ativo.

### 1.3 Cookies de Sessão Blindados
- Atributos obrigatórios:
  - `HttpOnly`: Impede acesso ao token de sessão via JavaScript (`document.cookie`), mitigando roubo de sessão.
  - `SameSite=Lax`: Proteção nativa contra ataques CSRF (*Cross-Site Request Forgery*).
  - `Secure`: Ativado automaticamente quando a requisição vier via HTTPS.

### 1.4 Validação Profunda de Uploads de Mídia (*Magic Bytes*)
- Validação da assinatura binária real do arquivo no servidor Rust antes de gravar no disco:
  - **PNG:** `89 50 4E 47 0D 0A 1A 0A`
  - **JPEG:** `FF D8 FF`
  - **WEBP:** `52 49 46 46 ... 57 45 42 50`
  - **GIF:** `47 49 46 38`
- Impede upload de executáveis ou scripts renomeados com extensão `.png`.

### 1.5 Rate Limiting (Proteção contra Força Bruta)
- Limitação de tentativas de autenticação nos endpoints `/api/auth/login` e `/api/auth/register` (ex: máximo de 5 tentativas por IP a cada minuto).

---

## ⚡ 2. Desempenho & Otimização de Rede

1. **Compressão HTTP (Gzip / Brotli):**
   - Ativação de `CompressionLayer` no Axum para compactar automaticamente o binário WebAssembly (`mta_sheet.wasm`) e os módulos CSS.
   - Redução estimada de 60% a 75% no tráfego de rede e tempo de carregamento inicial.
2. **Cache-Control com Imutabilidade:**
   - Headers `Cache-Control: public, max-age=31536000, immutable` para `/pkg/` e `/styles/`.

---

## 🗄️ 3. Resiliência do Banco de Dados & Backups

1. **Snapshots Online do SQLite:**
   - Execução periódica de `VACUUM INTO 'backups/mta_sheet_backup.db'` sem bloqueio de concorrência.
2. **Pool de Conexões Otimizado:**
   - Timeout de aquisição, limites de conexões simultâneas e reciclagem de conexões ociosas.

---

## 📊 4. Observabilidade & Monitoramento

1. **Endpoint `/health`:**
   - Verificação rápida de integridade do servidor e conectividade com o banco para probes de monitoramento.
2. **Logs Estruturados:**
   - Tracing estruturado com categorização de logs de segurança, autenticação e erros.

---

## 🐳 5. Containerização (Docker Multi-Stage)

- Imagem de compilação separada da imagem final de execução.
- Imagem de produção enxuta (~35MB) sem compiladores ou dependências de build.
