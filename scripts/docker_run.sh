#!/bin/sh
set -e

DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$DIR"

echo "======================================================="
echo "  MTA Sheet - Executando via Docker Compose"
echo "======================================================="

docker compose up -d

echo ""
echo "======================================================="
echo "[SUCESSO] MTA Sheet esta rodando em: http://localhost:3000"
echo "======================================================="
