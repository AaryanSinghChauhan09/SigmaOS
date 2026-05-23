# SigmaOS

> **Zero-Dependency. Zero-Compromise. Sovereign Silicon.**

SigmaOS is an ambitious architectural leap in operating system design. It entirely disregards POSIX standards and `libc` reliance in favor of absolute computational sovereignty. Every driver, file system, and utility is built from scratch to guarantee transparency, security, and deterministic performance.

## Architecture Overview

SigmaOS utilizes a microkernel design combined with a unified userland shell (`sigma-sh`).

- **Bootloader**: Custom `sigma_secure_boot` ensuring only cryptographically verified binaries execute.
- **Kernel Init**: Minimal footprint handling hardware interrupts, PCI enumeration, and CPU initialization.
- **Memory Setup**: Sovereign buddy-system allocator (`sigma_allocator`).
- **Scheduler**: Multi-branch support (e.g., Round Robin in `main`, deterministic EDF in `release/rtos`).
- **Syscall Layer**: Distinct non-POSIX ABI to prevent external pollution.

## Sovereign Components

- **Drivers**: Keyboard (PS/2), VGA Framebuffer, Storage (ATA/SATA/VirtIO), Network (e1000).
- **File Systems**: Native sovereign implementations of FAT32 and Ext2.
- **Tools**: Complete sovereign replacements for `ls`, `cat`, `awk`, `sed`, `tar`, and a minimal text-mode HTML browser.

## Build Instructions

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Build the main standalone image
make PROFILE=standalone iso

# Boot in QEMU
./qemu-boot.sh standalone
```

## Branch Strategy
SigmaOS maintains 19 distinct branches targeting various industrial applications. Check the `docs/ROADMAP.md` for our deployment timeline, or review the GitHub Wiki for per-branch guides.
