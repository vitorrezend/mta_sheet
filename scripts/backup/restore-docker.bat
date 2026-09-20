@echo off
setlocal enabledelayedexpansion

cd /d "%~dp0..\.."

echo ========================================================
echo   MTA Sheet - Restauracao de Backup Docker
echo ========================================================
echo.

where docker >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERRO] O Docker nao foi encontrado no PATH.
    exit /b 1
)

set BACKUP_PATH=%~1
set AUTO_CONFIRM=0
if /i "%2"=="--yes" set AUTO_CONFIRM=1
if /i "%2"=="-y" set AUTO_CONFIRM=1
if /i "%1"=="--yes" (
    set BACKUP_PATH=
    set AUTO_CONFIRM=1
)
if /i "%1"=="-y" (
    set BACKUP_PATH=
    set AUTO_CONFIRM=1
)

powershell -NoProfile -ExecutionPolicy Bypass -Command ^
  "$bp = '%BACKUP_PATH%';" ^
  "if (-not $bp) {" ^
  "  $latest = Get-ChildItem -Path 'scripts/backup/backups' -Recurse -Directory | Where-Object { Test-Path (Join-Path $_.FullName 'data/mta_sheet.db') } | Sort-Object LastWriteTime -Descending | Select-Object -First 1;" ^
  "  if (-not $latest) {" ^
  "    Write-Host '[ERRO] Nenhum backup valido encontrado em scripts/backup/backups/' -ForegroundColor Red;" ^
  "    exit 1;" ^
  "  }" ^
  "  $bp = $latest.FullName;" ^
  "}" ^
  "if (-not (Test-Path \"$bp/data/mta_sheet.db\")) {" ^
  "  Write-Host \"[ERRO] O caminho informado nao contem um banco mta_sheet.db valido: $bp\" -ForegroundColor Red;" ^
  "  exit 1;" ^
  "}" ^
  "Write-Host 'Backup selecionado para restauracao:' -ForegroundColor Cyan;" ^
  "Write-Host \"   $bp\" -ForegroundColor Yellow;" ^
  "Get-ChildItem \"$bp/data\" | Format-Table Name, Length, LastWriteTime;" ^
  "if (%AUTO_CONFIRM% -ne 1) {" ^
  "  $reply = Read-Host 'ATENCAO: Os dados atuais no container serao substituidos por este backup. Continuar? (s/N)';" ^
  "  if ($reply -notmatch '^[sSyY]$') {" ^
  "    Write-Host 'Restauracao cancelada pelo usuario.' -ForegroundColor Yellow;" ^
  "    exit 2;" ^
  "  }" ^
  "}" ^
  "Write-Host '[1/4] Parando temporariamente container para consistencia...' -ForegroundColor Cyan;" ^
  "docker compose stop mta_sheet | Out-Null;" ^
  "Write-Host '[2/4] Restaurando banco de dados SQLite...' -ForegroundColor Cyan;" ^
  "docker cp \"$bp/data\" mta_sheet_app:/app/;" ^
  "if (Test-Path \"$bp/uploads\") {" ^
  "  Write-Host '[3/4] Restaurando arquivos de uploads...' -ForegroundColor Cyan;" ^
  "  docker cp \"$bp/uploads\" mta_sheet_app:/app/;" ^
  "}" ^
  "Write-Host '[4/4] Reiniciando aplicacao...' -ForegroundColor Cyan;" ^
  "docker compose start mta_sheet | Out-Null;" ^
  "Write-Host '' ;" ^
  "Write-Host '========================================================' -ForegroundColor Green;" ^
  "Write-Host '[SUCESSO] Backup restaurado e aplicacao reiniciada!' -ForegroundColor Green;" ^
  "Write-Host '========================================================' -ForegroundColor Green;" ^
  "exit 0;"

exit /b %ERRORLEVEL%
