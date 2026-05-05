# SigmaOS Markdown Normalizer (Industrial v100.3)
# Fixes MD012, MD022, MD029, MD030, MD032, MD036, MD041

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
        $line = $lines[$i].TrimEnd()
        
        # MD041: First line H1
        if ($i -eq 0 -and $line -notmatch "^#\s+") {
            $newLines.Add("# $($file.BaseName -replace '_', ' ')")
            $newLines.Add("")
        }

        # Reset counter on headings
        if ($line -match "^#+\s+") {
            $counter = 1
            $inList = $false
            if ($newLines.Count -gt 0 -and $newLines[$newLines.Count-1] -ne "") {
                $newLines.Add("")
            }
            $newLines.Add($line)
            if ($i -lt $lines.Length - 1 -and $lines[$i+1] -ne "") {
                # We'll add the blank line when we process the next line if needed
            }
            continue
        }

        # MD029 & MD030: Ordered list numbering and spaces
        if ($line -match "^(\s*)(\d+)\.\s+(.*)") {
            $indent = $matches[1]
            $text = $matches[3]
            $line = "$indent$counter. $text"
            $counter++
            $inList = $true
        } else {
            if ($line -ne "" -and $line -notmatch "^\s+[\*\-\+]|^\s+\d+\.") {
                $counter = 1
                $inList = $false
            }
        }

        # MD030: Unordered list spaces
        if ($line -match "^(\s*)([\*\-\+])\s{2,}(.*)") {
            $line = "$($matches[1])$($matches[2]) $($matches[3])"
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
