<#
.SYNOPSIS
    Script de automação de commit e push para Windows (PowerShell).
.DESCRIPTION
    Verifica conflitos, exibe status, adiciona arquivos, realiza commit e opcionalmente faz o push para o remote.
.EXAMPLE
    .\scripts\commit.ps1
.EXAMPLE
    .\scripts\commit.ps1 "feat: atualizacoes no layout"
#>

[CmdletBinding()]
param (
    [Parameter(Position = 0, Mandatory = $false)]
    [string]$CommitMessage
)

$Host.UI.RawUI.WindowTitle = "MTA Sheet - Git Commit Helper"
$repoRoot = (Get-Item $PSScriptRoot).Parent.FullName

if (-not (Test-Path "$repoRoot\.git")) {
    $repoRoot = (Get-Location).Path
    if (-not (Test-Path "$repoRoot\.git")) {
        Write-Host "[ERRO] Repositorio Git nao encontrado." -ForegroundColor Red
        return
    }
}

Set-Location $repoRoot

Write-Host "=======================================================" -ForegroundColor Cyan
Write-Host "         MTA SHEET - GIT COMMIT (POWERSHELL)           " -ForegroundColor Cyan
Write-Host "=======================================================" -ForegroundColor Cyan
Write-Host ""

# 1. Verificar conflitos nao resolvidos
$conflicts = git diff --name-only --diff-filter=U 2>$null
if ($conflicts) {
    Write-Host "[AVISO CRITICO] Existem arquivos com conflitos de merge nao resolvidos:" -ForegroundColor Yellow
    foreach ($file in $conflicts) {
        Write-Host "  - $file" -ForegroundColor Red
    }
    Write-Host ""
    $proceed = Read-Host "Deseja prosseguir mesmo com conflitos? (s/N)"
    if ($proceed -ne "s" -and $proceed -ne "S") {
        Write-Host "Operacao cancelada." -ForegroundColor Yellow
        return
    }
}

# 2. Exibir status atual
Write-Host "Status dos arquivos alterados:" -ForegroundColor Cyan
Write-Host "-------------------------------------------------------" -ForegroundColor DarkGray
git status -s
Write-Host "-------------------------------------------------------" -ForegroundColor DarkGray
Write-Host ""

# 3. Verificar se ha alteracoes
$statusPorcelain = git status --porcelain 2>$null
if (-not $statusPorcelain) {
    Write-Host "[INFO] Nenhuma alteracao pendente detectada no repositorio." -ForegroundColor Green
    return
}

# 4. Perguntar stage
$stageOpt = Read-Host "Deseja adicionar todas as alteracoes (git add -A)? (S/n)"
if ($stageOpt -eq "" -or $stageOpt -eq "s" -or $stageOpt -eq "S") {
    Write-Host "[INFO] Executando 'git add -A'..." -ForegroundColor Cyan
    git add -A
}

# 5. Obter mensagem de commit
if ([string]::IsNullOrWhiteSpace($CommitMessage)) {
    $CommitMessage = Read-Host "Digite a mensagem do commit"
}

if ([string]::IsNullOrWhiteSpace($CommitMessage)) {
    Write-Host "[ERRO] A mensagem de commit nao pode ser vazia." -ForegroundColor Red
    return
}

# 6. Commit
Write-Host ""
Write-Host "[INFO] Realizando commit..." -ForegroundColor Cyan
git commit -m "$CommitMessage"

if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERRO] Falha ao realizar commit." -ForegroundColor Red
    return
}

Write-Host "[SUCESSO] Commit realizado com sucesso!" -ForegroundColor Green
Write-Host ""

# 7. Push
$currentBranch = (git branch --show-current 2>$null)
if (-not $currentBranch) {
    $currentBranch = "HEAD"
}

Write-Host "Branch atual: $currentBranch" -ForegroundColor Magenta
$pushOpt = Read-Host "Deseja enviar (push) para o repositorio remoto agora? (S/n)"
if ($pushOpt -eq "" -or $pushOpt -eq "s" -or $pushOpt -eq "S") {
    Write-Host "[INFO] Enviando alteracoes para origin/$currentBranch..." -ForegroundColor Cyan
    git push origin $currentBranch
    if ($LASTEXITCODE -ne 0) {
        Write-Host "[AVISO] Tentando push definindo upstream (-u origin $currentBranch)..." -ForegroundColor Yellow
        git push -u origin $currentBranch
    }

    if ($LASTEXITCODE -eq 0) {
        Write-Host "[SUCESSO] Push concluido com sucesso!" -ForegroundColor Green
    } else {
        Write-Host "[ERRO] Falha ao realizar push." -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "=======================================================" -ForegroundColor Cyan
Write-Host "Operacao finalizada." -ForegroundColor Cyan
Write-Host "=======================================================" -ForegroundColor Cyan
