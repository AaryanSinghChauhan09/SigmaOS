@echo off
:: -----------------------------------------------------------------------------
:: SigmaOS Sovereign Bootstrap Shard v1.0 (Native Windows Batch)
:: Principle: Atomic Build & Execution Zenith.
:: USP: One-Click Sovereignty.
:: -----------------------------------------------------------------------------

setlocal
set SIGMA_VERSION=v73.0

echo.
echo  #####################################################################
echo  #                                                                   #
echo  #         Σ SigmaOS: Sovereign Zenith Bootstrap (%SIGMA_VERSION%)       #
echo  #                                                                   #
echo  #####################################################################
echo.

echo Σ [BOOTSTRAP]: Initiating Atomic Multi-Linguistic Build...
make all
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Σ [BOOTSTRAP_ERROR]: Build Failed. Check Makefile and Shard Purity.
    exit /b %ERRORLEVEL%
)

echo.
echo Σ [BOOTSTRAP]: Build Zeniths ACHIEVED. Initiating Sovereign Test Suite...
echo.

make test
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Σ [BOOTSTRAP_ERROR]: Test Suite Failed. Mesh Convergence Impacted.
    exit /b %ERRORLEVEL%
)

echo.
echo Σ [BOOTSTRAP]: SigmaOS %SIGMA_VERSION% is OPERATIONAL.
echo Σ [BOOTSTRAP]: Your Hardware. Your Shard-Bus. Your Matrix. Your Sovereignty.
echo.

pause
endlocal
