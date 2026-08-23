@echo off
setlocal

:: Navega para a raiz do projeto a partir de scripts/
cd /d "%~dp0.."

echo ==========================================
echo   Iniciando MTA Sheet (Modo Windows)
echo ==========================================

if exist .env (
    for /f "usebackq tokens=*" %%i in (".env") do set %%i
)

if "%DATABASE_URL%"=="" set DATABASE_URL=sqlite:mta_sheet.db
if "%LEPTOS_SITE_ADDR%"=="" set LEPTOS_SITE_ADDR=0.0.0.0:3000

echo Banco de dados: %DATABASE_URL%
echo Endereco: http://%LEPTOS_SITE_ADDR%

cargo leptos --version >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo [INFO] Usando cargo-leptos para iniciar o projeto...
    cargo leptos watch
) else (
    echo [AVISO] cargo-leptos nao encontrado.
    echo [INFO] Tentando iniciar via cargo run...
    cargo run --features ssr
)

pause
