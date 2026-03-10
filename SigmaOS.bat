@echo off
setlocal
cd /d "%~dp0"
echo [SIGMA] Booting Sovereign Suite...
python launcher.py
if %ERRORLEVEL% NEQ 0 (
    echo [CRITICAL] SigmaOS failed to launch. 
    echo Check if Python and Tkinter are installed.
    pause
)
endlocal
