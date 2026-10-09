param(
    [string]$TargetUrl = "http://localhost:3000/?debug=true"
)

Write-Host "`n=====================================================================" -ForegroundColor Cyan
Write-Host "  MTA Sheet - Janela Interativa Mobile (iPhone 14)" -ForegroundColor Cyan
Write-Host "=====================================================================" -ForegroundColor Cyan
Write-Host "`nAbrindo janela visual de celular na tela..." -ForegroundColor Green

npx playwright open --device "iPhone 14" $TargetUrl
