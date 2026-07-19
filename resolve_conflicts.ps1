#!/usr/bin/env pwsh
# resolve_conflicts.ps1 - Resolve all remaining conflict markers in .rs files
# Strategy: keep the INCOMING (theirs) side of every conflict

$SrcDir = "src"

$files = Get-ChildItem -Recurse -Filter "*.rs" $SrcDir | 
    Select-String -Pattern "<<<<<<< HEAD" | 
    Select-Object -ExpandProperty Path | 
    Sort-Object -Unique

if ($files.Count -eq 0) {
    Write-Host "No conflict markers found." -ForegroundColor Green
    exit 0
}

Write-Host "Files with conflicts:" -ForegroundColor Yellow
$files | ForEach-Object { Write-Host "  $_" }
Write-Host ""

foreach ($file in $files) {
    Write-Host "Resolving: $file" -ForegroundColor Cyan
    $lines = Get-Content $file -Encoding UTF8

    $result = [System.Collections.ArrayList]@()
    $state = "normal"   # states: normal, head, theirs

    foreach ($line in $lines) {
        if ($line -match "^<<<<<<<") {
            $state = "head"
        } elseif ($line -match "^=======") {
            $state = "theirs"
        } elseif ($line -match "^>>>>>>>") {
            $state = "normal"
        } else {
            if ($state -eq "normal" -or $state -eq "theirs") {
                [void]$result.Add($line)
            }
        }
    }

    $output = ($result -join "`r`n") + "`r`n"
    [System.IO.File]::WriteAllText($file, $output, [System.Text.UTF8Encoding]::new($false))
    Write-Host "  Done." -ForegroundColor Green
}

Write-Host ""
Write-Host "All conflicts resolved." -ForegroundColor Green
