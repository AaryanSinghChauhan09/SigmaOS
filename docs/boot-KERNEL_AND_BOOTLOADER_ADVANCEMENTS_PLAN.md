# Kernel & Bootloader Advancements Development Plan

## Overview
This document specifies the architecture and implementation roadmap for SigmaOS Kernel and Bootloader Advancements, drawing direct inspiration from Linux and BSD operating systems (FreeBSD, OpenBSD, NetBSD, DragonFly BSD).

## Architecture & Subsystem Coverage

### 1. Multi-Bootloader Interoperability Engine
Supports parsing, multi-format cross-translation, and configuration generation across leading bootloaders:
- **GRUB2**: `grub.cfg` parsing, `menuentry`, `insmod`, kernel parameters, initrd staging.
- **systemd-boot**: Boot Loader Specification (BLS) `.conf` files (`title`, `linux`, `initrd`, `options`).
- **Syslinux / ISOLINUX**: `syslinux.cfg` / `isolinux.cfg` directives (`DEFAULT`, `LABEL`, `KERNEL`, `APPEND`, `INITRD`).
- **LILO**: Legacy LILO `lilo.conf` directives (`boot`, `prompt`, `image`, `label`, `initrd`).
- **BSD Bootloader (`loader.conf` & `boot.conf`)**: FreeBSD `/boot/loader.conf` (`kernel`, `zfs_load`, modules) and OpenBSD `/etc/boot.conf` (`boot`, `set timeout`, `stty`).
- **BSD Boot0 / Boot1**: MBR stage 0 slice selector and stage 1 BSD filesystem kernel loader.
- **rEFInd**: `refind.conf` stanza parser (`menuentry`, `icon`, `loader`, `initrd`, `options`).
- **Limine**: `limine.conf` entry parser (`PROTOCOL`, `KERNEL_PATH`, `CMDLINE`, `MODULE_PATH`).

### 2. UEFI / BIOS Firmware & GOP Framebuffer Engine
- **UEFI GOP (Graphics Output Protocol)**: Resolution negotiation across 4K, 2K, FHD, Ultrawide; pixel format conversion (XRGB8888, BGRX8888); linear physical framebuffer handoff.
- **Firmware Tables**: ACPI RSDP/XSDT table parsing, SMBIOS Type 0 (BIOS), Type 1 (System), Type 2 (Baseboard) parsing.
- **EFI Variables**: Non-volatile runtime variable store (`GetVariable`, `SetVariable`).
- **Legacy BIOS Handoff**: VBE (VESA BIOS Extensions) mode switching and real-mode interrupt handoff.

### 3. Flattened Device Tree (FDT) & Overlay Engine
- **DTB Header Parsing**: Magic (`0xd00dfeed`), version, off_dt_struct, off_dt_strings, totalsize.
- **Tree Traversal**: Property inspection (`compatible`, `reg`, `interrupts`), node hierarchy resolution.
- **Device Tree Overlays (`.dtbo`)**: Dynamic runtime device tree patching, base DTB merging, fragment overlay application.

### 4. Kexec Fast Kernel Boot & Crashdump Engine
- **Kexec Kernel Relocation**: Staging target Linux/BSD kernel binary, initrd, and FDT in designated memory segments.
- **Memory Segment Verification**: Range checking, overlap detection, page alignment.
- **Kexec Execution**: Direct jump into target kernel entry point bypassing BIOS/UEFI reboot delays.
- **Crashdump Staging**: KDUMP / kexec crash memory region reservation for kernel panic capture.

### 5. Kernel Modules & Firmware Blob Manager
- **Module Loader**: ELF `.ko` (Linux) and `.kmod` (BSD) module parser, symbol lookup, relocation (`R_X86_64_64`, `R_X86_64_PC32`), dependency graph resolution.
- **Firmware Blob Store**: Vendor firmware binary loading from `/lib/firmware`, firmware blob header validation, GPU GSP firmware staging.
- **Driver Binding**: Automated driver-to-device matching based on PCI IDs, USB IDs, or FDT `compatible` strings.

### 6. Initramfs / Initrd Staging Engine
- **CPIO Archive Support**: CPIO `newc` format header creation, file entry unpacking, directory tree construction.
- **Multi-Algorithm Compression**: Zstd, LZ4, XZ, Gzip compression framing and decompression.
- **Early Microcode Prepender**: Early CPU microcode CPIO archive prepending (`early_cpio` for Intel/AMD microcode updates).

## Verification & Parity Matrix
Fully validated via native Rust unit tests and integrated into `./run_sigma_tests.sh`.
