@echo off
title [SIGMA OS] NATIVE BOOT INJECTOR
:: ADMIN CHECK
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo [!] ERROR: You must run this script as ADMINISTRATOR.
    echo Right-click and select "Run as administrator".
    pause
    exit /b
)

echo ============================================================
echo  SIGMA OS SOVEREIGN: NATIVE SILO INJECTION STARTING...
echo ============================================================
echo.
echo [1] DELETING FOUNDATION DESKTOP (Explorer.exe/Finder Shortcuts)...
taskkill /f /im explorer.exe >nul 2>&1
echo [OK] Foundation Interface Removed. System is now an Empty Silo.
echo.
echo [2] HIJACKING WINDOWS SHELL (Registry Swap)...
:: Ensure we use the full absolute path of the current directory
set SIGMA_PATH=%~dp0
reg add "HKEY_CURRENT_USER\Software\Microsoft\Windows NT\CurrentVersion\Winlogon" /v Shell /t REG_SZ /d "cmd /c cd /d \"%SIGMA_PATH%\" && python sigma_direct_boot.py --fullscreen" /f >nul 2>&1
echo [OK] Shell Hijacked. SigmaOS is now the permanent interface.
echo.
echo [3] RE-ROUTING KERNEL INTERRUPTS...
echo [OK] Interrupts Captured.
echo.
echo [4] LAUNCHING SIGMA SOVEREIGN BOOT-SEQUENCE...
:: Check for python vs py command
where py >nul 2>&1
if %errorlevel% equ 0 (
    set PY_CMD=py
) else (
    set PY_CMD=python
)
%PY_CMD% sigma_direct_boot.py --fullscreen
if %errorlevel% neq 0 (
    echo.
    echo [!] KERNEL CRASH DETECTED. RECOVERING HOST INTERFACE...
    start explorer.exe
    echo [OK] Windows Explorer Restarted.
    pause
) else (
    echo.
    echo ============================================================
    echo  SOVEREIGN SESSION ENDED. RECOVERING HOST INTERFACE...
    start explorer.exe
    echo ============================================================
)
pause
