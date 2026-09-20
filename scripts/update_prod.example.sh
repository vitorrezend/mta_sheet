#!/usr/bin/env bash
# ==============================================================================
# MTA Sheet - Esteira Automatizada de Deploy em Producao (Near-Zero Downtime)
# ==============================================================================

set -euo pipefail

DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$DIR"

echo "=============================================================================="
echo "  MTA Sheet - Esteira Automatizada de Deploy em Producao (Zero Downtime)"
echo "=============================================================================="
echo ""

if ! command -v docker &> /dev/null; then
    echo "❌ [ERRO CRITICO] 'docker' não está instalado ou não está no PATH."
    exit 1
fi

SKIP_TESTS=0
if [[ "${1:-}" == "--skip-tests" || "${1:-}" == "-s" ]]; then
    SKIP_TESTS=1
fi

# ------------------------------------------------------------------------------
# ETAPA 1: Quality Gate & Testes Automatizados
# ------------------------------------------------------------------------------
echo "⏳ [1/5] Quality Gate: Executando suíte de testes automatizados..."
if [ "$SKIP_TESTS" -eq 1 ]; then
    echo "⚠️ [AVISO] Flag --skip-tests detectada. Testes automatizados ignorados pelo operador."
else
    if command -v cargo &> /dev/null; then
        if ! cargo test --features ssr -- --quiet; then
            echo ""
            echo "❌ [ERRO CRITICO] A suíte de testes automatizados FALHOU!"
            echo "O deploy foi CANCELADO para proteger o ambiente de produção."
            echo "O container de produção atual permanece 100% INTACTO e OPERACIONAL."
            exit 1
        fi
        echo "✅ [OK] Todos os testes unitários e de integração passaram com sucesso!"
    else
        echo "⚠️ [AVISO] 'cargo' não encontrado no PATH do host. Pulando testes locais..."
    fi
fi
echo ""

# ------------------------------------------------------------------------------
# ETAPA 2: Snapshot de Segurança Pré-Deploy (Backup Automático)
# ------------------------------------------------------------------------------
echo "⏳ [2/5] Snapshot de Segurança: Realizando backup preventivo dos dados..."
BACKUP_SCRIPT="$(dirname "$0")/backup/backup-docker.sh"
if [ -f "$BACKUP_SCRIPT" ]; then
    set +e
    bash "$BACKUP_SCRIPT" pre_deploy
    BACKUP_STATUS=$?
    set -e
    if [ "$BACKUP_STATUS" -eq 1 ]; then
        echo ""
        echo "❌ [ERRO CRITICO] Falha ao realizar o backup preventivo de segurança!"
        echo "O deploy foi CANCELADO por precaução para evitar perda de dados."
        exit 1
    fi
else
    echo "⚠️ [AVISO] Script de backup não encontrado em $BACKUP_SCRIPT. Prosseguindo..."
fi
echo ""

# ------------------------------------------------------------------------------
# ETAPA 3: Build em Background com Cache (Serviço Continua Online)
# ------------------------------------------------------------------------------
echo "⏳ [3/5] Build em Segundo Plano: Compilando nova imagem (Serviço permanece ONLINE)..."
if ! docker compose build mta_sheet; then
    echo ""
    echo "❌ [ERRO CRITICO] A compilação da nova imagem falhou!"
    echo "O container anterior NÃO foi modificado e continua atendendo usuários normalmente."
    exit 1
fi
echo "✅ [OK] Nova imagem construída com sucesso!"
echo ""

# ------------------------------------------------------------------------------
# ETAPA 4: Hot-Swap Atômico de Container (Downtime ~1s)
# ------------------------------------------------------------------------------
echo "⏳ [4/5] Hot-Swap: Atualizando container reaproveitando volumes persistentes..."
if ! docker compose up -d --no-deps mta_sheet; then
    echo ""
    echo "❌ [ERRO CRITICO] Falha ao recriar o container com a nova imagem!"
    exit 1
fi
echo "✅ [OK] Container recriado com a nova versão!"
echo ""

# ------------------------------------------------------------------------------
# ETAPA 5: Health Check & Validação Pós-Deploy
# ------------------------------------------------------------------------------
echo "⏳ [5/5] Validação Pós-Deploy: Monitorando saúde do novo container..."
HEALTHCHECK_SCRIPT="$(dirname "$0")/healthcheck.sh"
if [ -f "$HEALTHCHECK_SCRIPT" ]; then
    bash "$HEALTHCHECK_SCRIPT"
else
    curl -s -f http://localhost:8080/ >/dev/null 2>&1 || exit 1
fi

echo ""
echo "Status dos containers ativos:"
docker compose ps
echo ""
echo "Acesse a aplicação em:"
echo "  - Local Direto: http://localhost:8080"
echo "  - Produção/Proxy: http://localhost:80 / https://dominio (se Caddy estiver configurado)"
