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
if not exist "target\site\fonts" mkdir target\site\fonts
if not exist "styles" mkdir styles
if exist "fonts" copy /Y "fonts\*.*" "target\site\fonts\" >nul

echo [2/4] Verificando ferramentas necessarias (cargo-leptos, wasm target)...
rustup target list | findstr /C:"wasm32-unknown-unknown (installed)" >nul
if %ERRORLEVEL% NEQ 0 (
    echo   -^> Instalando target wasm32-unknown-unknown...
    rustup target add wasm32-unknown-unknown
)

where cargo-leptos >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo   -^> cargo-leptos nao encontrado. Instalando cargo-leptos v0.3.7...
    powershell -NoProfile -Command "irm https://github.com/leptos-rs/cargo-leptos/releases/download/v0.3.7/cargo-leptos-installer.ps1 | iex"
) else (
    cargo-leptos --version 2>&1 | findstr "0.2." >nul
    if !ERRORLEVEL! EQU 0 (
        echo   -^> cargo-leptos desatualizado [0.2.x]. Atualizando para 0.3.7...
        powershell -NoProfile -Command "irm https://github.com/leptos-rs/cargo-leptos/releases/download/v0.3.7/cargo-leptos-installer.ps1 | iex"
    )
)

echo [3/4] Compilando Frontend WASM e Servidor Backend via cargo-leptos [Release]...
cargo leptos build --release
if %ERRORLEVEL% NEQ 0 (
    echo [ERRO CRITICO] A compilacao via cargo-leptos falhou.
    pause
    exit /b %ERRORLEVEL%
)

:: Garante sincronizacao mta_sheet.wasm <-> mta_sheet_bg.wasm
if exist "target\site\pkg\mta_sheet.wasm" (
    if not exist "target\site\pkg\mta_sheet_bg.wasm" copy /Y "target\site\pkg\mta_sheet.wasm" "target\site\pkg\mta_sheet_bg.wasm" >nul
) else if exist "target\site\pkg\mta_sheet_bg.wasm" (
    copy /Y "target\site\pkg\mta_sheet_bg.wasm" "target\site\pkg\mta_sheet.wasm" >nul
)

echo   -^> [CSS Bundle] Empacotando suite de estilos em target\site\pkg\mta_sheet.css...
(
    type styles\00-tokens.css
    echo.
    type styles\01-variables.css
    echo.
    type styles\02-common.css
    echo.
    type styles\03-sheet-layout.css
    echo.
    type styles\04-page1-main.css
    echo.
    type styles\05-page2-magic-combat.css
    echo.
    type styles\06-page3-expanded.css
    echo.
    type styles\07-page4-history-visuals.css
    echo.
    type styles\08-print-pdf.css
    echo.
    type styles\09-gods-and-monsters.css
    echo.
    type styles\10-page5-grimoire.css
    echo.
    type styles\11-page6-notes.css
    echo.
    type styles\12-compendium.css
    echo.
    type styles\patch_notes.css
    echo.
    type styles\home.css
    echo.
    type styles\auth.css
    echo.
    type styles\rooms.css
    echo.
    type styles\logs.css
    echo.
    type styles\feed.css
    echo.
    type styles\profile.css
    echo.
    type styles\share.css
    echo.
    type styles\about.css
    echo.
    type styles\compendium-navigation.css
) > target\site\pkg\mta_sheet.css

echo   -^> [Standalone Embed] Vinculando assets estaticos diretamente no binario standalone...
cargo build --bin mta_sheet_server --release --features ssr

echo [4/4] Empacotando executavel standalone...
set "SERVER_BIN="
if exist "target\server\release\mta_sheet_server.exe" set "SERVER_BIN=target\server\release\mta_sheet_server.exe"
if "!SERVER_BIN!"=="" if exist "target\release\mta_sheet_server.exe" set "SERVER_BIN=target\release\mta_sheet_server.exe"
if "!SERVER_BIN!"=="" if exist "target\release\mta_sheet.exe" set "SERVER_BIN=target\release\mta_sheet.exe"

if "!SERVER_BIN!"=="" (
    echo [ERRO] O binario mta_sheet_server.exe nao foi encontrado em target\server\release ou target\release.
    pause
    exit /b 1
)

copy /Y "!SERVER_BIN!" ".\mta_sheet.exe" >nul

:: Sincroniza assets para dentro de target\release\site caso o usuario execute mta_sheet_server.exe diretamente de la
if not exist "target\release\site\pkg" mkdir "target\release\site\pkg" 2>nul
copy /Y "target\site\pkg\*.*" "target\release\site\pkg\" >nul 2>nul
if exist "target\site\fonts" (
    if not exist "target\release\site\fonts" mkdir "target\release\site\fonts" 2>nul
    copy /Y "target\site\fonts\*.*" "target\release\site\fonts\" >nul 2>nul
)

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

if /i "%1"=="no-run" goto :eof
if /i "%1"=="build" goto :eof

set /p RUN_NOW="Deseja iniciar o executavel release agora? [s/N]: "
if /i "!RUN_NOW!"=="s" (
    echo [INFO] Iniciando mta_sheet.exe...
    start http://localhost:3000
    .\mta_sheet.exe
)
