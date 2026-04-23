#!/bin/bash
# SigmaOS: Sovereign Build Orchestrator (v10.0 - Pure Silicon) - Linux/macOS
# Translated from PowerShell for cross-platform supremacy.

GCC="gcc"
NASM="nasm"
LD="ld"
BUILD_DIR="build"

echo "Σ [BUILD] Initiating Sovereign Build v23.0 (Absolute Finality)..."

mkdir -p $BUILD_DIR

# Flags
COMMON_FLAGS="-ffreestanding -nostdlib -fno-stack-protector -mno-red-zone -O2 -Wall -Wextra"
INCLUDES="-I. -Icore/lattice/include"

OBJS=()

# Find Sources (Suites + Core Lattice)
CSRCS=$(find suites core/lattice -name "*.c")
ASMSRCS=$(find suites core/lattice -name "*.asm")

# Compile ASM
for File in $ASMSRCS; do
    BaseName=$(basename "$File" .asm)
    Obj="$BUILD_DIR/$BaseName.o"
    echo "  Σ [ASM] $(basename "$File")..."
    $NASM -f elf64 "$File" -o "$Obj"
    OBJS+=("$Obj")
done

# Compile C
for File in $CSRCS; do
    BaseName=$(basename "$File" .c)
    Obj="$BUILD_DIR/$BaseName.o"
    echo "  Σ [CC]  $(basename "$File")..."
    $GCC -std=c11 $COMMON_FLAGS $INCLUDES -c "$File" -o "$Obj"
    OBJS+=("$Obj")
done

# Link
echo "Σ [LD] Linking Sovereign Lattice..."
LDFLAGS="-nostdlib -static -T suites/S01_Genesis/shards/sigma.ld"
$LD $LDFLAGS "${OBJS[@]}" -o "$BUILD_DIR/sigmaos_zenith"

echo -e "\nΣ [OK] Sovereign Build COMPLETE: $BUILD_DIR/sigmaos_zenith"
