@echo off
REM ==============================================================================
REM MTA Sheet — Emulador Mobile Interativo (Janela Visual de Celular)
REM Abre uma janela interativa no monitor com tamanho e toque de iPhone 14.
REM ==============================================================================

set TARGET_URL=%1
if "%TARGET_URL%"=="" (
    set TARGET_URL=http://localhost:3000/?debug=true
)

echo.
echo =====================================================================
echo   MTA Sheet - Janela Interativa Mobile (iPhone 14)
echo =====================================================================
echo.
echo Abrindo janela visual de celular na tela...
echo.

npx playwright open --device "iPhone 14" "%TARGET_URL%"
