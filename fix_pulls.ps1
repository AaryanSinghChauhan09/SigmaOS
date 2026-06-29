$branches = @(
    "main",
    "master",
    "release/standalone",
    "release/cloud",
    "release/mobile",
    "release/dual-boot",
    "release/microkernel",
    "drivers-dev",
    "docs-update",
    "fs-dev",
    "gh-pages",
    "kernel-exp",
    "performance-optimized",
    "prepare-sigmaos-launch",
    "release/app",
    "release/browser",
    "release/distributed",
    "release/rtos",
    "tools-dev"
)

Write-Host "Pulling and rebasing branches to fix non-fast-forward..."

foreach ($branch in $branches) {
    Write-Host "Checking out $branch..."
    git checkout $branch
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Failed to checkout $branch. Skipping." -ForegroundColor Red
        continue
    }

    Write-Host "Pulling with rebase from origin/$branch..."
    git pull --rebase origin $branch
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Rebase conflict or error in $branch. Aborting rebase." -ForegroundColor Yellow
        git rebase --abort
    } else {
        Write-Host "Successfully pulled and rebased $branch." -ForegroundColor Green
    }
}

Write-Host "Pushing all branches to origin..."
git push --all origin

Write-Host "Sync complete!"
