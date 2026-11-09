SigmaOS User Disk — FAT12 floppy image
======================================

Contents:
  HELLO.BIN  ring-3 test: SYS_TICKS + SYS_WRITE + SYS_EXIT
  CAT.BIN    ring-3 file reader: open/read/close via int 0x80
  README.TXT this file

Boot the ISO in QEMU and attach this image as the first hard disk:

  qemu-system-x86_64 -m 256M \
      -cdrom build/sigmaos-desktop-preview.iso \
      -drive format=raw,file=user/sigmaos-userdisk.img,if=ide,index=0 \
      -serial stdio

The kernel's ATA driver scans 0x1F0 (primary master) — the image appears
as the FAT volume. Then in the shell:

  fs            detect filesystem
  ls            list files (LFN-aware)
  df            free space
  cat README.TXT
  exec HELLO.BIN
  exec CAT.BIN README.TXT

Rebuild this image:  scripts/make_userdisk.sh
