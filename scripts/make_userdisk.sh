#!/usr/bin/env bash
# SigmaOS User Disk Builder — FAT12 floppy image with userland binaries.
# REPRODUCIBLE BUILDS: fixed geometry, deterministic mtools layout.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
USER_DIR="$ROOT_DIR/user"
IMG="$USER_DIR/sigmaos-userdisk.img"

for t in mformat mcopy; do
    command -v "$t" >/dev/null 2>&1 || { echo "ERROR: $t not found (install mtools)" >&2; exit 1; }
done
for f in hello.bin cat.bin rm.bin mkdir.bin touch.bin mv.bin append.bin stat.bin pwd.bin cd.bin ls.bin write.bin elfhello.elf find.elf tree.elf README.TXT; do
    [ -f "$USER_DIR/$f" ] || { echo "ERROR: missing $USER_DIR/$f — run make -C user first" >&2; exit 1; }
done

rm -f "$IMG"
mformat -i "$IMG" -C -f 1440 ::
mcopy -o -i "$IMG" "$USER_DIR/hello.bin"   ::HELLO.BIN
mcopy -o -i "$IMG" "$USER_DIR/cat.bin"     ::CAT.BIN
mcopy -o -i "$IMG" "$USER_DIR/rm.bin"      ::RM.BIN
mcopy -o -i "$IMG" "$USER_DIR/mkdir.bin"   ::MKDIR.BIN
mcopy -o -i "$IMG" "$USER_DIR/touch.bin"    ::TOUCH.BIN
mcopy -o -i "$IMG" "$USER_DIR/mv.bin"       ::MV.BIN
mcopy -o -i "$IMG" "$USER_DIR/append.bin"   ::APPEND.BIN
mcopy -o -i "$IMG" "$USER_DIR/stat.bin"     ::STAT.BIN
mcopy -o -i "$IMG" "$USER_DIR/pwd.bin"      ::PWD.BIN
mcopy -o -i "$IMG" "$USER_DIR/cd.bin"       ::CD.BIN
mcopy -o -i "$IMG" "$USER_DIR/ls.bin"       ::LS.BIN
mcopy -o -i "$IMG" "$USER_DIR/write.bin"    ::WRITE.BIN
mcopy -o -i "$IMG" "$USER_DIR/README.TXT"  ::README.TXT
mcopy -o -i "$IMG" "$USER_DIR/elfhello.elf" ::ELFHELLO.ELF
mcopy -o -i "$IMG" "$USER_DIR/find.elf"     ::FIND.ELF
mcopy -o -i "$IMG" "$USER_DIR/tree.elf"     ::TREE.ELF

echo "Built $IMG"
mdir -i "$IMG" ::
