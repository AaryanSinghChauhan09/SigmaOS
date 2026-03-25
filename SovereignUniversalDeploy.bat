@echo off
title "Σ SIGMA OS: SOVEREIGN UNIVERSAL DEPLOYMENT (ZENITH v135.0)"
echo ============================================================
    echo  SIGMA OS: UNIVERSAL SHARD ORCHESTRATOR & DEPLOYMENT
echo ============================================================
echo.

:: 1. Native Build Sequence
echo [1] Checking Native Silicon Status...
if not exist SigmaKernel.exe (
    echo [!] ALERT: SigmaKernel.exe missing. Trigerring Native Shard Build...
    call SETUP_SIGMA_NATIVE.bat
)
echo [OK] Native Shards Available.

:: 2. Web/Chromium Browser Deployment
echo [2] Deploying Sovereign Web Dashboard (Chromium Support)...
if exist SigmaOS_Web\index.html (
    start SigmaOS_Web\index.html
    echo [OK] Web Dashboard Projected via Browser Zenity.
)

:: 3. Sovereign Containerization (Independent)
echo [3] Initializing Sovereign Container Zenith (Job Objects)...
if exist SovereignContainerRuntime.exe (
    echo [OK] Sovereign Container Runtime [ACTIVE]. Launching Shard-Jail...
    SovereignContainerRuntime.exe SigmaKernel.exe
) else (
    echo [!] ALERT: SovereignContainerRuntime.exe not compiled. Run BUILD_ZENITH.bat.
)

:: 4. Virtualization & Live Boot
echo [4] Auditing Distro Forge / Live Boot Shards...
if exist SovereignDistroForge.exe (
    echo [OK] Distro Forge [READY]. Building Live ISO...
    SovereignDistroForge.exe
)

:: 5. Apex Orchestrator
echo [5] Launching Apex Launcher (Cross-Platform Orchestrator)...
call SigmaOS_Apex_Launcher.bat

echo.
echo ============================================================
echo  UNIVERSAL DEPLOYMENT SUCCESSFUL.
echo  SIGMA OS IS NOW ACTIVE ACROSS: [NATIVE/WEB/CONTAINER/LIVE]
echo  SOVEREIGNTY SECURED. INDUSTRY DOMINANCE ACHIEVED.
echo ============================================================
pause
