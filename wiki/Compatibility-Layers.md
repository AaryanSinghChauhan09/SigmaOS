# Compatibility Layers

SigmaOS provides comprehensive compatibility layers allowing software built for Arch Linux, Debian/Ubuntu, Fedora/RHEL, Linux Mint, and Omarchy to run natively on SigmaOS without recompilation. All compatibility engines are written in Rust.

---

## Architecture Overview

```
 Application (Arch / Debian / Fedora / Mint binary)
         │
 ┌───────▼──────────────────────────────────────────┐
 │         SigmaOS Compatibility Engine              │
 │  ┌────────────┐  ┌───────────┐  ┌─────────────┐  │
 │  │ Arch Parity│  │ Debian    │  │ Fedora      │  │
 │  │ (pacman)   │  │ (apt/dpkg)│  │ (dnf/rpm)   │  │
 │  └────────────┘  └───────────┘  └─────────────┘  │
 │  ┌────────────┐  ┌───────────┐  ┌─────────────┐  │
 │  │ Mint Tools │  │ Omarchy   │  │ Canonical   │  │
 │  │ Supremacy  │  │ Gaming    │  │ (snap/apt)  │  │
 │  └────────────┘  └───────────┘  └─────────────┘  │
 └───────────────────────────────────────────────────┘
         │
 SigmaOS Native ABI + syscall layer
```

---

## Arch Linux Parity (`src/compatibility/arch_parity.rs`)

Full compatibility with Arch Linux ecosystem:

| Feature | Status |
|---------|--------|
| pacman database format | ✅ Read + write |
| PKGBUILD parsing | ✅ |
| AUR packages | ✅ via `arch_compat.rs` |
| systemd units | ✅ Translated to SigmaServiced |
| pacman hooks | ✅ |
| Arch mirrors | ✅ Sync via `DependencyResolverEngine` |
| `pkgctl` / `arch-build` | ✅ `ArchPkgctlEngine` |

---

## Mint Tools Supremacy (`src/compatibility/mint_tools_supremacy.rs`)

Implements and supersedes key Linux Mint tools:

### MintStick (`SovereignMintStickEngine`)
- USB image writer with SHA256 verification
- Supports ISO, IMG, compressed images
- Safer than Mint's MintStick: atomic write with verify-after-write

### Bulky (`SovereignBulkyEngine`)
- Batch file renamer with regex, case, sequence, metadata modes
- Preview before rename
- Undo log for safe reversal

### MintReport (`SovereignMintReportEngine`)
- System health report generator
- Health score: 0–100 (weighted by severity of findings)
- Categories: hardware, kernel, software, network, security

### Nemo Actions (`SovereignNemoActionsEngine`)
- File manager context menu actions
- Per-MIME-type action registration
- Batch action execution across file selections

### XApps Manager (`SovereignXAppsManager`)
- Unified theming for cross-desktop apps (GTK3/GTK4)
- HiDPI awareness and per-app scale overrides
- Replaces Mint's `xapp` library with a pure-Rust implementation

---

## Omarchy Gaming Performance Suite (`src/distro/omarchy_gaming_performance_suite.rs`)

Implements and supersedes Omarchy's gaming setup:

### Gaming Governor (`SovereignOmarchyGamingGovernor`)
- Switches CPU to `performance` governor on game launch
- GPU power limit increase (NVIDIA/AMD)
- AMDGPU overdrive enable/disable
- Automatic restore on game exit

### HUD Telemetry Engine (`SovereignOmarchyHudEngine`)
- In-game overlay: FPS, frame time, CPU/GPU temp, VRAM
- MangoHUD-compatible data sources
- Zero-overhead when HUD is hidden

### Developer Stacks (`SovereignOmarchyDeveloperStacks`)
- One-command dev environment setup
- Supported: Rust, Go, Node.js, Python (native Rust tooling)
- Version management without `asdf`/`nvm` dependencies

---

## Fedora Compatibility (`src/compatibility/fedora.rs`)

- RPM package extraction and translation
- DNF repository format reader
- SELinux policy compatibility layer
- Fedora Flatpak remote integration

---

## Debian/Ubuntu Compatibility

- `.deb` extraction and metadata parsing
- APT repository format reader
- dpkg trigger emulation
- Ubuntu PPA proxy support

---

## Binary Compatibility

SigmaOS uses a compatibility ABI shim that intercepts:
- `glibc` function calls → `sigma-libc` equivalents
- `libstdc++` → `sigma-cxx-compat`
- `systemd` D-Bus APIs → SigmaServiced IPC

---

## Comparison vs Competitors

| Compatibility | Linux Mint | Omarchy | Fedora | **SigmaOS** |
|--------------|-----------|---------|--------|-------------|
| Arch packages | ❌ | ✅ native | ❌ | ✅ |
| Debian packages | ✅ native | ❌ | ❌ | ✅ |
| Flatpak | ✅ | ✅ | ✅ | ✅ |
| Snap | ✅ | ❌ | ❌ | ✅ compat |
| Gaming suite | ❌ | ✅ | ❌ | ✅ superior |
| AUR | ❌ | ✅ | ❌ | ✅ |

---

## Source Files

| File | Description |
|------|-------------|
| `src/compatibility/arch_parity.rs` | Arch Linux compatibility |
| `src/compatibility/mint_tools_supremacy.rs` | Mint tools reimplementation |
| `src/compatibility/fedora.rs` | Fedora/RPM compatibility |
| `src/compatibility/legacy_adapters.rs` | Legacy binary compatibility |
| `src/distro/omarchy_gaming_performance_suite.rs` | Omarchy gaming features |
| `src/distro/sovereign_mint_omarchy_supremacy_suite.rs` | Mint+Omarchy supremacy |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/compatibility/`, `src/distro/`
> - Update compatibility table when new distro layers are added
> - Document new `SovereignMint*` or `SovereignOmarchy*` structs
> - Keep gaming governor GPU model list current
