$files = Get-ChildItem -Path kernel -Include *.cpp, *.c, *.hpp, *.h -Recurse
foreach ($file in $files) {
    $content = Get-Content $file.FullName
    # Replace relative includes "../../include/XYZ.h" with "XYZ.h"
    $newContent = $content -replace '#include "../../include/(.*)"', '#include "$1"'
    # Replace angled includes <sigma_XYZ.h> with "sigma_XYZ.h"
    $newContent = $newContent -replace '#include <(sigma_.*\.h)>', '#include "$1"'
    
    # Save if changed
    if ($content -join "`n" -ne ($newContent -join "`n")) {
        $newContent | Set-Content $file.FullName
        Write-Host "Fixed includes in: $($file.FullName)"
    }
}
