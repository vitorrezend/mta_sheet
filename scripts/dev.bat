@echo off
setlocal enabledelayedexpansion

:: Navega para a raiz do projeto
cd /d "%~dp0.."

echo =======================================================
echo   MTA Sheet - Ambiente de Desenvolvimento [Dev]
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
set RUST_BACKTRACE=1

echo [CONFIG] Banco de Dados: %DATABASE_URL%
echo [CONFIG] Endereco:       http://%LEPTOS_SITE_ADDR%
echo [CONFIG] Logs:           %RUST_LOG%
echo.

:: 2.1 Limpa eventuais processos zumbis que ficaram travados na porta 3000
for /f "tokens=5" %%p in ('netstat -aon 2^>nul ^| findstr ":3000" ^| findstr "LISTENING"') do (
    taskkill /F /PID %%p >nul 2>nul
)

:: 3. Garante existencia de pastas necessarias
if not exist "uploads" mkdir uploads
if not exist "target\site\pkg" mkdir target\site\pkg
if not exist "styles" mkdir styles

:: 4. Se o argumento 'build' foi passado, apenas compila no modo dev
if /i "%1"=="build" (
    echo [INFO] Compilando no perfil DEV sem watcher...
    cargo leptos build
    goto :fim
)

:: 5. Execucao com cargo-leptos watch (Hot-Reload)
where cargo-leptos >nul 2>nul
if %ERRORLEVEL% NEQ 0 goto :no_cargo_leptos

echo [INFO] Iniciando servidor com Hot-Reload [cargo leptos watch]...
cargo leptos watch
goto :fim

:no_cargo_leptos
echo [AVISO] cargo-leptos nao foi encontrado no PATH.
echo.
echo Opcoes:
echo   [1] Iniciar servidor backend diretamente via cargo run
echo   [2] Instalar cargo-leptos agora
echo.
set /p DEV_OPT="Escolha uma opcao [1 ou 2, Enter para 1]: "
if "!DEV_OPT!"=="2" (
    echo [INFO] Baixando instalador do cargo-leptos...
    powershell -NoProfile -Command "irm https://github.com/leptos-rs/cargo-leptos/releases/download/v0.2.20/cargo-leptos-installer.ps1 | iex"
    cargo leptos watch
) else (
    cargo run --features ssr
)

:fim
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERRO] A execucao foi encerrada com erro.
)
