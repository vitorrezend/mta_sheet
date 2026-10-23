#!/bin/bash
# ==============================================================================
# MTA Sheet - Checagem completa antes de um release (substitui um CI)
#
#   1. clippy (erros de lint barram; avisos antigos apenas aparecem)
#   2. testes Rust (unidade + integração)
#   3. build do app e testes de ponta a ponta no motor do Safari (WebKit/iPhone)
#
# Uso:
#   scripts/check.sh            # tudo
#   scripts/check.sh --fast     # só clippy + testes Rust (sem build/E2E)
#
# Os testes E2E sobem uma instância isolada na porta 3099 com um banco descartável
# em target/e2e/, sem tocar no seu banco nem no servidor que estiver rodando.
# ==============================================================================
set -euo pipefail

DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$DIR"

PORT=3099
OUT="target/e2e"
FAST=0
[ "${1:-}" = "--fast" ] && FAST=1

step() { echo; echo "=== $* ==="; }

step "1/4 Clippy"
cargo clippy --features ssr --lib --bin mta_sheet_server

step "2/4 Testes Rust (unidade + integração)"
cargo test --features ssr --lib --test all_tests

if [ "$FAST" = "1" ]; then
    echo; echo "[OK] Checagem rápida concluída (sem E2E)."
    exit 0
fi

step "3/4 Build do app (cargo leptos build)"
cargo leptos build

step "4/4 Testes de ponta a ponta no WebKit (Safari)"
if [ ! -d scripts/e2e/node_modules ]; then
    (cd scripts/e2e && npm install --no-audit --no-fund)
fi
(cd scripts/e2e && npx playwright install webkit)

mkdir -p "$OUT"
rm -f "$OUT/check.db" "$OUT/check.db-shm" "$OUT/check.db-wal"

SERVER_BIN="target/debug/mta_sheet_server"
[ -f "$SERVER_BIN.exe" ] && SERVER_BIN="$SERVER_BIN.exe"
DATABASE_URL="sqlite:$OUT/check.db" LEPTOS_SITE_ADDR="127.0.0.1:$PORT" RUST_LOG=warn \
    "$SERVER_BIN" > "$OUT/server.log" 2>&1 &
SERVER_PID=$!
trap 'kill "$SERVER_PID" 2>/dev/null || true' EXIT

for _ in $(seq 1 60); do
    curl -s -o /dev/null -w "%{http_code}" "http://127.0.0.1:$PORT/" | grep -q 200 && break
    sleep 1
done

BASE="http://127.0.0.1:$PORT"
FAILED=0
run_suite() {
    echo; echo "--- $1"
    node "scripts/e2e/$1" "$BASE" "$OUT" "${2:-}" || FAILED=1
}

# O cadastro é limitado a 5 por minuto por IP; cada suíte cria usuários próprios
run_suite room_chat.js webkit
sleep 61
run_suite room_features.js
sleep 61
run_suite screens_sweep.js iphone
sleep 61
run_suite screens_sweep.js desktop

echo
if [ "$FAILED" = "0" ]; then
    echo "[OK] Checagem completa: clippy, testes Rust e E2E no WebKit passaram."
    echo "     Capturas de tela em $OUT/"
else
    echo "[FALHOU] Algum teste de ponta a ponta falhou. Veja acima e as capturas em $OUT/"
    exit 1
fi
