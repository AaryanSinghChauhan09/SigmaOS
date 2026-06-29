$branches = @(
    "main",
    "master",
    "release/standalone",
    "release/cloud",
    "release/mobile",
    "release/dual-boot",
    "release/microkernel"
)

Write-Host "Syncing linux-compat driver integration across core branches..."

foreach ($branch in $branches) {
    Write-Host "Checking out $branch..."
    git checkout $branch
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Failed to checkout $branch. Skipping." -ForegroundColor Red
        continue
    }

    Write-Host "Merging drivers-dev into $branch..."
    git merge drivers-dev -m "Merge drivers-dev (linux-compat layer) into $branch"
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Merge conflict or error in $branch. Aborting merge." -ForegroundColor Yellow
        git merge --abort
    } else {
        Write-Host "Successfully synced $branch." -ForegroundColor Green
    }
}

Write-Host "Returning to drivers-dev branch..."
git checkout drivers-dev
Write-Host "Sync complete!"
