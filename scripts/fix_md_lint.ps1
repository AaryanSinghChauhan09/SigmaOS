Get-ChildItem -Path . -Recurse -Filter *.md | ForEach-Object {
    $Path = $_.FullName
    $Content = Get-Content $Path -Raw
    if (-not $Content) { return }

    # MD009: No trailing spaces
    $NewContent = [regex]::Replace($Content, '(?m)[ \t]+$', '')

    # MD022: Blanks around headings
    $NewContent = [regex]::Replace($NewContent, '(?m)([^\r\n])\r?\n(#+ )', "`$1`r`n`r`n`$2")
    $NewContent = [regex]::Replace($NewContent, '(?m)(#+ [^\r\n]+)\r?\n([^\r\n])', "`$1`r`n`r`n`$2")

    # MD032: Blanks around lists
    $NewContent = [regex]::Replace($NewContent, '(?m)([^\r\n])\r?\n([-*+] |[0-9]+\. )', "`$1`r`n`r`n`$2")

    # MD036: No emphasis as heading
    $NewContent = [regex]::Replace($NewContent, '(?m)^\s*(\*\*|__)([^*_]+)\1\s*$', '### $2')

    # MD004: Unordered list style (Force -)
    $NewContent = [regex]::Replace($NewContent, '(?m)^[ \t]*\* ', '- ')

    # MD012: Multiple consecutive blank lines
    $NewContent = [regex]::Replace($NewContent, '(?:\r?\n){3,}', "`r`n`r`n")

    if ($Content -cne $NewContent) {
        Set-Content -Path $Path -Value $NewContent -NoNewline -Encoding UTF8
        Write-Host "Linted: $Path"
    }
}
