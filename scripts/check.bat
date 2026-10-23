@echo off
:: ==============================================================================
:: MTA Sheet - Checagem completa antes de um release (substitui um CI)
:: Executa scripts\check.sh pelo Git Bash (instalado junto com o Git for Windows).
::   scripts\check.bat          -> tudo (clippy, testes Rust, E2E no WebKit)
::   scripts\check.bat --fast   -> so clippy + testes Rust
:: ==============================================================================
setlocal
cd /d "%~dp0.."

set "BASH_EXE="
if exist "%ProgramFiles%\Git\bin\bash.exe" set "BASH_EXE=%ProgramFiles%\Git\bin\bash.exe"
if "%BASH_EXE%"=="" for /f "delims=" %%B in ('where bash 2^>nul') do if "%BASH_EXE%"=="" set "BASH_EXE=%%B"
if "%BASH_EXE%"=="" (
    echo [ERRO] Git Bash nao encontrado. Instale o Git for Windows ou rode scripts/check.sh em um terminal Bash.
    exit /b 1
)

"%BASH_EXE%" scripts/check.sh %*
exit /b %ERRORLEVEL%
