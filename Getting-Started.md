# Getting Started

Everything you need to build SigmaOS from source and test it in QEMU.

---

## Quick Start (5 minutes)

```bash

# Clone

git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Install Rust nightly (if not already)

rustup show   # confirms rust-toolchain.toml toolchain is active

# Build the kernel

cd kernel && cargo build --release

# Run in QEMU

cd .. && ./qemu-boot.sh standalone
```

Expected serial output:
```
Σ SigmaOS Zenith Kernel Initializing (Rust)
[IRQ] PIC remapped, PIT 1000Hz, IDT ready
[MEM] Slab memory manager initialized
[init] PID 1 running
System Ready. Waiting for input...
```

---

## Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| Rust nightly | see `rust-toolchain.toml` | Kernel + userland |
| NASM | ≥ 2.15 | x86 assembly |
| Zig | 0.13.0 | Bootloader + HAL |
| QEMU | ≥ 8.0 | Testing |
| OVMF | any | UEFI testing |

Install on Ubuntu/Debian:
```bash
apt install nasm qemu-system-x86 ovmf
```

Install on Windows:
```powershell
winget install NASM.NASM QEMU.QEMU
```

---

## Build Targets

```bash

# Kernel (no_std Rust)

cd kernel && cargo build --release

# All workspace crates

cargo build --release --workspace

# UEFI bootloader

cd sigma-boot && zig build -Dtarget=x86_64-uefi

# Shell

cd sigma-sh && cargo build --release

# Core utilities

cd userland/coreutils && cargo build --release

# Driver SDK

cd sdk/driver && cargo build --release
```

---

## UEFI Boot (Full Boot Path)

```bash

# Set up EFI System Partition

mkdir -p esp/EFI/BOOT esp/boot
cp sigma-boot/zig-out/bin/sigma-boot.efi esp/EFI/BOOT/BOOTX64.EFI
cp kernel/target/x86_64-sigmaos/release/sigma-kernel esp/boot/sigma-kernel.elf

# Boot with OVMF

qemu-system-x86_64 \
  -bios /usr/share/OVMF/OVMF.fd \
  -drive format=raw,file=fat:rw:esp \
  -serial stdio -m 256M -nographic
```

---

## Project Layout

```
SigmaOS/
├── arch/x86_64/       → CPU entry, GDT, IDT, context switch (NASM)
├── sigma-boot/        → UEFI bootloader (Zig)
├── kernel/            → Core kernel (Rust #![no_std])
│   ├── core/          → scheduler, MM, process, IPC, PCI, ACPI
│   ├── net/           → IP, TCP, UDP, sockets
│   ├── fs/            → VFS, tmpfs, ext4, procfs
│   └── security/      → pledge, capabilities
├── drivers/           → Hardware drivers
├── kabi/              → Stable ABI library
├── sdk/driver/        → Userspace driver SDK
├── sigma-sh/          → Interactive shell
├── userland/          → Shell, coreutils, daemons
├── sigmad/            → System daemons (updater, health, metrics)
├── virtualization/    → OCI container runtime
└── wiki_repo/         → This wiki
```

---

## First Contribution

1. Check [12-Week-Milestone-Plan](12-Week-Milestone-Plan) for current priorities

2. Look at [GITHUB_ISSUES.md](../docs/GITHUB_ISSUES.md) for open tasks

3. Read [Linux-Parity-Roadmap](Linux-Parity-Roadmap) for what needs implementing

4. Follow [Kernel Developer Handbook](../docs/KERNEL_DEVELOPER_HANDBOOK.md)

---

## Common Fixes

| Error | Fix |
|-------|-----|
| `can't find crate for std` | Normal for `#![no_std]` kernel crates |
| `rust-lld not found` | `rustup component add llvm-tools-preview` |
| QEMU no serial output | Add `-serial stdio -nographic` |
| QEMU `No bootable device` | Ensure `esp/EFI/BOOT/BOOTX64.EFI` exists |

---

*See also: [Architecture Overview](Architecture-Overview) · [Kernel Developer Handbook](../docs/KERNEL_DEVELOPER_HANDBOOK.md) · [Contributing Drivers](../docs/CONTRIBUTING_DRIVERS.md)*
