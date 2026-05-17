$dir = "c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\wiki_repo"
$mds = Get-ChildItem -Path $dir -Filter *.md

foreach ($file in $mds) {
    $c = Get-Content -Raw -Encoding UTF8 $file.FullName
    
    $c = [regex]::Replace($c, '(\*\*|\*|_|__)\s+(.*?)\s+(\1)', '$1$2$1')
    $c = [regex]::Replace($c, '(?m)^(#+ .*):(\s*)$', '$1$2')
    $c = [regex]::Replace($c, '(?m)^(\*\*|__)(.+?)\1\s*$', '### $2')
    $c = [regex]::Replace($c, '([^\n])\n```', '$1' + "`n`n" + '```')
    $c = [regex]::Replace($c, '```\n([^\n])', '```' + "`n`n" + '$1')
    
    if ($file.Name -eq "Common-OS-Problems-Solutions.md") {
        $count = 1
        $c = [regex]::Replace($c, '(?m)^### SigmaOS Solution$', { 
            $res = "### SigmaOS Solution $count"
            $script:count++
            $res
        })
    }
    
    if ($file.Name -eq "Cybersecurity.md") {
        $c = $c -replace '\|\:---\s*\|\:---\s*\|\:---\s*\|', '|:---|:---|:---|'
    }
    
    if ($file.Name -eq "Microkernel-Format.md" -or $file.Name -eq "Mobile-Format.md" -or $file.Name -eq "RTOS-Format.md") {
        $c = $c -replace '\|------\|----------\|--------------------------\|---------\|', '|---|---|---|---|'
    }

    if ($file.Name -eq "Developer-Roadmap.md") {
        $c = [regex]::Replace($c, '(?m)^\s*\d+\.\s', '1. ')
    }
    
    [IO.File]::WriteAllText($file.FullName, $c)
}
Write-Output "Fixed Markdown files."
