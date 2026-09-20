#!/usr/bin/env bash
# ==============================================================================
# MTA Sheet - Healthcheck Pos-Deploy (Linux / Bash)
# ==============================================================================

set -euo pipefail

MAX_ATTEMPTS=15
ATTEMPT=1
HEALTHY=0

while [ "$ATTEMPT" -le "$MAX_ATTEMPTS" ]; do
    if curl -s -f -m 3 http://localhost:8080/ >/dev/null 2>&1; then
        HEALTHY=1
        break
    fi
    STATUS=$(docker inspect --format '{{.State.Health.Status}}' mta_sheet_app 2>/dev/null || true)
    if [ "$STATUS" = "healthy" ]; then
        HEALTHY=1
        break
    fi
    echo "   Aguardando inicialização do servidor... ($ATTEMPT/$MAX_ATTEMPTS)"
    sleep 2
    ATTEMPT=$((ATTEMPT + 1))
done

if [ "$HEALTHY" -eq 1 ]; then
    echo ""
    echo "=============================================================================="
    echo "✅ [SUCESSO] MTA Sheet atualizado e operacional com Near-Zero Downtime!"
    echo "=============================================================================="
    exit 0
else
    echo ""
    echo "❌ [ALERTA CRITICO] O novo container não respondeu dentro do tempo limite!"
    docker compose logs --tail=30 mta_sheet
    exit 1
fi
