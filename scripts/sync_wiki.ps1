$sourceDir = "c:\Users\Aaryan\.gemini\antigravity-ide\scratch\SigmaOS"
$wikiDir = "c:\Users\Aaryan\.gemini\antigravity-ide\scratch\SigmaOS\wiki_repo"

# Ensure wiki dir exists
if (-not (Test-Path $wikiDir)) { New-Item -ItemType Directory -Path $wikiDir }

# Copy all .md files recursively from source, but flatten them for the wiki
Get-ChildItem -Path $sourceDir -Recurse -Filter *.md | Where-Object { $_.FullName -notmatch '\\wiki_repo\\' } | ForEach-Object {
    $destName = $_.Name.Replace(" ", "-")
    $destPath = Join-Path $wikiDir $destName
    
    # Standardize content (ensure dashes for lists, etc. already done by lint fix)
    Copy-Item $_.FullName $destPath -Force
}

# Fix specific wiki requirements
# GitHub Wiki Home page is Home.md
if (Test-Path (Join-Path $sourceDir "README.md")) {
    Copy-Item (Join-Path $sourceDir "README.md") (Join-Path $wikiDir "Home.md") -Force
}

Write-Host "Wiki Sync COMPLETE."
