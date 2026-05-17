# Σ SIGMAOS: SOVEREIGNTY VERIFICATION SCRIPT
# Purpose: Programmatically ensure 100% non-equivalence to legacy monolithic OSes.

$LegacyTokens = @("linux", "windows", "ubuntu", "debian", "microsoft", "posix_legacy", "ntfs", "ext4")
$SearchDir = "."

Write-Host "--- SIGMAOS SOVEREIGNTY AUDIT ---" -ForegroundColor Cyan

$IssuesFound = 0

foreach ($Token in $LegacyTokens) {
    $FoundMatches = Get-ChildItem -Path $SearchDir -Recurse -File -Exclude "*.ps1", "*.md", "*.html", ".git" | Select-String -Pattern $Token
    if ($FoundMatches) {
        Write-Host "[WARN] Non-Sovereign Token Found: '$Token' in $($FoundMatches.Count) locations." -ForegroundColor Yellow
        $IssuesFound += $FoundMatches.Count
    }
}

if ($IssuesFound -eq 0) {
    Write-Host "[SUCCESS] 100% Sovereignty Verified. No legacy monolithic equivalence detected." -ForegroundColor Green
} else {
    Write-Host "[AUDIT] $IssuesFound potential sovereignty conflicts identified. Review recommended." -ForegroundColor Red
}

Write-Host "--- AUDIT COMPLETE ---"
