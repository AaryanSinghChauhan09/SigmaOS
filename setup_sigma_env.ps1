# =========================================================================
# Σ SIGMAOS: SOVEREIGN ENVIRONMENT AUTOMATION (v1.0)
# =========================================================================
# This script automates the installation of essential toolchains
# for building SigmaOS natively on Windows, enforcing "Ease of Use"
# and "Automation/Personalization" directives.
# =========================================================================

Write-Host "[SIGMA-AUTO]: Bootstrapping Sovereign Build Environment..." -ForegroundColor Cyan

# Check for Chocolatey (Package Manager for Windows)
if (!(Get-Command choco -ErrorAction SilentlyContinue)) {
    Write-Host "[SIGMA-AUTO]: Chocolatey not found. Initiating automated installation..." -ForegroundColor Yellow
    Set-ExecutionPolicy Bypass -Scope Process -Force
    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
    Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
    Write-Host "[SIGMA-AUTO]: Chocolatey installed. Restart your shell to use choco commands natively." -ForegroundColor Green
} else {
    Write-Host "[SIGMA-AUTO]: Chocolatey detected." -ForegroundColor Green
}

# Install core build tools: make and gcc (mingw)
Write-Host "[SIGMA-AUTO]: Validating GCC and Make toolchains..." -ForegroundColor Cyan
choco install make -y
choco install mingw -y

Write-Host "[SIGMA-AUTO]: Environment automated. SigmaOS repository is ready for a raw 'make' build." -ForegroundColor Green
