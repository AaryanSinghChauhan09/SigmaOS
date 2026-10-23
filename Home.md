# SigmaOS Sovereign Operating System Documentation

Welcome to the official, consolidated GitHub Wiki for **SigmaOS** — a sovereign, `#![no_std]` Rust-native operating system designed for extreme performance, absolute self-sufficiency, security, and developer ergonomics.

---

## 🟢 Current Status (2026-10-23)

### Main Branch Health
- **Build Status**: ✅ `cargo check --lib` — PASSING (0 errors)
- **Open Branches**: ✅ **ZERO** — all 97 branches merged into `main` via sovereign unification
- **Open PRs**: ✅ **ZERO** — all 98 PRs closed (merged into main)
- **Root `.md`/`.txt` Files**: ✅ None — all moved to `docs/` (main kept clean)
- **Redundant Branches Deleted**: ✅ All 97 feature/docs/jules branches deleted from remote
- **Single Repository**: ✅ Only `main` branch on `origin` (+ `wiki-origin/master`)

### Sovereign Unification Summary (Oct 23, 2026)
| Metric | Result |
|---|---|
| Branches merged into `main` | 97 / 97 ✅ |
| PRs closed | 98 / 98 ✅ |
| Remote branches deleted | 97 / 97 ✅ |
| Root `.md` files moved to `docs/` | 26 files ✅ |
| Strategy used | `-Xtheirs` (THEIRS / XTHEIRS principles) |
| Final remote branches | `origin/main` only ✅ |

### ISO Architecture Improvements
- **New**: `src/iso/sovereign_iso_architecture.rs` — Complete ISO layer abstraction
  - `IsoLayer` trait (ABSTRACTION + INTERFACE SEGREGATION)
  - `BootLayer` with GRUB2 + Limine config generation
  - `LiveFilesystemLayer` with squashfs/overlayfs (ZERO-COPY)
  - `InstallerLayer` with Calamares/archinstall/TUI support
  - `PackageManifestLayer` (MODULARITY)
  - `SigningLayer` with GPG/SHA256/SBOM (SECURE DEFAULTS)
  - `SigmaIsoProfile` desktop + server profiles (COMPOSITION OVER INHERITANCE)
  - `IsoDeltaUpdater` (SOFTWARE UPDATING PROGRAM LIKE LINUX DISTROS)
  - `SupportChannel` LTS/Standard/Rolling/Edge (SOFTWARE SUPPORT PROGRAM)
  - Full test suite (ARRANGE-ACT-ASSERT + FIRST + DETERMINISTIC TESTING)

---

## 🏛️ Foundational Architectural & Engineering Principles

SigmaOS is engineered in strict compliance with all industry and systems engineering principles:

| Principle Category | Enforced Standards |
| :--- | :--- |
| **Architecture & Structure** | `Abstraction`, `Layered Design`, `KISS`, `Gall's Law`, `Booch Method`, `Composition over Inheritance`, `Minimal Kernel Philosophy`, `Modular Kernel Design` |
| **Object & Interface Design** | `SOLID`, `GRASP`, `Command-Query Separation (CQS)`, `Demeter Law`, `Interface Segregation`, `Opaque Types`, `Uniform Access`, `Open-Closed` |
| **Systems & Hardware Control** | `Low-Level Control`, `Zero-Copy`, `Cache Conscious Design`, `Bounded MMIO`, `Lock-Free Ring Buffers`, `DMA Safety`, `Direct Device Control`, `Low-Level Languages` |
| **Memory & Concurrency** | `Memory Safety`, `Manual Memory Discipline`, `Deadlock Avoidance`, `Safe Concurrency`, `Race-Free Design`, `Locality of Reference`, `Virtual Memory Abstraction` |
| **Testing & Robustness** | `Arrange-Act-Assert (AAA)`, `FIRST`, `Deterministic Testing`, `Fail-Fast`, `Defensive/Offensive Boundaries`, `Power of Ten` |
| **Code Quality** | `DRY`, `Boy Scout Rule`, `YAGNI`, `Rule of Three`, `Refactoring`, `Avoid Fragmentation`, `Optimise for Deletion` |
| **OS Design Patterns** | `Blackbox`, `Process Isolation`, `Least Privilege`, `Secure Defaults`, `Separation of Mechanism & Policy`, `Plug-In Driver Model` |
| **Community & Governance** | `FOSS`, `Meritocracy & Inclusivity`, `Governance & Do-Cracy`, `Respect & Ethical Participation`, `Practical Licensing Awareness` |
| **Version Control** | `Single Repository`, `Pristine Sources`, `VERSION CONTROL PRACTICES`, `THEIRS`, `XTHEIRS`, `Talk is Cheap Show Me the Code` |
| **ISO / Distro Principles** | `Software Support Program`, `Software Updating Program`, `Stable APIs`, `Bootloader Integration`, `Cross Compilation`, `Portability` |
| **Philosophy** | `Zen of Python`, `Worse is Better`, `Zero-One-Infinity`, `Ninety-Ninety Rule`, `Deutsch Limit`, `If It Ain't Broke Don't Fix It` |

