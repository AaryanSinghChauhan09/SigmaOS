$content = Get-Content 'zenith_desktop.css' -Raw
$content = [regex]::Replace($content, '(?m)^(\s*)(backdrop-filter:[^;]+;)\r?\n(\s*)(-webkit-backdrop-filter:[^;]+;)', '$1$4`n$3$2')
Set-Content -Path 'zenith_desktop.css' -Value $content -NoNewline
