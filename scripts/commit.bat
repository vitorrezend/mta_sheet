@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

:: Navega para a raiz do projeto a partir de scripts/
cd /d "%~dp0.."

if not exist ".git" (
    echo [ERRO] Nao foi possivel encontrar o repositorio Git na raiz do projeto.
    pause
    exit /b 1
)

call "%~dp0..\commit.bat" %*
