@echo off
setlocal

cd /d "%~dp0.."

echo ==============================================================================
echo   MTA Sheet - Esteira Automatizada de Deploy em Producao (Zero Downtime)
echo ==============================================================================
echo.

where docker.exe >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERRO CRITICO] O Docker nao foi encontrado no PATH.
    echo Certifique-se de que o Docker Desktop ou Docker Engine esta ativo.
    exit /b 1
)

set SKIP_TESTS=0
if /i "%1"=="--skip-tests" set SKIP_TESTS=1
if /i "%1"=="-s" set SKIP_TESTS=1

:: ------------------------------------------------------------------------------
:: ETAPA 1: Quality Gate & Testes Automatizados
:: ------------------------------------------------------------------------------
echo [1/5] Quality Gate: Executando suite de testes automatizados...
if %SKIP_TESTS% EQU 1 (
    echo [AVISO] Flag --skip-tests detectada. Testes automatizados ignorados pelo operador.
) else (
    where cargo.exe >nul 2>nul
    if %ERRORLEVEL% EQU 0 (
        cargo test --features ssr -- --quiet
        if errorlevel 1 (
            echo.
            echo [ERRO CRITICO] A suite de testes automatizados FALHOU.
            echo O deploy foi CANCELADO para proteger o ambiente de producao.
            echo O container de producao atual permanece 100%% INTACTO e OPERACIONAL.
            exit /b 1
        )
        echo [OK] Todos os testes unitarios e de integracao passaram com sucesso.
    ) else (
        echo [AVISO] 'cargo' nao encontrado no PATH do host. Pulando testes locais...
    )
)
echo.

:: ------------------------------------------------------------------------------
:: ETAPA 2: Snapshot de Seguranca Pre-Deploy (Backup Automatico)
:: ------------------------------------------------------------------------------
echo [2/5] Snapshot de Seguranca: Realizando backup preventivo dos dados...
call "%~dp0backup\backup-docker.bat" pre_deploy
if errorlevel 1 (
    echo.
    echo [ERRO CRITICO] Falha ao realizar o backup preventivo de seguranca.
    echo O deploy foi CANCELADO por precaucao para evitar perda de dados.
    exit /b 1
)
echo.

:: ------------------------------------------------------------------------------
:: ETAPA 3: Build em Background com Cache (Servico Continua Online)
:: ------------------------------------------------------------------------------
echo [3/5] Build em Segundo Plano: Compilando nova imagem (Servico permanece ONLINE)...
docker.exe compose build mta_sheet
if errorlevel 1 (
    echo.
    echo [ERRO CRITICO] A compilacao da nova imagem falhou.
    echo O container anterior NAO foi modificado e continua atendendo usuarios normalmente.
    exit /b 1
)
echo [OK] Nova imagem construida com sucesso.
echo.

:: ------------------------------------------------------------------------------
:: ETAPA 4: Hot-Swap Atomico de Container (Downtime ~1s)
:: ------------------------------------------------------------------------------
echo [4/5] Hot-Swap: Atualizando container reaproveitando volumes persistentes...
docker.exe compose up -d --no-deps mta_sheet
if errorlevel 1 (
    echo.
    echo [ERRO CRITICO] Falha ao recriar o container com a nova imagem.
    exit /b 1
)
echo [OK] Container recriado com a nova versao.
echo.

:: ------------------------------------------------------------------------------
:: ETAPA 5: Health Check & Validacao Pos-Deploy
:: ------------------------------------------------------------------------------
echo [5/5] Validacao Pos-Deploy: Monitorando saude do novo container...
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0healthcheck.ps1"
if errorlevel 1 (
    echo.
    echo [ERRO] O servico nao passou no healthcheck pos-deploy. Verifique os logs acima.
    exit /b 1
)

echo.
echo Status dos containers ativos:
docker.exe compose ps
echo.
echo Acesse a aplicacao em:
echo   - Local Direto: http://localhost:8080
echo   - Producao/Proxy: http://localhost:80 / https://dominio (se Caddy estiver configurado)
