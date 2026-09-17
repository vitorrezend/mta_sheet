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
if command -v cargo-leptos &> /dev/null; then
    cargo leptos build --release
else
    echo "  -> Instalando / usando cargo build direto..."
    cargo build --lib --target wasm32-unknown-unknown --release --no-default-features --features hydrate
    wasm-bindgen --target web --out-dir target/site/pkg --out-name mta_sheet target/wasm32-unknown-unknown/release/mta_sheet.wasm --no-typescript
    if command -v wasm-opt &> /dev/null; then
        wasm-opt -Oz target/site/pkg/mta_sheet_bg.wasm -o target/site/pkg/mta_sheet_bg.wasm
    fi
    cargo build --release --no-default-features --features ssr
fi

# Empacota toda a suite de estilos em um unico arquivo CSS otimizado
echo "  -> [CSS Bundle] Concatenando suite de estilos em target/site/pkg/mta_sheet.css..."
cat styles/01-variables.css styles/02-common.css styles/03-sheet-layout.css styles/04-page1-main.css styles/05-page2-magic-combat.css styles/06-page3-expanded.css styles/07-page4-history-visuals.css styles/08-print-pdf.css styles/09-gods-and-monsters.css styles/10-page5-grimoire.css styles/11-page6-notes.css styles/12-compendium.css styles/patch_notes.css > target/site/pkg/mta_sheet.css
if [ -f "target/site/pkg/mta_sheet_bg.wasm" ]; then
    cp -f target/site/pkg/mta_sheet_bg.wasm target/site/pkg/mta_sheet.wasm
elif [ -f "target/site/pkg/mta_sheet.wasm" ]; then
    cp -f target/site/pkg/mta_sheet.wasm target/site/pkg/mta_sheet_bg.wasm
fi

echo "[3/3] Empacotando executavel standalone..."
SERVER_BIN=""
if [ -f "target/release/mta_sheet_server" ]; then
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
