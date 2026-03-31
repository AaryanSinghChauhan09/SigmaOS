# Σ SIGMAOS: SOVEREIGN INDUSTRIAL BUILD SCRIPT (POWERSHELL)
# v160.0 - Pure C11 Master

$ErrorActionPreference = "Stop"

$CC = "gcc"
$NASM = "nasm"
$LD = "ld"

$CFLAGS = @("-std=c11", "-Wall", "-Wextra", "-O3", "-ffreestanding", "-fno-stack-protector", "-fno-builtin", "-mno-red-zone", "-I.", "-Ilibc", "-DSIGMA_INDUSTRIAL_BUILD=1")
$ASMFLAGS = @("-f", "elf64")
$LDFLAGS = @("-nostdlib", "-static", "-e", "main")

$BUILD_DIR = "build"
$KERNEL_DIR = "kernel"
$LIBC_DIR = "libc"

if (-not (Test-Path $BUILD_DIR)) { New-Item -ItemType Directory -Path $BUILD_DIR }

Write-Host "Σ Building SigmaOS Sovereign Zenith..." -ForegroundColor Cyan

# Compile LIBC
$LIBC_SRCS = Get-ChildItem -Path $LIBC_DIR -Filter *.c
foreach ($src in $LIBC_SRCS) {
    $obj = Join-Path $BUILD_DIR ("libc_" + $src.BaseName + ".o")
    Write-Host "[LIBC-CC] $($src.Name)"
    & $CC $CFLAGS -c $src.FullName -o $obj
}

# Compile KERNEL
$KERNEL_SRCS = Get-ChildItem -Path $KERNEL_DIR -Filter *.c
foreach ($src in $KERNEL_SRCS) {
    $obj = Join-Path $BUILD_DIR ("kernel_" + $src.BaseName + ".o")
    Write-Host "[SHARD-CC] $($src.Name)"
    & $CC $CFLAGS -c $src.FullName -o $obj
}

# Compile ASM
$ASM_SRCS = Get-ChildItem -Path @($KERNEL_DIR, $LIBC_DIR) -Filter *.asm
foreach ($src in $ASM_SRCS) {
    $obj = Join-Path $BUILD_DIR ($src.BaseName + ".o")
    Write-Host "[SHARD-ASM] $($src.Name)"
    & $NASM $ASMFLAGS $src.FullName -o $obj
}

# Link
$OBJS = Get-ChildItem -Path $BUILD_DIR -Filter *.o | ForEach-Object { $_.FullName }
Write-Host "[SHARD-LD] Linking Sovereign Zenith..."
& $LD $LDFLAGS $OBJS -o (Join-Path $BUILD_DIR "sigmaos_zenith")

Write-Host "Σ SIGMAOS ZENITH BUILD SUCCESSFUL (PURE C11 SHARDED)" -ForegroundColor Green
