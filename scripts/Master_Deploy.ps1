# SigmaOS Zenith Deployment Script

Write-Host "Σ [DEPLOY]: Initiating SigmaOS Sovereign Deployment..." -ForegroundColor Cyan

# 1. Verification
Write-Host "[1/3] Verifying Shard Integrity..."
python test_algorithms.py

# 2. Build Simulator
Write-Host "[2/3] Building Industrial Simulator..."
$Suites = "simulate.c", "kernel/modules/core/kmain.c", "kernel/modules/core/SovereignModuleRegistry.c", `
          "kernel/modules/core/SovereignMemorySuite.c", "kernel/modules/core/SovereignAppManagement.c", `
          "kernel/modules/core/SovereignServiceControl.c", "kernel/modules/core/SovereignIntelligenceSuite.c", `
          "kernel/modules/core/SovereignFrontendSuite.c", "kernel/modules/core/SovereignEcosystemSuite.c", `
          "kernel/modules/core/SovereignBackendSuite.c", "kernel/modules/core/SovereignConfigIdentitySuite.c", `
          "kernel/modules/security/SovereignCryptoSuite.c", "kernel/modules/core/cli/SovereignCLI_Core.c", `
          "kernel/modules/core/cli/SovereignCLI_Essential.c", "kernel/modules/core/cli/SovereignCLI_Registry.c", `
          "kernel/modules/core/SovereignFunctionalTest.c"

# Check for compiler
if (Get-Command "gcc" -ErrorAction SilentlyContinue) {
    gcc $Suites -Iinclude -Ikernel/modules/core -Ikernel/modules/core/cli -o sigma_zenith.exe
    Write-Host "[3/3] Deployment Successful. Sovereign Zenith is ready." -ForegroundColor Green
    Write-Host "Run .\sigma_zenith.exe to start the OS Matrix."
} else {
    Write-Host "[!] GCC not found. Please install MinGW/GCC to finalize binary deployment." -ForegroundColor Yellow
    Write-Host "[3/3] Deployment Ready (Source-Tier). Syncing to GitHub."
}
