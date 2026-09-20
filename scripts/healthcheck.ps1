# ==============================================================================
# MTA Sheet - Healthcheck Pos-Deploy (Windows / PowerShell)
# ==============================================================================

$maxAttempts = 15
$attempt = 1
$healthy = $false

while ($attempt -le $maxAttempts) {
    try {
        $res = Invoke-WebRequest -Uri "http://localhost:8080" -UseBasicParsing -TimeoutSec 3 -ErrorAction Stop
        if ($res.StatusCode -eq 200) {
            $healthy = $true
            break
        }
    } catch {
        # Continua tentando
    }

    $status = docker inspect --format "{{.State.Health.Status}}" mta_sheet_app 2>$null
    if ($status -eq "healthy") {
        $healthy = $true
        break
    }

    Write-Host "   Aguardando inicializacao do servidor... ($attempt/$maxAttempts)" -ForegroundColor DarkGray
    Start-Sleep -Seconds 2
    $attempt++
}

if ($healthy) {
    Write-Host ""
    Write-Host "==============================================================================" -ForegroundColor Green
    Write-Host "[SUCESSO] MTA Sheet atualizado e operacional com Near-Zero Downtime!" -ForegroundColor Green
    Write-Host "==============================================================================" -ForegroundColor Green
    exit 0
} else {
    Write-Host ""
    Write-Host "[ALERTA CRITICO] O novo container nao respondeu dentro do tempo limite!" -ForegroundColor Red
    docker compose logs --tail=30 mta_sheet
    exit 1
}
