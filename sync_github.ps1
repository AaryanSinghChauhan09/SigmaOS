# =========================================================================
# SigmaOS Windows GitHub Sync Script (PowerShell)
# =========================================================================
# This script commits all recent changes to the main repository and 
# pushes the docs/wiki folder to the GitHub Wiki repository.
# =========================================================================

$ErrorActionPreference = "Stop"

$MainRepoUrl = "https://github.com/AaryanSinghChauhan09/SigmaOS.git"
$WikiRepoUrl = "https://github.com/AaryanSinghChauhan09/SigmaOS.wiki.git"
$CommitMsg = "SigmaOS Phase 9: Zenith SDK Rust/Python Bindings, Builder CLI, and Dev Guide"

Write-Host "🚀 Starting Windows GitHub Sync for SigmaOS..." -ForegroundColor Cyan

# 1. Sync the Main Repository
Write-Host "[1/3] Syncing Main Repository..." -ForegroundColor Yellow
git add .
try {
    git commit -m $CommitMsg
} catch {
    Write-Host "No changes to commit in main repo." -ForegroundColor Gray
}

try {
    git push origin main
} catch {
    Write-Host "Failed to push main repo. Are you authenticated?" -ForegroundColor Red
}

# 2. Sync the Wiki Repository
Write-Host "[2/3] Syncing Wiki Repository..." -ForegroundColor Yellow
$TempWikiDir = Join-Path $env:TEMP "SigmaOS.wiki"

if (Test-Path $TempWikiDir) {
    Remove-Item -Recurse -Force $TempWikiDir
}

Write-Host "Cloning Wiki repository..." -ForegroundColor Gray
git clone $WikiRepoUrl $TempWikiDir

Write-Host "Copying wiki files..." -ForegroundColor Gray
# Ensure target dir exists
$TargetDocs = Join-Path $TempWikiDir "*"
Copy-Item -Path "docs\wiki\*" -Destination $TempWikiDir -Recurse -Force

# Change to temp directory to push wiki
Push-Location $TempWikiDir
try {
    git add .
    try {
        git commit -m "Docs: Phase 7 - Add Sovereignty Architecture manifesto"
    } catch {
        Write-Host "No wiki changes." -ForegroundColor Gray
    }
    git push origin master
} catch {
    Write-Host "Failed to push wiki. (Note: Wikis usually use 'master' branch)" -ForegroundColor Red
} finally {
    Pop-Location
    # Clean up
    Remove-Item -Recurse -Force $TempWikiDir
}

Write-Host "[3/3] ✅ Windows GitHub Sync Complete!" -ForegroundColor Green
Write-Host "Main Repo: $MainRepoUrl" -ForegroundColor Gray
Write-Host "Wiki Repo: $WikiRepoUrl" -ForegroundColor Gray
