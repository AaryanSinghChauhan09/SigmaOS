# run_sigma_tests.ps1
# SigmaOS: Sovereign Test Orchestrator (v2.0)

Write-Host "Σ [TEST]: Initiating Sovereign Atomic Test Suite..." -ForegroundColor Cyan

$Tests = Get-ChildItem -Path suites -Filter test_*.c -Recurse
$Passed = 0
$Failed = 0

foreach ($Test in $Tests) {
    $Binary = $Test.FullName.Replace(".c", ".bin")
    Write-Host "  Σ [RUN]: Testing $($Test.Name)..." -ForegroundColor Gray
    
    # Attempt to compile
    # Note: Linking against SovereignLibC for atomic verification
    $LibC = "suites\Sovereign-Kernel-Suite\SovereignLibC.c"
    & "C:\msys64\mingw64\bin\gcc.exe" -nostdlib $Test.FullName $LibC -Icore/include -o $Binary
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  Σ [PASS]: $($Test.Name) certified." -ForegroundColor Green
        $Passed++
    } else {
        Write-Host "  Σ [FAIL]: $($Test.Name) logic violation detected." -ForegroundColor Red
        $Failed++
    }
}

Write-Host "`nΣ [SUMMARY]: Tests Passed: $Passed | Tests Failed: $Failed" -ForegroundColor Yellow
if ($Failed -gt 0) { exit 1 }
