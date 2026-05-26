$branches = @(
    "main", "drivers-dev", "docs-update", "tools-dev", "kernel-exp", "fs-dev", 
    "gh-pages", "performance-optimized", "prepare-sigmaos-launch", 
    "release/app", "release/browser", "release/cloud", "release/distributed", 
    "release/dual-boot", "release/microkernel", "release/mobile", "release/rtos", 
    "release/standalone", "master"
)

git add .
git commit -m "fix(core): Resolve compilation errors and sync codebase"

foreach ($branch in $branches) {
    Write-Host "Syncing branch: $branch"
    git checkout -B $branch
    git push origin $branch -f
}

# Go back to main
git checkout main
Write-Host "All branches synced successfully!"
