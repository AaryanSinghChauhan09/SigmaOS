# SigmaOS — Sovereign Operating System

> **A modern, sovereign operating system distribution and workstation environment.**  
> Delivering an installable desktop distribution, modular system architecture, and defensive tooling.

```bash
# One-line workstation launch
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git && cd SigmaOS && ./run.sh
```

---

## 📊 Subsystem Status & Roadmap

In adherence to engineering integrity, Gall's Law, and defensive programming, SigmaOS distinguishes verified working components from experimental tracks and planned features:

| Subsystem / Capability | Current Status | Description & Verification |
|---|---|---|
| **Host Workstation Environment** | **Working** | Native binary (`bin/sigmaos`), interactive TUI, Python/GTK3 desktop launcher (`scripts/sigmaos_gui.py`). |
| **Defensive System Installer** | **Working** | Industrial installer (`iso_root/installer/install.sh`, `sigmaos-install`). Defaults to `DRY_RUN=1` simulation. Requires `--destructive --confirm-disk <DEV>`. Guards active host root drive. |
| **Diagnostic Health Suite** | **Working** | 54-point system health audit (`./run.sh --doctor`, `scripts/sigmaos_doctor.sh`). Validates display, GPU, fonts, Rust/Python runtimes, configs, and sovereign tooling. |
| **Component Sub-Repositories (22)** | **Working** | 22 implemented sub-repositories in pure `#![forbid(unsafe_code)]` Rust with comprehensive unit and integration test suites passing in CI. |
| **Desktop Profiles & Configuration** | **Working** | Wayland / Hyprland tiling desktop profile, Waybar configurations, dynamic Wallust theming, and Starship shell prompts. |
| **System CLI Tooling Suite** | **Working** | Full native shell CLI suite (`sigmaos-update`, `sigmaos-theme`, `sigmaos-wallpaper-set`, `sigmaos-pkgmgr`, `sigmaos-network`, `sigmaos-audio`, `sigmaos-bluetooth`, `sigmaos-snapshot`, `sigmaos-info`). |
| **Arch-based Live ISO Media** | **Working / Turnkey** | Turnkey Arch-based live ISO profile (`iso/archiso/`, `scripts/build_archiso.sh`) with 100 packages, Docker containerized builds (`--docker`), native Arch builds (`--native`), and defensive installer. |
| **Package Management Engine (`sigpkg`)**| **Experimental** | ALPM transaction model, dependency graph solver, and repository query client. |
| **Custom `#![no_std]` Microkernel** | **Planned / Research** | Clean-slate freestanding Rust/C microkernel research (`src/kernel/`, `iso-build/kernel-stub.c`). Maintained as a separate long-term research track. |

---

## ⚡ Turnkey Launch & Quickstart

| Command | Effect |
|---|---|
| `./run.sh` | Interactive sovereign workstation REPL |
| `./run.sh --desktop` | **Sovereign Desktop Profile** (Hyprland / Wayland / Waybar) |
| `./run.sh --minimal` | Sovereign Minimalist Profile (CLI core) |
| `./run.sh --gui` | Launch full graphical desktop application (GTK3 + WebKit2) |
| `./run.sh --native` | Launch pure GTK3 desktop interface |
| `./run.sh --doctor` | Run 54-point system health diagnostics |
| `./run.sh --dotfiles`| Deploy desktop configuration dotfiles to `~/.config` |
| `./run.sh --update` | Full update: system packages + dotfiles + Wallust theme |
| `./run.sh --theme` | Manage Wallust themes (list, apply, cycle next/prev) |
| `./run.sh --wallpaper`| Set wallpaper and dynamically regenerate color scheme |
| `./run.sh --pkgmgr` | Arch & AUR package management (install/search/clean/orphans) |
| `./run.sh --network` | Network connectivity CLI (wifi/ethernet/vpn/dns/speed) |
| `./run.sh --audio` | PipeWire audio output/input & volume manager |
| `./run.sh --bluetooth`| Bluetooth device manager (scan/pair/connect/status) |
| `./run.sh --snapshot`| Btrfs snapshot & rollback controller |
| `./run.sh --info` | System information display (Fastfetch style + JSON mode) |
| `./run.sh --check` | Run automated launch readiness validation |
| `./run.sh --qemu` | Boot test environment in QEMU virtual machine |

