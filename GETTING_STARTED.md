# Getting Started with SigmaOS Development

Everything you need to build, run, and contribute to SigmaOS from scratch.

---

## Prerequisites

### Required Tools

```bash

# Rust nightly toolchain (pinned in rust-toolchain.toml)

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup show   # should show nightly-2026-06-01 or later

# NASM (x86 assembler)

# Ubuntu/Debian:

apt install nasm

# Windows:

winget install NASM.NASM

# Zig (for HAL, bootloader, display drivers)

# https://ziglang.org/download/  — download 0.13.0

# QEMU (for testing)

# Ubuntu:   apt install qemu-system-x86

# Windows:  winget install QEMU.QEMU

# OVMF (UEFI firmware for QEMU)

# Ubuntu:   apt install ovmf

# Arch:     pacman -S edk2-ovmf

```

### Optional but Recommended

```bash

# cargo-bloat (binary size analysis)

cargo install cargo-bloat

# sigma-driver-porter (AI driver porting tool)

cd drivers/sigma && nim c sigma_driver_ai_porter.nim

# sigma-ddk-vendors (vendor registry)

cd drivers/ddk && nim c sigma_vendor_registry.nim
```

---

## Building the Kernel

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Build the kernel (no_std Rust)

cd kernel
cargo build --release

# Build all workspace crates

cd ..
cargo build --release --workspace 2>/dev/null || true

# Build sigma-sh (userspace shell)

cd sigma-sh
cargo build --release
```

---

## Building the UEFI Bootloader

```bash
cd sigma-boot
zig build -Dtarget=x86_64-uefi

# Output: sigma-boot/zig-out/bin/sigma-boot.efi

```

---

## Running in QEMU

### Quick test (multiboot2 / legacy BIOS)

```bash
./qemu-boot.sh standalone
```

### UEFI boot (recommended)

```bash

# Create EFI System Partition layout

mkdir -p esp/EFI/BOOT esp/boot
cp sigma-boot/zig-out/bin/sigma-boot.efi esp/EFI/BOOT/BOOTX64.EFI
cp kernel/target/x86_64-sigmaos/release/sigma-kernel esp/boot/sigma-kernel.elf

# Run with OVMF

qemu-system-x86_64 \
  -bios /usr/share/OVMF/OVMF.fd \
  -drive format=raw,file=fat:rw:esp \
  -serial stdio \
  -m 256M \
  -nographic
```

Expected output:

```text
SigmaOS Boot v15.0
[BOOT] Kernel loaded at 0x200000
Σ SigmaOS Zenith Kernel Initializing (Rust)
[IRQ] PIC remapped, PIT 1000Hz, IDT ready
[MEM] Slab memory manager initialized
[init] PID 1 running
System Ready. Waiting for input...
```

---

## Project Structure

```text
SigmaOS/
├── arch/x86_64/       # CPU-specific: head64.asm, gdt.asm, idt.asm, context_switch.asm

├── sigma-boot/        # UEFI bootloader (Zig)

├── kernel/
│   ├── core/          # scheduler, MM, IPC, process manager, PCI, ACPI

│   ├── net/           # IP, TCP, UDP, socket API

│   ├── fs/            # VFS, tmpfs, ext4, procfs

│   ├── security/      # pledge, unveil, capabilities

│   └── linux_compat/  # ELF loader, vDSO, /proc shim

├── drivers/
│   ├── char/          # console, /dev/null, /dev/zero

│   ├── display/       # VGA text, VESA framebuffer

│   ├── input/         # keyboard (PS/2 + USB HID)

│   ├── net/           # e1000, virtio-net, WiFi

│   ├── storage/       # NVMe, AHCI

│   └── ddk/           # Driver Development Kit

├── kabi/              # Kernel ABI stability library

├── sdk/driver/        # Userspace driver SDK

├── sigma-sh/          # Shell (Rust std)

├── userland/coreutils/# ls, cat, grep, etc (22 commands)

├── sigmad/updater/    # A/B transactional updater

├── virtualization/    # OCI container runtime

└── tools/             # syscall profiler, CI tools

```

---

## Development Workflow

```bash

# 1. Make changes

# 2. Build kernel

cd kernel && cargo build --release 2>&1 | tail -5

# 3. Run tests

cargo test -p kernel_core 2>/dev/null || echo "(kernel tests not yet wired)"

# 4. Boot test

./qemu-boot.sh standalone

# 5. Check binary size

cd kernel && cargo bloat --release --crates

# 6. Submit PR

git checkout -b feat/my-improvement
git add -A
git commit -m "feat: description"
git push origin feat/my-improvement

# Open PR at https://github.com/AaryanSinghChauhan09/SigmaOS

```

---

## Key Files to Read First

| File | Why |
| ------ | ----- |
| `Architecture.md` | Complete system design |
| `kernel/core/sovereign_kernel_main.rs` | Boot sequence |
| `kernel/core/sigma_sched.rs` | Scheduler |
| `kernel/core/sigma_mm.rs` | Memory manager |
| `kernel/vfs/sigma_vfs.rs` | Filesystem layer |
| `kernel/net/tcp.rs` | TCP state machine |
| `drivers/ddk/sigma_ddk.rs` | Driver framework |
| `kabi/src/lib.rs` | Stable ABI |

---

## Common Issues

### Build fails: "can't find crate for `std`"

→ Kernel crates use `#![no_std]`. This is correct. Use `cargo build` in the `kernel/` subdirectory with the correct target.

### QEMU: "No bootable device"

→ The UEFI bootloader needs to be at `esp/EFI/BOOT/BOOTX64.EFI`. See the UEFI boot section above.

### "error: linker `rust-lld` not found"

→ Install llvm: `apt install lld` or `rustup component add llvm-tools-preview`

### Serial output blank in QEMU

→ Add `-serial stdio` to the QEMU command. Use `-nographic` to redirect VGA to terminal.

---

*See also: [Architecture.md](../Architecture.md) · [Kernel Developer Handbook](KERNEL_DEVELOPER_HANDBOOK.md) · [Contributing Drivers](CONTRIBUTING_DRIVERS.md)*


---
## Merged from Getting-Started.md
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
