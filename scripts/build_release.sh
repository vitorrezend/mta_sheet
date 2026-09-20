#!/bin/bash
set -e

DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$DIR"

echo "========================================================================"
echo "  MTA Sheet - Build Release Standalone [Linux]"
echo "========================================================================"
echo ""

if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
fi

echo "[1/3] Preparando diretorios e limpando assets antigos..."
mkdir -p uploads target/site/pkg styles
rm -rf target/site/pkg/*

echo "[2/3] Compilando Frontend WASM e Backend via cargo-leptos release..."
if ! command -v cargo-leptos &> /dev/null; then
    echo "  -> cargo-leptos nao encontrado. Instalando..."
    curl -sSfL https://raw.githubusercontent.com/leptos-rs/cargo-leptos/main/cargo-leptos-installer.sh | sh
fi

cargo leptos build --release

if [ -f "target/site/pkg/mta_sheet.wasm" ] && [ ! -f "target/site/pkg/mta_sheet_bg.wasm" ]; then
    cp -f target/site/pkg/mta_sheet.wasm target/site/pkg/mta_sheet_bg.wasm
elif [ -f "target/site/pkg/mta_sheet_bg.wasm" ] && [ ! -f "target/site/pkg/mta_sheet.wasm" ]; then
    cp -f target/site/pkg/mta_sheet_bg.wasm target/site/pkg/mta_sheet.wasm
fi

# Empacota toda a suite de estilos em um unico arquivo CSS otimizado
echo "  -> [CSS Bundle] Concatenando suite de estilos em target/site/pkg/mta_sheet.css..."
cat styles/00-tokens.css styles/01-variables.css styles/02-common.css styles/03-sheet-layout.css styles/04-page1-main.css styles/05-page2-magic-combat.css styles/06-page3-expanded.css styles/07-page4-history-visuals.css styles/08-print-pdf.css styles/09-gods-and-monsters.css styles/10-page5-grimoire.css styles/11-page6-notes.css styles/12-compendium.css styles/patch_notes.css styles/home.css styles/auth.css styles/rooms.css styles/logs.css styles/feed.css styles/profile.css styles/share.css styles/about.css styles/compendium-navigation.css > target/site/pkg/mta_sheet.css

echo "[3/3] Empacotando executavel standalone..."
SERVER_BIN=""
if [ -f "target/server/release/mta_sheet_server" ]; then
    SERVER_BIN="target/server/release/mta_sheet_server"
elif [ -f "target/release/mta_sheet_server" ]; then
    SERVER_BIN="target/release/mta_sheet_server"
elif [ -f "target/release/mta_sheet" ]; then
    SERVER_BIN="target/release/mta_sheet"
fi

if [ -z "$SERVER_BIN" ]; then
    echo "[ERRO CRITICO] Binario compilado nao foi encontrado em target/release."
    exit 1
fi

cp "$SERVER_BIN" "./mta_sheet"
chmod +x "./mta_sheet"

SIZE=$(du -h "./mta_sheet" | cut -f1)

echo ""
echo "========================================================================"
echo "  Executavel Standalone para Linux Gerado com Sucesso!"
echo "========================================================================"
echo "  Localizacao : $(pwd)/mta_sheet"
echo "  Tamanho     : $SIZE"
echo "========================================================================"
