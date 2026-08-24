#!/bin/bash
set -e

# Navega para o diretorio raiz do projeto
cd "$(dirname "$0")/.."

echo "========================================================================"
echo "  MTA Sheet - Build Release Otimizado para Linux [Binario Standalone]"
echo "========================================================================"
echo ""

# 1. Carrega variaveis de ambiente se existir
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
fi

# 2. Cria diretorios essenciais
echo "[1/4] Preparando diretorios e assets..."
mkdir -p uploads
mkdir -p target/site/pkg
mkdir -p styles

# 3. Verifica target wasm32-unknown-unknown
echo "[2/4] Verificando target wasm32-unknown-unknown..."
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "  -> Instalando target wasm32-unknown-unknown..."
    rustup target add wasm32-unknown-unknown
fi

NEED_INSTALL_BINDGEN=0
if ! command -v wasm-bindgen &> /dev/null; then
    NEED_INSTALL_BINDGEN=1
else
    if ! wasm-bindgen --version 2>/dev/null | grep -q "0.2.121"; then
        echo "  -> wasm-bindgen desatualizado detectado. Atualizando para versao 0.2.121..."
        NEED_INSTALL_BINDGEN=1
    fi
fi

if [ "$NEED_INSTALL_BINDGEN" -eq 1 ]; then
    echo "  -> Instalando wasm-bindgen-cli versao 0.2.121..."
    cargo install wasm-bindgen-cli --version 0.2.121 --locked
fi

# 4. Compilacao
echo "[3/4] Compilando Frontend WASM e Backend com Assets Embutidos..."
echo "  -> [1/3] Compilando Frontend WASM Release..."
cargo build --target wasm32-unknown-unknown --release --no-default-features --features hydrate

echo "  -> [2/3] Gerando bindings JS e empacotando assets em pkg/..."
wasm-bindgen --target web --out-dir target/site/pkg --out-name mta_sheet target/wasm32-unknown-unknown/release/mta_sheet.wasm --no-typescript
cp style.css target/site/pkg/mta_sheet.css

echo "  -> [3/3] Compilando Servidor Backend SSR com Assets Embutidos..."
cargo build --release --no-default-features --features ssr

# 5. Empacotamento
echo "[4/4] Empacotando executavel unico..."

SERVER_BIN=""
if [ -f "target/release/mta_sheet_server" ]; then
    SERVER_BIN="target/release/mta_sheet_server"
elif [ -f "target/release/mta_sheet" ]; then
    SERVER_BIN="target/release/mta_sheet"
elif [ -f "target/server/release/mta_sheet_server" ]; then
    SERVER_BIN="target/server/release/mta_sheet_server"
elif [ -f "target/server/release/mta_sheet" ]; then
    SERVER_BIN="target/server/release/mta_sheet"
fi

if [ -z "$SERVER_BIN" ]; then
    echo "[ERRO CRITICO] Binario compilado nao foi encontrado em target/."
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
