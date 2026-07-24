# Phase 3: Package & Update System

## Overview

SigmaOS replaces APT/DNF/Pacman with `sigpkg` — a sovereign, POSIX-free, cryptographically-verified package manager. It is modeled after the reproducibility of Nix and the speed of eopkg (Solus), without any POSIX/libc dependency.

> [!IMPORTANT]
> **Status**: `sigpkg` and the **entire userspace** is now implemented in `#![no_std]` Rust. All legacy C/C++ files have been removed across:
> - `usr/sigpkg/` — Sovereign Package Manager (replaces `sigma_pkg.c`, `SovereignPkgManager.cpp`)
> - `usr/shell/` — OmniShell, sh, logd, sigma_env, update_agent (5 files)
> - `usr/apps/` — All 22 userspace apps
> - `usr/ui/` — Zenith Desktop, SovereignShell, UICore, Browser, Spotlight, Launcher, AppStore, Dash, Control, Edit (10 files)
> - `drivers/network/` — All 24 network driver files
> - `drivers/storage/` — All 7 storage driver files
> - `drivers/usb/` — All 3 USB driver files
> - `drivers/multimedia/` — All 2 multimedia driver files
> - `drivers/*.cpp`, `drivers/*.c` — 14 top-level driver files


---

## `sigpkg` — Sovereign Package Manager

### Core Design Goals

| Goal | Approach |
|------|----------|
| **No POSIX dependency** | All package scripts run in a Sovereign WASM sandbox |
| **Reproducible builds** | Every package carries a content-addressed Merkle hash |
| **Cryptographic verification** | Ed25519 signatures on all packages and repo manifests |
| **Rollback & generations** | Each install creates a new system generation (NixOS-inspired) |
| **Zero-downtime updates** | Atomic swap via OverlayFS — old root stays live until swap |

### Package Format: `.spkg`

```
mypackage-1.0.0.spkg
├── MANIFEST.toml           # Name, version, deps, checksums
├── payload/                # Binary + data files (zstd-compressed)
│   ├── bin/
│   ├── lib/
│   └── share/
└── scripts/
    ├── pre-install.wasm    # Pre-install hook (WASM sandbox)
    └── post-install.wasm   # Post-install hook (WASM sandbox)
```

### `sigpkg` CLI

```bash
sigpkg install <package>          # Install from sovereign repo
sigpkg remove  <package>          # Uninstall, prune orphans
sigpkg update                     # Fetch latest manifests & upgrade
sigpkg rollback                   # Revert to previous generation
sigpkg search  <query>            # Search sovereign package index
sigpkg verify  <package.spkg>     # Verify Ed25519 signature
sigpkg list    --installed        # Show installed packages
```

---

## Update Channels

| Channel | Description | Stability |
|---------|-------------|-----------|
| `stable` | LTS-quality packages, security patches only | ⭐⭐⭐⭐⭐ |
| `testing` | Upcoming stable; feature-complete but not yet hardened | ⭐⭐⭐⭐ |
| `nightly` | Latest main branch builds; may break | ⭐⭐ |

Switch channels:
```bash
sigpkg channel set stable   # or testing, nightly
```

---

## Ubuntu/APT Bridge (ubuntu target only)

When building with `TARGET_OS=ubuntu`, SigmaOS enables an optional APT bridge — so users dual-booting or running the Ubuntu compat target can still install `.deb` packages via a thin shim:

```bash
sigma install --apt firefox      # Resolves via APT bridge
sigma install --snap vlc         # Resolves via Snap bridge
```

> [!WARNING]
> APT bridge packages run in an isolated Sovereign Sandbox — they cannot access the native SigmaOS kernel ABI directly.

---

## Dependency Independence

All core SigmaOS packages satisfy:
- **No libc**: Uses `sigma_libc.h` (freestanding).
- **No POSIX syscalls**: Routed through the Sovereign Syscall Gate.
- **No dynamic linker**: All core packages are statically linked.

---

## 🔗 Related Pages

- [Phase 2: Modularization & Profiles](Phase-2-Modularization-And-Profiles)
- [Phase 4: CI/CD & Testing](Phase-4-CICD-And-Testing)
- [Sovereign Packaging Specification](Sovereign-Packaging-Specification)
- [Zero Dependency Architecture](Zero-Dependency-Architecture)
