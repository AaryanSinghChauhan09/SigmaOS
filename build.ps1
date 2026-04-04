# Σ SIGMAOS: SOVEREIGN INDUSTRIAL BUILD SCRIPT (POWERSHELL)
# v165.0 - Professional C11 Zenith

$ErrorActionPreference = "Stop"

$CC = "gcc"
$NASM = "nasm"
$LD = "ld"

function Check-Tool($tool) {
    if (Get-Command $tool -ErrorAction SilentlyContinue) {
        return $true
    }
    return $false
}

Write-Host "Σ Initiating Sovereign Build Sequence..." -ForegroundColor Cyan

# Check for required tools
$missing = @()
if (-not (Check-Tool $CC)) { $missing += $CC }
if (-not (Check-Tool $NASM)) { $missing += $NASM }
if (-not (Check-Tool $LD)) { $missing += $LD }

if ($missing.Count -gt 0) {
    Write-Host "[!] ERROR: Missing build tools: $($missing -join ', ')" -ForegroundColor Red
    Write-Host "[!] Please ensure MinGW-w64 and NASM are installed and in your PATH." -ForegroundColor Yellow
    Write-Host "[!] You can install them via winget:" -ForegroundColor White
    Write-Host "    winget install NASM.NASM" -ForegroundColor White
    Write-Host "    winget install msys2.msys2 (then pacman -S mingw-w64-x86_64-gcc)" -ForegroundColor White
    exit 1
}

$CFLAGS = @("-std=c11", "-Wall", "-Wextra", "-O3", "-ffreestanding", "-fno-stack-protector", "-fno-builtin", "-mno-red-zone", "-I.", "-Ilibc", "-DSIGMA_INDUSTRIAL_BUILD=1")
$ASMFLAGS = @("-f", "elf64")
$LDFLAGS = @("-nostdlib", "-static", "-e", "kmain")

$BUILD_DIR = "build"
$KERNEL_DIR = "kernel"
$LIBC_DIR = "libc"
$TOOLS_DIR = "sovereign_tools"

if (-not (Test-Path $BUILD_DIR)) { New-Item -ItemType Directory -Path $BUILD_DIR }

# Compile LIBC
Write-Host "[*] Auditing LibC Shards..." -ForegroundColor Cyan
$LIBC_SRCS = Get-ChildItem -Path $LIBC_DIR -Include @("*.c", "*.asm") -Recurse
foreach ($src in $LIBC_SRCS) {
    $obj = Join-Path $BUILD_DIR ("libc_" + $src.BaseName + ".o")
    if ($src.Extension -eq ".c") {
        Write-Host "[LIBC-CC] $($src.Name)"
        & $CC $CFLAGS -c $src.FullName -o $obj
    } else {
        Write-Host "[LIBC-ASM] $($src.Name)"
        & $NASM $ASMFLAGS $src.FullName -o $obj
    }
}

# Compile KERNEL
Write-Host "[*] Sharding Sovereign Kernel..." -ForegroundColor Cyan
$KERNEL_SRCS = Get-ChildItem -Path $KERNEL_DIR -Include @("*.c", "*.asm") -Recurse
foreach ($src in $KERNEL_SRCS) {
    $obj = Join-Path $BUILD_DIR ("kernel_" + $src.BaseName + ".o")
    if ($src.Extension -eq ".c") {
        Write-Host "[SHARD-CC] $($src.Name)"
        & $CC $CFLAGS -c $src.FullName -o $obj
    } else {
        Write-Host "[SHARD-ASM] $($src.Name)"
        & $NASM $ASMFLAGS $src.FullName -o $obj
    }
}

# Compile SOVEREIGN TOOLS
Write-Host "[*] Sharding Sovereign Tools..." -ForegroundColor Cyan
if (Test-Path $TOOLS_DIR) {
    $TOOLS_SRCS = Get-ChildItem -Path $TOOLS_DIR -Filter *.c
    foreach ($src in $TOOLS_SRCS) {
        $obj = Join-Path $BUILD_DIR ("tool_" + $src.BaseName + ".o")
        Write-Host "[TOOL-CC] $($src.Name)"
        & $CC $CFLAGS -c $src.FullName -o $obj
    }
}

# Link
$OBJS = Get-ChildItem -Path $BUILD_DIR -Filter *.o | ForEach-Object { $_.FullName }
Write-Host "[SHARD-LD] Linking Sovereign Zenith..." -ForegroundColor Cyan
& $LD $LDFLAGS $OBJS -o (Join-Path $BUILD_DIR "sigmaos_zenith")

Write-Host "Σ SIGMAOS ZENITH BUILD SUCCESSFUL (PURE C11 SHARDED)" -ForegroundColor Green
