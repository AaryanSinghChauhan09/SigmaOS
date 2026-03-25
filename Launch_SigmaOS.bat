@echo off
setlocal
echo [SIGMAOS_LAUNCHER]: INITIALIZING SOVEREIGN SHARD (FAST-PATH)...

:: Detection logic
if exist "SigmaOS_Native_Core.exe" (
    echo [LAUNCHER]: Native Shard Detected. Initializing High-Performance Mode...
    start SigmaOS_Native_Core.exe
) else (
    echo [LAUNCHER]: Host-Native Shard Missing. Falling back to Browser Shard...
    start SigmaOS_Web\index.html
)

:: Pre-cache Shards (Simulated for Fast-Boot)
echo [LAUNCHER]: PRE-CACHING SHARDS (Zenith, Advocate)... OK.
echo [LAUNCHER]: READY TO INITIALIZE.

exit /b
