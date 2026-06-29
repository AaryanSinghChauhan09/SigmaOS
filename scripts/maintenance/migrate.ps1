$main_repo = "c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"
$wiki_repo = "c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\wiki_repo"
$log_file = "c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\migration_log.txt"

if (Test-Path $log_file) { Remove-Item $log_file }

$md_files = Get-ChildItem -Path $main_repo -Filter *.md -Recurse | Where-Object { 
    $_.FullName -notmatch '\\wiki_repo\\' -and 
    $_.FullName -notmatch '\\.git\\' -and 
    $_.Name -ne 'README.md' 
}

$migrated_count = 0

foreach ($file in $md_files) {
    if ($file.Length -eq 0) { continue }
    
    $content = Get-Content -Raw -Encoding UTF8 $file.FullName
    if ($null -eq $content) { continue }
    
    # Check for completeness
    $has_todo = $content -match "(?i)\bTODO\b|\bplaceholder\b|\bTBD\b"
    
    # Basic empty section check: Heading followed immediately by another heading or EOF
    $has_empty_section = $content -match "(?m)^#+[^\n]*\s*^#+"
    
    if (-not $has_todo -and -not $has_empty_section) {
        $rel_path = $file.FullName.Substring($main_repo.Length + 1)
        $wiki_name = $rel_path -replace '\\|/', '-'
        $wiki_dest = Join-Path $wiki_repo $wiki_name
        
        Copy-Item -Path $file.FullName -Destination $wiki_dest
        Remove-Item -Path $file.FullName
        
        Add-Content -Path $log_file -Value "Migrated $rel_path to $wiki_name"
        $migrated_count++
    }
}

Write-Output "Migration complete. Migrated $migrated_count files."
