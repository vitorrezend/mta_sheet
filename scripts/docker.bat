@echo off
setlocal enabledelayedexpansion

cd /d "%~dp0.."

echo =======================================================
echo   MTA Sheet - Container / Docker
echo =======================================================
echo.

where docker >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERRO] O Docker nao foi encontrado no PATH. Certifique-se de que o Docker Desktop esta aberto.
    pause
    exit /b 1
)

:: Argumentos diretos via linha de comando
if /i "%1"=="build" goto :do_build
if /i "%1"=="up" goto :do_up
if /i "%1"=="down" goto :do_down
if /i "%1"=="logs" goto :do_logs
if /i "%1"=="restart" goto :do_restart

echo Opcoes:
echo   [1] Compilar imagem Docker (docker build -t mta_sheet:latest .)
echo   [2] Iniciar container em segundo plano (docker compose up -d)
echo   [3] Parar container (docker compose down)
echo   [4] Ver logs em tempo real (docker compose logs -f)
echo   [5] Reiniciar container (docker compose restart)
echo.

set /p OPT="Escolha uma opcao [1-5, padrao 2]: "
if "%OPT%"=="" set OPT=2

if "%OPT%"=="1" goto :do_build
if "%OPT%"=="2" goto :do_up
if "%OPT%"=="3" goto :do_down
if "%OPT%"=="4" goto :do_logs
if "%OPT%"=="5" goto :do_restart

:do_build
echo.
echo [INFO] Compilando imagem Docker multi-stage...
docker build -t mta_sheet:latest .
if %ERRORLEVEL% EQU 0 (
    echo.
    echo [SUCESSO] Imagem construida com sucesso!
    docker images mta_sheet:latest
)
goto :fim

:do_up
echo.
echo [INFO] Subindo aplicacao no Docker Compose...
docker compose up -d
if %ERRORLEVEL% EQU 0 (
    echo.
    echo [SUCESSO] MTA Sheet esta rodando em: http://localhost:3000
    start http://localhost:3000
)
goto :fim

:do_down
echo.
echo [INFO] Encerrando container...
docker compose down
goto :fim

:do_logs
echo.
echo [INFO] Exibindo logs (Ctrl+C para sair)...
docker compose logs -f
goto :fim

:do_restart
echo.
echo [INFO] Reiniciando container...
docker compose restart
goto :fim

:fim
pause
