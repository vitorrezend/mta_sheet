#!/bin/bash
set -e

# Navega até a raiz do projeto (um nível acima de scripts/)
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "=========================================="
echo "  Iniciando MTA Sheet (Modo Desenvolvimento)"
echo "=========================================="

# Carrega .env se existir
if [ -f .env ]; then
    set -a
    source .env
    set +a
fi

# Define padrões
export DATABASE_URL=${DATABASE_URL:-sqlite:mta_sheet.db}
export LEPTOS_SITE_ADDR=${LEPTOS_SITE_ADDR:-0.0.0.0:3000}

echo "Diretório do projeto : $PROJECT_ROOT"
echo "Banco de dados       : $DATABASE_URL"
echo "Endereço de escuta   : http://$LEPTOS_SITE_ADDR"
echo "=========================================="

if command -v cargo-leptos &> /dev/null; then
    echo "[INFO] Usando cargo-leptos para watch & reload..."
    cargo leptos watch
else
    echo "[AVISO] cargo-leptos não encontrado no PATH."
    echo "[INFO] Tentando iniciar com cargo run --features ssr..."
    cargo run --features ssr
fi
