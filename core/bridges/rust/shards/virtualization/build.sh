#!/usr/bin/env bash
# shards/virtualization/build.sh — Minimal shard build script
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$DIR/../.." && pwd)"
OUT="$ROOT/build/virt"
mkdir -p "$OUT"

CC="${CC:-x86_64-linux-gnu-gcc}"
CFLAGS="-std=c11 -O2 -Wall -Wextra -ffreestanding -nostdlib -I$ROOT/include"

echo "[VIRT] Compiling vm.c..."
$CC $CFLAGS -c "$DIR/vm.c" -o "$OUT/vm.o"

echo "[VIRT] Compiling Rust memory.rs (if rustc available)..."
if command -v rustc &>/dev/null; then
    rustc --edition 2021 -C opt-level=2 --crate-type=staticlib \
          --emit=obj "$DIR/memory.rs" -o "$OUT/memory.o" 2>/dev/null && \
    echo "[VIRT] memory.rs: OK" || echo "[VIRT] memory.rs: skipped (bare-metal target)"
fi

echo "[VIRT] Build complete. Objects in $OUT"
