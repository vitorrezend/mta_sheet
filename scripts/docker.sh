#!/bin/sh
set -e

DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$DIR"

echo "======================================================="
echo "  MTA Sheet - Container / Docker"
echo "======================================================="
echo ""

if ! command -v docker &> /dev/null; then
    echo "[ERRO] Docker nao encontrado no PATH."
    exit 1
fi

ACTION="$1"

if [ -z "$ACTION" ]; then
    echo "Opcoes:"
    echo "  1) Compilar imagem Docker (docker build)"
    echo "  2) Iniciar container (docker compose up -d)"
    echo "  3) Parar container (docker compose down)"
    echo "  4) Ver logs (docker compose logs -f)"
    echo "  5) Reiniciar container (docker compose restart)"
    echo ""
    read -p "Escolha uma opcao [1-5, padrao 2]: " OPT
    OPT="${OPT:-2}"
    case "$OPT" in
        1) ACTION="build" ;;
        2) ACTION="up" ;;
        3) ACTION="down" ;;
        4) ACTION="logs" ;;
        5) ACTION="restart" ;;
        *) ACTION="up" ;;
    esac
fi

case "$ACTION" in
    build)
        echo "[INFO] Compilando imagem Docker..."
        docker build -t mta_sheet:latest .
        echo "[SUCESSO] Imagem construida!"
        docker images mta_sheet:latest
        ;;
    up)
        echo "[INFO] Subindo container..."
        docker compose up -d
        echo "[SUCESSO] MTA Sheet rodando em: http://localhost:3000"
        ;;
    down)
        echo "[INFO] Encerrando container..."
        docker compose down
        ;;
    logs)
        echo "[INFO] Exibindo logs (Ctrl+C para sair)..."
        docker compose logs -f
        ;;
    restart)
        echo "[INFO] Reiniciando container..."
        docker compose restart
        ;;
    *)
        echo "[ERRO] Opcao invalida: $ACTION"
        exit 1
        ;;
esac
