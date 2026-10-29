# SigmaOS

**Sovereign, AI-Native Operating System** — built in `#![no_std]` Rust, inspired by Arch Linux, Omarchy, and Linux Mint.

---

## Quick Start

```bash
# 1. Install build dependencies (one-time)
bash scripts/setup.sh

# 2. Check everything is ready
bash scripts/check_launch_ready.sh

# 3. Launch interactive workstation dashboard
make launch

# 4. Build a bootable ISO (GRUB2-based, boots in QEMU)
make iso-stub

# 5. Boot in QEMU
make run-stub
```

## Requirements

| Tool | Purpose | Install |
|------|---------|---------|
| Rust (stable) | Core build | `curl https://sh.rustup.rs -sSf \| sh` |
| gcc (32-bit) | Multiboot kernel | `sudo apt install build-essential` |
| grub-pc-bin | ISO bootloader | `sudo apt install grub-pc-bin` |
| genisoimage | ISO creation | `sudo apt install genisoimage` |
| nasm *(optional)* | Stub kernel ASM | `sudo apt install nasm` |
| xorriso *(optional)* | Alt ISO tool | `sudo apt install xorriso` |
| qemu-system-x86 *(optional)* | VM testing | `sudo apt install qemu-system-x86` |

Run `bash scripts/setup.sh` to install everything automatically (detects apt/pacman/dnf/brew).

## Architecture

```
SigmaOS
├── src/            # ~1M LOC Rust library — 190 modules
│   ├── kernel/     # Microkernel: scheduler, VFS, memory, drivers
│   ├── bin/        # CLI binaries: sigmaos, sigpkg, sigma_pkg
│   └── ...         # Desktop, security, networking, creative suite
├── boot/           # Multiboot kernel (C), linker script, GRUB config
├── bin/            # Shell scripts: sigmaos-launch, sigmaos-shell, …
├── scripts/        # Build, ISO, QEMU, CI helpers
├── install/        # Package lists, post-install hooks, provisioning
└── tests/          # 40+ native test suites
```

## Inspirations

| Distro | What SigmaOS Takes |
|--------|-------------------|
| **Arch Linux** | Rolling release, pacman/pkgbuild, AUR compatibility |
| **Omarchy** | Hyprland WM, Quickshell bar, keyboard-first UX |
| **Linux Mint** | Ergonomics, Nemo file manager, MintUpdate GUI |
| **CachyOS** | BORE/EEVDF scheduler, AutoBOLT/PGO, zram-zstd |
| **FreeBSD** | Capsicum capability isolation, kqueue, Jails |
| **OpenBSD** | `pledge()`, `unveil()`, W^X PTE protection |
| **DragonFlyBSD** | HAMMER2 VFS, lightweight kernel threads |

## Build Commands

```bash
make check          # cargo check --lib (static analysis)
make check-fast     # Cached quick cargo check
make test           # Native standalone test suites (./run_sigma_tests.sh)
make test-fast      # cargo test --lib (parallel unit tests)
make fmt            # Verify formatting
make fmt-fix        # Auto-apply rustfmt
make iso-stub       # Build GRUB2 bootable ISO ✓ (works now)
make run-stub       # Boot stub ISO in QEMU ✓
make launch         # Interactive workstation dashboard ✓
make kernel         # Compile Multiboot C kernel
make clean          # Remove build artifacts
```

## 20-Module Craft Creative Suite

3DCraft · AniCraft · ArtCraft · CADCraft · CraftLibs · DeckCraft · DesignCraft · EffectCraft · FigCraft · FilmCraft · FontCraft · GridCraft · LightCraft · PDFCraft · PhotoCraft · PrintCraft · SoundCraft · VectorCraft · VoiceCraft · WordCraft

## Project Status

| Component | Status |
|-----------|--------|
| Rust library (190 modules) | ✅ `cargo check --lib` — 0 errors |
| Bootable ISO | ✅ 2.8 MB ISO9660 + GRUB2 El Torito |
| Multiboot kernel | ✅ 32-bit ELF, boots in QEMU |
| Native test suites | ✅ 40 suites, 300+ tests, 0 failures |
| Desktop shell daemon | ✅ `bin/sigmaos-shell ping` — responsive |
| Installation wizard | ✅ `installer/sigma-installer.rs` |
| Package manager | ✅ SigPkg (`src/bin/sigpkg.rs`) |

## Repository

[https://github.com/AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
