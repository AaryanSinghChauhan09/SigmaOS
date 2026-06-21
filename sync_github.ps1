# SigmaOS: Sovereign Git Sync & Wiki Update Script
# This script pushes the latest Phase 17 changes to your GitHub repository
# and updates the associated GitHub Wiki.

$repo_url = Read-Host "Enter your GitHub repository URL (e.g. https://github.com/username/SigmaOS.git)"
if ([string]::IsNullOrWhiteSpace($repo_url)) {
    Write-Host "Error: Repository URL cannot be empty." -ForegroundColor Red
    exit 1
}

# 1. Push main codebase
Write-Host "Pushing codebase to $repo_url..." -ForegroundColor Cyan
git remote add origin $repo_url 2>$null
git push -u origin main

# 2. Sync Wiki
$wiki_url = $repo_url -replace '\.git$', '.wiki.git'
Write-Host "Syncing wiki to $wiki_url..." -ForegroundColor Cyan

$wiki_dir = "$env:TEMP\sigmaos_wiki_temp"
if (Test-Path $wiki_dir) { Remove-Item -Recurse -Force $wiki_dir }

git clone $wiki_url $wiki_dir

if (Test-Path $wiki_dir) {
    # Copy the updated Sovereignty Architecture doc to the wiki
    Copy-Item ".\docs\wiki\Sovereignty-Architecture.md" -Destination "$wiki_dir\Sovereignty-Architecture.md" -Force
    
    Set-Location $wiki_dir
    git add .
    git commit -m "Update Sovereignty Architecture (Phase 17)"
    git push origin master
    
    Set-Location $PSScriptRoot
    Remove-Item -Recurse -Force $wiki_dir
    Write-Host "GitHub Wiki synced successfully!" -ForegroundColor Green
} else {
    Write-Host "Warning: Could not clone wiki repository. Please ensure the Wiki feature is enabled in your GitHub repo settings." -ForegroundColor Yellow
}
