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

# Assegura que style.css e fallback existam em target/site/pkg
cp style.css target/site/pkg/mta_sheet.css
if [ -f "target/site/pkg/mta_sheet_bg.wasm" ] && [ ! -f "target/site/pkg/mta_sheet.wasm" ]; then
    cp target/site/pkg/mta_sheet_bg.wasm target/site/pkg/mta_sheet.wasm
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
