@echo off
setlocal enabledelayedexpansion

cd /d "%~dp0..\.."

echo ========================================================
echo   MTA Sheet - Backup Docker (SQLite + Uploads)
echo ========================================================
echo.

set PREFIX=%~1
if "%PREFIX%"=="" set PREFIX=backup

where docker >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERRO] O Docker nao foi encontrado no PATH.
    exit /b 1
)

powershell -NoProfile -ExecutionPolicy Bypass -Command ^
  "$running = docker ps --filter 'name=^mta_sheet_app$' --filter 'status=running' -q;" ^
  "if (-not $running) {" ^
  "  Write-Host '[AVISO] O container mta_sheet_app nao esta em execucao. Backup nao realizado.' -ForegroundColor Yellow;" ^
  "  exit 2;" ^
  "}" ^
  "$prefix = '%PREFIX%';" ^
  "$dest = 'scripts/backup/backups/' + (Get-Date -Format 'yyyy/MM/dd') + '/' + $prefix + '_' + (Get-Date -Format 'yyyyMMdd_HHmmss');" ^
  "Write-Host '[1/3] Sincronizando transacoes SQLite (WAL checkpoint)...' -ForegroundColor Cyan;" ^
  "try { docker exec mta_sheet_app sqlite3 /app/data/mta_sheet.db 'PRAGMA wal_checkpoint(TRUNCATE);' 2>$null | Out-Null } catch {} ;" ^
  "New-Item -ItemType Directory -Path $dest -Force | Out-Null;" ^
  "Write-Host '[2/3] Copiando banco de dados SQLite (/app/data)...' -ForegroundColor Cyan;" ^
  "docker cp mta_sheet_app:/app/data \"$dest/data\";" ^
  "Write-Host '[3/3] Copiando pasta de uploads (/app/uploads)...' -ForegroundColor Cyan;" ^
  "docker cp mta_sheet_app:/app/uploads \"$dest/uploads\" 2>$null;" ^
  "if (Test-Path \"$dest/data/mta_sheet.db\") {" ^
  "  Write-Host '' ;" ^
  "  Write-Host '========================================================' -ForegroundColor Green;" ^
  "  Write-Host \"[SUCESSO] Backup salvo com sucesso em: $dest\" -ForegroundColor Green;" ^
  "  Write-Host '========================================================' -ForegroundColor Green;" ^
  "  Get-ChildItem \"$dest/data\" | Format-Table Name, Length, LastWriteTime;" ^
  "  exit 0;" ^
  "} else {" ^
  "  Write-Host '[ERRO] Falha ao validar copia do banco SQLite.' -ForegroundColor Red;" ^
  "  exit 1;" ^
  "}"

exit /b %ERRORLEVEL%
