# push_all.ps1
# Synchronizes all 18 branches to the same level as main and updates the Wiki.

$branches = @(
    "tools-dev",
    "release/standalone",
    "drivers-dev",
    "release/dual-boot",
    "release/mobile",
    "gh-pages",
    "release/microkernel",
    "release/cloud",
    "kernel-exp",
    "fs-dev",
    "master",
    "performance-optimized",
    "prepare-sigmaos-launch",
    "release/app",
    "release/browser",
    "release/distributed",
    "release/rtos",
    "docs-update"
)

Write-Host "Re-creating all local branches at HEAD of main..." -ForegroundColor Cyan
foreach ($branch in $branches) {
    # Delete the local branch if it already exists to ensure it aligns with main
    git branch -D $branch 2>$null
    # Create the branch pointing to main
    git branch $branch main
}

Write-Host "Force pushing all branches to remote origin..." -ForegroundColor Cyan
# Push main first
git push origin main --force

# Push each branch to its remote counterpart
foreach ($branch in $branches) {
    Write-Host "Pushing $branch to origin..." -ForegroundColor Yellow
    git push origin $branch --force
}

Write-Host "Checking wiki_repo status..." -ForegroundColor Cyan
cd wiki_repo
git checkout master 2>$null
git add .
git commit -m "Update wiki docs (automated sync)"
git push origin master

Write-Host "All branches and wiki have been successfully synchronized!" -ForegroundColor Green
