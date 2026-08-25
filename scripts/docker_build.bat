@echo off
setlocal enabledelayedexpansion

cd /d "%~dp0.."

echo =======================================================
echo   MTA Sheet - Compilando Imagem Docker Ultra-Leve
echo =======================================================
echo.

where docker >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERRO] O Docker nao foi encontrado no PATH. Certifique-se de que o Docker Desktop esta aberto.
    pause
    exit /b 1
)

echo [INFO] Iniciando build multi-stage otimizado...
docker build -t mta_sheet:latest .

if %ERRORLEVEL% EQU 0 (
    echo.
    echo =======================================================
    echo [SUCESSO] Imagem Docker construida com sucesso!
    echo =======================================================
    echo.
    echo Tamanho da imagem gerada:
    docker images mta_sheet:latest
    echo.
    echo Para executar:
    echo   docker compose up -d  OU  scripts\docker_run.bat
    echo.
) else (
    echo.
    echo [ERRO] Falha ao construir a imagem Docker.
)

pause
