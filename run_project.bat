@echo off
setlocal

echo ==========================================
echo   Iniciando MTA Sheet (Modo Full-Stack)
echo ==========================================

:: Tenta carregar variaveis do .env se existir
if exist .env (
    for /f "usebackq tokens=*" %%i in (".env") do set %%i
)

:: Define padrao se nao estiver definido
if "%DATABASE_URL%"=="" set DATABASE_URL=sqlite:mta_sheet.db

echo Banco de dados: %DATABASE_URL%

:: Verifica se o cargo-leptos esta instalado
cargo leptos --version >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo [INFO] Usando cargo-leptos para iniciar o projeto...
    cargo leptos watch
) else (
    echo [AVISO] cargo-leptos nao encontrado.
    echo [INFO] Tentando iniciar com cargo run --features ssr...
    echo [INFO] Nota: Hydration pode falhar se os assets nao forem compilados corretamente.

    cargo run --features ssr
)

pause
