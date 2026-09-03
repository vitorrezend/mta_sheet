@echo off
setlocal enabledelayedexpansion

cd /d "%~dp0.."

echo ========================================================================
echo   MTA Sheet - Build Release Standalone [Windows / .exe]
echo ========================================================================
echo.

if exist .env (
    for /f "usebackq tokens=* eol=#" %%i in (".env") do (
        set "%%i"
    )
)

:: Encerra eventuais instancias em execucao que possam travar os arquivos .exe
taskkill /F /IM mta_sheet_server.exe >nul 2>nul
taskkill /F /IM mta_sheet.exe >nul 2>nul

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

set "INSTALL_WASM_BINDGEN=0"
where wasm-bindgen >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    set "INSTALL_WASM_BINDGEN=1"
) else (
    wasm-bindgen --version 2>&1 | findstr /C:"0.2.121" >nul
    if !ERRORLEVEL! NEQ 0 set "INSTALL_WASM_BINDGEN=1"
)

if "!INSTALL_WASM_BINDGEN!"=="1" (
    echo   -^> Sincronizando wasm-bindgen-cli para versao 0.2.121...
    cargo install wasm-bindgen-cli --version 0.2.121 --locked --force
)

echo [3/4] Compilando Frontend WASM e Backend com Assets Embutidos...
echo   -^> [1/3] Compilando Frontend WASM Release...
cargo build --lib --target wasm32-unknown-unknown --release --no-default-features --features hydrate
if %ERRORLEVEL% NEQ 0 (
    echo [ERRO CRITICO] A compilacao WASM falhou.
    pause
    exit /b %ERRORLEVEL%
)

echo   -^> [2/3] Gerando bindings JS e empacotando assets em target\site\pkg...
if exist "target\site\pkg" del /Q "target\site\pkg\*.*" >nul 2>nul
wasm-bindgen --target web --out-dir target\site\pkg --out-name mta_sheet target\wasm32-unknown-unknown\release\mta_sheet.wasm --no-typescript
if %ERRORLEVEL% NEQ 0 (
    echo [ERRO CRITICO] O wasm-bindgen falhou.
    pause
    exit /b %ERRORLEVEL%
)

where wasm-opt >nul 2>nul
if !ERRORLEVEL! EQU 0 (
    echo   -^> [wasm-opt] Otimizando bytecode WebAssembly com -Oz...
    if exist "target\site\pkg\mta_sheet_bg.wasm" wasm-opt -Oz "target\site\pkg\mta_sheet_bg.wasm" -o "target\site\pkg\mta_sheet_bg.wasm"
    if exist "target\site\pkg\mta_sheet.wasm" wasm-opt -Oz "target\site\pkg\mta_sheet.wasm" -o "target\site\pkg\mta_sheet.wasm"
)

if exist "target\site\pkg\mta_sheet_bg.wasm" copy /Y "target\site\pkg\mta_sheet_bg.wasm" "target\site\pkg\mta_sheet.wasm" >nul
if exist "target\site\pkg\mta_sheet.wasm" copy /Y "target\site\pkg\mta_sheet.wasm" "target\site\pkg\mta_sheet_bg.wasm" >nul
copy /Y style.css target\site\pkg\mta_sheet.css >nul

echo   -^> [3/3] Compilando Servidor Backend SSR Release...
cargo build --release --no-default-features --features ssr
if %ERRORLEVEL% NEQ 0 (
    echo [ERRO CRITICO] A compilacao do servidor release falhou.
    pause
    exit /b %ERRORLEVEL%
)

echo [4/4] Empacotando executavel standalone...
set "SERVER_BIN="
if exist "target\release\mta_sheet_server.exe" set "SERVER_BIN=target\release\mta_sheet_server.exe"
if "!SERVER_BIN!"=="" if exist "target\release\mta_sheet.exe" set "SERVER_BIN=target\release\mta_sheet.exe"

if "!SERVER_BIN!"=="" (
    echo [ERRO] O binario mta_sheet.exe nao foi encontrado em target\release.
    pause
    exit /b 1
)

copy /Y "!SERVER_BIN!" ".\mta_sheet.exe" >nul

echo.
echo ========================================================================
echo   Executavel Standalone Gerado com Sucesso!
echo ========================================================================
echo   Arquivo : %CD%\mta_sheet.exe
if exist ".\mta_sheet.exe" (
    for %%F in (".\mta_sheet.exe") do echo   Tamanho : %%~zF bytes
)
echo ========================================================================
echo.

if /i "%1"=="run" (
    echo [INFO] Iniciando mta_sheet.exe...
    start http://localhost:3000
    .\mta_sheet.exe
    goto :eof
)

set /p RUN_NOW="Deseja iniciar o executavel release agora? [s/N]: "
if /i "!RUN_NOW!"=="s" (
    echo [INFO] Iniciando mta_sheet.exe...
    start http://localhost:3000
    .\mta_sheet.exe
)
