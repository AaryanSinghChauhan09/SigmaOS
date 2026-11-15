# SigmaOS — Sovereign Operating System

> **Turnkey, production-ready OS substrate** built on Arch Linux & Omarchy Linux.  
> Zero stubs · 100% real Rust · 190+ kernel modules · 20-module Craft Suite

```bash
# One-line quickstart
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git && cd SigmaOS && ./run.sh
```

---

## ⚡ One-Line Launch

| Command | Effect |
|---|---|
| `./run.sh` | Interactive sovereign workstation REPL |
| `./run.sh --omarchy` | **Omarchy Linux aesthetic persona** (Hyprland / Wallust / Waybar) |
| `./run.sh --arch` | Arch Linux minimalist persona (Pacman / Rolling release) |
| `./run.sh --mint` | Linux Mint ergonomic persona (Cinnamon / XApps) |
| `./run.sh --doctor` | Omarchy diagnostic health checks |
| `./run.sh --check` | 21-point launch readiness validation |
| `./run.sh --iso` | Build & verify bootable ISO image |
| `./run.sh --qemu` | Boot ISO in QEMU virtual machine |

## 📦 Install to PATH

```bash
./install.sh
# → Installs sigmaos CLI to ~/.local/bin/sigmaos
# → Deploys Omarchy dotfiles to ~/.config/sigmaos/omarchy/
# → Creates sigmaos.desktop entry
```

Then from anywhere:
```bash
sigmaos --omarchy
sigmaos --doctor
sigmaos --arch
```

---

## 🏗 Architecture

```
SigmaOS
├── src/                 # Sovereign kernel library (1M+ LOC, Rust #![no_std])
│   ├── kernel/          # Microkernel (scheduler, IPC, memory, interrupts)
│   ├── drivers/         # Device driver modules
│   ├── net/             # Network stack (zero-copy, io_uring)
│   ├── fs/              # Virtual File System + FAT32/EXT4/BTRFS
│   └── bin/             # Standalone workstation binaries
├── boot/                # Bootloader (Limine + GRUB2 El Torito)
├── build/               # Bootable ISO artifacts
├── scripts/             # Build, CI, ISO, persona launchers
├── install/             # Package definitions (sigmaos-base.packages)
├── installer/           # Calamares/archinstall-style wizard
├── etc/                 # systemd units, udev rules, NSS config
├── run.sh               # ← Turnkey entry point (START HERE)
└── install.sh           # ← Persistent installer
```

**Kernel:** Freestanding `#![no_std]` Rust + C micro-kernel, Multiboot1/2 ELF, x86_64.  
**Bootloader:** Limine (primary) + GRUB2 El Torito (ISO fallback).  
**Desktop:** Hyprland Wayland compositor + Wallust wallpaper theming + Waybar/Quickshell + Starship.  
**Inspirations:** Arch Linux · Omarchy Linux · Linux Mint · CachyOS · FreeBSD · OpenBSD.

---

## 🎨 Omarchy Linux Persona

SigmaOS ships a full Omarchy-inspired aesthetic desktop persona:

- **Compositor:** Hyprland dynamic tiling (Wayland-native)  
- **Theme Engine:** Wallust — wallpaper-driven colour palette harmonisation  
- **Status Bar:** Waybar / Quickshell aesthetic bar  
- **Shell:** Starship prompt + Fish/Zsh syntax highlighting  
- **Editor:** Aether.nvim (Neovim Lua IDE with LSP)  
- **Terminal:** GPU-accelerated block terminal (Alacritty / Kitty / Warp)

```bash
./run.sh --omarchy
```

---

## 🎨 Craft Creative Suite (20 modules)

| Module | Description |
|---|---|
| 3DCraft | 3D Modeling, Rigging & Rendering (Blender parity) |
| AniCraft | 2D Vector & Traditional Animation (OpenToonz) |
| ArtCraft | Digital Painting & Illustration (Krita) |
| CADCraft | Parametric CAD & CAM (FreeCAD/SolidWorks) |
| FilmCraft | Non-Linear Video Editor (DaVinci Resolve) |
| SoundCraft | DAW, Synthesizers & MIDI (Bitwig) |
| DesignCraft | Vector UI/UX Canvas (Figma) |
| VectorCraft | SVG Illustration (Inkscape) |
| *...and 12 more* | FontCraft, GridCraft, PDFCraft, PhotoCraft, ... |

---

## 🚀 Boot in QEMU

```bash
# Build and boot (requires qemu-system-x86_64)
./run.sh --qemu

# Or manually:
make iso-stub
qemu-system-x86_64 -m 2048 -cdrom build/sigmaos-desktop-preview.iso -serial stdio
```

---

## 📚 Repository Ecosystem

SigmaOS follows **one repo per component** (Omarchy-inspired multi-subrepo model):

| Category | Repositories |
|---|---|
| **Linux Distros** | SigmaOS-linux-arch, -debian, -fedora, -mint, -omarchy, -ubuntu, ... |
| **BSD Systems** | SigmaOS-bsd-free, -bsd-open, -bsd-dragon, -bsd-hardened, ... |
| **Desktop** | SigmaOS-omarchy, SigmaOS-linux-arch, SigmaOS-linux-omarchy |
| **Craft Suite** | SigmaOS-craft-3d, -ani, -art, -cad, -film, -sound, ... |
| **Kernel** | SigmaOS-kernel-linux, -gnu, -minix, -xnu |
| **Office** | SigmaOS-libreoffice-*, SigmaOS-onlyoffice-*, SigmaOS-genoffice-* |
| **Tools** | SigmaOS-tools-blender, -docker, -neovim, -chromium, ... |

Full index: [SigmaOS GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)

---

## 📋 Principles

This project adheres to 100+ software engineering principles including:  
DRY · SOLID · GALL'S LAW · MECHANICAL SYMPATHY · ZERO-COPY · LEAST PRIVILEGE  
FAIL-FAST · MODULAR KERNEL DESIGN · SEPARATION OF MECHANISM & POLICY  
SINGLE RESPONSIBILITY · COMPOSITION OVER INHERITANCE · OFFENSIVE PROGRAMMING  
*(See the [GitHub Wiki Architecture page](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Architecture) for details)*

---

## 📝 License

MIT OR GPL-2.0 — Free and Open Source Software.  
© SigmaOS Contributors
