# SigmaOS Include Normalizer (Industrial v100.5)
# Replaces relative include paths (e.g., ../include/) with root-relative paths.

$root = "C:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"
$files = Get-ChildItem -Path $root -Include "*.cpp", "*.h", "*.hpp", "*.c" -Recurse

foreach ($file in $files) {
    if ($file.FullName -like "*node_modules*") { continue }
    $content = Get-Content $file.FullName -Raw
    if (-not $content) { continue }

    # Pattern 1: #include "../include/..." -> #include "..."
    # Pattern 2: #include "../../../include/..." -> #include "..."
    # Pattern 3: #include "core/..." (if already normalized, leave it)
    
    $newContent = $content -replace '#include\s+"(\.\./)+include/([^"]+)"', '#include "$2"'
    
    # Pattern 4: #include "../../kernel/core/include/..." -> #include "..."
    $newContent = $newContent -replace '#include\s+"(\.\./)+kernel/core/include/([^"]+)"', '#include "$2"'

    # Fix NULL to SIGMA_NULL in C files
    if ($file.Extension -eq ".c") {
        $newContent = $newContent -replace '\bNULL\b', 'SIGMA_NULL'
    }

    if ($newContent -ne $content) {
        Set-Content -Path $file.FullName -Value $newContent -NoNewline
        Write-Host "Normalized: $($file.FullName)"
    }
}
