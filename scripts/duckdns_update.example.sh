#!/bin/bash
# ==============================================================================
# MTA Sheet - DuckDNS Dynamic IP Updater (Template de Exemplo para Linux)
# Copie este arquivo para 'duckdns_update.sh' e insira o seu token real.
# O arquivo 'duckdns_update.sh' é automaticamente ignorado pelo .gitignore.
# ==============================================================================

DUCKDNS_TOKEN="SEU_TOKEN_AQUI"
DUCKDNS_DOMAINS="mta-sheet,wod-sheet"

echo "[DuckDNS] Atualizando IP dinamico para dominios: $DUCKDNS_DOMAINS..."
curl -s "https://www.duckdns.org/update?domains=$DUCKDNS_DOMAINS&token=$DUCKDNS_TOKEN&ip="
echo ""
echo "[DuckDNS] Concluido!"
