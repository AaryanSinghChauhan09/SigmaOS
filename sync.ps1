# SigmaOS Workspace Sync Protocol
# Automates Git Commit & Push for the Antigravity IDE Loop

Write-Host "--- SIGMAOS SYNC INITIATED ---" -ForegroundColor Cyan

# Check if git is initialized
if (!(Test-Path .git)) {
    Write-Host "[!] Git not detected. Initializing..." -ForegroundColor Yellow
    git init
    git add .
    git commit -m "Initial Sovereign Commit"
}

# Sync Loop
$msg = "Sovereign Sync: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
git add .
git commit -m $msg
git push origin main

Write-Host "[✓] Workspace Synced with GitHub" -ForegroundColor Green
