param(
    [string]$TargetUrl = "http://localhost:3000/?debug=true"
)

Write-Host "`n=====================================================================" -ForegroundColor Cyan
Write-Host "  MTA Sheet - Teste no Safari (WebKit) com Emulação de iPhone" -ForegroundColor Cyan
Write-Host "=====================================================================" -ForegroundColor Cyan
Write-Host "`nURL de destino: $TargetUrl" -ForegroundColor Yellow
Write-Host "`nAbrindo janela visual do Safari (WebKit)..." -ForegroundColor Green
Write-Host "(Feche a janela do navegador para encerrar o teste)`n"

npx playwright codegen -b webkit --viewport-size=390,844 --color-scheme=dark $TargetUrl
