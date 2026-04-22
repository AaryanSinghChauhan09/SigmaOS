# build_sovereign.ps1
# SigmaOS: Sovereign Build Orchestrator (v10.0 - Pure Silicon)

$GCC = "C:\msys64\mingw64\bin\gcc.exe"
$NASM = "nasm"
$LD = "ld"
$BUILD_DIR = "build"

Write-Host "Σ [BUILD] Initiating Sovereign Build v10.0 (Pure Silicon)..." -ForegroundColor Cyan

if (!(Test-Path $BUILD_DIR)) { New-Item -ItemType Directory -Path $BUILD_DIR }

# Flags
$COMMON_FLAGS = "-ffreestanding", "-nostdlib", "-fno-stack-protector", "-mno-red-zone", "-O2", "-Wall", "-Wextra"
$INCLUDES = "-I.", "-Icore/include"

# Find Sources
$CSRCS = Get-ChildItem -Path suites -Filter *.c -Recurse
$ASMSRCS = Get-ChildItem -Path suites -Filter *.asm -Recurse

$OBJS = @()

# Compile ASM
foreach ($File in $ASMSRCS) {
    $Obj = "$BUILD_DIR/$($File.BaseName).o"
    Write-Host "  Σ [ASM] $($File.Name)..." -ForegroundColor Gray
    nasm -f elf64 $File.FullName -o $Obj
    $OBJS += $Obj
}

# Compile C
foreach ($File in $CSRCS) {
    $Obj = "$BUILD_DIR/$($File.BaseName).o"
    Write-Host "  Σ [CC]  $($File.Name)..." -ForegroundColor Gray
    & $GCC -std=c11 $COMMON_FLAGS $INCLUDES -c $File.FullName -o $Obj
    $OBJS += $Obj
}

# Link
Write-Host "Σ [LD] Linking Sovereign Lattice..." -ForegroundColor Green
$LDFLAGS = "-nostdlib", "-static", "-T", "suites/S01_Genesis/shards/sigma.ld"
& $LD $LDFLAGS $OBJS -o "$BUILD_DIR/sigmaos_zenith"

Write-Host "`nΣ [OK] Sovereign Build COMPLETE: $BUILD_DIR/sigmaos_zenith" -ForegroundColor Green
