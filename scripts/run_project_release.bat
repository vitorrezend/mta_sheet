@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

:: Navega para a raiz do projeto
cd /d "%~dp0.."

echo =======================================================
echo   🧙 MTA Sheet - Executando em Modo Release (Windows)
echo =======================================================
echo.

if not exist "mta_sheet.exe" (
    echo [AVISO] O executavel mta_sheet.exe nao foi encontrado na raiz.
    echo [INFO] Iniciando build de release primeiro...
    call "scripts\build_release_windows.bat"
    if not exist "mta_sheet.exe" (
        echo [ERRO] Executavel nao disponivel.
        pause
        exit /b 1
    )
)

:: Carrega variaveis de ambiente
if exist .env (
    for /f "usebackq tokens=* eol=#" %%i in (".env") do (
        set "%%i"
    )
)

if "%DATABASE_URL%"=="" set DATABASE_URL=sqlite:mta_sheet.db
if "%LEPTOS_SITE_ADDR%"=="" set LEPTOS_SITE_ADDR=127.0.0.1:3000
if "%RUST_LOG%"=="" set RUST_LOG=warn

echo [CONFIG] Banco de Dados: %DATABASE_URL%
echo [CONFIG] Endereco:       http://%LEPTOS_SITE_ADDR%
echo [CONFIG] Logs (RUST_LOG): %RUST_LOG%
echo.

echo Abrindo navegador em http://%LEPTOS_SITE_ADDR%...
start http://%LEPTOS_SITE_ADDR%

echo Iniciando servidor standalone...
.\mta_sheet.exe

pause
