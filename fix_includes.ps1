$files = Get-ChildItem -Path C:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS -Recurse -Include *.c,*.h,*.cpp,*.hpp
foreach ($file in $files) {
    if (Test-Path $file.FullName) {
        $content = Get-Content $file.FullName -Raw
        if ($content -match '#include "SigmaOOP.hpp"') {
            $content = $content -replace '#include "SigmaOOP.hpp"', '#include "SigmaC11.h"'
            Set-Content -Path $file.FullName -Value $content -NoNewline
            Write-Host "Updated $($file.FullName)"
        }
    }
}
Write-Host "Done"
