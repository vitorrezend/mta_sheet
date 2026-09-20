#!/usr/bin/env bash
# ==============================================================================
# MTA Sheet - Restauracao de Backup Docker [Linux]
# ==============================================================================

set -euo pipefail

DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$DIR"

echo "========================================================"
echo "  MTA Sheet - Restauracao de Backup Docker"
echo "========================================================"
echo ""

if ! command -v docker &> /dev/null; then
    echo "❌ [ERRO] 'docker' não está instalado ou não está no PATH."
    exit 1
fi

BACKUP_PATH="${1:-}"
AUTO_CONFIRM=0
if [[ "${2:-}" == "--yes" || "${2:-}" == "-y" || "${1:-}" == "--yes" || "${1:-}" == "-y" ]]; then
    AUTO_CONFIRM=1
    if [[ "${1:-}" == "--yes" || "${1:-}" == "-y" ]]; then
        BACKUP_PATH=""
    fi
fi

if [ -z "$BACKUP_PATH" ]; then
    LATEST=$(find scripts/backup/backups -type f -name "mta_sheet.db" -printf '%T@ %p\n' 2>/dev/null | sort -n | tail -n 1 | awk '{print $2}')
    if [ -z "$LATEST" ]; then
        echo "❌ [ERRO] Nenhum backup válido contendo mta_sheet.db foi encontrado."
        exit 1
    fi
    BACKUP_PATH=$(dirname "$(dirname "$LATEST")")
fi

if [ ! -f "$BACKUP_PATH/data/mta_sheet.db" ]; then
    echo "❌ [ERRO] O caminho informado não contém data/mta_sheet.db: $BACKUP_PATH"
    exit 1
fi

echo "Backup selecionado para restauração:"
echo "   $BACKUP_PATH"
ls -lh "$BACKUP_PATH/data"
echo ""

if [ "$AUTO_CONFIRM" -ne 1 ]; then
    read -r -p "ATENÇÃO: Os dados atuais no container serão substituídos por este backup. Continuar? (s/N): " REPLY
    if [[ ! "$REPLY" =~ ^[sSyY]$ ]]; then
        echo "Restauração cancelada pelo operador."
        exit 2
    fi
fi

echo "⏳ [1/4] Parando temporariamente container para consistência..."
docker compose stop mta_sheet >/dev/null

echo "⏳ [2/4] Restaurando banco de dados SQLite..."
docker cp "$BACKUP_PATH/data" mta_sheet_app:/app/

if [ -d "$BACKUP_PATH/uploads" ]; then
    echo "⏳ [3/4] Restaurando arquivos de uploads..."
    docker cp "$BACKUP_PATH/uploads" mta_sheet_app:/app/
fi

echo "⏳ [4/4] Reiniciando aplicação..."
docker compose start mta_sheet >/dev/null

echo ""
echo "========================================================"
echo "✅ [SUCESSO] Backup restaurado e aplicação reiniciada!"
echo "========================================================"
