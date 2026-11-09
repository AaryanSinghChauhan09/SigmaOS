SigmaOS User Disk — FAT12 floppy image
======================================

Contents:
  HELLO.BIN  ring-3 test: SYS_TICKS + SYS_WRITE + SYS_EXIT
  CAT.BIN    ring-3 file reader: open/read/close via int 0x80
  RM.BIN     ring-3 file delete: SYS_UNLINK via int 0x80
  MKDIR.BIN  ring-3 dir create: SYS_MKDIR via int 0x80
  TOUCH.BIN  ring-3 file create: SYS_CREAT via int 0x80
  ELFHELLO.ELF  ELF32 C program (gcc -m32 -nostdlib), exec ELF path
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
  exec ELFHELLO.ELF
  write TEST.TXT hello && append TEST.TXT world
  cat TEST.TXT && hex TEST.TXT && stat TEST.TXT
  exec RM.BIN TEST.TXT
  exec MKDIR.BIN MYDIR
  exec TOUCH.BIN NEWFILE.TXT

Rebuild this image:  scripts/make_userdisk.sh
