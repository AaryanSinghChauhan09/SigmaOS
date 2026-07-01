# SigmaOS Markdown Normalizer (Industrial v100.4)
# Fixes MD012, MD022, MD029, MD030, MD032, MD036, MD041

$docs = Get-ChildItem -Path "C:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS", "C:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\WIKI" -Filter "*.md" -Recurse

foreach ($file in $docs) {
    if ($file.FullName -like "*node_modules*") { continue }
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
            if ($line -match "^#+\s+") {
                 # It's a heading but not H1, convert or leave? 
                 # Let's enforce H1 if it's the very first line
                 $line = "# " + ($line -replace "^#+\s*", "")
            } else {
                $newLines.Add("# $($file.BaseName -replace '_', ' ')")
                $newLines.Add("")
            }
        }

        # MD036: No emphasis as heading (simplified: if line is just bold/italic and short)
        if ($line -match "^\s*[\*_]{1,2}[^\*_]+[\*_]{1,2}\s*$" -and $line.Length -lt 100) {
            $line = "### " + ($line -replace "[\*_]", "")
        }

        # MD029 & MD030: Ordered list numbering
        if ($line -match "^(\s*)(\d+)\.\s+(.*)") {
            $indent = $matches[1]
            $text = $matches[3]
            # Reset counter if indent changes significantly or if it's a new list
            if ($indent.Length -eq 0) {
                 $line = "$counter. $text"
                 $counter++
            } else {
                 # Nested list, we'll leave numbering as is for now or use 1.
                 $line = "$indent$($matches[2]). $text"
            }
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

        # MD022: Blanks around headings
        if ($line -match "^#+\s+") {
            if ($newLines.Count -gt 0 -and $newLines[$newLines.Count-1] -ne "") {
                $newLines.Add("")
            }
        }

        $newLines.Add($line)
        
        # MD022: Blank after heading
        if ($line -match "^#+\s+") {
            if ($i -lt $lines.Length - 1 -and $lines[$i+1] -ne "") {
                $newLines.Add("")
            }
        }
    }
    
    $content = $newLines -join "`r`n"
    # Final cleanup of multiple blanks again just in case
    $content = $content -replace "(\r?\n){3,}", "`r`n`r`n"
    
    Set-Content -Path $file.FullName -Value $content -NoNewline
    Write-Host "Normalized: $($file.Name)"
}
