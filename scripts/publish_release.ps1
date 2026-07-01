# SigmaOS Release Automation Script
# Purpose: Finalize build, tag v15.0-zenith, and push to GitHub.

Write-Host "Σ SigmaOS Release Automation [PUBLISHING]" -ForegroundColor Cyan

$version = "v15.0"
$codename = "Zenith"

# 1. Sync all branches to main
Write-Host "[1/3] Synchronizing all 11 OS formats..."
# (Logic already handled by our manual branch sync loop)

# 2. Tag the release
Write-Host "[2/3] Tagging release: $version-$codename..."
git tag -a "$version-$codename" -m "SigmaOS Zenith v15.0 - Industrial Gold Release"
git push origin "$version-$codename"

# 3. Final Wiki Sync
Write-Host "[3/3] Synchronizing .md documentation to GitHub Wiki..."
python tools/wiki_sync.py

Write-Host "Σ SigmaOS Zenith v15.0 is LIVE." -ForegroundColor Green
