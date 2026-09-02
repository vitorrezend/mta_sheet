#!/usr/bin/env bash
# ==============================================================================
# MTA Sheet - Backup SQLite com Desfragmentação (VACUUM INTO)
# ==============================================================================

set -euo pipefail

# ==============================================================================
# ⚙️ CONFIGURAÇÕES DE CAMINHO
# ==============================================================================
# Caminho de origem do banco SQLite
DB_SOURCE_PATH="$HOME/Dados_Extras/mta-sheet/mta_sheet/target/release/mta_sheet.db"

# Diretório base de destino para os backups
BACKUP_BASE_DEST="$HOME/Dados_Extras/mta-sheet/mta_sheet/scripts/backup/backups"

# ==============================================================================
# 🔧 VALIDAÇÕES
# ==============================================================================
# Expande til (~) se o usuário digitar ~/ no caminho
DB_SOURCE_PATH="${DB_SOURCE_PATH/#\~/$HOME}"
BACKUP_BASE_DEST="${BACKUP_BASE_DEST/#\~/$HOME}"

if ! command -v sqlite3 &> /dev/null; then
    echo "❌ Erro: 'sqlite3' não está instalado."
    exit 1
fi

if [ ! -f "$DB_SOURCE_PATH" ]; then
    echo "❌ Erro: Banco de dados não encontrado em: $DB_SOURCE_PATH"
    exit 1
fi

# ==============================================================================
# 🚀 EXECUÇÃO (ANO/MÊS/DIA)
# ==============================================================================
ANO=$(date +"%Y")
MES=$(date +"%m")
DIA=$(date +"%d")
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

DEST_DIR="${BACKUP_BASE_DEST}/${ANO}/${MES}/${DIA}"
mkdir -p "$DEST_DIR"

DB_FILENAME=$(basename "$DB_SOURCE_PATH" .db)
DEST_FILE="${DEST_DIR}/${DB_FILENAME}_vacuum_${TIMESTAMP}.db"

echo "⏳ Iniciando backup com desfragmentação de $DB_SOURCE_PATH..."
sqlite3 "$DB_SOURCE_PATH" "VACUUM INTO '$DEST_FILE';"

# Verificação de integridade
INTEGRIDADE=$(sqlite3 "$DEST_FILE" "PRAGMA integrity_check;")
if [ "$INTEGRIDADE" = "ok" ]; then
    TAMANHO=$(du -h "$DEST_FILE" | cut -f1)
    echo "✅ Backup desfragmentado concluído com SUCESSO!"
    echo "📁 Salvo em: $DEST_FILE"
    echo "📦 Tamanho : $TAMANHO"
    echo "🔍 Integridade : OK"
else
    echo "❌ Erro na integridade do backup: $INTEGRIDADE"
    exit 1
fi
