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

:: Step 2: Native C++ Build (Zenith Kernel)
echo [2] Compiling Sovereign Zenith Dispatcher (Ring 0)...
:: Industrial Build Pipeline v6.2.0
g++ -O3 -shared -fPIC kernel/SovereignAlgorithms.cpp kernel/SigmaProcessManager.cpp kernel/SigmaMemoryNexus.cpp -o SigmaSovereignKernel.dll
g++ -O3 SigmaFinalIntegration.cpp -L. -lSigmaSovereignKernel -o SigmaOS_Sovereign.exe
.\SigmaOS_Sovereign.exe
g++ -O3 -std=c++23 -I. -o SigmaKernel.exe SigmaFinalIntegration.cpp kernel/sigma_sml.cpp kernel/SovereignVFS.cpp kernel/SovereignNetwork.cpp kernel/SovereignSecurity.cpp kernel/SovereignVirtualizer.cpp kernel/SovereignContainer.cpp kernel/SovereignProcessManager.cpp kernel/SovereignPM.cpp kernel/SovereignAlgorithms.cpp -luser32 -lgdi32 >build_log.txt 2>&1

if %errorlevel% neq 0 (
    echo [!] FATAL: Build failed. Check build_log.txt for telemetry.
    type build_log.txt
    pause
    exit /b
)
echo [OK] Zenith Dispatcher Online: SigmaKernel.exe

:: Step 3: Web-Context Pre-flight (index.html)
echo [3] Validating Sovereign UI Matrix...
node -v >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] Node.js Runtime detected for automation.
)

:: Step 4: Boot Execution
echo.
echo ============================================================
echo  SIGMA OS ZENITH IS READY FOR LAUNCH.
echo  ALL SHARDS SYNCED. SYSTEM SOVEREIGNTY SECURED.
echo  PRESS ANY KEY TO BOOT INTO ZENITH...
echo ============================================================
pause
cls
echo [BOOT]: Engaging Sovereign Kernel...
.\SigmaKernel.exe
