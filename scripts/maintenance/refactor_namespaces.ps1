$files = Get-ChildItem -Path drivers, kernel -Recurse -Include *.cpp
foreach ($file in $files) {
    $content = Get-Content -Path $file.FullName -Raw
    $original = $content

    if ($content -match 'namespace SigmaOS \{\s*namespace Kernel \{\s*namespace ([A-Za-z0-9_]+) \{\s*namespace ([A-Za-z0-9_]+) \{') {
        $ns1 = $Matches[1]
        $ns2 = $Matches[2]
        
        $content = $content -replace '(?m)^\s*\}\s*$\s*^\s*\}\s*$\s*^\s*\}\s*$\s*^\s*\}\s*$', "} // namespace $ns2`n} // namespace $ns1`n} // namespace Kernel`n} // namespace SigmaOS"
    } elseif ($content -match 'namespace SigmaOS \{\s*namespace Kernel \{\s*namespace ([A-Za-z0-9_]+) \{') {
        $ns1 = $Matches[1]
        $content = $content -replace '(?m)^\s*\}\s*$\s*^\s*\}\s*$\s*^\s*\}\s*$', "} // namespace $ns1`n} // namespace Kernel`n} // namespace SigmaOS"
    }
    
    if ($content -ne $original) {
        Set-Content -Path $file.FullName -Value $content -NoNewline
    }
}
