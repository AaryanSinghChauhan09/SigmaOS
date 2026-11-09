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
for f in hello.bin cat.bin rm.bin mkdir.bin touch.bin mv.bin append.bin stat.bin pwd.bin cd.bin ls.bin write.bin elfhello.elf find.elf tree.elf wc.elf grep.elf sort.elf head.elf uniq.elf nl.elf tail.elf rev.elf cut.elf tr.elf fold.elf tac.elf expand.elf seq.elf factor.elf expr.elf yes.elf printf.elf basename.elf dirname.elf test.elf realpath.elf ll.elf stat.elf df.elf uname.elf mkdir.elf rm.elf mv.elf du.elf ln.elf rmdir.elf cp.elf true.elf false.elf uptime.elf hexdump.elf README.TXT; do
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
mcopy -o -i "$IMG" "$USER_DIR/wc.elf"       ::WC.ELF
mcopy -o -i "$IMG" "$USER_DIR/grep.elf"     ::GREP.ELF
mcopy -o -i "$IMG" "$USER_DIR/sort.elf"     ::SORT.ELF
mcopy -o -i "$IMG" "$USER_DIR/head.elf"     ::HEAD.ELF
mcopy -o -i "$IMG" "$USER_DIR/uniq.elf"     ::UNIQ.ELF
mcopy -o -i "$IMG" "$USER_DIR/nl.elf"       ::NL.ELF
mcopy -o -i "$IMG" "$USER_DIR/tail.elf"     ::TAIL.ELF
mcopy -o -i "$IMG" "$USER_DIR/rev.elf"     ::REV.ELF
mcopy -o -i "$IMG" "$USER_DIR/cut.elf"     ::CUT.ELF
mcopy -o -i "$IMG" "$USER_DIR/tr.elf"       ::TR.ELF
mcopy -o -i "$IMG" "$USER_DIR/fold.elf"     ::FOLD.ELF
mcopy -o -i "$IMG" "$USER_DIR/tac.elf"     ::TAC.ELF
mcopy -o -i "$IMG" "$USER_DIR/expand.elf"   ::EXPAND.ELF
mcopy -o -i "$IMG" "$USER_DIR/seq.elf"     ::SEQ.ELF
mcopy -o -i "$IMG" "$USER_DIR/factor.elf"   ::FACTOR.ELF
mcopy -o -i "$IMG" "$USER_DIR/expr.elf"     ::EXPR.ELF
mcopy -o -i "$IMG" "$USER_DIR/yes.elf"     ::YES.ELF
mcopy -o -i "$IMG" "$USER_DIR/printf.elf"  ::PRINTF.ELF
mcopy -o -i "$IMG" "$USER_DIR/basename.elf" ::BASENAME.ELF
mcopy -o -i "$IMG" "$USER_DIR/dirname.elf"  ::DIRNAME.ELF
mcopy -o -i "$IMG" "$USER_DIR/test.elf"     ::TEST.ELF
mcopy -o -i "$IMG" "$USER_DIR/realpath.elf" ::REALPATH.ELF
mcopy -o -i "$IMG" "$USER_DIR/ll.elf"      ::LL.ELF
mcopy -o -i "$IMG" "$USER_DIR/stat.elf"    ::STAT.ELF
mcopy -o -i "$IMG" "$USER_DIR/df.elf"      ::DF.ELF
mcopy -o -i "$IMG" "$USER_DIR/uname.elf"   ::UNAME.ELF
mcopy -o -i "$IMG" "$USER_DIR/mkdir.elf"   ::MKDIR.ELF
mcopy -o -i "$IMG" "$USER_DIR/rm.elf"      ::RM.ELF
mcopy -o -i "$IMG" "$USER_DIR/mv.elf"      ::MV.ELF
mcopy -o -i "$IMG" "$USER_DIR/du.elf"      ::DU.ELF
mcopy -o -i "$IMG" "$USER_DIR/ln.elf"      ::LN.ELF
mcopy -o -i "$IMG" "$USER_DIR/rmdir.elf"   ::RMDIR.ELF
mcopy -o -i "$IMG" "$USER_DIR/cp.elf"      ::CP.ELF
mcopy -o -i "$IMG" "$USER_DIR/true.elf"    ::TRUE.ELF
mcopy -o -i "$IMG" "$USER_DIR/false.elf"   ::FALSE.ELF
mcopy -o -i "$IMG" "$USER_DIR/uptime.elf"  ::UPTIME.ELF
mcopy -o -i "$IMG" "$USER_DIR/hexdump.elf" ::HEXDUMP.ELF
mcopy -o -i "$IMG" "$USER_DIR/ln.elf"      ::LN.ELF
mcopy -o -i "$IMG" "$USER_DIR/rmdir.elf"   ::RMDIR.ELF
mcopy -o -i "$IMG" "$USER_DIR/cp.elf"      ::CP.ELF
mcopy -o -i "$IMG" "$USER_DIR/true.elf"    ::TRUE.ELF
mcopy -o -i "$IMG" "$USER_DIR/false.elf"   ::FALSE.ELF
mcopy -o -i "$IMG" "$USER_DIR/uptime.elf"  ::UPTIME.ELF
mcopy -o -i "$IMG" "$USER_DIR/hexdump.elf" ::HEXDUMP.ELF

echo "Built $IMG"
mdir -i "$IMG" ::
