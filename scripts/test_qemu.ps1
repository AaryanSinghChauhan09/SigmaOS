# Σ SIGMAOS ZENITH: SOVEREIGN QEMU TEST RUNNER (v2200.0)
# Mission: Automated Boot & Smoke Test Verification.

# 1. Build Kernel Binary
Write-Host "Σ [BUILD]: Initiating Bare-Metal Compilation..." -ForegroundColor Cyan
make all

# 2. Verify Output
if (Test-Path "build/sigmaos_zenith") {
    Write-Host "Σ [SUCCESS]: Kernel Binary Generated: 0x$(Get-Item 'build/sigmaos_zenith' | % { $_.Length.ToString('X8') }) bytes." -ForegroundColor Green
} else {
    Write-Host "Σ [ERROR]: Kernel Binary NOT FOUND. Aborting Test." -ForegroundColor Red
    exit 1
}

# 3. Execute QEMU Emulation
Write-Host "Σ [TEST]: Launching QEMU Hardware Emulation..." -ForegroundColor Yellow
qemu-system-x86_64 -kernel build/sigmaos_zenith -m 512 -serial stdio -display none -no-reboot
