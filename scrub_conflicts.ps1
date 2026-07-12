$files = Get-ChildItem -Path "C:\Users\Aaryan\.gemini\antigravity-ide\scratch\SigmaOS\wiki_repo" -Filter "*.md" -Recurse
foreach ($file in $files) {
    $content = Get-Content $file.FullName
    if ($content -match "<<<<<<<" -or $content -match ">>>>>>>") {
        Write-Host "Scrubbing $($file.FullName)"
        # Very simple scrub: remove everything between <<<<<<< and >>>>>>> inclusive
        # Or better: keep the content between <<<<<<< HEAD and =======
        $newContent = @()
        $inConflict = $false
        $inTheirs = $false
        foreach ($line in $content) {
            if ($line -like "<<<<<<<*") {
                $inConflict = $true
                continue
            }
            if ($line -like "=======*") {
                $inTheirs = $true
                continue
            }
            if ($line -like ">>>>>>>*") {
                $inConflict = $false
                $inTheirs = $false
                continue
            }
            
            if (-not $inTheirs) {
                $newContent += $line
            }
        }
        $newContent | Set-Content $file.FullName
    }
}
