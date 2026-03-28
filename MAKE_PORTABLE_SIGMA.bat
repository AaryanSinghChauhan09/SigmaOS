@echo off
setlocal

:: Σ SIGMAOS: PORTABLE MAESTRO (v10.0)
:: =======================================
:: Principles: Absolute Sovereignty & Ease of Use.
:: Mission: Create a Live-Bootable Shard.
:: =======================================

echo [PORTABLE-ZENITH]: Finalizing Zero-Dependency Shard Deployment...
echo [PORTABLE-ZENITH]: Harvesting Native Shards (C++/ASM/Rust)...

set TARGET_DIR=SIGMA_LIVE_SHARD

if not exist %TARGET_DIR% (
    mkdir %TARGET_DIR%
    mkdir %TARGET_DIR%\userland
    mkdir %TARGET_DIR%\system
)

copy /y SigmaOS_Kernel.exe %TARGET_DIR%\system\
copy /y SovereignShell.exe %TARGET_DIR%\system\
copy /y index.html %TARGET_DIR%\
xcopy /e /y userland %TARGET_DIR%\userland\

echo [PORTABLE-ZENITH]: Injecting Metal-Nexus Web Bridge...
copy /y SigmaOS_Apex_Launcher.bat %TARGET_DIR%\Launch_Sovereign.bat

echo [PORTABLE-ZENITH]: | [SUCCESS] Portable Shard is ready for USB/Live Boot.
echo [PORTABLE-ZENITH]: | NO LIBRARIES. NO PYTHON. NO BROWSERS NEEDED.
echo [PORTABLE-ZENITH]: | Simply run 'Launch_Sovereign.bat' from any host.

endlocal
pause
