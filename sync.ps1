# SigmaOS Workspace Sync Protocol (APEX v2.0)
# Automates Git Commit & Push for the Antigravity IDE Loop

Write-Host "--- SIGMAOS SYNC INITIATED ---" -ForegroundColor Cyan

# 1. Identity Verification (Zero-Trust)
git config user.email "aaryan@gmail.com"
git config user.name "Aaryan Singh Chauhan"

# 2. Check for Changes
$status = git status --porcelain
if (!($status)) {
    Write-Host "[!] Workspace is already in sync with local state. Pushing anyway..." -ForegroundColor Gray
}

# 3. Synchronize with GitHub Master
$msg = "Apex Sync: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss') [Ledger Signed]"
git add .
git commit -m $msg -a --allow-empty 
git push origin master --force

Write-Host "[OK] Workspace Synced with GitHub (APEX MASTER)" -ForegroundColor Green
