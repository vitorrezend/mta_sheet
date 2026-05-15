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
    echo [INFO] Para uma melhor experiencia, instale com: cargo install cargo-leptos
    echo [INFO] Tentando iniciar manualmente...

    :: Certifica que o diretório de destino existe para o frontend (simulando cargo-leptos)
    if not exist "target\site" mkdir "target\site"

    :: Se o trunk estiver disponivel, builda o frontend
    trunk --version >nul 2>nul
    if %ERRORLEVEL% EQU 0 (
        echo [1/2] Compilando assets do frontend com Trunk...
        trunk build --release -d target/site
    ) else (
        echo [AVISO] Trunk nao encontrado. Assets estaticos podem nao funcionar.
    )

    echo [2/2] Iniciando servidor backend...
    cargo run --features ssr
)

pause
