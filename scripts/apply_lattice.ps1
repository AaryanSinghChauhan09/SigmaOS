$files = Get-ChildItem -Path kernel,drivers -Filter *.cpp -Recurse
foreach ($file in $files) {
    $content = Get-Content $file.FullName
    if ($content -and $content[0] -notmatch '#include "Lattice.h"') {
        $newContent = @('#include "Lattice.h"') + $content
        $newContent | Set-Content $file.FullName
        Write-Host "Applied Lattice.h to $($file.FullName)"
    }
}
