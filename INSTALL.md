# SigmaOS Installation Guide

> **Status (July 2026):** SigmaOS does not yet produce a bootable ISO (`make iso` is Phase G, v16.0 Apex). The QEMU demo runs the existing kernel stub. A bootable installer will ship with v16.0.

---

## Quick Start (QEMU Demo)

### Prerequisites

**Ubuntu / Debian:**
```bash
sudo apt install -y build-essential nasm cmake qemu-system-x86 \
  golang-go xorriso mtools grub-pc-bin grub-efi-amd64-bin lld
```

**Arch Linux:**
```bash
sudo pacman -S base-devel nasm cmake qemu-system-x86 go xorriso grub lld
```

**Windows (WSL2):**
```bash
# Inside WSL2 Ubuntu
sudo apt install -y build-essential nasm cmake qemu-system-x86 golang-go lld
```

**Rust Toolchain (Required for Kernel Build):**
```bash
# Install Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install components
rustup default stable
rustup component add rustfmt clippy
# Verify installation
rustc --version
cargo --version
lld --version
```

### Clone & Build

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
make clean
make all -j$(nproc)
```

### Run in QEMU

```bash
qemu-system-x86_64 \
  -cdrom build/sigmaos.iso \
  -m 2G \
  -serial stdio \
  -enable-kvm        # optional, Linux only
```

---

## Build Profiles

```bash
# Full desktop (standalone)
make PROFILE=standalone all -j$(nproc)

# Minimal microkernel
make PROFILE=microkernel all -j$(nproc)

# Cloud/container headless
make PROFILE=cloud all -j$(nproc)

# ARM64 (cross-compile)
make PROFILE=mobile ARCH=arm64 all -j$(nproc)
```

---

## Check for Stub Implementations

```bash
make check-stubs
```

This reports which kernel bodies are still placeholder stubs — useful for contributors.

---

## Development Environment (Dev Container)

The repo includes a `.devcontainer/` for VS Code Dev Containers:

1. Install Docker + VS Code Remote – Containers extension
2. Open repository folder in VS Code
3. Reopen in Container — all tools pre-installed

---

## Planned: Physical Install (v16.0+)

Once `make iso` produces a bootable image (Phase G):

1. Flash to USB: `dd if=SigmaOS.iso of=/dev/sdX bs=4M status=progress`
2. Boot from USB (disable Secure Boot temporarily until sigma-boot.efi is signed)
3. Follow the graphical installer (Calamares equivalent, Phase G)

### Dual-Boot (v16.0+)

SigmaOS will detect existing Windows/Linux installs and offer:
- Side-by-side EFI boot entries
- GRUB chainload fallback
- NTFS read-only access to Windows partition

---

## Troubleshooting

| Problem | Fix |
|---------|-----|
| `nasm: command not found` | `sudo apt install nasm` |
| QEMU black screen | Add `-vga std` to QEMU flags |
| Build fails on missing headers | Run `make fix-includes` |
| Clangd errors in IDE | Run `make compile_commands` to regenerate `compile_commands.json` |

---

*See also: [Building from Source (Wiki)](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Building-from-Source) · [CONTRIBUTING.md](CONTRIBUTING.md)*
