#!/bin/bash
set -e

DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$DIR"

echo "======================================================="
echo "  MTA Sheet - Ambiente de Desenvolvimento [Dev]"
echo "======================================================="
echo ""

# 1. Carrega variaveis de ambiente se existir
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
fi

export DATABASE_URL="${DATABASE_URL:-sqlite:mta_sheet.db}"
export LEPTOS_SITE_ADDR="${LEPTOS_SITE_ADDR:-0.0.0.0:3000}"
export RUST_LOG="${RUST_LOG:-info}"
export RUST_BACKTRACE=1

echo "[CONFIG] Banco de Dados: $DATABASE_URL"
echo "[CONFIG] Endereco:       http://$LEPTOS_SITE_ADDR"
echo "[CONFIG] Logs:           $RUST_LOG"
echo ""

# 2. Prepara diretorios
mkdir -p uploads target/site/pkg styles

# 3. Se o argumento 'build' for passado, apenas compila no modo dev
if [ "$1" = "build" ]; then
    echo "[INFO] Compilando no perfil DEV (sem watcher)..."
    cargo leptos build
    exit 0
fi

# 4. Executa com cargo-leptos watch ou fallback
if command -v cargo-leptos &> /dev/null; then
    echo "[INFO] Iniciando servidor com Hot-Reload (cargo leptos watch)..."
    cargo leptos watch
else
    echo "[AVISO] cargo-leptos nao encontrado. Iniciando via cargo run --features ssr..."
    cargo run --features ssr
fi
