@echo off
setlocal enabledelayedexpansion

cd /d "%~dp0.."

echo =======================================================
echo   MTA Sheet - Executando via Docker Compose
echo =======================================================
echo.

where docker >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERRO] O Docker nao foi encontrado no PATH. Certifique-se de que o Docker Desktop esta aberto.
    pause
    exit /b 1
)

echo Opcoes:
echo   [1] Iniciar container em segundo plano (docker compose up -d)
echo   [2] Parar container (docker compose down)
echo   [3] Ver logs em tempo real (docker compose logs -f)
echo   [4] Reiniciar container (docker compose restart)
echo.

set /p OPT="Escolha uma opcao [1-4, padrao 1]: "
if "%OPT%"=="" set OPT=1

if "%OPT%"=="1" (
    echo.
    echo [INFO] Subindo aplicacao no Docker...
    docker compose up -d
    if !ERRORLEVEL! EQU 0 (
        echo.
        echo [SUCESSO] MTA Sheet esta rodando em: http://localhost:3000
        start http://localhost:3000
    )
) else if "%OPT%"=="2" (
    echo.
    echo [INFO] Encerrando container...
    docker compose down
) else if "%OPT%"=="3" (
    echo.
    echo [INFO] Exibindo logs (Ctrl+C para sair)...
    docker compose logs -f
) else if "%OPT%"=="4" (
    echo.
    echo [INFO] Reiniciando container...
    docker compose restart
)

pause
