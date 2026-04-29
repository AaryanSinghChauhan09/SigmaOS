$files = Get-ChildItem -Path kernel -Filter *.cpp -Recurse
foreach ($file in $files) {
    $content = Get-Content $file.FullName
    # Replace both angled and relative paths with simple quoted includes
    $newContent = $content -replace '#include <sigma_(.*)\.h>', '#include "sigma_$1.h"'
    $newContent = $newContent -replace '#include "../../include/sigma_(.*)\.h"', '#include "sigma_$1.h"'
    $newContent | Set-Content $file.FullName
}

$headers = Get-ChildItem -Path include -Filter *.h -Recurse
foreach ($header in $headers) {
    $content = Get-Content $header.FullName
    # Ensure all internal headers use quoted includes
    $newContent = $content -replace '#include <sigma_(.*)\.h>', '#include "sigma_$1.h"'
    $newContent | Set-Content $header.FullName
}
