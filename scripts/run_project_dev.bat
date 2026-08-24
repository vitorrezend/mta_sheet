@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

:: Navega para a raiz do projeto
cd /d "%~dp0.."

echo =======================================================
echo   MTA Sheet - Modo de Desenvolvimento [Windows]
echo =======================================================
echo.

:: 1. Carrega variaveis do .env se existir
if exist .env (
    for /f "usebackq tokens=* eol=#" %%i in (".env") do (
        set "%%i"
    )
)

:: 2. Variaveis padrao para ambiente de desenvolvimento
if "%DATABASE_URL%"=="" set DATABASE_URL=sqlite:mta_sheet.db
if "%LEPTOS_SITE_ADDR%"=="" set LEPTOS_SITE_ADDR=127.0.0.1:3000
if "%RUST_LOG%"=="" set RUST_LOG=info

echo [CONFIG] Banco de Dados: %DATABASE_URL%
echo [CONFIG] Endereco:       http://%LEPTOS_SITE_ADDR%
echo [CONFIG] Logs:           %RUST_LOG%
echo.

:: 3. Garante existencia de pastas necessarias
if not exist "uploads" mkdir uploads
if not exist "target\site\pkg" mkdir target\site\pkg
if not exist "styles" mkdir styles

:: 4. Verifica se cargo-leptos esta instalado para Hot-Reload
where cargo-leptos >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo [INFO] Iniciando com cargo-leptos watch [Hot-Reloading ativado]...
    cargo leptos watch
) else (
    echo [AVISO] cargo-leptos nao foi encontrado no PATH.
    echo.
    echo Opcoes:
    echo   [1] Iniciar servidor backend diretamente via cargo run --features ssr
    echo   [2] Instalar cargo-leptos agora [necessario para Hot-Reload de WASM+SSR]
    echo.
    set /p DEV_OPT="Escolha uma opcao [1 ou 2, Enter para 1]: "
    if "!DEV_OPT!"=="2" (
        echo.
        echo [INFO] Baixando binario oficial pre-compilado do cargo-leptos...
        powershell -NoProfile -Command "irm https://github.com/leptos-rs/cargo-leptos/releases/download/v0.2.20/cargo-leptos-installer.ps1 | iex"
        if !ERRORLEVEL! EQU 0 (
            echo [SUCESSO] cargo-leptos instalado com sucesso! Iniciando watch...
            cargo leptos watch
        ) else (
            echo [ERRO] Falha ao instalar cargo-leptos. Iniciando via cargo run...
            cargo run --features ssr
        )
    ) else (
        echo.
        echo [INFO] Iniciando via cargo run --features ssr...
        cargo run --features ssr
    )
)

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERRO] O servidor foi encerrado com erro.
)

pause
