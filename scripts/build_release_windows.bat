@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

:: Navega para a raiz do projeto
cd /d "%~dp0.."

echo ========================================================================
echo   MTA Sheet - Build Release Otimizado para Windows [.exe Standalone]
echo ========================================================================
echo.

:: 1. Carrega variaveis do .env se existir
if exist .env (
    for /f "usebackq tokens=* eol=#" %%i in (".env") do (
        set "%%i"
    )
)

:: 2. Prepara diretorios essenciais
echo [1/4] Preparando diretorios e estrutura de assets...
if not exist "uploads" mkdir uploads
if not exist "target\site\pkg" mkdir target\site\pkg
if not exist "styles" mkdir styles

:: 3. Verifica e adiciona target wasm32-unknown-unknown caso necessario
echo [2/4] Verificando target wasm32-unknown-unknown...
rustup target list | findstr /C:"wasm32-unknown-unknown (installed)" >nul
if %ERRORLEVEL% NEQ 0 (
    echo   -^> Instalando target wasm32-unknown-unknown...
    rustup target add wasm32-unknown-unknown
)

:: 4. Compilacao do Frontend e Backend
echo [3/4] Compilando Frontend WASM e Backend com Assets Embutidos...
echo   -^> Otimizacoes ativas: LTO=true, Opt-Level='z', Codegen-Units=1, Strip=true

where cargo-leptos >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo   -^> Usando cargo-leptos para pipeline de Release...
    cargo leptos build --release
) else (
    echo   -^> Compilando Frontend WASM...
    cargo build --target wasm32-unknown-unknown --release --no-default-features --features hydrate
    echo   -^> Compilando Servidor Backend Release SSR com Assets Embutidos...
    cargo build --release --no-default-features --features ssr
)

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERRO CRITICO] A compilacao falhou. Verifique os erros acima.
    pause
    exit /b %ERRORLEVEL%
)

:: 5. Localiza e copia o binario standalone final
echo [4/4] Empacotando executavel unico...

set "SERVER_BIN="
if exist "target\server\release\mta_sheet_server.exe" (
    set "SERVER_BIN=target\server\release\mta_sheet_server.exe"
) else if exist "target\server\release\mta_sheet.exe" (
    set "SERVER_BIN=target\server\release\mta_sheet.exe"
) else if exist "target\release\mta_sheet_server.exe" (
    set "SERVER_BIN=target\release\mta_sheet_server.exe"
) else if exist "target\release\mta_sheet.exe" (
    set "SERVER_BIN=target\release\mta_sheet.exe"
)

if "!SERVER_BIN!"=="" (
    echo [ERRO] O binario mta_sheet.exe nao foi encontrado em target.
    pause
    exit /b 1
)

copy /Y "!SERVER_BIN!" ".\mta_sheet.exe" >nul
if %ERRORLEVEL% NEQ 0 (
    echo [AVISO] Nao foi possivel sobrescrever .\mta_sheet.exe [o arquivo pode estar em execucao].
)

echo.
echo ========================================================================
echo   Executavel Standalone para Windows Gerado com Sucesso!
echo ========================================================================
echo   Localizacao : %CD%\mta_sheet.exe
if exist ".\mta_sheet.exe" (
    for %%F in (".\mta_sheet.exe") do echo   Tamanho     : %%~zF bytes
)
echo ========================================================================
echo.
echo Caracteristicas do binario gerado:
echo   [+] 100%% Autocontido: Frontend WASM, JS, CSS e estilos embutidos no .exe.
echo   [+] Zero Dependencias Externas: Nao requer Node.js, nem Python, nem DLLs extras.
echo   [+] Banco SQLite Local: Cria e gerencia automaticamente mta_sheet.db.
echo.
echo Como executar:
echo   1. Dê duplo clique em 'mta_sheet.exe' ou use 'scripts\run_project_release.bat'.
echo   2. Acesse: http://localhost:3000
echo.

set /p RUN_NOW="Deseja iniciar o executavel release agora? [s/N]: "
if /i "!RUN_NOW!"=="s" (
    echo [INFO] Iniciando mta_sheet.exe...
    start http://localhost:3000
    .\mta_sheet.exe
)

pause
