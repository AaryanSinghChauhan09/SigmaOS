$wikiUrl = "https://github.com/AaryanSinghChauhan09/SigmaOS.wiki.git"
$wikiDir = "C:\Users\Aaryan\.gemini\antigravity-ide\scratch\SigmaOS.wiki"
$sourceDir = "C:\Users\Aaryan\.gemini\antigravity-ide\scratch\SigmaOS\wiki_repo\*"

if (Test-Path $wikiDir) {
    Remove-Item -Recurse -Force $wikiDir
}

git clone $wikiUrl $wikiDir

Copy-Item -Path $sourceDir -Destination $wikiDir -Recurse -Force

Set-Location $wikiDir
git add .
git commit -m "docs: Sync wiki with latest architecture roadmaps"
git push origin master -f

Write-Host "Wiki synced."
