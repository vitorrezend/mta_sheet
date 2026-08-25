#!/bin/sh
set -e

DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$DIR"

echo "======================================================="
echo "  MTA Sheet - Compilando Imagem Docker Ultra-Leve"
echo "======================================================="

docker build -t mta_sheet:latest .

echo ""
echo "======================================================="
echo "[SUCESSO] Imagem Docker construida com sucesso!"
echo "======================================================="
docker images mta_sheet:latest
