@echo off
:: ==============================================================================
# MTA Sheet - DuckDNS Dynamic IP Updater (Template de Exemplo)
# Copie este arquivo para 'duckdns_update.bat' e insira o seu token real.
# O arquivo 'duckdns_update.bat' é automaticamente ignorado pelo .gitignore.
:: ==============================================================================

set "DUCKDNS_TOKEN=SEU_TOKEN_AQUI"
set "DUCKDNS_DOMAINS=mta-sheet,wod-sheet"

echo [DuckDNS] Atualizando IP dinamico para dominios: %DUCKDNS_DOMAINS%...
curl -s "https://www.duckdns.org/update?domains=%DUCKDNS_DOMAINS%&token=%DUCKDNS_TOKEN%&ip="
echo.
echo [DuckDNS] Concluido!
