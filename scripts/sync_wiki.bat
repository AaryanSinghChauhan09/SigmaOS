@echo off
REM Native Batch script for syncing markdown documentation to the Wiki Repo.
REM Bypasses PowerShell and relies on minimal CMD.exe builtins.

setlocal enabledelayedexpansion

set SOURCE_DIR=.
set WIKI_DIR=.\wiki_repo

if not exist "%WIKI_DIR%" mkdir "%WIKI_DIR%"

echo Syncing Markdown files to Wiki...

for /r "%SOURCE_DIR%" %%f in (*.md) do (
    set "filepath=%%f"
    set "filename=%%~nxf"
    
    REM Exclude wiki_repo directory
    echo !filepath! | findstr /i /c:"\wiki_repo\" >nul
    if errorlevel 1 (
        REM Replace spaces with dashes (simple mapping)
        set "destname=!filename: =-!"
        copy /y "!filepath!" "%WIKI_DIR%\!destname!" >nul
    )
)

if exist "%SOURCE_DIR%\README.md" (
    copy /y "%SOURCE_DIR%\README.md" "%WIKI_DIR%\Home.md" >nul
)

echo Wiki Sync COMPLETE.
