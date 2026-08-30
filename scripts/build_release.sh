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

echo "[1/4] Preparando diretorios..."
mkdir -p uploads target/site/pkg styles

echo "[2/4] Verificando target wasm32-unknown-unknown..."
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "  -> Instalando target wasm32-unknown-unknown..."
    rustup target add wasm32-unknown-unknown
fi

if ! command -v wasm-bindgen &> /dev/null; then
    echo "  -> Instalando wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli --version 0.2.121 --locked
fi

echo "[3/4] Compilando Frontend WASM e Backend..."
echo "  -> [1/3] Compilando Frontend WASM Release..."
cargo build --lib --target wasm32-unknown-unknown --release --no-default-features --features hydrate

echo "  -> [2/3] Gerando bindings JS em target/site/pkg..."
wasm-bindgen --target web --out-dir target/site/pkg --out-name mta_sheet target/wasm32-unknown-unknown/release/mta_sheet.wasm --no-typescript
cp style.css target/site/pkg/mta_sheet.css

echo "  -> [3/3] Compilando Servidor Backend SSR Release..."
cargo build --release --no-default-features --features ssr

echo "[4/4] Empacotando executavel..."
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