---

## 📦 ISO Structure (Inspired by Real OS ISOs)

SigmaOS follows the standard live ISO layout used by Linux Mint, Arch Linux, Debian, FreeBSD, and Omarchy:

```
sigmaos-desktop-x86_64.iso
├── /boot/
│   ├── vmlinuz              # Linux kernel (or SigmaOS kernel stub)
│   ├── initrd.img           # initramfs with live-boot + installer
│   ├── memtest86+.bin       # Memory tester
│   ├── grub/
│   │   ├── grub.cfg         # GRUB2 menu (Live / Safe Mode / Installer)
│   │   └── fonts/unicode.pf2
│   └── limine/
│       └── limine.conf      # Limine bootloader config (UEFI alternative)
├── /EFI/
│   └── BOOT/
│       └── BOOTX64.EFI      # UEFI bootloader stub
├── /live/
│   └── filesystem.squashfs  # Compressed rootfs (zstd, 128K blocks)
├── /install/
│   └── filesystem.tar.gz    # Installer rootfs (server mode)
├── /pool/                   # Local package cache (offline install)
├── /isolinux/               # Legacy BIOS syslinux support
├── md5sum.txt               # SHA256 checksums
├── sigmaos.gpg              # GPG signature
└── sbom.json                # Software Bill of Materials (SLSA)
```

### Key ISO Innovations vs Other OSes

| Feature | Linux Mint | Arch ISO | FreeBSD | SigmaOS |
|---|---|---|---|---|
| Bootloader | GRUB2 | syslinux + GRUB | loader(8) | GRUB2 + Limine |
| Compression | zstd | zstd | xz | zstd (default) |
| Overlay FS | overlayfs | overlayfs | tmpfs | overlayfs |
| Installer | Calamares | archinstall | bsdinstall | Calamares + Rust TUI |
| Unattended | Partial | JSON profile | autoinstall | cloud-init / JSON |
| Delta updates | apt delta | — | freebsd-update | casync delta |
| Support model | LTS + Standard | Rolling | RELEASE | LTS + Rolling + Edge |
| SBOM | No | No | No | ✅ Yes (SLSA) |
| Multi-arch | amd64/arm64 | x86_64 | amd64/arm64 | x86_64/aarch64/riscv64 |

---

## 🗂️ Quick Navigation

### Core Architecture
- [[Architecture-Overview]] — Microkernel + modular design
- [[Kernel-Subsystems]] — Process, Memory, VFS, IPC, Network
- [[ISO-Architecture]] — Live ISO structure and boot process

### Development Plans
- [[26-System-Utilities-Development-Plan]] — CLI tools
- [[27-SigmaOS-CLI-Development-Plan]] — Shell + CLI
- [[28-Network-Development-And-Maintenance-Plan]] — Networking
- [[30-SigmaOS-GUI-Development-Plan]] — Desktop / Wayland
- [[35-SigmaOS-Device-Driver-Development-Plan]] — Drivers

### OS Parity & Inspirations  
- [[37-Missing-Components-vs-Linux-BSD-Distros]] — Gap analysis
- [[40-Omarchy-Linux-And-Sovereign-Desktop-Parity-AI-Agent-Roadmap]]
- [[41-Linux-BSD-Kernel-Security-And-Virtualization-AI-Agent-Roadmap]]
- [[42-Rust-OSDev-Ecosystem-Synthesis-And-Parity]]

### Creative Suite (ArtCraft)
- [[AniCraft-Animation-Engine]]
- [[WordCraft-Typographic-Document-Processor]]
- [[SoundCraft-DAW-Component-Guide]]
- [[VoiceCraft-Speech-and-Dictation]]
- [[3DCraft-Modeling-and-Rendering]]

### Security
- [[07-Security]] — Security architecture
- [[Apparmor-Seccomp-Isolation-Component-Guide]]
- [[Nftables-Pf-Packet-Filtering-Component-Guide]]

---

## 🚀 Getting Started

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Check build (no compilation errors)
make check

# Run test suite
make test

# Build stub ISO (boots in QEMU immediately)
make iso-stub

# Boot in QEMU
make run-stub

# Full interactive workstation
make launch
```

---

## 📜 License

SigmaOS is dual-licensed under **MIT** and **GPL-2.0** (your choice), in the spirit of Free and Open Source Software (FOSS) principles and Practical Licensing Awareness.

---

*Last updated: 2026-10-23 — Sovereign Unification Complete. 97 branches merged, 98 PRs closed, main branch is the single source of truth.*
