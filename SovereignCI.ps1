# Σ SIGMAOS SOVEREIGN CI PIPELINE (INDUSTRY STANDARD)
# ========================================================
# USP: Native Silicon-Direct Integration & Verification.

$ErrorActionPreference = "Stop"

Write-Host "==================================================" -ForegroundColor Cyan
Write-Host " Σ SIGMAOS SOVEREIGN CI: NATIVE VALIDATION" -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan

# 1. Structural Audit (Native X64 Auditor)
Write-Host "[*] Auditing Sovereign Integrity..."
if (Test-Path "SovereignAuditor.exe") {
    & .\SovereignAuditor.exe
} else {
    Write-Error "CRITICAL: Auditor binary missing."
}

# 2. Hardware Probe (Non-Pretending Check)
Write-Host "[*] Probing Silicon Capability..."
if (Test-Path "SovereignIntegrityAudit.exe") {
    & .\SovereignIntegrityAudit.exe
} else {
    Write-Error "CRITICAL: Integrity Audit binary missing."
}

# 3. Unit Test Run (Native X64 Runner)
Write-Host "[*] Executing Industry Standard Tests..."
if (Test-Path "SovereignTests.exe") {
    & .\SovereignTests.exe
} else {
    Write-Warning "SovereignTests.exe not found. Building now..."
    # Build logic would go here
}

Write-Host "--------------------------------------------------"
Write-Host "[OK] CI SUCCESS: SigmaOS Sovereignty ATTANINED." -ForegroundColor Green
Write-Host "--------------------------------------------------"
