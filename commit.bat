@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

:: Navega para a raiz do projeto onde o .git reside
cd /d "%~dp0"
if not exist ".git" (
    if exist "..\.git" (
        cd /d "%~dp0.."
    )
)

if not exist ".git" (
    echo [ERRO] Nao foi possivel encontrar o repositorio Git neste diretorio.
    pause
    exit /b 1
)

echo =======================================================
echo          MTA SHEET - SCRIPT DE COMMIT (WINDOWS)
echo =======================================================
echo.

:: 1. Exibir status atual
echo Status dos arquivos no repositorio:
echo -------------------------------------------------------
git status -s
echo -------------------------------------------------------
echo.

:: 2. Verificar se ha conflitos nao resolvidos
set HAS_CONFLICTS=0
for /f "tokens=1,2" %%A in ('git status -s 2^>nul') do (
    if "%%A"=="UU" set HAS_CONFLICTS=1
    if "%%A"=="AA" set HAS_CONFLICTS=1
    if "%%A"=="UD" set HAS_CONFLICTS=1
    if "%%A"=="DU" set HAS_CONFLICTS=1
    if "%%A"=="DD" set HAS_CONFLICTS=1
    if "%%A"=="AU" set HAS_CONFLICTS=1
    if "%%A"=="UA" set HAS_CONFLICTS=1
)

if "!HAS_CONFLICTS!"=="1" (
    echo [AVISO CRITICO] Existem arquivos com conflitos de merge pendentes!
    echo.
    echo Arquivos com conflito detectados:
    git status -s | findstr /R "^UU ^AA ^UD ^DU ^DD ^AU ^UA"
    echo.
    echo Recomendamos resolver os conflitos antes de prosseguir com o commit.
    echo.
    set /p PROCEED_CONFLICT="Deseja continuar mesmo assim? [s/N]: "
    if /i not "!PROCEED_CONFLICT!"=="s" (
        echo Operacao cancelada pelo usuario.
        pause
        exit /b 1
    )
)

:: 3. Verificar se ha qualquer alteracao no repositorio
set HAS_CHANGES=0
for /f "tokens=*" %%i in ('git status --porcelain 2^>nul') do (
    set HAS_CHANGES=1
)

if "!HAS_CHANGES!"=="0" (
    echo [INFO] Nenhuma alteracao pendente detectada no repositorio.
    echo Tudo atualizado!
    pause
    exit /b 0
)

:: 4. Adicionar arquivos ao stage
echo Opcoes de Stage:
echo [1] Adicionar tudo: git add -A (Padrao)
echo [2] Manter apenas o que ja estiver no stage
set /p STAGE_OPT="Escolha uma opcao [1 ou 2, Enter para 1]: "
if not "!STAGE_OPT!"=="2" (
    echo.
    echo [INFO] Adicionando alteracoes ao stage...
    git add -A
)

:: 5. Obter mensagem de commit
set COMMIT_MSG=%~1
if "!COMMIT_MSG!"=="" (
    echo.
    set /p COMMIT_MSG="Digite a mensagem do commit: "
)

if "!COMMIT_MSG!"=="" (
    echo [ERRO] A mensagem de commit nao pode ser vazia. Operacao cancelada.
    pause
    exit /b 1
)

:: 6. Executar o commit
echo.
echo [INFO] Realizando commit: "!COMMIT_MSG!"...
git commit -m "!COMMIT_MSG!"
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERRO] Falha ao realizar commit. Verifique as mensagens do Git acima.
    pause
    exit /b %ERRORLEVEL%
)
echo [SUCESSO] Commit realizado com sucesso!
echo.

:: 7. Obter branch atual e perguntar sobre Push
for /f "tokens=*" %%i in ('git branch --show-current 2^>nul') do set CURRENT_BRANCH=%%i
if "!CURRENT_BRANCH!"=="" set CURRENT_BRANCH=HEAD

echo Branch atual: !CURRENT_BRANCH!
set /p PUSH_OPT="Deseja enviar [push] para o repositorio remoto agora? [S/n]: "
if /i "!PUSH_OPT!"=="" set PUSH_OPT=s
if /i "!PUSH_OPT!"=="s" (
    echo.
    echo [INFO] Enviando alteracoes para origin/!CURRENT_BRANCH!...
    git push origin !CURRENT_BRANCH! 2>nul
    if !ERRORLEVEL! NEQ 0 (
        echo [AVISO] Tentando push definindo upstream: -u origin !CURRENT_BRANCH!...
        git push -u origin !CURRENT_BRANCH!
    )
    if !ERRORLEVEL! EQU 0 (
        echo [SUCESSO] Push concluido com sucesso!
    ) else (
        echo [ERRO] Falha no push. Verifique se ha divergencias ou permissoes.
    )
)

echo.
echo =======================================================
echo Operacao finalizada.
echo =======================================================
pause
