@echo off
:: SigmaOS Sovereign Apex Bootstrapper
:: =================================
:: USP: One-click sovereign environment activation.
:: Bypasses standard shell to launch the Apex Kernel.

set SIGMA_ROOT=%~dp0
cd /d %SIGMA_ROOT%

echo [*] Initializing SigmaOS Sovereign Apex v3.1...
echo [*] Silicon: x64 | Kernel: Shard-Load | State: LOCKED
echo.

:: Launch the native sovereign kernel (The only logic-holding C++ Zenith entry point)
set PYTHONPATH=%SIGMA_ROOT%
if exist SigmaKernel.exe (
    SigmaKernel.exe
) else (
    echo [!] ALERT: SigmaKernel.exe binary not found. Running setup sharding...
    call SETUP_SIGMA_NATIVE.bat
    SigmaKernel.exe
)

if %ERRORLEVEL% NEQ 0 (
    echo [!] KERNEL_HALT: OS failed to bootstrap. Check manifest.apex.
    pause
)