---

## 🛠 Native System CLI Suite

SigmaOS provides a complete set of ergonomic, shell-native management tools:

- **`sigmaos-update`**: Atomic updates across package manager, dotfiles, and dynamic themes with dry-run support.
- **`sigmaos-pkgmgr`**: Dual-layer package manager fronting official Arch Linux repositories and AUR helpers (`yay`/`paru`).
- **`sigmaos-network`**: Intuitive terminal network manager for WiFi, Ethernet, VPN, DNS, and speed benchmarking.
- **`sigmaos-audio`**: PipeWire / WirePlumber audio manager for volume, sink switching, microphone muting, and GUI mixer launch.
- **`sigmaos-bluetooth`**: BlueZ controller for scanning, pairing, connecting, and managing Bluetooth audio & peripherals.
- **`sigmaos-snapshot`**: Btrfs system recovery controller with pacman pre/post upgrade transaction hooks and subvolume rollbacks.
- **`sigmaos-theme`**: Wallpaper and palette engine supporting 6 built-in themes and dynamic Wallust generation.
- **`sigmaos-wallpaper-set`**: Multi-backend wallpaper switcher (`hyprpaper`, `swaybg`, `feh`, `nitrogen`).
- **`sigmaos-doctor`**: 54-point comprehensive system diagnostic validating display servers, GPU drivers, audio, and tools.
- **`sigmaos-info`**: Fastfetch-style terminal hardware/OS summary with JSON export for scripting.

---

## 🛡 Defensive Installation

SigmaOS prioritizes storage safety above all else. The installer protects host operating systems and connected storage devices:

```bash
# 1. Non-destructive simulation (DEFAULT behavior)
./iso_root/installer/install.sh /dev/vda
# → Simulates partition table, filesystem creation, and mounts. Writes ZERO bytes to disk.

# 2. Automated dry-run verification
./iso_root/installer/install.sh --auto --dry-run /dev/vda

# 3. Destructive installation (strictly requires both explicit flags):
# NOTE: Never run against host disks. Use only inside disposable QEMU virtual machines.
sudo ./iso_root/installer/install.sh --auto --destructive --confirm-disk /dev/vda /dev/vda
```

### Safety Guarantees:
- **Default Dry-Run:** `DRY_RUN=1` is enforced unless both `--destructive` and `--confirm-disk <DEVICE>` are supplied.
- **Active Root Guard:** Automatically identifies the host partition and physical disk hosting `/` and refuses execution if targeted.
- **Active Mount Protection:** Scans `/proc/mounts` and aborts if any partition on the target device is mounted.
- **Trap Cleanup:** Handles `SIGINT`, `SIGTERM`, and `EXIT` signals to unmount temporary target mounts and remove runtime lockfiles.

---

## 🖥 Virtual Machine Testing (QEMU)

Always verify live boot and installation within an isolated, disposable virtual disk:

```bash
# 1. Create a 32GB disposable virtual test disk
qemu-img create -f qcow2 /tmp/sigmaos_test_disk.qcow2 32G

# 2. Validate installer dry-run & boot capabilities via automated script
bash scripts/qemu_smoke_test.sh

# 3. Boot ISO in QEMU with disposable drive attached
qemu-system-x86_64 \
  -m 4096 \
  -enable-kvm \
  -drive file=/tmp/sigmaos_test_disk.qcow2,format=qcow2,if=virtio \
  -cdrom out/sigmaos-desktop.iso \
  -boot d
```

---

## 🏛 Package Management & Substrate Architecture

SigmaOS provides a curated packaging layer built upon standard ALPM binary package definitions and modular service components:

- **ALPM Engine:** Version comparison, dependency graph resolution, and sync database parsing.
- **`sigpkg` Interface:** Package transaction engine and system maintenance tools (`checkupdates`, `paccache`, `pactree`).
- **Hardware Integration:** Automatic graphics driver detection (Intel Iris/Xe, AMD Radeon, NVIDIA, VirtIO GPU) and PipeWire audio stack.

---

