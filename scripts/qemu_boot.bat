@echo off
REM =========================================================================
REM Σ SIGMAOS ZENITH: QEMU BOOT LAUNCHER (Windows) (v1.0)
REM =========================================================================
REM Usage: scripts\qemu_boot.bat
REM Requires: qemu-system-x86_64 in PATH, sigma_zenith.bin from `make all`
REM =========================================================================

SET KERNEL_BIN=sigma_zenith.bin
SET MEMORY=512M
SET CPUS=4
SET SERIAL_LOG=serial.log

IF NOT EXIST %KERNEL_BIN% (
    echo ERROR: %KERNEL_BIN% not found. Run 'make all' first.
    exit /b 1
)

echo [QEMU]: Booting SigmaOS Zenith Supreme...
echo   Kernel:  %KERNEL_BIN%
echo   Memory:  %MEMORY%
echo   CPUs:    %CPUS%
echo   Serial:  %SERIAL_LOG%
echo.

qemu-system-x86_64 ^
    -kernel %KERNEL_BIN% ^
    -m %MEMORY% ^
    -smp %CPUS% ^
    -serial file:%SERIAL_LOG% ^
    -monitor stdio ^
    -display sdl ^
    -no-reboot ^
    -no-shutdown

echo [QEMU]: SigmaOS session ended. Serial log: %SERIAL_LOG%
pause
