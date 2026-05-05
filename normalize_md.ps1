# SigmaOS Markdown Normalizer (Industrial v100)
# Fixes MD012 (Blank lines) and MD029 (List prefixes)

$docs = Get-ChildItem -Path "c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS-Repo\docs", "c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS-Repo\README.md", "c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS.wiki" -Filter "*.md" -Recurse

foreach ($file in $docs) {
    $content = Get-Content $file.FullName -Raw
    
    # Fix MD012: Multiple consecutive blank lines -> Single blank line
    $content = $content -replace "\r?\n\s*\r?\n(\s*\r?\n)+", "`r`n`r`n"
    
    # Fix MD029: Normalize ordered lists to start from 1 and be sequential
    # This is a bit complex for a regex, so we'll do it line by line for simple cases
    $lines = $content -split "`r`n"
    $newList = $false
    $counter = 1
    for ($i = 0; $i -lt $lines.Length; $i++) {
        if ($lines[$i] -match "^\d+\.\s+(.*)") {
            $lines[$i] = "$counter. $($matches[1])"
            $counter++
            $newList = $true
        } else {
            if ($newList) { $counter = 1; $newList = $false }
        }
    }
    $content = $lines -join "`r`n"
    
    Set-Content -Path $file.FullName -Value $content -NoNewline
    Write-Host "Normalized: $($file.Name)"
}
