#!/bin/bash
set -e

# ==============================================================================
# MTA Sheet - Script de Cross-Compilação Otimizada para Windows (.exe Standalone)
# Executável a partir do Linux usando Cargo + Zig + Rust-Embed
# ==============================================================================

# Navega até a raiz do projeto
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

# Garante que ~/.cargo/bin e ~/.local/bin estejam no PATH
export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"

echo "========================================================================"
echo "  🧙 MTA Sheet — Build Otimizado Standalone para Windows (.exe)"
echo "========================================================================"

# Carrega variáveis de ambiente caso exista .env
if [ -f .env ]; then
    set -a
    source .env
    set +a
fi

echo "[1/4] Verificando dependências de compilação cruzada..."

# 1. Garante os targets do Rust
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "  -> Instalando target wasm32-unknown-unknown..."
    rustup target add wasm32-unknown-unknown
fi

if ! rustup target list | grep -q "x86_64-pc-windows-gnu (installed)"; then
    echo "  -> Instalando target x86_64-pc-windows-gnu..."
    rustup target add x86_64-pc-windows-gnu
fi

# 2. Garante cargo-zigbuild e zig
if ! command -v cargo-zigbuild &> /dev/null; then
    echo "  -> Instalando cargo-zigbuild..."
    if command -v cargo-binstall &> /dev/null; then
        cargo binstall -y cargo-zigbuild
    else
        cargo install cargo-zigbuild
    fi
fi

if ! command -v zig &> /dev/null; then
    echo "  -> [AVISO] Compilador 'zig' não encontrado no PATH."
    echo "  -> Baixando e configurando Zig standalone..."
    mkdir -p "$HOME/.local/opt"
    curl -L "https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz" -o /tmp/zig.tar.xz
    tar -xf /tmp/zig.tar.xz -C "$HOME/.local/opt"
    ln -sf "$HOME/.local/opt/zig-linux-x86_64-0.13.0/zig" "$HOME/.cargo/bin/zig"
    echo "  -> Zig configurado com sucesso!"
fi

echo "[2/4] Preparando diretórios e assets..."
mkdir -p target/site/pkg
mkdir -p styles
mkdir -p uploads

echo "[3/4] Compilando e otimizando Frontend WebAssembly (WASM + JS + CSS)..."
if command -v cargo-leptos &> /dev/null; then
    cargo leptos build --release
else
    echo "  -> Compilando via Cargo WASM nativo..."
    cargo build --target wasm32-unknown-unknown --release --no-default-features --features hydrate
fi

echo "[4/4] Cross-compilando Servidor Backend para Windows (.exe) com Assets Embutidos..."
echo "  -> Flags de Otimização: LTO=true, Opt-Level='z' (Tamanho), Strip=true, Codegen-Units=1"
cargo zigbuild --target x86_64-pc-windows-gnu --release --no-default-features --features ssr

WIN_BIN="target/x86_64-pc-windows-gnu/release/mta_sheet.exe"

if [ ! -f "$WIN_BIN" ]; then
    echo "❌ Erro: Executável Windows não foi gerado em $WIN_BIN."
    exit 1
fi

# Copia para a raiz como executável único
cp "$WIN_BIN" "./mta_sheet.exe"

SIZE=$(du -h "./mta_sheet.exe" | cut -f1)

echo ""
echo "========================================================================"
echo "  🎉 Executável Único para Windows Gerado com Sucesso!"
echo "========================================================================"
echo "  📍 Arquivo : $PROJECT_ROOT/mta_sheet.exe"
echo "  📦 Tamanho : $SIZE"
echo "========================================================================"
echo ""
echo "Características do binário gerado:"
echo "  ✔ 100% Autocontido: Frontend WASM, JS, CSS, Fontes e HTML embutidos dentro do .exe."
echo "  ✔ Zero Dependências Externas: Não requer Node.js, nem Python, nem DLLs extras no Windows."
echo "  ✔ Banco de Dados SQLite Local: Cria e gerencia automaticamente 'mta_sheet.db' na mesma pasta."
echo ""
echo "Como usar no Windows:"
echo "  1. Copie APENAS o arquivo 'mta_sheet.exe' para qualquer máquina Windows (10/11 x64)."
echo "  2. Dê dois cliques em 'mta_sheet.exe' ou execute no Prompt de Comando / PowerShell."
echo "  3. Abra o navegador em: http://localhost:3000"
echo ""
