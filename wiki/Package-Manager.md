# Package Manager

> **Status:** In Progress
> **Language:** Rust + Shell (sigpkg CLI)
> **Source:** [`src/package/`](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/src/package) | [`scripts/sigpkg`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/scripts/sigpkg)

## Overview

SigmaOS's package management system is a multi-layer stack inspired by Linux Mint's celebrated update and package management UX and Omarchy's "omakase" zero-config philosophy. The Rust core implements dependency solving, tier-safety classification, and package metadata management. The `sigpkg` shell CLI provides an end-user-friendly interface.

## Architecture

```
SigmaOS Package Stack
├── sigpkg (Shell CLI)           ← User-facing CLI (mintinstall + omakase inspired)
├── src/package/                 ← Rust package management library
│   ├── dependency_resolver.rs   ← SAT/DPLL dependency solver
│   ├── dependency_graph.rs      ← Directed package dependency graph
│   ├── debian_apt.rs            ← APT/dpkg compatibility layer
│   ├── arch_aur.rs              ← AUR/pacman compatibility
│   ├── alpine_apk.rs            ← Alpine APK compatibility
│   ├── checkupdates.rs          ← Update availability checker
│   ├── cache.rs                 ← Package cache management
│   ├── cleanup.rs               ← Orphan removal, cache cleanup
│   ├── hardening.rs             ← Package integrity & signing
│   └── declarative_app.rs       ← Declarative app configuration
└── src/desktop/mint_update_manager.rs  ← 5-tier update safety model
```

### Key Components

- **sigpkg CLI**: Bash package manager with color output, tier-safety checks, BTRFS snapshots
- **DependencyResolver**: DPLL-based SAT solver for conflict-free package selection
- **MintUpdateManager**: 5-tier update safety classification (Certified → Dangerous)
- **MirrorNode**: Latency-aware mirror selection
- **declarative_app.rs**: Nix/Guix-inspired declarative app state

## Key Features

- **5-Tier Safety Model** (Linux Mint-inspired):
  - Tier 1: Certified core packages
  - Tier 2: Community tested  
  - Tier 3: Normal app updates
  - Tier 4: Upstream releases (unverified)
  - Tier 5: Kernel/deep system (requires snapshot)
- **Omakase Mode**: `sigpkg omakase` installs curated developer workstation in one command
- **BTRFS Snapshots**: Automatic pre-update snapshots (Timeshift-inspired)
- **Multi-format**: APT, AUR, APK, DNF, and Portage compatibility layers
- **DPLL SAT Solver**: Conflict-free dependency resolution
- **Mirror Selection**: Automatic fastest-mirror detection

## Inspiration

- **[Linux Mint mintinstall](https://github.com/linuxmint/mintinstall)** — Curated software center UX
- **[Linux Mint mintupdate](https://github.com/linuxmint/mintupdate)** — 5-tier safety update manager
- **[Omarchy omakase](https://github.com/basecamp/omarchy)** — Zero-config developer workstation setup
- **[Timeshift](https://github.com/linuxmint/timeshift)** — BTRFS snapshot integration

## Source Files

| File | Purpose |
|------|---------|
| `scripts/sigpkg` | User-facing CLI: install, remove, search, update, omakase |
| `src/package/dependency_resolver.rs` | DPLL SAT-based dependency solver |
| `src/package/dependency_graph.rs` | Directed graph for package relationships |
| `src/package/debian_apt.rs` | APT/dpkg compatibility |
| `src/package/arch_aur.rs` | AUR/pacman compatibility |
| `src/package/alpine_apk.rs` | Alpine APK compatibility |
| `src/package/checkupdates.rs` | Update availability checker |
| `src/package/cache.rs` | Package metadata cache |
| `src/package/cleanup.rs` | Orphan detection and removal |
| `src/package/hardening.rs` | Signature verification, integrity checks |
| `src/package/declarative_app.rs` | Declarative application state model |
| `src/desktop/mint_update_manager.rs` | 5-tier update classification engine |

## Configuration

```bash
# sigpkg configuration (read from /etc/sigmaos/sigpkg.conf)
TIER_LIMIT=3          # Only auto-apply updates Tier 1-3
AUTO_SNAPSHOT=true    # Snapshot before Tier 4/5 updates
MIRROR_COUNTRY=auto   # Auto-select fastest mirror
```

## Usage

```bash
# Install a package
sigpkg install neovim

# Search packages
sigpkg search "code editor"

# Show package info
sigpkg info firefox

# Apply tier-safe updates
sigpkg update

# Create BTRFS snapshot
sigpkg snapshot

# Install developer workstation (Omakase)
sigpkg omakase

# List installed packages
sigpkg list
```

## Testing

```bash
# Test Rust package modules
cargo test --lib -- package

# Test sigpkg CLI
bash -n scripts/sigpkg   # syntax check
```

## Roadmap

- [ ] GUI software center (mintinstall-style)
- [ ] Real BTRFS snapshot backend integration
- [ ] Package signature verification
- [ ] Flatpak/AppImage support
- [ ] Delta update downloads
- [ ] Build-from-source mode (Gentoo portage-inspired)

## See Also

- [Software-Store-and-Command-Palette](Software-Store-and-Command-Palette.md)
- [Backup-and-Recovery](Backup-and-Recovery.md)
- [LinuxMint-Parity](LinuxMint-Parity.md)
