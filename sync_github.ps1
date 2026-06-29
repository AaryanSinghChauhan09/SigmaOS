# SigmaOS: Sovereign Git Sync & Wiki Update Script
# Pushes latest changes to GitHub and syncs the associated Wiki.

$repo_url = "https://github.com/AaryanSinghChauhan09/SigmaOS.git"

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
    # Copy updated docs to the wiki
    $wiki_docs = @(
        ".\docs\wiki\Sovereignty-Architecture.md",
        ".\docs\wiki\CLI-Reference.md",
        ".\docs\wiki\CI-Workflows.md"
    )
    foreach ($doc in $wiki_docs) {
        if (Test-Path $doc) {
            Copy-Item $doc -Destination "$wiki_dir\$(Split-Path -Leaf $doc)" -Force
        }
    }

    Set-Location $wiki_dir
    git add .
    git diff --cached --quiet
    if ($LASTEXITCODE -ne 0) {
        git commit -m "Update wiki docs (automated sync)"
        git push origin master
        Write-Host "GitHub Wiki synced successfully!" -ForegroundColor Green
    } else {
        Write-Host "Wiki is already up to date." -ForegroundColor Yellow
    }

    Set-Location $PSScriptRoot
    Remove-Item -Recurse -Force $wiki_dir
} else {
    Write-Host "Warning: Could not clone wiki. Ensure the Wiki feature is enabled in GitHub repo settings." -ForegroundColor Yellow
}
