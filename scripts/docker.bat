@echo off
setlocal enabledelayedexpansion

cd /d "%~dp0.."

echo =======================================================
echo   MTA Sheet - Container / Docker
echo =======================================================
echo.

where docker.exe >nul 2>nul
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
if /i "%1"=="update" goto :do_update
if /i "%1"=="backup" goto :do_backup
if /i "%1"=="restore" goto :do_restore

echo Opcoes:
echo   [1] Compilar imagem Docker (docker build -t mta_sheet:latest .)
echo   [2] Iniciar container em segundo plano (docker compose up -d)
echo   [3] Parar container (docker compose down)
echo   [4] Ver logs em tempo real (docker compose logs -f)
echo   [5] Reiniciar container (docker compose restart)
echo   [6] Atualizar producao sem downtime (Esteira: Testes + Backup + Build + Swap)
echo   [7] Fazer backup dos dados (SQLite + Uploads)
echo   [8] Restaurar backup dos dados
echo.

set /p OPT="Escolha uma opcao [1-8, padrao 2]: "
if "%OPT%"=="" set OPT=2

if "%OPT%"=="1" goto :do_build
if "%OPT%"=="2" goto :do_up
if "%OPT%"=="3" goto :do_down
if "%OPT%"=="4" goto :do_logs
if "%OPT%"=="5" goto :do_restart
if "%OPT%"=="6" goto :do_update
if "%OPT%"=="7" goto :do_backup
if "%OPT%"=="8" goto :do_restore

:do_build
echo.
echo [INFO] Compilando imagem Docker multi-stage...
docker.exe build -t mta_sheet:latest .
if %ERRORLEVEL% EQU 0 (
    echo.
    echo [SUCESSO] Imagem construida com sucesso!
    docker.exe images mta_sheet:latest
)
goto :fim

:do_up
echo.
echo [INFO] Subindo aplicacao no Docker Compose...
docker.exe compose up -d
if %ERRORLEVEL% EQU 0 (
    echo.
    echo [SUCESSO] MTA Sheet esta rodando no Docker!
    echo   - Acesso Local Direto: http://localhost:8080
    echo   - Acesso HTTPS Seguro: https://%DOMAIN_NAME% (Porta 443 via Caddy)
    start http://localhost:8080
)
goto :fim

:do_down
echo.
echo [INFO] Encerrando container...
docker.exe compose down
goto :fim

:do_logs
echo.
echo [INFO] Exibindo logs (Ctrl+C para sair)...
docker.exe compose logs -f
goto :fim

:do_restart
echo.
echo [INFO] Reiniciando container...
docker.exe compose restart
goto :fim

:do_update
echo.
echo [INFO] Iniciando esteira de atualizacao sem downtime...
call "%~dp0update_prod.bat"
goto :fim

:do_backup
echo.
echo [INFO] Iniciando backup dos dados do container...
call "%~dp0backup\backup-docker.bat"
goto :fim

:do_restore
echo.
echo [INFO] Iniciando restauracao de backup...
call "%~dp0backup\restore-docker.bat"
goto :fim

:fim
pause
