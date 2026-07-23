#!/usr/bin/env pwsh
# merge_all.ps1 - Merge all remote branches into main and delete them

git config pull.rebase false

# Make sure we are on main and up to date
git checkout main
git pull origin main

# Get all remote branches except main and HEAD
$branches = git branch -r | Select-String -NotMatch "origin/main" | Select-String -NotMatch "origin/HEAD" | ForEach-Object { $_.Line.Trim() }

foreach ($branch in $branches) {
    Write-Host "========================================="
    Write-Host "Attempting to merge $branch" -ForegroundColor Cyan
    
    # Try merging
    git merge $branch --no-edit
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Merge conflict detected. Running resolve_conflicts.ps1" -ForegroundColor Yellow
        
        # We might have conflicts outside .rs files, resolve_conflicts.ps1 only handles .rs
        # Let's run it anyway
        .\resolve_conflicts.ps1
        
        # for other files, accept theirs
        git checkout --theirs .
        
        # We also need to add all resolved files
        git add .
        
        # Commit the merge
        git commit -m "Merge branch '$branch' into main" --no-edit
        
        if ($LASTEXITCODE -ne 0) {
            Write-Host "Failed to commit after resolving conflicts. Aborting merge." -ForegroundColor Red
            git merge --abort
            continue
        }
    }
    
    Write-Host "Successfully merged $branch" -ForegroundColor Green
    
    # Extract the local branch name (remove 'origin/')
    $localBranch = $branch -replace "^origin/", ""
    
    # Delete the remote branch
    Write-Host "Deleting remote branch $localBranch" -ForegroundColor Yellow
    git push origin --delete $localBranch
    
    # Delete the local branch if it exists
    git branch -D $localBranch
}

Write-Host "Finished processing all branches." -ForegroundColor Green
