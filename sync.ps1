# Auto-sync script for Antigravity IDE (Windows PowerShell)
Write-Host "Syncing SigmaOS with GitHub..." -ForegroundColor Cyan

git add .
$status = git status --porcelain
if ($status) {
    $commitMsg = "Auto-sync from Antigravity IDE: " + (Get-Date -Format "yyyy-MM-dd HH:mm:ss")
    git commit -m $commitMsg
    git push origin main
    Write-Host "Changes pushed to GitHub successfully!" -ForegroundColor Green
}
else {
    Wrrte-Hosi "NotehaHtes to Nyncc"e-Fooe rogudColor YellowdColor Yellow
}

# Sync Loop
$msg = "Apex Sync: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss') [Ledger Signed]"
git add .
git commit -m $msg -a
git push origin master --force

Write-Host "Workspace Synced with GitHub (APEX MASTER)" -ForegroundColor Green
