@echo off
setlocal enabledelayedexpansion

:: Navega para a raiz do projeto
cd /d "%~dp0.."

echo ========================================================================
echo   MTA Sheet - Build Release Otimizado para Windows [.exe Standalone]
echo ========================================================================
echo.

if exist .env (
    for /f "usebackq tokens=* eol=#" %%i in (".env") do (
        set "%%i"
    )
)

echo [1/4] Preparando diretorios e estrutura de assets...
if not exist "uploads" mkdir uploads
if not exist "target\site\pkg" mkdir target\site\pkg
if not exist "styles" mkdir styles

echo [2/4] Verificando target wasm32-unknown-unknown e wasm-bindgen...
rustup target list | findstr /C:"wasm32-unknown-unknown (installed)" >nul
if %ERRORLEVEL% NEQ 0 (
    echo   -^> Instalando target wasm32-unknown-unknown...
    rustup target add wasm32-unknown-unknown
)

set NEED_INSTALL_BINDGEN=0
where wasm-bindgen >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    set NEED_INSTALL_BINDGEN=1
) else (
    wasm-bindgen --version 2>nul | findstr /C:"0.2.121" >nul
    if !ERRORLEVEL! NEQ 0 (
        echo   -^> wasm-bindgen desatualizado detectado. Atualizando para versao 0.2.121...
        set NEED_INSTALL_BINDGEN=1
    )
)

if "!NEED_INSTALL_BINDGEN!"=="1" (
    echo   -^> Instalando wasm-bindgen-cli versao 0.2.121...
    cargo install wasm-bindgen-cli --version 0.2.121 --locked
)

echo [3/4] Compilando Frontend WASM e Backend com Assets Embutidos...
echo   -^> Otimizacoes ativas: LTO=true, Opt-Level='z', Codegen-Units=1, Strip=true

echo   -^> [1/3] Compilando Frontend WASM Release...
cargo build --target wasm32-unknown-unknown --release --no-default-features --features hydrate
if %ERRORLEVEL% NEQ 0 (
    echo [ERRO CRITICO] A compilacao WASM falhou.
    pause
    exit /b %ERRORLEVEL%
)

echo   -^> [2/3] Gerando bindings JS e empacotando assets em pkg/...
wasm-bindgen --target web --out-dir target\site\pkg --out-name mta_sheet target\wasm32-unknown-unknown\release\mta_sheet.wasm --no-typescript
if %ERRORLEVEL% NEQ 0 (
    echo [ERRO CRITICO] O wasm-bindgen falhou ao processar os bindings.
    pause
    exit /b %ERRORLEVEL%
)

copy /Y style.css target\site\pkg\mta_sheet.css >nul

echo   -^> [3/3] Compilando Servidor Backend SSR com Assets Embutidos...
cargo build --release --no-default-features --features ssr
if %ERRORLEVEL% NEQ 0 (
    echo [ERRO CRITICO] A compilacao do servidor release falhou.
    pause
    exit /b %ERRORLEVEL%
)

echo [4/4] Empacotando executavel unico...

set "SERVER_BIN="
if exist "target\release\mta_sheet_server.exe" (
    set "SERVER_BIN=target\release\mta_sheet_server.exe"
) else if exist "target\release\mta_sheet.exe" (
    set "SERVER_BIN=target\release\mta_sheet.exe"
) else if exist "target\server\release\mta_sheet_server.exe" (
    set "SERVER_BIN=target\server\release\mta_sheet_server.exe"
) else if exist "target\server\release\mta_sheet.exe" (
    set "SERVER_BIN=target\server\release\mta_sheet.exe"
)

if "!SERVER_BIN!"=="" (
    echo [ERRO] O binario mta_sheet_server.exe nao foi encontrado em target.
    pause
    exit /b 1
)

copy /Y "!SERVER_BIN!" ".\mta_sheet.exe" >nul
if %ERRORLEVEL% NEQ 0 (
    echo [AVISO] Nao foi possivel sobrescrever .\mta_sheet.exe ^(o arquivo pode estar em execucao^).
)

echo.
echo ========================================================================
echo   Executavel Standalone para Windows Gerado com Sucesso!
echo ========================================================================
echo   Localizacao : %CD%\mta_sheet.exe
if exist ".\mta_sheet.exe" (
    for %%F in (".\mta_sheet.exe") do echo   Tamanho     : %%~zF bytes ^(otimizado com LTO e strip^)
)
echo ========================================================================
echo.
echo Caracteristicas do binario gerado:
echo   [+] 100%% Autocontido: Frontend WASM, JS, CSS e estilos embutidos no .exe.
echo   [+] Zero Dependencias Externas: Nao requer Node.js, nem Python, nem DLLs extras.
echo   [+] Banco SQLite Local: Cria e gerencia automaticamente mta_sheet.db.
echo.
echo Como executar:
echo   1. De duplo clique em 'mta_sheet.exe' ou use 'scripts\run_release.bat'.
echo   2. Acesse: http://localhost:3000
echo.

set /p RUN_NOW="Deseja iniciar o executavel release agora? [s/N]: "
if /i "!RUN_NOW!"=="s" (
    echo [INFO] Iniciando mta_sheet.exe...
    start http://localhost:3000
    .\mta_sheet.exe
)

