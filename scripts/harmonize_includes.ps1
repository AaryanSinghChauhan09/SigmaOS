# PowerShell Script to harmonize SigmaOS includes
Get-ChildItem -Path . -Recurse -Include *.cpp, *.hpp, *.c, *.h | ForEach-Object {
    $path = $_.FullName
    $content = Get-Content -Raw $path
    
    # Replace any variant of include SigmaOOP.hpp with <SigmaOOP.hpp>
    $content = $content -replace '#include\s+["''][^>]*SigmaOOP\.hpp["'']', '#include <SigmaOOP.hpp>'
    
    # Replace any variant of include SovereignLibC.h with <SovereignLibC.h>
    $content = $content -replace '#include\s+["''][^>]*SovereignLibC\.h["'']', '#include <SovereignLibC.h>'
    
    # Standardize sigma_types.h if it appears as a relative include
    $content = $content -replace '#include\s+["''][^>]*include/sigma_types\.h["'']', '#include <include/sigma_types.h>'
    
    [System.IO.File]::WriteAllText($path, $content)
    Write-Host "Hardened includes in: $path"
}
