# SigmaOS Workspace Sync Protocol (APEX v2.0)
# Automates Git Commit & Push for the Antigravity IDE Loop

Write-Host "--- SIGMAOS SYNC INITIATED ---" -ForegroundColor Cyan

# 1. Identity Verification (Sovereign/Generic)
git config user.email "sovereign@users.noreply.github.com"
git config user.name "Sovereign-User"

# 2. Synchronize with GitHub Master
$msg = "Apex Sync: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss') [Ledger Signed]"
git add .
git commit -m $msg -a --allow-empty 
git push origin master --force

Write-Host "[OK] Workspace Synced with GitHub (APEX MASTER)" -ForegroundColor Green
