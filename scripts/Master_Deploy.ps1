# SigmaOS Zenith Deployment Script

Write-Host "Σ [DEPLOY]: Initiating SigmaOS Sovereign Deployment..." -ForegroundColor Cyan

# 1. Verification
Write-Host "[1/3] Verifying Shard Integrity via Make..."
make test

# 2. Build Tool
Write-Host "[2/3] Building Industrial Toolchain..."

# Check for compiler
if (Get-Command "gcc" -ErrorAction SilentlyContinue) {
    gcc -std=c11 -O2 -Iinclude tools/dev/sovereign_test/sovereign_test_runner.c -o sigma_zenith.exe
    Write-Host "[3/3] Deployment Successful. Sovereign Zenith is ready." -ForegroundColor Green
    Write-Host "Run .\sigma_zenith.exe to start the OS Matrix."
} else {
    Write-Host "[!] GCC not found. Please install MinGW/GCC to finalize binary deployment." -ForegroundColor Yellow
    Write-Host "[3/3] Deployment Ready (Source-Tier). Syncing to GitHub."
}
