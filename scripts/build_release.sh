#!/bin/bash
set -e

# Navega até a raiz do projeto
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

# Garante que as ferramentas do Cargo (~/.cargo/bin) estejam no PATH
export PATH="$HOME/.cargo/bin:$PATH"

echo "========================================================"
echo "  MTA Sheet - Compilação Standalone (Executável Único)"
echo "========================================================"

# Carrega variáveis de ambiente caso exista .env
if [ -f .env ]; then
    set -a
    source .env
    set +a
fi

echo "[1/3] Preparando diretórios e assets..."
mkdir -p target/site/pkg
mkdir -p styles
mkdir -p uploads

echo "[2/3] Compilando Frontend WASM + Backend com Assets Embutidos..."

if command -v cargo-leptos &> /dev/null; then
    cargo leptos build --release
else
    echo "  -> [AVISO] 'cargo-leptos' não encontrado no PATH."
    echo "  -> Compilando diretamente via Cargo nativo com --release..."
    cargo build --release --features ssr
fi

echo "[3/3] Empacotando executável único..."

SERVER_BIN=""
if [ -f "target/server/release/mta_sheet_server" ]; then
    SERVER_BIN="target/server/release/mta_sheet_server"
elif [ -f "target/server/release/mta_sheet" ]; then
    SERVER_BIN="target/server/release/mta_sheet"
elif [ -f "target/release/mta_sheet_server" ]; then
    SERVER_BIN="target/release/mta_sheet_server"
elif [ -f "target/release/mta_sheet" ]; then
    SERVER_BIN="target/release/mta_sheet"
fi

if [ -z "$SERVER_BIN" ]; then
    echo "❌ Erro: Binário compilado não foi encontrado em target/."
    exit 1
fi

# Copia para o executável único na raiz
cp "$SERVER_BIN" "./mta_sheet"
chmod +x "./mta_sheet"

SIZE=$(du -h "./mta_sheet" | cut -f1)

echo ""
echo "========================================================"
echo "  ✅ Executável Único Gerado com Sucesso!"
echo "========================================================"
echo "  📍 Arquivo : $PROJECT_ROOT/mta_sheet"
echo "  📦 Tamanho : $SIZE"
echo "========================================================"
echo ""
echo "Este arquivo é 100% autocontido (WASM, JS, CSS e estilos embutidos)."
echo "Você pode copiar APENAS o arquivo 'mta_sheet' para qualquer máquina Linux e rodar com:"
echo "  ./mta_sheet"
echo ""
