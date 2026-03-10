# Σ SIGMAOS: UNIVERSAL DEPLOYMENT SCRIPT (v1.0 Pro)
# ===============================================
# This powershell script automates the SigmaOS environment setup on any Windows machine.
# Usage: powershell.exe -ExecutionPolicy Bypass -File setup.ps1

Write-Host "--- 🚀 SIGMAOS: DEPLOYING SOVEREIGN ENVIRONMENT ---" -ForegroundColor Cyan

# 1. Environment Verification
$python_version = py --version
if ($LASTEXITCODE -ne 0) {
    Write-Host "[!] ERROR: Python (py launcher) not found. Please install Python from python.org." -ForegroundColor Red
    exit 1
}
Write-Host "[+] Found Python: $python_version"

# 2. Dependency Injection
Write-Host "[*] Injecting Core Dependencies: psutil, requests..."
py -m pip install psutil requests --quiet
if ($LASTEXITCODE -ne 0) {
    Write-Host "[!] Pip installation failed. Check internet connection for first-time setup." -ForegroundColor Yellow
}

# 3. Running Sigma Setup Engine
Write-Host "[*] Handing over to Sigma Setup Hub (Hydration Sequence)..."
py sigma_setup.py --portable
if ($LASTEXITCODE -ne 0) {
    Write-Host "[!] Sigma Setup Engine failed. Check local file permissions." -ForegroundColor Red
    exit 1
}

# 4. Identity Scrub (Sanity Check)
Write-Host "[*] Performing final Forensic Scrub..."
py sigma_scrubber.py

# 5. Boot Confirmation
Write-Host "--- [OK] DEPLOYMENT COMPLETE ---" -ForegroundColor Green
Write-Host "You can now launch the OS using: py boot.py" -ForegroundColor Cyan
