@echo off
setlocal
cd /d "%~dp0"
echo Starting SigmaOS Sovereign...
py sigma.py %*
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo SigmaOS encountered a fatal error.
    pause
)
endlocal
