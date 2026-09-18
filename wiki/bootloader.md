# SigmaOS Bootloader Documentation

## Overview

SigmaOS uses a two-stage bootloader that supports both **UEFI** (primary) and **BIOS/MBR** (legacy) boot paths.

## Boot Flow

```
Power On
  → UEFI Firmware (EFI System Partition)
    → sigma-boot.efi (Stage 1: EFI application)
      → Load kernel ELF from SigmaFS/ext4
      → Set up GOP framebuffer
      → Parse ACPI tables
      → Build memory map
      → Set up initial page tables
      → Jump to kernel entry point (sigma_start64)
        → Kernel initialization
```

## UEFI Boot

The EFI bootloader (`kernel/bootloader/efi_boot.c`) is a standard EFI application:

1. **EFI Services** — use Boot Services to read files, query memory map
2. **Framebuffer** — initialize GOP (Graphics Output Protocol) for early output
3. **Memory Map** — call `GetMemoryMap()` before `ExitBootServices()`
4. **Kernel Load** — read `sigma-kernel.elf` from `/EFI/SigmaOS/`
5. **Hand-off** — pass boot parameters via `SigmaBootInfo` struct

### Boot Partition Layout

```
/EFI/
└── SigmaOS/
    ├── sigma-boot.efi      ← UEFI bootloader
    ├── sigma-kernel.elf    ← Kernel ELF
    ├── sigma.toml          ← Boot configuration
    └── initramfs.cpio.gz   ← Initial RAM filesystem
```

## Boot Configuration (sigma.toml)

```toml
[boot]
kernel = "/EFI/SigmaOS/sigma-kernel.elf"
initramfs = "/EFI/SigmaOS/initramfs.cpio.gz"
timeout = 5

[kernel]
cmdline = "root=/dev/sda1 rw quiet"
heap_size = "64M"
scheduler = "eevdf-bore"
kaslr = true
loglevel = 4

[[entries]]
name = "SigmaOS (default)"
kernel = "/EFI/SigmaOS/sigma-kernel.elf"

[[entries]]
name = "SigmaOS (recovery)"
kernel = "/EFI/SigmaOS/sigma-kernel-recovery.elf"
cmdline = "root=/dev/sda1 ro recovery"
```

## Secure Boot

SigmaOS supports UEFI Secure Boot:

1. The bootloader (`sigma-boot.efi`) is signed with the SigmaOS vendor key
2. The kernel ELF is signed and verified by the bootloader
3. The initramfs is verified against an embedded hash

```bash
# Enroll SigmaOS Secure Boot key
sigma-secureboot enroll /path/to/sigma-vendor.cer

# Sign the kernel
sbsign --key sigma-vendor.key --cert sigma-vendor.cer \
       --output sigma-kernel-signed.elf sigma-kernel.elf
```

## A/B Boot Slots

SigmaOS supports ChromeOS-style A/B partition updates:

```
Disk Layout:
  Partition 1: EFI System (vfat, 512MB)
  Partition 2: SigmaOS Root A (ext4/SigmaFS, 4GB)
  Partition 3: SigmaOS Root B (ext4/SigmaFS, 4GB)  ← inactive slot for updates
  Partition 4: User Data (ext4/SigmaFS, remaining)
```

The bootloader reads slot priority flags from EFI variables to choose A or B.

## GRUB Compatibility

For systems using GRUB:

```
menuentry "SigmaOS" {
    insmod part_gpt
    insmod ext2
    search --no-floppy --fs-uuid --set=root SIGMA_ROOT_UUID
    linux /sigma-kernel root=UUID=SIGMA_ROOT_UUID rw quiet
    initrd /initramfs.cpio.gz
}
```
