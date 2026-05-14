#!/bin/bash

echo "=========================================="
echo "  Iniciando MTA Sheet (Modo Full-Stack)"
echo "=========================================="

# Carrega .env se existir de forma segura
if [ -f .env ]; then
    set -a
    source .env
    set +a
fi

# Define padrão se não estiver definido
export DATABASE_URL=${DATABASE_URL:-sqlite:mta_sheet.db}

echo "Banco de dados: $DATABASE_URL"

# Verifica se o cargo-leptos está instalado
if command -v cargo-leptos &> /dev/null; then
    echo "[INFO] Usando cargo-leptos para iniciar o projeto..."
    cargo leptos watch
else
    echo "[AVISO] cargo-leptos não encontrado."
    echo "[INFO] Tentando iniciar com cargo run --features ssr..."
    echo "[INFO] Nota: Hydration pode falhar se os assets não forem compilados corretamente."

    cargo run --features ssr
fi
