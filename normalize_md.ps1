# SigmaOS Markdown Normalizer (Industrial v100)
# Fixes MD012 (Blank lines) and MD029 (List prefixes)

$docs = Get-ChildItem -Path "c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS-Repo\docs", "c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS-Repo\README.md", "c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS.wiki" -Filter "*.md" -Recurse

foreach ($file in $docs) {
    $content = Get-Content $file.FullName -Raw
    if (-not $content) { continue }

    # Fix MD012: Multiple consecutive blank lines -> Single blank line
    $content = $content -replace "\r?\n\s*\r?\n(\s*\r?\n)+", "`r`n`r`n"
    
    $lines = $content -split "\r?\n"
    $newLines = New-Object System.Collections.Generic.List[string]
    
    $counter = 1
    $inList = $false
    
    for ($i = 0; $i -lt $lines.Length; $i++) {
        $line = $lines[$i]
        
        # MD041: First line H1 (Simple fix: if first non-empty line isn't H1, add one)
        if ($i -eq 0 -and $line -notmatch "^#\s+") {
            $newLines.Add("# $($file.BaseName -replace '_', ' ')")
            $newLines.Add("")
        }

        # MD022: Blanks around headings
        if ($line -match "^#+\s+") {
            if ($newLines.Count -gt 0 -and $newLines[$newLines.Count-1] -ne "") {
                $newLines.Add("")
            }
            $newLines.Add($line)
            if ($i -lt $lines.Length - 1 -and $lines[$i+1] -ne "") {
                $newLines.Add("")
            }
            continue
        }

        # MD030: Spaces after list markers
        if ($line -match "^(\s*)([\*\-\+]|\d+\.)\s{2,}(.*)") {
            $line = "$($matches[1])$($matches[2]) $($matches[3])"
        }

        # MD029: Ordered list numbering
        if ($line -match "^\d+\.\s+(.*)") {
            $line = "$counter. $($matches[1])"
            $counter++
            $inList = $true
        } else {
            if ($inList -and $line -notmatch "^\s+\d+\.") { $counter = 1; $inList = $false }
        }

        # MD036: Emphasis as heading (convert **Text** to # Text if it's the whole line)
        if ($line -match "^\s*(\*\*|__)(.+)(\*\*|__)\s*$") {
            $line = "# $($matches[2])"
        }

        # MD032: Blanks around lists
        if ($line -match "^([\*\-\+]|\d+\.)\s+") {
            if ($newLines.Count -gt 0 -and $newLines[$newLines.Count-1] -notmatch "^([\*\-\+]|\d+\.)\s+" -and $newLines[$newLines.Count-1] -ne "") {
                $newLines.Add("")
            }
        }

        $newLines.Add($line)
    }
    
    $content = $newLines -join "`r`n"
    Set-Content -Path $file.FullName -Value $content -NoNewline
    Write-Host "Normalized: $($file.Name)"
}
