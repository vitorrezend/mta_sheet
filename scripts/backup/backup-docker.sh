#!/usr/bin/env bash
# ==============================================================================
# MTA Sheet - Backup Docker (SQLite + Uploads) [Linux]
# ==============================================================================

set -euo pipefail

DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$DIR"

PREFIX="${1:-backup}"

echo "========================================================"
echo "  MTA Sheet - Backup Docker (SQLite + Uploads)"
echo "========================================================"
echo ""

if ! command -v docker &> /dev/null; then
    echo "❌ Erro: 'docker' não está instalado ou não está no PATH."
    exit 1
fi

if ! docker ps --filter "name=^mta_sheet_app$" --filter "status=running" -q | grep -q .; then
    echo "⚠️ Aviso: O container 'mta_sheet_app' não está em execução. Backup ignorado."
    exit 2
fi

ANO=$(date +"%Y")
MES=$(date +"%m")
DIA=$(date +"%d")
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

DEST_DIR="scripts/backup/backups/${ANO}/${MES}/${DIA}/${PREFIX}_${TIMESTAMP}"
mkdir -p "$DEST_DIR"

echo "⏳ [1/3] Sincronizando transações SQLite (WAL checkpoint)..."
docker exec mta_sheet_app sqlite3 /app/data/mta_sheet.db "PRAGMA wal_checkpoint(TRUNCATE);" 2>/dev/null || true

echo "⏳ [2/3] Copiando banco de dados SQLite (/app/data)..."
docker cp mta_sheet_app:/app/data "$DEST_DIR/data"

echo "⏳ [3/3] Copiando pasta de uploads (/app/uploads)..."
docker cp mta_sheet_app:/app/uploads "$DEST_DIR/uploads" 2>/dev/null || true

if [ -f "$DEST_DIR/data/mta_sheet.db" ]; then
    echo ""
    echo "========================================================"
    echo "✅ Backup concluído com SUCESSO!"
    echo "📁 Destino: $DEST_DIR"
    echo "📦 Conteúdo do banco de dados:"
    ls -lh "$DEST_DIR/data"
    echo "========================================================"
    exit 0
else
    echo "❌ Erro: Falha ao validar cópia do banco de dados."
    exit 1
fi
