@echo off
REM ==============================================================================
REM MTA Sheet — Safari (WebKit) Engine Audit
REM Renderiza o site no motor WebKit (Safari) e abre o print resultante.
REM ==============================================================================

setlocal enabledelayedexpansion

set TARGET_URL=%1
if "%TARGET_URL%"=="" (
    set TARGET_URL=http://localhost:3000/?debug=true
)

echo.
echo =====================================================================
echo   MTA Sheet - Auditoria no Motor WebKit (Safari do iPhone)
echo =====================================================================
echo.
echo URL: %TARGET_URL%
echo.
echo [1/2] Renderizando no motor WebKit com resolucao de iPhone 14...

npx playwright screenshot -b webkit --device="iPhone 14" "%TARGET_URL%" safari_resultado.png

if %ERRORLEVEL% equ 0 (
    echo [2/2] Sucesso! Abrindo a imagem gerada pelo Safari no seu visualizador...
    start safari_resultado.png
    echo.
    echo =====================================================================
    echo A imagem 'safari_resultado.png' foi aberta na sua tela!
    echo Ela mostra exatamente como o Safari do iPhone renderiza a aplicacao.
    echo =====================================================================
) else (
    echo.
    echo [ERRO] Falha ao executar o WebKit. Verifique se o servidor esta rodando.
)
