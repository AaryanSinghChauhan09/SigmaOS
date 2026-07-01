$BASE = 'c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS'

function Fix-File {
    param([string]$path, [scriptblock]$transform)
    if (-not (Test-Path $path)) { return }
    $c = [System.IO.File]::ReadAllText($path, [System.Text.Encoding]::UTF8)
    if (-not $c) { return }
    $new = & $transform $c
    if ($c -cne $new) {
        [System.IO.File]::WriteAllText($path, $new, [System.Text.Encoding]::UTF8)
        Write-Host "Fixed: $path"
    }
}

# MD036: **bold on its own line** -> ### heading
$md036 = @(
    "$BASE\docs\ALGORITHMS.md",
    "$BASE\docs\architecture\FORMATS.md",
    "$BASE\docs\architecture\HAL.md",
    "$BASE\docs\PERFORMANCE.md",
    "$BASE\docs\security\PQC_HARDENING.md",
    "$BASE\FINAL_RELEASE_CHECKLIST.md",
    "$BASE\GOVERNANCE.md",
    "$BASE\SOVEREIGNTY.md",
    "$BASE\wiki_repo\Application-Layer.md",
    "$BASE\wiki_repo\Beyond-Singularity.md",
    "$BASE\wiki_repo\Browser-Integration.md",
    "$BASE\wiki_repo\Components.md",
    "$BASE\wiki_repo\DEPLOYMENT_READINESS.md",
    "$BASE\wiki_repo\Dual-Boot-Coexistence.md",
    "$BASE\wiki_repo\Home.md",
    "$BASE\wiki_repo\Independent-Deployment.md",
    "$BASE\wiki_repo\Sovereign-Network-Stack.md"
)
foreach ($f in $md036) {
    Fix-File $f {
        param($c)
        [regex]::Replace($c, '(?m)^[ \t]*(\*\*|__)([^*_\r\n]+)\1[ \t]*$', '### $2')
    }
}

# MD026: trailing punctuation in headings (remove trailing . or :)
$md026 = @(
    "$BASE\SUPPORT.md",
    "$BASE\wiki_repo\AI-ML-Nexus.md",
    "$BASE\wiki_repo\Architecture.md",
    "$BASE\wiki_repo\Beyond-Singularity.md",
    "$BASE\wiki_repo\Common-OS-Problems-Solutions.md",
    "$BASE\wiki_repo\Logic.md",
    "$BASE\wiki_repo\SigmaOS-Components.md",
    "$BASE\wiki_repo\USP.md"
)
foreach ($f in $md026) {
    Fix-File $f {
        param($c)
        [regex]::Replace($c, '(?m)^(#{1,6} [^\r\n]+)[.:](\s*)$', '$1$2')
    }
}

# MD030: multiple spaces after list markers
$md030 = @(
    "$BASE\wiki_repo\Sovereign-Industrial-Scheduler.md",
    "$BASE\wiki_repo\Sovereign-Lattice-Filesystem.md",
    "$BASE\wiki_repo\Sovereign-Memory-Management.md"
)
foreach ($f in $md030) {
    Fix-File $f {
        param($c)
        $r = [regex]::Replace($c, '(?m)^([ \t]*[-*+])  +', '$1 ')
        [regex]::Replace($r, '(?m)^([ \t]*\d+\.)  +', '$1 ')
    }
}

# MD007: 4-space indented list items -> 2-space
Fix-File "$BASE\docs\UNIFIED_TASK_MANIFEST.md" {
    param($c)
    [regex]::Replace($c, '(?m)^    ([-*+] )', '  $1')
}

# MD012: multiple consecutive blank lines -> single
$md012 = @(
    "$BASE\wiki_repo\COMPETITOR_COMPARISON.md",
    "$BASE\wiki_repo\Logic.md"
)
foreach ($f in $md012) {
    Fix-File $f {
        param($c)
        [regex]::Replace($c, '(\r?\n\r?\n)(\r?\n)+', '$1')
    }
}

# MD009: trailing spaces
$md009 = @(
    "$BASE\docs\architecture\FORMATS.md",
    "$BASE\wiki_repo\Architecture.md",
    "$BASE\wiki_repo\Shard-Optimization-Practices.md"
)
foreach ($f in $md009) {
    Fix-File $f {
        param($c)
        [regex]::Replace($c, '(?m)[ \t]+(\r?\n)', '$1')
    }
}

# MD001: heading level jump in Home.md (h3 after h1 -> h2)
Fix-File "$BASE\wiki_repo\Home.md" {
    param($c)
    [regex]::Replace($c, '(?m)^(# [^\r\n]+\r?\n)(### )', '$1## ')
}

# MD025: multiple h1 in USP.md - demote second+ h1 to h2
$uspPath = "$BASE\wiki_repo\USP.md"
if (Test-Path $uspPath) {
    $lines = Get-Content $uspPath
    $firstH1 = $true
    $changed = $false
    $out = $lines | ForEach-Object {
        if ($_ -match '^# ') {
            if ($firstH1) { $firstH1 = $false; $_ }
            else { $changed = $true; '## ' + $_.Substring(2) }
        } else { $_ }
    }
    if ($changed) { $out | Set-Content $uspPath -Encoding UTF8; Write-Host "MD025: $uspPath" }
}

# MD029: ordered list item prefix 1/1/1
Fix-File "$BASE\wiki_repo\Sovereign-Memory-Management.md" {
    param($c)
    [regex]::Replace($c, '(?m)^(\d+)\. ', '1. ')
}

# MD031: blank lines around fenced code blocks
$md031 = @(
    "$BASE\wiki_repo\Release-Horizon.md",
    "$BASE\wiki_repo\Release-Zenith-App.md",
    "$BASE\wiki_repo\Release-Zenith-Core.md",
    "$BASE\wiki_repo\Release-Zenith-Standalone.md",
    "$BASE\docs\Kernel-Developer-Handbook.md"
)
foreach ($f in $md031) {
    Fix-File $f {
        param($c)
        # Add blank line before ```
        $r = [regex]::Replace($c, "(?m)([^\r\n])(\r?\n)(``````)", '$1$2' + "`r`n" + '$3')
        # Add blank line after closing ```
        [regex]::Replace($r, "(?m)(``````[^\r\n]*)(\r?\n)([^\r\n``])", '$1$2' + "`r`n" + '$3')
    }
}

# MD040: fenced code blocks without language
$md040 = @(
    "$BASE\docs\Dual-Boot-Compatibility-Matrix.md",
    "$BASE\docs\Kernel-Developer-Handbook.md"
)
foreach ($f in $md040) {
    Fix-File $f {
        param($c)
        [regex]::Replace($c, "(?m)^(``````)\s*(\r?\n)", '${1}text$2')
    }
}

Write-Host "All targeted MD fixes complete."
