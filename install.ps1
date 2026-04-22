# SigmaOS One-Command Install Script (Windows PowerShell)
# Usage: iwr -useb https://raw.githubusercontent.com/AaryanSinghChauhan09/SigmaOS/main/install.ps1 | iex
$ErrorActionPreference = "Stop"
$SIGMA_REPO   = "https://github.com/AaryanSinghChauhan09/SigmaOS"
$INSTALL_DIR  = "$env:LOCALAPPDATA\SigmaOS"
$BIN_DIR      = "$env:LOCALAPPDATA\SigmaOS\bin"

Write-Host "`n  SigmaOS Sovereign Lattice Installer v1.0`n" -ForegroundColor Cyan

foreach ($dep in @("git", "node", "python")) {
    if (Get-Command $dep -ErrorAction SilentlyContinue) {
        Write-Host "[OK] $dep found" -ForegroundColor Green
    } else {
        Write-Host "[WARN] $dep not found — install manually" -ForegroundColor Yellow
    }
}

if (Test-Path $INSTALL_DIR) {
    Write-Host "[INFO] Updating existing installation..." -ForegroundColor Cyan
    git -C $INSTALL_DIR pull --rebase
} else {
    Write-Host "[INFO] Cloning to $INSTALL_DIR..." -ForegroundColor Cyan
    git clone --recurse-submodules $SIGMA_REPO $INSTALL_DIR
}

Set-Location $INSTALL_DIR
npm install --silent

# Create sigmactl shim
New-Item -ItemType Directory -Force -Path $BIN_DIR | Out-Null
$shimContent = "@echo off`npython `"$INSTALL_DIR\sigmactl.py`" %*"
Set-Content "$BIN_DIR\sigmactl.cmd" $shimContent
Write-Host "[OK] sigmactl installed to $BIN_DIR\sigmactl.cmd" -ForegroundColor Green

# Add to PATH if not already present
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($currentPath -notlike "*$BIN_DIR*") {
    [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$BIN_DIR", "User")
    Write-Host "[OK] $BIN_DIR added to User PATH" -ForegroundColor Green
}

python "$INSTALL_DIR\sigmactl.py" wizard

Write-Host "`n[OK] SigmaOS installed!" -ForegroundColor Green
Write-Host "[INFO] Start Zenith: cd $INSTALL_DIR ; node server.js" -ForegroundColor Cyan
Write-Host "[INFO] Use CLI:      sigmactl --help" -ForegroundColor Cyan
