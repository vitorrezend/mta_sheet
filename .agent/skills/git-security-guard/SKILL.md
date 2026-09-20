---
name: git-security-guard
description: >-
  Enforces strict Git security, credential protection, and .gitignore auditing for the MTA Sheet repository.
  Blocks committing sensitive data, private keys, SSL/TLS certificates, tokens, passwords, .env secrets,
  SQLite production databases, and user-uploaded media.
---

# 🛡️ Git Security Guard (MTA Sheet)

Esta skill estabelece e impõe diretrizes estritas de segurança de repositório Git para o projeto **MTA Sheet**, garantindo que nenhum segredo, chave criptográfica, credencial ou dado sensível seja exposto acidentalmente no histórico de commits ou repositórios remotos (GitHub/GitLab).

---

## 🚫 1. Arquivos Estritamente Proibidos no Git

NUNCA permita ou crie commits contendo os seguintes tipos de arquivos:

### 🔑 Segredos, Chaves & Certificados (Risco Crítico)
- Chaves privadas SSL/TLS: `*.key`, `*.pem`, `*.pfx`, `*.p12`
- Certificados e requisições: `*.crt`, `*.cer`, `*.csr`
- Pastas de ACME / Certbot / Caddy: `certs/`, `ssl/`, `acme/`, `caddy_data/`, `caddy_config/`, `.caddy/`
- Arquivos de variáveis de ambiente com valores reais: `.env`, `.env.local`, `.env.production` (somente `.env.example` sem segredos é permitido).
- Tokens e senhas: `*.token`, `*.secret`, scripts de atualização do DuckDNS contendo tokens em texto puro (`duckdns_update.bat`, `duckdns_update.sh`).
- Scripts customizados de deploy de produção com caminhos ou overrides locais: `update_prod.bat`, `update_prod.sh`, `scripts/update_prod.bat`, `scripts/update_prod.sh` (somente `.example` sem dados locais é permitido).

### 💾 Bancos de Dados & Sessões (Risco de Vazamento de Dados)
- Arquivos SQLite reais: `*.db`, `*.sqlite`, `*.sqlite3`
- Arquivos de diário e lock do SQLite: `*-wal`, `*-shm`, `*-journal`
- Dumps SQL ou JSON contendo dados reais de usuários ou fichas privadas.

### 🖼️ Mídia e Uploads de Usuários
- Fotos de personagens enviadas por jogadores: `uploads/`, `data/uploads/`.
- Metadados de sincronização do Syncthing: `.stfolder/`, `.stversions/`, `*.sync-conflict-*`.

### 📦 Binários e Artefatos de Compilação
- Executáveis compilados: `*.exe`, `mta_sheet.exe`, `target/release/mta_sheet`
- Binários WebAssembly: `*.wasm` (gerados dinamicamente no build)
- Arquivos de debug e cache do Cargo: `target/`, `target_test/`, `dist/`, `pkg/`

---

## 🔍 2. Protocolo de Verificação Pré-Commit (Checklist Obrigatório)

Antes de qualquer `git commit` ou orientação ao usuário:

1. **Auditar o `git status`:**
   ```bash
   git status --short
   ```
   - Verifique minuciosamente a seção de arquivos não rastreados (`??`).
   - Se houver qualquer arquivo `.env`, `.pem`, `.key`, `.db`, `.token` ou imagem em `uploads/`, pare imediatamente.

2. **Verificar que o `.gitignore` está ativo:**
   Teste se um arquivo sensível está devidamente ignorado pelo Git:
   ```bash
   git check-ignore -v .env certs/privkey.pem data/mta_sheet.db
   ```
   A resposta deve exibir a linha exata do `.gitignore` que capturou o arquivo.

3. **Arquivos de Exemplo Seguros:**
   - Sempre forneça arquivos `.example` para guiar os usuários sem expor credenciais reais:
     - `.env.example`
     - `duckdns_update.example.bat`
     - `duckdns_update.example.sh`

---

## 🚨 3. Ação Corretiva em Caso de Detecção de Vazamento

Se um segredo for acidentalmente preparado na staging area (`git add`):
```bash
# Desfaz a preparação imediatamente sem perder edições locais
git reset HEAD <arquivo_sensivel>
```

Se o arquivo já tiver sido commitado localmente (mas NÃO enviado ao remote):
```bash
# Remove do Git mantendo o arquivo no disco local
git rm --cached <arquivo_sensivel>
git commit --amend -m "chore: remove sensitive file from git tracking"
```
