# build_sovereign.ps1
# SigmaOS: Sovereign Build Orchestrator (v10.0 - Pure Silicon)

$GCC = "g++"
$NASM = "nasm"
$LD = "ld"
$BUILD_DIR = "build"

Write-Host "Σ [BUILD] Initiating Sovereign Build v24.0 (Hyper-Synthesis)..." -ForegroundColor Cyan

if (!(Test-Path $BUILD_DIR)) { New-Item -ItemType Directory -Path $BUILD_DIR }

# 1. Ensure core directories exist
if (!(Test-Path "core/lattice/include")) { New-Item -ItemType Directory -Path "core/lattice/include" -Force }
if (!(Test-Path "suites/include")) { New-Item -ItemType Directory -Path "suites/include" -Force }

# 2. Build Include Path (Recursive)
$INCLUDES = @("-I.", "-Iinclude", "-Icore/lattice/include", "-Isuites/S01_Genesis/include", "-Isuites/S30_Supremacy")
$HEADER_DIRS = Get-ChildItem -Path suites, core, cli, userland -Directory -Recurse -ErrorAction SilentlyContinue
foreach ($dir in $HEADER_DIRS) {
    $INCLUDES += "-I$($dir.FullName)"
}

# 3. Compiler Flags
$COMMON_FLAGS = "-m64", "-ffreestanding", "-nostdlib", "-fno-stack-protector", "-mno-red-zone", "-O2", "-Wall", "-Wextra"
$CFLAGS = "-std=c++20", "-fno-exceptions", "-fno-rtti"
$ASMFLAGS = "-f", "elf64"

$OBJS = @()

# 4. Compile ASM
$ASMSRCS = Get-ChildItem -Path suites, core, cli, userland -Filter *.asm -Recurse -ErrorAction SilentlyContinue
foreach ($File in $ASMSRCS) {
    $ObjName = $File.FullName.Replace(":", "").Replace("\", "_").Replace("/", "_") + ".o"
    $Obj = "$BUILD_DIR/$ObjName"
    Write-Host "  Σ [ASM] $($File.FullName) -> $Obj" -ForegroundColor Gray
    & $NASM $ASMFLAGS $File.FullName -o $Obj
    $OBJS += $Obj
}

# 5. Compile C/C++
$CSRCS = Get-ChildItem -Path suites, core, cli, userland -Filter *.c -Recurse -ErrorAction SilentlyContinue
$CPPSRCS = Get-ChildItem -Path suites, core, cli, userland -Filter *.cpp -Recurse -ErrorAction SilentlyContinue
foreach ($File in ($CSRCS + $CPPSRCS)) {
    $ObjName = $File.FullName.Replace(":", "").Replace("\", "_").Replace("/", "_") + ".o"
    $Obj = "$BUILD_DIR/$ObjName"
    Write-Host "  Σ [CC]  $($File.FullName) -> $Obj" -ForegroundColor Gray
    & $GCC $CFLAGS $COMMON_FLAGS $INCLUDES -c $File.FullName -o $Obj
    $OBJS += $Obj
}

# 6. Link
Write-Host "Σ [LD] Linking Sovereign Lattice (641 Shards)..." -ForegroundColor Green
$LDFLAGS = "-nostdlib", "-static", "-T", "suites/S01_Genesis/shards/sigma.ld"
& $LD $LDFLAGS $OBJS -o "$BUILD_DIR/sigmaos_zenith"

if ($LASTEXITCODE -eq 0) {
    Write-Host "`nΣ [OK] Sovereign Build COMPLETE: $BUILD_DIR/sigmaos_zenith" -ForegroundColor Green
} else {
    Write-Host "`nΣ [FAIL] Sovereign Synthesis Interrupted." -ForegroundColor Red
    exit 1
}