## 📦 Implemented Component Fleet

All component sub-repositories are developed in safe `#![forbid(unsafe_code)]` Rust with automated test coverage:

| Component Repository | Purpose | Test Status |
|---|---|---|
| `SigmaOS-domain-blocker` | DNS-level filtering and domain sinkholing engine | 15 Unit Tests Passing |
| `SigmaOS-fish` | Smart fish-compatible shell and completion system | 15 Unit Tests Passing |
| `SigmaOS-packages-kit` | Transactional package kit and dependency solver | 20 Unit Tests Passing |
| `SigmaOS-player` | Media playback engine and audio backend | 15 Unit Tests Passing |
| `SigmaOS-renamer-file` | Batch regex file renamer and metadata transformer | 14 Unit Tests Passing |
| `SigmaOS-apturl` | URL protocol handler for package installation | 8 Unit Tests Passing |
| `SigmaOS-asdcontrol-git` | Display brightness and hardware controller | 7 Unit Tests Passing |
| `SigmaOS-audio-tuner` | Real-time audio equalization and DSP processor | 6 Unit Tests Passing |
| `SigmaOS-automate` | System workflow automation and task scheduling | 6 Unit Tests Passing |
| `SigmaOS-chatroom` | Encrypted peer-to-peer workspace messaging | 6 Unit Tests Passing |
| `SigmaOS-daemon-apt` | Background package daemon and cache coordinator | 6 Unit Tests Passing |
| `SigmaOS-desktop-portal` | Desktop portal bridge for sandboxed applications | 5 Unit Tests Passing |
| `SigmaOS-dev-tools` | Developer workstation diagnostics and profiling | 6 Unit Tests Passing |
| `SigmaOS-file-writer` | Atomic file writer with integrity validation | 6 Unit Tests Passing |
| `SigmaOS-folder-color-switcher` | Folder icon color customization daemon | 6 Unit Tests Passing |
| `SigmaOS-greeter` | Wayland login greeter and session manager | 6 Unit Tests Passing |
| `SigmaOS-office` | Unified document viewer and office suite bridge | 6 Unit Tests Passing |
| `SigmaOS-packages-tools` | Package auditing, linting, and verification tools | 6 Unit Tests Passing |
| `SigmaOS-player-library` | Media library indexer and metadata database | 6 Unit Tests Passing |
| `SigmaOS-theme-desktop-environment`| Global desktop theme and palette manager | 6 Unit Tests Passing |
| `SigmaOS-theme-gtk3` | GTK3 stylesheet generator and color harmonizer | 5 Unit Tests Passing |
| `SigmaOS-theme-icon` | Icon theme indexing and resolution engine | 5 Unit Tests Passing |

---

## 🎨 Craft Creative Suite (20 Modules)

SigmaOS incorporates a creative production suite across audio, video, vector, and 3D modeling:

- **3DCraft:** 3D Modeling, Rigging & Rendering engine
- **AniCraft:** 2D Vector & Traditional Animation suite
- **ArtCraft:** Digital Painting & Illustration canvas
- **CADCraft:** Parametric CAD & Mechanical Design
- **FilmCraft:** Non-Linear Video Editor & Compositor
- **SoundCraft:** Digital Audio Workstation (DAW) & Synthesizers
- **DesignCraft:** Vector UI/UX Canvas & Prototyping
- **VectorCraft:** Scalable Vector Graphics Editor
- *...along with FontCraft, GridCraft, PDFCraft, PhotoCraft, PrintCraft, and SoundCraft modules.*

---

## 📋 Foundational Principles

This project adheres to rigorous software engineering disciplines:
- **Gall's Law:** A complex system that works is invariably found to have evolved from a simple system that worked.
- **Defensive Programming:** Fail-safe defaults, dry-run simulations, and active disk protection.
- **Modularity & High Cohesion:** Isolated components communicating via defined interfaces.
- **Zero-Copy & Mechanical Sympathy:** Optimized I/O pipelines and memory discipline.
- **Least Astonishment:** Intuitive CLI semantics and standard Unix conventions.

---

## 📝 License

MIT OR GPL-2.0 — Free and Open Source Software.  
© SigmaOS Contributors
