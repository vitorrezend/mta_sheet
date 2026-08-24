#!/bin/bash
set -e

# Navega até a raiz do projeto
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "=========================================="
echo "  Iniciando MTA Sheet (Modo Produção/Release)"
echo "=========================================="

# Carrega .env se existir
if [ -f .env ]; then
    set -a
    source .env
    set +a
fi

export DATABASE_URL=${DATABASE_URL:-sqlite:mta_sheet.db}
export LEPTOS_SITE_ADDR=${LEPTOS_SITE_ADDR:-0.0.0.0:3000}

# Se o binário standalone compilado existir na raiz, executa diretamente
if [ -f "./mta_sheet" ]; then
    echo "[INFO] Executando binário único standalone ./mta_sheet..."
    ./mta_sheet
elif [ -f "./target/server/release/mta_sheet" ]; then
    echo "[INFO] Executando ./target/server/release/mta_sheet..."
    ./target/server/release/mta_sheet
elif command -v cargo-leptos &> /dev/null; then
    echo "[INFO] Compilando e servindo com cargo-leptos..."
    cargo leptos serve --release
else
    echo "[INFO] Executando via cargo run --release --features ssr..."
    cargo run --release --features ssr
fi
