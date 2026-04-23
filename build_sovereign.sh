#!/bin/bash
# SigmaOS: Sovereign Build Orchestrator (v24.0 - Hardened Finality)
# Optimized for absolute architectural parity and zero-latency synthesis.

GCC="g++"
NASM="nasm"
LD="ld"
BUILD_DIR="build"

echo "Σ [BUILD] Initiating Sovereign Build v24.0 (Hyper-Synthesis)..."

mkdir -p $BUILD_DIR

# 1. Ensure core directories exist
mkdir -p core/lattice/include
mkdir -p suites/include

# 2. Build Include Path (Recursive)
INCLUDES="-I. -Iinclude -Icore/lattice/include -Isuites/S01_Genesis/include -Isuites/S30_Supremacy"
# Automatically find all directories with headers
HEADER_DIRS=$(find suites core cli userland -type d 2>/dev/null)
for dir in $HEADER_DIRS; do
    INCLUDES="$INCLUDES -I$dir"
done

# 3. Compiler Flags
COMMON_FLAGS="-m64 -ffreestanding -nostdlib -fno-stack-protector -mno-red-zone -O2 -Wall -Wextra"
CFLAGS="-std=c++20 -fno-exceptions -fno-rtti $COMMON_FLAGS"
ASMFLAGS="-f elf64"

OBJS=()

# 4. Compile ASM
ASMSRCS=$(find suites core cli userland -name "*.asm" 2>/dev/null)
for File in $ASMSRCS; do
    # Create unique object name to avoid collision
    ObjName=$(echo "$File" | tr '/' '_').o
    Obj="$BUILD_DIR/$ObjName"
    echo "  Σ [ASM] $File -> $Obj"
    $NASM $ASMFLAGS "$File" -o "$Obj"
    OBJS+=("$Obj")
done

# 5. Compile C/C++ (All treated as C++ for Sovereign Parity)
CSRCS=$(find suites core cli userland -name "*.c" -o -name "*.cpp" 2>/dev/null)
for File in $CSRCS; do
    ObjName=$(echo "$File" | tr '/' '_').o
    Obj="$BUILD_DIR/$ObjName"
    echo "  Σ [CC]  $File -> $Obj"
    $GCC $CFLAGS $INCLUDES -c "$File" -o "$Obj"
    OBJS+=("$Obj")
done

# 6. Link
echo "Σ [LD] Linking Sovereign Lattice (641 Shards)..."
LDFLAGS="-nostdlib -static -T suites/S01_Genesis/shards/sigma.ld"
$LD $LDFLAGS "${OBJS[@]}" -o "$BUILD_DIR/sigmaos_zenith"

if [ $? -eq 0 ]; then
    echo -e "\nΣ [OK] Sovereign Build COMPLETE: $BUILD_DIR/sigmaos_zenith"
else
    echo -e "\nΣ [FAIL] Sovereign Synthesis Interrupted."
    exit 1
fi

