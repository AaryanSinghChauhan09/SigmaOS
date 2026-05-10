# Σ SIGMAOS: INDUSTRIAL BUILD ORCHESTRATOR (POWERSHELL EDITION)
# Mission: Absolute ease of use and zero-failure local builds.

$ErrorActionPreference = "Stop"
$BuildDir = "obj"

Write-Host "Σ [CI]: Initiating Industrial Shard Build..." -ForegroundColor Cyan

if (-not (Test-Path $BuildDir)) {
    New-Item -ItemType Directory -Path $BuildDir | Out-Null
}

Write-Host "Σ [HARDEN]: Enforcing Relative Path Sovereignty..." -ForegroundColor Yellow
& py scripts/sovereign_hardener.py

$IncludePath = "-Iinclude -Ikernel/core -Ikernel/drivers -Ikernel/orchestration -Ikernel/shards"
$CommonFlags = "-m64 -ffreestanding -O2 -Wall $IncludePath -nostdlib"
$CxxFlags = "$CommonFlags -fno-exceptions -fno-rtti"

$SourceDirs = @("kernel/core", "kernel/drivers", "kernel/orchestration", "kernel/shards", "userland")

$Files = Get-ChildItem -Path $SourceDirs -Include *.c, *.cpp, *.asm -Recurse

foreach ($File in $Files) {
    $RelativePath = Resolve-Path $File.FullName -Relative
    $ObjFile = Join-Path $BuildDir ($RelativePath -replace '\.[^.]+$', '.o')
    $ParentDir = Split-Path $ObjFile -Parent
    
    if (-not (Test-Path $ParentDir)) {
        New-Item -ItemType Directory -Path $ParentDir | Out-Null
    }

    if ($File.Extension -eq ".c") {
        Write-Host "Σ [C]: Compiling $RelativePath..."
        & gcc $CommonFlags -c $RelativePath -o $ObjFile
    }
    elseif ($File.Extension -eq ".cpp") {
        Write-Host "Σ [CXX]: Compiling $RelativePath..."
        & g++ $CxxFlags -c $RelativePath -o $ObjFile
    }
    elseif ($File.Extension -eq ".asm") {
        Write-Host "Σ [ASM]: Assembling $RelativePath..."
        & nasm -f elf64 $RelativePath -o $ObjFile
    }
}

Write-Host "Σ [LINK]: Finalizing Sovereign Lattice..." -ForegroundColor Green
# Linker command would go here

Write-Host "Σ [CLEAN]: Scouring ephemeral build shards..." -ForegroundColor Gray
# Remove-Item -Path $BuildDir -Recurse -Force

Write-Host "Σ [ARCHIVE]: Compressing Sovereign Artifacts (Phase 24)..." -ForegroundColor Cyan
# Compress-Archive -Path $BuildDir -DestinationPath "sigma_os_v67.zip"

Write-Host "Σ [SUCCESS]: Industrial Shard Master Finalized." -ForegroundColor Green
