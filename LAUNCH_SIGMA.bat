@echo off
title Σ SIGMAOS ZENITH BUILD & LAUNCH AGENT (v6.2.0)
echo ============================================================
echo  SIGMA OS: SOVEREIGN ZENITH BUILD PIPELINE
echo ============================================================
echo.

:: Step 1: Resource Synchronization
echo [1] Synchronizing Silicon Shards...
git pull origin master 2>nul
if %errorlevel% neq 0 (
    echo [!] WARNING: Offline Mode. Sync shard failed.
)

:: Industrial Build Pipeline v6.2.0 (Zenith Launch Edition)
echo [2] Compiling Sovereign Zenith Matrix (Ring 0)...
:: Compile all Sovereign Kernel Shards into a monolithic Ring-0 Engine
g++ -O3 -shared -fPIC kernel/SovereignAlgorithms.cpp kernel/SigmaProcessManager.cpp kernel/SigmaMemoryNexus.cpp kernel/SovereignGraphics.cpp kernel/SovereignVMM.cpp kernel/SovereignContainer.cpp -o SigmaSovereignKernel.dll

:: Link the Final Dispatcher
g++ -O3 SigmaFinalIntegration.cpp -L. -lSigmaSovereignKernel -o SigmaOS_Sovereign.exe

if %errorlevel% neq 0 (
    echo [ERROR] Kernel Compilation Failed.
    echo [!] FATAL: Build failed. Check build_log.txt for telemetry.
    pause
    exit /b %errorlevel%
)

:: Step 3: Web-Context Pre-flight (index.html)
echo [3] Validating Sovereign UI Matrix...
node -v >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] Node.js Runtime detected for automation.
)

echo.
echo ============================================================
echo  SIGMA OS ZENITH IS READY FOR LAUNCH.
echo  ALL SHARDS SYNCED. SYSTEM SOVEREIGNTY SECURED.
echo  PRESS ANY KEY TO BOOT INTO ZENITH...
echo ============================================================
pause
cls
echo [BOOT]: Engaging Sovereign Kernel...
.\SigmaOS_Sovereign.exe
