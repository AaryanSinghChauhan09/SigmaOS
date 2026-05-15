# 📱 SigmaOS v15.0 Zenith — App Edition

> **A sovereign app-first OS. Run any app, any platform, any runtime — on your silicon.**

[![Release](https://img.shields.io/badge/release-v15.0--zenith--app-green)](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0-zenith-app)
[![Architecture](https://img.shields.io/badge/arch-x86__64%20%7C%20ARM64-green)](https://github.com/AaryanSinghChauhan09/SigmaOS)
[![Runtimes](https://img.shields.io/badge/runtimes-Linux%20%7C%20Windows%20%7C%20Android%20%7C%20WASM-blue)](https://github.com/AaryanSinghChauhan09/SigmaOS)

---

## 📋 Overview

**SigmaOS Zenith App** is the universal application platform edition of SigmaOS v15.0. It is engineered to run applications from **any ecosystem** — Linux ELF binaries, Windows PE executables, Android APKs, and native Sovereign shards.

Built on the **SigmaOS Unified Core**, this edition includes the complete mandatory baseline toolset (Maintenance, Creative, Productivity) and is layered with universal runtimes (S-Wine, S-ARC) for maximum application compatibility.

This edition targets **developers, power users, creative professionals, and enterprise workstations** who need maximum application compatibility without sacrificing sovereign security.

| Property | Value |
|---|---|
| Edition | Zenith App |
| Version | v15.0.0 |
| Kernel | Sovereign Lattice Microkernel v15.0 |
| Architecture | x86_64, ARM64 |
| Desktop | Zenith App Launcher (ZAL) + Tiling WM |
| App Runtimes | Native, Linux ELF, Windows (S-Wine), Android (S-ARC), WASM |
| App Store | Sovereign App Nexus (SAN) — curated + PQC-verified |
| Target | Developer workstations, creative professionals, enterprise desktops |

---

## ⚡ Key Features

### 📦 Universal Application Runtime Layer (UARL)

- **Native Sovereign Apps**: `.sshard` packages — maximum performance, kernel-direct
- **Linux ELF Compatibility**: Run any Linux binary via POSIX compatibility layer
- **S-Wine (Windows Compat)**: Run Windows applications without a VM via S-Wine translation layer
- **S-Android Runtime (S-ARC)**: Run Android APKs natively using kernel-level ART translation
- **WebAssembly Runtime**: Run WASM modules as first-class sovereign shards
- **Flatpak/AppImage Support**: Standard portable app formats recognized and sandboxed
- **Container Runtime**: Run Docker/OCI containers as sovereign shards via `sigma-container`

### 🏪 Sovereign App Nexus (SAN)

- **Curated App Catalog**: 50,000+ applications across all categories
- **PQC-Attested Packages**: Every app cryptographically signed with Dilithium-5
- **Zero-Supply-Chain-Attack**: Reproducible builds with hash-verified compilation chains
- **Auto-Update Engine**: Background updates with automatic rollback on failure
- **Category Browsing**: Development, Productivity, Finance, Creative, Science, Games

### 🔒 Per-App Security (S-ARMOR App Isolation)

- **App Namespacing**: Each app gets isolated PID/NET/MNT/USER namespace
- **Permission Model**: Explicit permission grants for: camera, microphone, network, files
- **S-Sandbox Profiles**: Per-app filesystem view — apps only see what they're allowed to
- **Network Firewall per App**: Block/allow internet access on per-app basis
- **Screen Capture Protection**: Sensitive apps can prevent screenshots/screen recording
- **Clipboard Isolation**: Cross-app clipboard access requires explicit grant

### 🚀 Developer Tools (Built-In)

- **SovereignIDE**: Lightweight IDE with LSP support (syntax, completion, debugging)
- **Sovereign Playground**: Interactive compute sandbox — try C++/Rust/Python instantly
- **Terminal Emulator**: GPU-accelerated terminal with ligature font support
- **Git Integration**: Native git with Dilithium-5 commit signing
- **Debug Interface**: GDB-compatible debugger with kernel-aware backtraces
- **Profiler**: Built-in performance profiler with flame graph visualization

### 🖥️ Zenith App Launcher (ZAL) — Desktop

- **Dynamic App Grid**: All installed apps in searchable, categorized grid
- **Smart Dock**: Pin favorite apps with jump-list menus
- **Virtual Desktops**: 10 workspaces with per-desktop wallpaper and layout persistence
- **Window Snapping**: Magnetized window tiling with 1/2, 1/3, 2/3 presets
- **Quick Switch**: Rofi-style instant app switcher (Super+Tab)
- **App Streaming**: Cast app windows to remote displays via Sovereign Display Protocol

---

## 💻 System Requirements

| Component | Minimum | Recommended |
|---|---|---|
| CPU | x86_64 (SSE4.2+) or ARM64 | Intel 10th Gen+ / AMD Zen 3+ |
| RAM | 4 GB | 16 GB+ (32 GB for Android/Windows layers) |
| Storage | 30 GB | 100 GB+ NVMe |
| GPU | OpenGL 3.3+ | Vulkan 1.3 (NVIDIA RTX / AMD RDNA+) |
| Network | Ethernet/Wi-Fi | Gigabit for app streaming |
| Firmware | UEFI 2.4+ | UEFI 2.6+ |

> ⚠️ **S-Wine (Windows app compatibility)** requires **8 GB+ RAM** for optimal performance.
> ⚠️ **S-ARC (Android runtime)** requires **x86_64 with hardware virtualization** (VT-x/AMD-V).

---

## 🛠️ Installation Guide

### Step 1 — Download App Edition ISO

```bash
curl -LO https://github.com/AaryanSinghChauhan09/SigmaOS/releases/download/v15.0-zenith-app/SigmaOS-v15.0-Zenith-App-x86_64.iso

# Verify signature

sigma-verify --dilithium5 SigmaOS-v15.0-Zenith-App-x86_64.iso SigmaOS-v15.0-Zenith-App-x86_64.iso.sig
```

### Step 2 — Flash & Install

```bash

# Flash to USB

sudo dd if=SigmaOS-v15.0-Zenith-App-x86_64.iso of=/dev/sdX bs=4M status=progress && sync
```

Boot → Select **"Install SigmaOS App Edition"**

Recommended partition layout:

```
/dev/sda1  →  512MB    EFI
/dev/sda2  →  8GB      Swap (needed for large apps)
/dev/sda3  →  50GB+    / (root — App OS)
/dev/sda4  →  rest     /home (user data + app installations)
```

### Step 3 — First Boot — Runtime Setup

On first boot, a **Runtime Configuration Wizard** guides setup:

```
[1] Core Runtimes (always installed)
    ✅ Sovereign Native Runtime (SSHARD)
    ✅ Linux ELF Compatibility Layer

[2] Optional Runtimes (select what you need)
    [ ] S-Wine (Windows app compatibility) — requires 8GB RAM
    [ ] S-Android Runtime (S-ARC) — requires VT-x/AMD-V
    [ ] WebAssembly Runtime

[3] App Nexus
    ✅ Connect to Sovereign App Nexus
    [ ] Import from Linux (detect installed Flatpaks/AppImages)
    [ ] Import from Windows (detect installed apps via NTFS scan)
```

### Step 4 — Install Your First Apps

```bash

# Via command line:

sigma-san install "vscode"              # VS Code (Linux compatibility layer)

sigma-san install "firefox"             # Firefox (native Linux build)

sigma-san install "libreoffice"         # LibreOffice (native)

sigma-san install "gimp"                # GIMP image editor

sigma-san install "blender"             # Blender 3D (Vulkan-accelerated)

# Windows apps (via S-Wine):

sigma-wine install "photoshop-2024"     # Adobe Photoshop (compatibility)

sigma-wine install "office-365"         # Microsoft Office 365 (compatibility)

# Android apps (via S-ARC):

sigma-arc install "whatsapp.apk"        # WhatsApp APK

sigma-arc install --store "com.spotify.music"  # From Google Play (bridged)

```

---

## 🔧 App Management Functions Reference

### sigma-san — Sovereign App Nexus CLI

```bash
sigma-san search <query>               # Search app catalog

sigma-san install <app-id>             # Install app with PQC verification

sigma-san remove <app-id>              # Remove app and all data

sigma-san update                       # Update all installed apps

sigma-san update <app-id>              # Update specific app

sigma-san rollback <app-id>            # Rollback to previous version

sigma-san list                         # List all installed apps

sigma-san info <app-id>               # Show app details + permissions

sigma-san permissions <app-id>         # View/modify app permissions

sigma-san verify <app-id>             # Re-verify PQC signature

sigma-san export <app-id> ~/           # Export app as portable bundle

```

### sigma-wine — Windows App Compatibility

```bash
sigma-wine list                        # List installed Windows apps

sigma-wine install <app-installer.exe> # Install Windows .exe

sigma-wine run <app-id>               # Launch Windows app

sigma-wine config --dxvk               # Enable DXVK (DirectX → Vulkan)

sigma-wine config --vcredist            # Install Visual C++ runtimes

sigma-wine prefix create "Office"      # Create isolated Wine prefix

sigma-wine prefix list                 # List Wine prefixes

sigma-wine uninstall <app-id>          # Remove Windows app

```

### sigma-arc — Android Runtime Controller

```bash
sigma-arc status                       # Android runtime status

sigma-arc install <path/to/app.apk>   # Install APK from file

sigma-arc launch <package.name>        # Launch Android app

sigma-arc list                         # List installed Android apps

sigma-arc uninstall <package.name>     # Remove Android app

sigma-arc grant <package> camera       # Grant camera permission

sigma-arc revoke <package> location    # Revoke location permission

sigma-arc backup <package> ~/          # Backup Android app data

sigma-arc restore <backup-file>        # Restore from backup

```

### sigma-container — Container Runtime

```bash
sigma-container pull ubuntu:22.04      # Pull container image

sigma-container run ubuntu:22.04       # Run container

sigma-container list                   # List running containers

sigma-container stop <id>              # Stop container

sigma-container exec <id> /bin/bash    # Shell into container

sigma-container build ./Dockerfile     # Build container image

sigma-container export <id> app.tar   # Export container as archive

```

### App Permission Manager

```bash
sigma-permissions list <app-id>        # View all permissions

sigma-permissions grant <app> network  # Grant network access

sigma-permissions deny <app> microphone # Block microphone

sigma-permissions reset <app>          # Reset all permissions to default

sigma-permissions audit                # Show all apps with sensitive permissions

sigma-permissions lockdown <app>       # Restrict to minimal permissions

```

### Sovereign Playground (Developer Sandbox)

```bash
sigma-playground new cpp               # New C++ interactive session

sigma-playground new rust              # New Rust interactive session

sigma-playground new python            # New Python session

sigma-playground run ./experiment.cpp  # Run and profile a C++ file

sigma-playground share                 # Share session as URL

```

---

## 🏪 Featured App Categories

| Category | Featured Apps |
|---|---|
| Development | VS Code, JetBrains IDEs, Neovim, Git, Docker, Kubernetes CLI |
| Productivity | LibreOffice, Obsidian, Notion (Web), Thunderbird |
| Creative | GIMP, Inkscape, Blender, DaVinci Resolve, Kdenlive |
| Finance | GnuCash, Portfolio Visualizer, Sigma Financial Audit Suite |
| Science | Jupyter, RStudio, Octave, Sigma Data Forge |
| Communication | Signal, Thunderbird, Zoom (Linux), Discord |
| Gaming | Steam (Linux), Lutris, RetroArch |
| Security | Wireshark, Metasploit, Sigma PQC Toolkit |

---

## 🆘 Support & Resources

- **Release Page**: [v15.0-zenith-app](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0-zenith-app)
- **App Compatibility Docs**: [Application-Layer](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Application-Layer)
- **Developer Guide**: [Developer_Guide](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Developer_Guide)
- **Issue Tracker**: [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)

---

*SigmaOS v15.0 Zenith App — Every app. Every runtime. One sovereign platform.*
