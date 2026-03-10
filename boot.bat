@echo off
setlocal
echo ==========================================
echo       SIGMA OS -- APEX V5.0 STARTUP
echo ==========================================
echo.
echo Running System Diagnostics...
python diagnostic.py
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] System integrity check FAILED. Please check kernel logs.
    pause
    exit /b %ERRORLEVEL%
)
echo.
echo System integrity VERIFIED. Launching Sovereign GUI...
python boot.py
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] GUI process terminated with error code %ERRORLEVEL%.
    pause
    exit /b %ERRORLEVEL%
)
echo SigmaOS offline.
pause
