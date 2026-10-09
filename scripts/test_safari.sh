#!/bin/bash
# ==============================================================================
# MTA Sheet — Safari (WebKit) Mobile Testing Script
# Abre a aplicação no motor nativo WebKit (Safari) emulando um iPhone.
# ==============================================================================

TARGET_URL="${1:-http://localhost:3000/?debug=true}"

echo ""
echo "====================================================================="
echo "  MTA Sheet - Teste no Safari (WebKit) com Emulação de iPhone"
echo "====================================================================="
echo ""
echo "URL de destino: $TARGET_URL"
echo ""
echo "1. Abrindo navegador WebKit nativo com viewport e toque de iPhone 14..."
echo "2. Pressione Ctrl+C ou feche a janela do navegador para encerrar."
echo ""

npx playwright wk --device="iPhone 14" "$TARGET_URL"
