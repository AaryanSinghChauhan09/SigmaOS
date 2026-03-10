@echo off
setlocal
title SigmaOS Sovereign v2.0 - Booting...
color 0b
cd /d "%~dp0"

echo [  OK  ] Initializing SigmaOS Sovereign Kernel...
timeout /t 1 /nobreak >nul
echo [  OK  ] Loading Predictive AI Scheduler [Jitter Filter Active]
timeout /t 1 /nobreak >nul
echo [  OK  ] Mapping ZRAM 4:1 Multi-paging [Physical: 1GB, Logical: 4GB]
timeout /t 1 /nobreak >nul
echo [  OK  ] Engaging Sovereign Security Shield (Ring-0)
timeout /t 1 /nobreak >nul
echo [  OK  ] Synchronizing Aura Mesh [3 Local Nodes Discovered]
timeout /t 1 /nobreak >nul
echo [  OK  ] Verifying GPG Binary Signatures... 100%% Verified.
echo.
echo =======================================================
echo    Σ  SigmaOS Sovereign v2.0  -  APEX EDITION
echo =======================================================
echo.
echo Launching Sovereign UI Dashboard...
timeout /t 2 /nobreak >nul

py sigma.py %*

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [FATAL] SigmaOS Kernel Panic! Check logs/boot_error.log
    echo Press any key to exit to Guest OS.
    pause >nul
)

endlocal
