# Force-push all branches to sync with local main

$branches = @(
    "gh-pages",
    "performance-optimized",
    "release/app",
    "release/browser",
    "release/cloud",
    "release/distributed",
    "release/dual-boot",
    "release/mobile",
    "release/rtos",
    "release/standalone",
    "master",
    "drivers-dev",
    "fs-dev",
    "tools-dev",
    "kernel-exp",
    "docs-update"
)

foreach ($b in $branches) {
    Write-Host "--- Syncing branch: $b ---"
    # Delete and recreate from main to ensure clean state
    git branch -D $b 2>$null
    git branch $b main
    git push --force origin $b
}

# Push main itself
Write-Host "--- Pushing main ---"
git push origin main

# Push wiki
Write-Host "--- Pushing wiki ---"
Set-Location wiki_repo
git add .
git commit -m "docs: Phase 3 wiki updates - networking, GUI, PQC, OmniPkg" 2>$null
git push origin master --force 2>$null
git push origin main --force 2>$null
Set-Location ..
